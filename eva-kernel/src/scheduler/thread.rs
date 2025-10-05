use alloc::alloc::{alloc, dealloc, handle_alloc_error};
use core::alloc::{Layout, LayoutError};
use core::cell::Cell;
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicPtr, Ordering};
use core::{fmt, mem};

use crate::portability::{self, Impl as _};

use crate::scheduler;
use crate::scheduler::pause::{PauseCell, PauseToken, with_pause, yield_now_from_paused};

/// Thread entrypoint function.
pub type ThreadFn = unsafe extern "C" fn(*mut ());

pub use crate::scheduler::Priority;
use crate::utils::assert::harden_assert;

const THREAD_STACK_ALIGN: usize = 8;

/// Internal structure representing thread flags.
#[derive(Debug)]
pub(super) struct Flags(PauseCell<Cell<u8>>);

impl Flags {
    /// Thread suspend bit. Indicates thread is suspended and should not be executed.
    const SUSPEND: u8 = 1 << 0;
    /// Thread dead bit. Indicates thread is dead/terminates and should not be executed.
    const DEAD: u8 = 1 << 1;
    /// Thread idle bit. Indicates thread is the idle thread, and cannot be removed from ready queue.
    const IDLE: u8 = 1 << 2;

    /// Create a new empty flag set.
    pub(super) fn empty() -> Self {
        Self(PauseCell::from(0))
    }

    /// Query for the suspend bit.
    pub(super) fn suspend(&self, token: PauseToken) -> bool {
        (self.0.get(token) & Self::SUSPEND) != 0
    }

    /// Set the suspend bit.
    pub(super) fn set_suspend(&self, token: PauseToken) {
        self.0.update(token, |val| val | Self::SUSPEND);
    }

    /// Clear the suspend bit.
    pub(super) fn clear_suspend(&self, token: PauseToken) {
        self.0.update(token, |val| val & !Self::SUSPEND);
    }

    /// Query for the dead bit.
    pub(super) fn dead(&self, token: PauseToken) -> bool {
        (self.0.get(token) & Self::DEAD) != 0
    }

    /// Set the dead bit.
    pub(super) fn set_dead(&self, token: PauseToken) {
        self.0.update(token, |val| val | Self::DEAD);
    }

    /// Query for readiness, so not dead or suspended.
    pub(super) fn ready(&self, token: PauseToken) -> bool {
        (self.0.get(token) & (Self::DEAD | Self::SUSPEND)) == 0
    }

    /// Set the idle bit.
    pub(super) fn set_idle(&self, token: PauseToken) {
        self.0.update(token, |val| val | Self::IDLE);
    }

    /// Query for the idle bit.
    pub(super) fn idle(&self, token: PauseToken) -> bool {
        (self.0.get(token) & Self::IDLE) != 0
    }
}

/// Thread control block structure.
#[derive(Debug)]
pub(super) struct Tcb {
    /// Pointer to the top of the stack.
    pub stack_top_ptr: *mut u8,
    /// Pointer to the base of the allocated region.
    pub base_ptr: *mut u8,
    /// Size of the stack.
    pub stack_size: usize,
    /// Next pointer, used for scheduling queues.
    pub next: PauseCell<Cell<ThreadPtr>>,
    /// Previous pointer, used for scheduling queues.
    pub prev: PauseCell<Cell<ThreadPtr>>,
    /// Atomic next pointer, used for atomic linked list, such as defer queues.
    pub atomic_next: AtomicThreadPtr,
    /// Pointer to a thread waiting for a join.
    pub join_wait_thread: PauseCell<Cell<Option<ThreadPtr>>>,
    /// Thread flags.
    pub flags: Flags,
    /// Thread priority.
    pub priority: Priority,
}

/// Wrapper around a thread pointer.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreadPtr(NonNull<Tcb>);

unsafe impl Send for ThreadPtr {}
unsafe impl Sync for ThreadPtr {}

impl ThreadPtr {
    pub(super) const unsafe fn tcb(&self) -> &Tcb {
        unsafe { self.0.as_ref() }
    }

    pub(super) const fn into_tcb_ptr(thread: Option<ThreadPtr>) -> *mut Tcb {
        match thread {
            Some(thread) => thread.0.as_ptr(),
            None => ptr::null_mut(),
        }
    }

    pub(super) const fn from_tcb_ptr(tcb: *mut Tcb) -> Option<ThreadPtr> {
        match NonNull::new(tcb) {
            Some(tcb) => Some(ThreadPtr(tcb)),
            None => None,
        }
    }
}

#[repr(transparent)]
pub struct AtomicThreadPtr(AtomicPtr<Tcb>);

impl AtomicThreadPtr {
    pub const fn new(thread: Option<ThreadPtr>) -> Self {
        Self(AtomicPtr::new(ThreadPtr::into_tcb_ptr(thread)))
    }

    pub fn load(&self, order: Ordering) -> Option<ThreadPtr> {
        ThreadPtr::from_tcb_ptr(self.0.load(order))
    }

    pub fn store(&self, thread: Option<ThreadPtr>, order: Ordering) {
        self.0.store(ThreadPtr::into_tcb_ptr(thread), order);
    }

    pub fn swap(&self, thread: Option<ThreadPtr>, order: Ordering) -> Option<ThreadPtr> {
        ThreadPtr::from_tcb_ptr(self.0.swap(ThreadPtr::into_tcb_ptr(thread), order))
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
                ThreadPtr::into_tcb_ptr(current),
                ThreadPtr::into_tcb_ptr(new),
                success,
                failure,
            )
            .map(ThreadPtr::from_tcb_ptr)
            .map_err(ThreadPtr::from_tcb_ptr)
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
                ThreadPtr::into_tcb_ptr(current),
                ThreadPtr::into_tcb_ptr(new),
                success,
                failure,
            )
            .map(ThreadPtr::from_tcb_ptr)
            .map_err(ThreadPtr::from_tcb_ptr)
    }
}

