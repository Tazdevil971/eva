use core::cell::Cell;
use core::marker::PhantomPinned;
use core::pin::Pin;
use core::ptr::NonNull;

use crate::pause::{PauseMutex, PauseToken, with_pause};

#[repr(transparent)]
#[derive(Debug)]
struct PinnedPtr<T>(NonNull<T>);

impl<T> PinnedPtr<T> {
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

unsafe impl<T: Send> Send for PinnedPtr<T> {}
unsafe impl<T: Sync> Sync for PinnedPtr<T> {}

type NodePtr<T> = PinnedPtr<Node<T>>;
type RootPtr<T> = PinnedPtr<LinkedList<T>>;

#[derive(Debug)]
pub struct Node<T> {
    next: PauseMutex<Cell<Option<NodePtr<T>>>>,
    prev: PauseMutex<Cell<Option<NodePtr<T>>>>,
    root: PauseMutex<Cell<Option<RootPtr<T>>>>,
    value: T,
    _pin: PhantomPinned,
}

impl<T> Node<T> {
    pub const fn new(value: T) -> Self {
        Self {
            next: PauseMutex::new(Cell::new(None)),
            prev: PauseMutex::new(Cell::new(None)),
            root: PauseMutex::new(Cell::new(None)),
            value,
            _pin: PhantomPinned,
        }
    }

    pub const fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub const fn value(&self) -> &T {
        &self.value
    }

    fn link(self: Pin<&Self>, token: PauseToken, root: Pin<&LinkedList<T>>) {
        let old_root = self.root.replace(token, Some(root.into()));
        if let Some(root) = old_root {
            let root = unsafe { root.get_ref() };
            unsafe { root.remove(token, self) };
        }
    }

    pub fn is_linked(&self, token: PauseToken) -> bool {
        self.root.get(token).is_some()
    }

    pub fn is_linked2(&mut self) -> bool {
        self.root.get_mut().get().is_some()
    }

    pub fn root(self: Pin<&Self>, token: PauseToken) -> Option<Pin<&LinkedList<T>>> {
        self.root.get(token).map(|root| unsafe { root.get_ref() })
    }

    pub fn try_remove(self: Pin<&Self>, token: PauseToken) -> bool {
        let old_root = self.root.replace(token, None);
        if let Some(old_root) = old_root {
            let old_root = unsafe { old_root.get_ref() };
            unsafe { old_root.remove(token, self) };
            true
        } else {
            false
        }
    }

    pub fn try_insert_after(self: Pin<&Self>, token: PauseToken, node: Pin<&Node<T>>) -> bool {
        if let Some(root) = self.root(token) {
            unsafe { root.insert_after(token, self, node) };
            true
        } else {
            false
        }
    }

    pub fn try_insert_before(self: Pin<&Self>, token: PauseToken, node: Pin<&Node<T>>) -> bool {
        if let Some(root) = self.root(token) {
            unsafe { root.insert_before(token, self, node) };
            true
        } else {
            false
        }
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        assert!(!self.is_linked2(), "Dropped a rooted node!");
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: PauseMutex<Cell<Option<NodePtr<T>>>>,
    tail: PauseMutex<Cell<Option<NodePtr<T>>>>,
    _pin: PhantomPinned,
}

impl<T> LinkedList<T> {
    pub const fn empty() -> Self {
        Self {
            head: PauseMutex::new(Cell::new(None)),
            tail: PauseMutex::new(Cell::new(None)),
            _pin: PhantomPinned,
        }
    }

    pub fn is_empty(&self, token: PauseToken) -> bool {
        self.head.get(token).is_none() && self.tail.get(token).is_none()
    }

    pub fn is_empty2(&mut self) -> bool {
        self.head.get_mut().get().is_none() && self.tail.get_mut().get().is_none()
    }

