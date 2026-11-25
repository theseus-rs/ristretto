//! Repeated complete graph collections. Construction, warmup, validation and
//! teardown are excluded; elapsed time includes all collection phases and waits.
#![expect(clippy::print_stdout, clippy::panic_in_result_fn)]
use ristretto_gc::{
    Configuration, GarbageCollector, Gc, GcRootGuard, Result, RootStorage, Trace, TraceProgress,
};
use std::sync::Arc;
use std::time::Instant;

struct Node {
    children: Vec<Gc<Node>>,
    value: usize,
}

// SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Node {
    fn root(&mut self) {
        self.children.root();
    }
    unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
        unsafe { Trace::unroot(&mut self.children, owner) };
    }
    fn trace_work(&self) -> usize {
        self.children.len().clamp(1, 32)
    }
    fn trace(&self, gc: &GarbageCollector) {
        self.children.trace(gc);
    }
    fn trace_step(&self, gc: &GarbageCollector, offset: usize, budget: usize) -> TraceProgress {
        self.children.trace_step(gc, offset, budget)
    }
    fn clear_step(&mut self, offset: usize, budget: usize) -> TraceProgress {
        self.children.clear_step(offset, budget)
    }
    fn external_size(&self) -> usize {
        self.children.capacity() * size_of::<Gc<Node>>()
    }
}

fn graph(gc: &Arc<GarbageCollector>, nodes: usize, fanout: usize) -> Result<GcRootGuard<Node>> {
    let mut roots: Vec<Option<GcRootGuard<Node>>> = (0..nodes).map(|_| None).collect();
    for index in (0..nodes).rev() {
        let start = index.saturating_mul(fanout).saturating_add(1);
        let end = start.saturating_add(fanout).min(nodes);
        let mut children = Vec::new();
        if let Some(slots) = roots.get_mut(start..end) {
            for slot in slots {
                if let Some(child) = slot.take() {
                    children.push(child.into_rooted());
                }
            }
        }
        let root = Gc::try_new(
            gc,
            Node {
                children,
                value: index + 1,
            },
        )?;
        if let Some(slot) = roots.get_mut(index) {
            *slot = Some(root);
        }
    }
    roots
        .first_mut()
        .and_then(Option::take)
        .ok_or_else(|| ristretto_gc::Error::RootError("missing graph root".to_string()))
}

fn checksum(root: &Node) -> usize {
    let mut pending = vec![root];
    let mut sum = 0;
    while let Some(node) = pending.pop() {
        sum += node.value;
        pending.extend(node.children.iter().map(std::ops::Deref::deref));
    }
    sum
}

