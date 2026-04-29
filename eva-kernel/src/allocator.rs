use linked_list_allocator::Heap;

use alloc::alloc;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::ptr::{self, NonNull};

use crate::rt::pause::{PauseCell, with_pause};

struct KAlloc(PauseCell<RefCell<Heap>>);

impl KAlloc {
    unsafe fn new(start: *mut u8, end: *mut u8) -> Self {
        let heap = unsafe { Heap::new(start, end.byte_offset_from_unsigned(start)) };

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

#[unsafe(export_name = "eva_mem_alloc")]
pub unsafe fn alloc(layout: Layout) -> *mut u8 {
    unsafe { alloc::alloc(layout) }
}

#[unsafe(export_name = "eva_mem_dealloc")]
pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
    unsafe { alloc::dealloc(ptr, layout) }
}

const MALLOC_ALIGN: usize = 16;

pub(crate) unsafe fn emu_malloc(size: usize) -> *mut () {
    // Calculate size to accommodate malloc header
    let size = size + MALLOC_ALIGN;

    // Calculate chunk layout
    let Ok(layout) = Layout::from_size_align(size, MALLOC_ALIGN) else {
        return ptr::null_mut();
    };

    // Allocate the chunk, returning early in case of failed allocation
    let ptr = unsafe { alloc::alloc(layout).cast::<()>() };
    if ptr.is_null() {
        return ptr::null_mut();
    }

    // Write out the size in the header
    unsafe {
        ptr.cast::<usize>().write(size);
    }

    // Return an actual usable pointer
    unsafe { ptr.byte_add(MALLOC_ALIGN) }
}

pub(crate) unsafe fn emu_free(ptr: *mut ()) {
    // No-op in case of input null pointer
    if ptr.is_null() {
        return;
    }

    // Calculate the pointer to the root and read the size
    let ptr = unsafe { ptr.byte_sub(MALLOC_ALIGN) };
    let size = unsafe { ptr.cast::<usize>().read() };

    let layout = unsafe {
        // SAFETY: If we assume this chunk is valid, the layout must also be valid
        Layout::from_size_align_unchecked(size, MALLOC_ALIGN)
    };

    unsafe {
        alloc::dealloc(ptr.cast(), layout);
    }
}
