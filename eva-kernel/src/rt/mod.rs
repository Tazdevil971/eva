//! Threading and scheduler subsystem.

mod sched_queue;
mod thread;
mod wake_list;

pub mod pause;
pub mod sync;

use atomic::{Atomic, Ordering};
use bytemuck::NoUninit;

use core::cell::RefCell;
use core::time::Duration;
use core::{mem, ptr};

use pause::{PauseCell, PauseToken, with_pause, yield_now_from_paused};
use sched_queue::SchedQueue;
use thread::AtomicThreadPtr;
use wake_list::{TimedWakeList, TimedWakeup};

use crate::port::{self, Impl as _};

pub use thread::ThreadPtr;

/// Priority reserved to the idle thread.
pub const IDLE_PRIORITY: i8 = -1;
/// Minimum priority supported.
pub const MIN_PRIORITY: i8 = 0;
/// Maximum priority supported.
pub const MAX_PRIORITY: i8 = 31;

/// Check if a priority level is valid for user code.
pub fn is_valid_prio(prio: i8) -> bool {
    prio >= MIN_PRIORITY && prio <= MAX_PRIORITY
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, NoUninit)]
#[repr(u8)]
enum State {
    Shutdown,
    Running,
    Paused,
    PausedPend,
    Aborted,
}

/// Internal scheduler structure
struct Scheduler {
    state: Atomic<State>,
    // This is not possible, see https://github.com/Amanieu/atomic-rs/issues/49
    // current: Atomic<Option<ThreadPtr>>,
    current: AtomicThreadPtr,
    sched_queue: PauseCell<RefCell<SchedQueue>>,
    time_wake_list: TimedWakeList,
}

static SCHEDULER: Scheduler = Scheduler {
    state: Atomic::new(State::Shutdown),
    current: AtomicThreadPtr::new(None),
    sched_queue: PauseCell::new(RefCell::new(SchedQueue::new())),
    time_wake_list: TimedWakeList::new(),
};

/// Signature of a function to be used as the thread entrypoint.
pub type ThreadFn = unsafe extern "C" fn(*mut (), *mut (), *mut ());

/// An error indicating that the scheduler was already paused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlreadyPaused;

/// An error indicating that the scheduler was not paused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotPaused;

/// An error indicating that a thread join is already in process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlreadyJoined;

/// An error indicating that a thread is already in running state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlreadyRunning;

/// Check if the scheduler is paused.
pub fn is_paused() -> bool {
    let state = SCHEDULER.state.load(Ordering::SeqCst);
    matches!(state, State::Paused | State::PausedPend)
}

/// Try to pause the scheduler.
pub fn try_pause() -> Result<(), AlreadyPaused> {
    // TODO(davide.mor): Review memory ordering here
    let old_state = SCHEDULER.state.load(Ordering::SeqCst);
    match old_state {
        State::Running => {
            SCHEDULER.state.store(State::Paused, Ordering::SeqCst);
            Ok(())
        }
        State::Paused | State::PausedPend => Err(AlreadyPaused),
        State::Shutdown => {
            // No-op, there is no scheduler to pause
            Ok(())
        },
        State::Aborted => panic!("scheduler in abort state"),
    }
}

/// Try to unpause the scheduler.
/// # Safety
/// - No `PauseToken` must exist at this point.
pub unsafe fn try_unpause() -> Result<(), NotPaused> {
    // TODO(davide.mor): Review memory ordering here
    let old_state = SCHEDULER.state.load(Ordering::SeqCst);
    match old_state {
        State::Paused => {
            SCHEDULER.state.store(State::Running, Ordering::SeqCst);
            Ok(())
        }
        State::PausedPend => {
            SCHEDULER.state.store(State::Running, Ordering::SeqCst);
            yield_now();
            Ok(())
        }
        State::Running => Err(NotPaused),
        State::Shutdown => {
            // No-op, there is no scheduler to unpause
            Ok(())
        },
        State::Aborted => panic!("scheduler in abort state"),
    }
}