#[expect(
    clippy::too_many_lines,
    reason = "shared timing and accounting across collection shapes"
)]
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let shape = args.get(1).map_or("tree", String::as_str);
    let policy = args.get(2).map_or("counted", String::as_str);
    let nodes = args
        .get(3)
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(8191);
    let cycles = args
        .get(4)
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(32);
    let workers = args
        .get(5)
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let diagnostics = args.get(6).is_some_and(|v| v == "true");
    assert!(["tree", "wide", "barrier", "mixed"].contains(&shape));
    assert!(["gated", "counted"].contains(&policy));
    assert!((2..=65535).contains(&nodes));
    assert!((1..=10000).contains(&cycles));
    assert!(workers <= 16);
    if cfg!(target_family = "wasm") {
        assert_eq!(workers, 0);
    }
    let buffered = shape == "barrier";
    if buffered {
        // No consumer may pre-mark a target before its required record is queued.
        assert_eq!(workers, 0);
    }
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .threads(workers)
            .worker_yield_time_us(0)
            .allocation_threshold(usize::MAX)
            .max_heap_bytes(256 * 1024 * 1024)
            .incremental_step_size(if buffered { 1 } else { 256 })
            .max_pause_time_us(0)
            .root_storage(RootStorage::Slots)
            .barrier_buffer_size(if buffered { 32 } else { 0 })
            .metadata_shards(16)
            .concurrent_publication(policy == "counted")
            .independent_marking(true)
            .independent_phases(true)
            .work_stealing(true)
            .mark_batch_size(8)
            .diagnostics(diagnostics)
            .build(),
    );
    let root = graph(
        &gc,
        nodes,
        if matches!(shape, "tree" | "mixed") {
            8
        } else {
            nodes
        },
    )?;
    let contexts: Vec<_> = if buffered {
        root.children
            .chunks(32)
            .map(|_| gc.register_context())
            .collect()
    } else {
        Vec::new()
    };
    let collect = || -> Result<()> {
        if buffered {
            gc.collect();
            assert!(!gc.step());
            // The single setup unit cannot trace the wide root's children.
            // Each target is distinct and every context fits without overflow.
            for (context, children) in contexts.iter().zip(root.children.chunks(32)) {
                context.with(|| {
                    for child in children {
                        gc.write_barrier(child);
                    }
                });
            }
            while !gc.step() {}
            Ok(())
        } else {
            gc.collect_full()
        }
    };
    let expected_checksum = nodes * (nodes + 1) / 2;
    assert_eq!(checksum(&root), expected_checksum);
    if workers != 0 {
        gc.start();
    }
    assert_eq!(gc.registered_workers(), workers);
    for _ in 0..3 {
        collect()?;
    }
    let before = gc.statistics()?;
    let trace_before = gc.diagnostics().trace_tasks;
    let barriers_before = gc.diagnostics();
    assert_eq!(before.live_objects, nodes);
    // A mixed cycle starts with the same reachable tree and a fresh cohort
    // of unreachable scalars. Time only the collection that must reclaim it.
    let garbage = if shape == "mixed" {
        (nodes / 4).max(1)
    } else {
        0
    };
    let mut elapsed_ns = 0;
    for cycle in 1..=cycles {
        for _ in 0..garbage {
            drop(Gc::try_new(&gc, 17_u64)?);
        }
        let start = Instant::now();
        collect()?;
        if garbage != 0 {
            // A worker may finish a retired payload after the full-cycle
            // ticket. Include that work and its accounting in this cycle.
            let target = before.objects_swept + cycle * garbage;
            while gc.statistics()?.objects_swept < target {
                assert!(
                    start.elapsed() < std::time::Duration::from_secs(10),
                    "reclamation deadline"
                );
                gc.step();
                #[cfg(not(target_family = "wasm"))]
                std::thread::yield_now();
            }
        }
        elapsed_ns += start.elapsed().as_nanos();
    }
    let after = gc.statistics()?;
    let trace_tasks = gc.diagnostics().trace_tasks - trace_before;
    let barriers_after = gc.diagnostics();
    let barrier_buffered = barriers_after.barrier_buffered - barriers_before.barrier_buffered;
    let barrier_flushed = barriers_after.barrier_flushed - barriers_before.barrier_flushed;
    let barrier_overflows = barriers_after.barrier_overflows - barriers_before.barrier_overflows;
    let expected_barriers = if diagnostics && buffered {
        (nodes - 1) * cycles
    } else {
        0
    };
    assert_eq!(barrier_buffered, expected_barriers as u64);
    assert_eq!(barrier_flushed, expected_barriers as u64);
    assert_eq!(barrier_overflows, 0);
    let swept = after.objects_swept - before.objects_swept;
    let freed = after.managed_bytes_freed - before.managed_bytes_freed;
    assert_eq!(swept, cycles * garbage);
    assert_eq!(freed, swept * Gc::allocation_size(&17_u64)?);
    let marked = after.objects_marked - before.objects_marked;
    let completed = after.full_collections - before.full_collections;
    assert_eq!(completed, cycles);
    assert_eq!(marked, nodes * cycles);
    assert_eq!(after.live_objects, nodes);
    assert_eq!(checksum(&root), expected_checksum);
    assert_eq!(gc.registered_workers(), workers);
    drop((root, contexts));
    gc.stop()?;
    let final_stats = gc.statistics()?;
    assert_eq!(gc.registered_workers(), 0);
    assert_eq!(final_stats.live_objects, 0);
    assert_eq!(final_stats.bytes_managed, 0);
    println!(
        "{{\"schema\":1,\"shape\":\"{shape}\",\"policy\":\"{policy}\",\"nodes\":{nodes},\"cycles\":{cycles},\"workers\":{workers},\"diagnostics\":{diagnostics},\"elapsed_ns\":{elapsed_ns},\"swept\":{swept},\"freed\":{freed},\"marked\":{marked},\"completed\":{completed},\"trace_tasks\":{trace_tasks},\"barrier_buffered\":{barrier_buffered},\"barrier_flushed\":{barrier_flushed},\"barrier_overflows\":{barrier_overflows},\"checksum\":{expected_checksum},\"teardown_live\":0,\"teardown_bytes\":0}}"
    );
    Ok(())
}
