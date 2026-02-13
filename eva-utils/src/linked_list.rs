use core::cell::Cell;
use core::fmt::{self, Debug};
use core::marker::PhantomPinned;
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicPtr, Ordering};

use bytemuck::Zeroable;

/// Structure used to embed the required pointers inside the node.
#[repr(align(2))]
#[derive(Debug)]
pub struct Link<T> {
    next: Cell<Option<NonNull<T>>>,
    prev: Cell<Option<NonNull<T>>>,
    _pin: PhantomPinned,
}

impl<T> Link<T> {
    // TODO: Provenance?
    /// Internal marker used to signal an unlinked node.
    const UNLINKED_MARKER: Option<NonNull<T>> =
        unsafe { Some(NonNull::new_unchecked(1 as *mut T)) };

    /// Create a new unlinked node.
    pub const fn unlinked() -> Self {
        Self {
            next: Cell::new(Self::UNLINKED_MARKER),
            prev: Cell::new(Self::UNLINKED_MARKER),
            _pin: PhantomPinned,
        }
    }
}

impl<T> LinkOps for Link<T> {
    type Value = T;

    fn is_linked(&self) -> bool {
        self.next.get() != Self::UNLINKED_MARKER
    }

    fn get_next(&self) -> Option<NonNull<Self::Value>> {
        self.next.get()
    }

    fn get_prev(&self) -> Option<NonNull<Self::Value>> {
        self.prev.get()
    }

    unsafe fn set_unlinked(&self) {
        self.next.set(Self::UNLINKED_MARKER);
        self.prev.set(Self::UNLINKED_MARKER);
    }

    unsafe fn set_next(&self, ptr: Option<NonNull<Self::Value>>) {
        self.next.set(ptr);
    }

    unsafe fn set_prev(&self, ptr: Option<NonNull<Self::Value>>) {
        self.prev.set(ptr);
    }
}

impl<T> Drop for Link<T> {
    fn drop(&mut self) {
        // Shouldn't happen, but check anyways
        assert!(!self.is_linked(), "dropped node linked to a list");
    }
}

/// Structure used to embed the required pointers inside the node.
#[repr(align(2))]
#[derive(Debug)]
pub struct AtomicLink<T> {
    next: AtomicPtr<T>,
    prev: AtomicPtr<T>,
    _pin: PhantomPinned,
}

impl<T> AtomicLink<T> {
    // TODO: Provenance?
    /// Internal marker used to signal an unlinked node.
    const UNLINKED_MARKER: *mut T = 1 as *mut T;

    /// Create a new unlinked node.
    pub const fn unlinked() -> Self {
        Self {
            next: AtomicPtr::new(Self::UNLINKED_MARKER),
            prev: AtomicPtr::new(Self::UNLINKED_MARKER),
            _pin: PhantomPinned,
        }
    }
}

impl<T> LinkOps for AtomicLink<T> {
    type Value = T;

    fn is_linked(&self) -> bool {
        // TODO(davide.mor): Review memory ordering here
        self.next.load(Ordering::Relaxed) != Self::UNLINKED_MARKER
    }

    fn get_next(&self) -> Option<NonNull<Self::Value>> {
        // TODO(davide.mor): Review memory ordering here
        NonNull::new(self.next.load(Ordering::Relaxed))
    }

    fn get_prev(&self) -> Option<NonNull<Self::Value>> {
        // TODO(davide.mor): Review memory ordering here
        NonNull::new(self.prev.load(Ordering::Relaxed))
    }

    unsafe fn set_unlinked(&self) {
        // TODO(davide.mor): Review memory ordering here
        self.next.store(Self::UNLINKED_MARKER, Ordering::Relaxed);
        self.prev.store(Self::UNLINKED_MARKER, Ordering::Relaxed);
    }

    unsafe fn set_next(&self, ptr: Option<NonNull<Self::Value>>) {
        // TODO(davide.mor): Review memory ordering here
        self.next.store(
            ptr.map(NonNull::as_ptr).unwrap_or(ptr::null_mut()),
            Ordering::Relaxed,
        );
    }

    unsafe fn set_prev(&self, ptr: Option<NonNull<Self::Value>>) {
        // TODO(davide.mor): Review memory ordering here
        self.prev.store(
            ptr.map(NonNull::as_ptr).unwrap_or(ptr::null_mut()),
            Ordering::Relaxed,
        );
    }
}

impl<T> Drop for AtomicLink<T> {
    fn drop(&mut self) {
        // Shouldn't happen, but check anyways
        assert!(!self.is_linked(), "dropped node linked to a list");
    }
}

pub trait LinkOps {
    /// The value
    type Value;

    fn is_linked(&self) -> bool;
    fn get_next(&self) -> Option<NonNull<Self::Value>>;
    fn get_prev(&self) -> Option<NonNull<Self::Value>>;

