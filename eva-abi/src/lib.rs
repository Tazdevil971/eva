type Priority = i8;

#[repr(C)]
struct ThreadPtr(usize);

extern "C" {
    // Threading functions
    fn eva_thread_spawn(
        stack_size: usize,
        priority: Priority,
        entry: unsafe extern "C" fn(*mut ()),
        user: *mut ()
    ) -> ThreadPtr;

    fn eva_thread_join(thread: ThreadPtr);
    fn eva_thread_resume(thread: ThreadPtr);
    fn eva_thread_suspend_paused(thread: ThreadPtr);
    fn eva_thread_resume_paused(thread: ThreadPtr);
    fn eva_thread_current() -> ThreadPtr;

    // Yield and task control
    fn eva_yield_now();
    fn eva_pend_yield();

    // Scheduler functions
    fn eva_scheduler_is_running() -> bool;
    fn eva_scheduler_is_paused() -> bool;
    fn eva_scheduler_try_pause() -> bool;
    fn eva_scheduler_unpause();

    // Mutex functions
    // TODO:
}