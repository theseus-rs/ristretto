use criterion::{criterion_group, criterion_main, Criterion};
use ristretto_classfile::{ClassFile, Result};
use std::io::Cursor;

const CLASS_BYTES: &[u8] = include_bytes!("../../classes/Simple.class");

fn benchmarks(criterion: &mut Criterion) {
    bench_lifecycle(criterion).ok();
}

fn bench_lifecycle(criterion: &mut Criterion) -> Result<()> {
    let class_file = ClassFile::from_bytes(&mut Cursor::new(CLASS_BYTES.to_vec()))?;
    criterion.bench_function("from_bytes", |bencher| {
        bencher.iter(|| {
            from_bytes().ok();
        });
    });
    criterion.bench_function("to_bytes", |bencher| {
        bencher.iter(|| {
            to_bytes(&class_file).ok();
        });
    });
    criterion.bench_function("verify", |bencher| {
        bencher.iter(|| {
            verify(&class_file).ok();
        });
    });
    criterion.bench_function("to_string", |bencher| {
        bencher.iter(|| {
            to_string(&class_file).ok();
        });
    });

    Ok(())
}

fn from_bytes() -> Result<()> {
    let mut original_bytes = Cursor::new(CLASS_BYTES.to_vec());
    let _class_file = ClassFile::from_bytes(&mut original_bytes)?;
    Ok(())
}

fn to_bytes(class_file: &ClassFile) -> Result<()> {
    class_file.to_bytes(&mut Vec::new())?;
    Ok(())
}

fn verify(class_file: &ClassFile) -> Result<()> {
    class_file.verify()?;
    Ok(())
}

fn to_string(class_file: &ClassFile) -> Result<()> {
    class_file.to_string();
    Ok(())
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = benchmarks
);
criterion_main!(benches);
