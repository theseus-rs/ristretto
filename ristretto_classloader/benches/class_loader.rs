use criterion::{criterion_group, criterion_main, Criterion};
use ristretto_classloader::{runtime, Result};

fn benchmarks(criterion: &mut Criterion) {
    bench_lifecycle(criterion).ok();
}

fn bench_lifecycle(criterion: &mut Criterion) -> Result<()> {
    let (_version, class_loader) = runtime::class_loader("21.0.4.7.1")?;
    let class_loader = class_loader;

    criterion.bench_function("runtime_v8", |bencher| {
        bencher.iter(|| {
            runtime_class_loader("8.422.05.1").ok();
        });
    });
    criterion.bench_function("runtime_v11", |bencher| {
        bencher.iter(|| {
            runtime_class_loader("11.0.24.8.1").ok();
        });
    });
    criterion.bench_function("runtime_v17", |bencher| {
        bencher.iter(|| {
            runtime_class_loader("17.0.12.7.1").ok();
        });
    });
    criterion.bench_function("runtime_v21", |bencher| {
        bencher.iter(|| {
            runtime_class_loader("21.0.4.7.1").ok();
        });
    });
    criterion.bench_function("load_hash_map", |bencher| {
        bencher.iter(|| {
            let _ = class_loader.load("java/util/HashMap").ok();
        });
    });
    criterion.bench_function("load_invalid_class", |bencher| {
        bencher.iter(|| {
            let _ = class_loader.load("foo").err();
        });
    });

    Ok(())
}

fn runtime_class_loader(version: &str) -> Result<()> {
    let (_runtime_version, class_loader) = runtime::class_loader(version)?;
    let _class = class_loader.load("java.lang.Object")?;
    Ok(())
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = benchmarks
);
criterion_main!(benches);
