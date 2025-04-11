use alloc::alloc::{alloc, dealloc, handle_alloc_error};
use core::alloc::{Layout, LayoutError};
use core::cell::Cell;
use core::mem;
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicPtr, AtomicU8, Ordering};

use crate::pause::{PauseMutex, PauseToken, ask_for_preempt, with_pause};
use crate::portability;
use crate::scheduler;

pub type ThreadFn = unsafe extern "C" fn(*mut ());

const THREAD_STACK_ALIGN: usize = 8;

pub struct ThreadParams {
    pub stack_size: usize,
    pub priority: u8,
    pub entry: ThreadFn,
    pub user: *mut (),
}

#[derive(Debug)]
pub(crate) struct Flags(PauseMutex<Cell<u8>>);

impl Flags {
    const SUSPEND: u8 = 1 << 0;
    const DEAD: u8 = 1 << 1;

    pub(crate) fn empty() -> Self {
        Self(PauseMutex::from(0))
    }

    pub(crate) fn suspend(&self, token: PauseToken) -> bool {
        (self.0.get(token) & Self::SUSPEND) != 0
    }

    pub(crate) fn set_suspend(&self, token: PauseToken) {
        self.0.update(token, |val| val | Self::SUSPEND);
    }

    pub(crate) fn clear_suspend(&self, token: PauseToken) {
        self.0.update(token, |val| val & !Self::SUSPEND);
    }

    pub(crate) fn dead(&self, token: PauseToken) -> bool {
        (self.0.get(token) & Self::DEAD) != 0
    }

    pub(crate) fn set_dead(&self, token: PauseToken) {
        self.0.update(token, |val| val | Self::DEAD);
    }

    pub(crate) fn clear_dead(&self, token: PauseToken) {
        self.0.update(token, |val| val & !Self::DEAD);
    }

