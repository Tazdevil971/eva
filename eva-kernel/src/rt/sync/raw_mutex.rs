use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;

use crate::rt::pause::with_pause;
use crate::rt;
use crate::rt::wake_list::PriorityWakeList;
use crate::time::get_time;

use lock_api::RawMutex as _;

pub struct RawMutex {
    locked: AtomicBool,
    wait_list: PriorityWakeList,
}

unsafe impl lock_api::RawMutex for RawMutex {
    type GuardMarker = lock_api::GuardSend;

    const INIT: Self = Self {
        locked: AtomicBool::new(false),
        wait_list: PriorityWakeList::new(),
    };

    fn lock(&self) {
        // Fast path
        if self.try_lock() {
            return;
        }

        with_pause(|token| {
            // This is required because we could have a context switch just before the pause that releases the mutex
            if self.try_lock() {
                return;
            }

            self.wait_list.with_wakeup(token, |_, wakeup| {
                loop {
                    rt::suspend_and_yield_paused(token);

                    if wakeup.is_signaled(token) {
                        break;
                    }
                }
            });
        });
    }

    fn try_lock(&self) -> bool {
        self.locked
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }

    unsafe fn unlock(&self) {
        with_pause(|token| {
            let waked = self.wait_list.wakeup_one(token);
            if !waked {
                self.locked.store(false, Ordering::SeqCst);
            }
        });
    }

    fn is_locked(&self) -> bool {
        self.locked.load(Ordering::SeqCst)
    }
}

unsafe impl lock_api::RawMutexFair for RawMutex {
    unsafe fn unlock_fair(&self) {
        unsafe {
            lock_api::RawMutex::unlock(self);
        }
    }
}

unsafe impl lock_api::RawMutexTimed for RawMutex {
    type Duration = Duration;
    type Instant = Duration;

    fn try_lock_for(&self, timeout: Duration) -> bool {
        let now = get_time();
        self.try_lock_until(now + timeout)
    }

    fn try_lock_until(&self, timeout: Duration) -> bool {
        // Fast path
        if self.try_lock() {
            return true;
        }

        with_pause(|token| {
            // This is required because we could have a context switch just before the pause that releases the mutex
            if self.try_lock() {
                return true;
            }

            rt::with_timed_wakeup(token, timeout, |token, wakeup| {
                self.wait_list.with_wakeup(token, |token, wakeup2| {
                    loop {
                        rt::suspend_and_yield_paused(token);

                        if wakeup2.is_signaled(token) {
                            return true;
                        }

                        if wakeup.is_expired(token) {
                            return false;
                        }
                    }
                })
            })
        })
    }
}
