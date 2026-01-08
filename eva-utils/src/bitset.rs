use core::fmt::{self, Debug};
use core::mem;

use num_traits::int::PrimInt;

/// Set of integers implemented as a bit set.
#[derive(Clone, Copy)]
pub struct BitSet<T, const N: usize>([T; N]);

impl<const N: usize> BitSet<u8, N> {
    /// Construct a new empty bitset.
    pub const fn empty() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> BitSet<u16, N> {
    /// Construct a new empty bitset.
    pub const fn empty() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> BitSet<u32, N> {
    /// Construct a new empty bitset.
    pub const fn empty() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> BitSet<u64, N> {
    /// Construct a new empty bitset.
    pub const fn empty() -> Self {
        Self([0; N])
    }
}

impl<T, const N: usize> BitSet<T, N>
where
    T: PrimInt,
{
    const BIT_STRIDE: usize = mem::size_of::<T>() * 8;

    /// Number of elements this bit set can contain.
    pub const SIZE: usize = mem::size_of::<T>() * 8 * N;

    /// Check if this bitset is empty.
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|b| b.is_zero())
    }

    #[inline(always)]
    fn idx_to_offset_and_shift(idx: usize) -> (usize, usize) {
        assert!(idx < Self::SIZE);
        (idx / Self::BIT_STRIDE, idx % Self::BIT_STRIDE)
    }

    /// Insert a new element in the bitset.
    pub fn insert(&mut self, idx: usize) {
        let (offset, shift) = Self::idx_to_offset_and_shift(idx);
        self.0[offset] = self.0[offset] | (T::one() << shift);
    }

    /// Remove an element from the bitset.
    pub fn remove(&mut self, idx: usize) {
        let (offset, shift) = Self::idx_to_offset_and_shift(idx);
        self.0[offset] = self.0[offset] & !(T::one() << shift);
    }

    /// Check if this bitset contains a specific element.
    pub fn contains(&self, idx: usize) -> bool {
        let (offset, shift) = Self::idx_to_offset_and_shift(idx);
        !(self.0[offset] & (T::one() << shift)).is_zero()
    }

    /// Return the element with the highest value.
    pub fn highest(&self) -> Option<usize> {
        self.0.iter().enumerate().find_map(|(offset, b)| {
            if b.is_zero() {
                None
            } else {
                Some((31 - b.leading_zeros()) as usize + offset * Self::BIT_STRIDE)
            }
        })
    }

    /// Iterate over the elements of the set.
    pub fn iter(&self) -> impl Iterator<Item = usize> {
        self.0.iter().enumerate().flat_map(|(offset, b)| {
            (0..Self::BIT_STRIDE)
                .filter(|shift| !(*b & (T::one() << *shift)).is_zero())
                .map(move |idx| idx + offset * Self::BIT_STRIDE)
        })
    }
}

impl<T, const N: usize> Debug for BitSet<T, N>
where
    T: PrimInt,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}
