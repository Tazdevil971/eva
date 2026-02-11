use alloc::alloc::{alloc, dealloc, handle_alloc_error};
use core::alloc::{Layout, LayoutError};
use core::cell::{Cell, UnsafeCell};
use core::ffi::CStr;
use core::fmt::{self, Debug};
use core::ops::Deref;
use core::ptr::{self, NonNull};
use core::slice;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::port::{self, Impl as _};
use crate::rt::pause::PauseCell;
use crate::rt::tls::LocalStore;

use eva_abi::{Priority, ThreadId};
use eva_utils::{assert_send, assert_sync, linked_list, singly_linked_list};

const THREAD_STACK_ALIGN: usize = 16;

pub const THREAD_CANARY_VALUE: [u8; 16] = [
    0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Ready,
    Suspend,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TermAction {
    None,
    Resume(ThreadPtr),
    Detach,
}

#[derive(Debug, Clone, Copy)]
pub struct ThreadPointers {
    pub base: NonNull<u8>,
    pub stack_base: NonNull<u8>,
    pub stack_top: NonNull<u8>,
    pub switchctx: NonNull<u8>,
    pub name: NonNull<u8>,
}

unsafe impl Send for ThreadPointers {}
unsafe impl Sync for ThreadPointers {}

#[derive(Debug, Clone, Copy)]
struct ThreadLayout {
    layout: Layout,
    stack_base_offset: usize,
    stack_top_offset: usize,
    switchctx_offset: usize,
    tcb_offset: usize,
    name_offset: usize,
}

impl ThreadLayout {
    fn compute(stack_size: usize, name_size: usize) -> Result<Self, LayoutError> {
        let layout =
            Layout::from_size_align(stack_size + THREAD_CANARY_VALUE.len(), THREAD_STACK_ALIGN)?
                .pad_to_align();

        let (layout, switchctx_offset) = layout.extend(port::GlobalImpl::switchctx_layout())?;
        let (layout, tcb_offset) = layout.extend(Layout::new::<Tcb>())?;
        let (layout, name_offset) = layout.extend(Layout::from_size_align(name_size, 1)?)?;

        Ok(Self {
            layout,
            stack_base_offset: 0,
            stack_top_offset: switchctx_offset,
            switchctx_offset,
            tcb_offset,
            name_offset,
        })
    }

    unsafe fn alloc(&self) -> (ThreadPointers, NonNull<Tcb>) {
        unsafe {
            let ptr = alloc(self.layout) as *mut u8;
            let Some(ptr) = NonNull::new(ptr) else {
                handle_alloc_error(self.layout);
            };

            (
                ThreadPointers {
                    base: ptr,
                    stack_base: ptr.add(self.stack_base_offset),
                    stack_top: ptr.add(self.stack_top_offset),
                    switchctx: ptr.add(self.switchctx_offset),
                    name: ptr.add(self.name_offset),
                },
                ptr.add(self.tcb_offset).cast(),
            )
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8) {
        unsafe {
            dealloc(ptr, self.layout);
        }
    }
}

pub struct ThreadLocalInner {
    /// Thread local store
    pub store: LocalStore,
}

pub struct ThreadLocal(UnsafeCell<ThreadLocalInner>);

impl ThreadLocal {
    pub fn new() -> Self {
        Self(UnsafeCell::new(ThreadLocalInner {
            store: LocalStore::new(),
        }))
    }

    pub unsafe fn get(&self) -> &'static ThreadLocalInner {
        unsafe { self.0.get().as_ref().unwrap() }
    }
}

unsafe impl Send for ThreadLocal {}
unsafe impl Sync for ThreadLocal {}

impl fmt::Debug for ThreadLocal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ThreadLocal").finish_non_exhaustive()
    }
}

pub struct Tcb {
    // Link used for scheduling operations:
    // - scheduling queues
    sched_link: linked_list::AtomicLink<Self>,
    // Link used for the thread list:
    // - active thread list
    // - idle deletion defer operation
    thread_link: linked_list::AtomicLink<Self>,
    // Link used for atomic defer operations:
    // - defer resume
    defer_link: singly_linked_list::AtomicLink<Self>,

