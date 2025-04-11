use crate::scheduler::{MAX_PRIORITY, MIN_PRIORITY};

/// Internal function used for checking if a priority is valid.
const fn is_valid(prio: u8) -> bool {
    prio >= MIN_PRIORITY && prio <= MAX_PRIORITY
}

/// Wrapper around a priority value.
/// This value is ABI compatible with an `u8`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Priority(u8);

impl Priority {
    /// Minimum allowed priority value.
    pub const MIN: Priority = Priority(MIN_PRIORITY);
    /// Maximum allowed priority value.
    pub const MAX: Priority = Priority(MAX_PRIORITY);

    /// Try to create a new `Priority`, returning `None` if `prio` doesn't respect priority ranges.
    pub const fn try_new(prio: u8) -> Option<Self> {
        if is_valid(prio) {
            unsafe {
                // SAFETY: Priority has been checked for bounds
                Some(Self::new_unchecked(prio))
            }
        } else {
            None
        }
    }

    /// Create an new `Priority`, panicking if `prio` is invalid.
    pub const fn new(prio: u8) -> Self {
        Self::try_new(prio).expect("Invalid priority value")
    }

    /// Unsafely create a new `Priority`, without checking its validity.
    /// # Safety
    /// `prio` must be between `Priority::MIN` and `Priority::MAX`.
    pub const unsafe fn new_unchecked(prio: u8) -> Self {
        debug_assert!(is_valid(prio), "invalid priority value");
        Self(prio)
    }

    /// Retrieve the raw priority value.
    pub fn value(self) -> u8 {
        self.0
    }
}
