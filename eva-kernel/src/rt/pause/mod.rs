use core::sync::atomic::{AtomicBool, Ordering, compiler_fence};

use crate::rt;

use scopeguard::defer;

mod cell;

pub use cell::PauseCell;
pub use eva_abi::PauseToken;

static PAUSED: AtomicBool = AtomicBool::new(false);
static PEND: AtomicBool = AtomicBool::new(false);

/// Check if the scheduler is paused.
#[unsafe(export_name = "eva_rt_is_paused")]
pub fn is_paused() -> bool {
    // TODO(davide.mor): Review memory ordering
    PAUSED.load(Ordering::SeqCst)
}

/// Force a scheduler pause.
#[unsafe(export_name = "eva_rt_pause")]
pub fn pause() {
    compiler_fence(Ordering::SeqCst);
    // TODO(davide.mor): Review memory ordering
    PAUSED.store(true, Ordering::SeqCst);
}

/// Force unpause the scheduler.
/// # Safety
/// - No `PauseToken` must exist at this point.
#[unsafe(export_name = "eva_rt_unpause")]
pub unsafe fn unpause() {
    // TODO(davide.mor): Review memory ordering
    PAUSED.store(false, Ordering::SeqCst);

    // TODO(davide.mor): Review memory ordering
    if PEND.load(Ordering::SeqCst) {
        // We have a pended yield
        rt::yield_now();
    }

    compiler_fence(Ordering::SeqCst);
}

/// Pend a yield to happen as soon as the pause is ended.
#[unsafe(export_name = "eva_rt_pend_yield")]
pub fn pend_yield(_token: PauseToken) {
    // TODO: Check current status?
    // TODO(davide.mor): Review memory ordering
    PEND.store(true, Ordering::SeqCst);
}

/// Try to pause the scheduler.
#[unsafe(export_name = "eva_rt_try_pause")]
pub fn try_pause() -> bool {
    // TODO(davide.mor): Review memory ordering
    if PAUSED.load(Ordering::SeqCst) {
        false
    } else {
        compiler_fence(Ordering::SeqCst);
        // TODO(davide.mor): Review memory ordering
        PAUSED.store(true, Ordering::SeqCst);
        true
    }
}

/// Try to unpause the scheduler.
/// # Safety
/// - No `PauseToken` must exist at this point.
#[unsafe(export_name = "eva_rt_try_unpause")]
pub unsafe fn try_unpause() -> bool {
    // TODO(davide.mor): Review memory ordering
    if PAUSED.load(Ordering::SeqCst) {
        // Scheduler was not paused previously
        false
    } else {
        // TODO(davide.mor): Review memory ordering
        PAUSED.store(true, Ordering::SeqCst);

        // TODO(davide.mor): Review memory ordering
        if PEND.load(Ordering::SeqCst) {
            rt::yield_now();
        }

        compiler_fence(Ordering::SeqCst);
        true
    }
}

pub(super) fn run_scheduler<F>(f: F)
where
    F: FnOnce(PauseToken),
{
    // TODO(davide.mor): Review memory ordering
    if PAUSED.load(Ordering::SeqCst) {
        // The system is already paused, pend
        // TODO(davide.mor): Review memory ordering
        PEND.store(true, Ordering::SeqCst);
    } else {
        // Enable the paused state
        // TODO(davide.mor): Review memory ordering
        PAUSED.store(true, Ordering::SeqCst);

        // Clear any pending state unconditionally
        // TODO(davide.mor): Review memory ordering
        PEND.store(false, Ordering::SeqCst);

        defer! {
            // TODO(davide.mor): Review memory ordering
            PAUSED.store(false, Ordering::SeqCst);
        }

        // We managed to pause the scheduler
        f(unsafe {
            // SAFETY: The scheduler is paused at this point
            PauseToken::new()
        })
    }
}

/// Enter a critical section using a scheduler pause.
pub fn with_pause<F, T>(f: F) -> T
where
    F: FnOnce(PauseToken) -> T,
{
    let has_paused = try_pause();

    defer! {
        if has_paused {
            unsafe {
                // SAFETY: We destroyed our token, so now we can safely unpause
                unpause();
            }
        }
    }

    f(unsafe {
        // SAFETY: The scheduler is paused at this point
        PauseToken::new()
    })
}

/// Yield now from a paused state, first releasing the pause, yielding, and resuming.
pub fn yield_now_from_paused(_token: PauseToken) {
    unsafe {
        // SAFETY: We did not destroy the token, but no code is allowed to access any PauseCell
        // during the following section
        unpause();
    }

    defer! {
        pause();
    }

    rt::yield_now();
}

/// Run the given function only if the scheduler is already paused.
pub fn if_paused<F, T>(f: F) -> Option<T>
where
    F: FnOnce(PauseToken) -> T,
{
    if is_paused() {
        Some(f(unsafe {
            // SAFETY: The scheduler is paused at this point
            PauseToken::new()
        }))
    } else {
        None
    }
}
