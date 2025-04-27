use core::cell::{Ref, RefCell, RefMut};
use core::ptr;
use core::sync::atomic::{AtomicU8, Ordering};

use crate::pause::{PauseMutex, PauseToken};
use crate::portability;
use crate::raw_thread::{AtomicRawThread, Priority, RawThread, ThreadParams};

pub const MIN_PRIORITY: u8 = 0;
pub const MAX_PRIORITY: u8 = 31;

/// Internal structure representing scheduler pause state
struct PauseState(AtomicU8);

impl PauseState {
    /// Bit marking a pending schedule operation.
    const SCHED_PEND: u8 = 1 << 0;
    /// Bit marking scheduler pause active.
    const PAUSED: u8 = 1 << 1;
    /// Bit marking scheduler offline.
    const OFFLINE: u8 = 1 << 2;

    /// Create a new `PauseState` already in offline state.
    #[inline(always)]
    const fn offline() -> Self {
        Self(AtomicU8::new(Self::OFFLINE))
    }

    #[inline(always)]
    fn online(&self) {
        // TODO: Is Relaxed here correct?
        let old = self.0.fetch_and(!Self::OFFLINE, Ordering::Relaxed);
    }

    /// Try to set the pause bit. Returns true if successful.
    #[inline(always)]
    fn pause(&self) -> bool {
        // TODO: Is Relaxed here correct?
        let old = self.0.fetch_or(Self::PAUSED, Ordering::Relaxed);
        (old & Self::PAUSED) == 0
    }

    /// Unset the pause and schedule pending bit. Returns true if the schedule pending bit was on.
    #[inline(always)]
    fn unpause(&self) -> bool {
        // TODO: Is Relaxed here correct?
        let old = self.0.fetch_and(!Self::PAUSED, Ordering::Relaxed);
        (old & Self::SCHED_PEND) != 0
    }

    /// Query if either the pause or offline bit are on.
    #[inline(always)]
    fn is_paused(&self) -> bool {
        // TODO: Is Relaxed here correct?
        let value = self.0.load(Ordering::Relaxed);
        (value & (Self::PAUSED | Self::OFFLINE)) != 0
    }

    #[inline(always)]
    fn is_offline(&self) -> bool {
        // TODO: Is Relaxed here correct?
        let value = self.0.load(Ordering::Relaxed);
        (value & Self::OFFLINE) != 0
    }

    /// Set the schedule pending bit.
    #[inline(always)]
    fn set_sched_pend(&self) {
        // TODO: Is Relaxed here correct?
        self.0.fetch_or(Self::SCHED_PEND, Ordering::Relaxed);
    }

    /// Clear the schedule pending bit.
    #[inline(always)]
    fn clear_sched_pend(&self) {
        // TODO: Is Relaxed here correct?
        self.0.fetch_and(!Self::SCHED_PEND, Ordering::Relaxed);
    }
}

struct Rings {
    /// Bitmask representing queues that are not empty.
    prio_mask: u32,
    /// List of priority queues.
    prio: [Option<RawThread>; 32],
    /// Idle thread.
    idle: Option<RawThread>,
    // wait: Option<RawThread>,
}

struct Scheduler {
    rings: PauseMutex<RefCell<Rings>>,
    cur: AtomicRawThread,
    state: PauseState,
}

static SCHEDULER: Scheduler = Scheduler {
    rings: PauseMutex::new(RefCell::new(Rings {
        prio_mask: 0,
        prio: [None; 32],
        idle: None,
    })),
    cur: AtomicRawThread::new(None),
    state: PauseState::offline(),
};

/// Internal function to get ring at specific priority.
fn get_ring_at_prio(rings: &Rings, prio: Priority) -> &Option<RawThread> {
    unsafe {
        // SAFETY: prio.value() is guaranteed to be between 0 and 31
        rings.prio.get_unchecked(prio.value() as usize)
    }
}

/// Internal function to get ring at specific priority.
fn get_ring_at_prio_mut(rings: &mut Rings, prio: Priority) -> &mut Option<RawThread> {
    unsafe {
        // SAFETY: prio.value() is guaranteed to be between 0 and 31
        rings.prio.get_unchecked_mut(prio.value() as usize)
    }
}

