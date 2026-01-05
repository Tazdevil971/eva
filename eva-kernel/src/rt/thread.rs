use alloc::alloc::{alloc, dealloc, handle_alloc_error};
use core::alloc::{Layout, LayoutError};
use core::cell::Cell;
use core::ffi::{CStr, c_char};
use core::fmt::{self, Debug};
use core::mem;
use core::ops::Deref;
use core::ptr::{self, NonNull};
use core::slice;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::port::{self, Impl as _};
use crate::rt::pause::{PauseCell, PauseToken};
use crate::rt::tls::LocalStore;
use crate::utils::linked_list::{self, Link};

use eva_abi::{OsError, Priority, ThreadId};

const THREAD_STACK_ALIGN: usize = 8;

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

#[derive(Debug, Clone)]
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

    unsafe fn alloc(&self) -> NonNull<u8> {
        unsafe {
            let ptr = alloc(self.layout) as *mut u8;
            let Some(ptr) = NonNull::new(ptr) else {
                handle_alloc_error(self.layout);
            };

            ptr
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8) {
        unsafe {
            dealloc(ptr, self.layout);
        }
    }
}

pub struct Tcb {
    sched_link: PauseCell<Link>,
    thread_link: PauseCell<Link>,

    pub state: PauseCell<Cell<State>>,
    pub term_action: PauseCell<Cell<TermAction>>,
    pub priority: Priority,
    pub id: ThreadId,

    pub stack_size: usize,
    pub name_size: usize,

    // Thread local store
    pub local_store: LocalStore,

    // Layout stuff
    pub base_ptr: *mut u8,
    pub stack_base_ptr: *mut u8,
    pub stack_top_ptr: *mut u8,
    pub switchctx_ptr: *mut u8,
    pub name_ptr: *mut u8,
}

impl Tcb {
    pub fn name_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.name_ptr, self.name_size) }
    }

    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(self.name_bytes()) }
    }

    pub fn run_dtors(&self) {
        self.local_store.run_dtors();
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
            .field("state", &self.state)
            .field("priority", &self.priority)
            .field("term_action", &self.term_action)
            .field("id", &self.id)
            .field("switchctx", &SwitchCtx)
            .field("stack_size", &self.stack_size)
            .field("stack_base_ptr", &self.stack_base_ptr)
            .field("stack_top_ptr", &self.stack_top_ptr)
            .finish()
    }
}

unsafe impl Send for Tcb {}
unsafe impl Sync for Tcb {}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ThreadPtr(pub(super) NonNull<Tcb>);

unsafe impl Send for ThreadPtr {}
unsafe impl Sync for ThreadPtr {}

pub struct SchedListAdapter;

unsafe impl linked_list::Adapter for SchedListAdapter {
    type Ptr = ThreadPtr;
    type Value = Tcb;

    fn offset_of_link(&self) -> usize {
        mem::offset_of!(Tcb, sched_link)
    }

    fn ptr_to_raw(&self, ptr: ThreadPtr) -> NonNull<Tcb> {
        ptr.0
    }

    unsafe fn ptr_from_raw(&self, raw: NonNull<Tcb>) -> ThreadPtr {
        ThreadPtr(raw)
    }
}

pub struct ThreadListAdapter;

unsafe impl linked_list::Adapter for ThreadListAdapter {
    type Ptr = ThreadPtr;
    type Value = Tcb;

    fn offset_of_link(&self) -> usize {
        mem::offset_of!(Tcb, thread_link)
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

    pub fn read_canary(self) -> [u8; 16] {
        // TODO: Why read volatile here? Is it really true that we must use volatile?
        unsafe { self.stack_base_ptr.cast::<[u8; 16]>().read_volatile() }
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
        let base_ptr = unsafe { layout.alloc() };

        // Calculate the various pointers
        let (stack_base_ptr, stack_top_ptr, switchctx_ptr, tcb_ptr, name_ptr) = unsafe {
            (
                base_ptr.add(layout.stack_base_offset),
                base_ptr.add(layout.stack_top_offset),
                base_ptr.add(layout.switchctx_offset),
                base_ptr.add(layout.tcb_offset),
                base_ptr.add(layout.name_offset),
            )
        };

        let tcb_ptr = tcb_ptr.cast::<Tcb>();
        let thread_ptr = ThreadPtr(tcb_ptr);

        // Paint stack canary
        unsafe { stack_base_ptr.cast().write(THREAD_CANARY_VALUE) }

        // Write thread name
        unsafe {
            ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_ptr.as_ptr(), name_bytes.len());
        }

        // Initialize thread context
        let tcb = Tcb {
            sched_link: PauseCell::new(Link::unlinked()),
            thread_link: PauseCell::new(Link::unlinked()),

            state: PauseCell::cell(State::Ready),
            term_action: PauseCell::cell(TermAction::None),
            priority,
            id,

            local_store: LocalStore::new(),

            stack_size,
            name_size,

            base_ptr: base_ptr.as_ptr(),
            stack_base_ptr: stack_base_ptr.as_ptr(),
            stack_top_ptr: stack_top_ptr.as_ptr(),
            switchctx_ptr: switchctx_ptr.as_ptr(),
            name_ptr: name_ptr.as_ptr(),
        };

        unsafe {
            port::GlobalImpl::init_switchctx(
                switchctx_ptr.as_ptr(),
                stack_top_ptr.as_ptr(),
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
            base_ptr,
            switchctx_ptr,
            ..
        } = unsafe { tcb.read() };

        // Compute the layout of the thread
        let layout = ThreadLayout::compute(stack_size, name_size)
            .expect("failed to calculate thread layout");

        unsafe {
            // Run drop for switchctx
            port::GlobalImpl::drop_switchctx(switchctx_ptr);
            // Deallocate
            layout.dealloc(base_ptr);
        }
    }

    pub unsafe fn set_as_global_switchctx(self) {
        unsafe {
            port::GlobalImpl::set_global_switchctx(self.switchctx_ptr);
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

    pub fn swap(&self, thread: Option<ThreadPtr>, order: Ordering) -> Option<ThreadPtr> {
        Self::from_raw(self.0.swap(Self::into_raw(thread), order))
    }

    pub fn compare_exchange(
        &self,
        current: Option<ThreadPtr>,
        new: Option<ThreadPtr>,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Option<ThreadPtr>, Option<ThreadPtr>> {
        self.0
            .compare_exchange(
                Self::into_raw(current),
                Self::into_raw(new),
                success,
                failure,
            )
            .map(Self::from_raw)
            .map_err(Self::from_raw)
    }

    pub fn compare_exchange_weak(
        &self,
        current: Option<ThreadPtr>,
        new: Option<ThreadPtr>,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Option<ThreadPtr>, Option<ThreadPtr>> {
        self.0
            .compare_exchange_weak(
                Self::into_raw(current),
                Self::into_raw(new),
                success,
                failure,
            )
            .map(Self::from_raw)
            .map_err(Self::from_raw)
    }
}
