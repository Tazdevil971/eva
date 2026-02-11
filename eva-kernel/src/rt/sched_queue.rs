use crate::rt::thread::{SchedListAdapter, ThreadPtr};

use eva_utils::bitset::BitSet;
use eva_utils::linked_list::LinkedList;

#[derive(Debug)]
pub struct SchedQueue {
    ready: BitSet<u32, 1>,
    lists: [LinkedList<SchedListAdapter>; 32],
    idle: Option<ThreadPtr>,
}

impl SchedQueue {
    pub const fn new() -> Self {
        Self {
            ready: BitSet::<u32, 1>::empty(),
            lists: [const { LinkedList::new(SchedListAdapter) }; 32],
            idle: None,
        }
    }

    pub fn push_thread(&mut self, thread: ThreadPtr) {
        let priority = thread.priority;
        if priority == super::IDLE_PRIORITY {
            self.idle = Some(thread);
        } else {
            let list = unsafe { self.lists.get_unchecked_mut(priority as usize) };

            list.push_back(thread);

            self.ready.insert(priority as _);
        }
    }

    pub fn pop_thread(&mut self) -> Option<ThreadPtr> {
        if let Some(highest) = self.ready.highest() {
            let list = unsafe { self.lists.get_unchecked_mut(highest) };

            let next = unsafe { list.pop_front().unwrap_unchecked() };
            if list.is_empty() {
                self.ready.remove(highest);
            }

            Some(next)
        } else {
            // No threads ready, default to idle
            self.idle.take()
        }
    }
}
