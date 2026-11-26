//! Cross-shard allocation, selection, accounting and contention regressions.
use super::*;

#[test]
fn one_allocator_reuses_existing_space_before_opening_another_shard() {
    let heap = RegionHeap::with_shards(1024, 4096, 0, Budget::new(4096), false, 16);
    let layout = Layout::from_size_align(64, 64).expect("layout");
    let blocks: Vec<_> = (0..16)
        .map(|_| heap.allocate(layout, None).expect("small allocation"))
        .collect();
    let stats = heap.statistics();
    assert_eq!(stats.registry_shards, 16);
    assert_eq!(stats.regions, 1);
    assert_eq!(stats.committed_bytes, 1024);
    drop(blocks);
    while !heap.trim(1) {}
    assert_eq!(heap.statistics().shared_committed_bytes, 0);
}

#[test]
fn legacy_packing_can_move_between_registries() {
    let heap = RegionHeap::with_shards(1024, 4096, 0, Budget::new(4096), false, 2);
    let layout = Layout::from_size_align(128, 64).expect("layout");
    let earlier = heap
        .allocate_selected(layout, None, false, 0)
        .expect("first region");
    let source = heap
        .allocate_selected(layout, None, false, 1)
        .expect("second region");
    assert!(earlier.id() < source.id());
    let destination = heap
        .allocate(layout, Some(&source))
        .expect("cross-shard destination");
    assert_eq!(destination.id(), earlier.id());
    drop((source, destination, earlier));
    while !heap.trim(1) {}
    assert_eq!(heap.statistics().shared_committed_bytes, 0);
}

#[test]
fn evacuation_ranks_all_shards_and_shares_one_source_quota() {
    let heap = RegionHeap::with_shards(1024, 16 * 1024, 0, Budget::new(16 * 1024), true, 16);
    let blocks: Vec<_> = (0..8)
        .map(|index| {
            // The cheapest sources are deliberately in the later registries.
            let size = if index < 4 { 512 } else { 128 };
            let layout = Layout::from_size_align(size, 64).expect("layout");
            heap.allocate_selected(layout, None, false, index)
                .expect("independent region")
        })
        .collect();
    assert!(heap.begin_evacuation());
    for step in 0..100 {
        if heap.select_evacuation() {
            break;
        }
        assert!(
            step < 99,
            "selection must charge bounded entries and shard advances"
        );
    }
    assert_eq!(heap.statistics().regions_selected, 4);
    assert_eq!(heap.statistics().selected_occupied_bytes, 4 * 128);
    for (index, block) in blocks.iter().enumerate() {
        let state = lock(&heap.states[index]);
        assert_eq!(
            state
                .evacuation
                .as_ref()
                .expect("policy")
                .source(&block.region),
            index >= 4
        );
    }
    heap.end_evacuation();
    drop(blocks);
    while !heap.trim(1) {}
    assert_eq!(heap.statistics().shared_committed_bytes, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn busy_preferred_registry_does_not_block_an_available_shard() {
    use std::sync::mpsc;
    use std::time::Duration;
    for existing in [true, false] {
        let heap = RegionHeap::with_shards(1024, 4096, 0, Budget::new(4096), false, 2);
        let layout = Layout::from_size_align(128, 64).expect("layout");
        let (preferred_tx, preferred_rx) = mpsc::channel();
        let (go_tx, go_rx) = mpsc::channel();
        let (done_tx, done_rx) = mpsc::channel();
        std::thread::scope(|scope| {
            let heap = &heap;
            let worker = scope.spawn(move || {
                preferred_tx
                    .send(heap.preferred_shard())
                    .expect("preference");
                go_rx.recv().expect("start");
                let block = heap.allocate(layout, None).expect("unrelated shard");
                done_tx.send(block).expect("result");
            });
            let preferred = preferred_rx.recv().expect("preference");
            let available = existing.then(|| {
                heap.allocate_selected(layout, None, false, 1 - preferred)
                    .expect("available hole")
            });
            let busy = lock(&heap.states[preferred]);
            go_tx.send(()).expect("start allocation");
            let result = done_rx.recv_timeout(Duration::from_secs(10));
            drop(busy);
            worker.join().expect("allocator");
            let block = result.expect("allocation blocked behind an unrelated registry");
            assert_eq!(heap.index(block.id()), 1 - preferred);
            if let Some(available) = available {
                assert_eq!(block.id(), available.id());
            }
            assert_eq!(heap.statistics().shared_committed_bytes, 1024);
        });
        while !heap.trim(1) {}
        assert_eq!(heap.statistics().shared_committed_bytes, 0);
    }
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn concurrent_shards_and_escaped_blocks_preserve_one_physical_cap() {
    let budget = Budget::new(4096);
    let heap = RegionHeap::with_shards(1024, 4096, 0, budget.clone(), false, 16);
    let barrier = std::sync::Barrier::new(16);
    let layout = Layout::from_size_align(1024, 64).expect("layout");
    let blocks = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..16)
            .map(|_| {
                let heap = &heap;
                let barrier = &barrier;
                scope.spawn(move || {
                    let block = heap.allocate(layout, None).ok();
                    barrier.wait();
                    block
                })
            })
            .collect();
        handles
            .into_iter()
            .filter_map(|handle| handle.join().expect("allocator"))
            .collect::<Vec<_>>()
    });
    assert_eq!(blocks.len(), 4);
    assert_eq!(heap.statistics().shared_committed_bytes, 4096);
    assert_eq!(heap.statistics().peak_shared_committed_bytes, 4096);
    drop(heap);
    assert_eq!(budget.used.load(Ordering::Acquire), 4096);
    drop(blocks);
    assert_eq!(budget.used.load(Ordering::Acquire), 0);
}

