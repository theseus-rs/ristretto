use criterion::{criterion_group, criterion_main, Criterion};
use ristretto_classfile::{ClassFile, Result};
use std::io::Cursor;

const CLASS_BYTES: &[u8] = include_bytes!("../classes/Simple.class");

fn benchmarks(criterion: &mut Criterion) {
    bench_lifecycle(criterion).ok();
}

fn bench_lifecycle(criterion: &mut Criterion) -> Result<()> {
    criterion.bench_function("from_bytes", |bencher| {
        bencher.iter(|| {
            from_bytes().ok();
        });
    });

    Ok(())
}

fn from_bytes() -> Result<()> {
    let mut original_bytes = Cursor::new(CLASS_BYTES.to_vec());
    let _class_file = ClassFile::from_bytes(&mut original_bytes)?;
    Ok(())
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = benchmarks
);
criterion_main!(benches);
