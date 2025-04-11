use core::fmt;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicU8, Ordering};

use crate::raw_thread;
use crate::utils::assert::harden_assert;

mod cell;
mod cs;

pub use cell::PauseCell;

const STATE_PEND_BIT: u8 = 1 << 0;
const STATE_PAUSED_BIT: u8 = 1 << 1;
static STATE: AtomicU8 = AtomicU8::new(0);

/// Error returned in case the scheduler was already paused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlreadyPausedError;

/// Check if the scheduler is paused.
#[inline(always)]
pub fn is_paused() -> bool {
    let state = STATE.load(Ordering::SeqCst);
    (state & STATE_PAUSED_BIT) != 0
}

/// Try to pause the scheduler.
#[inline(always)]
pub fn try_pause() -> Result<(), AlreadyPausedError> {
    // TODO(davide.mor): Review memory ordering here
    let old_state = STATE.fetch_or(STATE_PAUSED_BIT, Ordering::SeqCst);
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
    let old_state = STATE.swap(0, Ordering::SeqCst);
    if (old_state & STATE_PEND_BIT) != 0 {
        raw_thread::yield_now();
    }
}

/// Set a pause pending, so that a reschedule is initiated as soon as the pause ends.
#[inline(always)]
pub fn pend_yield(_token: PauseToken) {
    // TODO(davide.mor): Review memory ordering here
    STATE.store(STATE_PAUSED_BIT | STATE_PEND_BIT, Ordering::SeqCst);
}

/// Internal, used to run the scheduler routine.
pub(crate) unsafe fn run_scheduler<F>(f: F)
where
    F: FnOnce(PauseToken),
{
    // TODO(davide.mor): Review memory ordering here
    let state = STATE.load(Ordering::SeqCst);
    if (state & STATE_PAUSED_BIT) != 0 {
        // Some thread has paused the system, pend a yield and do nothing
        STATE.store(STATE_PAUSED_BIT | STATE_PEND_BIT, Ordering::SeqCst);
    } else {
        f(unsafe {
            // SAFETY: The scheduler is unpaused, but this IS the scheduler, we are guaranteeing that no-one else has the token, so we can have it
            PauseToken::new()
        })
    }
}

/// Enter a critical section using a scheduler pause.
pub fn with_pause<F, T>(f: F) -> T
where
    F: FnOnce(PauseToken) -> T,
{
    struct UnpauseGuard;

    impl Drop for UnpauseGuard {
        fn drop(&mut self) {
            unsafe {
                // SAFETY: We destroyed our token, so now we can safely unpause
                unpause();
            }
        }
    }

    let _guard = try_pause().map(|_| UnpauseGuard);
    f(unsafe {
        // SAFETY: The scheduler is paused at this point
        PauseToken::new()
    })
}

/// Yield now from a paused state, first releasing the pause, yielding, and resuming.
pub fn yield_now_from_paused(_token: PauseToken) {
    struct PauseGuard;

    impl Drop for PauseGuard {
        fn drop(&mut self) {
            let _ = try_pause();
        }
    }

    unsafe {
        // SAFETY: We did not destroy the token, but no code is allowed to access any PauseCell
        // during the following section
        unpause();
    }

    let _guard = PauseGuard;
    raw_thread::yield_now();
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
