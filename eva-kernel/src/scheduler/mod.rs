/// Scheduler pause primitives.
pub mod pause;
/// Synchronization primitives.
pub mod sync;
/// Low level threading interface.
pub mod thread;
/// Time driver.
pub mod time;

/// Internal panic hook for the scheduler.
mod panic;
/// Scheduler implementation.
mod sched;

use core::ptr;
use core::sync::atomic::{AtomicU8, Ordering};

use pause::{PauseCell, PauseToken, with_pause};
use sched::Sched;
use thread::{AtomicThreadPtr, ThreadPtr};

use crate::utils::assert::harden_assert;

pub const IDLE_PRIORITY: i8 = -1;
pub const MIN_PRIORITY: i8 = 0;
pub const MAX_PRIORITY: i8 = 31;

pub fn is_valid_prio(prio: i8) -> bool {
    prio >= MIN_PRIORITY && prio <= MAX_PRIORITY
}

const STATE_RUNNING_BIT: u8 = 1 << 0;
const STATE_PAUSED_BIT: u8 = 1 << 1;
const STATE_PEND_BIT: u8 = 1 << 2;

/// Scheduler data structure
struct Scheduler {
    /// Internal scheduler state
    state: AtomicU8,
    /// Thread holding structure.
    sched: PauseCell<Sched>,
    /// Current thread.
    cur: AtomicThreadPtr,
}

static SCHEDULER: Scheduler = Scheduler {
    state: AtomicU8::new(0),
    sched: PauseCell::new(Sched::new()),
    cur: AtomicThreadPtr::new(None),
};

/// Error returned in case the scheduler was already paused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlreadyPausedError;

#[inline(always)]
pub fn is_running() -> bool {
    // TODO(davide.mor): Review memory ordering here
    let state = SCHEDULER.state.load(Ordering::SeqCst);
    (state & STATE_RUNNING_BIT) != 0
}

/// Check if the scheduler is paused.
#[inline(always)]
pub fn is_paused() -> bool {
    let state = SCHEDULER.state.load(Ordering::SeqCst);
    (state & STATE_PAUSED_BIT) != 0
}

/// Try to pause the scheduler.
#[inline(always)]
pub fn try_pause() -> Result<(), AlreadyPausedError> {
    // TODO(davide.mor): Review memory ordering here
    let old_state = SCHEDULER.state.fetch_or(STATE_PAUSED_BIT, Ordering::SeqCst);
    if (old_state & STATE_PAUSED_BIT) != 0 {
        Err(AlreadyPausedError)
    } else {
        Ok(())
    }
}

/// Unpause the scheduler.
/// # Safety
/// - No `PauseToken` instances should exist anymore.
/// - The scheduler must be in the paused state.
pub unsafe fn unpause() {
    harden_assert!(is_paused(), "scheduler not paused");

    // TODO(davide.mor): Review memory ordering here
    let old_state = SCHEDULER
        .state
        .fetch_and(!(STATE_PAUSED_BIT | STATE_PEND_BIT), Ordering::SeqCst);
    if (old_state & STATE_PEND_BIT) != 0 {
        thread::yield_now();
    }
}

/// Set a pause pending, so that a reschedule is initiated as soon as the pause ends.
#[inline(always)]
pub fn pend_yield(_token: PauseToken) {
    harden_assert!(is_paused(), "scheduler not paused");

    // TODO(davide.mor): Review memory ordering here
    SCHEDULER.state.fetch_or(STATE_PEND_BIT, Ordering::SeqCst);
}

/// Shutdown the scheduler due to a panic
fn panic_shutdown() {
    SCHEDULER.state.store(0, Ordering::SeqCst);
}

/// Set the currently active thread.
unsafe fn set_current_paused(_token: PauseToken, thread: ThreadPtr) {
    unsafe {
        // Set the global switch context pointer
        // SAFETY: This can only be called with a paused scheduler
        thread.set_as_global_switchctx();
    }
    // Set the global variable pointing to our thread
    // TODO(davide.mor): Review memory ordering here
    SCHEDULER.cur.store(Some(thread), Ordering::Relaxed);
}

