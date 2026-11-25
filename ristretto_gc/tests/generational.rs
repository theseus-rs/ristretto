//! Collector invariants exercised by the same tests on native and cooperative hosts.
#![expect(clippy::panic_in_result_fn)]
#![cfg_attr(not(target_family = "wasm"), expect(clippy::unwrap_in_result))]
use ristretto_gc::sync::RwLock;
use ristretto_gc::{Configuration, GarbageCollector, Gc, Result, Trace};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
struct Node {
    edges: Vec<Gc<RwLock<Node>>>,
    drops: Arc<AtomicUsize>,
}
// SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Node {
    fn root(&mut self) {
        self.edges.root();
    }
    fn trace(&self, gc: &GarbageCollector) {
        self.edges.trace(gc);
    }
    unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
        unsafe { Trace::unroot(&mut self.edges, owner) };
    }
}
impl Drop for Node {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}
fn collector(concurrent: bool, overlap: bool) -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(concurrent)
            .overlapping_generations(overlap)
            .old_collection_interval(if overlap { usize::MAX } else { 0 })
            .independent_phases(overlap)
            .region_shards(if overlap { 16 } else { 1 })
            .metadata_shards(16)
            .work_stealing(true)
            .independent_marking(true)
            .mark_batch_size(8)
            .allocation_threshold(usize::MAX)
            .full_collection_interval(usize::MAX)
            .incremental_step_size(8)
            .max_pause_time_us(0)
            .build(),
    )
}
fn young(gc: &GarbageCollector) {
    gc.collect_young();
    while !gc.step() {}
}

#[test]
fn periodic_full_collection_does_not_need_another_allocation_request() -> Result<()> {
    for (concurrent, overlap) in [(false, false), (true, false), (false, true), (true, true)] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .overlapping_generations(overlap)
                .old_collection_interval(if overlap { usize::MAX } else { 0 })
                .independent_phases(overlap)
                .region_shards(if overlap { 16 } else { 1 })
                .metadata_shards(16)
                .work_stealing(true)
                .independent_marking(true)
                .mark_batch_size(8)
                .allocation_threshold(usize::MAX)
                .full_collection_interval(2)
                .build(),
        );
        young(&gc);
        assert_eq!(gc.statistics()?.full_collections, 0);
        // Coalesced requests still represent one additional young cycle.
        gc.collect_young();
        gc.collect_young();
        while !gc.step() {}
        assert_eq!(gc.statistics()?.young_collections, 2);
        assert_eq!(gc.statistics()?.full_collections, 1);
        young(&gc);
        assert_eq!(gc.statistics()?.full_collections, 1);
    }
    Ok(())
}

