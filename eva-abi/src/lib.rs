#![no_std]

use core::alloc::Layout;
use core::cell::UnsafeCell;
use core::ffi::{CStr, c_char, c_int};
use core::fmt::{self, Debug};
use core::marker::PhantomData;
use core::num::NonZeroU32;
use core::ptr::{self, NonNull};
use core::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsError {
    NotJoinable,
    AlreadyRunning,
    InvalidThread,
    InvalidTlsKey,
}

impl From<OsError> for c_int {
    fn from(value: OsError) -> Self {
        match value {
            OsError::NotJoinable => NOT_JOINABLE_ERROR,
            OsError::AlreadyRunning => ALREADY_RUNNING_ERROR,
            OsError::InvalidThread => INVALID_THREAD_ERROR,
            OsError::InvalidTlsKey => INVALID_TLS_KEY_ERROR,
        }
    }
}

pub const OK: c_int = 0;
pub const NOT_JOINABLE_ERROR: c_int = -1;
pub const ALREADY_RUNNING_ERROR: c_int = -2;
pub const INVALID_THREAD_ERROR: c_int = -3;
pub const INVALID_TLS_KEY_ERROR: c_int = -4;

/// Token representing the paused state of the scheduler, existence of this token proves that the scheduler is paused.
#[derive(Clone, Copy)]
pub struct PauseToken<'a> {
    // Token is COVARIANT over 'a!
    // It means that a shorter token is a subtype of a longer token!
    _marker: PhantomData<&'a ()>,
    _not_send_sync: PhantomData<*mut ()>,
}

impl PauseToken<'_> {
    /// Create a new `PauseToken`.
    /// # Safety
    /// Caller must guarantee that the scheduler will remain paused as long as this token exists.
    pub unsafe fn new() -> Self {
        Self {
            _marker: PhantomData,
            _not_send_sync: PhantomData,
        }
    }
}

impl fmt::Debug for PauseToken<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PauseToken")
    }
}

