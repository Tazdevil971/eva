use alloc::alloc::{alloc, dealloc, handle_alloc_error};
use core::alloc::{Layout, LayoutError};
use core::cell::Cell;
use core::mem;
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicPtr, AtomicU8, Ordering};

use crate::pause::{PauseMutex, PauseToken, with_pause, yield_later};
use crate::portability;
use crate::scheduler;

/// Thread entrypoint function.
pub type ThreadFn = unsafe extern "C" fn(*mut ());

const THREAD_STACK_ALIGN: usize = 8;

/// Thread parameters block.
pub struct ThreadParams {
    /// Requested minimum stack size.
    pub stack_size: usize,
    /// Thread priority after creation.
    pub priority: Priority,
    /// Thread entrypoint.
    pub entry: ThreadFn,
    /// User pointer, passed to thread entrypoint.
    pub user: *mut (),
}

/// Safe wrapper around priority values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Priority(u8);

impl Priority {
    /// Maximum allowed priority value.
    pub const MAX: Self = Self(scheduler::MAX_PRIORITY);
    /// Minimum allowed priority value.
    pub const MIN: Self = Self(scheduler::MIN_PRIORITY);

    /// Unsafely create a new `Priority`, skipping any validity checks.
    /// # Safety
    /// `prio` must be between `MIN` and `MAX` priorities.
    pub unsafe fn new_unchecked(prio: u8) -> Self {
        debug_assert!(
            prio >= scheduler::MIN_PRIORITY && prio <= scheduler::MAX_PRIORITY,
            "invalid priority"
        );
        Self(prio)
    }

    /// Create a new `Priority`, panics if `prio` is outside the allowed range.
    pub fn new(prio: u8) -> Self {
        Self::try_new(prio).expect("Invalid priority value!")
    }

    /// Create a new `Priority`, returns `None` if `prio` is outside the allowed range.
    pub fn try_new(prio: u8) -> Option<Self> {
        if prio >= scheduler::MIN_PRIORITY && prio <= scheduler::MAX_PRIORITY {
            Some(unsafe {
                // SAFETY: We just checked that priority is between 0 and 31
                Self::new_unchecked(prio)
            })
        } else {
            None
        }
    }

    /// Retrieve the raw priority value.
    pub fn value(self) -> u8 {
        self.0
    }
}

/// Internal structure representing thread flags.
#[derive(Debug)]
pub(crate) struct Flags(PauseMutex<Cell<u8>>);

impl Flags {
    /// Thread suspend bit. Indicates thread is suspended and should not be executed.
    const SUSPEND: u8 = 1 << 0;
    /// Thread dead bit. Indicates thread is dead/terminates and should not be executed.
    const DEAD: u8 = 1 << 1;
    /// Thread idle bit. Indicates thread is the idle thread, and cannot be removed from ready queue.
    const IDLE: u8 = 1 << 2;

    /// Create a new empty flag set.
    pub(crate) fn empty() -> Self {
        Self(PauseMutex::from(0))
    }

    /// Query for the suspend bit.
    pub(crate) fn suspend(&self, token: PauseToken) -> bool {
        (self.0.get(token) & Self::SUSPEND) != 0
    }

    /// Set the suspend bit.
    pub(crate) fn set_suspend(&self, token: PauseToken) {
        self.0.update(token, |val| val | Self::SUSPEND);
    }

    /// Clear the suspend bit.
    pub(crate) fn clear_suspend(&self, token: PauseToken) {
        self.0.update(token, |val| val & !Self::SUSPEND);
    }

    /// Query for the dead bit.
    pub(crate) fn dead(&self, token: PauseToken) -> bool {
        (self.0.get(token) & Self::DEAD) != 0
    }

    /// Set the dead bit.
    pub(crate) fn set_dead(&self, token: PauseToken) {
        self.0.update(token, |val| val | Self::DEAD);
    }

    /// Clear the dead bit.
    pub(crate) fn clear_dead(&self, token: PauseToken) {
        self.0.update(token, |val| val & !Self::DEAD);
    }

    /// Query for readiness, so not dead or suspended.
    pub(crate) fn ready(&self, token: PauseToken) -> bool {
        (self.0.get(token) & (Self::DEAD | Self::SUSPEND)) == 0
    }

    /// Set the idle bit.
    pub(crate) fn set_idle(&self, token: PauseToken) {
        self.0.update(token, |val| val | Self::IDLE);
    }

    /// Query for the idle bit.
    pub(crate) fn idle(&self, token: PauseToken) -> bool {
        (self.0.get(token) & Self::IDLE) != 0
    }
}

