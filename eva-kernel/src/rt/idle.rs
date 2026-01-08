use core::cell::RefCell;
use core::ptr;

use crate::rt::pause::{PauseCell, PauseToken, with_pause};
use crate::rt::thread::{ThreadListAdapter, ThreadPtr};
use crate::rt::{self, IDLE_PRIORITY, yield_now};
use eva_utils::linked_list::LinkedList;

const IDLE_STACK: usize = 4096;

static TO_DESTROY: PauseCell<RefCell<LinkedList<ThreadListAdapter>>> =
    PauseCell::ref_cell(LinkedList::new(ThreadListAdapter));

pub fn defer_thread_destroy_paused(token: PauseToken, thread: ThreadPtr) {
    TO_DESTROY.borrow_ref_mut(token).push_back(thread);
}

extern "C" fn idle_task(_: *mut (), _: *mut (), _: *mut (), _: *mut ()) -> ! {
    // The first thing we need to do in the idle thread, is to set the scheduler to running
    rt::start();

    // Yield immediately, this will switch control to the first actual thread
    yield_now();

    loop {
        // Otherwise run "idle" tasks
        let thread = with_pause(|token| TO_DESTROY.borrow_ref_mut(token).pop_back());

        if let Some(thread) = thread {
            unsafe {
                thread.destroy();
            }
        } else {
            // If there are no more threads, just run the scheduler
            yield_now();
        }
    }
}

pub fn create() -> ThreadPtr {
    ThreadPtr::create(
        IDLE_STACK,
        IDLE_PRIORITY,
        c"Idle",
        rt::gen_tid(),
        idle_task,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    )
}
