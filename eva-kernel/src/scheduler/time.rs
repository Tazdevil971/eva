use core::cell::Cell;
use core::mem;
use core::pin::pin;
use core::time::Duration;

use crate::scheduler::pause::{PauseCell, PauseToken, with_pause};
use crate::scheduler::thread::{self, ThreadPtr};
use crate::utils::linked_list::*;

struct SleepList;

unsafe impl LinkedListAdapter for SleepList {
    type Node = TimedWakeup;

    fn offset_to_links() -> usize {
        mem::offset_of!(TimedWakeup, links)
    }
}

pub(super) struct TimedWakeup {
    links: Links,
    thread: ThreadPtr,
    expire: Duration,
    signal: PauseCell<Cell<bool>>,
}

impl TimedWakeup {
    pub(super) fn is_expired(&self, token: PauseToken) -> bool {
        self.signal.get(token)
    }
}

struct TimeDriver {
    sleep_list: StackLinkedList<SleepList>,
}

static TIME_DRIVER: PauseCell<TimeDriver> = PauseCell::new(TimeDriver {
    sleep_list: StackLinkedList::empty(),
});

pub(super) fn with_timed_wakeup<F, T>(token: PauseToken, expire: Duration, f: F) -> T
where
    F: FnOnce(PauseToken, &TimedWakeup) -> T,
{
    let node = TimedWakeup {
        links: Links::unlinked(),
        thread: thread::current(),
        expire,
        signal: PauseCell::new(Cell::new(false)),
    };

    let node = pin!(node);
    let node = node.as_ref();

    let sleep_list = &TIME_DRIVER.borrow(token).sleep_list;

    unsafe {
        let point = sleep_list.iter().find(|node2| node2.expire > expire);
        if let Some(point) = point {
            sleep_list.insert_before(point, node);
        } else {
            sleep_list.push_back(node);
        }
    }

    let ret = f(token, &*node);

    if !node.signal.get(token) {
        unsafe {
            // SAFETY: ...
            sleep_list.remove(node);
        }
    }

    ret
}

pub fn sleep(duration: Duration) {
    sleep_until(crate::time::get_time() + duration);
}

pub fn sleep_until(expire: Duration) {
    with_pause(|token| {
        with_timed_wakeup(token, expire, |_, wakeup| {
            while !wakeup.is_expired(token) {
                thread::suspend_and_yield_paused(token);
            }
        })
    })
}

pub(super) fn run_time_driver_paused(token: PauseToken, instant: Duration) {
    let sleep_list = &TIME_DRIVER.borrow(token).sleep_list;

    loop {
        let head = unsafe {
            // SAFETY: ...
            sleep_list.head()
        };

        if let Some(head) = head {
            if head.expire > instant {
                break;
            }
        } else {
            break;
        }

        let head = unsafe {
            // SAFETY: ...
            sleep_list.pop_front().unwrap()
        };

        head.signal.borrow(token).set(true);
        unsafe {
            head.thread.resume_paused(token);
        }
    }
}
