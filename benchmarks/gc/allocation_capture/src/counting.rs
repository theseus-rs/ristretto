//! Process Rust global-allocator accounting, never a timer or payload-only counter.
#![allow(unsafe_code)]
use portable_atomic::AtomicU64;
use serde::Serialize;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

struct Counting;
static ENABLED: AtomicBool = AtomicBool::new(true);
static VALID: AtomicBool = AtomicBool::new(true);
static REQUESTS: AtomicU64 = AtomicU64::new(0);
static REQUESTED: AtomicU64 = AtomicU64::new(0);
static SUCCESSES: AtomicU64 = AtomicU64::new(0);
static ALLOCATED: AtomicU64 = AtomicU64::new(0);
static RELEASES: AtomicU64 = AtomicU64::new(0);
static RELEASED: AtomicU64 = AtomicU64::new(0);
static LIVE: AtomicU64 = AtomicU64::new(0);
static PEAK: AtomicU64 = AtomicU64::new(0);

fn add(counter: &AtomicU64, value: u64) -> u64 {
    let old = counter.fetch_add(value, Relaxed);
    let (next, overflow) = old.overflowing_add(value);
    if overflow {
        VALID.store(false, Relaxed);
    }
    next
}

fn requested(size: usize) {
    add(&REQUESTS, 1);
    add(&REQUESTED, size as u64);
}

fn allocated(pointer: *mut u8, size: usize) {
    if !pointer.is_null() {
        add(&SUCCESSES, 1);
        add(&ALLOCATED, size as u64);
        PEAK.fetch_max(add(&LIVE, size as u64), Relaxed);
    }
}

fn released(size: usize) {
    add(&RELEASES, 1);
    add(&RELEASED, size as u64);
    if LIVE.fetch_sub(size as u64, Relaxed) < size as u64 {
        VALID.store(false, Relaxed);
    }
}

// SAFETY: every layout, pointer and result is forwarded unchanged to System.
// Counter operations allocate nothing and never panic, including on overflow.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let enabled = ENABLED.load(Relaxed);
        if enabled {
            requested(layout.size());
        }
        // SAFETY: the caller supplies a valid layout.
        let result = unsafe { System.alloc(layout) };
        if enabled {
            allocated(result, layout.size());
        }
        result
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let enabled = ENABLED.load(Relaxed);
        if enabled {
            requested(layout.size());
        }
        // SAFETY: the valid layout and zero-initialization contract are preserved.
        let result = unsafe { System.alloc_zeroed(layout) };
        if enabled {
            allocated(result, layout.size());
        }
        result
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: the caller supplies this allocator's live pointer and layout.
        unsafe { System.dealloc(pointer, layout) };
        if ENABLED.load(Relaxed) {
            released(layout.size());
        }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let enabled = ENABLED.load(Relaxed);
        if enabled {
            requested(size);
        }
        // SAFETY: preserve the caller's valid old allocation and requested new size.
        let result = unsafe { System.realloc(pointer, layout, size) };
        if enabled && !result.is_null() {
            // A successful realloc replaces the old logical requested-byte charge,
            // even in place. Hidden allocator overlap is not observable here.
            released(layout.size());
            allocated(result, size);
        }
        result
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// Irreversible check-only mode: no samples are read and accounting never resumes.
pub fn disable() {
    ENABLED.store(false, Relaxed);
}

#[derive(Clone, Copy)]
pub struct Snapshot {
    requests: u64,
    requested: u64,
    successes: u64,
    allocated: u64,
    releases: u64,
    released: u64,
    live: u64,
}

fn snapshot() -> Snapshot {
    Snapshot {
        requests: REQUESTS.load(Relaxed),
        requested: REQUESTED.load(Relaxed),
        successes: SUCCESSES.load(Relaxed),
        allocated: ALLOCATED.load(Relaxed),
        releases: RELEASES.load(Relaxed),
        released: RELEASED.load(Relaxed),
        live: LIVE.load(Relaxed),
    }
}

pub fn begin() -> Snapshot {
    let before = snapshot();
    PEAK.store(before.live, Relaxed);
    before
}

#[derive(Debug, Serialize)]
pub struct Counts {
    pub requests: u64,
    pub requested_bytes: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub allocated_bytes: u64,
    pub releases: u64,
    pub released_bytes: u64,
    pub live_bytes_before: u64,
    pub live_bytes_after: u64,
    pub peak_live_bytes: u64,
    pub peak_above_baseline: u64,
}

pub fn end(before: Snapshot) -> Result<Counts, &'static str> {
    let after = snapshot();
    let peak = PEAK.load(Relaxed);
    let sub = |a: u64, b: u64| a.checked_sub(b).ok_or("allocation counter wrapped");
    if !VALID.load(Relaxed) || !ENABLED.load(Relaxed) {
        return Err("allocation counters invalid or disabled");
    }
    let requests = sub(after.requests, before.requests)?;
    let successes = sub(after.successes, before.successes)?;
    let allocated_bytes = sub(after.allocated, before.allocated)?;
    let released_bytes = sub(after.released, before.released)?;
    if u128::from(before.live) + u128::from(allocated_bytes)
        != u128::from(after.live) + u128::from(released_bytes)
        || peak < after.live
    {
        return Err("allocation snapshot is not internally balanced");
    }
    Ok(Counts {
        requests,
        requested_bytes: sub(after.requested, before.requested)?,
        successful_requests: successes,
        failed_requests: sub(requests, successes)?,
        allocated_bytes,
        releases: sub(after.releases, before.releases)?,
        released_bytes,
        live_bytes_before: before.live,
        live_bytes_after: after.live,
        peak_live_bytes: peak,
        peak_above_baseline: sub(peak, before.live)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_alloc_zero_realloc_and_release_accounting() -> Result<(), &'static str> {
        // Run this sole allocator test with one test thread. Explicit allocator
        // calls cannot be removed as unused Box/Vec allocations by optimization.
        let layout = Layout::from_size_align(32, 16).map_err(|_| "layout")?;
        let grown = Layout::from_size_align(96, 16).map_err(|_| "layout")?;
        let before = begin();
        // SAFETY: layouts are valid; successful pointers are consumed exactly once.
        unsafe {
            let first = ALLOCATOR.alloc_zeroed(layout);
            if first.is_null() {
                return Err("allocation failed");
            }
            for index in 0..32 {
                if *first.add(index) != 0 {
                    ALLOCATOR.dealloc(first, layout);
                    return Err("zeroing lost");
                }
            }
            let second = ALLOCATOR.realloc(first, layout, 96);
            if second.is_null() {
                ALLOCATOR.dealloc(first, layout);
                return Err("reallocation failed");
            }
            ALLOCATOR.dealloc(second, grown);
        }
        let counts = end(before)?;
        assert_eq!(counts.requests, 2);
        assert_eq!(counts.requested_bytes, 128);
        assert_eq!(counts.successful_requests, 2);
        assert_eq!(counts.failed_requests, 0);
        assert_eq!(counts.allocated_bytes, 128);
        assert_eq!(counts.releases, 2);
        assert_eq!(counts.released_bytes, 128);
        assert_eq!(counts.live_bytes_after, counts.live_bytes_before);
        assert_eq!(counts.peak_above_baseline, 96);
        Ok(())
    }
}
