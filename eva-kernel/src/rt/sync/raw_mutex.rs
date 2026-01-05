use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;

use crate::rt;
use crate::rt::pause::{PauseToken, with_pause};
use crate::rt::wake_list::PriorityWakeList;
use crate::time::get_time;
use crate::utils::{assert_abi_compatible, assert_send, assert_sync};

use bytemuck::Zeroable;

#[derive(Zeroable)]
pub struct RawMutex {
    locked: AtomicBool,
    wait_list: PriorityWakeList,
}

assert_send!(RawMutex);
assert_sync!(RawMutex);
assert_abi_compatible!(eva_abi::Mutex2 => RawMutex);

impl RawMutex {
    pub fn lock_paused(&self, token: PauseToken) {
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
    }

    pub unsafe fn unlock_paused(&self, token: PauseToken) {
        let waked = self.wait_list.wakeup_one(token);
        if !waked {
            self.locked.store(false, Ordering::SeqCst);
        }
    }

    #[unsafe(export_name = "eva_rt_sync_mutex_lock")]
    pub fn lock(&self) {
        // Fast path
        if self.try_lock() {
            return;
        }

        with_pause(|token| {
            self.lock_paused(token);
        });
    }

    #[unsafe(export_name = "eva_rt_sync_mutex_try_lock")]
    pub fn try_lock(&self) -> bool {
        self.locked
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }

    #[unsafe(export_name = "eva_rt_sync_mutex_unlock")]
    pub unsafe fn unlock(&self) {
        with_pause(|token| unsafe {
            self.unlock_paused(token);
        });
    }

    #[unsafe(export_name = "eva_rt_sync_mutex_is_locked")]
    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::SeqCst)
    }

    #[unsafe(export_name = "eva_rt_sync_mutex_try_lock_for")]
    pub fn try_lock_for(&self, timeout: Duration) -> bool {
        let now = get_time();
        self.try_lock_until(now + timeout)
    }

    #[unsafe(export_name = "eva_rt_sync_mutex_try_lock_until")]
    pub fn try_lock_until(&self, timeout: Duration) -> bool {
        // Fast path
        if self.try_lock() {
            return true;
        }

        with_pause(|token| {
            // This is required because we could have a context switch just before the pause that releases the mutex
            if self.try_lock() {
                return true;
            }

            rt::time::with_timed_wakeup(token, timeout, |token, wakeup| {
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

unsafe impl lock_api::RawMutex for RawMutex {
    type GuardMarker = lock_api::GuardSend;

    const INIT: Self = Self {
        locked: AtomicBool::new(false),
        wait_list: PriorityWakeList::new(),
    };

    fn lock(&self) {
        self.lock()
    }

    fn try_lock(&self) -> bool {
        self.try_lock()
    }

    unsafe fn unlock(&self) {
        unsafe { self.unlock() }
    }
}

unsafe impl lock_api::RawMutexFair for RawMutex {
    unsafe fn unlock_fair(&self) {
        unsafe { self.unlock() }
    }
}

unsafe impl lock_api::RawMutexTimed for RawMutex {
    type Duration = Duration;
    type Instant = Duration;

    fn try_lock_for(&self, timeout: Self::Duration) -> bool {
        self.try_lock_for(timeout)
    }

    fn try_lock_until(&self, timeout: Self::Instant) -> bool {
        self.try_lock_until(timeout)
    }
}
