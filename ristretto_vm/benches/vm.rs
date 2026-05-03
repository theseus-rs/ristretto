use criterion::{Criterion, criterion_group, criterion_main};
use ristretto_classfile::Error;
use ristretto_classloader::ClassPath;
use ristretto_vm::{ConfigurationBuilder, Result, VM};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

const CARGO_MANIFEST: &str = env!("CARGO_MANIFEST_DIR");

fn benchmarks(criterion: &mut Criterion) {
    bench_lifecycle(criterion).ok();
}

fn bench_lifecycle(criterion: &mut Criterion) -> Result<()> {
    let mut builder = {
        #[cfg(target_family = "wasm")]
        {
            tokio::runtime::Builder::new_current_thread()
        }
        #[cfg(not(target_family = "wasm"))]
        {
            tokio::runtime::Builder::new_multi_thread()
        }
    };
    let runtime = builder
        .enable_all()
        .build()
        .map_err(|error| Error::IoError(error.to_string()))?;

    criterion.bench_function("vm_init_int", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                vm_init(true).await.ok();
            });
        });
    });
    criterion.bench_function("vm_init", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                vm_init(false).await.ok();
            });
        });
    });
    criterion.bench_function("hello_world_int", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                hello_world(true).await.ok();
            });
        });
    });
    criterion.bench_function("hello_world", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                hello_world(false).await.ok();
            });
        });
    });

    Ok(())
}

async fn vm_init(interpreted: bool) -> Result<()> {
    let cargo_manifest = PathBuf::from(CARGO_MANIFEST);
    let classes_jar_path = cargo_manifest
        .join("..")
        .join("classes")
        .join("classes.jar");
    let class_path = ClassPath::from(&[classes_jar_path]);
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path)
        .interpreted(interpreted)
        .build()?;
    let _ = VM::new(configuration).await?;
    Ok(())
}

async fn hello_world(interpreted: bool) -> Result<()> {
    let cargo_manifest = PathBuf::from(CARGO_MANIFEST);
    let classes_jar_path = cargo_manifest
        .join("..")
        .join("classes")
        .join("classes.jar");
    let class_path = ClassPath::from(&[classes_jar_path]);
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path)
        .interpreted(interpreted)
        .stdout(Arc::new(Mutex::new(std::io::sink())))
        .main_class("HelloWorld")
        .build()?;
    let vm = VM::new(configuration).await?;
    let parameters: Vec<&str> = Vec::new();
    let _result = vm.invoke_main(&parameters).await?;
    Ok(())
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = benchmarks
);
criterion_main!(benches);
