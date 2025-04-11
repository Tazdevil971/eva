use core::sync::atomic::{AtomicBool, Ordering};

use lock_api::RawMutex as _;

use crate::pause::with_pause;
use crate::wait_queue::WaitQueue;

pub type Mutex<T> = lock_api::Mutex<RawMutex, T>;

pub struct RawMutex {
    locked: AtomicBool,
    wait_queue: WaitQueue,
}

impl RawMutex {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
            wait_queue: WaitQueue::new(),
        }
    }
}

unsafe impl lock_api::RawMutex for RawMutex {
    type GuardMarker = lock_api::GuardSend;

    const INIT: Self = Self::new();

    fn lock(&self) {
        if !self.try_lock() {
            // We failed to lock, use to the slow path
            self.wait_queue.wait(|_token| self.try_lock());
        }
    }

    fn try_lock(&self) -> bool {
        self.locked
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }

    unsafe fn unlock(&self) {
        with_pause(|token| {
            self.locked.store(false, Ordering::SeqCst);
            self.wait_queue.wake_highest(token);
        })
    }

    fn is_locked(&self) -> bool {
        self.locked.load(Ordering::SeqCst)
    }
}