    unsafe fn set_unlinked(&self);
    unsafe fn set_next(&self, ptr: Option<NonNull<Self::Value>>);
    unsafe fn set_prev(&self, ptr: Option<NonNull<Self::Value>>);
}

/// Trait defining how a particular node should be used.
pub trait Adapter {
    /// Pointer type used to transfer ownership from/to the linked list.
    type Ptr;
    /// Value obtained by dereferencing the pointer, and obtained by borrowing from the list.
    type Value;
    /// Type of the links used.
    type Link: LinkOps<Value = Self::Value>;

    /// Obtain a pointer to the links from a raw pointer to the node.
    unsafe fn raw_to_link(&self, raw: NonNull<Self::Value>) -> NonNull<Self::Link>;

    /// Convert a `Ptr` to a raw pointer to `Value`.
    fn ptr_to_raw(&self, ptr: Self::Ptr) -> NonNull<Self::Value>;
    /// Convert a raw pointer to `Value` to a `Ptr`.
    /// # Safety
    /// - `raw` must be a valid pointer.
    unsafe fn ptr_from_raw(&self, raw: NonNull<Self::Value>) -> Self::Ptr;
}

/// This trait asserts that the pointer does not allow for sharing of the contents, once inserted in the linked list the linked list owns the value.
pub unsafe trait OwningAdapter: Adapter {}

/// Intrusive linked list.
#[derive(Zeroable)]
pub struct LinkedList<A: Adapter> {
    head: Option<NonNull<A::Value>>,
    tail: Option<NonNull<A::Value>>,
    adapter: A,
}

unsafe impl<A> Send for LinkedList<A>
where
    A: Adapter + Send,
    A::Value: Send,
{
}

unsafe impl<A> Sync for LinkedList<A>
where
    A: Adapter + Sync,
    A::Value: Sync,
{
}

impl<A: Adapter> LinkedList<A> {
    /// Create a new, empty, intrusive linked list.
    pub const fn new(adapter: A) -> Self {
        Self {
            head: None,
            tail: None,
            adapter,
        }
    }

    unsafe fn node_to_links<'a>(&self, node: NonNull<A::Value>) -> &'a A::Link {
        unsafe { self.adapter.raw_to_link(node).as_ref() }
    }

    /// Internal method used to link two nodes together.
    unsafe fn link2(&mut self, prev: Option<NonNull<A::Value>>, next: Option<NonNull<A::Value>>) {
        if let Some(prev) = prev {
            unsafe {
                self.node_to_links(prev).set_next(next);
            }
        } else {
            self.head = next;
        }

        if let Some(next) = next {
            unsafe {
                self.node_to_links(next).set_prev(prev);
            }
        } else {
            self.tail = prev;
        }
    }

    /// Internal method used to link three nodes together.
    unsafe fn link3(
        &mut self,
        prev: Option<NonNull<A::Value>>,
        center: NonNull<A::Value>,
        next: Option<NonNull<A::Value>>,
    ) {
        unsafe {
            self.link2(prev, Some(center));
            self.link2(Some(center), next);
        }
    }

    unsafe fn unlink(&self, node: NonNull<A::Value>) {
        unsafe { self.node_to_links(node).set_unlinked() }
    }

    unsafe fn is_linked(&self, node: NonNull<A::Value>) -> bool {
        unsafe { self.node_to_links(node).is_linked() }
    }

    unsafe fn next(&self, node: NonNull<A::Value>) -> Option<NonNull<A::Value>> {
        unsafe { self.node_to_links(node).get_next() }
    }

    unsafe fn prev(&self, node: NonNull<A::Value>) -> Option<NonNull<A::Value>> {
        unsafe { self.node_to_links(node).get_prev() }
    }

    unsafe fn release_ownership(&self, node: NonNull<A::Value>) -> A::Ptr {
        unsafe {
            // SAFETY: Caller assures that link is a valid pointer
            self.unlink(node);
            self.adapter.ptr_from_raw(node)
        }
    }

    fn take_ownership(&self, ptr: A::Ptr) -> NonNull<A::Value> {
        let node = self.adapter.ptr_to_raw(ptr);

        // Check that the node was not linked to anything
        assert!(
            unsafe { !self.is_linked(node) },
            "Attempted to link an already linked node!"
        );

        node
    }
}

impl<A: OwningAdapter> LinkedList<A> {
    /// Obtain an iterator over all of the elements of the list.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut A::Value> {
        let mut cur = self.head;

        core::iter::from_fn(move || {
            let mut node = cur?;
            cur = unsafe {
                // SAFETY: node is always a valid pointer
                self.next(node)
            };

            Some(unsafe {
                // SAFETY: node is always a valid pointer
                node.as_mut()
            })
        })
    }
}

