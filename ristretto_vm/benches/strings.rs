//! Java string construction and repeated string-pool lookup benchmarks.
#![expect(
    clippy::expect_used,
    reason = "benchmarks must fail on unsuccessful operations"
)]

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use ristretto_classfile::{JavaString, VerifyMode};
use ristretto_gc::{ConfigurationBuilder as GcConfigurationBuilder, GarbageCollector};
use ristretto_types::JavaObject;
use ristretto_vm::{ConfigurationBuilder, Thread, VM};
use std::hint::black_box;
use std::sync::Arc;
use std::time::Duration;

struct Fixture<'a> {
    runtime: &'a tokio::runtime::Runtime,
    vm: Arc<VM>,
    thread: Arc<Thread>,
}

impl<'a> Fixture<'a> {
    fn new(runtime: &'a tokio::runtime::Runtime) -> Self {
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
        Self {
            runtime,
            vm,
            thread,
        }
    }
}

impl Drop for Fixture<'_> {
    fn drop(&mut self) {
        self.runtime
            .block_on(self.vm.wait_for_non_daemon_threads())
            .expect("shutdown");
    }
}

fn benchmarks(criterion: &mut Criterion) {
    const STRINGS_PER_BATCH: u64 = 1024;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");
    let inputs = [
        ("empty", String::new()),
        ("ascii", "java/lang/String".to_owned()),
        ("ascii_long", "java/lang/String".repeat(256)),
        ("latin1", "caf\u{e9}\u{ff}".repeat(512)),
        ("bmp", "\u{100}\u{20ac}\u{4e2d}".repeat(512)),
        ("supplementary", "\u{1f600}".repeat(512)),
        ("null", "a\0b".repeat(512)),
    ];

    let mut group = criterion.benchmark_group("string_creation");
    group.throughput(Throughput::Elements(STRINGS_PER_BATCH));
    for (name, input) in &inputs {
        let java_string = JavaString::from(input.as_str());
        let java_str = java_string.as_java_str();
        for source in ["str", "String", "JavaStr"] {
            group.bench_function(BenchmarkId::new(source, name), |b| {
                // Java allocations live until VM teardown. Bound their number and
                // exclude VM setup, class resolution, validation, and teardown.
                b.iter_batched_ref(
                    || {
                        let fixture = Fixture::new(&runtime);
                        runtime.block_on(async {
                            for value in [
                                input
                                    .as_str()
                                    .to_object(&fixture.thread)
                                    .await
                                    .expect("str"),
                                input.to_object(&fixture.thread).await.expect("String"),
                                java_str.to_object(&fixture.thread).await.expect("JavaStr"),
                            ] {
                                assert_eq!(value.as_string().expect("contents"), *input);
                            }
                        });
                        fixture
                    },
                    |fixture| {
                        runtime.block_on(async {
                            for _ in 0..STRINGS_PER_BATCH {
                                let value = match source {
                                    "str" => {
                                        black_box(input.as_str()).to_object(&fixture.thread).await
                                    }
                                    "String" => black_box(input).to_object(&fixture.thread).await,
                                    _ => black_box(java_str).to_object(&fixture.thread).await,
                                };
                                black_box(value.expect("string construction"));
                            }
                        });
                    },
                    BatchSize::PerIteration,
                );
            });
        }
    }
    group.finish();

    let fixture = Fixture::new(&runtime);
    let thread = &fixture.thread;
    let mut group = criterion.benchmark_group("string_pool_hit");
    for (name, input) in &inputs {
        runtime
            .block_on(ristretto_types::Thread::intern_string(&**thread, input))
            .expect("intern");
        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, input| {
            b.iter(|| {
                black_box(
                    runtime
                        .block_on(ristretto_types::Thread::intern_string(
                            &**thread,
                            black_box(input),
                        ))
                        .expect("intern"),
                )
            });
        });
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(3));
    targets = benchmarks
);
criterion_main!(benches);
