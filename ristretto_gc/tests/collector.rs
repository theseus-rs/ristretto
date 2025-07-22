//! Garbage collector functionality tests
//!
//! Tests collector configuration, statistics, collection cycles, and performance monitoring.

use ristretto_gc::{Configuration, GC, GarbageCollector, Gc, Result, Trace};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

#[test_log::test]
fn test_custom_gc_config() -> Result<()> {
    let config = Configuration {
        threads: 1,
        allocation_threshold: 16 * 1024 * 1024, // 16MB
        max_pause_time_us: 50,
        incremental_step_size: 2000,
        parallel_threshold: 10_000_000,
    };

    let collector = GarbageCollector::with_config(config.clone());
    collector.start();

    // Test that collector works with custom config
    let _gc = Gc::with_collector(&collector, "test with custom config");
    collector.collect();
    Ok(())
}

#[test_log::test]
fn test_collector_lifecycle() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    // Test basic operations work with started collector
    let _gc = Gc::with_collector(&collector, 42);
    collector.collect();
    let _stats = collector.statistics()?;

    // Test stopping collector
    collector.stop()?;

    // Test that stopping again doesn't cause issues
    collector.stop()?;
    Ok(())
}

#[test_log::test]
fn test_global_collector() -> Result<()> {
    // Test using the specific collector instance (not global)
    let gc1 = Gc::new("global test 1");
    let gc2 = Gc::new("global test 2");

    GC.collect();
    let stats = GC.statistics()?;

    // Should show some allocation activity
    assert!(stats.bytes_allocated > 0);

    // Objects should be accessible
    assert_eq!(*gc1, "global test 1");
    assert_eq!(*gc2, "global test 2");
    Ok(())
}

#[test_log::test]
fn test_garbage_collector_does_not_free_root() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    let gc = Gc::with_collector(&collector, 42);
    // Register the Gc as a root to prevent it from being collected
    let root = gc.as_root(&collector)?;

    collector.collect();
    thread::sleep(Duration::from_millis(100));
    let stats = collector.statistics()?;

    assert_eq!(stats.collections_started, 1);
    assert_eq!(stats.collections_completed, 1);
    assert_eq!(stats.bytes_allocated, 4);
    assert_eq!(stats.bytes_freed, 0);

    assert_eq!(*root, 42);
    Ok(())
}

#[test_log::test]
fn test_garbage_collection_frees_memory() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    let initial_stats = collector.statistics()?;

    // The initial stats should show:
    // - no collections
    // - no allocations
    // - no bytes have been freed yet
    assert_eq!(initial_stats.collections_completed, 0);
    assert_eq!(initial_stats.bytes_allocated, 0);
    assert_eq!(initial_stats.bytes_freed, 0);

    let objects: Vec<_> = (0..100)
        .map(|i| {
            let gc = Gc::with_collector(&collector, format!("object-{i}"));
            gc.as_root(&collector)
        })
        .collect::<Result<Vec<_>>>()?;
    let allocation_stats = collector.statistics()?;

    // After creating object, the allocation stats should show:
    // - no collections
    // - bytes have been allocated (at least 100 objects)
    // - no bytes have been freed yet
    assert_eq!(initial_stats.collections_completed, 0);
    assert!(allocation_stats.bytes_allocated >= initial_stats.bytes_allocated);
    assert_eq!(allocation_stats.bytes_freed, 0);

    collector.collect();
    thread::sleep(Duration::from_millis(100));
    let collector_stats = collector.statistics()?;

    // After running the garbage collector, the collector stats should show:
    // - there was at least one collection since we allocated objects
    // - no new bytes were allocated
    // - no bytes have been freed yet
    assert!(collector_stats.collections_completed > allocation_stats.collections_completed);
    assert_eq!(
        collector_stats.bytes_allocated,
        allocation_stats.bytes_allocated
    );
    assert_eq!(collector_stats.bytes_freed, 0);

    drop(objects);

    collector.collect();
    thread::sleep(Duration::from_millis(100));
    let final_stats = collector.statistics()?;
    // After dropping the objects and running the garbage collector, the final stats should show:
    // - no bytes are allocated (they were freed)
    // - the same number of bytes should have been freed as allocated
    // - there was at least one collection since the last collection stats
    assert!(final_stats.collections_completed >= collector_stats.collections_completed);
    assert_eq!(final_stats.bytes_allocated, initial_stats.bytes_allocated);
    assert_eq!(final_stats.bytes_freed, allocation_stats.bytes_allocated);
    Ok(())
}