impl<A: Adapter> LinkedList<A> {
    /// Check if this list is empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Remove all of the elements from this list
    pub fn clear(&mut self) {
        let mut cur = self.head;

        while let Some(node) = cur {
            cur = unsafe {
                // SAFETY: node is always a valid pointer
                self.next(node)
            };

            unsafe {
                // SAFETY: node is always a valid pointer
                self.release_ownership(node);
            }
        }

        self.head = None;
        self.tail = None;
    }

    /// Retrieve the first element of the linked list.
    pub fn front(&self) -> Option<&A::Value> {
        self.head.map(|node| unsafe {
            // SAFETY: head is always a valid pointer
            node.as_ref()
        })
    }

    /// Retrieve the last element of the linked list.
    pub fn back(&self) -> Option<&A::Value> {
        self.tail.map(|node| unsafe {
            // SAFETY: tail is always a valid pointer
            node.as_ref()
        })
    }

    /// Obtain an iterator over all of the elements of the list.
    pub fn iter(&self) -> impl Iterator<Item = &A::Value> {
        let mut cur = self.head;

        core::iter::from_fn(move || {
            let node = cur?;
            cur = unsafe {
                // SAFETY: node is always a valid pointer
                self.next(node)
            };

            Some(unsafe {
                // SAFETY: node is always a valid pointer
                node.as_ref()
            })
        })
    }

    /// Push a new element to the front of the list.
    pub fn push_front(&mut self, node: A::Ptr) {
        let node = self.take_ownership(node);

        unsafe {
            // SAFETY: take_ownership always returns a valid pointer
            // SAFETY: head is always a valid pointer
            self.link3(None, node, self.head);
        }
    }

    /// Push a new element to the back of the list.
    pub fn push_back(&mut self, node: A::Ptr) {
        let node = self.take_ownership(node);

        unsafe {
            // SAFETY: take_ownership always returns a valid pointer
            // SAFETY: tail is always a valid pointer
            self.link3(self.tail, node, None);
        }
    }

    /// Pop an element from the front of the list.
    pub fn pop_front(&mut self) -> Option<A::Ptr> {
        if let Some(head) = self.head {
            let next = unsafe {
                // SAFETY: head is always a valid pointer
                self.next(head)
            };

            unsafe {
                // SAFETY: next is always a valid pointer
                self.link2(None, next);
            }

            Some(unsafe {
                // SAFETY: head is always valid
                // SAFETY: head no longer belongs to the queue
                self.release_ownership(head)
            })
        } else {
            None
        }
    }

    /// Pop an element from the back of the list.
    pub fn pop_back(&mut self) -> Option<A::Ptr> {
        if let Some(tail) = self.tail {
            let prev = unsafe {
                // SAFETY: tail is always a valid pointer
                self.prev(tail)
            };

            unsafe {
                // SAFETY: prev is always a valid pointer
                self.link2(prev, None);
            }

            Some(unsafe {
                // SAFETY: tail is always valid
                // SAFETY: tail no longer belongs to the queue
                self.release_ownership(tail)
            })
        } else {
            None
        }
    }

    /// Pop an element from the front of the queue, if a predicate is true.
    pub fn pop_front_if(&mut self, f: impl FnOnce(&A::Value) -> bool) -> Option<A::Ptr> {
        if let Some(head) = self.head {
            {
                // Scope is needed to limit the lifetime of head
                let head = unsafe {
                    // SAFETY: head is always a valid pointer
                    // SAFETY: We reduced the scope that this reference doesn't outlive the node
                    head.as_ref()
                };

                if !f(head) {
                    return None;
                }
            }

            let next = unsafe {
                // SAFETY: head is always a valid pointer
                self.next(head)
            };

            unsafe {
                // SAFETY: next is always a valid pointer
                self.link2(None, next);
            }

            Some(unsafe {
                // SAFETY: head is always valid
                // SAFETY: head no longer belongs to the queue
                self.release_ownership(head)
            })
        } else {
            None
        }
    }

    /// Pop an element from the back of the queue, if a predicate is true.
    pub fn pop_back_if(&mut self, f: impl FnOnce(&A::Value) -> bool) -> Option<A::Ptr> {
        if let Some(tail) = self.tail {
            {
                // Scope is needed to limit the lifetime of tail
                let tail = unsafe {
                    // SAFETY: tail is always a valid pointer
                    // SAFETY: We reduced the scope that this reference doesn't outlive the node
                    tail.as_ref()
                };

                if !f(tail) {
                    return None;
                }
            }

            let prev = unsafe {
                // SAFETY: tail is always a valid pointer
                self.prev(tail)
            };

            unsafe {
                // SAFETY: prev is always a valid pointer
                self.link2(prev, None);
            }

            Some(unsafe {
                // SAFETY: tail is always valid
                // SAFETY: tail no longer belongs to the queue
                self.release_ownership(tail)
            })
        } else {
            None
        }
    }

