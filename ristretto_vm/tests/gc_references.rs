//! Executes reference processing and finalization in Ristretto on supported JDK class libraries.
#![cfg(not(target_family = "wasm"))]
#![expect(clippy::panic_in_result_fn)]
mod common;
use ristretto_classloader::{
    JAVA_8_VERSION, JAVA_11_VERSION, JAVA_17_VERSION, JAVA_21_VERSION, JAVA_25_VERSION, Value,
};
use ristretto_vm::{
    ClassPath, ConfigurationBuilder, Error, JavaError, JavaObject, Result, Thread, VM,
};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

fn compile_reference_fixture() -> Result<PathBuf> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let source = root.join("tests/fixtures/gc");
    let output = root.join("../target/gc-reference-fixtures");
    let home = common::java::home(JAVA_25_VERSION)?;
    common::java::compile(
        &home,
        &source,
        &output,
        Some("8"),
        false,
        Duration::from_secs(30),
    )?;
    Ok(output)
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "one reference lifecycle across library and publication policies"
)]
fn reference_ordering_resurrection_and_finalization_once() -> Result<()> {
    let output = compile_reference_fixture()?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    runtime.block_on(async {
        for (concurrent, overlap) in [(false, false), (true, false), (false, true), (true, true)] {
            for compact in [false, true] {
                for version in [
                    JAVA_8_VERSION,
                    JAVA_11_VERSION,
                    JAVA_17_VERSION,
                    JAVA_21_VERSION,
                    JAVA_25_VERSION,
                ] {
                    let gc = ristretto_gc::GarbageCollector::with_config(
                        ristretto_gc::Configuration::builder()
                            .concurrent_publication(concurrent)
                            .overlapping_generations(overlap)
                            .old_collection_interval(if overlap { 4 } else { 0 })
                            .independent_phases(overlap)
                            .region_shards(if overlap { 16 } else { 1 })
                            .compact_heap(compact)
                            .cost_aware_relocation(compact)
                            .relocation_reserve_bytes(1024 * 1024)
                            .threads(4)
                            .work_stealing(true)
                            .root_storage(ristretto_gc::RootStorage::Contexts)
                            .barrier_buffer_size(4)
                            .local_allocation_buffers(true)
                            .adaptive_workers(true)
                            .independent_relocation(true)
                            .independent_marking(true)
                            .metadata_shards(16)
                            .mark_batch_size(8)
                            .build(),
                    );
                    let config = ConfigurationBuilder::new()
                        .garbage_collector(gc)
                        .java_version(version)
                        .class_path(ClassPath::from(std::slice::from_ref(&output)))
                        .build()?;
                    let vm = VM::new(config).await?;
                    assert_eq!(
                        vm.invoke("Test", "cloneCopies()Z", &[] as &[Value]).await?,
                        Some(Value::from(true))
                    );
                    vm.invoke("Test", "allocate()V", &[] as &[Value]).await?;
                    let class = vm.class("Test").await?;
                    vm.garbage_collector().collect_full()?;
                    vm.garbage_collector().collect_full()?;
                    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
                    while class.static_value("finalized")?.as_i32()? == 0 {
                        assert!(
                            tokio::time::Instant::now() < deadline,
                            "finalizer did not run on {version}"
                        );
                        tokio::time::sleep(Duration::from_millis(1)).await;
                    }
                    assert_eq!(class.static_value("finalized")?.as_i32()?, 1, "{version}");
                    assert!(!class.static_value("resurrected")?.is_null(), "{version}");
                    assert!(
                        class
                            .static_value("weak")?
                            .as_object_ref()?
                            .value("referent")?
                            .is_null(),
                        "{version}"
                    );
                    vm.invoke("Test", "release()V", &[] as &[Value]).await?;
                    vm.garbage_collector().collect_full()?;
                    vm.garbage_collector().collect_full()?;
                    assert_eq!(class.static_value("finalized")?.as_i32()?, 1, "{version}");
                    let mut queued = 0;
                    while queued != 3 {
                        if let Some(value) = vm.invoke("Test", "poll()I", &[] as &[Value]).await? {
                            queued |= value.as_i32()?;
                        }
                        assert!(
                            tokio::time::Instant::now() < deadline,
                            "references not enqueued on {version}: {queued}"
                        );
                        tokio::time::sleep(Duration::from_millis(1)).await;
                    }
                    vm.invoke("Test", "allocateFinalizableReference()V", &[] as &[Value])
                        .await?;
                    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
                    while class.static_value("referenceOwnerFinalized")?.as_i32()? == 0 {
                        vm.garbage_collector().collect_full()?;
                        assert!(
                            tokio::time::Instant::now() < deadline,
                            "Reference subclass finalizer did not run on {version}"
                        );
                        tokio::time::sleep(Duration::from_millis(1)).await;
                    }
                    assert_eq!(
                        class.static_value("referenceOwnerCleared")?.as_i32()?,
                        1,
                        "weak clearing must include finalizable Reference objects on {version}"
                    );
                }
                check_allocation_pressure(output.clone(), compact, concurrent, overlap).await?;
            }
        }
        Ok(())
    })
}

