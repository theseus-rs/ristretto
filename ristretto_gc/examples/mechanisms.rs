//! Collector-only ablations. Collection between batches is excluded from operation
//! time and included in wall time. These are not scheduled-arrival latency tests.
#![expect(clippy::print_stdout, clippy::panic_in_result_fn)]
use ristretto_gc::{Configuration, GarbageCollector, Gc, Result, RootStorage, Trace};
use std::hint::black_box;
use std::time::{Duration, Instant};

#[path = "support/construction_batches.rs"]
mod construction_batches;

struct Payload([u64; 4]);
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Payload {
    fn trace(&self, _: &GarbageCollector) {}
}

struct DeclaredLeaf([u64; 4]);
// SAFETY: the fixture stores only primitive integers and has no tracing action.
#[expect(unsafe_code, reason = "explicit reference-free construction fixture")]
unsafe impl Trace for DeclaredLeaf {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let workload = args.get(1).map_or("allocation", String::as_str);
    let policy = args.get(2).map_or("candidate", String::as_str);
    let operations = args
        .get(3)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(100_000);
    let diagnostics = args.get(4).is_some_and(|s| s == "true");
    run(workload, policy, operations, diagnostics)
}

#[expect(
    clippy::too_many_lines,
    reason = "keep identical timing and cleanup boundaries across mechanism cases"
)]
fn run(workload: &str, policy: &str, operations: u64, diagnostics: bool) -> Result<()> {
    validate(workload, policy, operations);
    let gc = GarbageCollector::with_config(configuration(policy, diagnostics));
    let context = gc.register_context();
    let resident = Gc::try_new(&gc, 42_u64)?;
    // Borrow the guard's heap edge so root acquisition measures a registration.
    let edge: &Gc<u64> = &resident;
    let working_set = match workload {
        "barrier4" => 4,
        "barrier16" => 16,
        "barrier64" => 64,
        _ => 0,
    };
    let edges = (0..working_set)
        .map(|_| Gc::try_new(&gc, 42_u64).map(ristretto_gc::GcRootGuard::into_rooted))
        .collect::<Result<Vec<_>>>()?;
    let mut cursor = 0_usize;
    let shared = ["clones", "unroot"]
        .contains(&workload)
        .then(|| context.with(|| edge.rooted_clone()));
    let weak = (workload == "weak")
        .then(|| ristretto_gc::GcWeak::new(edge, ristretto_gc::ReferenceKind::Weak, 0));
    if shared.is_some() && ["contexts", "local", "candidate"].contains(&policy) {
        assert_eq!(context.root_statistics().slot_roots, 1);
    }
    let owner = Gc::try_new(&gc, ristretto_gc::cell::RwLock::new(None::<Gc<u64>>))?;
    let leaf = (workload == "leaf_write")
        .then(|| Gc::try_new(&gc, ristretto_gc::cell::RwLock::new(42_u64)))
        .transpose()?;
    let nested = (workload == "nested_write")
        .then(|| {
            Gc::try_new(
                &gc,
                vec![ristretto_gc::cell::RwLock::new(Some(edge.rooted_clone()))],
            )
        })
        .transpose()?;
    let mut elapsed = Duration::ZERO;
    let mut completed = 0;
    let mut checksum = 0_u64;
    let wall = Instant::now();
    context.with(|| -> Result<()> {
        while completed < operations {
            // Keep strong tracing active throughout a barrier batch. No background
            // workers: this lane isolates publication from concurrent drain costs.
            if working_set != 0
                || ["barrier", "overwrite", "leaf_write", "nested_write"].contains(&workload)
            {
                gc.collect();
                assert!(!gc.step());
            }
            let count = (operations - completed).min(2048);
            // Queue preparation/collection is outside operation timing. Retain
            // every weak slot until its owner has been transferred to the caller.
            let mut pending_slots = Vec::new();
            if workload == "pending_transfer" {
                let mut owners = Vec::new();
                for _ in 0..count {
                    let target = Gc::try_new(&gc, 0_u64)?;
                    let reference = Gc::try_new(&gc, 42_u64)?;
                    pending_slots.push(ristretto_gc::GcWeak::new(
                        &target,
                        ristretto_gc::ReferenceKind::Weak,
                        reference.as_ptr() as usize,
                    ));
                    owners.push(reference);
                }
                gc.collect_full()?;
                drop(owners);
            }
            let start = Instant::now();
            for _ in 0..count {
                match workload {
                    "construction_batches" => {
                        let root =
                            Gc::try_new(&gc, construction_batches::ConstructionBatch::new())?;
                        checksum += black_box(root.value());
                    }
                    "allocation" => {
                        let root = Gc::try_new(&gc, Payload(black_box([42_u64; 4])))?;
                        checksum += black_box(root.0[0]);
                    }
                    "leaf_allocation" => {
                        let root = Gc::try_new(&gc, DeclaredLeaf(black_box([42_u64; 4])))?;
                        checksum += black_box(root.0[0]);
                    }
                    "owning_allocation" => {
                        let root = Gc::try_new(&gc, Payload(black_box([42_u64; 4])))?.into_rooted();
                        checksum += black_box(root.0[0]);
                    }
                    "cell_allocation" => {
                        let root = Gc::try_new(
                            &gc,
                            ristretto_gc::cell::RwLock::new(Some(black_box(edge).rooted_clone())),
                        )?;
                        if let Some(child) = &*root.read() {
                            checksum += black_box(**child);
                        }
                    }
                    "roots" => {
                        let root = black_box(edge).rooted_clone();
                        checksum += black_box(*root);
                    }
                    "clones" => {
                        if let Some(root) = &shared {
                            let clone = black_box(root).clone();
                            checksum += black_box(*clone);
                        }
                    }
                    "guard_clones" => {
                        let clone = black_box(&resident).clone();
                        checksum += black_box(**clone);
                    }
                    "edge_clones" => {
                        let clone = black_box(edge).clone();
                        checksum += black_box(*clone);
                    }
                    "unroot" => {
                        if let Some(root) = &shared {
                            let mut clone = black_box(root).clone();
                            // SAFETY: The resident and shared owning handles protect every borrowed access.
                            #[expect(unsafe_code, reason = "explicit non-owning edge fixture")]
                            unsafe {
                                clone.unroot();
                            };
                            checksum += black_box(*clone);
                        }
                    }
                    "handles" => {
                        let root =
                            Gc::<u64>::from_handle_rooted(&gc, black_box(edge.as_handle_i64()))?;
                        checksum += black_box(*root);
                    }
                    "pending_poll" => {
                        checksum += 42
                            * u64::from(black_box(gc.take_pending_reference::<u64>()?).is_none());
                    }
                    "pending_transfer" => {
                        let root = gc.take_pending_reference::<u64>()?.ok_or_else(|| {
                            ristretto_gc::Error::InvalidPointer(
                                "pending reference disappeared".into(),
                            )
                        })?;
                        checksum += black_box(*root);
                    }
                    "weak" => {
                        let Some(root) = weak.as_ref().and_then(|weak| black_box(weak).upgrade())
                        else {
                            return Err(ristretto_gc::Error::InvalidPointer(
                                "live weak target disappeared".into(),
                            ));
                        };
                        checksum += black_box(*root);
                    }
                    "barrier" => {
                        gc.write_barrier(black_box(edge));
                        checksum += black_box(**edge);
                    }
                    "barrier4" | "barrier16" | "barrier64" => {
                        let target = edges.get(cursor & (working_set - 1)).ok_or_else(|| {
                            ristretto_gc::Error::InvalidPointer("barrier working set".into())
                        })?;
                        gc.write_barrier(black_box(target));
                        checksum += black_box(**target);
                        cursor = cursor.wrapping_add(1);
                    }
                    "overwrite" => {
                        *owner.write() = Some(black_box(edge).clone());
                        checksum += black_box(**edge);
                    }
                    "leaf_write" => {
                        if let Some(leaf) = &leaf {
                            let mut value = leaf.write();
                            *value = black_box(42);
                            checksum += black_box(*value);
                        }
                    }
                    "nested_write" => {
                        if let Some(cell) = nested.as_ref().and_then(|cells| cells.first()) {
                            let mut value = cell.write();
                            if let Some(child) = value.take() {
                                checksum += black_box(*child);
                                *value = Some(child);
                            }
                        }
                    }
                    _ => checksum += black_box(42_u64),
                }
            }
            elapsed += start.elapsed();
            drop(pending_slots);
            completed += count;
            // Reclaim between batches, including floating garbage from SATB.
            context.flush();
            gc.collect_full()?;
            gc.collect_full()?;
        }
        Ok(())
    })?;
    let wall_ns = wall.elapsed().as_nanos();
    assert_eq!(checksum, operations * 42);
    let d = gc.diagnostics();
    drop((edges, leaf, nested, weak, shared, owner, resident, context));
    gc.stop()?;
    assert_eq!(gc.statistics()?.live_objects, 0);
    assert_eq!(gc.managed_bytes(), 0);
    println!(
        "{{\"workload\":\"{workload}\",\"policy\":\"{policy}\",\"diagnostics\":{diagnostics},\"operations\":{operations},\"operation_ns\":{},\"wall_ns\":{wall_ns},\"checksum\":{checksum},\"barrier_buffered\":{},\"barrier_flushed\":{},\"barrier_overflows\":{},\"local_hits\":{},\"local_refills\":{},\"barrier_coalesced\":{},\"publication_ns\":{},\"root_acquire_ns\":{},\"root_release_ns\":{},\"barrier_record_ns\":{},\"barrier_drain_ns\":{},\"teardown_live\":0}}",
        elapsed.as_nanos(),
        d.barrier_buffered,
        d.barrier_flushed,
        d.barrier_overflows,
        d.local_allocation_hits,
        d.local_allocation_refills,
        d.barrier_coalesced,
        d.descriptor_publication.total_ns,
        d.root_acquire.total_ns,
        d.root_release.total_ns,
        d.barrier_record.total_ns,
        d.barrier_drain.total_ns,
    );
    Ok(())
}

