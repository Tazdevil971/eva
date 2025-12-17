//! Threading and scheduler subsystem.

mod sched_queue;
mod thread;
mod wake_list;

pub mod pause;
pub mod sync;
pub mod tls;

use core::cell::Cell;
use core::cell::RefCell;
use core::ffi::CStr;
use core::sync::atomic::Ordering;
use core::time::Duration;
use core::{mem, ptr};

use pause::pend_yield;
use pause::{PauseCell, PauseToken, run_scheduler, with_pause, yield_now_from_paused};
use sched_queue::SchedQueue;
use thread::{AtomicThreadPtr, ThreadPtr};
use wake_list::{TimedWakeList, TimedWakeup};

pub use eva_abi::error::{JoinError, ResumeError};
pub use eva_abi::{Priority, Thread, ThreadFn};

use crate::port::{self, Impl as _};

/// Priority reserved to the idle thread.
pub const IDLE_PRIORITY: Priority = -1;
/// Minimum priority supported.
pub const MIN_PRIORITY: Priority = 0;
/// Maximum priority supported.
pub const MAX_PRIORITY: Priority = 31;

/// Check if a priority level is valid for user code.
pub fn is_valid_prio(prio: Priority) -> bool {
    prio >= MIN_PRIORITY && prio <= MAX_PRIORITY
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Idle,
    Running,
    Abort,
}

/// Internal scheduler structure
struct Scheduler {
    state: PauseCell<Cell<State>>,
    sched_queue: PauseCell<RefCell<SchedQueue>>,
    current: AtomicThreadPtr,
    time_wake_list: TimedWakeList,
}

static SCHEDULER: Scheduler = Scheduler {
    state: PauseCell::cell(State::Idle),
    sched_queue: PauseCell::ref_cell(SchedQueue::new()),
    current: AtomicThreadPtr::new(None),
    time_wake_list: TimedWakeList::new(),
};

/// Immediately abort execution, no scheduling will be done past this call
#[unsafe(export_name = "eva_rt_abort")]
pub fn abort() {
    with_pause(|token| {
        SCHEDULER.state.set(token, State::Abort);
    })
}

unsafe fn set_current_paused(_token: PauseToken, thread: ThreadPtr) {
    unsafe {
        thread.set_as_global_switchctx();
    }
    SCHEDULER.current.store(Some(thread), Ordering::Relaxed);
}

/// Yield now to some other thread.
#[unsafe(export_name = "eva_rt_yield_now")]
pub fn yield_now() {
    port::GlobalImpl::yield_now();
}

fn current_raw() -> ThreadPtr {
    // TODO(davide.mor): Review memory ordering here
    SCHEDULER
        .current
        .load(Ordering::Relaxed)
        .expect("No current thread, scheduler is running!")
}

/// Retrieve the currently running thread.
#[unsafe(export_name = "eva_rt_current")]
pub fn current() -> Thread {
    current_raw().to_abi()
}

fn thread_exit_pad() -> ! {
    let thread = current_raw();

    // First run TLS destructors
    thread.local_store.run_dtors();

    // Then wake threads waiting for join and transition to zombie
    with_pause(|token| {
        if let Some(join_wait_thread) = thread.join_wait_thread.get(token) {
            resume_paused_raw(token, join_wait_thread).expect("thread in wake list but awake");
        }

        thread.state.set(token, thread::State::Zombie);
    });

    yield_now();
    unreachable!("zombie thread resumed running!")
}

/// Spawn a thread.
#[unsafe(export_name = "eva_rt_spawn")]
pub fn spawn(
    stack_size: usize,
    priority: Priority,
    entry: ThreadFn,
    name: &CStr,
    user: *mut (),
) -> Option<Thread> {
    assert!(is_valid_prio(priority), "invalid priority");

    unsafe extern "C" fn launcher(
        arg1: *mut (),
        arg2: *mut (),
        _arg3: *mut (),
        _arg4: *mut (),
    ) -> ! {
        let entry = unsafe { mem::transmute::<_, ThreadFn>(arg1) };

        (entry)(arg2);

        thread_exit_pad();
    }

    let thread = ThreadPtr::create(stack_size, priority, name);
    unsafe {
        thread.init_switchctx(launcher, entry as _, user, ptr::null_mut(), ptr::null_mut());
    }

    with_pause(|token| {
        // Insert the new thread in the scheduler
        SCHEDULER
            .sched_queue
            .borrow_ref_mut(token)
            .push_thread(thread);
    });

    Some(thread.to_abi())
}

/// Join a thread, destroying it in the process.
/// # Safety:
/// - `thread` must be the only instance pointing to this thread.
#[unsafe(export_name = "eva_rt_join_unchecked")]
pub unsafe fn join_unchecked(thread: Thread) -> Result<(), JoinError> {
    let thread = unsafe { ThreadPtr::from_abi(thread) };

    let thread = with_pause(|token| {
        if thread.state.get(token) == thread::State::Zombie {
            // The thread is already in zombie state
            return Ok(thread);
        }

        // Check that no one is already waiting on the thread
        if thread.join_wait_thread.get(token).is_some() {
            return Err(JoinError::AlreadyJoining);
        }

        let current = current_raw();

        // The thread is still alive and kicking!
        // Register for wait!
        thread.join_wait_thread.set(token, Some(current));

        loop {
            // Wait for someone to wake us
            suspend_and_yield_paused(token);
            // Check if the thread is dead, do this to avoid spurious wakeups
            if thread.state.get(token) == thread::State::Zombie {
                break;
            }
        }

        Ok(thread)
    })?;

    // Finally destroy the thread
    unsafe {
        // SAFETY: The caller ensures that this is safe
        thread.destroy();
    }

    Ok(())
}