/// Pend a yield to happen as soon as the pause is ended.
pub fn pend_yield(_token: PauseToken) {
    // TODO(davide.mor): Review memory ordering here
    let old_state = SCHEDULER.state.load(Ordering::SeqCst);
    if old_state == State::Paused {
        SCHEDULER.state.store(State::PausedPend, Ordering::SeqCst);
    }

    // TODO(davide.mor): How should we handle other cases?
}

/// Immediately abort the scheduler, scheduling after this call will be disabled.
pub fn abort() {
    SCHEDULER.state.store(State::Aborted, Ordering::SeqCst);
}

unsafe fn set_current_paused(_token: PauseToken, thread: ThreadPtr) {
    thread.set_as_global_switchctx();
    SCHEDULER.current.store(Some(thread), Ordering::Relaxed);
}

/// Yield now to some other thread.
pub fn yield_now() {
    port::GlobalImpl::yield_now();
}

/// Retrieve the currently running thread.
pub fn current() -> ThreadPtr {
    // TODO(davide.mor): Review memory ordering here
    SCHEDULER
        .current
        .load(Ordering::Relaxed)
        .expect("No current thread, scheduler is running!")
}

fn thread_exit_pad() -> ! {
    let thread = current();

    with_pause(|token| {
        if let Some(join_wait_thread) = thread.tcb().join_wait_thread.get(token) {
            resume_paused(token, join_wait_thread).expect("thread in wake list but awake");
        }

        thread.tcb().state.set(token, thread::State::Zombie);
    });

    yield_now();
    unreachable!("zombie thread resumed running!")
}

/// Spawn a thread.
pub fn spawn(
    stack_size: usize,
    priority: i8,
    entry: ThreadFn,
    user1: *mut (),
    user2: *mut (),
    user3: *mut (),
) -> ThreadPtr {
    assert!(is_valid_prio(priority), "invalid priority");

    unsafe extern "C" fn launcher(arg1: *mut (), arg2: *mut (), arg3: *mut (), arg4: *mut ()) -> ! {
        let entry = unsafe { mem::transmute::<_, ThreadFn>(arg1) };

        unsafe {
            (entry)(arg2, arg3, arg4);
        }

        thread_exit_pad();
    }

    let thread = ThreadPtr::create(stack_size, priority, "Test");
    thread.init_switchctx(launcher, entry as _, user1, user2, user3);

    with_pause(|token| {
        SCHEDULER
            .sched_queue
            .borrow_ref_mut(token)
            .push_thread(thread);
    });

    thread
}

/// Join a thread, destroying it in the process.
/// # Safety:
/// - `thread` must be the only instance pointing to this thread.
pub unsafe fn join(thread: ThreadPtr) -> Result<(), AlreadyJoined> {
    let current = current();

    with_pause(|token| {
        if thread.tcb().state.get(token) == thread::State::Zombie {
            // The thread is already in zombie state
            return Ok(());
        }

        // Check that no one is already waiting on the thread
        if thread.tcb().join_wait_thread.get(token).is_some() {
            return Err(AlreadyJoined);
        }

        // The thread is still alive and kicking!
        // Register for wait!
        thread.tcb().join_wait_thread.set(token, Some(current));

        loop {
            // Wait for someone to wake us
            suspend_and_yield_paused(token);
            // Check if the thread is dead, do this to avoid spurious wakeups
            if thread.tcb().state.get(token) == thread::State::Zombie {
                break;
            }
        }

        Ok(())
    })?;

    // Finally destroy the thread
    unsafe {
        // SAFETY: The caller ensures that this is safe
        thread.destroy();
    }

    Ok(())
}

/// Resume a thread from a suspend state.
pub fn resume(thread: ThreadPtr) -> Result<(), AlreadyRunning> {
    with_pause(|token| resume_paused(token, thread))
}