    pub fn push_back(self: Pin<&Self>, token: PauseToken, node: Pin<&Node<T>>) {
        // Link the node with the current list
        node.link(token, self);

        // Replace the tail with the new node
        let old_prev = self.tail.replace(token, Some(node.into()));

        node.next.set(token, None);
        node.prev.set(token, old_prev);

        if let Some(old_prev) = old_prev {
            // Ok we have to link the prev node
            let old_prev = unsafe { old_prev.get_ref() };
            old_prev.next.set(token, Some(node.into()))
        } else {
            // The next node was the head, update that
            self.head.set(token, Some(node.into()));
        }
    }

    pub fn push_front(self: Pin<&Self>, token: PauseToken, node: Pin<&Node<T>>) {
        // Link the node with the current list
        node.link(token, self);

        // Replace the head with the new node
        let old_next = self.head.replace(token, Some(node.into()));

        node.next.set(token, old_next);
        node.prev.set(token, None);

        if let Some(old_next) = old_next {
            // Ok we have to link the next node
            let old_next = unsafe { old_next.get_ref() };
            old_next.prev.set(token, Some(node.into()))
        } else {
            // The next node was the tail, update that
            self.tail.set(token, Some(node.into()));
        }
    }

    pub fn first(&self, token: PauseToken) -> Option<Pin<&Node<T>>> {
        unsafe { self.head.get(token).map(|ptr| ptr.get_ref()) }
    }

    pub fn last(&self, token: PauseToken) -> Option<Pin<&Node<T>>> {
        unsafe { self.tail.get(token).map(|ptr| ptr.get_ref()) }
    }

    pub fn iter(&self, token: PauseToken) -> impl Iterator<Item = Pin<&Node<T>>> {
        let mut iter = self.head.get(token);
        core::iter::from_fn(move || {
            let value = unsafe { iter?.get_ref() };
            iter = value.next.get(token);

            Some(value)
        })
    }

    pub fn iter_rev(&self, token: PauseToken) -> impl Iterator<Item = Pin<&Node<T>>> {
        let mut iter = self.tail.get(token);
        core::iter::from_fn(move || {
            let value = unsafe { iter?.get_ref() };
            iter = value.prev.get(token);

            Some(value)
        })
    }

    unsafe fn insert_after(
        self: Pin<&Self>,
        token: PauseToken,
        prev: Pin<&Node<T>>,
        node: Pin<&Node<T>>,
    ) {
        // Link the node with the current list
        node.link(token, self);

        let old_next = prev.next.replace(token, Some(node.into()));

        node.next.set(token, old_next);
        node.prev.set(token, Some(prev.into()));

        if let Some(old_next) = old_next {
            // Ok we have to link the next node
            let old_next = unsafe { old_next.get_ref() };
            old_next.prev.set(token, Some(node.into()))
        } else {
            // The next node was the tail, update that
            self.tail.set(token, Some(node.into()));
        }
    }

    unsafe fn insert_before(
        self: Pin<&Self>,
        token: PauseToken,
        next: Pin<&Node<T>>,
        node: Pin<&Node<T>>,
    ) {
        // Link the node with the current list
        node.link(token, self);

        let old_prev = next.prev.replace(token, Some(node.into()));

        node.next.set(token, Some(next.into()));
        node.prev.set(token, old_prev);

        if let Some(old_prev) = old_prev {
            // Ok we have to link the prev node
            let old_prev = unsafe { old_prev.get_ref() };
            old_prev.next.set(token, Some(node.into()))
        } else {
            // The next node was the tail, update that
            self.head.set(token, Some(node.into()));
        }
    }

    unsafe fn remove(&self, token: PauseToken, node: Pin<&Node<T>>) {
        let next = node.next.replace(token, None);
        let prev = node.prev.replace(token, None);

        if let Some(prev) = prev {
            let prev = unsafe { prev.get_ref() };
            prev.next.set(token, next);
        } else {
            self.head.set(token, next);
        }

        if let Some(next) = next {
            let next = unsafe { next.get_ref() };
            next.prev.set(token, prev);
        } else {
            self.tail.set(token, prev);
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        assert!(self.is_empty2(), "Dropped a non empty queue!");
    }
}
