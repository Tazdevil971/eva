use core::time::Duration;

use crate::rt;
use crate::rt::pause::{PauseToken, with_pause};
use crate::rt::sync::raw_mutex::RawMutex;
use crate::rt::wake_list::PriorityWakeList;
use crate::time::get_time;
use crate::utils::{assert_abi_compatible, assert_send, assert_sync};

use scopeguard::defer;

use bytemuck::Zeroable;

#[derive(Zeroable)]
pub struct RawCondvar {
    wait_list: PriorityWakeList,
}

assert_send!(RawCondvar);
assert_sync!(RawCondvar);
assert_abi_compatible!(eva_abi::Condvar2 => RawCondvar);

impl RawCondvar {
    pub const fn new() -> Self {
        Self {
            wait_list: PriorityWakeList::new(),
        }
    }

    unsafe fn wait_internal<F, R>(&self, mutex: &RawMutex, f: F) -> R
    where
        F: FnOnce(PauseToken) -> R,
    {
        with_pause(|token| {
            // Unlock the mutex, we are paused so this is atomic
            unsafe {
                mutex.unlock_paused(token);
            }

            defer! {
                mutex.lock_paused(token);
            }

            f(token)
        })
    }

    #[unsafe(export_name = "eva_rt_sync_condvar_wait")]
    pub unsafe fn wait(&self, mutex: &RawMutex) {
        unsafe {
            self.wait_internal(mutex, |token| {
                // Wait for a wakeup
                self.wait_list.with_wakeup(token, |_, wakeup| {
                    loop {
                        rt::suspend_and_yield_paused(token);

                        if wakeup.is_signaled(token) {
                            break;
                        }
                    }
                });
            })
        }
    }

    #[unsafe(export_name = "eva_rt_sync_condvar_wait_for")]
    pub unsafe fn wait_for(&self, mutex: &RawMutex, timeout: Duration) -> bool {
        unsafe { self.wait_until(mutex, get_time() + timeout) }
    }

    #[unsafe(export_name = "eva_rt_sync_condvar_wait_until")]
    pub unsafe fn wait_until(&self, mutex: &RawMutex, timeout: Duration) -> bool {
        unsafe {
            self.wait_internal(mutex, |token| {
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

    #[unsafe(export_name = "eva_rt_sync_condvar_notify_one")]
    pub fn notify_one(&self) {
        with_pause(|token| {
            self.wait_list.wakeup_one(token);
        })
    }

    #[unsafe(export_name = "eva_rt_sync_condvar_notify_all")]
    pub fn notify_all(&self) {
        with_pause(|token| {
            self.wait_list.wakeup_all(token);
        })
    }
}
