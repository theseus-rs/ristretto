//! Runtime assertions for wasm32-unknown-unknown, without browser APIs or a host clock.
#![expect(
    clippy::panic_in_result_fn,
    reason = "exported smoke test asserts collector invariants"
)]
use ristretto_gc::sync::RwLock;
use ristretto_gc::{
    Configuration, GarbageCollector, Gc, GcWeak, HeapBuffer, ReferenceKind, Result,
};
use std::future::Future;
use std::task::{Context, Poll, Waker};

/// Runs on a bare WebAssembly host; an assertion failure traps the invocation.
#[expect(unsafe_code, reason = "export a named WebAssembly test entry point")]
#[unsafe(no_mangle)]
/// # Panics
/// Panics when a collector invariant fails.
pub extern "C" fn gc_smoke() {
    let result = run();
    assert!(result.is_ok(), "cooperative collector smoke test failed");
}

fn run() -> Result<()> {
    context_smoke()?;
    reference_construction_smoke()?;
    locked_leaf_smoke()?;
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .metadata_shards(16)
            .region_shards(16)
            .concurrent_publication(true)
            .overlapping_generations(true)
            .old_collection_interval(4)
            .independent_phases(true)
            .work_stealing(true)
            .independent_marking(true)
            .mark_batch_size(8)
            .diagnostics(true)
            .predictive_collection(true)
            .root_storage(ristretto_gc::RootStorage::Slots)
            .incremental_step_size(4)
            .allocation_threshold(1024)
            .max_heap_bytes(4 * 1024 * 1024)
            .build(),
    );
    gc.start();
    #[cfg(target_family = "wasm")]
    assert_eq!(gc.registered_workers(), 0);
    let root = Gc::try_new(&gc, vec![7_u8; 1024 * 1024])?.into_rooted();
    let weak = GcWeak::new(&root, ReferenceKind::Weak, 0);
    gc.collect_full()?;
    assert_eq!(root.first(), Some(&7));
    assert!(gc.managed_bytes() >= 1024 * 1024);
    drop(root);
    gc.collect_full()?;
    assert!(weak.upgrade().is_none());
    assert_eq!(gc.managed_bytes(), 0);
    assert!(Gc::try_new(&gc, vec![0_u8; 4 * 1024 * 1024]).is_err());
    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    assert_clock_free_statistics(&gc)?;
    // Exercise the same old-to-young barrier and weak decisions without OS threads.
    let parent = Gc::try_new(&gc, RwLock::new(Vec::<Gc<Vec<u8>>>::new()))?.into_rooted();
    gc.collect_full()?;
    gc.collect_full()?;
    let child = Gc::try_new(&gc, vec![9_u8; 128])?.into_rooted();
    let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
    parent.write().push(child);
    gc.collect_young();
    while !gc.step() {}
    assert!(weak.upgrade().is_some());
    parent.write().clear();
    gc.collect_young();
    while !gc.step() {}
    assert!(weak.upgrade().is_none());
    drop(parent);
    gc.stop()?;
    assert_eq!(gc.registered_workers(), 0);
    assert_eq!(gc.root_statistics().pages, 0);
    let compact = GarbageCollector::with_config(
        Configuration::builder()
            .metadata_shards(16)
            .region_shards(16)
            .concurrent_publication(true)
            .overlapping_generations(true)
            .old_collection_interval(4)
            .independent_phases(true)
            .work_stealing(true)
            .independent_marking(true)
            .mark_batch_size(8)
            .compact_heap(true)
            .cost_aware_relocation(true)
            .relocation_reserve_bytes(64 * 1024)
            .region_layout(4096, 128)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let mut live = Vec::new();
    for i in 0..32_u64 {
        let values: HeapBuffer<_> = vec![i; 64].into();
        live.push(Some(Gc::try_new(&compact, RwLock::new(values))?));
    }
    for item in live.iter_mut().step_by(2) {
        *item = None;
    }
    for _ in 0..4 {
        compact.collect_full()?;
    }
    if let Some(heap) = compact.region_heap() {
        assert_eq!(heap.statistics().regions, 2);
        assert!(heap.statistics().relocations > 0);
    }
    for (i, item) in live.iter().enumerate() {
        if let Some(item) = item {
            assert_eq!(item.read().first(), Some(&(i as u64)));
        }
    }
    drop(live);
    compact.collect_full()?;
    assert_eq!(compact.managed_bytes(), 0);
    recovery()
}