pub type Priority = i8;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreadId(pub NonZeroU32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreadId2(pub u32);

impl From<ThreadId> for ThreadId2 {
    fn from(value: ThreadId) -> Self {
        Self(value.0.get())
    }
}

impl ThreadId2 {
    pub const INVALID: Self = Self(0);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Thread(pub NonNull<()>);

impl Thread {
    pub unsafe fn from_unchecked(other: Thread2) -> Self {
        unsafe { Self(NonNull::new_unchecked(other.0)) }
    }
}

impl TryFrom<Thread2> for Thread {
    type Error = OsError;

    fn try_from(value: Thread2) -> Result<Self, OsError> {
        NonNull::new(value.0)
            .map(Thread)
            .ok_or(OsError::InvalidThread)
    }
}

unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Thread2(pub *mut ());

impl Thread2 {
    pub const INVALID: Self = Self(ptr::null_mut());
}

impl From<Option<Thread>> for Thread2 {
    fn from(value: Option<Thread>) -> Self {
        value.map(Into::into).unwrap_or(Self::INVALID)
    }
}

impl From<Thread> for Thread2 {
    fn from(value: Thread) -> Self {
        Thread2(value.0.as_ptr())
    }
}

unsafe impl Send for Thread2 {}
unsafe impl Sync for Thread2 {}

pub type ThreadFn = extern "C" fn(*mut ());

pub type TlsDtor = extern "C" fn(*mut ());

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TlsKey(pub NonZeroU32);

impl TryFrom<TlsKey2> for TlsKey {
    type Error = OsError;

    fn try_from(value: TlsKey2) -> Result<Self, OsError> {
        NonZeroU32::new(value.0)
            .map(TlsKey)
            .ok_or(OsError::InvalidTlsKey)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TlsKey2(pub u32);

impl TlsKey2 {
    pub const INVALID: Self = Self(0);
}

impl From<Option<TlsKey>> for TlsKey2 {
    fn from(value: Option<TlsKey>) -> Self {
        value.map(Into::into).unwrap_or(Self::INVALID)
    }
}

impl From<TlsKey> for TlsKey2 {
    fn from(value: TlsKey) -> Self {
        Self(value.0.get())
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Duration2 {
    pub secs: u64,
    pub nanos: u32,
}

impl From<Duration> for Duration2 {
    fn from(value: Duration) -> Self {
        Self {
            secs: value.as_secs(),
            nanos: value.subsec_nanos(),
        }
    }
}

impl From<Duration2> for Duration {
    fn from(value: Duration2) -> Self {
        Self::new(value.secs, value.nanos)
    }
}

#[repr(C, align(8))]
pub struct Mutex2(UnsafeCell<[u8; 32]>);

unsafe impl Send for Mutex2 {}
unsafe impl Sync for Mutex2 {}

impl Mutex2 {
    pub const INIT: Self = Self(UnsafeCell::new([0; 32]));
}

impl Debug for Mutex2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mutex2").finish_non_exhaustive()
    }
}

#[repr(C, align(8))]
pub struct Condvar2(UnsafeCell<[u8; 32]>);

impl Condvar2 {
    pub const INIT: Self = Self(UnsafeCell::new([0; 32]));
}

impl Debug for Condvar2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Condvar2").finish_non_exhaustive()
    }
}

unsafe impl Send for Condvar2 {}
unsafe impl Sync for Condvar2 {}

unsafe extern "Rust" {
    // General functions
    pub unsafe fn eva_get_time() -> Duration;
    pub unsafe fn eva_io_kwrite(data: &[u8]) -> usize;
    pub unsafe fn eva_io_kread(data: &mut [u8]) -> usize;

    // Allocation functions
    pub unsafe fn eva_mem_alloc(layout: Layout) -> *mut u8;
    pub unsafe fn eva_mem_dealloc(ptr: *mut u8, layout: Layout);

    // Threading functions
    pub unsafe fn eva_rt_spawn(
        stack_size: usize,
        priority: Priority,
        entry: ThreadFn,
        name: &CStr,
        user: *mut (),
    ) -> Option<Thread>;
    pub unsafe fn eva_rt_exists(thread: Thread) -> bool;
    pub unsafe fn eva_rt_exists_paused(token: PauseToken, thread: Thread) -> bool;
    pub unsafe fn eva_rt_join(thread: Thread) -> Result<(), OsError>;
    pub unsafe fn eva_rt_join_unchecked(thread: Thread) -> Result<(), OsError>;
    pub unsafe fn eva_rt_detach(thread: Thread) -> Result<(), OsError>;
    pub unsafe fn eva_rt_detach_unchecked(thread: Thread) -> Result<(), OsError>;

    // Thread getters
    // TODO(davide.mor): Better API?
    // pub unsafe fn eva_rt_get_name(thread: Thread) -> *const c_char;
    // TODO(davide.mor): Again, API sucks
    // pub unsafe fn eva_rt_get_priority(thread: Thread) -> Priority;
    // pub unsafe fn eva_rt_get_tid(thread: Thread) -> ThreadId;
    // pub unsafe fn eva_rt_get_priority_unchecked(thread: Thread) -> Priority;
    // pub unsafe fn eva_rt_get_tid_unchecked(thread: Thread) -> ThreadId;

    // TODO(davide.mor): Better API?
    // pub unsafe fn eva_rt_get_current_name() -> *const c_char;
    pub unsafe fn eva_rt_get_current_priority() -> Priority;
    pub unsafe fn eva_rt_get_current_tid() -> ThreadId;

    // Thread state management
    pub unsafe fn eva_rt_suspend_paused(token: PauseToken);
    pub unsafe fn eva_rt_suspend_and_yield();
    pub unsafe fn eva_rt_suspend_and_yield_paused(token: PauseToken);
    pub unsafe fn eva_rt_resume(thread: Thread) -> Result<(), OsError>;
    pub unsafe fn eva_rt_resume_unchecked(thread: Thread) -> Result<(), OsError>;
    pub unsafe fn eva_rt_resume_irq_unchecked(thread: Thread);
    pub unsafe fn eva_rt_resume_paused(token: PauseToken, thread: Thread) -> Result<(), OsError>;
    pub unsafe fn eva_rt_resume_paused_unchecked(
        token: PauseToken,
        thread: Thread,
    ) -> Result<(), OsError>;
    pub unsafe fn eva_rt_current() -> Thread;

    // Time functions
    pub unsafe fn eva_rt_sleep_for(time: Duration);
    pub unsafe fn eva_rt_sleep_until(time: Duration);
    pub unsafe fn eva_rt_suspend_and_yield_for(time: Duration) -> bool;
    pub unsafe fn eva_rt_suspend_and_yield_until(time: Duration) -> bool;
    pub unsafe fn eva_rt_suspend_and_yield_paused_for(token: PauseToken, time: Duration) -> bool;
    pub unsafe fn eva_rt_suspend_and_yield_paused_until(token: PauseToken, time: Duration) -> bool;

    // Yield and task control
    pub unsafe fn eva_rt_yield_now();
    pub unsafe fn eva_rt_pend_yield(token: PauseToken);

    // Scheduler functions
    pub unsafe fn eva_rt_abort();
    pub unsafe fn eva_rt_is_paused() -> bool;
    pub unsafe fn eva_rt_try_pause() -> bool;
    pub unsafe fn eva_rt_try_unpause() -> bool;

    // TLS functions
    pub unsafe fn eva_rt_tls_key_create(dtor: Option<TlsDtor>) -> TlsKey;
    pub unsafe fn eva_rt_tls_key_delete(key: TlsKey) -> Result<(), OsError>;
    pub unsafe fn eva_rt_tls_set_specific(key: TlsKey, data: *mut ()) -> Result<(), OsError>;
    pub unsafe fn eva_rt_tls_get_specific(key: TlsKey) -> Option<NonNull<()>>;

    // Mutex functions
    pub unsafe fn eva_rt_sync_mutex_try_lock(mutex: &Mutex2) -> bool;
    pub unsafe fn eva_rt_sync_mutex_try_lock_for(mutex: &Mutex2, time: Duration) -> bool;
    pub unsafe fn eva_rt_sync_mutex_try_lock_until(mutex: &Mutex2, time: Duration) -> bool;
    pub unsafe fn eva_rt_sync_mutex_lock(mutex: &Mutex2);
    pub unsafe fn eva_rt_sync_mutex_unlock(mutex: &Mutex2);
    pub unsafe fn eva_rt_sync_mutex_is_locked(mutex: &Mutex2);

    // Condvar functions
    pub unsafe fn eva_rt_sync_condvar_wait(cvar: &Condvar2, mutex: &Mutex2);
    pub unsafe fn eva_rt_sync_condvar_wait_for(
        cvar: &Condvar2,
        mutex: &Mutex2,
        time: Duration,
    ) -> bool;
    pub unsafe fn eva_rt_sync_condvar_wait_until(
        cvar: &Condvar2,
        mutex: &Mutex2,
        time: Duration,
    ) -> bool;
    pub unsafe fn eva_rt_sync_condvar_notify_one(cvar: &Condvar2);
    pub unsafe fn eva_rt_sync_condvar_notify_all(cvar: &Condvar2);
}

unsafe extern "C" {
    // General functions
    pub unsafe fn eva_c_get_time() -> Duration2;
    pub unsafe fn eva_c_io_kwrite(data: *const u8, len: usize) -> usize;
    pub unsafe fn eva_c_io_kread(data: *mut u8, len: usize) -> usize;

    // Allocation functions
    pub unsafe fn eva_c_alloc(size: usize, align: usize) -> *mut ();
    pub unsafe fn eva_c_dealloc(ptr: *mut (), size: usize, align: usize);
    pub unsafe fn eva_c_emu_malloc(size: usize) -> *mut ();
    pub unsafe fn eva_c_emu_free(ptr: *mut ());

    // Threading functions
    pub unsafe fn eva_c_rt_spawn(
        stack_size: usize,
        priority: Priority,
        entry: extern "C" fn(*mut ()),
        name: *const c_char,
        user: *mut (),
    ) -> Thread2;
    pub unsafe fn eva_c_rt_exists(thread: Thread2) -> bool;
    pub unsafe fn eva_c_rt_exists_paused(thread: Thread2) -> bool;
    pub unsafe fn eva_c_rt_join(thread: Thread2) -> c_int;
    pub unsafe fn eva_c_rt_join_unchecked(thread: Thread2) -> c_int;
    pub unsafe fn eva_c_rt_detach(thread: Thread2) -> c_int;
    pub unsafe fn eva_c_rt_detach_unchecked(thread: Thread2) -> c_int;

    // Thread getters
    pub unsafe fn eva_c_rt_get_current_priority() -> Priority;
    pub unsafe fn eva_c_rt_get_current_tid() -> ThreadId2;

    // Thread state management
    pub unsafe fn eva_c_rt_suspend_paused();
    pub unsafe fn eva_c_rt_suspend_and_yield();
    pub unsafe fn eva_c_rt_suspend_and_yield_paused();
    pub unsafe fn eva_c_rt_resume(thread: Thread2) -> c_int;
    pub unsafe fn eva_c_rt_resume_unchecked(thread: Thread2) -> c_int;
    pub unsafe fn eva_c_rt_resume_irq_unchecked(thread: Thread2);
    pub unsafe fn eva_c_rt_resume_paused(thread: Thread2) -> c_int;
    pub unsafe fn eva_c_rt_resume_paused_unchecked(thread: Thread2) -> c_int;
    pub unsafe fn eva_c_rt_current() -> Thread2;

    // Time functions
    pub unsafe fn eva_c_rt_sleep_for(time: Duration2);
    pub unsafe fn eva_c_rt_sleep_until(time: Duration2);
    pub unsafe fn eva_c_rt_suspend_and_yield_for(time: Duration2) -> bool;
    pub unsafe fn eva_c_rt_suspend_and_yield_until(time: Duration2) -> bool;
    pub unsafe fn eva_c_rt_suspend_and_yield_paused_for(time: Duration2) -> bool;
    pub unsafe fn eva_c_rt_suspend_and_yield_paused_until(time: Duration2) -> bool;

    // Yield and task control
    pub unsafe fn eva_c_rt_yield_now();
    pub unsafe fn eva_c_rt_pend_yield();

    // Scheduler functions
    pub unsafe fn eva_c_rt_abort();
    pub unsafe fn eva_c_rt_is_paused() -> bool;
    pub unsafe fn eva_c_rt_try_pause() -> bool;
    pub unsafe fn eva_c_rt_try_unpause() -> bool;

    // TLS functions
    pub unsafe fn eva_c_rt_tls_key_create(dtor: Option<extern "C" fn(*mut ())>) -> TlsKey2;
    pub unsafe fn eva_c_rt_tls_key_delete(key: TlsKey2) -> c_int;
    pub unsafe fn eva_c_rt_tls_set_specific(key: TlsKey2, data: *mut ()) -> c_int;
    pub unsafe fn eva_c_rt_tls_get_specific(key: TlsKey2) -> *mut ();

    // Mutex functions
    pub unsafe fn eva_c_rt_sync_mutex_try_lock(mutex: *mut Mutex2) -> bool;
    pub unsafe fn eva_c_rt_sync_mutex_try_lock_for(mutex: *mut Mutex2, time: Duration2) -> bool;
    pub unsafe fn eva_c_rt_sync_mutex_try_lock_until(mutex: *mut Mutex2, time: Duration2) -> bool;
    pub unsafe fn eva_c_rt_sync_mutex_lock(mutex: *mut Mutex2);
    pub unsafe fn eva_c_rt_sync_mutex_unlock(mutex: *mut Mutex2);
    pub unsafe fn eva_c_rt_sync_mutex_is_locked(mutex: *mut Mutex2) -> bool;

    // Condvar functions
    pub unsafe fn eva_c_rt_sync_condvar_wait(cvar: *mut Condvar2, mutex: *mut Mutex2);
    pub unsafe fn eva_c_rt_sync_condvar_wait_for(
        cvar: *mut Condvar2,
        mutex: *mut Mutex2,
        time: Duration2,
    ) -> bool;
    pub unsafe fn eva_c_rt_sync_condvar_wait_until(
        cvar: *mut Condvar2,
        mutex: *mut Mutex2,
        time: Duration2,
    ) -> bool;
    pub unsafe fn eva_c_rt_sync_condvar_notify_one(cvar: *mut Condvar2);
    pub unsafe fn eva_c_rt_sync_condvar_notify_all(cvar: *mut Condvar2);
}