#[cfg(not(target_family = "wasm"))]
std::thread_local! {
    static RETRY: std::cell::Cell<Option<std::sync::mpsc::Sender<()>>> = const {
        std::cell::Cell::new(None)
    };
}

#[cfg(not(target_family = "wasm"))]
pub(super) fn record_contended_retry() {
    if let Some(sender) = RETRY.take() {
        sender.send(()).expect("allocator reached the shared cap");
    }
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_saturated_allocator_observes_a_returned_hole_while_its_preferred_shard_is_parked() {
    use std::sync::mpsc;
    use std::time::Duration;
    let budget = Budget::with_reserve(2048, 1024);
    let heap = RegionHeap::with_shards(1024, 2048, 1024, budget.clone(), false, 2);
    let layout = Layout::from_size_align(512, 64).expect("layout");
    let (index_tx, index_rx) = mpsc::channel();
    let (go_tx, go_rx) = mpsc::channel();
    let (miss_tx, miss_rx) = mpsc::channel();
    let (done_tx, done_rx) = mpsc::channel();
    std::thread::scope(|scope| {
        let heap = &heap;
        scope.spawn(move || {
            index_tx.send(heap.preferred_shard()).expect("preference");
            go_rx.recv().expect("allocation start");
            RETRY.set(Some(miss_tx));
            done_tx
                .send(heap.allocate(layout, None))
                .expect("allocation result");
        });
        let preferred = index_rx.recv().expect("preference");
        let index = 1 - preferred;
        let live = heap
            .allocate_selected(layout, None, false, index)
            .expect("live allocation");
        let returned = heap
            .allocate_selected(layout, None, false, index)
            .expect("allocation to return");
        let reusable_address = returned.ptr::<u8>();
        assert_ne!(live.ptr::<u8>(), reusable_address);
        let parked = lock(heap.shard(preferred));
        go_tx.send(()).expect("start");
        let miss = miss_rx.recv_timeout(Duration::from_secs(5));
        drop(returned);
        let completed = done_rx.recv_timeout(Duration::from_secs(5));
        drop(parked);
        miss.expect("peer must reach the physical cap first");
        let block = completed
            .expect("peer blocked on the parked shard")
            .expect("reused range");
        assert_eq!(block.id(), live.id());
        assert_eq!(block.ptr::<u8>(), reusable_address);
        assert_eq!(budget.peak_used.load(Ordering::Acquire), 1024);
    });
    while !heap.trim(1) {}
    assert_eq!(budget.used.load(Ordering::Acquire), 0);
}

#[test]
fn evacuation_roles_switch_before_the_first_shard_visit() {
    let heap = RegionHeap::with_shards(1024, 4096, 0, Budget::new(4096), true, 2);
    let layout = Layout::from_size_align(128, 64).expect("layout");
    let old = heap
        .allocate_selected(layout, None, false, 1)
        .expect("source");
    assert!(heap.begin_evacuation());
    while !heap.select_evacuation() {}
    let is_source = || {
        lock(heap.shard(1))
            .evacuation
            .as_ref()
            .expect("policy")
            .source(&old.region)
    };
    assert!(is_source());
    heap.end_evacuation();
    assert!(!is_source());
    assert!(heap.begin_evacuation());
    assert!(!is_source());
    // This allocation precedes even the first shard visit of the new epoch.
    let refill = heap
        .allocate_selected(layout, None, false, 1)
        .expect("reusable source");
    assert_eq!(refill.id(), old.id());
    while !heap.select_evacuation() {}
    let state = lock(heap.shard(1));
    let plan = state.evacuation.as_ref().expect("policy");
    assert!(plan.is_destination(&old.region));
    assert!(!plan.source(&old.region));
    drop(state);
    heap.end_evacuation();
    drop((old, refill));
    while !heap.trim(4) {}
    assert_eq!(heap.statistics().shared_committed_bytes, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn parked_registry_allows_phase_publication_and_bounded_selection_retry() {
    use std::sync::mpsc;
    use std::time::Duration;
    for enabled in [false, true] {
        let heap = RegionHeap::with_shards(1024, 4096, 0, Budget::new(4096), enabled, 2);
        let layout = Layout::from_size_align(128, 64).expect("layout");
        let block = heap
            .allocate_selected(layout, None, false, 1)
            .expect("source");
        std::thread::scope(|scope| {
            let parked = lock(heap.shard(1));
            let (done, result) = mpsc::channel();
            let heap = &heap;
            scope.spawn(move || {
                assert_eq!(heap.begin_evacuation(), enabled);
                if enabled {
                    assert!(!heap.select_evacuation()); // Visit the empty shard.
                    assert!(!heap.select_evacuation()); // Defer the parked shard.
                    assert!(heap.select_evacuation());
                } else {
                    assert!(heap.select_evacuation());
                }
                heap.end_evacuation();
                done.send(()).expect("phase completion");
            });
            let completed = result.recv_timeout(Duration::from_secs(3));
            drop(parked);
            completed.expect("a parked registry delayed an evacuation transition");
        });
        if enabled {
            assert!(heap.begin_evacuation());
            while !heap.select_evacuation() {}
            let state = lock(heap.shard(1));
            assert!(
                state
                    .evacuation
                    .as_ref()
                    .expect("policy")
                    .source(&block.region)
            );
            drop(state);
            heap.end_evacuation();
        }
    }
}

#[test]
fn fresh_collections_trim_an_available_shard_and_finish_while_the_other_is_parked() {
    for cost_aware in [false, true] {
        for counted in [false, true] {
            let gc = crate::GarbageCollector::with_config(
                crate::Configuration::builder()
                    .concurrent_publication(counted)
                    .compact_heap(true)
                    .cost_aware_relocation(cost_aware)
                    .region_shards(2)
                    .region_layout(1024, 32)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let heap = gc.region_heap().expect("regional payloads");
            for i in 0..2 {
                drop(
                    heap.allocate_selected(Layout::new::<u64>(), None, false, i)
                        .expect("seed backing"),
                );
            }
            assert_eq!(heap.statistics().committed_bytes, 2048);
            let peer = lock(heap.shard(1)).backing.clone();
            let root = crate::Gc::new(&gc, 47_u64);
            let parked = lock(heap.shard(0));
            for _ in 0..2 {
                drop(crate::Gc::new(&gc, 17_u64));
                let before = gc.completed_full_ticket();
                gc.collect();
                for _ in 0..4096 {
                    if gc.completed_full_ticket() > before {
                        break;
                    }
                    gc.step();
                }
                assert!(
                    gc.completed_full_ticket() > before,
                    "cost aware {cost_aware}, counted {counted}"
                );
                assert_eq!(
                    gc.managed_bytes(),
                    crate::Gc::allocation_size(&47_u64).expect("live scalar")
                );
                assert_eq!(peer.committed.load(Ordering::Acquire), 0);
                assert_eq!(parked.backing.committed.load(Ordering::Acquire), 1024);
                assert_eq!(**root, 47);
            }
            drop(parked);
            gc.collect_full().expect("next physical trim");
            assert_eq!(heap.statistics().committed_bytes, 0);
        }
    }
}

#[test]
fn skipped_evacuation_index_from_an_older_epoch_is_never_ranked() {
    let heap = RegionHeap::with_shards(1024, 4096, 0, Budget::new(4096), true, 2);
    let layout = Layout::from_size_align(128, 64).expect("layout");
    let parked_source = heap
        .allocate_selected(layout, None, false, 0)
        .expect("source");
    let peer_source = heap
        .allocate_selected(layout, None, false, 1)
        .expect("peer");
    // Leave both indexes populated immediately before the old ranking step.
    assert!(heap.begin_evacuation());
    while heap.selection_shard.load(Ordering::Acquire) != 2 {
        assert!(!heap.select_evacuation());
    }
    heap.end_evacuation();
    assert!(heap.begin_evacuation());
    {
        let _parked = lock(heap.shard(0));
        assert!(!heap.select_evacuation());
    }
    while !heap.select_evacuation() {}
    let epoch = heap
        .selection
        .as_ref()
        .expect("policy")
        .epoch()
        .expect("active");
    assert!(!Evacuation::source_at(&parked_source.region, epoch));
    assert!(Evacuation::source_at(&peer_source.region, epoch));
    heap.end_evacuation();
    assert!(heap.begin_evacuation());
    while !heap.select_evacuation() {}
    let epoch = heap
        .selection
        .as_ref()
        .expect("policy")
        .epoch()
        .expect("active");
    assert!(Evacuation::source_at(&parked_source.region, epoch));
    heap.end_evacuation();
}

#[test]
fn a_retired_source_keeps_its_release_accounting_after_the_epoch_ends() {
    let heap = RegionHeap::with_policy(1024, 4096, 0, Budget::new(4096), true);
    let layout = Layout::from_size_align(128, 64).expect("layout");
    let block = heap.allocate(layout, None).expect("source");
    assert!(heap.begin_evacuation());
    while !heap.select_evacuation() {}
    let backing = block.region.clone();
    drop(block);
    heap.end_evacuation();
    let held = heap.statistics();
    assert_eq!(held.regions, 0);
    assert_eq!(held.evacuation_bytes_released, 0);
    assert_eq!(held.shared_committed_bytes, 1024);
    drop(backing);
    let released = heap.statistics();
    assert_eq!(released.evacuation_regions_released, 1);
    assert_eq!(released.evacuation_bytes_released, 1024);
    assert_eq!(released.shared_committed_bytes, 0);
}

#[cfg(not(target_family = "wasm"))]
impl RegionHeap {
    pub(crate) fn with_parked_registry(&self, work: impl FnOnce()) {
        let _guard = lock(self.shard(0));
        work();
    }
}

#[cfg(not(target_family = "wasm"))]
std::thread_local! {
    static PROBE_PUBLICATION: std::cell::Cell<Option<Box<dyn FnOnce()>>> = const { std::cell::Cell::new(None) };
}

#[cfg(not(target_family = "wasm"))]
pub(super) fn before_probe_publication() {
    if let Some(hook) = PROBE_PUBLICATION.take() {
        hook();
    }
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn cancelled_probe_releases_unpublished_backing_while_the_registry_is_parked() {
    use std::sync::mpsc;
    use std::task::Poll;
    use std::time::Duration;
    let heap = RegionHeap::new(1024, 1024, 0);
    let (ready, reached) = mpsc::channel();
    let (resume, wait) = mpsc::channel();
    let (done, finished) = mpsc::channel();
    std::thread::scope(|scope| {
        let peer_heap = &heap;
        let peer = scope.spawn(move || {
            PROBE_PUBLICATION.set(Some(Box::new(move || {
                ready.send(()).expect("notify registry owner");
                wait.recv_timeout(Duration::from_secs(5))
                    .expect("resume publication");
            })));
            let result = AllocationProbe::default().poll(peer_heap, Layout::new::<u64>());
            done.send(matches!(result, Poll::Pending))
                .expect("report rollback");
        });
        reached
            .recv_timeout(Duration::from_secs(5))
            .expect("raw backing reserved");
        let registry = lock(heap.shard(0));
        assert_eq!(registry.backing.committed.load(Ordering::Acquire), 1024);
        assert_eq!(heap.budget.used.load(Ordering::Acquire), 1024);
        resume.send(()).expect("attempt publication");
        let rolled_back = finished.recv_timeout(Duration::from_secs(2));
        let local = registry.backing.committed.load(Ordering::Acquire);
        let shared = heap.budget.used.load(Ordering::Acquire);
        assert!(registry.regions.is_empty());
        drop(registry);
        peer.join().expect("allocation completed");
        assert_eq!(rolled_back, Ok(true));
        assert_eq!((local, shared), (0, 0));
    });
    assert_eq!(heap.returned_bytes.load(Ordering::Acquire), 0);
}

#[test]
fn probe_reuses_a_returned_peer_range_before_a_parked_registry_resumes() {
    use std::task::Poll;
    let heap =
        RegionHeap::with_shards(1024, 2048, 1024, Budget::with_reserve(2048, 1024), false, 2);
    let preferred = heap.preferred_shard();
    let layout = Layout::from_size_align(512, 64).expect("layout");
    let keep = heap
        .allocate_selected(layout, None, false, 1 - preferred)
        .expect("first half");
    let donor = heap
        .allocate_selected(layout, None, false, 1 - preferred)
        .expect("second half");
    let address = donor.ptr::<u8>().addr();
    let parked = lock(heap.shard(preferred));
    let mut probe = AllocationProbe::default();
    assert!(probe.poll(&heap, layout).is_pending());
    drop(donor);
    let Poll::Ready(Ok(reused)) = probe.poll(&heap, layout) else {
        panic!("free peer range");
    };
    assert_eq!(reused.ptr::<u8>().addr(), address);
    assert_eq!(heap.budget.used.load(Ordering::Acquire), 1024);
    drop(parked);
    drop((reused, keep));
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn workerless_allocation_reclaims_an_unreachable_peer_range_before_registry_release() {
    contended_allocation_reclaims_unreachable_peer(false);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn native_worker_reclaims_an_unreachable_peer_range_before_registry_release() {
    contended_allocation_reclaims_unreachable_peer(true);
}

#[cfg(not(target_family = "wasm"))]
fn contended_allocation_reclaims_unreachable_peer(worker: bool) {
    use std::future::Future;
    use std::sync::mpsc;
    use std::task::{Context, Poll, Waker};
    use std::time::Duration;

    let gc = crate::GarbageCollector::with_config(
        crate::Configuration::builder()
            .threads(1)
            .region_shards(2)
            .compact_heap(true)
            .cost_aware_relocation(true)
            .region_layout(1024, 0)
            .max_heap_bytes(2048)
            .relocation_reserve_bytes(1024)
            .build(),
    );
    let heap = gc.region_heap().expect("payload heap");
    let buffer = crate::HeapBuffer::try_with_fn(Some(heap), 896, |_| Ok(0_u8)).expect("donor");
    let source = heap.index(buffer.relocation_region(heap).expect("regional donor"));
    let root = crate::Gc::try_new(&gc, buffer).expect("publish donor");
    assert_eq!(heap.statistics().shared_committed_bytes, 1024);
    if worker {
        gc.start();
    }
    assert_eq!(gc.registered_workers(), usize::from(worker));

    let (send, recv) = mpsc::channel();
    std::thread::scope(|scope| {
        let parked = lock(heap.shard(1 - source));
        drop(root);
        let gc = &gc;
        scope.spawn(move || {
            let mut future =
                std::pin::pin!(crate::HeapBuffer::try_with_fn_in_async(gc, 256, |_| {
                    Ok(3_u8)
                }));
            let mut cx = Context::from_waker(Waker::noop());
            let mut completed = None;
            for _ in 0..4096 {
                if let Poll::Ready(result) = future.as_mut().poll(&mut cx) {
                    completed =
                        Some(result.map(|buf| buf.iter().copied().map(usize::from).sum::<usize>()));
                    break;
                }
                if worker {
                    std::thread::yield_now();
                }
            }
            send.send(completed)
                .expect("report allocation after 4,096 polls");
        });
        let before_release = recv.recv_timeout(Duration::from_secs(3));
        drop(parked);
        assert!(
            matches!(before_release, Ok(Some(Ok(768)))),
            "{before_release:?}"
        );
    });
    if worker {
        gc.stop().expect("worker shutdown");
    }
}

#[test]
fn probe_resumes_alignment_search_after_its_extent_budget() {
    use std::task::Poll;
    let heap = RegionHeap::new(1024, 1024, 0);
    let bytes: Vec<_> = (0..1024)
        .map(|_| heap.allocate(Layout::new::<u8>(), None).expect("byte"))
        .collect();
    let base = bytes.first().expect("populated region").ptr::<u8>().addr();
    let retained: Vec<_> = bytes
        .into_iter()
        .filter(|block| {
            let offset = block.ptr::<u8>().addr() - base;
            !(offset < 640 && (1..9).contains(&(offset % 16)) || (960..968).contains(&offset))
        })
        .collect();
    let mut probe = AllocationProbe::default();
    let layout = Layout::from_size_align(8, 64).expect("aligned request");
    assert!(probe.poll(&heap, layout).is_pending());
    let Poll::Ready(Ok(block)) = probe.poll(&heap, layout) else {
        panic!("aligned late hole");
    };
    assert_eq!(block.ptr::<u8>().addr(), base + 960);
    drop((block, retained));
}

#[cfg(not(target_family = "wasm"))]
fn parked_payload_allocation(park_range_map: bool) {
    use std::future::Future;
    use std::sync::mpsc;
    use std::task::{Context, Poll, Wake, Waker};
    use std::time::Duration;

    struct Ready(AtomicBool);
    impl Wake for Ready {
        fn wake(self: Arc<Self>) {
            self.0.store(true, Ordering::Release);
        }
    }

    for shards in [1, 2] {
        let gc = crate::GarbageCollector::with_config(
            crate::Configuration::builder()
                .region_shards(shards)
                .compact_heap(true)
                .max_heap_bytes(1024)
                .relocation_reserve_bytes(0)
                .region_layout(1024, 0)
                .build(),
        );
        let heap = gc.region_heap().expect("payload heap");
        let held = heap
            .allocate_selected(Layout::new::<u64>(), None, false, 0)
            .expect("seed an existing region with free space");
        let initial = heap.statistics();
        let (started, polling) = mpsc::channel();
        let (first, result) = mpsc::channel();
        let (resume, released) = mpsc::channel();
        let initialized = AtomicUsize::new(0);
        std::thread::scope(|scope| {
            let registry_guard = (!park_range_map).then(|| lock(heap.shard(0)));
            let range_guard = park_range_map.then(|| lock(&held.region.free));
            let initialized = &initialized;
            let gc = &gc;
            let peer = scope.spawn(move || {
                let ready = Arc::new(Ready(AtomicBool::new(false)));
                let waker = Waker::from(ready.clone());
                let mut context = Context::from_waker(&waker);
                let mut buffer = Box::pin(crate::HeapBuffer::try_with_fn_in_async(gc, 64, |_| {
                    initialized.fetch_add(1, Ordering::Relaxed);
                    Ok(9_u8)
                }));
                started.send(()).expect("poll observer");
                let initial = buffer.as_mut().poll(&mut context);
                first
                    .send(initial.is_pending())
                    .expect("observe first poll");
                released.recv().expect("release the registry");
                match initial {
                    Poll::Ready(result) => result.expect("buffer after release"),
                    Poll::Pending => {
                        for _ in 0..64 {
                            assert!(
                                ready.0.swap(false, Ordering::AcqRel),
                                "pending poll lost its wake"
                            );
                            if let Poll::Ready(result) = buffer.as_mut().poll(&mut context) {
                                return result.expect("buffer after release");
                            }
                        }
                        panic!("available payload range was not reused");
                    }
                }
            });
            polling.recv().expect("worker started");
            let first_poll = result.recv_timeout(Duration::from_secs(2));
            let before_release = initialized.load(Ordering::Relaxed);
            drop((registry_guard, range_guard));
            resume.send(()).expect("resume allocation");
            let buffer = peer.join().expect("allocation worker");
            assert_eq!(
                first_poll,
                Ok(true),
                "the first poll waited on a parked allocator"
            );
            assert_eq!(before_release, 0);
            assert_eq!(initialized.load(Ordering::Relaxed), 64);
            assert_eq!(&*buffer, &[9_u8; 64]);
        });
        let after = heap.statistics();
        assert_eq!(after.shared_committed_bytes, initial.shared_committed_bytes);
        assert_eq!(after.occupied_bytes, initial.occupied_bytes);
    }
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn async_buffer_yields_while_the_only_fitting_registry_is_parked() {
    parked_payload_allocation(false);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn async_buffer_yields_while_a_fitting_range_map_is_parked() {
    parked_payload_allocation(true);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_copy_starts_and_continues_while_its_source_registry_is_parked() {
    use crate::{HeapBuffer, TraceProgress};
    use std::sync::mpsc;
    use std::time::Duration;

    for already_copied in [0, 64] {
        let heap =
            RegionHeap::with_shards(1024, 4096, 1024, Budget::with_reserve(4096, 1024), true, 2);
        let mut source = HeapBuffer::try_with_fn(Some(&heap), 256, |_| Ok(91_u8)).expect("payload");
        let source_id = source.relocation_region(&heap).expect("source region");
        let source_index = heap.index(source_id);
        let filler = heap
            .allocate_selected(
                Layout::from_size_align(768, 64).expect("layout"),
                None,
                false,
                1 - source_index,
            )
            .expect("destination with a free 256-byte range");
        assert!(heap.begin_evacuation());
        while !heap.select_evacuation() {}
        if already_copied != 0 {
            assert!(matches!(
                source.compact_step(&heap, 64),
                TraceProgress::Continue(64)
            ));
        }
        std::thread::scope(|scope| {
            let (done, recv) = mpsc::channel();
            let source = &mut source;
            let heap = &heap;
            let parked = lock(heap.shard(source_index));
            scope.spawn(move || {
                let _ = done.send(source.compact_step(heap, 64));
            });
            let result = recv.recv_timeout(Duration::from_secs(3));
            drop(parked);
            assert!(
                matches!(result.expect("copy waited for the source registry"),
                TraceProgress::Continue(n) if n == already_copied + 64)
            );
        });
        source.compact_step(&heap, 256);
        assert!(source.iter().all(|value| *value == 91));
        assert_eq!(source.relocation_region(&heap), Some(filler.id()));
        assert_eq!(heap.statistics().bytes_copied, 256);
        heap.end_evacuation();
        drop((source, filler));
        while !heap.trim(4) {}
        assert_eq!(heap.statistics().shared_committed_bytes, 0);
    }
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn last_unlinked_region_releases_its_backing_while_registry_is_parked() {
    use std::sync::mpsc;
    use std::time::Duration;
    let heap = RegionHeap::new(1024, 4096, 0);
    let block = heap.allocate(Layout::new::<[u8; 4096]>(), None).unwrap();
    let region = block.region.clone();
    drop(block);
    assert_eq!(heap.statistics().regions, 0);
    assert_eq!(heap.budget.used.load(Ordering::Acquire), 4096);
    let (done, finished) = mpsc::channel();
    std::thread::scope(|scope| {
        let locked = lock(heap.shard(0));
        scope.spawn(move || {
            drop(region);
            done.send(()).unwrap();
        });
        let before_release = finished.recv_timeout(Duration::from_secs(1));
        drop(locked);
        assert!(before_release.is_ok());
    });
    let stats = heap.statistics();
    assert_eq!(stats.shared_committed_bytes, 0);
    assert_eq!(stats.committed_bytes, 0);
    assert_eq!(stats.regions_released, 1);
}

#[test]
fn clearing_a_partial_buffer_keeps_its_elements_until_staging_can_return() {
    use crate::{HeapBuffer, Trace, TraceProgress};
    let heap = RegionHeap::new(1024, 2048, 0);
    let progress = heap.recovery_progress();
    let mut buffer: HeapBuffer<u64> = (0..64).collect();
    assert!(matches!(
        buffer.prepare_step(&heap, 8).unwrap(),
        TraceProgress::Continue(_)
    ));
    assert_eq!(heap.statistics().occupied_bytes, 512);
    {
        let _parked = lock(heap.shard(0));
        assert_eq!(heap.recovery_progress(), progress);
        assert_eq!(buffer.clear_step(0, 1), TraceProgress::Busy);
        assert_eq!(buffer.as_ref(), (0..64).collect::<Vec<_>>());
    }
    assert_eq!(buffer.clear_step(0, 1), TraceProgress::Complete);
    assert!(buffer.is_empty());
    assert_eq!(heap.recovery_progress(), progress);
    assert_eq!(heap.statistics().occupied_bytes, 0);
}

#[test]
fn a_busy_free_range_map_keeps_the_block_and_its_exact_charge() {
    let heap = RegionHeap::new(1024, 1024, 0);
    let mut block = heap.allocate(Layout::new::<[u64; 8]>(), None).unwrap();
    let region = block.region.clone();
    {
        let _parked = lock(&region.free);
        assert!(!block.try_return());
        assert_eq!(region.occupied(), 64);
    }
    assert!(block.try_return());
    assert!(block.try_return());
    drop(block);
    assert_eq!(heap.statistics().occupied_bytes, 0);
    assert_eq!(region.occupied(), 0);
}

#[test]
fn clearing_partial_transfers_drops_each_noncopy_element_once() {
    use crate::{HeapBuffer, Trace, TraceProgress};
    struct Element<'a>(&'a AtomicUsize);
    // SAFETY: each element only points to its host-side destruction counter.
    unsafe impl Trace for Element<'_> {
        fn trace(&self, _: &crate::GarbageCollector) {}
    }
    impl Drop for Element<'_> {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    let counters = [const { AtomicUsize::new(0) }; 32];
    let make_elements = || counters.iter().map(Element).collect::<HeapBuffer<_>>();
    let size = counters.len() * size_of::<Element<'_>>();
    let heap = RegionHeap::new(1024, 2048, 0);
    let mut preparing = make_elements();
    assert!(matches!(
        preparing.prepare_step(&heap, 1),
        Ok(TraceProgress::Continue(_))
    ));
    {
        let _parked = lock(heap.shard(0));
        assert_eq!(preparing.clear_step(0, 1), TraceProgress::Busy);
        assert!(counters.iter().all(|n| n.load(Ordering::Relaxed) == 0));
    }
    for _ in 1..counters.len() {
        assert_eq!(preparing.clear_step(0, 1), TraceProgress::Continue(0));
    }
    assert_eq!(preparing.clear_step(0, 1), TraceProgress::Complete);
    drop(preparing);
    assert!(counters.iter().all(|n| n.load(Ordering::Relaxed) == 1));
    assert_eq!(heap.recovery_progress().0, 0);

    let heap = RegionHeap::with_shards(1024, 4096, 1024, Budget::with_reserve(4096, 1024), true, 2);
    let mut compacting = make_elements();
    compacting.prepare(&heap).unwrap();
    let id = compacting.relocation_region(&heap).unwrap();
    let source = heap.index(id);
    let filler = heap
        .allocate_selected(
            Layout::from_size_align(1024 - size, 64).unwrap(),
            None,
            false,
            1 - source,
        )
        .unwrap();
    assert!(heap.begin_evacuation());
    while !heap.select_evacuation() {}
    assert!(matches!(
        compacting.compact_step(&heap, 1),
        TraceProgress::Continue(_)
    ));
    {
        let _parked = lock(heap.shard(1 - source));
        assert_eq!(compacting.clear_step(0, 1), TraceProgress::Busy);
        assert!(counters.iter().all(|n| n.load(Ordering::Relaxed) == 1));
    }
    {
        let _parked = lock(heap.shard(source));
        for _ in 1..counters.len() {
            assert_eq!(compacting.clear_step(0, 1), TraceProgress::Continue(0));
        }
        for _ in 0..2 {
            assert_eq!(compacting.clear_step(0, 1), TraceProgress::Busy);
            assert!(counters.iter().all(|n| n.load(Ordering::Relaxed) == 2));
            assert_eq!(heap.recovery_progress().0, 0);
        }
    }
    assert_eq!(compacting.clear_step(0, 1), TraceProgress::Complete);
    drop(compacting);
    assert_eq!(heap.recovery_progress().0, u64::try_from(size).unwrap());
    assert!(counters.iter().all(|n| n.load(Ordering::Relaxed) == 2));
    drop(filler);
}
