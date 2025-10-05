use core::cell::Cell;
use core::marker::PhantomPinned;
use core::pin::Pin;
use core::ptr::{self, NonNull};

use crate::utils::assert::harden_assert;

#[derive(Debug)]
struct PinnedPtr<T>(NonNull<T>);

impl<T> PinnedPtr<T> {
    unsafe fn get_ref<'a>(self) -> Pin<&'a T> {
        unsafe { Pin::new_unchecked(self.0.as_ref()) }
    }
}

impl<T> Clone for PinnedPtr<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<Pin<&T>> for PinnedPtr<T> {
    fn from(value: Pin<&T>) -> Self {
        Self(NonNull::from(&*value))
    }
}

impl<T> Copy for PinnedPtr<T> {}

unsafe impl<T: Sync> Send for PinnedPtr<T> {}
unsafe impl<T: Sync> Sync for PinnedPtr<T> {}

type NodePtr<T: DoubleLinkImpl> = PinnedPtr<T::Node>;

pub struct DoubleLink<T: DoubleLinkImpl> {
    next: Cell<Option<NodePtr<T>>>,
    prev: Cell<Option<NodePtr<T>>>,
    _pin: PhantomPinned,
}

impl<T: DoubleLinkImpl> DoubleLink<T> {
    pub const fn unlinked() -> Self {
        Self {
            next: Cell::new(None),
            prev: Cell::new(None),
            _pin: PhantomPinned,
        }
    }
}

pub trait DoubleLinkImpl: Sized {
    type Node;

    fn get_links(node: Pin<&Self::Node>) -> Pin<&DoubleLink<Self>>;
}

#[inline(always)]
fn link2<T: DoubleLinkImpl>(before: Pin<&T::Node>, after: Pin<&T::Node>) {
    T::get_links(before).next.set(Some(PinnedPtr::from(after)));
    T::get_links(after).prev.set(Some(PinnedPtr::from(before)));
}

#[inline(always)]
fn link3<T: DoubleLinkImpl>(before: Pin<&T::Node>, center: Pin<&T::Node>, after: Pin<&T::Node>) {
    link2::<T>(before, center);
    link2::<T>(center, after);
}

#[inline(always)]
fn autolink<T: DoubleLinkImpl>(node: Pin<&T::Node>) {
    link2::<T>(node, node);
}

#[inline(always)]
fn unlink<T: DoubleLinkImpl>(node: Pin<&T::Node>) {
    T::get_links(node).next.set(None);
    T::get_links(node).prev.set(None);
}

#[inline(always)]
pub fn is_same<T: DoubleLinkImpl>(left: Pin<&T::Node>, right: Pin<&T::Node>) -> bool {
    ptr::addr_eq(&*left, &*right)
}

#[inline(always)]
pub fn is_linked<T: DoubleLinkImpl>(node: Pin<&T::Node>) -> bool {
    let links = T::get_links(node);
    harden_assert!(
        links.next.get().is_some() != links.prev.get().is_some(),
        "linked list inconsistent"
    );

    links.next.get().is_some()
}

#[inline(always)]
unsafe fn before<'a, T: DoubleLinkImpl>(node: Pin<&T::Node>) -> Option<Pin<&'a T::Node>> {
    T::get_links(node).prev.get().map(|ptr| unsafe {
        // SAFETY: The caller ensures correct lifetime
        ptr.get_ref()
    })
}

#[inline(always)]
unsafe fn before_unchecked<'a, T: DoubleLinkImpl>(node: Pin<&T::Node>) -> Pin<&'a T::Node> {
    let prev = unsafe {
        // SAFETY: The caller ensures correct lifetime
        before::<T>(node)
    };

    unsafe {
        harden_assert!(prev.is_some(), "called before_unchecked on unlinked node");
        // SAFETY: The caller ensures that this is possible
        prev.unwrap_unchecked()
    }
}

#[inline(always)]
unsafe fn after<'a, T: DoubleLinkImpl>(node: Pin<&T::Node>) -> Option<Pin<&'a T::Node>> {
    T::get_links(node).next.get().map(|ptr| unsafe {
        // SAFETY: The caller ensures correct lifetime
        ptr.get_ref()
    })
}

#[inline(always)]
unsafe fn after_unchecked<'a, T: DoubleLinkImpl>(node: Pin<&T::Node>) -> Pin<&'a T::Node> {
    let next = unsafe {
        // SAFETY: The caller ensures correct lifetime
        after::<T>(node)
    };

    unsafe {
        harden_assert!(next.is_some(), "called after_unchecked on unlinked node");
        // SAFETY: The caller ensures that this is possible
        next.unwrap_unchecked()
    }
}

pub struct DoubleLinkedList<T: DoubleLinkImpl> {
    head: Cell<Option<NodePtr<T>>>,
}

impl<T: DoubleLinkImpl> DoubleLinkedList<T> {
    /// Construct a new, empty doubly linked list.
    pub const fn empty() -> Self {
        Self {
            head: Cell::new(None),
        }
    }

    /// Returns wether this list is empty or not.
    pub fn is_empty(&self) -> bool {
        self.head.get().is_none()
    }

    /// Retrieve the head of the list.
    /// # Safety
    /// The caller must ensure correct lifetime for the returned node.
    pub unsafe fn head<'a>(&self) -> Option<Pin<&'a T::Node>> {
        self.head.get().map(|ptr| unsafe {
            // SAFETY: The caller is responsible for ensuring that the lifetime is valid
            ptr.get_ref()
        })
    }

