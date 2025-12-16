use core::cell::Cell;
use core::fmt::{self, Debug};
use core::ptr::NonNull;

/// Structure used to embed the required pointers inside the node.
#[repr(align(2))]
#[derive(Debug)]
pub struct Link {
    next: Cell<Option<NonNull<Link>>>,
    prev: Cell<Option<NonNull<Link>>>,
}

unsafe impl Send for Link {}

impl Link {
    /// Internal marker used to signal an unlinked node.
    const UNLINKED_MARKER: Option<NonNull<Link>> =
        unsafe { Some(NonNull::new_unchecked(1 as *mut Link)) };

    /// Create a new unlinked node.
    pub const fn unlinked() -> Self {
        Self {
            next: Cell::new(Self::UNLINKED_MARKER),
            prev: Cell::new(Self::UNLINKED_MARKER),
        }
    }

    /// Check if this node is linked to any list.
    pub fn is_linked(&self) -> bool {
        self.next.get() != Self::UNLINKED_MARKER
    }

    fn set_next(&self, value: Option<NonNull<Link>>) {
        self.next.set(value)
    }

    fn set_prev(&self, value: Option<NonNull<Link>>) {
        self.prev.set(value)
    }

    fn set_unlinked(&self) {
        self.set_next(Self::UNLINKED_MARKER);
        self.set_prev(Self::UNLINKED_MARKER);
    }
}

/// Trait defining how a particular node should be used.
/// # Safety
/// This trait is unsafe because `offset_of_link` despite being safe to call, must return a valid offset.
pub unsafe trait Adapter {
    /// Pointer type used to transfer ownership from/to the linked list.
    type Ptr;
    /// Value obtained by dereferencing the pointer, and obtained by borrowing from the list.
    type Value;

    /// Offset in bytes of `Link` from the start of `Value`.
    fn offset_of_link(&self) -> usize;
    /// Convert a `Ptr` to a raw pointer to `Value`.
    fn ptr_to_raw(&self, ptr: Self::Ptr) -> NonNull<Self::Value>;
    /// Convert a raw pointer to `Value` to a `Ptr`.
    /// # Safety
    /// - `raw` must be a valid pointer.
    unsafe fn ptr_from_raw(&self, raw: NonNull<Self::Value>) -> Self::Ptr;
}

/// Intrusive linked list.
pub struct LinkedList<A> {
    head: Option<NonNull<Link>>,
    tail: Option<NonNull<Link>>,
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

impl<A> LinkedList<A> {
    /// Create a new, empty, intrusive linked list.
    pub const fn new(adapter: A) -> Self {
        Self {
            head: None,
            tail: None,
            adapter,
        }
    }

    /// Check if this list is empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Internal method used to link two nodes together.
    unsafe fn link2(&mut self, prev: Option<NonNull<Link>>, next: Option<NonNull<Link>>) {
        if let Some(prev) = prev {
            unsafe {
                prev.as_ref().set_next(next);
            }
        } else {
            self.head = next;
        }

        if let Some(next) = next {
            unsafe {
                next.as_ref().set_prev(prev);
            }
        } else {
            self.tail = prev;
        }
    }

    /// Internal method used to link three nodes together.
    unsafe fn link3(
        &mut self,
        prev: Option<NonNull<Link>>,
        center: NonNull<Link>,
        next: Option<NonNull<Link>>,
    ) {
        unsafe {
            self.link2(prev, Some(center));
            self.link2(Some(center), next);
        }
    }

    /// Internal method used to unlink a center node from a sequence of three nodes.
    unsafe fn unlink(link: NonNull<Link>) {
        unsafe { link.as_ref().set_unlinked() }
    }

    unsafe fn next(link: NonNull<Link>) -> Option<NonNull<Link>> {
        unsafe { link.as_ref().next.get() }
    }

