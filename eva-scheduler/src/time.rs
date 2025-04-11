use core::time::Duration;

use crate::linked_list::{LinkedList, Node};
use crate::pause::{PauseToken, with_pause};
use crate::portability;
use crate::raw_thread::{self, RawThread};

pub struct TimeNode {
    thread: RawThread,
    when: Duration,
}

static TIME_QUEUE: LinkedList<TimeNode> = LinkedList::new();

/// Query the time driver for the current time.
pub fn get_time() -> Duration {
    portability::get_time()
}

pub(crate) fn run_time_driver(token: PauseToken) {
    let time = get_time();

    while let Some(first) = unsafe {
        // SAFETY: The node never escapes this section
        TIME_QUEUE.front(token)
    } {
        // Break out if we find something that hasn't expired yet
        if first.value().when <= time {
            break;
        }

        unsafe {
            // SAFETY: We just got this node from the queue, so it must be valid
            TIME_QUEUE.remove(token, first);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimedResult {
    Ok,
    Timeout,
}

pub fn timed_suspend_and_yield_paused(token: PauseToken, when: Duration) -> TimedResult {
    // We already surpassed the end time
    if when <= get_time() {
        return TimedResult::Timeout;
    }

    let thread = raw_thread::current();

    Node::new(TimeNode { thread, when }, |node| {
        unsafe {
            // SAFETY: The node is unlinked before being destroyed
            TIME_QUEUE.insert_sorted_by_key(token, node, |node| node.value().when);
        }

        raw_thread::suspend_and_yield_paused(token);

        if unsafe {
            // SAFETY: node is either unlinked or linked to this queue
            TIME_QUEUE.try_remove(token, node)
        } {
            TimedResult::Timeout
        } else {
            TimedResult::Ok
        }
    })
}

pub fn sleep_until_paused(token: PauseToken, when: Duration) {
    while timed_suspend_and_yield_paused(token, when) != TimedResult::Timeout {}
}

pub fn sleep_until(when: Duration) {
    with_pause(|token| sleep_until_paused(token, when));
}
