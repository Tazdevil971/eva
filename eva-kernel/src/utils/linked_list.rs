use core::cell::Cell;
use core::marker::{PhantomData, PhantomPinned};
use core::pin::Pin;
use core::ptr::NonNull;

use crate::utils::assert::harden_assert;

#[derive(Debug, Clone)]
pub struct Links {
    next: Cell<Option<NonNull<Links>>>,
    prev: Cell<Option<NonNull<Links>>>,
    _pin: PhantomPinned,
}

unsafe impl Send for Links {}

impl Links {
    pub const fn unlinked() -> Self {
        Self {
            next: Cell::new(None),
            prev: Cell::new(None),
            _pin: PhantomPinned,
        }
    }

    unsafe fn next(ptr: NonNull<Self>) -> Option<NonNull<Self>> {
        unsafe {
            ptr.as_ref().next.get()
        }
    }

    unsafe fn prev(ptr: NonNull<Self>) -> Option<NonNull<Self>> {
        unsafe {
            ptr.as_ref().prev.get()
        }
    }
}

#[derive(Debug)]
struct RawLinkedList {
    head: Cell<Option<NonNull<Links>>>,
    tail: Cell<Option<NonNull<Links>>>,
}

unsafe impl Send for RawLinkedList {}

impl RawLinkedList {
    const fn empty() -> Self {
        Self {
            head: Cell::new(None),
            tail: Cell::new(None),
        }
    }

    fn is_empty(&self) -> bool {
        harden_assert!(self.head.get().is_none() == self.tail.get().is_none());
        self.head.get().is_none()
    }

    unsafe fn link2(&self, prev: Option<NonNull<Links>>, next: Option<NonNull<Links>>) {
        if let Some(prev) = prev {
            unsafe {
                // SAFETY:
                prev.as_ref().next.set(next);
            }
        } else {
            self.head.set(next);
        }

        if let Some(next) = next {
            unsafe {
                // SAFETY:
                next.as_ref().prev.set(prev);
            }
        } else {
            self.tail.set(prev);
        }
    }

    unsafe fn link3(
        &self,
        prev: Option<NonNull<Links>>,
        middle: NonNull<Links>,
        next: Option<NonNull<Links>>,
    ) {
        unsafe {
            // SAFETY:
            self.link2(prev, Some(middle));
            self.link2(Some(middle), next);
        }
    }

    fn iter(&self) -> impl Iterator<Item = NonNull<Links>> {
        let mut iter = self.head.get();

        core::iter::from_fn(move || {
            let node = iter?;
            iter = unsafe {
                // SAFETY:
                Links::next(node)
            };

            Some(node)
        })
    }

    unsafe fn insert_after(&self, prev: NonNull<Links>, node: NonNull<Links>) {
        let next = unsafe {
            // SAFETY:
            Links::next(prev)
        };

        unsafe {
            // SAFETY:
            self.link3(Some(prev), node, next);
        }
    }

    unsafe fn insert_before(&self, next: NonNull<Links>, node: NonNull<Links>) {
        let prev = unsafe {
            // SAFETY:
            Links::prev(next)
        };

        unsafe {
            // SAFETY:
            self.link3(prev, node, Some(next));
        }
    }

    unsafe fn push_front(&self, node: NonNull<Links>) {
        let head = self.head.get();
        unsafe {
            // SAFETY:
            self.link3(None, node, head);
        }
    }

    unsafe fn push_back(&self, node: NonNull<Links>) {
        let tail = self.tail.get();
        unsafe {
            // SAFETY:
            self.link3(tail, node, None);
        }
    }

    unsafe fn remove(&self, node: NonNull<Links>) {
        let (prev, next) = unsafe {
            // SAFETY:
            (Links::prev(node), Links::next(node))
        };

        unsafe {
            // SAFETY:
            self.link2(prev, next);
        }
    }

    fn pop_front(&self) -> Option<NonNull<Links>> {
        if let Some(head) = self.head.get() {
            let next = unsafe {
                // SAFETY:
                Links::next(head)
            };

            unsafe {
                // SAFETY:
                self.link2(None, next);
            }

            Some(head)
        } else {
            None
        }
    }

    fn pop_back(&self) -> Option<NonNull<Links>> {
        if let Some(tail) = self.tail.get() {
            let prev = unsafe { Links::prev(tail) };

            unsafe {
                // SAFETY:
                self.link2(prev, None);
            }

            Some(tail)
        } else {
            None
        }
    }
}

pub unsafe trait LinkedListAdapter {
    type Node;

    fn offset_to_links() -> usize;
}

struct TypedLinkedList<T: LinkedListAdapter> {
    inner: RawLinkedList,
    // TODO: Check variance over T
    _phantom: PhantomData<*const T::Node>,
}

unsafe impl<T: LinkedListAdapter> Send for TypedLinkedList<T> where T::Node: Send {}

impl<T: LinkedListAdapter> TypedLinkedList<T> {
    #[inline(always)]
    const fn empty() -> Self {
        Self {
            inner: RawLinkedList::empty(),
            _phantom: PhantomData,
        }
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline(always)]
    fn links_to_node(links: NonNull<Links>) -> NonNull<T::Node> {
        unsafe {
            // SAFETY:
            links.cast::<T::Node>().byte_sub(T::offset_to_links())
        }
    }

