#![no_std]
use critical_section::Mutex;
use linked_list_allocator::Heap;

use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::ptr::{self, NonNull};

struct Allocator(Mutex<RefCell<Heap>>);

impl Allocator {
    unsafe fn new(start: *mut u8, end: *mut u8) -> Self {
        unsafe { Self::from_heap(Heap::new(start, end as usize - start as usize)) }
    }

    const fn empty() -> Self {
        Self::from_heap(Heap::empty())
    }

    const fn from_heap(heap: Heap) -> Self {
        Self(Mutex::new(RefCell::new(heap)))
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        critical_section::with(|cs| {
            self.0
                .borrow_ref_mut(cs)
                .allocate_first_fit(layout)
                .ok()
                .map_or(ptr::null_mut(), NonNull::as_ptr)
        })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        critical_section::with(|cs| unsafe {
            self.0
                .borrow_ref_mut(cs)
                .deallocate(NonNull::new_unchecked(ptr), layout);
        })
    }
}

#[global_allocator]
static mut ALLOCATOR: Allocator = Allocator::empty();

pub unsafe fn init(start: *mut u8, end: *mut u8) {
    unsafe { ALLOCATOR = Allocator::new(start, end) }
}
