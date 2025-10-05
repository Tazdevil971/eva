use core::cell::Cell;
use core::marker::PhantomPinned;
use core::pin::{Pin, pin};
use core::ptr::{NonNull, addr_eq};

use crate::scheduler::pause::{PauseCell, PauseToken};

/// Internal wrapper around a pointer that comes from a pinned reference.
#[repr(transparent)]
#[derive(Debug)]
struct PinnedPtr<T>(NonNull<T>);

impl<T> PinnedPtr<T> {
    /// Retrieve a pinned reference from this pointer
    /// # Safety
    /// The underlying pointer must be valid.
    unsafe fn get_ref<'a>(self) -> Pin<&'a T> {
        unsafe { Pin::new_unchecked(self.0.as_ref()) }
    }
}

impl<T> From<Pin<&T>> for PinnedPtr<T> {
    fn from(value: Pin<&T>) -> Self {
        Self(NonNull::from(&*value))
    }
}

impl<T> Clone for PinnedPtr<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Copy for PinnedPtr<T> {}

unsafe impl<T: Sync> Send for PinnedPtr<T> {}
unsafe impl<T: Sync> Sync for PinnedPtr<T> {}

type NodePtr<T> = PinnedPtr<Node<T>>;

/// A single node inside the linked list.
#[derive(Debug)]
pub struct Node<T> {
    next: PauseCell<Cell<Option<NodePtr<T>>>>,
    prev: PauseCell<Cell<Option<NodePtr<T>>>>,
    value: T,
    _pin: PhantomPinned,
}

impl<T> Node<T> {
    /// Create a new scoped/pinned node.
    /// Remember to always unlink a node before returning!
    pub fn new<F, R>(value: T, f: F) -> R
    where
        F: FnOnce(Pin<&Node<T>>) -> R,
    {
        let node = Node {
            next: PauseCell::from(None),
            prev: PauseCell::from(None),
            value,
            _pin: PhantomPinned,
        };

        let node = pin!(node);
        let node = node.into_ref();

        f(node)
    }

    /// Retrieve the value inside of this node.
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Check if two nodes are the same (not if they contain the same value!).
    pub fn is_same(&self, other: &Node<T>) -> bool {
        addr_eq(self, other)
    }

    /// Does this node have a next node?
    pub fn has_next(&self, token: PauseToken) -> bool {
        self.next.get(token).is_some()
    }

    /// Does this node have a previous node?
    pub fn has_prev(&self, token: PauseToken) -> bool {
        self.prev.get(token).is_some()
    }

    /// Retrieve the next node in the list.
    /// # Safety
    /// Caller must ensure that the returned node has the correct lifetime.
    pub unsafe fn next<'a>(&self, token: PauseToken<'a>) -> Option<Pin<&'a Node<T>>> {
        self.next.get(token).map(|ptr| unsafe { ptr.get_ref() })
    }

    /// Retrieve the previous node in the list.
    /// # Safety
    /// Caller must ensure that the returned node has the correct lifetime.
    pub unsafe fn prev<'a>(&self, token: PauseToken<'a>) -> Option<Pin<&'a Node<T>>> {
        self.prev.get(token).map(|ptr| unsafe { ptr.get_ref() })
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: PauseCell<Cell<Option<NodePtr<T>>>>,
    tail: PauseCell<Cell<Option<NodePtr<T>>>>,
}

impl<T> LinkedList<T> {
    /// Create a new empty linked list.
    pub const fn new() -> Self {
        Self {
            head: PauseCell::new(Cell::new(None)),
            tail: PauseCell::new(Cell::new(None)),
        }
    }

    /// Check if this linked list is empty.
    pub fn is_empty(&self, token: PauseToken) -> bool {
        self.head.get(token).is_none() && self.tail.get(token).is_none()
    }

    /// Check if a given node is linked to this list.
    /// # Safety
    /// The provided node must be either unlinked or linked to **this** list.
    pub unsafe fn contains(&self, token: PauseToken, node: Pin<&Node<T>>) -> bool {
        unsafe {
            // SAFETY: The scheduler is paused so we have access to all the nodes in this scope
            self.front(token)
                .map(|node2| node2.is_same(&*node))
                .unwrap_or(false)
                || node.has_prev(token)
        }
    }

    /// Retrieve the front (or first) node of this queue.
    /// # Safety
    /// Caller must ensure that the returned node has the correct lifetime.
    pub unsafe fn front<'a>(&'a self, token: PauseToken<'a>) -> Option<Pin<&'a Node<T>>> {
        self.head.get(token).map(|ptr| unsafe { ptr.get_ref() })
    }

    /// Retrieve the back (or last) node of this queue.
    /// # Safety
    /// Caller must ensure that the returned node has the correct lifetime.
    pub unsafe fn back<'a>(&'a self, token: PauseToken<'a>) -> Option<Pin<&'a Node<T>>> {
        self.tail.get(token).map(|ptr| unsafe { ptr.get_ref() })
    }

