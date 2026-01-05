use core::ptr::{NonNull, addr_eq};

use crate::rt::thread::{ThreadListAdapter, ThreadPtr};
use crate::utils::linked_list::LinkedList;

pub struct ThreadList {
    list: LinkedList<ThreadListAdapter>,
}

impl ThreadList {
    pub const fn new() -> Self {
        Self {
            list: LinkedList::new(ThreadListAdapter),
        }
    }

    pub fn exists(&self, thread: NonNull<()>) -> bool {
        self.list
            .iter()
            .find(|thread2| addr_eq(*thread2, thread.as_ptr()))
            .is_some()
    }

    pub unsafe fn remove_thread(&mut self, thread: ThreadPtr) {
        unsafe {
            self.list.cursor_mut_from_raw(thread.0).remove_current();
        }
    }

    pub fn add_thread(&mut self, thread: ThreadPtr) {
        self.list.push_back(thread);
    }
}
