use alloc::vec::Vec;
use core::fmt::{self, Debug};
use core::mem::ManuallyDrop;

/// Internal structure used to hold a slot, either a used slot, or the index of the next free slot.
union Slot<T> {
    used: ManuallyDrop<T>,
    next: usize,
}

/// Slot map data structure. This structure allows for insertion/removal/retrieval with O(1) time complexity.
pub struct SlotMap<T> {
    inner: Vec<(u16, Slot<T>)>,
    next: usize,
}

/// Id used to access elements inside the slot map.
pub type SlotId = u32;

fn pack_slot_id(idx: usize, generation: u16) -> u32 {
    (generation as u32) | ((idx as u32) << 16)
}

fn unpack_slot_id(slot_id: u32) -> (usize, u16) {
    ((slot_id >> 16) as usize, (slot_id & 0xffff) as u16)
}

impl<T> SlotMap<T> {
    /// Create a new, empty slot map.
    pub const fn new() -> Self {
        Self {
            inner: Vec::new(),
            next: 0,
        }
    }

    /// Insert a new element in the slot map, returning its corresponding id.
    pub fn insert(&mut self, value: T) -> SlotId {
        let next = self.next;
        if next == self.inner.len() {
            self.next += 1;
            self.inner.push((
                0,
                Slot {
                    used: ManuallyDrop::new(value),
                },
            ));

            pack_slot_id(next, 0)
        } else {
            let slot = unsafe {
                // SAFETY: next is always guaranteed to be below or equal the length of the vector, and we just checked if it is equal, so it must be less than length.
                self.inner.get_unchecked_mut(next)
            };

            self.next = unsafe {
                // SAFETY: We retrieved the slot via next, so it must be empty.
                slot.1.next
            };

            slot.0 += 1;
            slot.1.used = ManuallyDrop::new(value);

            pack_slot_id(next, slot.0)
        }
    }

    fn get_slot(&self, key: SlotId) -> Option<&(u16, Slot<T>)> {
        let (index, generation) = unpack_slot_id(key);

        self.inner.get(index).filter(|slot| slot.0 == generation)
    }

    fn get_slot_mut(&mut self, key: SlotId) -> Option<&mut (u16, Slot<T>)> {
        let (index, generation) = unpack_slot_id(key);

        self.inner
            .get_mut(index)
            .filter(|slot| slot.0 == generation)
    }

    /// Retrieve a particular element.
    pub fn get(&self, id: SlotId) -> Option<&T> {
        self.get_slot(id).map(|(_, item)| unsafe {
            // SAFETY: get_slot checks the generation, so this slot is used
            &*item.used
        })
    }

    /// Retrieve a particular element, mutably.
    pub fn get_mut(&mut self, key: SlotId) -> Option<&mut T> {
        self.get_slot_mut(key).map(|(_, item)| unsafe {
            // SAFETY: get_slot_mut checks the generation, so this slot is used

            &mut *item.used
        })
    }

    /// Take/remove an element from this slot map.
    pub fn take(&mut self, key: SlotId) -> Option<T> {
        let (index, _) = unpack_slot_id(key);

        let next = self.next;
        if let Some(slot) = self.get_slot_mut(key) {
            let value = unsafe {
                // SAFETY: get_slot_mut checks the generation, so this slot is used
                ManuallyDrop::take(&mut slot.1.used)
            };

            // IMPORTANT! Increment the generation, empty slot always have a odd generation!
            slot.0 += 1;
            slot.1.next = next;

            self.next = index;

            Some(value)
        } else {
            None
        }
    }

    /// Iterate over all elements.
    /// WARNING: This method is rather slow, and should be used sparingly!
    pub fn iter(&self) -> impl Iterator<Item = (SlotId, &T)> {
        self.inner
            .iter()
            .enumerate()
            .filter(|(_, slot)| slot.0 % 2 == 0)
            .map(|(idx, slot)| {
                (pack_slot_id(idx, slot.0), unsafe {
                    // SAFETY: We iterate only on slots with even generation, that indicates that a slot is occupied
                    &*slot.1.used
                })
            })
    }
}

impl<T> Drop for SlotMap<T> {
    fn drop(&mut self) {
        let iter = self.inner.iter_mut().filter(|slot| slot.0 % 2 == 0);

        for slot in iter {
            // The slot is occupied, drop the contents
            unsafe {
                // SAFETY: We iterate only over slots with even generation, that indicates that a slot is occupied
                ManuallyDrop::drop(&mut slot.1.used);
            }
        }
    }
}

impl<T: Debug> Debug for SlotMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
