use criterion::{Criterion, criterion_group, criterion_main};
use ristretto_classfile::ClassFile;
use ristretto_classloader::runtime::default_class_loader;
use ristretto_jimage::{Image, Result};
use std::io::Cursor;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn benchmarks(criterion: &mut Criterion) {
    bench_lifecycle(criterion).ok();
}

fn bench_lifecycle(criterion: &mut Criterion) -> Result<()> {
    let runtime = Runtime::new()?;
    let image = runtime.block_on(async { get_image().await })?;
    let image = Arc::new(image);

    criterion.bench_function("jimage_load_hash_map", |bencher| {
        bencher.iter(|| {
            let _ = load_class(&image, "/java.base/java/util/HashMap.class").ok();
        });
    });

    Ok(())
}

async fn get_image() -> Result<Image> {
    let (java_home, _java_version, _class_loader) =
        default_class_loader().await.expect("java home");
    let path = java_home.join("lib").join("modules");
    let image = Image::from_file(&path)?;
    Ok(image)
}

fn load_class(image: &Image, class_name: &str) -> Result<()> {
    let resource = image.get_resource(class_name)?;
    let mut bytes = Cursor::new(resource.data());
    let _class_file = ClassFile::from_bytes(&mut bytes).expect("read classfile");
    Ok(())
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = benchmarks
);
criterion_main!(benches);
