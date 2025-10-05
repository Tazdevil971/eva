#[derive(Debug, Clone, Copy)]
pub struct Bitset32(u32);

impl Bitset32 {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn insert(&mut self, idx: usize) {
        self.0 |= 1 << idx;
    }

    pub fn remove(&mut self, idx: usize) {
        self.0 &= !(1 << idx);
    }

    pub fn contains(&self, idx: usize) -> bool {
        (self.0 & (1 << idx)) != 0
    }

    pub fn highest(&self) -> Option<usize> {
        if self.0 == 0 {
            None
        } else {
            Some((31 - self.0.leading_zeros()) as usize)
        }
    }
}