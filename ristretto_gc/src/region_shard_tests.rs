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
        let available = heap
            .allocate_selected(layout, None, false, 1 - preferred)
            .expect("available hole");
        let busy = lock(&heap.states[preferred]);
        go_tx.send(()).expect("start allocation");
        let result = done_rx.recv_timeout(Duration::from_secs(10));
        drop(busy);
        worker.join().expect("allocator");
        let block = result.expect("allocation blocked behind an unrelated registry");
        assert_eq!(block.id(), available.id());
    });
    while !heap.trim(1) {}
    assert_eq!(heap.statistics().shared_committed_bytes, 0);
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