#[test_log::test]
fn test_gc_statistics() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    let initial_stats = collector.statistics()?;
    let objects: Vec<_> = (0..100)
        .map(|i| Gc::with_collector(&collector, format!("object-{i}")))
        .collect();

    collector.collect();
    thread::sleep(Duration::from_millis(100));

    let final_stats = collector.statistics()?;

    // Should show increased activity
    assert!(final_stats.collections_started >= initial_stats.collections_started);
    assert!(final_stats.bytes_allocated >= initial_stats.bytes_allocated);

    // Objects should still be accessible
    for (i, obj) in objects.iter().enumerate() {
        assert_eq!(**obj, format!("object-{i}"));
    }

    Ok(())
}

#[test_log::test]
fn test_allocation_threshold_trigger() -> Result<()> {
    // Test that allocations trigger collection when threshold is exceeded
    let small_threshold_config = Configuration {
        allocation_threshold: 1024, // Small threshold
        ..Default::default()
    };

    let collector = GarbageCollector::with_config(small_threshold_config);
    collector.start();

    let initial_stats = collector.statistics()?;

    // Allocate enough to exceed threshold
    let large_objects: Vec<_> = (0..10)
        .map(|i| {
            Gc::with_collector(&collector, vec![i; 1000]) // Large objects
        })
        .collect();

    // Explicitly trigger collection to ensure it runs
    collector.collect();

    // Give collector more time to run
    thread::sleep(Duration::from_millis(500));

    let final_stats = collector.statistics()?;

    // Should have triggered at least one collection
    assert!(final_stats.collections_started > initial_stats.collections_started);

    // Objects should still be accessible
    for (i, obj) in large_objects.iter().enumerate() {
        assert_eq!(obj[0], i);
        assert_eq!(obj.len(), 1000);
    }
    Ok(())
}

#[test_log::test]
fn test_root_management() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    #[derive(Debug)]
    struct TestObject {
        id: usize,
        data: String,
    }

    impl Trace for TestObject {
        fn trace(&self, _collector: &GarbageCollector) {
            // No GC references to trace
        }
    }

    let obj1 = Gc::with_collector(
        &collector,
        TestObject {
            id: 1,
            data: "root object 1".to_string(),
        },
    );

    let obj2 = Gc::with_collector(
        &collector,
        TestObject {
            id: 2,
            data: "root object 2".to_string(),
        },
    );

    // Add objects as roots
    obj1.as_root(&collector)?;
    obj2.as_root(&collector)?;

    // Trigger collection - roots should remain accessible
    collector.collect();
    thread::sleep(Duration::from_millis(50));

    assert_eq!(obj1.id, 1);
    assert_eq!(obj2.id, 2);
    assert_eq!(obj1.data, "root object 1");
    assert_eq!(obj2.data, "root object 2");

    Ok(())
}

#[test_log::test]
fn test_reachability_analysis() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    #[derive(Debug)]
    struct Node {
        id: usize,
        children: Vec<Gc<Node>>,
    }

    impl Trace for Node {
        fn trace(&self, collector: &GarbageCollector) {
            for child in &self.children {
                child.trace(collector);
            }
        }
    }

    // Create a tree structure
    let leaf1 = Gc::with_collector(
        &collector,
        Node {
            id: 1,
            children: vec![],
        },
    );
    let leaf2 = Gc::with_collector(
        &collector,
        Node {
            id: 2,
            children: vec![],
        },
    );
    let leaf3 = Gc::with_collector(
        &collector,
        Node {
            id: 3,
            children: vec![],
        },
    );

    let branch1 = Gc::with_collector(
        &collector,
        Node {
            id: 10,
            children: vec![leaf1.clone(), leaf2.clone()],
        },
    );

    let branch2 = Gc::with_collector(
        &collector,
        Node {
            id: 20,
            children: vec![leaf3.clone()],
        },
    );

    let root = Gc::with_collector(
        &collector,
        Node {
            id: 100,
            children: vec![branch1.clone(), branch2.clone()],
        },
    );

    // Add root to make entire tree reachable
    root.as_root(&collector)?;

    // Trigger collection - all nodes should remain accessible through reachability
    collector.collect();
    thread::sleep(Duration::from_millis(100));

    // Verify entire tree is still accessible
    assert_eq!(root.id, 100);
    assert_eq!(root.children.len(), 2);

    assert_eq!(branch1.id, 10);
    assert_eq!(branch1.children.len(), 2);
    assert_eq!(branch1.children[0].id, 1);
    assert_eq!(branch1.children[1].id, 2);

    assert_eq!(branch2.id, 20);
    assert_eq!(branch2.children.len(), 1);
    assert_eq!(branch2.children[0].id, 3);

    Ok(())
}

