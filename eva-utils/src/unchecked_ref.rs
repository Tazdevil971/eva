use core::fmt::{self, Debug, Pointer};
use core::ops::Deref;
use core::ptr::NonNull;

// TODO: This is not very sound, find a better way of doing this!

/// Erase the lifetime of a reference, allowing it to be moved around more easily.
/// This only provides immutable access to the erased reference to provide _slightly_ more safety.
/// This has the same layout as `NonNull<T>` and is guaranteed to be ABI compatible with it.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct UncheckedRef<T> {
    ptr: NonNull<T>,
}

impl<T> UncheckedRef<T> {
    /// Construct a new `UncheckedRef<T>` by erasing the lifetime of an existing reference.
    /// # Safety
    /// You can view this object as an immutable reference, so you have to upheld all of the same requirements manually.
    /// - The object pointed by `value` must not be destroyed while this object is alive.
    /// - You cannot construct any mutable reference to the original object pointed by `value` (aka no mutable aliasing).
    pub unsafe fn erase_lifetime<'a>(value: &'a T) -> Self {
        Self {
            ptr: NonNull::from(value),
        }
    }

    /// Construct this erased reference from a raw pointer.
    /// # Safety
    /// - `ptr` must point to valid memory for the whole lifetime of this `UncheckedRef<T>`.
    /// - `ptr` must not be used elsewhere for mutable access of `T` (aka no mutable aliasing).
    pub unsafe fn from_raw(ptr: NonNull<T>) -> Self {
        Self { ptr }
    }

    /// Convert this object into a raw pointer to the referenced object.
    pub fn into_raw(self) -> NonNull<T> {
        self.ptr
    }
}

impl<T> Deref for UncheckedRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Pointer for UncheckedRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Pointer::fmt(&self.ptr.as_ptr(), f)
    }
}

impl<T: Debug> Debug for UncheckedRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&**self, f)
    }
}