    /// Retrieve an iterator over all nodes.
    /// # Safety
    /// Caller must ensure that the returned node and iterator has the correct lifetime.
    pub unsafe fn iter<'a>(
        &'a self,
        token: PauseToken<'a>,
    ) -> impl Iterator<Item = Pin<&'a Node<T>>> {
        let mut iter = unsafe {
            // SAFETY: Lifetime contract must be upheld by the caller
            self.front(token)
        };

        core::iter::from_fn(move || {
            let value = iter?;
            iter = unsafe {
                // SAFETY: Lifetime contract must be upheld by the caller
                value.next(token)
            };

            Some(value)
        })
    }

    /// Internal function used to link two nodes together.
    #[inline(always)]
    fn link2(&self, token: PauseToken, prev: Option<Pin<&Node<T>>>, next: Option<Pin<&Node<T>>>) {
        {
            let next = next.map(PinnedPtr::from);
            if let Some(prev) = prev {
                prev.next.set(token, next);
            } else {
                self.head.set(token, next);
            }
        }

        {
            let prev = prev.map(PinnedPtr::from);
            if let Some(next) = next {
                next.prev.set(token, prev);
            } else {
                self.tail.set(token, prev);
            }
        }
    }

    /// Remove a node from the linked list.
    /// # Safety
    /// The node must be linked to this exact linked list.
    pub unsafe fn remove(&self, token: PauseToken, node: Pin<&Node<T>>) {
        let next = node.next.take(token);
        let prev = node.prev.take(token);

        let next = next.map(|ptr| unsafe { ptr.get_ref() });
        let prev = prev.map(|ptr| unsafe { ptr.get_ref() });

        self.link2(token, prev, next);
    }

    /// Remove a node from the linked list only if it linked
    /// # Safety
    /// The provided node must be either unlinked or linked to **this** list.
    pub unsafe fn try_remove(&self, token: PauseToken, node: Pin<&Node<T>>) -> bool {
        if unsafe {
            // SAFETY: The caller must ensure that this is callable
            self.contains(token, node)
        } {
            unsafe {
                // SAFETY: We just checked that the node is inside this list
                self.remove(token, node);
            }
            true
        } else {
            false
        }
    }

    /// Insert a node after another one
    /// # Safety
    /// - `prev` must be linked to this exact linked list.
    /// - `node` must be removed before being dropped.
    pub unsafe fn insert_after(
        &self,
        token: PauseToken,
        prev: Option<Pin<&Node<T>>>,
        node: Pin<&Node<T>>,
    ) {
        let old_next = if let Some(prev) = prev {
            prev.next.get(token)
        } else {
            self.head.get(token)
        };

        let old_next = old_next.map(|ptr| unsafe { ptr.get_ref() });

        self.link2(token, prev, Some(node));
        self.link2(token, Some(node), old_next);
    }

    /// Insert a node before another one
    /// # Safety
    /// - `prev` must be linked to this exact linked list.
    /// - `node` must be removed before being dropped.
    pub unsafe fn insert_before(
        &self,
        token: PauseToken,
        next: Option<Pin<&Node<T>>>,
        node: Pin<&Node<T>>,
    ) {
        let old_prev = if let Some(next) = next {
            next.prev.get(token)
        } else {
            self.tail.get(token)
        };

        let old_prev = old_prev.map(|ptr| unsafe { ptr.get_ref() });

        self.link2(token, old_prev, Some(node));
        self.link2(token, Some(node), next);
    }

    /// Insert a node, keeping the list sorted by the given key.
    /// Nodes with matching keys are ordered in from right to left, in order of insertion.
    /// # Safety
    /// `node` must be removed before being dropped.
    pub unsafe fn insert_sorted_by_key<K, F>(
        &self,
        token: PauseToken,
        node: Pin<&Node<T>>,
        mut f: F,
    ) where
        F: FnMut(Pin<&Node<T>>) -> K,
        K: Ord,
    {
        unsafe {
            // SAFETY: We guarantee that the iterator or node does not escape this scope
            let point = self.iter(token).find(|&node2| f(node2) > f(node));
            // SAFETY: We just obtained point from an iterator, thus it's valid
            self.insert_before(token, point, node);
        }
    }

    /// Push a node to the front of the queue.
    /// # Safety
    /// `node` must be removed before being dropped.
    pub unsafe fn push_front(&self, token: PauseToken, node: Pin<&Node<T>>) {
        unsafe {
            self.insert_after(token, None, node);
        }
    }

    /// Push a node to the back of the queue.
    /// # Safety
    /// `node` must be removed before being dropped.
    pub unsafe fn push_back(&self, token: PauseToken, node: Pin<&Node<T>>) {
        unsafe {
            self.insert_before(token, None, node);
        }
    }

    /// Remove and return the first element of the list.
    /// # Safety
    /// Caller must ensure that the returned node and iterator has the correct lifetime.
    pub unsafe fn pop_front<'a>(&'a self, token: PauseToken<'a>) -> Option<Pin<&'a Node<T>>> {
        let front = unsafe {
            // SAFETY: Caller must ensure that this lifetime is valid
            self.front(token)
        };

        if let Some(front) = front {
            unsafe {
                // SAFETY: We just received this node from the list, it is valid and linked
                self.remove(token, front);
            }
        }

        front
    }

    /// Remove and return the last element of the list.
    /// # Safety
    /// Caller must ensure that the returned node and iterator has the correct lifetime.
    pub unsafe fn pop_back<'a>(&'a self, token: PauseToken<'a>) -> Option<Pin<&'a Node<T>>> {
        let back = unsafe {
            // SAFETY: Caller must ensure that this lifetime is valid
            self.back(token)
        };

        if let Some(back) = back {
            unsafe {
                // SAFETY: We just received this node from the list, it is valid and linked
                self.remove(token, back);
            }
        }

        back
    }
}
