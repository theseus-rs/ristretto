//! Isolated helper-cost fixture, invoked by `benchmarks/gc/frame_roots.py`.
//! It does not enable reference-bearing production compilation or measure a JVM
//! workload. VM startup, warmup, content checks and collection are outside timing.
use super::*;
use ristretto_classfile::{ClassFile, ConstantPool};
use std::hint::black_box;
use std::time::Instant;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "isolated benchmark; run benchmarks/gc/frame_roots.py with frozen release test binaries"]
#[expect(
    clippy::too_many_lines,
    reason = "keep setup, timed region and post-timing ownership checks visible together in this isolated fixture"
)]
async fn measure_frame_publication() -> Result<()> {
    if cfg!(feature = "execution-profile") {
        return Err(InternalError(
            "timing requires profiling disabled".to_owned(),
        ));
    }
    let workload = std::env::var("RISTRETTO_FRAME_WORKLOAD")
        .map_err(|error| InternalError(error.to_string()))?;
    let policy = std::env::var("RISTRETTO_FRAME_POLICY")
        .map_err(|error| InternalError(error.to_string()))?;
    let operations = environment_count("RISTRETTO_FRAME_OPERATIONS")?;
    let warmup = environment_count("RISTRETTO_FRAME_WARMUP")?;
    let concurrent = match policy.as_str() {
        "gated" => false,
        "counted" => true,
        _ => return Err(InternalError("expected gated or counted policy".to_owned())),
    };
    let gc = GarbageCollector::with_config(
        ristretto_gc::Configuration::builder()
            .max_heap_bytes(64 * 1024 * 1024)
            .allocation_threshold(usize::MAX)
            .max_pause_time_us(0)
            .threads(0)
            .root_storage(ristretto_gc::RootStorage::Contexts)
            .local_allocation_buffers(true)
            .barrier_buffer_size(32)
            .concurrent_publication(concurrent)
            .build(),
    );
    let java_home = std::env::var_os("RISTRETTO_FRAME_JDK")
        .ok_or_else(|| InternalError("missing fixture JDK".to_owned()))?;
    let vm = VM::new(
        crate::ConfigurationBuilder::new()
            .java_home(java_home.into())
            .garbage_collector(gc.clone())
            .batch_compilation(false)
            .interpreted(true)
            .build()?,
    )
    .await?;
    let thread = Thread::new(&Arc::downgrade(&vm), 3);
    let mut constant_pool = ConstantPool::default();
    let this_class = constant_pool.add_class("FramePublicationBenchmark")?;
    let class = Class::from(
        None,
        ClassFile {
            constant_pool,
            this_class,
            ..Default::default()
        },
    )?;
    let ctx = RuntimeContext::new(&gc, &vm, &thread, &class)?;
    // VM startup calls start(); native threads=0 selects one worker. Stop/join
    // it explicitly so the fixture's workerless label matches the actual state.
    gc.stop()?;
    assert_eq!(gc.registered_workers(), 0);
    let execution = gc.register_context();
    let measurement = execution.with(|| -> Result<_> {
        let (count, changing) = match workload.as_str() {
            "empty" => (0, false),
            "stable1" | "duplicates" => (1, false),
            "stable8" => (8, false),
            "stable64" => (64, false),
            "changing1" => (1, true),
            "changing8" => (8, true),
            _ => {
                return Err(InternalError(
                    "unknown frame publication workload".to_owned(),
                ));
            }
        };
        let targets = (0..count * if changing { 2 } else { 1 })
            .map(|_| {
                Ok(
                    Gc::try_new(&gc, RwLock::new(Reference::IntArray(vec![42].into())))?
                        .into_rooted(),
                )
            })
            .collect::<Result<Vec<_>>>()?;
        let weak = targets
            .iter()
            .map(|target| ristretto_gc::GcWeak::new(target, ristretto_gc::ReferenceKind::Weak, 0))
            .collect::<Vec<_>>();
        let first = targets
            .iter()
            .take(count)
            .map(Gc::as_handle_i64)
            .collect::<Vec<_>>();
        let mut second = if changing {
            targets
                .iter()
                .skip(count)
                .map(Gc::as_handle_i64)
                .collect::<Vec<_>>()
        } else {
            first.clone()
        };
        if workload == "duplicates" {
            second = first.iter().copied().cycle().take(7).collect();
            second.insert(0, 0);
            second.push(0);
        }
        // Indirect call preserves the generated-code/helper boundary. Spill stores
        // and compiled instruction execution are not included in this microbenchmark.
        let publish =
            black_box(jit_publish_frame_roots as extern "C" fn(*const u8, *const i64, i32) -> i32);
        let first_len = i32::try_from(first.len())?;
        let second_len = i32::try_from(second.len())?;
        let run = |count: u64| -> Result<u64> {
            let mut checksum = 0;
            for index in 0..count {
                let (handles, length) = if index % 2 == 0 {
                    (&first, first_len)
                } else {
                    (&second, second_len)
                };
                checksum +=
                    u64::try_from(publish(ctx.as_ptr(), black_box(handles.as_ptr()), length))?;
            }
            Ok(checksum)
        };
        assert_eq!(run(warmup)?, warmup);
        let collections_before = gc.statistics()?.collections_completed;
        let workers_before = gc.registered_workers();
        let started = Instant::now();
        let checksum = run(operations)?;
        let elapsed_ns = started.elapsed().as_nanos();
        let workers_after = gc.registered_workers();
        assert_eq!((workers_before, workers_after), (0, 0));
        let collections_during = gc.statistics()?.collections_completed - collections_before;
        assert_eq!(checksum, operations);
        assert_eq!(ctx.pending_exception(), 0);
        let expected = if operations % 2 == 1 { &first } else { &second };
        let expected = expected
            .iter()
            .copied()
            .filter(|&handle| handle != 0)
            .collect::<std::collections::BTreeSet<_>>();
        let actual = ctx
            .reference_roots
            .lock()
            .keys()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(actual, expected);
        let mut payload_checksum = 0;
        for target in &targets {
            match &*target.read() {
                Reference::IntArray(array) => {
                    payload_checksum += i64::from(
                        *array
                            .first()
                            .ok_or_else(|| InternalError("missing fixture element".to_owned()))?,
                    );
                }
                _ => return Err(InternalError("fixture payload changed type".to_owned())),
            }
        }
        let target_count = targets.len();
        assert_eq!(payload_checksum, i64::try_from(target_count)? * 42);
        drop(targets);
        drop(ctx);
        gc.collect_full()?;
        gc.collect_full()?;
        let survivors = weak
            .iter()
            .filter(|reference| reference.upgrade().is_some())
            .count();
        assert_eq!(survivors, 0);
        Ok(serde_json::json!({
            "schema_version": 2, "workload": workload, "policy": policy,
            "operations": operations, "warmup": warmup, "elapsed_ns": elapsed_ns,
            "checksum": checksum, "target_count": target_count,
            "payload_checksum": payload_checksum, "survivors": survivors,
            "collections_during": collections_during,
            "registered_workers_before": workers_before,
            "registered_workers_after": workers_after,
            "execution_profile": cfg!(feature = "execution-profile"),
            "qualification": false,
        }))
    })?;
    println!("FRAME_PUBLICATION {measurement}");
    Ok(())
}

fn environment_count(name: &str) -> Result<u64> {
    let value = std::env::var(name).map_err(|error| InternalError(error.to_string()))?;
    let value = value
        .parse::<u64>()
        .map_err(|error| InternalError(error.to_string()))?;
    if !(1..=10_000_000).contains(&value) {
        return Err(InternalError(format!("{name} must be in 1..=10000000")));
    }
    Ok(value)
}