/// Query the currently active thread.
fn current_thread() -> ThreadPtr {
    // This is an expect rather than an assert + unwrap_unchecked in order to make this function safe

    // TODO(davide.mor): Review memory ordering here
    SCHEDULER
        .cur
        .load(Ordering::Relaxed)
        .expect("No current thread, scheduler is not running!")
}

/// Add a thread to the ready queue. Internally acquires a scheduler pause.
/// # Safety
/// Same as `add_ready_paused`.
unsafe fn add_ready(thread: ThreadPtr) {
    with_pause(|token| unsafe {
        add_ready_paused(token, thread);
    });
}

/// Add a new thread to the ready queue. Can only be called with scheduler paused.
/// # Safety
/// - `thread` must be a valid `ThreadPtr`.
/// - `thread` must not be present in any ready queue.
/// - `thread` must not be the idle thread.
unsafe fn add_ready_paused(token: PauseToken, thread: ThreadPtr) {
    harden_assert!(
        unsafe { thread.priority() != IDLE_PRIORITY },
        "cannot add to ready queue idle thread"
    );

    harden_assert!(
        unsafe { thread.tcb().flags.ready(token) },
        "cannot add to ready queue a non-ready thread"
    );

    unsafe {
        SCHEDULER.sched.borrow(token).add_thread(thread);
    }
}

fn run_scheduler_paused(token: PauseToken) {
    let cur = unsafe { SCHEDULER.cur.load(Ordering::Relaxed).unwrap_unchecked() };
    if unsafe { cur.tcb().flags.ready(token) } {
        unsafe {
            SCHEDULER.sched.borrow(token).add_thread(cur);
        }
    }

    let next = unsafe { SCHEDULER.sched.borrow(token).pop_next_thread() };

    harden_assert!(
        unsafe { next.tcb().flags.ready(token) },
        "selected thread is not actually ready!"
    );

    unsafe {
        // SAFETY: ready will always be a valid ThreadPtr, freed thread will always be removed from the respective queue
        set_current_paused(token, next);
    }
}

/// Runs the main scheduling algorithm, respects scheduler pauses.
/// # Safety
/// Non preemption must be guaranteed externally by the caller.
pub unsafe fn tick() {
    // TODO(davide.mor): Review memory ordering here
    let state = SCHEDULER.state.load(Ordering::SeqCst);
    if (state & STATE_RUNNING_BIT) == 0 {
        // The scheduler is offline, just return
        return;
    }

    if (state & STATE_PAUSED_BIT) != 0 {
        // Some thread has paused the system, pend a yield and do nothing
        SCHEDULER
            .state
            .store(state | STATE_PEND_BIT, Ordering::SeqCst);
    } else {
        // The scheduler is actually running
        let token = unsafe {
            // SAFETY: The scheduler is unpaused, but this IS the scheduler, we are guaranteeing that no-one else has the token, so we can have it
            PauseToken::new()
        };

        let now = crate::time::get_time();

        time::run_time_driver_paused(token, now);
        run_scheduler_paused(token);
    }
}

/// Initialize the scheduler, should only be called with the kernel paused.
/// # Safety
/// Can only be called once, during boot.
pub unsafe fn init() {
    const IDLE_STACK: usize = 4096;

    unsafe extern "C" fn idle_entry(_: *mut ()) {
        idle_task();
    }

    let idle = ThreadPtr::create(IDLE_STACK, IDLE_PRIORITY);
    unsafe {
        idle.init(idle_entry, ptr::null_mut());
    }

    let token = unsafe {
        // SAFETY: The scheduler is offline
        PauseToken::new()
    };

    unsafe {
        // Set the newly created thread as the idle one
        SCHEDULER.sched.borrow(token).set_idle(idle);
    }

    unsafe {
        // Set this thread as the current one, as we are about to run it
        set_current_paused(token, idle);
    }

    // Now we can finally set the scheduler as running
    // TODO(davide.mor): Review memory ordering here
    SCHEDULER.state.store(STATE_RUNNING_BIT, Ordering::SeqCst);
}

/// Internal function to run inside the idle thread.
fn idle_task() -> ! {
    thread::yield_now();
    loop {}
}