    pub state: PauseCell<Cell<State>>,
    pub term_action: PauseCell<Cell<TermAction>>,
    pub priority: Priority,
    pub id: ThreadId,

    pub stack_size: usize,
    pub name_size: usize,

    // Thread local store
    pub local: ThreadLocal,
    pub ptrs: ThreadPointers,
}

impl Tcb {
    pub fn name_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptrs.name.as_ptr(), self.name_size) }
    }

    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(self.name_bytes()) }
    }
}

impl Debug for Tcb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct SwitchCtx;

        impl Debug for SwitchCtx {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("<...>")
            }
        }

        f.debug_struct("Tcb")
            .field("name", &self.name())
            .field("sched_link", &self.sched_link)
            .field("thread_link", &self.thread_link)
            .field("defer_link", &self.defer_link)
            .field("state", &self.state)
            .field("term_action", &self.term_action)
            .field("priority", &self.priority)
            .field("id", &self.id)
            .field("switchctx", &SwitchCtx)
            .field("stack_size", &self.stack_size)
            .field("local", &self.local)
            .field("ptrs", &self.ptrs)
            .finish()
    }
}

assert_send!(Tcb);
assert_sync!(Tcb);

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ThreadPtr(pub(super) NonNull<Tcb>);

unsafe impl Send for ThreadPtr {}
unsafe impl Sync for ThreadPtr {}

#[derive(Clone, Copy)]
pub struct SchedListAdapter;

impl linked_list::Adapter for SchedListAdapter {
    type Ptr = ThreadPtr;
    type Value = Tcb;
    type Link = linked_list::AtomicLink<Tcb>;

    unsafe fn raw_to_link(&self, raw: NonNull<Self::Value>) -> NonNull<Self::Link> {
        unsafe {
            let ptr = ptr::addr_of_mut!((*raw.as_ptr()).sched_link);
            NonNull::new_unchecked(ptr)
        }
    }

    fn ptr_to_raw(&self, ptr: ThreadPtr) -> NonNull<Tcb> {
        ptr.0
    }

    unsafe fn ptr_from_raw(&self, raw: NonNull<Tcb>) -> ThreadPtr {
        ThreadPtr(raw)
    }
}

#[derive(Clone, Copy)]
pub struct ThreadListAdapter;

impl linked_list::Adapter for ThreadListAdapter {
    type Ptr = ThreadPtr;
    type Value = Tcb;
    type Link = linked_list::AtomicLink<Tcb>;

    unsafe fn raw_to_link(&self, raw: NonNull<Self::Value>) -> NonNull<Self::Link> {
        unsafe {
            let ptr = ptr::addr_of_mut!((*raw.as_ptr()).thread_link);
            NonNull::new_unchecked(ptr)
        }
    }

    fn ptr_to_raw(&self, ptr: ThreadPtr) -> NonNull<Tcb> {
        ptr.0
    }

    unsafe fn ptr_from_raw(&self, raw: NonNull<Tcb>) -> ThreadPtr {
        ThreadPtr(raw)
    }
}

#[derive(Clone, Copy)]
pub struct DeferAdapter;

impl singly_linked_list::Adapter for DeferAdapter {
    type Ptr = ThreadPtr;
    type Value = Tcb;
    type Link = singly_linked_list::AtomicLink<Tcb>;

    unsafe fn raw_to_link(&self, raw: NonNull<Self::Value>) -> NonNull<Self::Link> {
        unsafe {
            let ptr = ptr::addr_of_mut!((*raw.as_ptr()).defer_link);
            NonNull::new_unchecked(ptr)
        }
    }

    fn ptr_to_raw(&self, ptr: ThreadPtr) -> NonNull<Tcb> {
        ptr.0
    }

    unsafe fn ptr_from_raw(&self, raw: NonNull<Tcb>) -> ThreadPtr {
        ThreadPtr(raw)
    }
}

impl ThreadPtr {
    pub fn to_abi(self) -> eva_abi::Thread {
        eva_abi::Thread(self.0.cast())
    }

    pub unsafe fn from_abi(ptr: eva_abi::Thread) -> Self {
        Self(ptr.0.cast())
    }

    pub fn tcb(&self) -> &Tcb {
        unsafe { self.0.as_ref() }
    }

