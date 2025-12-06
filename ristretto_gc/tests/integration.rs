//! Integration and edge case tests
//!
//! Tests complex scenarios, edge cases, and integration between different components.

use ristretto_gc::{Configuration, GarbageCollector, Gc, GcRootGuard, Result, Trace};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

#[test_log::test]
fn test_empty_collections() {
    let collector = GarbageCollector::new();
    collector.start();

    // Test behavior with empty collections
    let empty_vec: Vec<Gc<i32>> = vec![];
    let gc_empty_vec = Gc::with_collector(&collector, empty_vec);
    gc_empty_vec.trace(&collector);

    let none_option: Option<Gc<String>> = None;
    none_option.trace(&collector);

    assert_eq!(gc_empty_vec.len(), 0);
}

#[test_log::test]
fn test_zero_sized_types() {
    let collector = GarbageCollector::new();
    collector.start();

    // Test with zero-sized types
    #[derive(Debug, PartialEq)]
    struct ZeroSized;

    impl Trace for ZeroSized {
        fn trace(&self, _collector: &GarbageCollector) {
            // Nothing to trace
        }
    }

    let gc_zst = Gc::with_collector(&collector, ZeroSized);
    let gc_zst_clone = gc_zst.clone();

    assert_eq!(**gc_zst, ZeroSized);
    assert!(Gc::ptr_eq(&gc_zst, &gc_zst_clone));
}

#[test_log::test]
fn test_large_objects() {
    let collector = GarbageCollector::new();
    collector.start();

    // Test with larger objects
    let large_vec = Gc::with_collector(&collector, vec![42u64; 10000]);
    let large_string = Gc::with_collector(&collector, "x".repeat(10000));

    assert_eq!(large_vec.len(), 10000);
    assert_eq!(large_string.len(), 10000);

    // Should work with cloning - all clones point to same object in reachability analysis
    let large_vec_clone = large_vec.clone();
    assert!(Gc::ptr_eq(&large_vec, &large_vec_clone));
}

#[test_log::test]
fn test_deeply_nested_structures() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    #[derive(Debug)]
    #[expect(dead_code)]
    enum NestedEnum {
        Leaf(i32),
        Branch {
            left: Gc<NestedEnum>,
            right: Gc<NestedEnum>,
            value: String,
        },
    }

    impl Trace for NestedEnum {
        fn trace(&self, collector: &GarbageCollector) {
            match self {
                NestedEnum::Leaf(_) => {}
                NestedEnum::Branch { left, right, .. } => {
                    left.trace(collector);
                    right.trace(collector);
                }
            }
        }
    }

    // Create a nested binary tree structure
    let leaf1 = Gc::with_collector(&collector, NestedEnum::Leaf(1));
    let leaf2 = Gc::with_collector(&collector, NestedEnum::Leaf(2));
    let leaf3 = Gc::with_collector(&collector, NestedEnum::Leaf(3));
    let leaf4 = Gc::with_collector(&collector, NestedEnum::Leaf(4));

    let branch1 = Gc::with_collector(
        &collector,
        NestedEnum::Branch {
            left: leaf1.clone_gc(),
            right: leaf2.clone_gc(),
            value: "branch1".to_string(),
        },
    );

    let branch2 = Gc::with_collector(
        &collector,
        NestedEnum::Branch {
            left: leaf3.clone_gc(),
            right: leaf4.clone_gc(),
            value: "branch2".to_string(),
        },
    );

    let root = Gc::with_collector(
        &collector,
        NestedEnum::Branch {
            left: branch1.clone_gc(),
            right: branch2.clone_gc(),
            value: "root".to_string(),
        },
    );

    // Add the root as a GC root to ensure reachability
    root.as_root(&collector)?;

    // Verify the structure is properly accessible
    match &**root {
        NestedEnum::Branch { value, .. } => assert_eq!(value, "root"),
        _ => panic!("Expected root to be a branch"),
    }

    Ok(())
}

#[test_log::test]
fn test_concurrent_allocation_and_collection() {
    let collector = GarbageCollector::new();
    collector.start();

    static ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);

    let mut handles = vec![];

    // Spawn multiple threads that allocate objects concurrently
    for thread_id in 0..4 {
        let collector = collector.clone();
        let handle = thread::spawn(move || {
            for i in 0..100 {
                let data = format!("thread-{thread_id}-object-{i}");
                let _gc = Gc::with_collector(&collector, data);
                ALLOCATION_COUNT.fetch_add(1, Ordering::Relaxed);

                // Occasionally trigger collection
                if i % 20 == 0 {
                    collector.collect();
                }

                // Small delay to allow concurrent execution
                thread::sleep(Duration::from_millis(1));
            }
        });
        handles.push(handle);
    }

    // Wait for all allocation threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Trigger final collection
    collector.collect();
    thread::sleep(Duration::from_millis(100));

    // Verify that we allocated the expected number of objects
    assert_eq!(ALLOCATION_COUNT.load(Ordering::Relaxed), 400);
}

