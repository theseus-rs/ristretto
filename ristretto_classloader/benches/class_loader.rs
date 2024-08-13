use criterion::{criterion_group, criterion_main, Criterion};
use ristretto_classloader::{runtime, ClassLoader, Result};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;

fn benchmarks(criterion: &mut Criterion) {
    bench_lifecycle(criterion).ok();
}

fn bench_lifecycle(criterion: &mut Criterion) -> Result<()> {
    let runtime = Runtime::new().unwrap();

    criterion.bench_function("runtime_v8", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                runtime_class_loader("8.422.05.1").await.ok();
            });
        });
    });
    criterion.bench_function("runtime_v11", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                runtime_class_loader("11.0.24.8.1").await.ok();
            });
        });
    });
    criterion.bench_function("runtime_v17", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                runtime_class_loader("17.0.12.7.1").await.ok();
            });
        });
    });
    criterion.bench_function("runtime_v21", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                runtime_class_loader("21.0.4.7.1").await.ok();
            });
        });
    });

    Ok(())
}

async fn runtime_class_loader(version: &str) -> Result<()> {
    let (_runtime_version, class_loader) = runtime::class_loader(version).await?;
    let _class = ClassLoader::load_class(&Arc::new(class_loader), "java.lang.Object").await?;
    Ok(())
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(10);
    targets = benchmarks
);
criterion_main!(benches);
