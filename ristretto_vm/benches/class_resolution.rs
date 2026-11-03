//! Mirror and explicit-lookup benchmarks shared with the comparison revision.
#![expect(
    clippy::expect_used,
    reason = "benchmarks must fail on unsuccessful operations"
)]

use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use ristretto_classfile::{ClassFile, ConstantPool, VerifyMode};
use ristretto_classloader::{Class, Reference, Value};
use ristretto_gc::{ConfigurationBuilder as GcConfigurationBuilder, GarbageCollector};
use ristretto_intrinsics::java::lang::class::{for_name_0, get_class_no_init};
use ristretto_intrinsics::java::lang::reflect::array::{multi_new_array, new_array};
use ristretto_types::{JavaObject, Parameters};
use ristretto_vm::{ConfigurationBuilder, Thread, VM};
use std::hint::black_box;
use std::sync::Arc;
use std::time::Duration;

struct AllocationFixture<'a> {
    runtime: &'a tokio::runtime::Runtime,
    vm: Arc<VM>,
    thread: Arc<Thread>,
    parameters: Parameters,
}

impl Drop for AllocationFixture<'_> {
    fn drop(&mut self) {
        self.runtime
            .block_on(self.vm.wait_for_non_daemon_threads())
            .expect("shutdown allocation VM");
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "related lookup benchmarks share VM setup"
)]
fn benchmarks(criterion: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");
    let configuration = ConfigurationBuilder::new()
        .interpreted(true)
        .verify_mode(VerifyMode::None)
        .garbage_collector(GarbageCollector::with_config(
            GcConfigurationBuilder::new().threads(1).build(),
        ))
        .build()
        .expect("configuration");
    let vm = runtime.block_on(VM::new(configuration)).expect("VM");
    let thread = Thread::new(&Arc::downgrade(&vm), 3);
    runtime.block_on(async {
        let bootstrap = thread
            .class("java.lang.Object")
            .await
            .expect("Object")
            .class_loader()
            .expect("loader metadata")
            .expect("bootstrap loader");
        let mut pool = ConstantPool::new();
        let this_class = pool
            .add_class("bench/BootstrapGenerated")
            .expect("constant");
        let generated = Class::from(
            Some(Arc::downgrade(&bootstrap)),
            ClassFile {
                constant_pool: pool,
                this_class,
                ..Default::default()
            },
        )
        .expect("generated class");
        ristretto_types::Thread::register_class(&*thread, generated)
            .await
            .expect("register");
    });
    let system_loader = runtime
        .block_on(thread.try_invoke(
            "java.lang.ClassLoader",
            "getSystemClassLoader()Ljava/lang/ClassLoader;",
            &[] as &[Value],
        ))
        .expect("system loader");
    for (label, name) in [
        ("ordinary", "java.lang.String"),
        ("object_array", "[Ljava.lang.String;"),
        ("primitive_array", "[[I"),
        ("bootstrap_generated", "bench.BootstrapGenerated"),
    ] {
        let (mirror, name_value) = runtime.block_on(async {
            let class = thread.class(name).await.expect("class");
            (
                class.to_object(&thread).await.expect("mirror"),
                name.to_object(&thread).await.expect("name"),
            )
        });
        criterion.bench_function(&["resolution/mirror/", label].concat(), |b| {
            b.iter(|| {
                black_box(
                    runtime
                        .block_on(get_class_no_init(&thread, black_box(&mirror)))
                        .expect("resolve"),
                )
            });
        });
        criterion.bench_function(&["resolution/for_name/", label].concat(), |b| {
            b.iter(|| {
                let parameters: Parameters = [
                    name_value.clone(),
                    Value::from(false),
                    Value::Object(None),
                    Value::Object(None),
                ]
                .into_iter()
                .collect();
                black_box(
                    runtime
                        .block_on(for_name_0(thread.clone(), parameters))
                        .expect("forName"),
                )
            });
        });
        criterion.bench_function(&["resolution/for_name_system/", label].concat(), |b| {
            b.iter(|| {
                let parameters: Parameters = [
                    name_value.clone(),
                    Value::from(false),
                    system_loader.clone(),
                    Value::Object(None),
                ]
                .into_iter()
                .collect();
                black_box(
                    runtime
                        .block_on(for_name_0(thread.clone(), parameters))
                        .expect("forName"),
                )
            });
        });
    }
    runtime
        .block_on(vm.wait_for_non_daemon_threads())
        .expect("shutdown");
}

