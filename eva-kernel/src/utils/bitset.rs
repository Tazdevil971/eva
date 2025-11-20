use core::fmt::{self, Debug};

/// Set of integers (range 0-31) implemented as a bitset.
#[derive(Clone, Copy)]
pub struct Bitset32(u32);

impl Bitset32 {
    /// Construct a new empty bitset.
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Check if this bitset is empty.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Insert a new element in the bitset.
    pub fn insert(&mut self, idx: usize) {
        assert!(idx < 32);
        self.0 |= 1 << idx;
    }

    /// Remove an element from the bitset.
    pub fn remove(&mut self, idx: usize) {
        assert!(idx < 32);
        self.0 &= !(1 << idx);
    }

    /// Check if this bitset contains a specific element.
    pub fn contains(&self, idx: usize) -> bool {
        (self.0 & (1 << idx)) != 0
    }

    /// Return the element with the highest value.
    pub fn highest(&self) -> Option<usize> {
        if self.0 == 0 {
            None
        } else {
            Some((31 - self.0.leading_zeros()) as usize)
        }
    }

    /// Iterate over the elements of the set.
    pub fn iter(&self) -> impl Iterator<Item = usize> {
        (0..32).filter(|idx| (self.0 & (1 << idx)) != 0)
    }
}

impl Debug for Bitset32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}