#[unsafe(export_name = "eva_rt_get_priority")]
pub unsafe fn get_priority(thread: Thread) -> Priority {
    let thread = unsafe { ThreadPtr::from_abi(thread) };
    thread.tcb().priority
}

#[unsafe(export_name = "eva_rt_get_current_priority")]
pub fn get_current_priority() -> Priority {
    current_raw().tcb().priority
}

fn resume_paused_raw(token: PauseToken, thread: ThreadPtr) -> Result<(), ResumeError> {
    let state = thread.state.get(token);
    if state != thread::State::Suspend {
        return Err(ResumeError::AlreadyRunning);
    }

    thread.state.set(token, thread::State::Ready);
    SCHEDULER
        .sched_queue
        .borrow_ref_mut(token)
        .push_thread(thread);

    // Check if we woke up a thread with higher priority
    let current = current_raw();
    if current.priority < thread.priority {
        // Pend a yield
        pend_yield(token);
    }

    Ok(())
}

#[unsafe(export_name = "eva_rt_resume_irq_unchecked")]
pub unsafe fn resume_irq_unchecked(_thread: Thread) -> Result<(), ResumeError> {
    todo!("IRQ support not yet implemented!")
}

/// Resume a thread from a suspend state. Usable in a paused context.
#[unsafe(export_name = "eva_rt_resume_paused_unchecked")]
pub unsafe fn resume_paused_unchecked(
    token: PauseToken,
    thread: Thread,
) -> Result<(), ResumeError> {
    let thread = unsafe { ThreadPtr::from_abi(thread) };
    resume_paused_raw(token, thread)
}

/// Suspend executing of the current thread, after the next yield this thread won't be scheduled anymore. Usable in a paused context.
#[unsafe(export_name = "eva_rt_suspend_paused")]
pub fn suspend_paused(token: PauseToken) {
    let thread = current_raw();

    let state = thread.state.get(token);
    if state == thread::State::Ready {
        thread.state.set(token, thread::State::Suspend);
    }
}

/// Resume a thread from a suspend state.
#[unsafe(export_name = "eva_rt_resume_unchecked")]
pub unsafe fn resume_unchecked(thread: Thread) -> Result<(), ResumeError> {
    with_pause(|token| unsafe { resume_paused_unchecked(token, thread) })
}

/// Suspend the current thread and yield to the scheduler.
#[unsafe(export_name = "eva_rt_suspend_and_yield_paused")]
pub fn suspend_and_yield_paused(token: PauseToken) {
    suspend_paused(token);
    yield_now_from_paused(token);
}

#[unsafe(export_name = "eva_rt_suspend_and_yield")]
pub fn suspend_and_yield() {
    with_pause(|token| suspend_and_yield_paused(token))
}

#[unsafe(export_name = "eva_rt_suspend_and_yield_for")]
pub fn suspend_and_yield_for(time: Duration) -> bool {
    with_pause(|token| suspend_and_yield_paused_for(token, time))
}

#[unsafe(export_name = "eva_rt_suspend_and_yield_until")]
pub fn suspend_and_yield_until(time: Duration) -> bool {
    with_pause(|token| suspend_and_yield_paused_until(token, time))
}

#[unsafe(export_name = "eva_rt_suspend_and_yield_paused_for")]
pub fn suspend_and_yield_paused_for(token: PauseToken, time: Duration) -> bool {
    suspend_and_yield_paused_until(token, crate::time::get_time() + time)
}

#[unsafe(export_name = "eva_rt_suspend_and_yield_paused_until")]
pub fn suspend_and_yield_paused_until(token: PauseToken, time: Duration) -> bool {
    with_timed_wakeup(token, time, |_, wakeup| {
        suspend_and_yield_paused(token);
        !wakeup.is_expired(token)
    })
}

fn with_timed_wakeup<F, T>(token: PauseToken, timeout: Duration, f: F) -> T
where
    F: FnOnce(PauseToken, &TimedWakeup) -> T,
{
    SCHEDULER.time_wake_list.with_wakeup(token, timeout, f)
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

fn run_time_driver_paused(token: PauseToken, instant: Duration) {
    SCHEDULER.time_wake_list.wakeup_until(token, instant);
}

fn run_scheduler_paused(token: PauseToken) {
    let thread = current_raw();

    let state = thread.state.get(token);
    let mut sched_queue = SCHEDULER.sched_queue.borrow_ref_mut(token);

    if state == thread::State::Ready {
        sched_queue.push_thread(thread);
    }

    let next = sched_queue
        .pop_thread()
        .expect("no available thread, scheduler is not running!");

    unsafe {
        set_current_paused(token, next);
    }
}

/// Run the scheduler from inside the appropriate interrupt context. Called from the BSP.
pub unsafe fn tick() {
    run_scheduler(|token| {
        if SCHEDULER.state.get(token) == State::Running {
            let now = crate::time::get_time();

            run_scheduler_paused(token);
            run_time_driver_paused(token, now);
        }
    });
}

/// Initialize the scheduler, switching it to running mode.
pub unsafe fn init() {
    const IDLE_STACK: usize = 4096;

    unsafe extern "C" fn idle_launcher(_: *mut (), _: *mut (), _: *mut (), _: *mut ()) -> ! {
        yield_now();
        loop {}
    }

    let idle = ThreadPtr::create(IDLE_STACK, IDLE_PRIORITY, c"Idle");
    unsafe {
        idle.init_switchctx(
            idle_launcher,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    // Install idle thread
    with_pause(|token| {
        SCHEDULER
            .sched_queue
            .borrow_ref_mut(token)
            .push_thread(idle);

        unsafe {
            set_current_paused(token, idle);
        }

        // We are ready! Set state to running
        SCHEDULER.state.set(token, State::Running);
    });
}