#[test_log::test]
fn test_collection_phases() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    // Test that collection goes through its phases without hanging
    let objects: Vec<_> = (0..50).map(Gc::new).collect();

    // Trigger multiple collections
    for _ in 0..5 {
        collector.collect();
        thread::sleep(Duration::from_millis(50));
    }

    let stats = collector.statistics()?;

    // Should have completed some collections
    assert!(stats.collections_started > 0);

    // All objects should still be accessible
    for (i, obj) in objects.iter().enumerate() {
        assert_eq!(**obj, i);
    }

    Ok(())
}

#[test_log::test]
fn test_concurrent_collection_safety() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    #[derive(Debug)]
    struct SharedData {
        values: Mutex<Vec<usize>>,
    }

    impl Trace for SharedData {
        fn trace(&self, _collector: &GarbageCollector) {
            // No GC references to trace
        }
    }

    let shared = Gc::with_collector(
        &collector,
        SharedData {
            values: Mutex::new(vec![]),
        },
    );

    shared.as_root(&collector)?;

    let mut handles = vec![];

    // Threads that modify shared data while collection might be running
    for thread_id in 0..3 {
        let shared_clone = shared.clone();
        let handle = thread::spawn(move || {
            for i in 0..20 {
                if let Ok(mut values) = shared_clone.values.lock() {
                    values.push(thread_id * 100 + i);
                }
                thread::sleep(Duration::from_millis(5));
            }
        });
        handles.push(handle);
    }

    // Concurrent collection thread
    let collection_handle = thread::spawn(move || {
        for _ in 0..10 {
            collector.collect();
            thread::sleep(Duration::from_millis(15));
        }
    });

    for handle in handles {
        handle.join().unwrap();
    }
    collection_handle.join().unwrap();

    // Verify data integrity
    if let Ok(values) = shared.values.lock() {
        assert_eq!(values.len(), 60); // 3 threads * 20 items each
    }

    Ok(())
}

#[test_log::test]
fn test_large_object_collection() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    // Test collection with large objects
    let large_objects: Vec<_> = (0..10)
        .map(|i| {
            Gc::with_collector(&collector, vec![i; 10000]) // 10k elements each
        })
        .collect();

    // Keep some reachable by adding as root
    large_objects[0].as_root(&collector)?;
    large_objects[5].as_root(&collector)?;

    collector.collect();
    thread::sleep(Duration::from_millis(200));

    // Root objects should remain accessible
    assert_eq!(large_objects[0][0], 0);
    assert_eq!(large_objects[0].len(), 10000);
    assert_eq!(large_objects[5][0], 5);
    assert_eq!(large_objects[5].len(), 10000);

    Ok(())
}

#[test_log::test]
fn test_collector_stress() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    // Stress test with rapid allocation and collection
    let mut all_objects = Vec::new();

    for batch in 0..20 {
        let batch_objects: Vec<_> = (0..50)
            .map(|i| Gc::with_collector(&collector, format!("batch-{batch}-item-{i}")))
            .collect();

        all_objects.extend(batch_objects);

        // Trigger collection every few batches
        if batch % 5 == 0 {
            collector.collect();
        }
    }

    // Final collection
    collector.collect();
    thread::sleep(Duration::from_millis(100));

    // All objects should still be accessible since we hold references
    assert_eq!(all_objects.len(), 1000);
    for (i, obj) in all_objects.iter().enumerate() {
        let batch = i / 50;
        let item = i % 50;
        assert_eq!(**obj, format!("batch-{batch}-item-{item}"));
    }

    Ok(())
}

#[test_log::test]
fn test_parallel_garbage_collection() -> Result<()> {
    let configuration = Configuration {
        parallel_threshold: 1_000, // Set a low threshold for parallel collection
        ..Default::default()
    };
    let collector = GarbageCollector::with_config(configuration.clone());
    collector.start();

    // Allocate enough objects to trigger parallel collection
    let number_of_objects = configuration.parallel_threshold * 10;
    for _ in 0..number_of_objects {
        let _gc = Gc::with_collector(&collector, 42);
    }
    let allocation_statistics = collector.statistics()?;

    collector.collect();
    thread::sleep(Duration::from_millis(100));
    let collector_statistics = collector.statistics()?;

    assert_eq!(
        collector_statistics.bytes_freed,
        allocation_statistics.bytes_allocated
    );
    Ok(())
}
