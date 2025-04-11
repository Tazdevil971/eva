use core::cell::{Cell, Ref, RefCell, RefMut, UnsafeCell};
use core::fmt;

use super::{PauseToken, if_paused};

pub struct PauseCell<T> {
    inner: UnsafeCell<T>,
}

impl<T> PauseCell<T> {
    /// Create a new `PauseCell`.
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

impl<T> From<T> for PauseCell<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Default> Default for PauseCell<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> PauseCell<RefCell<T>> {
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

impl<T: Default> PauseCell<RefCell<T>> {
    pub fn take<'a>(&'a self, token: PauseToken<'a>) -> T {
        self.borrow(token).take()
    }
}

impl<T> From<T> for PauseCell<RefCell<T>> {
    fn from(value: T) -> Self {
        Self::new(RefCell::new(value))
    }
}

impl<T> PauseCell<Cell<T>> {
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

impl<T: Copy> PauseCell<Cell<T>> {
    pub fn get<'a>(&self, token: PauseToken<'a>) -> T {
        self.borrow(token).get()
    }

    pub fn update<'a>(&self, token: PauseToken<'a>, f: impl FnOnce(T) -> T) {
        let old = self.get(token);
        self.set(token, f(old));
    }
}

impl<T: Default> PauseCell<Cell<T>> {
    pub fn take<'a>(&self, token: PauseToken<'a>) -> T {
        self.borrow(token).take()
    }
}

impl<T> From<T> for PauseCell<Cell<T>> {
    fn from(value: T) -> Self {
        Self::new(Cell::new(value))
    }
}

unsafe impl<T: Send> Sync for PauseCell<T> {}

impl<T: fmt::Debug> fmt::Debug for PauseCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut d = f.debug_struct("PauseCell");

        let _ = if_paused(|token| {
            d.field("value", self.borrow(token));
        }).ok_or_else(|| {
            d.field("value", &"<unpaused>");
        });

        d.finish()
    }
}
