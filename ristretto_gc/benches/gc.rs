use criterion::{Criterion, criterion_group, criterion_main};
use ristretto_gc::{GarbageCollector, Gc, Result};
use std::time::Duration;

fn benchmarks(criterion: &mut Criterion) {
    bench_lifecycle(criterion).ok();
}

fn bench_lifecycle(criterion: &mut Criterion) -> Result<()> {
    let collector = GarbageCollector::new();

    criterion.bench_function("gc_objects", |bencher| {
        bencher.iter(|| gc_objects(&collector));
    });

    Ok(())
}

#[expect(dead_code)]
enum Data {
    Small(i32),
    Large(Vec<u8>),
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
