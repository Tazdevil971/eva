//! Threading and scheduler subsystem.

mod idle;
mod sched_queue;
mod thread;
mod thread_list;
mod wake_list;

pub mod pause;
pub mod sync;
pub mod time;
pub mod tls;

use core::cell::Cell;
use core::cell::UnsafeCell;
use core::ffi::CStr;
use core::num::NonZeroU32;
use core::sync::atomic::{AtomicU32, Ordering};
use core::time::Duration;
use core::{mem, ptr};

use eva_abi::ThreadId;
use pause::pend_yield;
use pause::{PauseCell, PauseToken, run_scheduler, with_pause, yield_now_from_paused};
use sched_queue::SchedQueue;
use thread::{AtomicThreadPtr, DeferAdapter, TermAction, ThreadLocalInner, ThreadPtr};
use thread_list::ThreadList;

use eva_abi::OsError;
pub use eva_abi::{Priority, Thread, ThreadFn};
use eva_utils::atomic_linked_list::AtomicLinkedList;

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
    sched_queue: PauseCell<UnsafeCell<SchedQueue>>,
    thread_list: PauseCell<UnsafeCell<ThreadList>>,
    resume_defer_list: AtomicLinkedList<DeferAdapter>,
    next_tid: AtomicU32,
    current: AtomicThreadPtr,
}

static SCHEDULER: Scheduler = Scheduler {
    state: PauseCell::cell(State::Idle),
    sched_queue: PauseCell::unsafe_cell(SchedQueue::new()),
    thread_list: PauseCell::unsafe_cell(ThreadList::new()),
    resume_defer_list: AtomicLinkedList::new(DeferAdapter),
    next_tid: AtomicU32::new(1),
    current: AtomicThreadPtr::new(None),
};

fn gen_tid() -> ThreadId {
    let id = SCHEDULER.next_tid.fetch_add(1, Ordering::Release);
    unsafe { ThreadId(NonZeroU32::new(id).unwrap_unchecked()) }
}

fn start() {
    with_pause(|token| {
        SCHEDULER.state.set(token, State::Running);
    })
}

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
    unsafe { SCHEDULER.current.load(Ordering::Relaxed).unwrap_unchecked() }
}

fn local_raw() -> &'static ThreadLocalInner {
    let current = current_raw();
    unsafe {
        // SAFETY: We obtained this thread via current_raw
        current.local.get()
    }
}

/// Retrieve the currently running thread.
#[unsafe(export_name = "eva_rt_current")]
pub fn current() -> Thread {
    current_raw().to_abi()
}

fn thread_exit_pad() -> ! {
    let thread = current_raw();

    {
        let local = unsafe {
            // SAFETY: We obtained this thread via current_raw
            thread.local.get()
        };

        // Run thread destructors
        local.store.run_dtors();
    }

    with_pause(|token| {
        // First transition to terminated
        thread.state.set(token, thread::State::Terminated);

        // Second run the termination action
        match thread.term_action.get(token) {
            TermAction::None => {
                // No-op
            }
            TermAction::Resume(thread) => {
                // Someone is waiting for our demise, notify them!
                let _ = resume_paused_raw(token, thread);
            }
            TermAction::Detach => {
                // We are detached, self-clean

                // First remove from thread list
                unsafe {
                    SCHEDULER
                        .thread_list
                        .as_mut_unchecked(token)
                        .remove_thread(thread);
                }

                // Finally defer its demise to the idle thread
                idle::defer_thread_destroy_paused(token, thread);
            }
        }
    });

    // Thread should no longer be schedulable
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
    // ============ Disabled for fast! ============
    // assert!(is_valid_prio(priority), "invalid priority");
    // ============ Disabled for fast! ============

    extern "C" fn launcher(arg1: *mut (), arg2: *mut (), _arg3: *mut (), _arg4: *mut ()) -> ! {
        let entry = unsafe { mem::transmute::<_, ThreadFn>(arg1) };

        (entry)(arg2);

        thread_exit_pad();
    }

    let thread = ThreadPtr::create(
        stack_size,
        priority,
        name,
        gen_tid(),
        launcher,
        entry as _,
        user,
        ptr::null_mut(),
        ptr::null_mut(),
    );

    with_pause(|token| unsafe {
        // Insert the new thread in the scheduler
        SCHEDULER
            .sched_queue
            .as_mut_unchecked(token)
            .push_thread(thread);

        // Insert it also in the active thread list
        SCHEDULER
            .thread_list
            .as_mut_unchecked(token)
            .add_thread(thread);
    });

    Some(thread.to_abi())
}

