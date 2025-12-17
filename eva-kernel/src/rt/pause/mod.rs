use core::sync::atomic::{AtomicU8, Ordering};

use crate::rt;
use crate::utils::scopeguard::defer;

mod cell;

pub use cell::PauseCell;
pub use eva_abi::PauseToken;

const STATE_PEND_BIT: u8 = 1 << 0;
const STATE_PAUSED_BIT: u8 = 1 << 1;

static STATE: AtomicU8 = AtomicU8::new(0);

/// Check if the scheduler is paused.
#[unsafe(export_name = "eva_rt_is_paused")]
pub fn is_paused() -> bool {
    // TODO(davide.mor): Review memory ordering
    (STATE.load(Ordering::SeqCst) & STATE_PAUSED_BIT) != 0
}

/// Try to pause the scheduler.
#[unsafe(export_name = "eva_rt_try_pause")]
pub fn try_pause() -> bool {
    // TODO(davide.mor): Review memory ordering
    let state = STATE.load(Ordering::SeqCst);
    if (state & STATE_PAUSED_BIT) != 0 {
        return false;
    }

    // TODO(davide.mor): Review memory ordering
    STATE.store(STATE_PAUSED_BIT, Ordering::SeqCst);
    true
}

/// Try to pause the scheduler, or pend a yield.
fn try_pause_or_pend() -> bool {
    // TODO(davide.mor): Review memory ordering
    let state = STATE.load(Ordering::SeqCst);
    if (state & STATE_PAUSED_BIT) != 0 {
        // TODO(davide.mor): Review memory ordering
        STATE.store(STATE_PAUSED_BIT | STATE_PEND_BIT, Ordering::SeqCst);
        return false;
    }

    // TODO(davide.mor): Review memory ordering
    STATE.store(STATE_PAUSED_BIT, Ordering::SeqCst);
    true
}

/// Try to unpause the scheduler.
/// # Safety
/// - No `PauseToken` must exist at this point.
#[unsafe(export_name = "eva_rt_try_unpause")]
pub unsafe fn try_unpause() -> bool {
    // TODO(davide.mor): Review memory ordering
    let state = STATE.load(Ordering::SeqCst);
    if (state & STATE_PAUSED_BIT) == 0 {
        return false;
    }

    // TODO(davide.mor): Review memory ordering
    STATE.store(0, Ordering::SeqCst);

    if (state & STATE_PEND_BIT) != 0 {
        rt::yield_now();
    }

    true
}

/// Pend a yield to happen as soon as the pause is ended.
#[unsafe(export_name = "eva_rt_pend_yield")]
pub fn pend_yield(_token: PauseToken) {
    // TODO: Check current status?
    // TODO(davide.mor): Review memory ordering
    STATE.store(STATE_PAUSED_BIT | STATE_PEND_BIT, Ordering::SeqCst);
}

pub(super) fn run_scheduler<F>(f: F)
where
    F: FnOnce(PauseToken),
{
    if try_pause_or_pend() {
        defer! {
            // TODO(davide.mor): Review memory ordering
            STATE.store(0, Ordering::SeqCst);
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
                assert!(try_unpause(), "kernel was already unpaused");
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
        assert!(try_unpause(), "kernel was already unpaused");
    }

    defer! {
        assert!(try_pause(), "kernel was not unpaused properly");
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
