use core::cell::{Cell, Ref, RefCell, RefMut, UnsafeCell};
use core::fmt::{self, Debug};

use super::{PauseToken, if_paused};

use bytemuck::Zeroable;

#[repr(transparent)]
#[derive(Zeroable)]
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

    pub const fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }

    pub const fn borrow<'a>(&'a self, _token: PauseToken<'a>) -> &'a T {
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
    pub const fn ref_cell(value: T) -> Self {
        Self::new(RefCell::new(value))
    }

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
        Self::ref_cell(value)
    }
}

impl<T> PauseCell<Cell<T>> {
    pub const fn cell(value: T) -> Self {
        Self::new(Cell::new(value))
    }

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
        self.borrow(token).update(f);
    }
}

impl<T: Default> PauseCell<Cell<T>> {
    pub fn take<'a>(&self, token: PauseToken<'a>) -> T {
        self.borrow(token).take()
    }
}

impl<T> From<T> for PauseCell<Cell<T>> {
    fn from(value: T) -> Self {
        Self::cell(value)
    }
}

impl<T> PauseCell<UnsafeCell<T>> {
    pub const fn unsafe_cell(value: T) -> Self {
        Self::new(UnsafeCell::new(value))
    }

    pub const fn get<'a>(&self, token: PauseToken<'a>) -> *mut T {
        self.borrow(token).get()
    }

    pub const unsafe fn as_ref_unchecked<'a>(&'a self, token: PauseToken<'a>) -> &'a T {
        unsafe { self.borrow(token).get().as_ref().unwrap_unchecked() }
    }

    pub const unsafe fn as_mut_unchecked<'a>(&'a self, token: PauseToken<'a>) -> &'a mut T {
        unsafe { self.borrow(token).get().as_mut().unwrap_unchecked() }
    }
}

impl<T> From<T> for PauseCell<UnsafeCell<T>> {
    fn from(value: T) -> Self {
        Self::unsafe_cell(value)
    }
}

unsafe impl<T: Send> Sync for PauseCell<T> {}

impl<T: Debug> Debug for PauseCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Unpaused;

        impl Debug for Unpaused {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("<unpaused>")
            }
        }

        let mut d = f.debug_struct("PauseCell");

        if_paused(|token| {
            d.field("value", self.borrow(token));
        })
        .unwrap_or_else(|| {
            d.field("value", &Unpaused);
        });

        d.finish()
    }
}