#[unsafe(export_name = "eva_rt_exists")]
pub fn exists(thread: Thread) -> bool {
    with_pause(|token| exists_paused(token, thread))
}

#[unsafe(export_name = "eva_rt_exists_paused")]
pub fn exists_paused(token: PauseToken, thread: Thread) -> bool {
    unsafe {
        SCHEDULER
            .thread_list
            .as_mut_unchecked(token)
            .exists(thread.0)
    }
}

unsafe fn join_inner(token: PauseToken, thread: Thread) -> Result<ThreadPtr, OsError> {
    let thread = unsafe { ThreadPtr::from_abi(thread) };

    // First check if another operation is already in progress
    if thread.term_action.get(token) != TermAction::None {
        return Err(OsError::NotJoinable);
    }

    if thread.state.get(token) != thread::State::Terminated {
        // Only wait if the thread isn't already terminated
        let current = current_raw();

        // Register our wake as the thread termination action
        thread.term_action.set(token, TermAction::Resume(current));
        loop {
            // Wait for someone to wake us
            suspend_and_yield_paused(token);
            // Check if the thread is actually terminated, we need to avoid spurious wakeups
            if thread.state.get(token) == thread::State::Terminated {
                break;
            }
        }
    }

    // Remove it from the thread list
    unsafe {
        // SAFETY: This thread is alive and guaranteed to be valid, it must be in the thread list
        SCHEDULER
            .thread_list
            .as_mut_unchecked(token)
            .remove_thread(thread);
    }

    Ok(thread)
}

#[unsafe(export_name = "eva_rt_join")]
pub fn join(thread: Thread) -> Result<(), OsError> {
    let thread = with_pause(|token| {
        // Check if the thread still exists
        if !exists_paused(token, thread) {
            return Err(OsError::InvalidThread);
        }

        unsafe { join_inner(token, thread) }
    })?;

    // Finally destroy the thread
    unsafe {
        thread.destroy();
    }

    Ok(())
}

/// Join a thread, destroying it in the process.
/// # Safety:
/// - `thread` must be the only instance pointing to this thread.
#[unsafe(export_name = "eva_rt_join_unchecked")]
pub unsafe fn join_unchecked(thread: Thread) -> Result<(), OsError> {
    let thread = with_pause(|token| unsafe { join_inner(token, thread) })?;

    // Finally destroy the thread
    unsafe {
        thread.destroy();
    }

    Ok(())
}

unsafe fn detach_inner(token: PauseToken, thread: Thread) -> Result<Option<ThreadPtr>, OsError> {
    let thread = unsafe { ThreadPtr::from_abi(thread) };

    // First check if another operation is already in progress
    if thread.term_action.get(token) != TermAction::None {
        return Err(OsError::NotJoinable);
    }

    if thread.state.get(token) == thread::State::Terminated {
        // The thread already terminated, clean-up now
        unsafe {
            SCHEDULER
                .thread_list
                .as_mut_unchecked(token)
                .remove_thread(thread);
        }

        Ok(Some(thread))
    } else {
        // Set its termination action as detach
        thread.term_action.set(token, TermAction::Detach);
        Ok(None)
    }
}

#[unsafe(export_name = "eva_rt_detach")]
pub fn detach(thread: Thread) -> Result<(), OsError> {
    let thread = with_pause(|token| {
        // Check if the thread still exists
        if !exists_paused(token, thread) {
            return Err(OsError::InvalidThread);
        }

        unsafe { detach_inner(token, thread) }
    })?;

    if let Some(thread) = thread {
        // If the thread needs to be destroyed now, do it
        unsafe {
            thread.destroy();
        }
    }

    Ok(())
}

