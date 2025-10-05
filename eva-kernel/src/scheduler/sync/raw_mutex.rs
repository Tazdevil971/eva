use core::sync::atomic::{AtomicBool, Ordering};

use crate::scheduler::pause::with_pause;
use crate::scheduler::pend_yield;
use crate::scheduler::thread::{self, ThreadPtr};
use crate::utils::linked_list::{LinkedList, Node};

#[derive(Debug)]
struct MutexNode {
    thread: ThreadPtr,
}

pub struct RawMutex {
    locked: AtomicBool,
    wait_queue: LinkedList<MutexNode>,
}

unsafe impl lock_api::RawMutex for RawMutex {
    type GuardMarker = lock_api::GuardSend;

    const INIT: Self = Self {
        locked: AtomicBool::new(false),
        wait_queue: LinkedList::new(),
    };

    fn lock(&self) {
        with_pause(|token| {
            if self.try_lock() {
                return;
            }

            // We didn't succeed, fall back to wait list
            Node::new(
                MutexNode {
                    thread: thread::current(),
                },
                |node| {
                    unsafe {
                        // LIFO wake priority
                        // SAFETY: We guarantee that node will be unlinked before returning
                        self.wait_queue.push_front(token, node);
                    }

                    loop {
                        thread::suspend_and_yield_paused(token);

                        if unsafe {
                            // SAFETY: node is either unlinked or linked to this queue
                            !self.wait_queue.contains(token, node)
                        } {
                            break;
                        }
                    }
                },
            );
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
                self.wait_queue.pop_back(token)
            };

            if let Some(to_wake) = to_wake {
                let to_wake = to_wake.value();

                // Ok someone was waiting, wake him but DON'T unlock the mutex
                unsafe {
                    // SAFETY: If a thread is inside the queue, it is waiting and valid
                    to_wake.thread.resume_paused(token);

                    // We should reschedule ONLY if the current priority is strictly lower than the new thread!
                    if thread::current().priority() < to_wake.thread.priority() {
                        pend_yield(token);
                    }
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
