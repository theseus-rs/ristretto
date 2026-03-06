use criterion::{Criterion, criterion_group, criterion_main};
use ristretto_classfile::{ClassFile, Result};

const CLASS_BYTES: &[u8] = include_bytes!("../../classes/Simple.class");

fn benchmarks(criterion: &mut Criterion) {
    bench_lifecycle(criterion).ok();
}

fn bench_lifecycle(criterion: &mut Criterion) -> Result<()> {
    let class_file = ClassFile::from_slice(CLASS_BYTES)?;
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
            to_string(&class_file);
        });
    });

    Ok(())
}

fn from_bytes() -> Result<()> {
    let _class_file = ClassFile::from_slice(CLASS_BYTES)?;
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

fn to_string(class_file: &ClassFile) {
    class_file.to_string();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = benchmarks
);
criterion_main!(benches);
