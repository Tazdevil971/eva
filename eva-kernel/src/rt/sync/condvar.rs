use core::time::Duration;

use crate::rt::sync::raw_condvar::RawCondvar;
use crate::rt::sync::raw_mutex::RawMutex;

use lock_api::MutexGuard;

pub struct Condvar {
    inner: RawCondvar,
}

impl Condvar {
    pub const fn new() -> Self {
        Self {
            inner: RawCondvar::new(),
        }
    }

    pub unsafe fn raw(&self) -> &RawCondvar {
        &self.inner
    }

    pub fn wait<'a, T>(&self, guard: MutexGuard<'a, RawMutex, T>) -> MutexGuard<'a, RawMutex, T> {
        unsafe {
            self.inner.wait(MutexGuard::mutex(&guard).raw());
        }

        guard
    }

    pub fn wait_for<'a, T>(
        &self,
        guard: MutexGuard<'a, RawMutex, T>,
        timeout: Duration,
    ) -> (MutexGuard<'a, RawMutex, T>, bool) {
        let res = unsafe {
            self.inner
                .wait_for(MutexGuard::mutex(&guard).raw(), timeout)
        };
        (guard, res)
    }

    pub fn wait_until<'a, T>(
        &self,
        guard: MutexGuard<'a, RawMutex, T>,
        timeout: Duration,
    ) -> (MutexGuard<'a, RawMutex, T>, bool) {
        let res = unsafe {
            self.inner
                .wait_until(MutexGuard::mutex(&guard).raw(), timeout)
        };
        (guard, res)
    }

    pub fn notify_one(&self) {
        self.inner.notify_one();
    }

    pub fn notify_all(&self) {
        self.inner.notify_all();
    }
}