/// Set the currently active thread.
unsafe fn set_current_paused(token: PauseToken, thread: RawThread) {
    unsafe {
        // Set the global switch context pointer
        // SAFETY: This can only be called with a paused scheduler
        portability::set_global_switchctx(thread.tcb().stack_top_ptr);
    }
    // Set the global variable pointing to our thread
    // TODO: Is Relaxed here correct?
    SCHEDULER.cur.store(Some(thread), Ordering::Relaxed);
}

/// Query the currently active thread.
pub fn current() -> RawThread {
    // TODO: Is Relaxed here correct?
    SCHEDULER
        .cur
        .load(Ordering::Relaxed)
        .expect("No current thread, scheduler should not be running!")
}

/// Try to pause the scheduler. Returns true if successful.
pub fn pause() -> bool {
    SCHEDULER.state.pause()
}

/// Unpause the scheduler.
/// # Safety
/// Can only be called when no `PauseToken` instance exist anymore.
pub unsafe fn unpause() {
    if SCHEDULER.state.unpause() {
        // Someone pended a preemption, do it now
        yield_now();
    }
}

/// Enter a critical section using a scheduler pause.
pub fn with_pause<F, T>(f: F) -> T
where
    F: FnOnce(PauseToken<'_>) -> T,
{
    let ret;
    let success = pause();
    {
        let token = unsafe {
            // SAFETY: The scheduler is paused here
            PauseToken::new()
        };

        ret = f(token);
    }

    if success {
        unsafe {
            // SAFETY: We destroyed our token, so now we can safely unpause
            unpause();
        }
    }

    ret
}

/// Yield as soon as the scheduler pause ends.
pub fn yield_later(_token: PauseToken) {
    SCHEDULER.state.set_sched_pend();
}

/// Yield immediately.
pub fn yield_now() {
    portability::yield_now();
}

/// Query if the scheduler is paused of offline.
pub fn is_paused() -> bool {
    SCHEDULER.state.is_paused()
}

/// Query if the scheduler is online
pub fn is_offline() -> bool {
    SCHEDULER.state.is_offline()
}

/// Set the idle thread.
/// # Safety
/// - `thread` must be a valid `RawThread`.
/// - `thread` must not be present in any ready queue.
unsafe fn set_idle_paused(token: PauseToken, thread: RawThread) {
    unsafe {
        // Set idle flag in thread flags
        // SAFETY: thread is guaranteed to be valid by caller
        thread.tcb().flags.set_idle(token);
    }

    let mut rings = SCHEDULER.rings.borrow_ref_mut(token);
    debug_assert!(rings.idle.is_none(), "set_idle can only be called once!");
    rings.idle = Some(thread);
}

/// Add a thread to the ready queue. Internally acquires a scheduler pause.
/// # Safety
/// Same as `add_ready_paused`.
pub(crate) unsafe fn add_ready(thread: RawThread) {
    with_pause(|token| unsafe {
        add_ready_paused(token, thread);
    });
}

/// Add a new thread to the ready queue. Can only be called with scheduler paused.
/// # Safety
/// - `thread` must be a valid `RawThread`.
/// - `thread` must not be present in any ready queue.
pub(crate) unsafe fn add_ready_paused(token: PauseToken, thread: RawThread) {
    debug_assert!(
        unsafe { !thread.is_idle_paused(token) },
        "cannot add to ready queue idle thread"
    );

    let prio = unsafe {
        // SAFETY: thread is guaranteed to be valid by the caller
        thread.priority()
    };

    let mut rings = SCHEDULER.rings.borrow_ref_mut(token);
    let mut ring = get_ring_at_prio_mut(&mut rings, prio);
    if let Some(ring) = *ring {
        unsafe {
            // SAFETY: thread is guaranteed to be valid by the caller
            thread.link_before(token, ring);
        }
    } else {
        *ring = Some(thread);
        // This ring was previously empty, also update the mask
        rings.prio_mask |= (1 << prio.value() as u32);
    }
}

/// Remove a thread from the ready queue. Internally acquires a scheduler pause.
/// # Safety
/// Same as `remove_ready_paused`.
pub(crate) unsafe fn remove_ready(thread: RawThread) {
    with_pause(|token| unsafe {
        remove_ready_paused(token, thread);
    });
}

/// Remove a thread from the ready queue. Can only be called with scheduler paused.
/// # Safety
/// - `thread` must be a valid `RawThread`.
/// - `thread` must be present in its respective ready queue.
pub(crate) unsafe fn remove_ready_paused(token: PauseToken, thread: RawThread) {
    debug_assert!(
        unsafe { !thread.is_idle_paused(token) },
        "cannot remove from ready queue idle thread"
    );

    let prio = unsafe {
        // SAFETY: thread is guaranteed to be valid by the caller
        thread.priority()
    };

    let next = unsafe {
        // SAFETY: thread is guaranteed to be valid by the caller
        thread.unlink(token)
    };

    let mut rings = SCHEDULER.rings.borrow_ref_mut(token);
    let mut ring = get_ring_at_prio_mut(&mut rings, prio);
    if *ring == Some(thread) {
        // This thread WAS the head, update the head!
        *ring = next;

        if next.is_none() {
            // The queue is actually empty now, update the mask
            rings.prio_mask &= !(1 << prio.value() as u32);
        }
    }
}

/// Runs the main scheduling algorithm, respects scheduler pauses.
/// # Safety
/// Non preemption must be guaranteed externally by the caller.
pub unsafe fn run_scheduler() {
    if SCHEDULER.state.is_paused() {
        SCHEDULER.state.set_sched_pend();
        return;
    }

    let token = unsafe {
        // SAFETY: This is the scheduler function, so we can safely access scheduler data structures
        PauseToken::new()
    };

    // We are currently running the scheduler, clear any pending reschedule
    SCHEDULER.state.clear_sched_pend();

    let mut rings = SCHEDULER.rings.borrow_ref_mut(token);
    let ready = if rings.prio_mask == 0 {
        // The priority mask is empty, no thread is ready, just return the idle one
        unsafe {
            debug_assert!(rings.idle.is_some(), "no idle thread has been installed");
            // SAFETY: enter() will install an idle thread before unpausing the scheduler
            rings.idle.unwrap_unchecked()
        }
    } else {
        // Find the highest priority non empty queue
        let prio = (31 - rings.prio_mask.leading_zeros() as u8);
        let prio = unsafe {
            debug_assert!(
                prio >= MIN_PRIORITY && prio <= MAX_PRIORITY,
                "invalid priority in prio_mask"
            );
            // SAFETY: prio will always be between 0 and 31
            Priority::new_unchecked(prio)
        };

        let ring = get_ring_at_prio_mut(&mut rings, prio);
        let ready = unsafe {
            debug_assert!(
                ring.is_some(),
                "prio_mask indicates non empty ring, but ring is empty"
            );
            // SAFETY: prio_mask always indicates non-empty rings
            ring.unwrap_unchecked()
        };

        let ready = unsafe { ready.tcb().next.get(token) };

        *ring = Some(ready);
        ready
    };

    debug_assert!(
        unsafe { ready.tcb().flags.ready(token) },
        "selected thread is not actually ready!"
    );

    unsafe {
        // SAFETY: ready will always be a valid RawThread, freed thread will always be removed from the respective queue
        set_current_paused(token, ready);
    }
}

/// Enter the scheduler, should only be called with the kernel paused.
/// # Safety
/// Can only be called once, during boot.
pub unsafe fn enter() -> ! {
    debug_assert!(
        is_offline(),
        "the scheduler can only be entered if it is offline"
    );

    const IDLE_STACK: usize = 4096;

    unsafe extern "C" fn idle_entry() -> ! {
        idle_task()
    }

    let thread = RawThread::create(IDLE_STACK);

    {
        let token = unsafe {
            // SAFETY: The scheduler is offline
            PauseToken::new()
        };

        unsafe {
            // Set this new thread as the idle one
            set_idle_paused(token, thread);
            // Set this thread as the current one, as we are about to run it
            set_current_paused(token, thread);
        }
    }

    unsafe {
        // Bring the scheduler online
        SCHEDULER.state.online();
        // Enter the thread
        thread.enter_first_thread(idle_entry)
    }
}

/// Internal function to run inside the idle thread.
fn idle_task() -> ! {
    loop {
        yield_now();
    }
}