// Validate outside the timed region: incorrect shapes must never look like faster allocations.
#[expect(
    clippy::panic,
    reason = "invalid benchmark results must fail validation"
)]
fn validate_array(value: &Value, descriptor: &str, dimensions: &[i32]) {
    let reference = value.as_reference().expect("array reference");
    assert_eq!(reference.class_name().expect("array class"), descriptor);
    let (&length, remaining) = dimensions.split_first().expect("array dimensions");
    let length = usize::try_from(length).expect("nonnegative length");
    match &*reference {
        Reference::Array(array) => {
            assert_eq!(array.elements.len(), length);
            if dimensions.len() > 1 {
                for child in &array.elements {
                    validate_array(
                        child,
                        descriptor.strip_prefix('[').expect("array descriptor"),
                        remaining,
                    );
                }
            } else {
                assert!(
                    array
                        .elements
                        .iter()
                        .all(|v| matches!(v, Value::Object(None)))
                );
            }
        }
        Reference::IntArray(array) => {
            assert_eq!(dimensions.len(), 1);
            assert_eq!(array.len(), length);
            assert!(array.iter().all(|&v| v == 0));
        }
        _ => panic!("unexpected array kind"),
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "allocation cases share bounded VM setup"
)]
fn allocation_benchmarks(criterion: &mut Criterion) {
    const ALLOCATIONS_PER_BATCH: u64 = 1024;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");
    let mut group = criterion.benchmark_group("allocation");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(250));
    group.measurement_time(Duration::from_secs(1));
    group.throughput(Throughput::Elements(ALLOCATIONS_PER_BATCH));
    for (label, name, dimensions, descriptor, single) in [
        (
            "reflect_new_array",
            "java.lang.String",
            &[1][..],
            "[Ljava/lang/String;",
            true,
        ),
        (
            "reflect_multi_new_array/primitive",
            "int",
            &[4, 4][..],
            "[[I",
            false,
        ),
        (
            "reflect_multi_new_array/reference",
            "java.lang.String",
            &[4, 4][..],
            "[[Ljava/lang/String;",
            false,
        ),
        (
            "reflect_multi_new_array/array_component",
            "[Ljava.lang.String;",
            &[4, 4][..],
            "[[[Ljava/lang/String;",
            false,
        ),
        (
            "reflect_multi_new_array/three_dimensional",
            "int",
            &[4, 4, 4][..],
            "[[[I",
            false,
        ),
        (
            "reflect_multi_new_array/empty_outer",
            "java.lang.String",
            &[0, 4][..],
            "[[Ljava/lang/String;",
            false,
        ),
    ] {
        group.bench_function(label, |b| {
            // Java allocations are retained until VM teardown. Bound each VM's
            // allocation count and keep setup, warming, and teardown outside timing.
            b.iter_batched_ref(
                || {
                    let configuration = ConfigurationBuilder::new()
                        .interpreted(true)
                        .verify_mode(VerifyMode::None)
                        .garbage_collector(GarbageCollector::with_config(
                            GcConfigurationBuilder::new().threads(1).build(),
                        ))
                        .build()
                        .expect("configuration");
                    let vm = runtime.block_on(VM::new(configuration)).expect("VM");
                    let thread = Thread::new(&Arc::downgrade(&vm), 3);
                    let parameters = runtime.block_on(async {
                        let class = thread.class(name).await.expect("component");
                        let mirror = class.to_object(&thread).await.expect("mirror");
                        let lengths = if single {
                            Value::Int(*dimensions.first().expect("array length"))
                        } else {
                            Value::new_object(
                                vm.garbage_collector(),
                                Reference::from(dimensions.to_vec()),
                            )
                        };
                        let parameters: Parameters = [mirror, lengths].into_iter().collect();
                        let allocated = if single {
                            new_array(thread.clone(), parameters.clone()).await
                        } else {
                            multi_new_array(thread.clone(), parameters.clone()).await
                        }
                        .expect("warm allocation")
                        .expect("array value");
                        validate_array(&allocated, descriptor, dimensions);
                        parameters
                    });
                    AllocationFixture {
                        runtime: &runtime,
                        vm,
                        thread,
                        parameters,
                    }
                },
                |fixture| {
                    runtime.block_on(async {
                        for _ in 0..ALLOCATIONS_PER_BATCH {
                            black_box(
                                if single {
                                    new_array(fixture.thread.clone(), fixture.parameters.clone())
                                        .await
                                } else {
                                    multi_new_array(
                                        fixture.thread.clone(),
                                        fixture.parameters.clone(),
                                    )
                                    .await
                                }
                                .expect("allocate array"),
                            );
                        }
                    });
                },
                BatchSize::PerIteration,
            );
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100).warm_up_time(Duration::from_secs(2)).measurement_time(Duration::from_secs(4));
    targets = benchmarks, allocation_benchmarks
}
criterion_main!(benches);