    /// Retrieve the tail of the list.
    /// # Safety
    /// The caller must ensure correct lifetime for the returned node.
    pub unsafe fn tail<'a>(&self) -> Option<Pin<&'a T::Node>> {
        let head = unsafe {
            // SAFETY: This node wont escape this scope
            self.head()?
        };

        unsafe {
            // SAFETY: The node is linked in the list, so there must be a before node
            // SAFETY: The caller is responsible for ensuring that the lifetime is valid
            Some(before_unchecked::<T>(head))
        }
    }

    /// Return an iterator over all of the nodes in the list.
    /// # Safety
    /// The caller must ensure correct lifetime for the returned nodes.
    pub unsafe fn iter<'a>(&'a self) -> impl Iterator<Item = Pin<&'a T::Node>> {
        let mut iter = unsafe {
            // SAFETY: This node wont escape this scope
            self.head()
        };

        core::iter::from_fn(move || {
            let node = iter?;
            let next = unsafe {
                // SAFETY: The node is linked in the list, so there must be an after node
                after_unchecked::<T>(node)
            };

            iter = if self.is_head(next) { None } else { Some(next) };

            Some(node)
        })
    }

    #[inline(always)]
    fn set_head(&self, node: Pin<&T::Node>) {
        self.head.set(Some(PinnedPtr::from(node)));
    }

    #[inline(always)]
    fn init_head(&self, node: Pin<&T::Node>) {
        self.set_head(node);
        autolink::<T>(node);
    }

    unsafe fn insert_before(&self, next: Pin<&T::Node>, node: Pin<&T::Node>) {
        harden_assert!(is_linked::<T>(next), "next is not linked");
        harden_assert!(!self.is_empty(), "list is empty");

        let prev = unsafe {
            // SAFETY:
            let prev = before::<T>(next);

            harden_assert!(prev.is_some(), "node is unlinked and in list");
            // SAFETY: If a node is inside the list, it must be linked to some other node
            prev.unwrap_unchecked()
        };

        link3::<T>(prev, node, next);

        if self.is_head(next) {
            self.set_head(node);
        }
    }

    pub unsafe fn insert_after(&self, prev: Pin<&T::Node>, node: Pin<&T::Node>) {
        harden_assert!(is_linked::<T>(prev), "next is not linked");
        harden_assert!(!self.is_empty(), "list is empty");

        let next = unsafe {
            // SAFETY:
            let next = after::<T>(prev);

            harden_assert!(next.is_some(), "node is unlinked and in list");
            // SAFETY: If a node is inside the list, it must be linked to some other node
            next.unwrap_unchecked()
        };

        link3::<T>(prev, node, next);
    }

    pub unsafe fn insert_sorted_by_key<K, F>(&self, node: Pin<&T::Node>, mut f: F)
    where
        F: FnMut(Pin<&T::Node>) -> K,
        K: Ord,
    {
        let point = unsafe {
            // SAFETY: We guarantee that the iterator or node does not escape this scope
            self.iter().find(|&node2| f(node2) > f(node))
        };

        if let Some(point) = point {
            unsafe {
                // SAFETY:
                self.insert_before(point, node);
            }
        } else {
            unsafe {
                // SAFETY:
                self.push_back(node);
            }
        }
    }

    pub unsafe fn push_front(&self, node: Pin<&T::Node>) {
        let head = unsafe {
            // SAFETY:
            self.head()
        };

        if let Some(head) = head {
            unsafe {
                // SAFETY:
                self.insert_before(head, node);
            }
        } else {
            // The list is empty
            self.init_head(node);
        }
    }

    pub unsafe fn push_back(&self, node: Pin<&T::Node>) {
        let head = unsafe {
            // SAFETY:
            self.head()
        };

        if let Some(head) = head {
            unsafe {
                // SAFETY:
                self.insert_before(head, node);
            }
        } else {
            // The list is empty
            self.init_head(node);
        }
    }

    unsafe fn remove_raw(&self, node: Pin<&T::Node>) {
        let next = unsafe {
            // SAFETY:
            after_unchecked::<T>(node)
        };

        let prev = unsafe {
            // SAFETY:
            before_unchecked::<T>(node)
        };

        link2::<T>(prev, next);
        unlink::<T>(node);
    }

    pub unsafe fn remove(&self, node: Pin<&T::Node>) {
        unsafe {
            // SAFETY:
            self.remove_raw(node);
        }

        // We might update the head
        if self.is_head_unchecked(node) {
            let next = unsafe {
                // SAFETY:
                after_unchecked::<T>(node)
            };

            if is_same::<T>(node, next) {
                // This was the only element
                self.head.set(None);
            } else {
                self.set_head(next);
            }
        }
    }

    pub unsafe fn pop_front<'a>(&'a self) -> Option<Pin<&'a T::Node>> {
        let node = unsafe {
            // SAFETY:
            self.head()?
        };

        unsafe {
            // SAFETY:
            self.remove_raw(node);
        }

        let next = unsafe {
            // SAFETY:
            after_unchecked::<T>(node)
        };

        if is_same::<T>(node, next) {
            // This was the only element
            self.head.set(None);
        } else {
            self.set_head(next);
        }

        Some(node)
    }

    pub unsafe fn pop_back<'a>(&'a self) -> Option<Pin<&'a T::Node>> {
        let node = unsafe {
            // SAFETY:
            self.head()?
        };

        unsafe {
            // SAFETY:
            self.remove_raw(node);
        }

        let next = unsafe {
            // SAFETY:
            after_unchecked::<T>(node)
        };

        if is_same::<T>(node, next) {
            // This was the only element
            self.head.set(None);
        }

        Some(node)
    }
}