/// Thread control block structure.
#[derive(Debug)]
pub(crate) struct Tcb {
    /// Pointer to the top of the stack.
    pub stack_top_ptr: *mut u8,
    /// Pointer to the base of the allocated region.
    pub base_ptr: *mut u8,
    /// Size of the stack.
    pub stack_size: usize,
    /// Next pointer, used for scheduling queues.
    pub next: PauseMutex<Cell<RawThread>>,
    /// Previous pointer, used for scheduling queues.
    pub prev: PauseMutex<Cell<RawThread>>,
    /// Atomic next pointer, used for atomic linked list, such as defer queues.
    pub atomic_next: AtomicRawThread,
    /// Pointer to a thread waiting for a join.
    pub join_wait_thread: PauseMutex<Cell<Option<RawThread>>>,
    /// Thread flags.
    pub flags: Flags,
    /// Thread priority.
    pub priority: Priority,
}

/// Wrapper around a thread pointer.
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
#[derive(Debug)]
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

    pub fn swap(&self, thread: Option<RawThread>, order: Ordering) -> Option<RawThread> {
        RawThread::from_tcb_ptr(self.0.swap(RawThread::into_tcb_ptr(thread), order))
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

    pub(crate) unsafe fn atomic_link(self, head: &AtomicRawThread) {
        let mut ptr = head.load(Ordering::Acquire);
        loop {
            unsafe {
                self.tcb().atomic_next.store(ptr, Ordering::Relaxed);
            }

            match head.compare_exchange(ptr, Some(self), Ordering::Release, Ordering::Acquire) {
                Ok(_) => break,
                Err(new_ptr) => ptr = new_ptr,
            }
        }
    }

    pub(crate) unsafe fn atomic_unlink(head: &AtomicRawThread) -> Option<RawThread> {
        head.swap(None, Ordering::AcqRel)
    }

    pub(crate) unsafe fn atomic_link_iter(
        start: Option<RawThread>,
    ) -> impl Iterator<Item = RawThread> {
        let mut iter = start;
        core::iter::from_fn(move || unsafe {
            let cur = iter?;

            iter = cur.tcb().atomic_next.load(Ordering::Relaxed);
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
            atomic_next: AtomicRawThread::new(None),
            join_wait_thread: PauseMutex::default(),
            flags: Flags::empty(),
            priority: Priority::new(1),
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

    unsafe fn set_global_switchctx(self) {
        unsafe {
            portability::set_global_switchctx(self.tcb().stack_top_ptr);
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
                self.tcb().stack_top_ptr,
                self.tcb().stack_top_ptr,
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

            let thread = current();

            with_pause(|token| unsafe {
                // Check if someone is waiting
                if let Some(waiting) = thread.tcb().join_wait_thread.take(token) {
                    waiting.resume_paused(token);
                }

                // Terminate this thread
                thread.tcb().flags.set_dead(token);
                scheduler::remove_ready_paused(token, thread);
                // Schedule a preemption as soon as the pause ends
                yield_later(token);
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
            portability::enter_first_thread(
                self.tcb().stack_top_ptr,
                self.tcb().stack_top_ptr,
                entry,
            )
        }
    }

    pub unsafe fn priority(self) -> Priority {
        unsafe { self.tcb().priority }
    }

    pub unsafe fn resume(self) {
        with_pause(|token| unsafe {
            self.resume_paused(token);
        })
    }

    pub unsafe fn resume_paused(self, token: PauseToken) {
        unsafe {
            if self.tcb().flags.suspend(token) {
                self.tcb().flags.clear_suspend(token);
                scheduler::add_ready_paused(token, self);
            }
        }
    }

    pub unsafe fn suspend(self) {
        with_pause(|token| unsafe {
            self.suspend_paused(token);
        })
    }

    pub unsafe fn suspend_paused(self, token: PauseToken) {
        unsafe {
            if !self.tcb().flags.suspend(token) {
                self.tcb().flags.set_suspend(token);
                scheduler::remove_ready_paused(token, self);
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
                current.suspend_paused(token);
                // Yield as soon as the pause ends!
                yield_later(token);
            }
        });

        // Finally destroy the thread completely
        unsafe {
            self.destroy();
        }
    }

    pub unsafe fn is_idle_paused(self, token: PauseToken) -> bool {
        unsafe { self.tcb().flags.idle(token) }
    }
}

pub use scheduler::{current, yield_now};

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

pub fn suspend_paused(token: PauseToken) {
    unsafe {
        current().suspend_paused(token);
    }
}
