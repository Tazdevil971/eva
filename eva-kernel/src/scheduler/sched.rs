use core::cell::Cell;

use crate::scheduler;
use crate::scheduler::thread::{ThreadList, ThreadPtr};
use crate::utils::assert::harden_assert;
use crate::utils::bitset::Bitset32;
use crate::utils::linked_list::HeapLinkedList;

/// Internal mutable thread structure.
pub struct Sched {
    /// Bitmask representing queues that are not empty.
    set: Bitset32,
    queues: [HeapLinkedList<ThreadList>; 32],
    /// Idle thread.
    idle: Cell<Option<ThreadPtr>>,
}

impl Sched {
    pub const fn new() -> Self {
        Self {
            set: Bitset32::empty(),
            queues: [const { HeapLinkedList::empty() }; 32],
            idle: Cell::new(None),
        }
    }

    pub unsafe fn set_idle(&self, thread: ThreadPtr) {
        self.idle.set(Some(thread));
    }

    pub unsafe fn push_thread(&self, thread: ThreadPtr) {
        let prio = unsafe { thread.priority() };
        if prio == scheduler::IDLE_PRIORITY {
            // This is the idle thread
            return;
        }

        let queue = unsafe {
            harden_assert!(scheduler::is_valid_prio(prio), "invalid priority");
            self.queues.get_unchecked(prio as usize)
        };

        unsafe {
            queue.push_back(thread);
        }
        self.set.insert(prio as usize);
    }

    pub unsafe fn pop_thread(&self) -> ThreadPtr {
        if let Some(prio) = self.set.highest() {
            let list = unsafe { self.queues.get_unchecked(prio) };

            let thread = list.pop_front();
            if list.is_empty() {
                self.set.remove(prio);
            }

            let thread = unsafe {
                harden_assert!(thread.is_some(), "prio indicates a empty queue");
                thread.unwrap_unchecked()
            };

            thread
        } else {
            let idle = self.idle.get();
            unsafe {
                harden_assert!(idle.is_some(), "no idle thread installed");
                idle.unwrap_unchecked()
            }
        }
    }
}
