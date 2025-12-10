use core::fmt;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicU8, Ordering};

use crate::rt;
use crate::utils::scopeguard::defer;

mod cell;

pub use cell::PauseCell;

/// An error indicating that the scheduler was already paused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlreadyPaused;

/// An error indicating that the scheduler was not paused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotPaused;

const STATE_PEND_BIT: u8 = 1 << 0;
const STATE_PAUSED_BIT: u8 = 1 << 1;

static STATE: AtomicU8 = AtomicU8::new(0);

/// Check if the scheduler is paused.
pub fn is_paused() -> bool {
    // TODO(davide.mor): Review memory ordering
    (STATE.load(Ordering::SeqCst) & STATE_PAUSED_BIT) != 0
}

/// Try to pause the scheduler.
pub fn try_pause() -> Result<(), AlreadyPaused> {
    // TODO(davide.mor): Review memory ordering
    let state = STATE.load(Ordering::SeqCst);
    if (state & STATE_PAUSED_BIT) != 0 {
        return Err(AlreadyPaused);
    }

    // TODO(davide.mor): Review memory ordering
    STATE.store(STATE_PAUSED_BIT, Ordering::SeqCst);
    Ok(())
}

/// Try to pause the scheduler, or pend a yield.
pub fn try_pause_or_pend() -> Result<(), AlreadyPaused> {
    // TODO(davide.mor): Review memory ordering
    let state = STATE.load(Ordering::SeqCst);
    if (state & STATE_PAUSED_BIT) != 0 {
        // TODO(davide.mor): Review memory ordering
        STATE.store(STATE_PAUSED_BIT | STATE_PEND_BIT, Ordering::SeqCst);
        return Err(AlreadyPaused);
    }

    // TODO(davide.mor): Review memory ordering
    STATE.store(STATE_PAUSED_BIT, Ordering::SeqCst);
    Ok(())
}

/// Try to unpause the scheduler.
/// # Safety
/// - No `PauseToken` must exist at this point.
pub unsafe fn try_unpause() -> Result<(), NotPaused> {
    // TODO(davide.mor): Review memory ordering
    let state = STATE.load(Ordering::SeqCst);
    if (state & STATE_PAUSED_BIT) == 0 {
        return Err(NotPaused);
    }

    // TODO(davide.mor): Review memory ordering
    STATE.store(0, Ordering::SeqCst);

    if (state & STATE_PEND_BIT) != 0 {
        rt::yield_now();
    }

    Ok(())
}

/// Pend a yield to happen as soon as the pause is ended.
pub fn pend_yield(_token: PauseToken) {
    // TODO: Check current status?
    // TODO(davide.mor): Review memory ordering
    STATE.store(STATE_PAUSED_BIT | STATE_PEND_BIT, Ordering::SeqCst);
}

pub(super) fn run_scheduler<F>(f: F)
where
    F: FnOnce(PauseToken),
{
    if try_pause_or_pend().is_ok() {
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
    let has_paused = try_pause().is_ok();

    defer! {
        if has_paused {
            unsafe {
                // SAFETY: We destroyed our token, so now we can safely unpause
                try_unpause().expect("kernel was already unpaused");
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
        try_unpause().expect("kernel was already unpaused");
    }

    defer! {
        let _ = try_pause();
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

/// Token representing the paused state of the scheduler, existence of this token proves that the scheduler is paused.
#[derive(Clone, Copy)]
pub struct PauseToken<'a> {
    // Token is COVARIANT over 'a!
    // It means that a shorter token is a subtype of a longer token!
    _marker: PhantomData<&'a ()>,
    _not_send_sync: PhantomData<*mut ()>,
}

impl PauseToken<'_> {
    /// Create a new `PauseToken`.
    /// # Safety
    /// Caller must guarantee that the scheduler will remain paused as long as this token exists.
    pub unsafe fn new() -> Self {
        Self {
            _marker: PhantomData,
            _not_send_sync: PhantomData,
        }
    }
}

impl fmt::Debug for PauseToken<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PauseToken")
    }
}
