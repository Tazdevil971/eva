/// Scheduler pause primitives.
pub mod pause;
/// Low level threading interface.
pub mod thread;
// Time is WIP
// /// Time driver.
// pub mod time;
/// Synchronization primitives.
pub mod sync;

use core::cell::RefCell;
use core::ptr;
use core::sync::atomic::{AtomicU8, Ordering};

use pause::{PauseCell, PauseToken, with_pause};
use thread::{AtomicThreadPtr, ThreadList, ThreadPtr};

use crate::utils::assert::harden_assert;
use crate::utils::bitset::Bitset32;
use crate::utils::linked_list::HeapLinkedList;

fn is_valid_prio(prio: i8) -> bool {
    prio >= 0 && prio <= 31
}

/// Internal mutable thread structure.
struct Rings {
    /// Bitmask representing queues that are not empty.
    set: Bitset32,
    prio: [HeapLinkedList<ThreadList>; 32],
    /// Idle thread.
    idle: Option<ThreadPtr>,
}

impl Rings {
    unsafe fn add_thread(&mut self, thread: ThreadPtr) {
        let prio = unsafe { thread.priority() };

        unsafe {
            harden_assert!(is_valid_prio(prio), "invalid priority");
            self.prio.get_unchecked(prio as usize).push_back(thread);
        }

        self.set.insert(prio as usize);
    }

    unsafe fn pop_thread(&mut self) -> ThreadPtr {
        if let Some(prio) = self.set.highest() {
            let thread = unsafe {
                self.prio.get_unchecked(prio)
                    .pop_front()
            };
            
            let thread = unsafe {
                harden_assert!(thread.is_some(), "");
                thread.unwrap_unchecked()
            };

            

            thread
        } else {
            unsafe {
                harden_assert!(self.idle.is_some(), "no idle thread installed");
                self.idle.unwrap_unchecked()
            }
        }
    }
}

const STATE_PAUSED_BIT: u8 = 1 << 0;
const STATE_RUNNING_BIT: u8 = 1 << 1;
const STATE_PEND_BIT: u8 = 1 << 2;

/// Scheduler data structure
struct Scheduler {
    /// Internal scheduler state
    state: AtomicU8,
    /// Thread holding structure.
    rings: PauseCell<RefCell<Rings>>,
    /// Current thread.
    cur: AtomicThreadPtr,
}

static SCHEDULER: Scheduler = Scheduler {
    state: AtomicU8::new(0),
    rings: PauseCell::new(RefCell::new(Rings {
        set: Bitset32::empty(),
        prio: [const { HeapLinkedList::empty() }; 32],
        idle: None,
    })),
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
        unsafe { !thread.tcb().flags.idle(token) },
        "cannot add to ready queue idle thread"
    );

    let mut rings = SCHEDULER.rings.borrow_ref_mut(token);

    let prio = unsafe { thread.priority() };
    harden_assert!(is_valid_prio(prio), "invalid priority");

    unsafe {
        rings.prio.get_unchecked(prio as usize).push_back(thread);
    }

    rings.set.insert(prio as usize);
}

fn run_scheduler_paused(token: PauseToken) {
    // First put back the currently executing thread
    let cur = unsafe { SCHEDULER.cur.load(Ordering::Relaxed).unwrap_unchecked() };

    let mut rings = SCHEDULER.rings.borrow_ref_mut(token);

    if unsafe { !cur.tcb().flags.idle(token) && cur.tcb().flags.ready(token) } {
        let prio = unsafe { cur.priority() };
        harden_assert!(is_valid_prio(prio), "invalid priority");

        unsafe {
            rings.prio.get_unchecked(prio as usize).push_back(cur);
        }

        rings.set.insert(prio as usize);
    }

    let ready = if let Some(highest) = rings.set.highest() {
        let ring = &mut rings.prio[highest];

        // Retrieve the head
        let ready = unsafe {
            harden_assert!(
                !ring.is_empty(),
                "prio_mask indicates non empty ring, but ring is empty"
            );

            // SAFETY: prio_mask always indicates non-empty rings
            ring.pop_front().unwrap_unchecked()
        };

        if ring.is_empty() {
            rings.set.remove(highest);
        }

        ready
    } else {
        // The priority set is empty, no thread is ready, just return the idle one
        unsafe {
            harden_assert!(rings.idle.is_some(), "no idle thread has been installed");
            // SAFETY: enter() will install an idle thread before starting the scheduler
            rings.idle.unwrap_unchecked()
        }
    };

    harden_assert!(
        unsafe { ready.tcb().flags.ready(token) },
        "selected thread is not actually ready!"
    );

    unsafe {
        // SAFETY: ready will always be a valid ThreadPtr, freed thread will always be removed from the respective queue
        set_current_paused(token, ready);
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
        SCHEDULER.state.store(STATE_PEND_BIT, Ordering::SeqCst);
    } else {
        // The scheduler is actually running
        let token = unsafe {
            // SAFETY: The scheduler is unpaused, but this IS the scheduler, we are guaranteeing that no-one else has the token, so we can have it
            PauseToken::new()
        };

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

    let idle = ThreadPtr::create(IDLE_STACK, -1);
    unsafe {
        idle.init(idle_entry, ptr::null_mut());
    }

    let token = unsafe {
        // SAFETY: The scheduler is offline
        PauseToken::new()
    };

    unsafe {
        // Set idle flag in thread flags
        // SAFETY: thread is valid.
        idle.tcb().flags.set_idle(token);
    }

    // Set the newly created thread as the idle one
    SCHEDULER.rings.borrow_ref_mut(token).idle = Some(idle);

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
    loop {
        thread::yield_now();
    }
}
