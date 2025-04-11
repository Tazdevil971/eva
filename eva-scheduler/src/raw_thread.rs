use alloc::alloc::{alloc, dealloc, handle_alloc_error};
use core::alloc::{Layout, LayoutError};
use core::cell::Cell;
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicPtr, Ordering};
use core::{fmt, mem};

use crate::pause::{PauseCell, PauseToken, with_pause, yield_now_from_paused};
use crate::{kprintln, portability, scheduler};

pub use crate::scheduler::Priority;

/// Thread entrypoint function.
pub type ThreadFn = unsafe extern "C" fn(*mut ());

const THREAD_STACK_ALIGN: usize = 8;

/// Internal structure representing thread flags.
#[derive(Debug)]
pub(crate) struct Flags(PauseCell<Cell<u8>>);

impl Flags {
    /// Thread suspend bit. Indicates thread is suspended and should not be executed.
    const SUSPEND: u8 = 1 << 0;
    /// Thread dead bit. Indicates thread is dead/terminates and should not be executed.
    const DEAD: u8 = 1 << 1;
    /// Thread idle bit. Indicates thread is the idle thread, and cannot be removed from ready queue.
    const IDLE: u8 = 1 << 2;

    /// Create a new empty flag set.
    pub(crate) fn empty() -> Self {
        Self(PauseCell::from(0))
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
    pub next: PauseCell<Cell<RawThread>>,
    /// Previous pointer, used for scheduling queues.
    pub prev: PauseCell<Cell<RawThread>>,
    /// Atomic next pointer, used for atomic linked list, such as defer queues.
    pub atomic_next: AtomicRawThread,
    /// Pointer to a thread waiting for a join.
    pub join_wait_thread: PauseCell<Cell<Option<RawThread>>>,
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

impl fmt::Debug for AtomicRawThread {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AtomicRawThread")
            .field(&self.load(Ordering::Relaxed))
            .finish()
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
}

impl RawThread {
    #[allow(unused)] // TODO: Remove this unused
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

    #[allow(unused)] // TODO: Remove this unused
    pub(crate) unsafe fn atomic_unlink(head: &AtomicRawThread) -> Option<RawThread> {
        head.swap(None, Ordering::AcqRel)
    }

    #[allow(unused)] // TODO: Remove this unused
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

    pub(crate) fn create(stack_size: usize, priority: Priority) -> Self {
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
            next: PauseCell::from(thread_ptr),
            prev: PauseCell::from(thread_ptr),
            atomic_next: AtomicRawThread::new(None),
            join_wait_thread: PauseCell::default(),
            flags: Flags::empty(),
            priority,
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

    pub(crate) unsafe fn set_global_switchctx(self) {
        unsafe {
            portability::set_global_switchctx(self.tcb().stack_top_ptr);
        }
    }

    unsafe fn init_switchctx(
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
                self.tcb().stack_size,
                entry,
                arg1,
                arg2,
            );
        }
    }

    pub(crate) unsafe fn init(self, entry: ThreadFn, user: *mut ()) {
        unsafe extern "C" fn launcher(arg1: *mut (), arg2: *mut ()) -> ! {
            let entry = unsafe { mem::transmute::<_, ThreadFn>(arg1) };

            unsafe {
                entry(arg2);
            }

            // Exit the thread and never come back
            with_pause(|token| unsafe {
                current().exit_paused(token);
            })
        }

        unsafe {
            self.init_switchctx(launcher, entry as _, user);
        }
    }

    unsafe fn exit_paused(self, token: PauseToken) -> ! {
        unsafe {
            debug_assert!(!self.tcb().flags.dead(token), "dead thread is running");

            // Wake waiting threads, if there is one
            if let Some(join_wait_thread) = self.tcb().join_wait_thread.take(token) {
                join_wait_thread.resume_paused(token);
            }

            // Set the thread as dead
            self.tcb().flags.set_dead(token);
            scheduler::remove_ready_paused(token, self);

            // Yield to never be scheduled again
            yield_now_from_paused(token);

            unreachable!()
        }
    }

    pub(crate) unsafe fn init_stack_payload<T>(self, value: T) -> *mut () {
        unsafe {
            let ptr = self.tcb().base_ptr;

            ptr.cast::<T>().write(value);
            ptr.cast::<()>()
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

    unsafe fn suspend_paused(self, token: PauseToken) {
        unsafe {
            debug_assert!(
                !self.tcb().flags.suspend(token),
                "suspended thread is running"
            );

            self.tcb().flags.set_suspend(token);
            scheduler::remove_ready_paused(token, self);
        }
    }

    pub unsafe fn resume_paused(self, token: PauseToken) {
        unsafe {
            if self.tcb().flags.suspend(token) {
                self.tcb().flags.clear_suspend(token);
                scheduler::add_ready_paused(token, self);
            }
        }
    }

    pub unsafe fn join(self) {
        let current = current();

        with_pause(|token| unsafe {
            if self.tcb().flags.dead(token) {
                // The thread is already dead
                return;
            }

            // The thread is still alive and kicking!
            // Register for wait!
            self.tcb().join_wait_thread.set(token, Some(current));

            loop {
                // Wait for someone to wake us
                suspend_and_yield_paused(token);
                // Check if the thread is dead, do this to avoid spurious wakeups
                if self.tcb().flags.dead(token) {
                    break;
                }
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

/// Yield immediately.
pub fn yield_now() {
    portability::yield_now();
}

pub use crate::scheduler::current;

pub unsafe fn spawn(
    stack_size: usize,
    priority: Priority,
    entry: unsafe extern "C" fn(*mut ()),
    user: *mut (),
) -> RawThread {
    let thread = RawThread::create(stack_size, priority);

    unsafe {
        // Initialize the thread initial context and priority
        thread.init(entry, user);
        // Register the thread to the scheduler
        scheduler::add_ready(thread);
    }

    thread
}

pub fn spawn2<F>(stack_size: usize, priority: Priority, entry: F) -> RawThread
where
    F: FnOnce() + Send + Sync,
{
    let stack_size = stack_size.max(mem::size_of::<F>() + 512);

    unsafe extern "C" fn launcher<F>(arg: *mut ())
    where
        F: FnOnce() + Send + Sync,
    {
        // Extract function
        let entry = unsafe { arg.cast::<F>().read() };
        // Invoke it
        (entry)();
    }

    let thread = RawThread::create(stack_size, priority);

    unsafe {
        // Write the stack payload
        let user = thread.init_stack_payload(entry);
        // Initialize the thread initial context and priority
        thread.init(launcher::<F>, user);
        // Register the thread to the scheduler
        scheduler::add_ready(thread);
    }

    thread
}

pub fn suspend_and_yield_paused(token: PauseToken) {
    unsafe {
        // SAFETY: current() is always a valid thread
        current().suspend_paused(token);
    }
    yield_now_from_paused(token);
}
