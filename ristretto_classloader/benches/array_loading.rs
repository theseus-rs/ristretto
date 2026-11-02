//! Class and array lookup benchmarks; use the same source on the comparison revision.
#![expect(
    clippy::expect_used,
    reason = "benchmarks must fail on unsuccessful operations"
)]

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use ristretto_classfile::{ClassFile, ConstantPool, JavaStr};
use ristretto_classloader::{Class, ClassLoader, ClassLoaderType, ClassPath};
use std::hint::black_box;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;

fn fixture(runtime: &Runtime, child: bool) -> Arc<ClassLoader> {
    runtime.block_on(async {
        let bootstrap =
            ClassLoader::new_builtin(ClassLoaderType::Bootstrap, ClassPath::new(Vec::new()));
        let loader = if child {
            let loader =
                ClassLoader::new_builtin(ClassLoaderType::System, ClassPath::new(Vec::new()));
            loader.set_parent(Some(bootstrap)).await;
            loader
        } else {
            bootstrap
        };
        let mut pool = ConstantPool::new();
        let this_class = pool.add_class("bench/Element").expect("constant");
        let class = Class::from(
            Some(Arc::downgrade(&loader)),
            ClassFile {
                constant_pool: pool,
                this_class,
                ..Default::default()
            },
        )
        .expect("class");
        loader.register(class).await.expect("register");
        loader
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "related loading cases share fixtures"
)]
fn benchmarks(criterion: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");
    let loader = fixture(&runtime, false);
    for (label, name) in [
        ("ordinary_internal", "bench/Element"),
        ("ordinary_binary", "bench.Element"),
        ("object_array", "[Lbench/Element;"),
        ("primitive_array", "[I"),
        ("multidimensional_array", "[[Lbench/Element;"),
    ] {
        let name = JavaStr::try_from_str(name).expect("name");
        runtime.block_on(loader.load(name)).expect("warm lookup");
        criterion.bench_function(&["loading/warm/", label].concat(), |b| {
            b.iter(|| {
                black_box(
                    runtime
                        .block_on(loader.load(black_box(name)))
                        .expect("load"),
                )
            });
        });
    }
    for (label, name, child) in [
        ("object_array", "[Lbench/Element;", false),
        ("child_object_array", "[Lbench/Element;", true),
        ("primitive_array", "[I", false),
        ("multidimensional_array", "[[Lbench/Element;", false),
    ] {
        let name = JavaStr::try_from_str(name).expect("name");
        criterion.bench_function(&["loading/cold/", label].concat(), |b| {
            let probe = fixture(&runtime, child);
            let array = runtime.block_on(probe.load(name)).expect("array class");
            let defining_loader = array
                .class_loader()
                .expect("loader metadata")
                .expect("loader");
            assert!(Arc::ptr_eq(&probe, &defining_loader));
            b.iter_batched_ref(
                || fixture(&runtime, child),
                |loader| {
                    black_box(
                        runtime
                            .block_on(loader.load(black_box(name)))
                            .expect("load"),
                    )
                },
                BatchSize::SmallInput,
            );
        });
    }
    let parent = fixture(&runtime, false);
    let child = ClassLoader::new("child", ClassPath::new(Vec::new()));
    runtime.block_on(child.set_parent(Some(parent)));
    let ordinary_name = JavaStr::try_from_str("bench/Element").expect("name");
    criterion.bench_function("loading/warm/parent_ordinary", |b| {
        b.iter(|| black_box(runtime.block_on(child.load(ordinary_name)).expect("load")));
    });
    let classes = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../classes");
    for (label, path) in [
        ("directory", classes.clone()),
        ("jar", classes.join("classes.jar")),
    ] {
        let class_path = ClassPath::from(&[&path]);
        // Reuse the class path, as loaders sharing an archive do; each loader is fresh.
        runtime
            .block_on(class_path.read_class("HelloWorld"))
            .expect("class file");
        let name = JavaStr::try_from_str("HelloWorld").expect("name");
        criterion.bench_function(&["loading/cold/ordinary_", label].concat(), |b| {
            b.iter_batched_ref(
                || ClassLoader::new("test", class_path.clone()),
                |loader| black_box(runtime.block_on(loader.load(name)).expect("load")),
                BatchSize::SmallInput,
            );
        });
        criterion.bench_function(&["loading/fresh_path/ordinary_", label].concat(), |b| {
            b.iter_batched_ref(
                || ClassLoader::new("test", ClassPath::from(&[&path])),
                |loader| black_box(runtime.block_on(loader.load(name)).expect("load")),
                BatchSize::SmallInput,
            );
        });
    }
    let empty = ClassLoader::new("empty", ClassPath::new(Vec::new()));
    let missing = JavaStr::try_from_str("bench/Missing").expect("name");
    criterion.bench_function("loading/missing", |b| {
        b.iter(|| black_box(runtime.block_on(empty.load(missing)).expect_err("missing")));
    });
    let class = runtime
        .block_on(loader.load(JavaStr::try_from_str("bench/Element").expect("name")))
        .expect("class");
    criterion.bench_function("loading/ordinary_definition", |b| {
        b.iter(|| {
            black_box(
                Class::from(Some(Arc::downgrade(&loader)), class.class_file().clone())
                    .expect("definition"),
            )
        });
    });
    criterion.bench_function("loading/public_name", |b| {
        b.iter(|| black_box(class.public_name()));
    });
    criterion.bench_function("loading/is_hidden", |b| {
        b.iter(|| black_box(black_box(&class).is_hidden()));
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100).warm_up_time(Duration::from_secs(2)).measurement_time(Duration::from_secs(4));
    targets = benchmarks
}
criterion_main!(benches);
