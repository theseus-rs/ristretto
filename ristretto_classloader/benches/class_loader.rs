use criterion::{criterion_group, criterion_main, Criterion};
use ristretto_classloader::{runtime, Result};
use std::sync::Arc;
use tokio::runtime::Runtime;

fn benchmarks(criterion: &mut Criterion) {
    bench_lifecycle(criterion).ok();
}

fn bench_lifecycle(criterion: &mut Criterion) -> Result<()> {
    let runtime = Runtime::new()?;
    let (_version, class_loader) =
        runtime.block_on(async { runtime::class_loader("21.0.5.11.1").await })?;
    let class_loader = Arc::new(class_loader);

    criterion.bench_function("runtime_v8", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                runtime_class_loader("8.432.06.1").await.ok();
            });
        });
    });
    criterion.bench_function("runtime_v11", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                runtime_class_loader("11.0.25.9.1").await.ok();
            });
        });
    });
    criterion.bench_function("runtime_v17", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                runtime_class_loader("17.0.13.11.1").await.ok();
            });
        });
    });
    criterion.bench_function("runtime_v21", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                runtime_class_loader("21.0.5.11.1").await.ok();
            });
        });
    });
    criterion.bench_function("load_hash_map", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                let _ = class_loader.load("java.util.HashMap").await.ok();
            });
        });
    });
    criterion.bench_function("load_invalid_class", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                let _ = class_loader.load("foo").await.err();
            });
        });
    });

    Ok(())
}

async fn runtime_class_loader(version: &str) -> Result<()> {
    let (_runtime_version, class_loader) = runtime::class_loader(version).await?;
    let _class = class_loader.load("java.lang.Object").await?;
    Ok(())
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = benchmarks
);
criterion_main!(benches);
