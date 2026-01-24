use criterion::{Criterion, criterion_group, criterion_main};
use ristretto_classfile::mutf8;
use std::hint::black_box;

fn bench_to_bytes(c: &mut Criterion) {
    let ascii_str = "Hello, World!";
    let null_str = "Hello\u{0000}World";
    let supplementary_str = "Hello\u{1F600}World"; // contains emoji which is a surrogate pair in MUTF-8
    let mixed_str = "A\u{0000}β\u{1F600}";

    let mut group = c.benchmark_group("to_bytes");
    group.bench_function("ascii", |b| {
        b.iter(|| mutf8::to_bytes(black_box(ascii_str)));
    });
    group.bench_function("null", |b| b.iter(|| mutf8::to_bytes(black_box(null_str))));
    group.bench_function("supplementary", |b| {
        b.iter(|| mutf8::to_bytes(black_box(supplementary_str)));
    });
    group.bench_function("mixed", |b| {
        b.iter(|| mutf8::to_bytes(black_box(mixed_str)));
    });
    group.finish();
}

fn bench_from_bytes(c: &mut Criterion) {
    let ascii_bytes = b"Hello, World!";
    let null_bytes = &[
        0x48, 0x65, 0x6C, 0x6C, 0x6F, 0xC0, 0x80, 0x57, 0x6F, 0x72, 0x6C, 0x64,
    ]; // "Hello\0World"
    let supplementary_bytes = &[
        0x48, 0x65, 0x6C, 0x6C, 0x6F, 0xED, 0xA1, 0x98, 0xED, 0xB8, 0x80, 0x57, 0x6F, 0x72, 0x6C,
        0x64,
    ]; // "Hello😀World"
    // "A\0β😀"
    let mixed_bytes = &[
        0x41, 0xC0, 0x80, 0xCE, 0xB2, 0xED, 0xA0, 0xBD, 0xED, 0xB8, 0x83,
    ];

    let mut group = c.benchmark_group("from_bytes");
    group.bench_function("ascii", |b| {
        b.iter(|| mutf8::from_bytes(black_box(ascii_bytes)));
    });
    group.bench_function("null", |b| {
        b.iter(|| mutf8::from_bytes(black_box(null_bytes)));
    });
    group.bench_function("supplementary", |b| {
        b.iter(|| mutf8::from_bytes(black_box(supplementary_bytes)));
    });
    group.bench_function("mixed", |b| {
        b.iter(|| mutf8::from_bytes(black_box(mixed_bytes)));
    });
    group.finish();
}

criterion_group!(benches, bench_to_bytes, bench_from_bytes);
criterion_main!(benches);