#[test_log::test]
fn test_stress_test_rapid_allocation() {
    let collector = GarbageCollector::new();
    collector.start();

    // Rapid allocation stress test
    let mut objects = Vec::new();

    for i in 0..1000 {
        let data = vec![i; 100]; // Each object contains 100 elements
        let gc_object = Gc::with_collector(&collector, data);
        objects.push(gc_object);

        // Trigger collection periodically
        if i % 100 == 0 {
            collector.collect();
        }
    }

    // Verify all objects are still accessible
    for (i, obj) in objects.iter().enumerate() {
        assert_eq!(obj[0], i);
        assert_eq!(obj.len(), 100);
    }

    // Final collection
    collector.collect();
    thread::sleep(Duration::from_millis(50));
}

#[test_log::test]
fn test_collector_configuration() -> Result<()> {
    // Test custom collector configuration
    let config = Configuration {
        allocation_threshold: 1024 * 1024, // 1MB
        ..Default::default()
    };

    let collector = GarbageCollector::with_config(config);
    collector.start();

    // Create some objects with the custom collector
    let gc1 = Gc::with_collector(&collector, "test1".to_string());
    let gc2 = Gc::with_collector(&collector, "test2".to_string());

    assert_eq!(**gc1, "test1");
    assert_eq!(**gc2, "test2");
    Ok(())
}

#[test_log::test]
fn test_gc_with_collections() {
    let collector = GarbageCollector::new();
    collector.start();

    let mut map = HashMap::new();
    map.insert("key1", Gc::with_collector(&collector, 42).clone_gc());
    map.insert("key2", Gc::with_collector(&collector, 84).clone_gc());

    let gc_map = Gc::with_collector(&collector, map);

    // Verify map operations work
    assert_eq!(**gc_map.get("key1").unwrap(), 42);
    assert_eq!(**gc_map.get("key2").unwrap(), 84);

    let mut set = HashSet::new();
    set.insert(Gc::with_collector(&collector, "item1".to_string()).clone_gc());
    set.insert(Gc::with_collector(&collector, "item2".to_string()).clone_gc());

    let gc_set = Gc::with_collector(&collector, set);
    assert_eq!(gc_set.len(), 2);
}

#[test_log::test]
fn test_gc_statistics() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    // Test that GC statistics are properly maintained
    let initial_stats = collector.statistics()?;

    // Allocate some objects
    let _objects: Vec<_> = (0..10).map(Gc::new).collect();

    // Trigger collection
    collector.collect();
    thread::sleep(Duration::from_millis(100));

    let final_stats = collector.statistics()?;

    // Stats should show some activity
    assert!(final_stats.collections_started >= initial_stats.collections_started);
    assert!(final_stats.bytes_allocated >= initial_stats.bytes_allocated);

    Ok(())
}

#[test_log::test]
fn test_edge_case_empty_objects() {
    let collector = GarbageCollector::new();
    collector.start();

    // Test with empty objects and containers
    let empty_string = Gc::with_collector(&collector, String::new());
    let empty_vec: GcRootGuard<Vec<i32>> = Gc::with_collector(&collector, Vec::new());

    assert_eq!(**empty_string, "");
    assert_eq!(empty_vec.len(), 0);

    // These should work without issues
    let _empty_clone = empty_string.clone();
    assert!(Gc::ptr_eq(&empty_string, &_empty_clone));
}

#[test_log::test]
fn test_complex_reachability_scenario() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    // Test complex reachability patterns
    struct ComplexObject {
        id: usize,
        references: Vec<Gc<ComplexObject>>,
        optional_ref: Option<Gc<ComplexObject>>,
    }

    impl Trace for ComplexObject {
        fn trace(&self, collector: &GarbageCollector) {
            for reference in &self.references {
                reference.trace(collector);
            }
            if let Some(ref optional) = self.optional_ref {
                optional.trace(collector);
            }
        }
    }

    // Create a complex graph of objects
    let obj1 = Gc::with_collector(
        &collector,
        ComplexObject {
            id: 1,
            references: Vec::new(),
            optional_ref: None,
        },
    );

    let obj2 = Gc::with_collector(
        &collector,
        ComplexObject {
            id: 2,
            references: vec![obj1.clone_gc()],
            optional_ref: None,
        },
    );

    let obj3 = Gc::with_collector(
        &collector,
        ComplexObject {
            id: 3,
            references: vec![obj1.clone_gc(), obj2.clone_gc()],
            optional_ref: Some(obj1.clone_gc()),
        },
    );

    // Add one as root to keep the graph reachable
    obj3.as_root(&collector)?;

    // Verify the structure
    assert_eq!(obj1.id, 1);
    assert_eq!(obj2.id, 2);
    assert_eq!(obj3.id, 3);
    assert_eq!(obj3.references.len(), 2);
    assert!(obj3.optional_ref.is_some());

    Ok(())
}
