use core::time::Duration;

use crate::rt;
use crate::rt::pause::{PauseToken, with_pause};
use crate::rt::sync::raw_mutex::RawMutex;
use crate::rt::wake_list::PriorityWakeList;
use crate::time::get_time;

use lock_api::MutexGuard;
use scopeguard::defer;

pub struct Condvar {
    wait_list: PriorityWakeList,
}

impl Condvar {
    pub const fn new() -> Self {
        Self {
            wait_list: PriorityWakeList::new(),
        }
    }

    fn wait_internal<'a, T, F, R>(
        &self,
        guard: MutexGuard<'a, RawMutex, T>,
        f: F,
    ) -> (MutexGuard<'a, RawMutex, T>, R)
    where
        F: FnOnce(PauseToken) -> R,
    {
        let ret = with_pause(|token| {
            // Unlock the mutex, we are paused so this is atomic
            unsafe {
                // SAFETY: We won't be using guard, so we can safely unlock the underlying mutex
                MutexGuard::mutex(&guard).raw().unlock_paused(token);
            }

            defer! {
                unsafe {
                    // SAFETY: We previously unlocked the mutex, so we must lock it again at the end of the scope
                    MutexGuard::mutex(&guard).raw().lock_paused(token);
                }
            }

            f(token)
        });

        (guard, ret)
    }

    pub fn wait<'a, T>(&self, guard: MutexGuard<'a, RawMutex, T>) -> MutexGuard<'a, RawMutex, T> {
        self.wait_internal(guard, |token| {
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
        .0
    }

    pub fn wait_for<'a, T>(
        &self,
        guard: MutexGuard<'a, RawMutex, T>,
        timeout: Duration,
    ) -> (MutexGuard<'a, RawMutex, T>, bool) {
        let now = get_time();
        self.wait_until(guard, now + timeout)
    }

    pub fn wait_until<'a, T>(
        &self,
        guard: MutexGuard<'a, RawMutex, T>,
        timeout: Duration,
    ) -> (MutexGuard<'a, RawMutex, T>, bool) {
        self.wait_internal(guard, |token| {
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

    pub fn notify_one(&self) {
        with_pause(|token| {
            self.wait_list.wakeup_one(token);
        })
    }

    pub fn notify_all(&self) {
        with_pause(|token| {
            self.wait_list.wakeup_all(token);
        })
    }
}
