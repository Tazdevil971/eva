use core::cell::Cell;

#[derive(Debug, Clone)]
pub struct Bitset32(Cell<u32>);

impl Bitset32 {
    pub const fn empty() -> Self {
        Self(Cell::new(0))
    }

    pub fn is_empty(&self) -> bool {
        self.0.get() == 0
    }

    pub fn insert(&self, idx: usize) {
        self.0.update(|num| num | 1 << idx);
    }

    pub fn remove(&self, idx: usize) {
        self.0.update(|num| num & !(1 << idx));
    }

    pub fn contains(&self, idx: usize) -> bool {
        (self.0.get() & (1 << idx)) != 0
    }

    pub fn highest(&self) -> Option<usize> {
        let val = self.0.get();
        if val == 0 {
            None
        } else {
            Some((31 - val.leading_zeros()) as usize)
        }
    }
}