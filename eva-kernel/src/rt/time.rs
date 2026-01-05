use core::time::Duration;

use crate::rt::pause::{PauseToken, with_pause};
use crate::rt::suspend_and_yield_paused;
use crate::rt::wake_list::{TimedWakeList, TimedWakeup};

static TIME_WAKE_LIST: TimedWakeList = TimedWakeList::new();

pub(super) fn with_timed_wakeup<F, T>(token: PauseToken, timeout: Duration, f: F) -> T
where
    F: FnOnce(PauseToken, &TimedWakeup) -> T,
{
    TIME_WAKE_LIST.with_wakeup(token, timeout, f)
}

/// Sleep and yield for a given amount of time.
#[unsafe(export_name = "eva_rt_sleep_for")]
pub fn sleep_for(duration: Duration) {
    sleep_until(crate::time::get_time() + duration);
}

/// Sleep until a specific point in the future.
#[unsafe(export_name = "eva_rt_sleep_until")]
pub fn sleep_until(expire: Duration) {
    with_pause(|token| {
        with_timed_wakeup(token, expire, |_, wakeup| {
            while !wakeup.is_expired(token) {
                suspend_and_yield_paused(token);
            }
        })
    })
}

pub(super) fn run_time_driver_paused(token: PauseToken, instant: Duration) {
    TIME_WAKE_LIST.wakeup_until(token, instant);
}
