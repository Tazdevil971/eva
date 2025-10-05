use core::fmt;
use core::marker::PhantomData;

use crate::scheduler::{self, thread};

mod cell;

pub use cell::PauseCell;

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
                scheduler::unpause();
            }
        }
    }

    let _guard = scheduler::try_pause().map(|_| UnpauseGuard);
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
            let _ = scheduler::try_pause();
        }
    }

    unsafe {
        // SAFETY: We did not destroy the token, but no code is allowed to access any PauseCell
        // during the following section
        scheduler::unpause();
    }

    let _guard = PauseGuard;
    thread::yield_now();
}

/// Run the given function only if the scheduler is already paused.
pub fn if_paused<F, T>(f: F) -> Option<T>
where
    F: FnOnce(PauseToken) -> T,
{
    if scheduler::is_paused() {
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