#[unsafe(export_name = "eva_rt_detach_unchecked")]
pub unsafe fn detach_unchecked(thread: Thread) -> Result<(), OsError> {
    let thread = with_pause(|token| unsafe { detach_inner(token, thread) })?;

    if let Some(thread) = thread {
        // If the thread needs to be destroyed now, do it
        unsafe {
            thread.destroy();
        }
    }

    Ok(())
}

#[unsafe(export_name = "eva_rt_get_current_priority")]
pub fn get_current_priority() -> Priority {
    current_raw().tcb().priority
}

#[unsafe(export_name = "eva_rt_get_current_tid")]
pub fn get_current_tid() -> ThreadId {
    current_raw().tcb().id
}

fn resume_paused_raw(token: PauseToken, thread: ThreadPtr) -> Result<(), OsError> {
    let state = thread.state.get(token);
    if state != thread::State::Suspend {
        return Err(OsError::AlreadyRunning);
    }

    thread.state.set(token, thread::State::Ready);
    unsafe {
        SCHEDULER
            .sched_queue
            .as_mut_unchecked(token)
            .push_thread(thread);
    }

    // Check if we woke up a thread with higher priority
    let current = current_raw();
    if current.priority < thread.priority {
        // Pend a yield
        pend_yield(token);
    }

    Ok(())
}

fn resume_irq_raw(thread: ThreadPtr) {
    SCHEDULER.resume_defer_list.push_front(thread);
}

#[unsafe(export_name = "eva_rt_resume_irq_unchecked")]
pub unsafe fn resume_irq_unchecked(thread: Thread) {
    let thread = unsafe { ThreadPtr::from_abi(thread) };
    resume_irq_raw(thread);
}

#[unsafe(export_name = "eva_rt_resume_paused")]
pub fn resume_paused(token: PauseToken, thread: Thread) -> Result<(), OsError> {
    if !exists_paused(token, thread) {
        return Err(OsError::InvalidThread);
    }

    unsafe { resume_paused_unchecked(token, thread) }
}

/// Resume a thread from a suspend state. Usable in a paused context.
#[unsafe(export_name = "eva_rt_resume_paused_unchecked")]
pub unsafe fn resume_paused_unchecked(token: PauseToken, thread: Thread) -> Result<(), OsError> {
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

#[unsafe(export_name = "eva_rt_resume")]
pub fn resume(thread: Thread) -> Result<(), OsError> {
    with_pause(|token| resume_paused(token, thread))
}

/// Resume a thread from a suspend state.
#[unsafe(export_name = "eva_rt_resume_unchecked")]
pub unsafe fn resume_unchecked(thread: Thread) -> Result<(), OsError> {
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
    time::with_timed_wakeup(token, time, |_, wakeup| {
        suspend_and_yield_paused(token);
        !wakeup.is_expired()
    })
}

fn run_scheduler_defer_ops(token: PauseToken) {
    let mut resume = SCHEDULER.resume_defer_list.take();
    while let Some(thread) = resume.pop_front() {
        // Not much we can do about errors here
        let _ = resume_paused_raw(token, thread);
    }
}

fn run_scheduler_paused(token: PauseToken) {
    let thread = current_raw();

    let state = thread.state.get(token);
    let sched_queue = unsafe { SCHEDULER.sched_queue.as_mut_unchecked(token) };

    if state == thread::State::Ready {
        sched_queue.push_thread(thread);
    }

    let next = unsafe { sched_queue.pop_thread().unwrap_unchecked() };

    unsafe {
        set_current_paused(token, next);
    }
}

/// Run the scheduler from inside the appropriate interrupt context. Called from the BSP.
pub unsafe fn tick() {
    run_scheduler(|token| {
        if SCHEDULER.state.get(token) == State::Running {
            let now = crate::time::get_time();

            run_scheduler_defer_ops(token);
            run_scheduler_paused(token);
            time::run_time_driver_paused(token, now);
        }
    });
}

/// Initialize the scheduler, switching it to running mode.
pub unsafe fn init() {
    let idle = idle::create();

    // Install idle thread
    with_pause(|token| unsafe {
        SCHEDULER
            .sched_queue
            .as_mut_unchecked(token)
            .push_thread(idle);

        set_current_paused(token, idle);
    });
}