impl fmt::Debug for AtomicThreadPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AtomicThreadPtr")
            .field(&self.load(Ordering::Relaxed))
            .finish()
    }
}

impl ThreadPtr {
    #[inline(always)]
    pub(super) unsafe fn link2(token: PauseToken, thread1: ThreadPtr, thread2: ThreadPtr) {
        unsafe {
            thread1.tcb().next.set(token, thread2);
            thread2.tcb().prev.set(token, thread1);
        }
    }

    #[inline(always)]
    pub(super) unsafe fn link3(
        token: PauseToken,
        thread1: ThreadPtr,
        thread2: ThreadPtr,
        thread3: ThreadPtr,
    ) {
        unsafe {
            Self::link2(token, thread1, thread2);
            Self::link2(token, thread2, thread3);
        }
    }

    pub(super) unsafe fn link_before(self, token: PauseToken, thread: ThreadPtr) {
        unsafe {
            let prev = thread.tcb().prev.get(token);
            Self::link3(token, prev, self, thread);
        }
    }

    pub(super) unsafe fn unlink(self, token: PauseToken) -> Option<ThreadPtr> {
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

impl ThreadPtr {
    #[allow(unused)] // TODO: Remove this unused
    pub(super) unsafe fn atomic_link(self, head: &AtomicThreadPtr) {
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
    pub(super) unsafe fn atomic_unlink(head: &AtomicThreadPtr) -> Option<ThreadPtr> {
        head.swap(None, Ordering::AcqRel)
    }

    #[allow(unused)] // TODO: Remove this unused
    pub(super) unsafe fn atomic_link_iter(
        start: Option<ThreadPtr>,
    ) -> impl Iterator<Item = ThreadPtr> {
        let mut iter = start;
        core::iter::from_fn(move || unsafe {
            let cur = iter?;

            iter = cur.tcb().atomic_next.load(Ordering::Relaxed);
            Some(cur)
        })
    }
}

impl ThreadPtr {
    fn layout_for(stack_size: usize) -> Result<(Layout, usize, usize), LayoutError> {
        let layout = Layout::from_size_align(stack_size, THREAD_STACK_ALIGN)?.pad_to_align();

        let (layout, stack_top_offset) =
            layout.extend(portability::GlobalImpl::switchctx_layout())?;
        let (layout, tcb_offset) = layout.extend(Layout::new::<Tcb>())?;

        Ok((layout, stack_top_offset, tcb_offset))
    }

    pub(super) fn create(stack_size: usize, priority: Priority) -> Self {
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
        let thread_ptr = ThreadPtr(tcb_ptr);

        // Initialize thread context
        let tcb = Tcb {
            stack_top_ptr: stack_top_ptr.as_ptr(),
            base_ptr: base_ptr.as_ptr(),
            stack_size,
            next: PauseCell::from(thread_ptr),
            prev: PauseCell::from(thread_ptr),
            atomic_next: AtomicThreadPtr::new(None),
            join_wait_thread: PauseCell::default(),
            flags: Flags::empty(),
            priority,
        };

        unsafe {
            tcb_ptr.write(tcb);
        }

        thread_ptr
    }

    pub(super) unsafe fn destroy(self) {
        let stack_size = unsafe { self.tcb().stack_size };
        let base_ptr = unsafe { self.tcb().base_ptr };

        // Compute the layout of the thread
        let (layout, _, _) = Self::layout_for(stack_size).expect("Failed to calculate thread size");

        // Deallocate
        unsafe {
            dealloc(base_ptr, layout);
        }
    }

    pub(super) unsafe fn set_as_global_switchctx(self) {
        unsafe {
            portability::GlobalImpl::set_global_switchctx(self.tcb().stack_top_ptr);
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
            portability::GlobalImpl::init_switchctx(
                self.tcb().stack_top_ptr,
                self.tcb().stack_top_ptr,
                self.tcb().stack_size,
                entry,
                arg1,
                arg2,
            );
        }
    }

    pub(super) unsafe fn init(self, entry: ThreadFn, user: *mut ()) {
        unsafe extern "C" fn launcher(arg1: *mut (), arg2: *mut ()) -> ! {
            let entry = unsafe { mem::transmute::<_, ThreadFn>(arg1) };

            unsafe {
                entry(arg2);
            }

            // Exit the thread and never come back
            with_pause(|token| unsafe { current().exit_paused(token) })
        }

        unsafe {
            self.init_switchctx(launcher, entry as _, user);
        }
    }

    unsafe fn exit_paused(self, token: PauseToken) -> ! {
        unsafe {
            harden_assert!(!self.tcb().flags.dead(token), "dead thread is running");

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

    pub(super) unsafe fn init_stack_payload<T>(self, value: T) -> *mut () {
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
            harden_assert!(
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
    portability::GlobalImpl::yield_now();
}

pub fn current() -> ThreadPtr {
    scheduler::current_thread()
}

pub unsafe fn spawn(
    stack_size: usize,
    priority: Priority,
    entry: unsafe extern "C" fn(*mut ()),
    user: *mut (),
) -> ThreadPtr {
    let thread = ThreadPtr::create(stack_size, priority);

    unsafe {
        // Initialize the thread initial context and priority
        thread.init(entry, user);
        // Register the thread to the scheduler
        scheduler::add_ready(thread);
    }

    thread
}

pub fn spawn2<F>(stack_size: usize, priority: Priority, entry: F) -> ThreadPtr
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

    let thread = ThreadPtr::create(stack_size, priority);

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