fn locked_leaf_smoke() -> Result<()> {
    for independent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(true)
                .independent_phases(independent)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let root = Gc::try_new(&gc, RwLock::new(42_u64))?;
        let mut guard = root.write();
        for cycle in 1..=3 {
            drop(Gc::try_new(&gc, 7_u64)?);
            gc.collect();
            for _ in 0..10_000 {
                if gc.step() {
                    break;
                }
            }
            assert_eq!(gc.statistics()?.full_collections, cycle);
            assert_eq!(gc.statistics()?.live_objects, 1);
        }
        *guard = 43;
        drop(guard);
        assert_eq!(*root.read(), 43);
        drop(root);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

fn reference_construction_smoke() -> Result<()> {
    struct Reference(GcWeak<u64>);
    // SAFETY: the only GC link is conditional; its slot handles tracing and owner binding.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl ristretto_gc::Trace for Reference {
        fn trace(&self, _: &GarbageCollector) {}
        #[expect(
            clippy::expect_used,
            reason = "exported smoke test asserts hook invariants"
        )]
        unsafe fn attach(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
            let gc = owner.collector();
            let address = owner.address();
            self.0.set_owner(address);
            let gc = gc.upgrade().expect("construction collector");
            for _ in 0..3 {
                gc.collect_full().expect("collection during attachment");
                assert_eq!(self.0.upgrade().as_deref(), Some(&42));
                assert_eq!(gc.statistics().expect("statistics").live_objects, 1);
                assert_eq!(gc.diagnostics().active_publications, 0);
            }
        }
    }
    for concurrent in [false, true] {
        for kind in [
            ReferenceKind::Soft,
            ReferenceKind::Weak,
            ReferenceKind::Final,
            ReferenceKind::Phantom,
            ReferenceKind::PhantomRetained,
        ] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .independent_phases(true)
                    .diagnostics(true)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let target = Gc::try_new(&gc, 42_u64)?;
            let weak = GcWeak::new(&target, kind, 0);
            drop(target);
            drop(Gc::try_new(&gc, 7_u64)?);
            let owner = Gc::try_new(&gc, Reference(weak))?;
            drop(owner);
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
            assert_eq!(gc.root_statistics().compatibility_roots, 0);
        }
    }
    Ok(())
}

fn recovery() -> Result<()> {
    // Force pressure recovery and multi-poll initialization without a runtime or clock.
    let cooperative = GarbageCollector::with_config(
        Configuration::builder()
            .metadata_shards(16)
            .region_shards(16)
            .concurrent_publication(true)
            .overlapping_generations(true)
            .old_collection_interval(4)
            .independent_phases(true)
            .work_stealing(true)
            .independent_marking(true)
            .mark_batch_size(8)
            .diagnostics(true)
            .compact_heap(true)
            .cost_aware_relocation(true)
            .relocation_reserve_bytes(64 * 1024)
            .max_heap_bytes(512 * 1024)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .build(),
    );
    finish(async {
        for _ in 0..12 {
            let buffer =
                HeapBuffer::try_with_fn_in_async(&cooperative, 128 * 1024, |_| Ok(9_u8)).await?;
            let root = Gc::try_new_async(&cooperative, RwLock::new(buffer)).await?;
            assert_eq!(root.read().last(), Some(&9));
        }
        cooperative.collect_full_async().await
    })?;
    assert_eq!(cooperative.managed_bytes(), 0);
    assert_eq!(cooperative.diagnostics().allocation_recovery.count, 0);
    assert!(cooperative.diagnostics().async_assists > 0);
    assert!(cooperative.diagnostics().async_allocation_recovery.count > 0);
    assert_eq!(cooperative.diagnostics().active_payload_guards, 0);
    assert!(cooperative.diagnostics().payload_guard_holds.count > 0);
    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    assert_eq!(cooperative.diagnostics().payload_guard_holds.total_ns, 0);
    Ok(())
}

fn finish<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(result) = future.as_mut().poll(&mut context) {
            return result;
        }
    }
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
fn assert_clock_free_statistics(gc: &GarbageCollector) -> Result<()> {
    assert!(gc.statistics()?.max_step_duration.is_none());
    let diagnostics = gc.diagnostics();
    assert!(!diagnostics.clock_available);
    assert!(diagnostics.caller_steps.count > 0);
    assert_eq!(diagnostics.caller_steps.total_ns, 0);
    let scheduling = gc.scheduling_statistics();
    assert!(scheduling.enabled);
    assert!(scheduling.samples > 0);
    assert!(!scheduling.clock_available);
    assert_eq!(scheduling.allocation_bytes_per_second, None);
    assert_eq!(scheduling.reclamation_latency_ns, None);
    Ok(())
}

fn context_smoke() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .root_storage(ristretto_gc::RootStorage::Contexts)
            .barrier_buffer_size(2)
            .diagnostics(true)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let context = gc.register_context();
    let root = context.with(|| Gc::try_new(&gc, 42_u64))?;
    gc.collect();
    gc.step();
    context.with(|| drop(root));
    drop(context);
    assert!(gc.diagnostics().pending_barrier_records > 0);
    while !gc.step() {}
    assert_eq!(gc.statistics()?.live_objects, 1);
    gc.stop()?;
    assert_eq!(gc.statistics()?.live_objects, 0);
    assert_eq!(gc.diagnostics().execution_contexts, 0);
    assert_eq!(gc.diagnostics().pending_barrier_records, 0);
    assert_eq!(gc.root_statistics().pages, 0);
    Ok(())
}
