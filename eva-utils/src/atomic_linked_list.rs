use core::fmt::{self, Debug};
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::singly_linked_list::{self, SinglyLinkedList};

pub use singly_linked_list::{Adapter, Link, LinkOps as _};

pub struct AtomicLinkedList<A: Adapter + Clone> {
    head: AtomicPtr<A::Value>,
    adapter: A,
}

unsafe impl<A> Send for AtomicLinkedList<A>
where
    A: Adapter + Clone + Send,
    A::Value: Send,
{
}

unsafe impl<A> Sync for AtomicLinkedList<A>
where
    A: Adapter + Clone + Sync,
    A::Value: Sync,
{
}

impl<A: Adapter + Clone> AtomicLinkedList<A> {
    pub const fn new(adapter: A) -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
            adapter,
        }
    }

    unsafe fn node_to_links<'a>(&self, node: NonNull<A::Value>) -> &'a A::Link {
        unsafe { self.adapter.raw_to_link(node).as_ref() }
    }

    unsafe fn is_linked(&self, node: NonNull<A::Value>) -> bool {
        unsafe { self.node_to_links(node).is_linked() }
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

impl<A: Adapter + Clone> AtomicLinkedList<A> {
    /// Check if this list is empty.
    pub fn is_empty(&self) -> bool {
        // TODO(davide.mor): Review memory ordering here
        self.head.load(Ordering::SeqCst).is_null()
    }

    /// Push a new element to the front of the list.
    pub fn push_front(&self, node: A::Ptr) {
        let node = self.take_ownership(node);

        let mut head = self.head.load(Ordering::SeqCst);
        loop {
            // Update the next pointer on the new node
            unsafe {
                // SAFETY: take_ownership always returns a valid pointer
                // SAFETY: head is always a valid pointer
                self.node_to_links(node).set_next(NonNull::new(head));
            }

            // TODO(davide.mor): Review memory ordering here
            // Try to insert the node
            match self.head.compare_exchange(
                head,
                node.as_ptr(),
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => break,
                Err(new_head) => head = new_head,
            }
        }
    }

    /// Take the entirety of the queue.
    pub fn take(&self) -> SinglyLinkedList<A>
    where
        A: Clone,
    {
        // TODO(davide.mor): Review memory ordering here
        let head = self.head.swap(ptr::null_mut(), Ordering::SeqCst);
        SinglyLinkedList {
            head: NonNull::new(head),
            adapter: self.adapter.clone(),
        }
    }
}

impl<A: Adapter + Clone> Drop for AtomicLinkedList<A> {
    fn drop(&mut self) {
        // TODO(davide.mor): Review memory ordering here
        let head = self.head.load(Ordering::SeqCst);
        let _ = SinglyLinkedList {
            head: NonNull::new(head),
            adapter: self.adapter.clone(),
        };
    }
}

impl<A> Debug for AtomicLinkedList<A>
where
    A: Adapter + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().finish_non_exhaustive()
    }
}
