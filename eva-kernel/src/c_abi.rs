use core::alloc::Layout;
use core::ffi::{CStr, c_char, c_int};
use core::ptr::{self, NonNull};

use crate::rt::sync::raw_condvar::RawCondvar;
use crate::rt::sync::raw_mutex::RawMutex;

use eva_abi::{
    Condvar2, Duration2, Mutex2, PauseToken, Priority, Thread, Thread2, ThreadFn, TlsDtor, TlsKey2,
};

fn convert_result<T>(res: Result<(), T>) -> c_int
where
    T: Into<c_int>,
{
    res.map(|_| eva_abi::OK).unwrap_or_else(Into::into)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_get_time() -> Duration2 {
    crate::time::get_time().into()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_kputs(str: *const c_char) {
    let str = unsafe { CStr::from_ptr(str) };
    crate::io::kwrite(str.to_bytes());
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_alloc(size: usize, align: usize) -> *mut () {
    let Ok(layout) = Layout::from_size_align(size, align) else {
        return ptr::null_mut();
    };

    unsafe { crate::allocator::alloc(layout).cast() }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_dealloc(ptr: *mut (), size: usize, align: usize) {
    unsafe { crate::allocator::dealloc(ptr.cast(), Layout::from_size_align_unchecked(size, align)) }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_emu_malloc(size: usize) -> *mut () {
    unsafe { crate::allocator::emu_malloc(size) }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_emu_free(ptr: *mut ()) {
    unsafe { crate::allocator::emu_free(ptr) }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_spawn(
    stack_size: usize,
    priority: Priority,
    entry: ThreadFn,
    name: *const c_char,
    user: *mut (),
) -> Thread2 {
    let name = unsafe { CStr::from_ptr(name) };
    crate::rt::spawn(stack_size, priority, entry, name, user).into()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_join_unchecked(thread: Thread2) -> c_int {
    let thread = unsafe { Thread::from_unchecked(thread) };
    convert_result(unsafe { crate::rt::join_unchecked(thread) })
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_get_priority(thread: Thread2) -> Priority {
    let thread = unsafe { Thread::from_unchecked(thread) };
    unsafe { crate::rt::get_priority(thread) }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_get_current_priority() -> Priority {
    crate::rt::get_current_priority()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_suspend_paused() {
    let token = unsafe { PauseToken::new() };
    crate::rt::suspend_paused(token);
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_suspend_and_yield() {
    crate::rt::suspend_and_yield();
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_suspend_and_yield_paused() {
    let token = unsafe { PauseToken::new() };
    crate::rt::suspend_and_yield_paused(token);
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_resume_unchecked(thread: Thread2) -> c_int {
    let thread = unsafe { Thread::from_unchecked(thread) };
    convert_result(unsafe { crate::rt::resume_unchecked(thread) })
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_resume_irq_unchecked(thread: Thread2) -> c_int {
    let thread = unsafe { Thread::from_unchecked(thread) };
    convert_result(unsafe { crate::rt::resume_irq_unchecked(thread) })
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_resume_paused_unchecked(thread: Thread2) -> c_int {
    let token = unsafe { PauseToken::new() };
    let thread = unsafe { Thread::from_unchecked(thread) };
    convert_result(unsafe { crate::rt::resume_paused_unchecked(token, thread) })
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_current() -> Thread2 {
    crate::rt::current().into()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_sleep_for(time: Duration2) {
    crate::rt::sleep_for(time.into());
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_sleep_until(time: Duration2) {
    crate::rt::sleep_until(time.into());
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_suspend_and_yield_for(time: Duration2) -> bool {
    crate::rt::suspend_and_yield_for(time.into())
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_suspend_and_yield_until(time: Duration2) -> bool {
    crate::rt::suspend_and_yield_until(time.into())
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_suspend_and_yield_paused_for(time: Duration2) -> bool {
    let token = unsafe { PauseToken::new() };
    crate::rt::suspend_and_yield_paused_for(token, time.into())
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_suspend_and_yield_paused_until(time: Duration2) -> bool {
    let token = unsafe { PauseToken::new() };
    crate::rt::suspend_and_yield_paused_until(token, time.into())
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_yield_now() {
    crate::rt::yield_now();
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_pend_yield() {
    let token = unsafe { PauseToken::new() };
    crate::rt::pause::pend_yield(token);
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_abort() {
    crate::rt::abort();
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_is_paused() -> bool {
    crate::rt::pause::is_paused()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_try_pause() -> bool {
    crate::rt::pause::try_pause()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_try_unpause() -> bool {
    unsafe { crate::rt::pause::try_unpause() }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_tls_key_create(dtor: Option<TlsDtor>) -> TlsKey2 {
    crate::rt::tls::key_create(dtor).into()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_tls_key_delete(key: TlsKey2) -> c_int {
    let Ok(key) = key.try_into() else {
        return eva_abi::error::TLS_INVALID_KEY_ERROR;
    };
    convert_result(crate::rt::tls::key_delete(key))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_tls_set_specific(key: TlsKey2, data: *mut ()) -> c_int {
    let Ok(key) = key.try_into() else {
        return eva_abi::error::TLS_INVALID_KEY_ERROR;
    };
    convert_result(crate::rt::tls::set_specific(key, NonNull::new(data)))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_tls_get_specific(key: TlsKey2) -> *mut () {
    let Ok(key) = key.try_into() else {
        return ptr::null_mut();
    };
    crate::rt::tls::get_specific(key)
        .map(|ptr| ptr.as_ptr())
        .unwrap_or(ptr::null_mut())
}

unsafe fn to_rt_mutex<'a>(mutex: *mut Mutex2) -> &'a RawMutex {
    unsafe {
        mutex
            .cast::<RawMutex>()
            .as_ref()
            .expect("invalid ptr passed to C ABI")
    }
}

unsafe fn to_rt_condvar<'a>(mutex: *mut Condvar2) -> &'a RawCondvar {
    unsafe {
        mutex
            .cast::<RawCondvar>()
            .as_ref()
            .expect("invalid ptr passed to C ABI")
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_sync_mutex_try_lock(mutex: *mut Mutex2) -> bool {
    let mutex = unsafe { to_rt_mutex(mutex) };
    mutex.try_lock()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_sync_mutex_try_lock_for(mutex: *mut Mutex2, time: Duration2) -> bool {
    let mutex = unsafe { to_rt_mutex(mutex) };
    mutex.try_lock_for(time.into())
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_sync_mutex_try_lock_until(
    mutex: *mut Mutex2,
    time: Duration2,
) -> bool {
    let mutex = unsafe { to_rt_mutex(mutex) };
    mutex.try_lock_until(time.into())
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_sync_mutex_lock(mutex: *mut Mutex2) {
    let mutex = unsafe { to_rt_mutex(mutex) };
    mutex.lock()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_sync_mutex_unlock(mutex: *mut Mutex2) {
    let mutex = unsafe { to_rt_mutex(mutex) };
    unsafe { mutex.unlock() }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_sync_mutex_is_locked(mutex: *mut Mutex2) -> bool {
    let mutex = unsafe { to_rt_mutex(mutex) };
    mutex.is_locked()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_sync_condvar_wait(cvar: *mut Condvar2, mutex: *mut Mutex2) {
    let cvar = unsafe { to_rt_condvar(cvar) };
    let mutex = unsafe { to_rt_mutex(mutex) };
    unsafe { cvar.wait(mutex) }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_sync_condvar_wait_for(
    cvar: *mut Condvar2,
    mutex: *mut Mutex2,
    time: Duration2,
) -> bool {
    let cvar = unsafe { to_rt_condvar(cvar) };
    let mutex = unsafe { to_rt_mutex(mutex) };
    unsafe { cvar.wait_for(mutex, time.into()) }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_sync_condvar_wait_until(
    cvar: *mut Condvar2,
    mutex: *mut Mutex2,
    time: Duration2,
) -> bool {
    let cvar = unsafe { to_rt_condvar(cvar) };
    let mutex = unsafe { to_rt_mutex(mutex) };
    unsafe { cvar.wait_until(mutex, time.into()) }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_sync_condvar_notify_one(cvar: *mut Condvar2) {
    let cvar = unsafe { to_rt_condvar(cvar) };
    cvar.notify_one()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn eva_c_rt_sync_condvar_notify_all(cvar: *mut Condvar2) {
    let cvar = unsafe { to_rt_condvar(cvar) };
    cvar.notify_all()
}
