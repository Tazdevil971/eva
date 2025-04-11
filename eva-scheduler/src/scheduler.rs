use core::cell::RefCell;
use core::ptr;
use core::sync::atomic::{AtomicU8, Ordering};

use crate::pause::{PauseMutex, PauseToken};
use crate::portability;
use crate::raw_thread::{AtomicRawThread, RawThread, ThreadParams};

struct PauseState(AtomicU8);

impl PauseState {
    const SCHED_PEND: u8 = 1 << 0;
    const PAUSED: u8 = 1 << 1;

    const fn paused() -> Self {
        Self(AtomicU8::new(Self::PAUSED))
    }

    fn pause(&self) -> bool {
        let old = self.0.fetch_or(Self::PAUSED, Ordering::SeqCst);
        (old & Self::PAUSED) != 0
    }

    fn unpause(&self) -> bool {
        let old = self.0.fetch_and(!Self::PAUSED, Ordering::SeqCst);
        debug_assert!((old & Self::PAUSED) != 0);
        (old & Self::SCHED_PEND) != 0
    }

    fn is_paused(&self) -> bool {
        let value = self.0.load(Ordering::SeqCst);
        (value & Self::PAUSED) != 0
    }

    fn set_sched_pend(&self) {
        self.0.fetch_or(Self::SCHED_PEND, Ordering::SeqCst);
    }

    fn clear_sched_pend(&self) {
        self.0.fetch_and(!Self::SCHED_PEND, Ordering::SeqCst);
    }
}

struct Rings {
    prio: [Option<RawThread>; 8],
    idle: Option<RawThread>,
    // wait: Option<RawThread>,
}

pub(crate) struct Scheduler {
    rings: PauseMutex<RefCell<Rings>>,
    cur: AtomicRawThread,
    state: PauseState,
}

pub(crate) static SCHEDULER: Scheduler = Scheduler {
    rings: PauseMutex::new(RefCell::new(Rings {
        prio: [None; 8],
        idle: None,
    })),
    cur: AtomicRawThread::new(None),
    state: PauseState::paused(),
};

fn set_current(thread: RawThread) {
    // Also set current switch context
    unsafe {
        portability::set_global_switchctx(thread.switchctx_ptr());
    }
    SCHEDULER.cur.store(Some(thread), Ordering::SeqCst);
}

pub fn current() -> RawThread {
    SCHEDULER
        .cur
        .load(Ordering::SeqCst)
        .expect("No current thread, scheduler should not be running!")
}

pub unsafe fn pause() -> bool {
    SCHEDULER.state.pause()
}

pub unsafe fn unpause(state: bool) {
    if !state {
        if SCHEDULER.state.unpause() {
            // Someone pended a preemption, do it now
            portability::preempt();
        }
    }
}

pub fn with_pause<F, T>(f: F) -> T
where
    F: FnOnce(PauseToken<'_>) -> T,
{
    unsafe {
        let state = pause();
        let ret = f(unsafe { PauseToken::new() });
        unpause(state);
        ret
    }
}

pub fn ask_for_preempt(_token: PauseToken) {
    SCHEDULER.state.set_sched_pend();
}

pub fn is_paused() -> bool {
    SCHEDULER.state.is_paused()
}

unsafe fn set_idle(thread: RawThread) {
    debug_assert!(is_paused());
    // The scheduler should be stopped when setting idle!
    let token = unsafe { PauseToken::new() };

    let mut rings = SCHEDULER.rings.borrow_ref_mut(token);
    rings.idle = Some(thread);
}

pub(crate) unsafe fn add_ready(thread: RawThread) {
    with_pause(|token| unsafe {
        add_ready_pt(token, thread);
    });
}

pub(crate) unsafe fn add_ready_pt(token: PauseToken, thread: RawThread) {
    unsafe {
        let mut rings = SCHEDULER.rings.borrow_ref_mut(token);
        let ring = &mut rings.prio[thread.priority() as usize];
        if let Some(ring) = ring {
            thread.link_before(token, *ring);
        } else {
            *ring = Some(thread);
        }
    }
}

pub(crate) unsafe fn remove_ready(thread: RawThread) {
    with_pause(|token| unsafe {
        remove_ready_pt(token, thread);
    });
}

pub(crate) unsafe fn remove_ready_pt(token: PauseToken, thread: RawThread) {
    unsafe {
        let mut rings = SCHEDULER.rings.borrow_ref_mut(token);
        rings.prio[thread.priority() as usize] = thread.unlink(token);
    }
}

pub unsafe fn rotate() {
    // This is a special handler, this is the scheduler routine and it cannot be preempted
    let token = unsafe { PauseToken::new() };

    if SCHEDULER.state.is_paused() {
        SCHEDULER.state.set_sched_pend();
        return;
    }

    // We are currently running the scheduler
    SCHEDULER.state.clear_sched_pend();

    unsafe {
        let mut rings = SCHEDULER.rings.borrow_ref_mut(token);

        // Iterate over all priority rings
        for ring in rings.prio.iter_mut().filter_map(Option::as_mut) {
            let ready = ring.tcb().next.get(token);
            debug_assert!(unsafe { ring.tcb().flags.ready(token) });

            *ring = ready;
            set_current(ready);
            return;
        }

        // We did not find any, return the idle thread, which is always ready
        let ready = rings
            .idle
            .expect("No idle thread installed, scheduler should not be running!");

        set_current(ready);
    }
}

pub unsafe fn enter() -> ! {
    debug_assert!(is_paused());
    const IDLE_STACK: usize = 4096;

    unsafe extern "C" fn idle_entry() -> ! {
        unsafe { idle_task() }
    }

    let thread = RawThread::create(IDLE_STACK);

    unsafe {
        // Set this new thread as the idle one
        set_idle(thread);
        // Set this thread as the current one, as we are about to run it
        set_current(thread);
        // Unpause the kernel to enable scheduling
        SCHEDULER.state.unpause();
        // Enter the thread
        thread.enter_first_thread(idle_entry)
    }
}

unsafe fn idle_task() -> ! {
    loop {
        portability::preempt();
    }
}
