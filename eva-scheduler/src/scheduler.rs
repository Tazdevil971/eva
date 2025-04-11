use core::cell::RefCell;
use core::ptr;
use core::sync::atomic::Ordering;

use crate::pause::{self, PauseCell, PauseToken, with_pause};
use crate::{kprintln, priority};
use crate::raw_thread::{self, AtomicRawThread, RawThread};

pub use crate::priority::Priority;

/// Internal mutable thread structure.
struct Rings {
    /// Bitmask representing queues that are not empty.
    set: priority::Bitset,
    prio: priority::Array<Option<RawThread>>,
    /// Idle thread.
    idle: Option<RawThread>,
}

/// Scheduler data structure
struct Scheduler {
    /// Thread holding structure.
    rings: PauseCell<RefCell<Rings>>,
    /// Current thread.
    cur: AtomicRawThread,
}

static SCHEDULER: Scheduler = Scheduler {
    rings: PauseCell::new(RefCell::new(Rings {
        set: priority::Bitset::new(),
        prio: priority::Array::new([None; 32]),
        idle: None,
    })),
    cur: AtomicRawThread::new(None),
};

/// Set the currently active thread.
unsafe fn set_current_paused(_token: PauseToken, thread: RawThread) {
    unsafe {
        // Set the global switch context pointer
        // SAFETY: This can only be called with a paused scheduler
        thread.set_global_switchctx();
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
/// - `thread` must not be the idle thread.
pub(crate) unsafe fn add_ready_paused(token: PauseToken, thread: RawThread) {
    debug_assert!(
        unsafe { !thread.tcb().flags.idle(token) },
        "cannot add to ready queue idle thread"
    );

    let prio = unsafe {
        // SAFETY: thread is guaranteed to be valid by the caller
        thread.priority()
    };

    let mut rings = SCHEDULER.rings.borrow_ref_mut(token);
    let ring = &mut rings.prio[prio];

    if let Some(ring) = *ring {
        unsafe {
            // SAFETY: thread is guaranteed to be valid by the caller
            thread.link_before(token, ring);
        }
    } else {
        *ring = Some(thread);
        // This ring was previously empty, also update the mask
        rings.set.insert(prio);
    }
}

/// Remove a thread from the ready queue. Can only be called with scheduler paused.
/// # Safety
/// - `thread` must be a valid `RawThread`.
/// - `thread` must be present in its respective ready queue.
pub(crate) unsafe fn remove_ready_paused(token: PauseToken, thread: RawThread) {
    debug_assert!(
        unsafe { !thread.tcb().flags.idle(token) },
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
    let ring = &mut rings.prio[prio];

    if *ring == Some(thread) {
        // This thread WAS the head, update the head!
        *ring = next;

        if next.is_none() {
            // The queue is actually empty now, update the mask
            rings.set.remove(prio);
        }
    }
}

pub(crate) fn run_scheduler_paused(token: PauseToken) {
    let mut rings = SCHEDULER.rings.borrow_ref_mut(token);
    let ready = if let Some(highest) = rings.set.highest() {
        let ring = &mut rings.prio[highest];

        // Retrieve the head
        let ready = unsafe {
            debug_assert!(
                ring.is_some(),
                "prio_mask indicates non empty ring, but ring is empty"
            );
            // SAFETY: prio_mask always indicates non-empty rings
            ring.unwrap_unchecked()
        };

        // Rotate the queue to get the next
        let ready = unsafe {
            // SAFETY: The thread is ready, so it must be valid
            ready.tcb().next.get(token)
        };

        // Set the new head
        *ring = Some(ready);
        ready
    } else {
        // The priority set is empty, no thread is ready, just return the idle one
        unsafe {
            debug_assert!(rings.idle.is_some(), "no idle thread has been installed");
            // SAFETY: enter() will install an idle thread before unpausing the scheduler
            rings.idle.unwrap_unchecked()
        }
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

/// Runs the main scheduling algorithm, respects scheduler pauses.
/// # Safety
/// Non preemption must be guaranteed externally by the caller.
pub unsafe fn scheduler_tick() {
    unsafe {
        // SAFETY: This is the scheduler function!
        pause::run_scheduler(|token| {
            run_scheduler_paused(token);
        });
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

    let idle = RawThread::create(IDLE_STACK, Priority::MIN);
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
}

/// Internal function to run inside the idle thread.
fn idle_task() -> ! {
    loop {
        raw_thread::yield_now();
    }
}
