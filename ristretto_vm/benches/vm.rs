use criterion::{Criterion, criterion_group, criterion_main};
use ristretto_classfile::Error;
use ristretto_classloader::ClassPath;
use ristretto_vm::{ConfigurationBuilder, Result, VM};
use std::path::PathBuf;
use tokio::runtime::Runtime;

const CARGO_MANIFEST: &str = env!("CARGO_MANIFEST_DIR");

fn benchmarks(criterion: &mut Criterion) {
    bench_lifecycle(criterion).ok();
}

fn bench_lifecycle(criterion: &mut Criterion) -> Result<()> {
    let runtime = Runtime::new().map_err(|error| Error::IoError(error.to_string()))?;

    criterion.bench_function("vm_init", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                vm_init().await.ok();
            });
        });
    });
    criterion.bench_function("hello_world", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                hello_world().await.ok();
            });
        });
    });

    Ok(())
}

async fn vm_init() -> Result<()> {
    let cargo_manifest = PathBuf::from(CARGO_MANIFEST);
    let classes_jar_path = cargo_manifest
        .join("..")
        .join("classes")
        .join("classes.jar");
    let class_path = ClassPath::from(classes_jar_path.to_string_lossy());
    let configuration = ConfigurationBuilder::new().class_path(class_path).build()?;
    let _ = VM::new(configuration).await?;
    Ok(())
}

async fn hello_world() -> Result<()> {
    let cargo_manifest = PathBuf::from(CARGO_MANIFEST);
    let classes_jar_path = cargo_manifest
        .join("..")
        .join("classes")
        .join("classes.jar");
    let class_path = ClassPath::from(classes_jar_path.to_string_lossy());
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path)
        .main_class("HelloWorld")
        .build()?;
    let vm = VM::new(configuration).await?;
    let parameters: Vec<&str> = Vec::new();
    let _result = vm.invoke_main(parameters).await?;
    Ok(())
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = benchmarks
);
criterion_main!(benches);
