use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use parking_lot::RwLock;
use ristretto_gc::{ConfigurationBuilder, Finalize, GarbageCollector, Gc, Trace};
use std::sync::Arc;
use std::time::Duration;

/// Small fixed-size data (4 bytes).
struct SmallData(i32);

impl Trace for SmallData {
    fn trace(&self, _collector: &GarbageCollector) {}
}

/// Large heap-allocated data (configurable size).
struct LargeData(Vec<u8>);

impl Trace for LargeData {
    fn trace(&self, _collector: &GarbageCollector) {}
}

/// Data with a finalizer.
struct FinalizableData(i32);

impl Trace for FinalizableData {
    fn trace(&self, _collector: &GarbageCollector) {}
}

impl Finalize for FinalizableData {
    fn finalize(&self) {
        // intentionally empty;measures overhead of the finalizer path
    }
}

/// Node in a linked list for tracing benchmarks.
struct LinkedNode {
    next: Option<Gc<RwLock<LinkedNode>>>,
}

impl Trace for LinkedNode {
    fn trace(&self, collector: &GarbageCollector) {
        if let Some(ref next) = self.next {
            next.trace(collector);
        }
    }
}

/// Node in a wide tree for tracing benchmarks.
struct WideNode {
    children: Vec<Gc<RwLock<WideNode>>>,
}

impl Trace for WideNode {
    fn trace(&self, collector: &GarbageCollector) {
        for child in &self.children {
            child.trace(collector);
        }
    }
}

/// Creates a single-threaded GC for deterministic benchmarks.
fn bench_collector() -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        ConfigurationBuilder::new()
            .threads(1)
            .allocation_threshold(64 * 1024 * 1024) // 64 MB;avoid auto-collection
            .build(),
    )
}

/// Creates a GC with a low allocation threshold to force collection.
fn collection_collector() -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        ConfigurationBuilder::new()
            .threads(1)
            .allocation_threshold(1024) // 1 KB; triggers collection quickly
            .build(),
    )
}

/// Triggers a collection and spins until the GC reports it has completed at least
/// one more cycle.  Avoids fixed `thread::sleep` calls that would dominate
/// benchmark measurements.
fn collect_and_wait(collector: &GarbageCollector) {
    let prior = collector
        .statistics()
        .map_or(0, |s| s.collections_completed);
    collector.collect();
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while std::time::Instant::now() < deadline {
        if let Ok(stats) = collector.statistics()
            && stats.collections_completed > prior
        {
            return;
        }
        std::thread::yield_now();
    }
}

fn allocation_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation");

    group.bench_function("small_object", |b| {
        let collector = bench_collector();
        b.iter(|| {
            let _guard = Gc::with_collector(&collector, SmallData(42));
        });
    });

    group.bench_function("large_object_1mb", |b| {
        let collector = bench_collector();
        b.iter(|| {
            let data = vec![0u8; 1024 * 1024];
            let _guard = Gc::with_collector(&collector, LargeData(data));
        });
    });

    for count in [10, 100, 1000] {
        group.bench_with_input(
            BenchmarkId::new("mixed_batch", count),
            &count,
            |b, &count| {
                let collector = bench_collector();
                b.iter(|| {
                    for i in 0..count {
                        if i % 2 == 0 {
                            let _guard = Gc::with_collector(&collector, SmallData(i));
                        } else {
                            let data = vec![0u8; 1024];
                            let _guard = Gc::with_collector(&collector, LargeData(data));
                        }
                    }
                });
            },
        );
    }

    group.bench_function("with_finalizer", |b| {
        let collector = bench_collector();
        b.iter(|| {
            let _guard = Gc::with_collector_and_finalizer(&collector, FinalizableData(1));
        });
    });

    group.finish();
}