    #[allow(unused)]
    pub fn read_canary(self) -> [u8; 16] {
        // TODO: Why read volatile here? Is it really true that we must use volatile?
        unsafe { self.ptrs.stack_base.cast::<[u8; 16]>().read_volatile() }
    }

    pub fn create(
        stack_size: usize,
        priority: Priority,
        name: &CStr,
        id: ThreadId,
        entry: extern "C" fn(*mut (), *mut (), *mut (), *mut ()) -> !,
        arg1: *mut (),
        arg2: *mut (),
        arg3: *mut (),
        arg4: *mut (),
    ) -> Self {
        let name_bytes = name.to_bytes_with_nul();
        let name_size = name_bytes.len();

        // Compute the layout of the thread
        let layout = ThreadLayout::compute(stack_size, name_size)
            .expect("failed to calculate thread layout");

        // Allocate the whole structure
        let (ptrs, tcb_ptr) = unsafe { layout.alloc() };

        let thread_ptr = ThreadPtr(tcb_ptr);

        // Paint stack canary
        unsafe { ptrs.stack_base.cast().write(THREAD_CANARY_VALUE) }

        // Write thread name
        unsafe {
            ptr::copy_nonoverlapping(name_bytes.as_ptr(), ptrs.name.as_ptr(), name_bytes.len());
        }

        // Initialize thread context
        let tcb = Tcb {
            sched_link: linked_list::AtomicLink::unlinked(),
            thread_link: linked_list::AtomicLink::unlinked(),
            defer_link: singly_linked_list::AtomicLink::unlinked(),

            state: PauseCell::cell(State::Ready),
            term_action: PauseCell::cell(TermAction::None),
            priority,
            id,

            stack_size,
            name_size,

            local: ThreadLocal::new(),
            ptrs,
        };

        unsafe {
            port::GlobalImpl::init_switchctx(
                ptrs.switchctx.as_ptr(),
                ptrs.stack_top.as_ptr(),
                stack_size,
                entry,
                arg1,
                arg2,
                arg3,
                arg4,
            );
        }

        // Actually write out TCB
        unsafe {
            tcb_ptr.write(tcb);
        }

        thread_ptr
    }

    pub unsafe fn destroy(self) {
        let ThreadPtr(tcb) = self;

        // Read out TCB
        let Tcb {
            stack_size,
            name_size,
            ptrs,
            ..
        } = unsafe { tcb.read() };

        // Compute the layout of the thread
        let layout = ThreadLayout::compute(stack_size, name_size)
            .expect("failed to calculate thread layout");

        unsafe {
            // Run drop for switchctx
            port::GlobalImpl::drop_switchctx(ptrs.switchctx.as_ptr());
            // Deallocate
            layout.dealloc(ptrs.base.as_ptr());
        }
    }

    pub unsafe fn set_as_global_switchctx(self) {
        unsafe {
            port::GlobalImpl::set_global_switchctx(self.ptrs.switchctx.as_ptr());
        }
    }
}

impl Debug for ThreadPtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self.tcb(), f)
    }
}

impl Deref for ThreadPtr {
    type Target = Tcb;

    fn deref(&self) -> &Tcb {
        self.tcb()
    }
}

#[repr(transparent)]
pub struct AtomicThreadPtr(AtomicPtr<Tcb>);

impl AtomicThreadPtr {
    const fn from_raw(raw: *mut Tcb) -> Option<ThreadPtr> {
        // TODO: Option::map in const is unstable
        match NonNull::new(raw) {
            Some(raw) => Some(ThreadPtr(raw)),
            None => None,
        }
    }

    const fn into_raw(ptr: Option<ThreadPtr>) -> *mut Tcb {
        // TODO: Option::map in const is unstable
        match ptr {
            Some(ptr) => ptr.0.as_ptr(),
            None => ptr::null_mut(),
        }
    }

    pub const fn new(thread: Option<ThreadPtr>) -> Self {
        Self(AtomicPtr::new(Self::into_raw(thread)))
    }

    pub fn load(&self, order: Ordering) -> Option<ThreadPtr> {
        Self::from_raw(self.0.load(order))
    }

    pub fn store(&self, thread: Option<ThreadPtr>, order: Ordering) {
        self.0.store(Self::into_raw(thread), order);
    }
}
