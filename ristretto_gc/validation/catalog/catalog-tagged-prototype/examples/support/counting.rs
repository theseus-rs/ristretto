use ristretto_catalog_tagged_prototype::audit;
use std::alloc::{GlobalAlloc, Layout, System};
struct Counting;
// SAFETY: all allocator arguments and results are forwarded unchanged. Audit
// counters perform only atomic operations and cannot allocate or unwind.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let value = unsafe { System.alloc(layout) };
        if !value.is_null() {
            audit::allocated(layout.size());
        }
        value
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let value = unsafe { System.alloc_zeroed(layout) };
        if !value.is_null() {
            audit::allocated(layout.size());
        }
        value
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) };
        audit::freed(layout.size());
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let value = unsafe { System.realloc(pointer, layout, size) };
        if !value.is_null() {
            audit::freed(layout.size());
            audit::allocated(size);
        }
        value
    }
}
#[global_allocator]
static ALLOCATOR: Counting = Counting;
