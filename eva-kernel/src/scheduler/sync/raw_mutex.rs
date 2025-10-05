use core::cell::Cell;
use core::mem;
use core::pin::pin;
use core::sync::atomic::{AtomicBool, Ordering};

use crate::scheduler::pause::{PauseCell, with_pause};
use crate::scheduler::pend_yield;
use crate::scheduler::thread::{self, ThreadPtr};
use crate::utils::linked_list::*;

struct MutexWaitList;

unsafe impl LinkedListAdapter for MutexWaitList {
    type Node = MutexNode;

    fn offset_to_links() -> usize {
        mem::offset_of!(MutexNode, links)
    }
}

struct MutexNode {
    links: Links,
    thread: ThreadPtr,
    signal: PauseCell<Cell<bool>>,
}

pub struct RawMutex {
    locked: AtomicBool,
    wait_queue: PauseCell<StackLinkedList<MutexWaitList>>,
}

unsafe impl lock_api::RawMutex for RawMutex {
    type GuardMarker = lock_api::GuardSend;

    const INIT: Self = Self {
        locked: AtomicBool::new(false),
        wait_queue: PauseCell::new(StackLinkedList::empty()),
    };

    fn lock(&self) {
        // Fast path
        if self.try_lock() {
            return;
        }

        // Slow path
        let node = MutexNode {
            links: Links::unlinked(),
            thread: thread::current(),
            signal: PauseCell::new(Cell::new(false)),
        };

        let node = pin!(node);
        let node = node.as_ref();

        with_pause(|token| {
            if self.try_lock() {
                return;
            }

            unsafe {
                self.wait_queue.borrow(token).push_front(node);
            }

            loop {
                thread::suspend_and_yield_paused(token);

                if node.signal.get(token) {
                    break;
                }
            }
        });
    }

    fn try_lock(&self) -> bool {
        self.locked
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }

    unsafe fn unlock(&self) {
        with_pause(|token| {
            let to_wake = unsafe {
                // SAFETY: This node won't escape the pause, and we don't yield
                self.wait_queue.borrow(token).pop_back()
            };

            if let Some(to_wake) = to_wake {
                // Signal and wake the thread
                to_wake.signal.set(token, true);
                // SAFETY: If a thread is inside the queue, it is waiting and valid
                to_wake.thread.resume_paused(token);

                // Ok someone was waiting, wake him but DON'T unlock the mutex
                // We should reschedule ONLY if the current priority is strictly lower than the new thread!
                if thread::current().priority() < to_wake.thread.priority() {
                    pend_yield(token);
                }
            } else {
                // No one to wake, just release the lock
                self.locked.store(false, Ordering::SeqCst);
            }
        });
    }

    fn is_locked(&self) -> bool {
        self.locked.load(Ordering::SeqCst)
    }
}
