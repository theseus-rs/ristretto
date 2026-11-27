use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};
use ristretto_gc::{Configuration, GarbageCollector, Gc};

struct Counter;
static ENABLED: AtomicBool = AtomicBool::new(false);
static CALLS: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);
fn observe(bytes: usize) {
    if ENABLED.load(Relaxed) {
        CALLS.fetch_add(1, Relaxed);
        BYTES.fetch_add(bytes, Relaxed);
    }
}

// Each operation forwards the original pointer and layout to the same allocator.
// The counters neither allocate nor inspect or retain the returned storage.
unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        observe(layout.size());
        unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        observe(layout.size());
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        observe(size);
        unsafe { System.realloc(ptr, layout, size) }
    }
}

#[global_allocator]
static ALLOCATOR: Counter = Counter;
fn main() -> ristretto_gc::Result<()> {
    let gc = GarbageCollector::with_config(Configuration::builder()
        .threads(0).predictive_collection(true).max_heap_bytes(64 << 20).allocation_threshold(usize::MAX).incremental_step_size(32)
        .max_pause_time_us(0).independent_marking(true).independent_phases(true)
        .diagnostics(true).build());
    let mut owners = Vec::new();
    for i in 0..256_u64 { owners.push(Gc::try_new(&gc, i)?); }
    for _ in 0..8 { gc.collect_full()?; }
    let before = gc.statistics()?;
    ENABLED.store(true, Relaxed);
    for _ in 0..64 { gc.collect_full()?; }
    ENABLED.store(false, Relaxed);
    let after = gc.statistics()?;
    assert_eq!(after.full_collections - before.full_collections, 64);
    assert_eq!(after.live_objects, 256);
    assert_eq!(owners.iter().map(|x| ***x).sum::<u64>(), (0..256_u64).sum());
    println!("{} {} {}", CALLS.load(Relaxed), BYTES.load(Relaxed),
             after.collection_steps - before.collection_steps);
    Ok(())
}
