use criterion::{Criterion, criterion_group, criterion_main};
use ristretto_classloader::{ClassLoader, Result, runtime};
use std::sync::Arc;
use tokio::runtime::Runtime;

fn benchmarks(criterion: &mut Criterion) {
    bench_lifecycle(criterion).ok();
}

fn bench_lifecycle(criterion: &mut Criterion) -> Result<()> {
    let runtime = Runtime::new()?;
    let class_loader = runtime.block_on(async { default_class_loader().await })?;
    let class_loader = Arc::new(class_loader);

    criterion.bench_function("default_class_loader", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                let class_loader = default_class_loader()
                    .await
                    .expect("Failed to create class loader");
                let _class = class_loader.load("java.lang.Object").await.ok();
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

async fn default_class_loader() -> Result<Arc<ClassLoader>> {
    let (_java_home, _version, class_loader) = runtime::default_class_loader().await?;
    let class_loader = Arc::new(class_loader);
    Ok(class_loader)
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = benchmarks
);
criterion_main!(benches);
