//! Registry, binding, argument, and future-allocation benchmarks.
#![expect(
    clippy::expect_used,
    clippy::panic,
    reason = "benchmarks must not time failed operations"
)]

use criterion::{Criterion, criterion_group, criterion_main};
use ristretto_classfile::JAVA_25;
use ristretto_classloader::Value;
use ristretto_types::IntrinsicRegistry;
use ristretto_vm::{IntrinsicMethod, MethodRegistry, Parameters, Thread};
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::hint::black_box;
use std::sync::Weak;
use std::time::Duration;

thread_local! {
    static ALLOCATIONS: Cell<(bool, usize, usize)> = const { Cell::new((false, 0, 0)) };
}

#[derive(Debug)]
struct CountingAllocator;

fn allocated(bytes: usize) {
    let _ = ALLOCATIONS.try_with(|counter| {
        let (enabled, count, total) = counter.get();
        if enabled {
            counter.set((true, count + 1, total + bytes));
        }
    });
}

// SAFETY: every allocation and deallocation is forwarded unchanged to System.
#[expect(
    unsafe_code,
    reason = "benchmark-only transparent global allocator instrumentation"
)]
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        allocated(layout.size());
        unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        allocated(layout.size());
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) }
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        allocated(size);
        unsafe { System.realloc(pointer, layout, size) }
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

fn allocations(name: &str, operation: impl FnOnce()) {
    ALLOCATIONS.with(|counter| counter.set((true, 0, 0)));
    operation();
    let (_, count, bytes) = ALLOCATIONS.with(|counter| counter.replace((false, 0, 0)));
    eprintln!("{name}: {count} allocations, {bytes} allocated bytes (calling thread)");
}

#[expect(
    clippy::too_many_lines,
    reason = "related intrinsic benchmark cases share setup"
)]
fn benchmarks(criterion: &mut Criterion) {
    let registry = MethodRegistry::new(&JAVA_25).expect("registry");
    let thread = Thread::new(&Weak::new(), 1);
    let Some(IntrinsicMethod::Sync(sync)) = registry
        .method("java/lang/Float", "floatToRawIntBits", "(F)I")
        .copied()
    else {
        panic!("expected intrinsic calling convention");
    };
    let Some(IntrinsicMethod::Async(yielding)) = registry
        .method("java/lang/Thread", "yield0", "()V")
        .copied()
    else {
        panic!("expected intrinsic calling convention");
    };
    let Some(IntrinsicMethod::Async(sleep)) = registry
        .method("java/lang/Thread", "sleepNanos0", "(J)V")
        .copied()
    else {
        panic!("expected intrinsic calling convention");
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");
    let mut raw = IntrinsicRegistry::<Thread>::new(&JAVA_25);
    ristretto_intrinsics::register(&mut raw).expect("registry");
    let mut sorted: Vec<_> = raw
        .methods()
        .iter()
        .map(|(name, implementation)| (*name, *implementation))
        .collect();
    sorted.sort_unstable_by_key(|(name, _)| *name);

    allocations("registry/cold", || {
        let mut registry = IntrinsicRegistry::<Thread>::new(&JAVA_25);
        ristretto_intrinsics::register(&mut registry).expect("registry");
        black_box(registry);
    });
    allocations("arguments/inline", || {
        black_box([Value::Float(1.0)].into_iter().collect::<Parameters>());
    });
    allocations("arguments/owned_vec", || {
        black_box(Parameters::new(black_box(vec![Value::Float(1.0)])));
    });
    allocations("dispatch/sync_bound", || {
        black_box(sync(thread.clone(), [Value::Float(1.0)].into_iter().collect()).expect("sync"));
    });
    allocations("dispatch/async_adapter", || {
        drop(black_box(yielding(thread.clone(), Parameters::default())));
    });
    // Warm the timer driver before accounting for a genuinely suspending invocation.
    runtime
        .block_on(sleep(
            thread.clone(),
            [Value::Long(1)].into_iter().collect(),
        ))
        .expect("sleep");
    allocations("dispatch/async_suspending", || {
        runtime
            .block_on(sleep(
                thread.clone(),
                [Value::Long(1)].into_iter().collect(),
            ))
            .expect("sleep");
    });

    criterion.bench_function("intrinsics/registry_cold", |b| {
        b.iter(|| {
            let mut registry = IntrinsicRegistry::<Thread>::new(&JAVA_25);
            ristretto_intrinsics::register(&mut registry).expect("registry");
            black_box(registry)
        });
    });
    criterion.bench_function("intrinsics/lookup_signature", |b| {
        b.iter(|| {
            black_box(registry.method(black_box("java/lang/Float"), "floatToRawIntBits", "(F)I"))
        });
    });
    criterion.bench_function("intrinsics/lookup_sorted", |b| {
        b.iter(|| {
            black_box(sorted.binary_search_by_key(
                &black_box("java/lang/Float.floatToRawIntBits(F)I"),
                |(name, _)| *name,
            ))
        });
    });
    criterion.bench_function("intrinsics/sync_bound_inline", |b| {
        b.iter(|| {
            black_box(
                sync(
                    thread.clone(),
                    [Value::Float(black_box(1.0))].into_iter().collect(),
                )
                .expect("sync"),
            )
        });
    });
    criterion.bench_function("intrinsics/sync_lookup_owned", |b| {
        b.iter(|| {
            let Some(IntrinsicMethod::Sync(function)) =
                registry.method(black_box("java/lang/Float"), "floatToRawIntBits", "(F)I")
            else {
                panic!("expected synchronous intrinsic");
            };
            function(
                thread.clone(),
                Parameters::new(black_box(vec![Value::Float(1.0)])),
            )
            .expect("sync")
        });
    });
    criterion.bench_function("intrinsics/async_yield_adapter", |b| {
        b.iter(|| {
            runtime
                .block_on(yielding(thread.clone(), Parameters::default()))
                .expect("yield")
        });
    });
    criterion.bench_function("intrinsics/async_suspending", |b| {
        b.iter(|| {
            runtime
                .block_on(sleep(
                    thread.clone(),
                    [Value::Long(1)].into_iter().collect(),
                ))
                .expect("sleep")
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(20).warm_up_time(Duration::from_millis(200)).measurement_time(Duration::from_secs(1));
    targets = benchmarks
}
criterion_main!(benches);
