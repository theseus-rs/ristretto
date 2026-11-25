//! Count root-owner preparation and aliasing separately from the timing example.
//! Requested allocator bytes are neither RSS nor the allocator's usable size.
#![allow(unsafe_code)]
#![expect(clippy::print_stdout, reason = "machine-readable allocation counts")]
use portable_atomic::AtomicU64;
use ristretto_gc::{Error, Result};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::Ordering::Relaxed;

struct Counting;
static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static BYTES: AtomicU64 = AtomicU64::new(0);
static RELEASES: AtomicU64 = AtomicU64::new(0);
static RELEASED_BYTES: AtomicU64 = AtomicU64::new(0);
static LIVE: AtomicU64 = AtomicU64::new(0);
static PEAK: AtomicU64 = AtomicU64::new(0);

fn allocated(pointer: *mut u8, size: usize) {
    if !pointer.is_null() {
        let bytes = size as u64;
        ALLOCATIONS.fetch_add(1, Relaxed);
        BYTES.fetch_add(bytes, Relaxed);
        PEAK.fetch_max(LIVE.fetch_add(bytes, Relaxed) + bytes, Relaxed);
    }
}
fn released(size: usize) {
    RELEASES.fetch_add(1, Relaxed);
    RELEASED_BYTES.fetch_add(size as u64, Relaxed);
    LIVE.fetch_sub(size as u64, Relaxed);
}

// SAFETY: forward every pointer and layout unchanged to System; the counters
// allocate no storage and do not affect ownership or allocator failure behavior.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: caller supplies a valid layout.
        let pointer = unsafe { System.alloc(layout) };
        allocated(pointer, layout.size());
        pointer
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: preserve the valid layout and zero initialization.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        allocated(pointer, layout.size());
        pointer
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: caller supplies this allocator's live pointer and layout.
        unsafe { System.dealloc(pointer, layout) };
        released(layout.size());
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        // SAFETY: preserve the caller's valid old allocation and new size.
        let result = unsafe { System.realloc(pointer, layout, size) };
        if !result.is_null() {
            // Successful realloc is one release and allocation, even in place.
            released(layout.size());
            allocated(result, size);
        }
        result
    }
}
#[global_allocator]
static ALLOCATOR: Counting = Counting;

#[derive(Clone, Copy, Default)]
struct Snapshot {
    allocations: u64,
    bytes: u64,
    releases: u64,
    released_bytes: u64,
    live: u64,
    peak: u64,
}
fn snapshot() -> Snapshot {
    Snapshot {
        allocations: ALLOCATIONS.load(Relaxed),
        bytes: BYTES.load(Relaxed),
        releases: RELEASES.load(Relaxed),
        released_bytes: RELEASED_BYTES.load(Relaxed),
        live: LIVE.load(Relaxed),
        peak: PEAK.load(Relaxed),
    }
}
fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let [mode, copies] = args.as_slice() else {
        return Err(Error::RootError(
            "usage: root_aliases_counts MODE COPIES".into(),
        ));
    };
    let copies = copies
        .parse()
        .map_err(|_| Error::RootError("invalid copies".into()))?;
    drop(ristretto_gc::experimental_root_aliases(
        mode,
        copies,
        |_| {},
    )?);
    let mut checkpoints = [("", Snapshot::default()); 5];
    let mut index = 0;
    let report = ristretto_gc::experimental_root_aliases(mode, copies, |name| {
        if let Some(slot) = checkpoints.get_mut(index) {
            *slot = (name, snapshot());
        }
        index += 1;
    })?;
    if index != checkpoints.len() {
        return Err(Error::RootError(
            "root alias checkpoint count changed".into(),
        ));
    }
    for (name, s) in checkpoints {
        println!(
            "{{\"checkpoint\":\"{name}\",\"allocations\":{},\"bytes\":{},\"releases\":{},\"released_bytes\":{},\"live\":{},\"peak\":{}}}",
            s.allocations, s.bytes, s.releases, s.released_bytes, s.live, s.peak
        );
    }
    println!("{report}");
    Ok(())
}
