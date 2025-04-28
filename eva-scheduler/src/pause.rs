use core::cell::{Cell, Ref, RefCell, RefMut, UnsafeCell};
use core::fmt;
use core::marker::PhantomData;

use crate::scheduler;

pub use scheduler::{is_paused, pause, unpause, with_pause, yield_now_paused};

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

pub struct PauseMutex<T> {
    inner: UnsafeCell<T>,
}

impl<T> PauseMutex<T> {
    /// Create a new `PauseMutex`.
    pub const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }

    pub fn borrow<'a>(&'a self, _token: PauseToken<'a>) -> &'a T {
        unsafe { &*self.inner.get() }
    }
}

impl<T> From<T> for PauseMutex<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Default> Default for PauseMutex<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> PauseMutex<RefCell<T>> {
    pub fn replace<'a>(&'a self, token: PauseToken<'a>, t: T) -> T {
        self.borrow(token).replace(t)
    }

    pub fn replace_with<'a, F>(&'a self, token: PauseToken<'a>, f: F) -> T
    where
        F: FnOnce(&mut T) -> T,
    {
        self.borrow(token).replace_with(f)
    }

    pub fn swap<'a>(&'a self, token: PauseToken<'a>, other: &RefCell<T>) {
        self.borrow(token).swap(other)
    }

    pub fn borrow_ref<'a>(&'a self, token: PauseToken<'a>) -> Ref<'a, T> {
        self.borrow(token).borrow()
    }

    pub fn borrow_ref_mut<'a>(&'a self, token: PauseToken<'a>) -> RefMut<'a, T> {
        self.borrow(token).borrow_mut()
    }
}

impl<T: Default> PauseMutex<RefCell<T>> {
    pub fn take<'a>(&'a self, token: PauseToken<'a>) -> T {
        self.borrow(token).take()
    }
}

impl<T> From<T> for PauseMutex<RefCell<T>> {
    fn from(value: T) -> Self {
        Self::new(RefCell::new(value))
    }
}

impl<T> PauseMutex<Cell<T>> {
    pub fn set<'a>(&self, token: PauseToken<'a>, val: T) {
        self.borrow(token).set(val);
    }

    pub fn swap<'a>(&self, token: PauseToken<'a>, other: &Cell<T>) {
        self.borrow(token).swap(other);
    }

    pub fn replace<'a>(&self, token: PauseToken<'a>, val: T) -> T {
        self.borrow(token).replace(val)
    }
}

impl<T: Copy> PauseMutex<Cell<T>> {
    pub fn get<'a>(&self, token: PauseToken<'a>) -> T {
        self.borrow(token).get()
    }

    pub fn update<'a>(&self, token: PauseToken<'a>, f: impl FnOnce(T) -> T) {
        let old = self.get(token);
        self.set(token, f(old));
    }
}

impl<T: Default> PauseMutex<Cell<T>> {
    pub fn take<'a>(&self, token: PauseToken<'a>) -> T {
        self.borrow(token).take()
    }
}

impl<T> From<T> for PauseMutex<Cell<T>> {
    fn from(value: T) -> Self {
        Self::new(Cell::new(value))
    }
}

unsafe impl<T: Send> Sync for PauseMutex<T> {}

impl<T: fmt::Debug> fmt::Debug for PauseMutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut d = f.debug_struct("PauseMutex");

        if is_paused() {
            let token = unsafe {
                // SAFETY: The scheduler is actually paused
                PauseToken::new()
            };

            d.field("value", self.borrow(token));
        } else {
            d.field("value", &"<unpaused>");
        }

        d.finish()
    }
}