#[test]
fn dead_before_first_collection_is_reclaimed() -> Result<()> {
    for (concurrent, overlap) in [(false, false), (true, false), (false, true), (true, true)] {
        let gc = collector(concurrent, overlap);
        drop(Gc::new(&gc, vec![0_u8; 1024 * 1024]));
        assert!(gc.managed_bytes() >= 1024 * 1024);
        gc.collect_full()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn live_bytes_are_not_allocation_debt() -> Result<()> {
    for (concurrent, overlap) in [(false, false), (true, false), (false, true), (true, true)] {
        let gc = collector(concurrent, overlap);
        let root = Gc::new(&gc, vec![1_u8; 1024 * 1024]).into_rooted();
        let bytes = gc.managed_bytes();
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), bytes);
        assert_eq!(gc.statistics()?.allocation_debt, 0);
        assert_eq!(root.len(), 1024 * 1024);
        drop(root);
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn stop_preserves_escaped_owning_handles() -> Result<()> {
    for (concurrent, overlap) in [(false, false), (true, false), (false, true), (true, true)] {
        let gc = collector(concurrent, overlap);
        let root = Gc::new(&gc, 42_u64).into_rooted();
        gc.stop()?;
        assert_eq!(*root, 42);
        drop(gc);
        assert_eq!(*root, 42);
    }
    Ok(())
}

#[test]
fn managed_cycles_are_edges_and_can_be_reclaimed() -> Result<()> {
    for (concurrent, overlap) in [(false, false), (true, false), (false, true), (true, true)] {
        let gc = collector(concurrent, overlap);
        let drops = Arc::new(AtomicUsize::new(0));
        let a = Gc::new(
            &gc,
            RwLock::new(Node {
                drops: drops.clone(),
                edges: Vec::new(),
            }),
        )
        .into_rooted();
        let b = Gc::new(
            &gc,
            RwLock::new(Node {
                drops: drops.clone(),
                edges: Vec::new(),
            }),
        )
        .into_rooted();
        a.write().edges.push(b.rooted_clone());
        b.write().edges.push(a.rooted_clone());
        gc.collect_full()?;
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        drop(a);
        drop(b);
        gc.collect_full()?;
        assert_eq!(drops.load(Ordering::Relaxed), 2);
    }
    Ok(())
}

#[test]
fn clean_old_objects_are_not_retraced_and_mutations_remember_them() -> Result<()> {
    for (concurrent, overlap) in [(false, false), (true, false), (false, true), (true, true)] {
        let gc = collector(concurrent, overlap);
        let old = Gc::new(&gc, RwLock::new(Node::default())).into_rooted();
        gc.collect_full()?;
        young(&gc); // Promote.
        young(&gc); // Remove the initial conservative remembered entry.
        let marked = gc.statistics()?.objects_marked;
        for _ in 0..10 {
            young(&gc);
        }
        assert_eq!(gc.statistics()?.objects_marked, marked);

        let drops = Arc::new(AtomicUsize::new(0));
        let child = Gc::new(
            &gc,
            RwLock::new(Node {
                drops: drops.clone(),
                edges: Vec::new(),
            }),
        )
        .into_rooted();
        old.write().edges.push(child.rooted_clone());
        drop(child);
        young(&gc);
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        old.write().edges.clear();
        young(&gc);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
    }
    Ok(())
}

#[test]
fn integer_handles_validate_both_address_and_allocation_type() -> Result<()> {
    for (concurrent, overlap) in [(false, false), (true, false), (false, true), (true, true)] {
        let gc = collector(concurrent, overlap);
        let original = Gc::new(&gc, 42_u64).into_rooted();
        assert!(Gc::<String>::from_raw_rooted(&gc, original.as_ptr_i64()).is_err());
        assert!(Gc::<u64>::from_raw_rooted(&gc, 1).is_err());
        let copy = Gc::<u64>::from_raw_rooted(&gc, original.as_ptr_i64())?;
        drop(original);
        gc.collect_full()?;
        assert_eq!(*copy, 42);
    }
    Ok(())
}

#[test]
fn object_identities_reject_expired_and_foreign_heap_handles() -> Result<()> {
    for (concurrent, overlap) in [(false, false), (true, false), (false, true), (true, true)] {
        let gc = collector(concurrent, overlap);
        let other_heap = collector(concurrent, overlap);
        let original = Gc::new(&gc, 42_u64);
        let id = original.as_handle_i64();
        assert!(Gc::<u64>::from_handle_rooted(&other_heap, id).is_err());
        assert!(Gc::<String>::from_handle_rooted(&gc, id).is_err());
        let copy = Gc::<u64>::from_handle_rooted(&gc, id)?;
        drop(original);
        gc.collect_full()?;
        assert_eq!(*copy, 42);
        drop(copy);
        gc.collect_full()?;
        let replacement = Gc::new(&gc, 99_u64);
        assert_ne!(replacement.as_handle_i64(), id);
        assert!(Gc::<u64>::from_handle_rooted(&gc, id).is_err());
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn native_worker_collects_while_mutators_run() -> Result<()> {
    for (concurrent, overlap) in [(false, false), (true, false), (false, true), (true, true)] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .overlapping_generations(overlap)
                .old_collection_interval(if overlap { usize::MAX } else { 0 })
                .independent_phases(overlap)
                .region_shards(if overlap { 16 } else { 1 })
                .metadata_shards(16)
                .work_stealing(true)
                .independent_marking(true)
                .mark_batch_size(8)
                .allocation_threshold(4096)
                .incremental_step_size(32)
                .build(),
        );
        gc.start();
        let mut workers = Vec::new();
        for _ in 0..4 {
            let gc = gc.clone();
            workers.push(std::thread::spawn(move || {
                for i in 0..2000_u64 {
                    let root = Gc::new(&gc, vec![i; 16]).into_rooted();
                    std::thread::yield_now();
                    assert_eq!(root.first(), Some(&i));
                }
            }));
        }
        for worker in workers {
            worker.join().unwrap();
        }
        gc.collect_full()?;
        gc.collect_full()?; // SATB may retain objects dropped during the preceding cycle.
        assert_eq!(
            gc.statistics()?.live_objects,
            0,
            "concurrent={concurrent} overlap={overlap} stats={:?} roots={:?}",
            gc.statistics()?,
            gc.root_statistics()
        );
        assert!(gc.statistics()?.collections_completed > 1);
        gc.stop()?;
    }
    Ok(())
}

#[test]
fn heap_limit_counts_payload_and_failed_allocations_do_not_charge_bytes() -> Result<()> {
    for (concurrent, overlap) in [(false, false), (true, false), (false, true), (true, true)] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .overlapping_generations(overlap)
                .old_collection_interval(if overlap { usize::MAX } else { 0 })
                .independent_phases(overlap)
                .region_shards(if overlap { 16 } else { 1 })
                .metadata_shards(16)
                .work_stealing(true)
                .independent_marking(true)
                .mark_batch_size(8)
                .max_heap_bytes(2048)
                .build(),
        );
        let root = Gc::try_new(&gc, vec![0_u8; 1024])?.into_rooted();
        let before = gc.managed_bytes();
        assert!(matches!(
            Gc::try_new(&gc, vec![0_u8; 1024]),
            Err(ristretto_gc::Error::OutOfMemory)
        ));
        assert_eq!(gc.managed_bytes(), before);
        drop(root);
        gc.make_room(2048)?;
        let root = Gc::try_new(&gc, vec![0_u8; 1024])?;
        assert_eq!(root.len(), 1024);
    }
    Ok(())
}