fn root_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("roots");

    group.bench_function("add_remove_root", |b| {
        let collector = bench_collector();
        b.iter(|| {
            let guard = Gc::with_collector(&collector, SmallData(1));
            drop(guard); // removes root
        });
    });

    group.bench_function("root_guard_clone", |b| {
        let collector = bench_collector();
        let guard = Gc::with_collector(&collector, SmallData(1));
        b.iter(|| {
            let _cloned = guard.clone();
        });
    });

    for count in [10, 100, 1000] {
        group.bench_with_input(
            BenchmarkId::new("many_roots", count),
            &count,
            |b, &count| {
                let collector = bench_collector();
                b.iter(|| {
                    let guards: Vec<_> = (0..count)
                        .map(|i| Gc::with_collector(&collector, SmallData(i)))
                        .collect();
                    drop(guards);
                });
            },
        );
    }

    group.finish();
}

fn collection_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("collection");

    group.bench_function("collect_no_garbage", |b| {
        let collector = collection_collector();
        // Keep roots alive so nothing is swept
        let _guards: Vec<_> = (0..100)
            .map(|i| Gc::with_collector(&collector, SmallData(i)))
            .collect();
        b.iter(|| {
            collect_and_wait(&collector);
        });
    });

    group.bench_function("collect_all_garbage", |b| {
        let collector = collection_collector();
        b.iter(|| {
            // Allocate then immediately drop roots so objects are unreachable
            for i in 0..100 {
                let guard = Gc::with_collector(&collector, SmallData(i));
                drop(guard);
            }
            collect_and_wait(&collector);
        });
    });

    group.bench_function("collect_mixed", |b| {
        let collector = collection_collector();
        b.iter(|| {
            let mut kept = Vec::new();
            for i in 0..100 {
                let guard = Gc::with_collector(&collector, SmallData(i));
                if i % 2 == 0 {
                    kept.push(guard); // keep even-numbered roots alive
                }
                // odd-numbered guards are dropped here -> garbage
            }
            collect_and_wait(&collector);
            drop(kept);
        });
    });

    group.finish();
}

fn tracing_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("tracing");

    for depth in [10, 100, 1000] {
        group.bench_with_input(
            BenchmarkId::new("linear_chain", depth),
            &depth,
            |b, &depth| {
                b.iter(|| {
                    let collector = collection_collector();
                    // Build a linked list of `depth` nodes
                    let mut prev: Option<Gc<RwLock<LinkedNode>>> = None;
                    let mut root_guard = None;
                    for _ in 0..depth {
                        let node = LinkedNode { next: prev };
                        let guard = Gc::with_collector(&collector, RwLock::new(node));
                        prev = Some(guard.clone_gc());
                        root_guard = Some(guard);
                    }
                    collect_and_wait(&collector);
                    drop(root_guard);
                });
            },
        );
    }

    for width in [10, 100] {
        group.bench_with_input(BenchmarkId::new("wide_tree", width), &width, |b, &width| {
            b.iter(|| {
                let collector = collection_collector();
                // Build a single-level tree with `width` children
                let children: Vec<_> = (0..width)
                    .map(|_| {
                        let child = WideNode {
                            children: Vec::new(),
                        };
                        let guard = Gc::with_collector(&collector, RwLock::new(child));
                        guard.clone_gc()
                    })
                    .collect();
                let root = WideNode { children };
                let _root_guard = Gc::with_collector(&collector, RwLock::new(root));
                collect_and_wait(&collector);
            });
        });
    }

    group.finish();
}

fn lifecycle_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("lifecycle");

    group.bench_function("create_destroy", |b| {
        b.iter(|| {
            let collector = bench_collector();
            drop(collector);
        });
    });

    group.bench_function("create_allocate_destroy", |b| {
        b.iter(|| {
            let collector = bench_collector();
            for i in 0..100 {
                let _guard = Gc::with_collector(&collector, SmallData(i));
            }
            drop(collector);
        });
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets =
        allocation_benchmarks,
        root_benchmarks,
        collection_benchmarks,
        tracing_benchmarks,
        lifecycle_benchmarks
);
criterion_main!(benches);
