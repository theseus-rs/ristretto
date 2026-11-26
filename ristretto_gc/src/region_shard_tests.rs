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
        .allocate(layout, Some(source.id()))
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
        if heap.select_evacuation().expect("uncontended selection") {
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
    while !heap.select_evacuation().expect("uncontended selection") {}
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
    while !heap.select_evacuation().expect("uncontended selection") {}
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
                    assert_eq!(heap.select_evacuation(), Some(false)); // Visit the empty shard.
                    assert_eq!(heap.select_evacuation(), None); // Parked next shard.
                    assert_eq!(heap.select_evacuation(), None);
                } else {
                    assert_eq!(heap.select_evacuation(), Some(true));
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
            while !heap.select_evacuation().expect("uncontended selection") {}
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
fn a_retired_source_keeps_its_release_accounting_after_the_epoch_ends() {
    let heap = RegionHeap::with_policy(1024, 4096, 0, Budget::new(4096), true);
    let layout = Layout::from_size_align(128, 64).expect("layout");
    let block = heap.allocate(layout, None).expect("source");
    assert!(heap.begin_evacuation());
    while !heap.select_evacuation().expect("uncontended selection") {}
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