    #[inline(always)]
    fn links_from_node(node: NonNull<T::Node>) -> NonNull<Links> {
        unsafe {
            // SAFETY:
            node.byte_add(T::offset_to_links()).cast::<Links>()
        }
    }

    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = NonNull<T::Node>> {
        self.inner.iter().map(Self::links_to_node)
    }

    #[inline(always)]
    unsafe fn insert_after(&self, prev: NonNull<T::Node>, node: NonNull<T::Node>) {
        unsafe {
            self.inner
                .insert_after(Self::links_from_node(prev), Self::links_from_node(node));
        }
    }

    #[inline(always)]
    unsafe fn insert_before(&self, next: NonNull<T::Node>, node: NonNull<T::Node>) {
        unsafe {
            self.inner
                .insert_before(Self::links_from_node(next), Self::links_from_node(node));
        }
    }

    #[inline(always)]
    unsafe fn push_front(&self, node: NonNull<T::Node>) {
        unsafe {
            self.inner.push_front(Self::links_from_node(node));
        }
    }

    #[inline(always)]
    unsafe fn push_back(&self, node: NonNull<T::Node>) {
        unsafe {
            self.inner.push_back(Self::links_from_node(node));
        }
    }

    #[inline(always)]
    unsafe fn remove(&self, node: NonNull<T::Node>) {
        unsafe {
            self.inner.remove(Self::links_from_node(node));
        }
    }

    #[inline(always)]
    fn pop_front(&self) -> Option<NonNull<T::Node>> {
        self.inner.pop_front().map(Self::links_to_node)
    }

    #[inline(always)]
    fn pop_back(&self) -> Option<NonNull<T::Node>> {
        self.inner.pop_back().map(Self::links_to_node)
    }
}

pub struct StackLinkedList<T: LinkedListAdapter>(TypedLinkedList<T>);

impl<T: LinkedListAdapter> StackLinkedList<T> {
    pub const fn empty() -> Self {
        Self(TypedLinkedList::empty())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub unsafe fn iter<'a>(&'a self) -> impl Iterator<Item = Pin<&'a T::Node>> {
        self.0
            .iter()
            .map(|ptr| unsafe { Pin::new_unchecked(ptr.as_ref()) })
    }

    pub unsafe fn insert_after(&self, prev: Pin<&T::Node>, node: Pin<&T::Node>) {
        unsafe {
            self.0
                .insert_after(NonNull::from(prev.get_ref()), NonNull::from(node.get_ref()));
        }
    }

    pub unsafe fn insert_before(&self, next: Pin<&T::Node>, node: Pin<&T::Node>) {
        unsafe {
            self.0
                .insert_before(NonNull::from(next.get_ref()), NonNull::from(node.get_ref()));
        }
    }

    pub unsafe fn push_front(&self, node: Pin<&T::Node>) {
        unsafe {
            self.0.push_front(NonNull::from(node.get_ref()));
        }
    }

    pub unsafe fn push_back(&self, node: Pin<&T::Node>) {
        unsafe {
            self.0.push_back(NonNull::from(node.get_ref()));
        }
    }

    pub unsafe fn remove(&self, node: Pin<&T::Node>) {
        unsafe { self.0.remove(NonNull::from(node.get_ref())) }
    }

    pub unsafe fn pop_front<'a>(&'a self) -> Option<Pin<&'a T::Node>> {
        self.0
            .pop_front()
            .map(|ptr| unsafe { Pin::new_unchecked(ptr.as_ref()) })
    }

    pub unsafe fn pop_back<'a>(&'a self) -> Option<Pin<&'a T::Node>> {
        self.0
            .pop_back()
            .map(|ptr| unsafe { Pin::new_unchecked(ptr.as_ref()) })
    }
}

pub trait HeapLinkedListAdapter: LinkedListAdapter {
    type Handle;

    unsafe fn handle_from_ptr(ptr: NonNull<Self::Node>) -> Self::Handle;
    fn handle_to_ptr(handle: Self::Handle) -> NonNull<Self::Node>;
}

pub struct HeapLinkedList<T: HeapLinkedListAdapter>(TypedLinkedList<T>);

impl<T: HeapLinkedListAdapter> HeapLinkedList<T> {
    pub const fn empty() -> Self {
        Self(TypedLinkedList::empty())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub unsafe fn push_front(&self, node: T::Handle) {
        unsafe {
            self.0.push_front(T::handle_to_ptr(node));
        }
    }

    pub unsafe fn push_back(&self, node: T::Handle) {
        unsafe {
            self.0.push_back(T::handle_to_ptr(node));
        }
    }

    pub fn pop_front(&self) -> Option<T::Handle> {
        self.0.pop_front().map(|ptr| unsafe {
            T::handle_from_ptr(ptr)
        })
    }

    pub fn pop_back(&self) -> Option<T::Handle> {
        self.0.pop_back().map(|ptr| unsafe {
            T::handle_from_ptr(ptr)
        })
    }

    pub unsafe fn remove(&self, node: T::Handle) {
        unsafe {
            self.0.remove(T::handle_to_ptr(node));
        }
    }
}