    unsafe fn prev(link: NonNull<Link>) -> Option<NonNull<Link>> {
        unsafe { link.as_ref().prev.get() }
    }
}

impl<A> LinkedList<A>
where
    A: Adapter,
{
    unsafe fn link_to_raw(&self, link: NonNull<Link>) -> NonNull<A::Value> {
        unsafe {
            // SAFETY: adapter must return a valid offset
            // SAFETY: Caller assures that link is a valid pointer
            link.byte_sub(self.adapter.offset_of_link())
                .cast::<A::Value>()
        }
    }

    unsafe fn link_from_raw(&self, raw: NonNull<A::Value>) -> NonNull<Link> {
        unsafe {
            // SAFETY: adapter must return a valid offset
            // SAFETY: Caller assures that raw is a valid pointer
            raw.byte_add(self.adapter.offset_of_link()).cast::<Link>()
        }
    }

    unsafe fn link_to_ptr(&self, link: NonNull<Link>) -> A::Ptr {
        unsafe {
            // SAFETY: Caller assures that link is a valid pointer
            self.adapter.ptr_from_raw(self.link_to_raw(link))
        }
    }

    fn link_from_ptr(&self, ptr: A::Ptr) -> NonNull<Link> {
        let raw = self.adapter.ptr_to_raw(ptr);
        unsafe {
            // SAFETY: ptr_to_raw must return a valid pointer
            self.link_from_raw(raw)
        }
    }

    unsafe fn link_to_ref<'a>(&self, link: NonNull<Link>) -> &'a A::Value {
        unsafe {
            // SAFETY: Caller assures that link is a valid pointer
            // SAFETY: Caller assures that this does not alias
            self.link_to_raw(link).as_ref()
        }
    }

    unsafe fn link_to_ref_mut<'a>(&self, link: NonNull<Link>) -> &'a mut A::Value {
        unsafe {
            // SAFETY: Caller assures that link is a valid pointer
            // SAFETY: Caller assures that this does not alias
            self.link_to_raw(link).as_mut()
        }
    }
}

impl<A> LinkedList<A>
where
    A: Adapter,
{
    /// Retrieve the first element of the linked list.
    pub fn front(&self) -> Option<&A::Value> {
        self.head.map(|link| unsafe {
            // SAFETY: head is always a valid pointer
            self.link_to_ref(link)
        })
    }

    /// Retrieve the last element of the linked list.
    pub fn back(&self) -> Option<&A::Value> {
        self.tail.map(|link| unsafe {
            // SAFETY: tail is always a valid pointer
            self.link_to_ref(link)
        })
    }

    /// Obtain an iterator over all of the elements of the list.
    pub fn iter(&self) -> impl Iterator<Item = &A::Value> {
        let mut cur = self.head;

        core::iter::from_fn(move || {
            let link = cur?;
            cur = unsafe {
                // SAFETY: link is always a valid pointer
                LinkedList::<A>::next(link)
            };

            Some(unsafe {
                // SAFETY: link is always a valid pointer
                self.link_to_ref(link)
            })
        })
    }

    /// Obtain an iterator over all of the elements of the list.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut A::Value> {
        let mut cur = self.head;

        core::iter::from_fn(move || {
            let link = cur?;
            cur = unsafe {
                // SAFETY: link is always a valid pointer
                LinkedList::<A>::next(link)
            };

            Some(unsafe {
                // SAFETY: link is always a valid pointer
                self.link_to_ref_mut(link)
            })
        })
    }

    /// Push a new element to the front of the list.
    pub fn push_front(&mut self, node: A::Ptr) {
        let link = self.link_from_ptr(node);

        unsafe {
            // SAFETY: link_from_ptr always returns a valid pointer
            // SAFETY: head is always a valid pointer
            self.link3(None, link, self.head);
        }
    }

    /// Push a new element to the back of the list.
    pub fn push_back(&mut self, node: A::Ptr) {
        let link = self.link_from_ptr(node);

        unsafe {
            // SAFETY: link_from_ptr always returns a valid pointer
            // SAFETY: tail is always a valid pointer
            self.link3(self.tail, link, None);
        }
    }