#[test]
fn zero_sized_objects_have_distinct_identities() -> Result<()> {
    for (concurrent, overlap) in [(false, false), (true, false), (false, true), (true, true)] {
        struct Empty;
        // SAFETY: this fixture contains no outgoing GC references.
        #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
        unsafe impl Trace for Empty {
            fn trace(&self, _: &GarbageCollector) {}
        }
        let gc = collector(concurrent, overlap);
        let first = Gc::new(&gc, Empty);
        let second = Gc::new(&gc, Empty);
        assert!(!Gc::ptr_eq(&first, &second));
        gc.collect_full()?;
        assert_eq!(gc.statistics()?.live_objects, 2);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn concurrent_old_to_young_stores_publish_roots_before_reclamation() -> Result<()> {
    for (concurrent, overlap) in [(false, false), (true, false), (false, true), (true, true)] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .overlapping_generations(overlap)
                .old_collection_interval(if overlap { usize::MAX } else { 0 })
                .independent_phases(overlap)
                .region_shards(if overlap { 16 } else { 1 })
                .metadata_shards(16)
                .work_stealing(true)
                .independent_marking(true)
                .mark_batch_size(8)
                .allocation_threshold(4096)
                .threads(4)
                .incremental_step_size(8)
                .full_collection_interval(8)
                .build(),
        );
        let parents: Vec<_> = (0..4)
            .map(|_| Gc::new(&gc, RwLock::new(Vec::<Gc<Vec<u64>>>::new())).into_rooted())
            .collect();
        gc.collect_full()?;
        gc.collect_full()?;
        gc.start();
        let workers: Vec<_> = parents
            .iter()
            .map(|parent| {
                let parent = parent.rooted_clone();
                let gc = gc.clone();
                std::thread::spawn(move || {
                    for i in 0..1000 {
                        let child = Gc::new(&gc, vec![i; 32]).into_rooted();
                        {
                            let mut edges = parent.write();
                            edges.clear();
                            edges.push(child);
                        }
                        std::thread::yield_now();
                        let witness = parent.read().first().expect("child").rooted_clone();
                        assert_eq!(witness.first(), Some(&i));
                    }
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
        drop(parents);
        gc.collect_full()?;
        gc.collect_full()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
        gc.stop()?;
    }
    Ok(())
}

#[test]
fn old_cadence_retains_precise_full_intervals() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .old_collection_interval(2)
                .full_collection_interval(4)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        young(&gc);
        assert_eq!(gc.statistics()?.old_collections, 0);
        young(&gc);
        assert_eq!(gc.statistics()?.old_collections, 1);
        assert_eq!(gc.statistics()?.full_collections, 0);
        young(&gc);
        young(&gc);
        assert_eq!(gc.statistics()?.old_collections, 1);
        assert_eq!(gc.statistics()?.full_collections, 1);
    }
    Ok(())
}

#[test]
fn reused_small_slot_revokes_the_original_typed_identity() -> Result<()> {
    for counted in [false, true] {
        let gc = collector(counted, false);
        let anchor = Gc::try_new(&gc, 7_u64)?;
        let original = Gc::try_new(&gc, 11_u64)?.into_rooted();
        let address = original.as_ptr();
        let handle = original.as_handle_i64();
        drop(original);
        gc.collect_full()?;
        let replacement = Gc::try_new(&gc, 17_u64)?.into_rooted();
        assert_eq!(replacement.as_ptr(), address);
        assert_ne!(replacement.as_handle_i64(), handle);
        assert!(Gc::<u64>::from_handle_rooted(&gc, handle).is_err());
        assert!(Gc::<i64>::from_handle_rooted(&gc, replacement.as_handle_i64()).is_err());
        assert_eq!(
            *Gc::<u64>::from_handle_rooted(&gc, replacement.as_handle_i64())?,
            17
        );
        drop((replacement, anchor));
        gc.collect_full()?;
        assert_eq!(gc.statistics()?.bytes_managed, 0);
    }
    Ok(())
}