async fn check_allocation_pressure(
    output: PathBuf,
    compact: bool,
    concurrent: bool,
    overlap: bool,
) -> Result<()> {
    let gc = ristretto_gc::GarbageCollector::with_config(
        ristretto_gc::Configuration::builder()
            .concurrent_publication(concurrent)
            .overlapping_generations(overlap)
            .old_collection_interval(if overlap { 4 } else { 0 })
            .independent_phases(overlap)
            .region_shards(if overlap { 16 } else { 1 })
            .compact_heap(compact)
            .cost_aware_relocation(compact)
            // This fixture needs the full ordinary cap for its 9 MiB source.
            // Reserved backing is exercised by the reference matrix above.
            .relocation_reserve_bytes(0)
            .diagnostics(true)
            .threads(4)
            .work_stealing(true)
            .root_storage(ristretto_gc::RootStorage::Contexts)
            .barrier_buffer_size(4)
            .local_allocation_buffers(true)
            .adaptive_workers(true)
            .independent_relocation(true)
            .independent_marking(true)
            .metadata_shards(16)
            .mark_batch_size(8)
            .max_heap_bytes(16 * 1024 * 1024)
            .allocation_threshold(1024 * 1024)
            .build(),
    );
    let config = ConfigurationBuilder::new()
        .java_version(JAVA_25_VERSION)
        .class_path(ClassPath::from(std::slice::from_ref(&output)))
        .garbage_collector(gc.clone())
        .build()?;
    let vm = VM::new(config).await?;
    assert_eq!(
        vm.invoke("Test", "catchOom()Z", &[] as &[Value]).await?,
        Some(Value::from(true))
    );
    gc.collect_full()?;
    assert!(gc.managed_bytes() < 16 * 1024 * 1024);
    let before_clone = gc.diagnostics();
    assert_eq!(
        vm.invoke("Test", "cloneOom()Z", &[] as &[Value]).await?,
        Some(Value::from(true))
    );
    let after_clone = gc.diagnostics();
    assert_eq!(
        after_clone.allocation_recovery.count, before_clone.allocation_recovery.count,
        "Object.clone ran a full recovery cycle on its requesting task"
    );
    assert!(
        after_clone.async_allocation_recovery.count > before_clone.async_allocation_recovery.count
    );
    assert_eq!(after_clone.allocation_waiters, 0);
    check_string_pressure(&vm).await?;
    check_runtime_object_pressure(&vm).await?;
    Ok(())
}

async fn check_string_pressure(vm: &Arc<VM>) -> Result<()> {
    let thread = Thread::new(&Arc::downgrade(vm), 1_000_000);
    thread.class("java.lang.String").await?;
    let gc = vm.garbage_collector();
    gc.collect_full()?;
    let source = Value::try_new_object(
        gc,
        ristretto_classloader::Reference::from(vec![7_i8; 9 * 1024 * 1024]),
    )?;
    let input = "a".repeat(9 * 1024 * 1024);
    let before = gc.diagnostics();
    assert!(matches!(
        input.to_object(&thread).await,
        Err(Error::JavaError(JavaError::OutOfMemoryError(_)))
    ));
    let after = gc.diagnostics();
    assert_eq!(
        after.allocation_recovery.count, before.allocation_recovery.count,
        "String conversion ran full recovery cycles on its requesting task"
    );
    assert!(after.async_allocation_recovery.count > before.async_allocation_recovery.count);
    assert_eq!(after.allocation_waiters, 0);
    assert!(source.as_byte_vec_ref()?.iter().all(|&byte| byte == 7));
    Ok(())
}

async fn check_runtime_object_pressure(vm: &Arc<VM>) -> Result<()> {
    use ristretto_gc::{Gc, Trace};
    struct LiveBytes(Vec<u8>);
    // SAFETY: this fixture contains no outgoing GC references.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for LiveBytes {
        fn trace(&self, _: &ristretto_gc::GarbageCollector) {}
        fn external_size(&self) -> usize {
            self.0.len()
        }
    }

    let thread = Thread::new(&Arc::downgrade(vm), 1_000_001);
    thread.class("java.lang.Object").await?;
    let gc = vm.garbage_collector();
    gc.collect_full()?;
    gc.collect_full()?;
    let limit = 16 * 1024 * 1024;
    let overhead = Gc::allocation_size(&LiveBytes(Vec::new()))?;
    let length = limit - gc.managed_bytes() - overhead;
    // A pinned host payload isolates logical exhaustion from arena fragmentation.
    let live = Gc::try_new_async(gc, LiveBytes(vec![7_u8; length])).await?;
    let before = gc.diagnostics();
    assert!(
        matches!(
            thread.object("java.lang.Object", "", &[] as &[Value]).await,
            Err(Error::JavaError(JavaError::OutOfMemoryError(_)))
        ),
        "runtime constructor bypassed the managed heap limit"
    );
    let after = gc.diagnostics();
    assert_eq!(
        after.allocation_recovery.count,
        before.allocation_recovery.count
    );
    assert!(after.async_allocation_recovery.count > before.async_allocation_recovery.count);
    assert_eq!(after.allocation_waiters, 0);
    assert!(gc.managed_bytes() <= limit);
    assert!(live.0.iter().all(|&byte| byte == 7));
    Ok(())
}
