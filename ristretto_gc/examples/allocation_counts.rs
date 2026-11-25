//! Count successful system-allocation requests, separately from timing benchmarks.
//! Reallocation counts as one release and one allocation, including in-place growth.
//! Arguments: baseline|candidate, operation count. No collector workers run here.
#![allow(unsafe_code)]
#![expect(clippy::print_stdout, clippy::panic_in_result_fn)]
use portable_atomic::AtomicU64;
use ristretto_gc::{Configuration, GarbageCollector, Gc, RootStorage, Trace};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::Ordering;

#[path = "support/construction_batches.rs"]
mod construction_batches;

struct Counting;
static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static ALLOCATED_BYTES: AtomicU64 = AtomicU64::new(0);
static DEALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static RELEASED_BYTES: AtomicU64 = AtomicU64::new(0);

fn allocated(ptr: *mut u8, bytes: usize) {
    if !ptr.is_null() {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        ALLOCATED_BYTES.fetch_add(bytes as u64, Ordering::Relaxed);
    }
}

fn released(bytes: usize) {
    DEALLOCATIONS.fetch_add(1, Ordering::Relaxed);
    RELEASED_BYTES.fetch_add(bytes as u64, Ordering::Relaxed);
}

// SAFETY: every operation forwards the identical pointer/layout to System.
// Counters allocate no memory and do not change allocation or unwind behavior.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: forward the caller's valid layout to the system allocator.
        let ptr = unsafe { System.alloc(layout) };
        allocated(ptr, layout.size());
        ptr
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: forward the caller's valid layout, preserving zero initialization.
        let ptr = unsafe { System.alloc_zeroed(layout) };
        allocated(ptr, layout.size());
        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: the caller supplies this allocator's live pointer and layout.
        unsafe { System.dealloc(ptr, layout) };
        released(layout.size());
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        // SAFETY: preserve the caller's pointer, old layout, and requested new size.
        let result = unsafe { System.realloc(ptr, layout, size) };
        if !result.is_null() {
            released(layout.size());
            allocated(result, size);
        }
        result
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

#[derive(Clone, Copy)]
struct Snapshot {
    allocations: u64,
    bytes: u64,
    deallocations: u64,
    released: u64,
}
fn snapshot() -> Snapshot {
    Snapshot {
        allocations: ALLOCATIONS.load(Ordering::Relaxed),
        bytes: ALLOCATED_BYTES.load(Ordering::Relaxed),
        deallocations: DEALLOCATIONS.load(Ordering::Relaxed),
        released: RELEASED_BYTES.load(Ordering::Relaxed),
    }
}

struct Payload([u64; 4]);
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Payload {
    fn trace(&self, _: &GarbageCollector) {}
    fn trace_work(&self) -> usize {
        1
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let policy = args.get(1).map_or("candidate", String::as_str);
    let candidate = match policy {
        "baseline" => false,
        "candidate" => true,
        _ => return Err("expected baseline or candidate".into()),
    };
    let operations = args.get(2).map_or(Ok(10_000_u64), |s| s.parse())?;
    if !(1..=100_000).contains(&operations) {
        return Err("operations must be 1..=100000".into());
    }
    let mode = args.get(3).map_or("ephemeral", String::as_str);
    if !["ephemeral", "weak_retention", "construction_batches"].contains(&mode) {
        return Err("expected ephemeral, weak_retention or construction_batches mode".into());
    }
    let weak_capacity = if mode == "weak_retention" {
        usize::try_from(operations)?
    } else {
        0
    };
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .allocation_threshold(usize::MAX)
            .max_heap_bytes(64 * 1024 * 1024)
            .max_pause_time_us(0)
            .root_storage(if candidate {
                RootStorage::Contexts
            } else {
                RootStorage::Tree
            })
            .barrier_buffer_size(if candidate { 32 } else { 0 })
            .local_allocation_buffers(candidate)
            .concurrent_publication(candidate)
            .build(),
    );
    let context = gc.register_context();
    let (before, after, reclaimed, weak_dropped, checksum) =
        context.with(|| -> ristretto_gc::Result<_> {
            for _ in 0..1024 {
                if mode == "construction_batches" {
                    drop(Gc::try_new(
                        &gc,
                        construction_batches::ConstructionBatch::new(),
                    )?);
                } else {
                    drop(Gc::try_new(&gc, Payload([42; 4]))?);
                }
            }
            gc.collect_full()?;
            gc.collect_full()?;
            let mut weak = Vec::with_capacity(weak_capacity);
            let before = snapshot();
            let mut checksum = 0;
            for _ in 0..operations {
                if mode == "construction_batches" {
                    let root = Gc::try_new(&gc, construction_batches::ConstructionBatch::new())?;
                    checksum += std::hint::black_box(root.value());
                    continue;
                }
                let root = Gc::try_new(&gc, Payload(std::hint::black_box([42; 4])))?;
                checksum += std::hint::black_box(root.0[0]);
                if mode == "weak_retention" {
                    weak.push(ristretto_gc::GcWeak::new(
                        &root,
                        ristretto_gc::ReferenceKind::Weak,
                        0,
                    ));
                }
            }
            let after = snapshot();
            gc.collect_full()?;
            gc.collect_full()?;
            assert!(weak.iter().all(|reference| reference.upgrade().is_none()));
            let reclaimed = snapshot();
            drop(weak);
            Ok((before, after, reclaimed, snapshot(), checksum))
        })?;
    drop(context);
    gc.stop()?;
    assert_eq!(checksum, operations * 42);
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(gc.statistics()?.live_objects, 0);
    println!(
        "{{\"policy\":\"{policy}\",\"mode\":\"{mode}\",\"operations\":{operations},\"checksum\":{checksum},\"allocations\":{},\"allocated_bytes\":{},\"deallocations\":{},\"released_bytes\":{},\"collection_allocations\":{},\"collection_allocated_bytes\":{},\"collection_deallocations\":{},\"collection_released_bytes\":{},\"retained_after_collection_bytes\":{},\"released_on_weak_drop_bytes\":{},\"teardown_live\":0}}",
        after.allocations - before.allocations,
        after.bytes - before.bytes,
        after.deallocations - before.deallocations,
        after.released - before.released,
        reclaimed.allocations - after.allocations,
        reclaimed.bytes - after.bytes,
        reclaimed.deallocations - after.deallocations,
        reclaimed.released - after.released,
        reclaimed.bytes - reclaimed.released,
        weak_dropped.released - reclaimed.released,
    );
    Ok(())
}