    /// Obtain a cursor from a raw reference.
    /// Allows to obtain a reference to a node with only a raw pointer to it.
    /// This allows to store a reference to a node without having to worry about lifetimes.
    /// # Safety
    /// - `node` must be a valid pointer.
    /// - `node` must be a node currently inserted in this queue.
    pub unsafe fn cursor_mut_from_raw(&mut self, node: NonNull<A::Value>) -> CursorMut<'_, A> {
        CursorMut {
            cur: Some(node),
            list: self,
        }
    }

    /// Obtain a cursor to the front of the queue.
    pub fn cursor_front_mut(&mut self) -> CursorMut<'_, A> {
        CursorMut {
            cur: self.tail,
            list: self,
        }
    }

    /// Obtain a cursor to the back of the queue.
    pub fn cursor_back_mut(&mut self) -> CursorMut<'_, A> {
        CursorMut {
            cur: self.tail,
            list: self,
        }
    }
}

impl<A: Adapter> Drop for LinkedList<A> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<A> Debug for LinkedList<A>
where
    A: Adapter,
    A::Value: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// Mutable cursor to allow for positioned operations inside the linked list.
/// Conceptually this threats the linked list as a circular list where the two extremities are joined by a "fictitious" node.
pub struct CursorMut<'a, A: Adapter> {
    cur: Option<NonNull<A::Value>>,
    list: &'a mut LinkedList<A>,
}

impl<'a, A> CursorMut<'a, A>
where
    A: OwningAdapter,
{
    /// Obtain the currently pointed value, mutably.
    /// Returns None if this points to the fictitious node.
    pub fn current_mut(&mut self) -> Option<&mut A::Value> {
        self.cur.map(|mut node| unsafe {
            // SAFETY: cur is always a valid pointer
            node.as_mut()
        })
    }
}

impl<'a, A> CursorMut<'a, A>
where
    A: Adapter,
{
    /// Obtain the currently pointed value.
    /// Returns None if this points to the fictitious node.
    pub fn current(&self) -> Option<&A::Value> {
        self.cur.map(|node| unsafe {
            // SAFETY: cur is always a valid pointer
            node.as_ref()
        })
    }

    /// Moves to the next node.
    pub fn move_next(&mut self) {
        if let Some(cur) = self.cur {
            self.cur = unsafe {
                // SAFETY: cur is always a valid pointer
                self.list.next(cur)
            };
        } else {
            self.cur = self.list.head;
        }
    }

    /// Moves to the previous node.
    pub fn move_prev(&mut self) {
        if let Some(cur) = self.cur {
            self.cur = unsafe {
                // SAFETY: cur is always a valid pointer
                self.list.prev(cur)
            };
        } else {
            self.cur = self.list.tail;
        }
    }

    /// Insert after the current node.
    pub fn insert_after(&mut self, ptr: A::Ptr) {
        let next = if let Some(cur) = self.cur {
            unsafe {
                // SAFETY: cur is always a valid pointer
                self.list.next(cur)
            }
        } else {
            self.list.head
        };

        let link = self.list.take_ownership(ptr);

        unsafe {
            // SAFETY: cur is always a valid pointer
            // SAFETY: next is always a valid pointer
            self.list.link3(self.cur, link, next);
        }
    }

    /// Insert before the current node.
    pub fn insert_before(&mut self, ptr: A::Ptr) {
        let prev = if let Some(cur) = self.cur {
            unsafe {
                // SAFETY: cur is always a valid pointer
                self.list.prev(cur)
            }
        } else {
            self.list.tail
        };

        let link = self.list.take_ownership(ptr);

        unsafe {
            // SAFETY: cur is always a valid pointer
            // SAFETY: next is always a valid pointer
            self.list.link3(prev, link, self.cur);
        }
    }

    /// Remove the current node, and move to the next one.
    pub fn remove_current(&mut self) -> Option<A::Ptr> {
        if let Some(cur) = self.cur {
            let (next, prev) = unsafe {
                // SAFETY: cur is always a valid pointer
                (self.list.next(cur), self.list.prev(cur))
            };

            unsafe {
                // SAFETY: prev is always a valid pointer
                // SAFETY: next is always a valid pointer
                self.list.link2(prev, next);
            }

            self.cur = next;

            Some(unsafe {
                // SAFETY: cur is always valid
                // SAFETY: cur no longer belongs to the queue
                self.list.release_ownership(cur)
            })
        } else {
            None
        }
    }
}

impl<A> Debug for CursorMut<'_, A>
where
    A: Adapter,
    A::Value: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("CursorMut").field(&self.current()).finish()
    }
}
