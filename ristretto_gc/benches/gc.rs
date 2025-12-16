use criterion::{Criterion, criterion_group, criterion_main};
use ristretto_gc::{GarbageCollector, Gc, Trace};
use std::time::Duration;

fn benchmarks(criterion: &mut Criterion) {
    bench_lifecycle(criterion);
}

fn bench_lifecycle(_criterion: &mut Criterion) {
    let collector = GarbageCollector::new();

    // Delete the following line and uncomment the next lines to enable the benchmark
    gc_objects(&collector);
    // criterion.bench_function("gc_objects", |bencher| {
    //     bencher.iter(|| gc_objects(&collector));
    // });
}

enum Data {
    Small(i32),
    Large(Vec<u8>),
}

impl Trace for Data {
    fn trace(&self, _collector: &GarbageCollector) {}
}

fn gc_objects(collector: &GarbageCollector) {
    for i in 0..1_000 {
        if i % 2 == 0 {
            Gc::with_collector(collector, Data::Small(i));
        } else {
            let data = vec![0u8; 1024 * 1024]; // 1MB
            Gc::with_collector(collector, Data::Large(data));
        }
    }
    collector.collect();
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = benchmarks
);
criterion_main!(benches);
