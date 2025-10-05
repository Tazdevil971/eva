use core::fmt;
use core::ops::{Index, IndexMut};

use crate::utils::assert::harden_assert;

const fn is_valid(prio: u8) -> bool {
    prio <= 31
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Priority(u8);

impl Priority {
    /// Minimum allowed priority value.
    pub const MIN: Priority = Priority(0);
    /// Maximum allowed priority value.
    pub const MAX: Priority = Priority(31);

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
        harden_assert!(is_valid(prio), "invalid priority value");
        Self(prio)
    }

    /// Retrieve the raw priority value.
    pub fn value(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Array<T>([T; 32]);

impl<T> Array<T> {
    pub const fn new(inner: [T; 32]) -> Self {
        Self(inner)
    }
}

impl<T> Index<Priority> for Array<T> {
    type Output = T;

    fn index(&self, prio: Priority) -> &Self::Output {
        unsafe {
            // SAFETY: Priority is always between 0 and 31
            self.0.get_unchecked(prio.value() as usize)
        }
    }
}

impl<T> IndexMut<Priority> for Array<T> {
    fn index_mut(&mut self, prio: Priority) -> &mut Self::Output {
        unsafe {
            // SAFETY: Priority is always between 0 and 31
            self.0.get_unchecked_mut(prio.value() as usize)
        }
    }
}

#[derive(Clone, Copy)]
pub struct Bitset(u32);

impl Bitset {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn highest(&self) -> Option<Priority> {
        if self.0 == 0 {
            None
        } else {
            let prio = (31 - self.0.leading_zeros()) as u8;

            Some(unsafe {
                // SAFETY: prio is always between 0 and 31
                Priority::new_unchecked(prio)
            })
        }
    }

    pub fn contains(&self, prio: Priority) -> bool {
        (self.0 & (1 << prio.value())) != 0
    }

    pub fn insert(&mut self, prio: Priority) {
        self.0 |= 1 << prio.value();
    }

    pub fn remove(&mut self, prio: Priority) {
        self.0 &= !(1 << prio.value());
    }
}

impl fmt::Debug for Bitset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bitset({:032b})", self.0)
    }
}
