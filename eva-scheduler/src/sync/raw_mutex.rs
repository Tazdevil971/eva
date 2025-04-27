use core::cell::Cell;
use core::pin::{Pin, pin};
use core::sync::atomic::{AtomicBool, Ordering};

use super::linked_list::{LinkedList, Node};
use crate::pause::{PauseMutex, with_pause, yield_later};
use crate::raw_thread::{self, RawThread};

struct MutexNode {
    thread: RawThread,
}

pub struct RawMutex {
    locked: AtomicBool,
    wait_queue: LinkedList<MutexNode>,
}

unsafe impl lock_api::RawMutex for RawMutex {
    type GuardMarker = lock_api::GuardSend;

    const INIT: Self = Self {
        locked: AtomicBool::new(false),
        wait_queue: LinkedList::empty(),
    };

    fn lock(&self) {
        if !self.try_lock() {
            let node = Node::new(MutexNode {
                thread: raw_thread::current(),
            });

            let node = pin!(node);
            let node = node.into_ref();

            let wait_queue = unsafe { Pin::new_unchecked(&self.wait_queue) };

            // We failed to lock, use to the slow path
            while with_pause(|token| {
                if self.try_lock() {
                    // Make sure the node is unlinked
                    node.try_remove(token);
                    return false;
                }

                // The node is not yet linked
                if !node.is_linked(token) {
                    wait_queue.push_back(token, node);
                }

                raw_thread::suspend_paused(token);
                yield_later(token);

                true
            }) {}
        }
    }

    fn try_lock(&self) -> bool {
        self.locked
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }

    unsafe fn unlock(&self) {
        with_pause(|token| {
            // Find the thread with the highest priority and wake him up
            let to_wake = self
                .wait_queue
                .iter(token)
                .max_by_key(|node| unsafe { node.value().thread.priority() });

            if let Some(to_wake) = to_wake {
                to_wake.try_remove(token);
                unsafe { to_wake.value().thread.resume_paused(token) };
            }

            self.locked.store(false, Ordering::SeqCst);
        })
    }

    fn is_locked(&self) -> bool {
        self.locked.load(Ordering::SeqCst)
    }
}