fn configuration(policy: &str, diagnostics: bool) -> Configuration {
    let contexts = ["contexts", "local", "candidate"].contains(&policy);
    Configuration::builder()
        .allocation_threshold(usize::MAX)
        .max_heap_bytes(64 * 1024 * 1024)
        .incremental_step_size(64)
        .max_pause_time_us(0)
        .concurrent_publication(["counted", "candidate"].contains(&policy))
        .root_storage(if contexts {
            RootStorage::Contexts
        } else {
            RootStorage::Tree
        })
        .barrier_buffer_size(if contexts { 32 } else { 0 })
        .local_allocation_buffers(["local", "candidate"].contains(&policy))
        .diagnostics(diagnostics)
        .build()
}

fn validate(workload: &str, policy: &str, operations: u64) {
    assert!((1..=100_000_000).contains(&operations));
    assert!(
        [
            "control",
            "allocation",
            "leaf_allocation",
            "owning_allocation",
            "cell_allocation",
            "construction_batches",
            "nested_write",
            "roots",
            "clones",
            "guard_clones",
            "pending_poll",
            "pending_transfer",
            "edge_clones",
            "unroot",
            "handles",
            "weak",
            "barrier",
            "barrier4",
            "barrier16",
            "barrier64",
            "overwrite",
            "leaf_write"
        ]
        .contains(&workload)
    );
    assert!(["baseline", "counted", "contexts", "local", "candidate"].contains(&policy));
}