/// Suspend executing of the current thread, after the next yield this thread won't be scheduled anymore.
pub fn suspend() {
    with_pause(|token| suspend_paused(token))
}

/// Resume a thread from a suspend state. Usable in a paused context.
pub fn resume_paused(token: PauseToken, thread: ThreadPtr) -> Result<(), AlreadyRunning> {
    let state = thread.tcb().state.get(token);
    if state == thread::State::Suspend {
        thread.tcb().state.set(token, thread::State::Ready);
        SCHEDULER
            .sched_queue
            .borrow_ref_mut(token)
            .push_thread(thread);
        Ok(())
    } else {
        Err(AlreadyRunning)
    }
}

/// Suspend executing of the current thread, after the next yield this thread won't be scheduled anymore. Usable in a paused context.
pub fn suspend_paused(token: PauseToken) {
    let thread = SCHEDULER
        .current
        .load(Ordering::Relaxed)
        .expect("no current thread, scheduler is not running!");

    let state = thread.tcb().state.get(token);
    if state == thread::State::Ready {
        thread.tcb().state.set(token, thread::State::Suspend);
    }
}

/// Suspend the current thread and yield to the scheduler.
pub fn suspend_and_yield_paused(token: PauseToken) {
    suspend_paused(token);
    yield_now_from_paused(token);
}

fn with_timed_wakeup<F, T>(token: PauseToken, timeout: Duration, f: F) -> T
where
    F: FnOnce(PauseToken, &TimedWakeup) -> T,
{
    SCHEDULER.time_wake_list.with_wakeup(token, timeout, f)
}

/// Sleep and yield for a given amount of time.
pub fn sleep(duration: Duration) {
    sleep_until(crate::time::get_time() + duration);
}

/// Sleep until a specific point in the future.
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
    let thread = SCHEDULER
        .current
        .load(Ordering::Relaxed)
        .expect("no current thread, scheduler is not running!");

    let state = thread.tcb().state.get(token);
    let mut sched = SCHEDULER.sched_queue.borrow_ref_mut(token);

    
    if state == thread::State::Ready {
        sched.push_thread(thread);
    }

    let next = sched
        .pop_thread()
        .expect("no available thread, scheduler is not running!");

    unsafe {
        set_current_paused(token, next);
    }
}

/// Run the scheduler from inside the appropriate interrupt context. Called from the BSP.
pub unsafe fn tick() {
    let state = SCHEDULER.state.load(Ordering::SeqCst);
    match state {
        State::Shutdown => {
            // Scheduler is in shutdown, do nothing
        }
        State::Paused => {
            // We are paused, pend a yield as soon as possible
            SCHEDULER.state.store(State::PausedPend, Ordering::SeqCst);
        }
        State::PausedPend => {
            // Already pended, do nothing
        }
        State::Running => {
            // Normal operation
            let token = unsafe {
                // SAFETY: We are inside the scheduler
                PauseToken::new()
            };

            let now = crate::time::get_time();

            run_scheduler_paused(token);
            run_time_driver_paused(token, now);
        }
        State::Aborted => {
            // Scheduler is in abort, do nothing
        }
    }
}

/// Initialize the scheduler, switching it to running mode.
pub unsafe fn init() {
    const IDLE_STACK: usize = 4096;

    unsafe extern "C" fn idle_launcher(_: *mut (), _: *mut (), _: *mut (), _: *mut ()) -> ! {
        yield_now();
        loop {}
    }

    let idle = ThreadPtr::create(IDLE_STACK, IDLE_PRIORITY, "Idle");
    idle.init_switchctx(
        idle_launcher,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    );

    let token = unsafe {
        // SAFETY: The scheduler has not been started yet, so this is safe
        PauseToken::new()
    };

    SCHEDULER
        .sched_queue
        .borrow_ref_mut(token)
        .push_thread(idle);

    unsafe {
        set_current_paused(token, idle);
    }

    SCHEDULER.state.store(State::Running, Ordering::SeqCst);
}
