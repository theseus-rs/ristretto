//! Count the successful and rejected raw WASI projections in a single-threaded process.
use ristretto_gc::sync::{RecursiveRwLock, RecursiveRwLockReadGuard, RecursiveRwLockWriteGuard};
use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicUsize, Ordering};

#[global_allocator]
static ALLOCATOR: Counter = Counter;
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

struct Counter;

// SAFETY: both methods forward the caller's layout and pointer unchanged to System.
unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
    }
}

fn main() {
    let lock = RecursiveRwLock::new((1_u64, 2_u64));
    let repetitions = 4096;
    let start = ALLOCATIONS.load(Ordering::Relaxed);
    for _ in 0..repetitions {
        let first = RecursiveRwLockReadGuard::map(lock.read(), |pair| &pair.0);
        black_box(&first);
        drop(first);
        let second = RecursiveRwLockReadGuard::try_map(lock.read(), |pair| Some(&pair.1)).unwrap();
        black_box(&second);
        drop(second);
        let mut first = RecursiveRwLockWriteGuard::map(lock.write(), |pair| &mut pair.0);
        **black_box(&mut first) += 1;
        drop(first);
        let mut second =
            RecursiveRwLockWriteGuard::try_map(lock.write(), |pair| Some(&mut pair.1)).unwrap();
        **black_box(&mut second) += 1;
        drop(second);
    }
    let accepted = ALLOCATIONS.load(Ordering::Relaxed) - start;
    let start = ALLOCATIONS.load(Ordering::Relaxed);
    for _ in 0..repetitions {
        let read = RecursiveRwLockReadGuard::try_map(lock.read(), |_| None::<&u64>).unwrap_err();
        black_box(&read);
        drop(read);
        let write =
            RecursiveRwLockWriteGuard::try_map(lock.write(), |_| None::<&mut u64>).unwrap_err();
        black_box(&write);
        drop(write);
    }
    let rejected = ALLOCATIONS.load(Ordering::Relaxed) - start;
    assert_eq!(*lock.read(), (1 + repetitions, 2 + repetitions));
    println!(
        "accepted_operations={},accepted_allocations={accepted},rejected_operations={},rejected_allocations={rejected}",
        repetitions * 4,
        repetitions * 2
    );
}
