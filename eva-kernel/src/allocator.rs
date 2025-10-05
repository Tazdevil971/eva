use linked_list_allocator::Heap;

use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::ptr::{self, NonNull};

use crate::scheduler::pause::{PauseCell, with_pause};

struct KAlloc(PauseCell<RefCell<Heap>>);

impl KAlloc {
    unsafe fn new(start: *mut u8, end: *mut u8) -> Self {
        let heap = unsafe { Heap::new(start, end as usize - start as usize) };

        Self::from_heap(heap)
    }

    const fn empty() -> Self {
        Self::from_heap(Heap::empty())
    }

    const fn from_heap(heap: Heap) -> Self {
        Self(PauseCell::new(RefCell::new(heap)))
    }

    fn with_heap<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut Heap) -> T,
    {
        with_pause(|token| f(&mut *self.0.borrow_ref_mut(token)))
    }
}

unsafe impl GlobalAlloc for KAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.with_heap(|heap| {
            heap.allocate_first_fit(layout)
                .ok()
                .map(NonNull::as_ptr)
                .unwrap_or(ptr::null_mut())
        })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.with_heap(|heap| unsafe {
            heap.deallocate(NonNull::new_unchecked(ptr), layout);
        })
    }
}

#[global_allocator]
static mut ALLOCATOR: KAlloc = KAlloc::empty();

pub unsafe fn init(start: *mut u8, end: *mut u8) {
    unsafe { ALLOCATOR = KAlloc::new(start, end) }
}