    pub(crate) fn ready(&self, token: PauseToken) -> bool {
        (self.0.get(token) & (Self::DEAD | Self::SUSPEND)) == 0
    }
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct Tcb {
    pub stack_top_ptr: *mut u8,
    pub base_ptr: *mut u8,
    pub stack_size: usize,
    pub next: PauseMutex<Cell<RawThread>>,
    pub prev: PauseMutex<Cell<RawThread>>,
    pub join_wait_thread: PauseMutex<Cell<Option<RawThread>>>,
    pub flags: Flags,
    pub priority: u8,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawThread(NonNull<Tcb>);

unsafe impl Send for RawThread {}
unsafe impl Sync for RawThread {}

impl RawThread {
    pub(crate) const unsafe fn tcb(&self) -> &Tcb {
        unsafe { self.0.as_ref() }
    }

    pub(crate) const unsafe fn tcb_mut(&mut self) -> &mut Tcb {
        unsafe { self.0.as_mut() }
    }

    pub(crate) const unsafe fn stack_top_ptr(self) -> *mut u8 {
        unsafe { self.tcb().stack_top_ptr }
    }

    pub(crate) const unsafe fn switchctx_ptr(self) -> *mut u8 {
        unsafe { self.tcb().stack_top_ptr }
    }

    pub(crate) const fn into_tcb_ptr(thread: Option<RawThread>) -> *mut Tcb {
        match thread {
            Some(thread) => thread.0.as_ptr(),
            None => ptr::null_mut(),
        }
    }

    pub(crate) const fn from_tcb_ptr(tcb: *mut Tcb) -> Option<RawThread> {
        match NonNull::new(tcb) {
            Some(tcb) => Some(RawThread(tcb)),
            None => None,
        }
    }
}

#[repr(transparent)]
pub struct AtomicRawThread(AtomicPtr<Tcb>);

impl AtomicRawThread {
    pub const fn new(thread: Option<RawThread>) -> Self {
        Self(AtomicPtr::new(RawThread::into_tcb_ptr(thread)))
    }

    pub fn load(&self, order: Ordering) -> Option<RawThread> {
        RawThread::from_tcb_ptr(self.0.load(order))
    }

    pub fn store(&self, thread: Option<RawThread>, order: Ordering) {
        self.0.store(RawThread::into_tcb_ptr(thread), order);
    }

    pub fn compare_exchange(
        &self,
        current: Option<RawThread>,
        new: Option<RawThread>,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Option<RawThread>, Option<RawThread>> {
        self.0
            .compare_exchange(
                RawThread::into_tcb_ptr(current),
                RawThread::into_tcb_ptr(new),
                success,
                failure,
            )
            .map(RawThread::from_tcb_ptr)
            .map_err(RawThread::from_tcb_ptr)
    }

    pub fn compare_exchange_weak(
        &self,
        current: Option<RawThread>,
        new: Option<RawThread>,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Option<RawThread>, Option<RawThread>> {
        self.0
            .compare_exchange_weak(
                RawThread::into_tcb_ptr(current),
                RawThread::into_tcb_ptr(new),
                success,
                failure,
            )
            .map(RawThread::from_tcb_ptr)
            .map_err(RawThread::from_tcb_ptr)
    }
}

impl RawThread {
    #[inline(always)]
    pub(crate) unsafe fn link2(token: PauseToken, thread1: RawThread, thread2: RawThread) {
        unsafe {
            thread1.tcb().next.set(token, thread2);
            thread2.tcb().prev.set(token, thread1);
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn link3(
        token: PauseToken,
        thread1: RawThread,
        thread2: RawThread,
        thread3: RawThread,
    ) {
        unsafe {
            Self::link2(token, thread1, thread2);
            Self::link2(token, thread2, thread3);
        }
    }

    pub(crate) unsafe fn link_before(self, token: PauseToken, thread: RawThread) {
        unsafe {
            let prev = thread.tcb().prev.get(token);
            Self::link3(token, prev, self, thread);
        }
    }

    pub(crate) unsafe fn link_after(self, token: PauseToken, thread: RawThread) {
        unsafe {
            let next = thread.tcb().next.get(token);
            Self::link3(token, thread, self, next);
        }
    }

    pub(crate) unsafe fn unlink(self, token: PauseToken) -> Option<RawThread> {
        unsafe {
            let prev = self.tcb().prev.get(token);
            let next = self.tcb().next.get(token);

            if prev == self && next == self {
                // The thread is already unlinked
                None
            } else {
                Self::link2(token, self, self);
                Self::link2(token, prev, next);
                Some(next)
            }
        }
    }

    pub(crate) unsafe fn link_iter(
        start: RawThread,
        token: PauseToken,
    ) -> impl Iterator<Item = RawThread> {
        let mut iter = Some(start);

        core::iter::from_fn(move || unsafe {
            let cur = iter?;

            let next = cur.tcb().next.get(token);
            if next == start {
                iter = None;
            } else {
                iter = Some(next);
            }

            Some(cur)
        })
    }
}

impl RawThread {
    fn layout_for(stack_size: usize) -> Result<(Layout, usize, usize), LayoutError> {
        let layout = Layout::from_size_align(stack_size, THREAD_STACK_ALIGN)?.pad_to_align();

        let (layout, stack_top_offset) = layout.extend(portability::switchctx_layout())?;
        let (layout, tcb_offset) = layout.extend(Layout::new::<Tcb>())?;

        Ok((layout, stack_top_offset, tcb_offset))
    }

    pub(crate) fn create(stack_size: usize) -> Self {
        // Compute the layout of the thread
        let (layout, stack_top_offset, tcb_offset) =
            Self::layout_for(stack_size).expect("Failed to calculate thread size");

        // Allocate the whole structure
        let base_ptr = unsafe {
            let ptr = alloc(layout) as *mut u8;
            let Some(ptr) = NonNull::new(ptr) else {
                handle_alloc_error(layout);
            };

            ptr
        };

        // Calculate the various pointers
        let stack_top_ptr = unsafe { base_ptr.add(stack_top_offset) };
        let tcb_ptr = unsafe { base_ptr.add(tcb_offset) };

        let tcb_ptr = tcb_ptr.cast::<Tcb>();
        let thread_ptr = RawThread(tcb_ptr);

        // Initialize thread context
        let tcb = Tcb {
            stack_top_ptr: stack_top_ptr.as_ptr(),
            base_ptr: base_ptr.as_ptr(),
            stack_size,
            next: PauseMutex::from(thread_ptr),
            prev: PauseMutex::from(thread_ptr),
            join_wait_thread: PauseMutex::default(),
            flags: Flags::empty(),
            priority: 0,
        };

        unsafe {
            tcb_ptr.write(tcb);
        }

        thread_ptr
    }

    pub(crate) unsafe fn destroy(self) {
        let stack_size = unsafe { self.tcb().stack_size };
        let base_ptr = unsafe { self.tcb().base_ptr };

        // Compute the layout of the thread
        let (layout, _, _) = Self::layout_for(stack_size).expect("Failed to calculate thread size");

        // Deallocate
        unsafe {
            dealloc(base_ptr, layout);
        }
    }

    unsafe fn init_raw(
        self,
        entry: unsafe extern "C" fn(*mut (), *mut ()) -> !,
        arg1: *mut (),
        arg2: *mut (),
    ) {
        unsafe {
            // The switch context and the stack both sit at the same address, they just grow in different directions
            portability::init_switchctx(
                self.switchctx_ptr(),
                self.stack_top_ptr(),
                entry,
                arg1,
                arg2,
            );
        }
    }

    pub(crate) unsafe fn init(self, entry: ThreadFn, user: *mut ()) {
        unsafe extern "C" fn thread_entry(arg1: *mut (), arg2: *mut ()) -> ! {
            let entry = unsafe { mem::transmute::<_, ThreadFn>(arg1) };

            unsafe {
                entry(arg2);
            }

            // TODO: Remove this usage of global state!
            let thread = current();

            with_pause(|token| unsafe {
                // Check if someone is waiting
                if let Some(waiting) = thread.tcb().join_wait_thread.take(token) {
                    waiting.resume_pt(token);
                }

                // Terminate this thread
                thread.tcb().flags.set_dead(token);
                scheduler::remove_ready_pt(token, thread);
                // Schedule a preemption as soon as the pause ends
                ask_for_preempt(token);
            });

            // This will never be reached, as this will never be scheduled
            unreachable!()
        }

        unsafe {
            self.init_raw(thread_entry, entry as _, user);
        }
    }

    pub(crate) unsafe fn enter_first_thread(self, entry: unsafe extern "C" fn() -> !) -> ! {
        unsafe {
            portability::enter_first_thread(self.switchctx_ptr(), self.stack_top_ptr(), entry)
        }
    }

    pub unsafe fn priority(self) -> u8 {
        unsafe { self.tcb().priority }
    }

    pub unsafe fn resume(self) {
        with_pause(|token| unsafe {
            self.resume_pt(token);
        })
    }

    pub unsafe fn resume_pt(self, token: PauseToken) {
        unsafe {
            if self.tcb().flags.suspend(token) {
                self.tcb().flags.clear_suspend(token);
                scheduler::add_ready_pt(token, self);
            }
        }
    }

    pub unsafe fn suspend(self) {
        with_pause(|token| unsafe {
            self.suspend_pt(token);
        })
    }

    pub unsafe fn suspend_pt(self, token: PauseToken) {
        unsafe {
            if !self.tcb().flags.suspend(token) {
                self.tcb().flags.set_suspend(token);
                scheduler::remove_ready_pt(token, self);
            }
        }
    }

    pub unsafe fn join(self) {
        let current = current();

        with_pause(|token| unsafe {
            if !self.tcb().flags.dead(token) {
                // The thread is still alive and kicking!
                // Register for wait!
                self.tcb().join_wait_thread.set(token, Some(current));
                current.suspend_pt(token);
                // Pend a preemption as soon as the pause ends!
                ask_for_preempt(token);
            }
        });

        // Finally destroy the thread completely
        unsafe {
            self.destroy();
        }
    }
}

pub use scheduler::current;

pub fn spawn(params: ThreadParams) -> RawThread {
    let mut thread = RawThread::create(params.stack_size);

    unsafe {
        // Initialize the thread initial context and priority
        thread.init(params.entry, params.user);
        thread.tcb_mut().priority = params.priority;
        // Register the thread to the scheduler
        scheduler::add_ready(thread);
    }

    thread
}

pub fn suspend() {
    unsafe {
        current().suspend();
    }
}

pub fn suspend_pt(token: PauseToken) {
    unsafe {
        current().suspend_pt(token);
    }
}

pub fn preempt() {
    portability::preempt();
}