    /// Pop an element from the front of the list.
    pub fn pop_front(&mut self) -> Option<A::Ptr> {
        if let Some(head) = self.head {
            let next = unsafe {
                // SAFETY: head is always a valid pointer
                Self::next(head)
            };

            unsafe {
                // SAFETY: next is always a valid pointer
                self.link2(None, next);
                // SAFETY: head belongs to the queue
                LinkedList::<A>::unlink(head);
            }

            Some(unsafe {
                // SAFETY: head is always valid
                // SAFETY: head no longer belongs to the queue
                self.link_to_ptr(head)
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
                Self::next(tail)
            };

            unsafe {
                // SAFETY: prev is always a valid pointer
                self.link2(prev, None);
                // SAFETY: tail belongs to the queue
                LinkedList::<A>::unlink(tail);
            }

            Some(unsafe {
                // SAFETY: tail is always valid
                // SAFETY: tail no longer belongs to the queue
                self.link_to_ptr(tail)
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
                    self.link_to_ref(head)
                };

                if !f(head) {
                    return None;
                }
            }

            let next = unsafe {
                // SAFETY: head is always a valid pointer
                Self::next(head)
            };

            unsafe {
                // SAFETY: next is always a valid pointer
                self.link2(None, next);
                // SAFETY: head belongs to the queue
                LinkedList::<A>::unlink(head);
            }

            Some(unsafe {
                // SAFETY: head is always valid
                // SAFETY: head no longer belongs to the queue
                self.link_to_ptr(head)
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
                    self.link_to_ref(tail)
                };

                if !f(tail) {
                    return None;
                }
            }

            let prev = unsafe {
                // SAFETY: tail is always a valid pointer
                Self::prev(tail)
            };

            unsafe {
                // SAFETY: prev is always a valid pointer
                self.link2(prev, None);
                // SAFETY: tail belongs to the queue
                LinkedList::<A>::unlink(tail);
            }

            Some(unsafe {
                // SAFETY: tail is always valid
                // SAFETY: tail no longer belongs to the queue
                self.link_to_ptr(tail)
            })
        } else {
            None
        }
    }

    /// Obtain a cursor from a raw reference.
    /// Allows to obtain a reference to a node with only a raw pointer to it.
    /// This allows to store a reference to a node without having to worry about lifetimes.
    /// # Safety
    /// - `raw` must be a valid pointer.
    /// - `raw` must be a node currently inserted in this queue.
    pub unsafe fn cursor_mut_from_raw(&mut self, raw: NonNull<A::Value>) -> CursorMut<'_, A> {
        let link = unsafe {
            // SAFETY: Caller assures that raw is a valid pointer
            self.link_from_raw(raw)
        };

        CursorMut {
            cur: Some(link),
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
pub struct CursorMut<'a, A> {
    cur: Option<NonNull<Link>>,
    list: &'a mut LinkedList<A>,
}

impl<'a, A> CursorMut<'a, A>
where
    A: Adapter,
{
    /// Obtain the currently pointed value.
    /// Returns None if this points to the fictitious node.
    pub fn current(&self) -> Option<&A::Value> {
        self.cur.map(|link| unsafe {
            // SAFETY: cur is always a valid pointer
            self.list.link_to_ref(link)
        })
    }

    /// Obtain the currently pointed value, mutably.
    /// Returns None if this points to the fictitious node.
    pub fn current_mut(&mut self) -> Option<&mut A::Value> {
        self.cur.map(|link| unsafe {
            // SAFETY: cur is always a valid pointer
            self.list.link_to_ref_mut(link)
        })
    }

    /// Moves to the next node.
    pub fn move_next(&mut self) {
        if let Some(cur) = self.cur {
            self.cur = unsafe {
                // SAFETY: cur is always a valid pointer
                LinkedList::<A>::next(cur)
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
                LinkedList::<A>::next(cur)
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
                LinkedList::<A>::next(cur)
            }
        } else {
            self.list.head
        };

        let link = self.list.link_from_ptr(ptr);

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
                LinkedList::<A>::prev(cur)
            }
        } else {
            self.list.tail
        };

        let link = self.list.link_from_ptr(ptr);

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
                (LinkedList::<A>::next(cur), LinkedList::<A>::prev(cur))
            };

            unsafe {
                // SAFETY: prev is always a valid pointer
                // SAFETY: next is always a valid pointer
                self.list.link2(prev, next);
                // SAFETY: cur belongs to the linked list
                LinkedList::<A>::unlink(cur);
            }

            self.cur = next;

            Some(unsafe {
                // SAFETY: cur is always valid
                // SAFETY: cur no longer belongs to the queue
                self.list.link_to_ptr(cur)
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
