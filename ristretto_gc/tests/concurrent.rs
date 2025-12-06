//! Concurrent garbage collection tests
//!
//! Tests concurrent collection behavior, thread safety, and performance under concurrent load.

use ristretto_gc::{GarbageCollector, Gc, Result, Trace};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[test_log::test]
fn test_concurrent_allocation() {
    let collector = GarbageCollector::new();
    collector.start();

    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let thread_count = 4;
    let allocations_per_thread = 100;

    let mut handles = vec![];

    for thread_id in 0..thread_count {
        let collector = collector.clone();
        let handle = thread::spawn(move || {
            for i in 0..allocations_per_thread {
                let data = format!("thread-{thread_id}-{i}");
                let _gc = Gc::with_collector(&collector, data);
                COUNTER.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(
        COUNTER.load(Ordering::Relaxed),
        thread_count * allocations_per_thread
    );
}

#[test_log::test]
fn test_concurrent_collection_triggers() {
    let collector = GarbageCollector::new();
    collector.start();

    let object_count = 1000;
    let gc_objects: Vec<_> = (0..object_count)
        .map(|value| Gc::with_collector(&collector, value))
        .collect();

    let mut handles = vec![];

    // Spawn threads that trigger collection
    for _ in 0..3 {
        let collector = collector.clone();
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                collector.collect();
                thread::sleep(Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }

    // Spawn threads that access objects
    for thread_id in 0..2 {
        let objects = gc_objects.clone();
        let handle = thread::spawn(move || {
            for i in 0..100 {
                let index = (thread_id * 100 + i) % objects.len();
                let _value = **objects[index];
                thread::sleep(Duration::from_millis(5));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // All objects should still be accessible
    for (i, obj) in gc_objects.iter().enumerate() {
        assert_eq!(***obj, i);
    }
}

#[test_log::test]
fn test_stress_concurrent_gc() {
    let collector = GarbageCollector::new();
    collector.start();

    let duration = Duration::from_millis(500);
    let start_time = Instant::now();

    let allocation_counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    // Allocator threads
    for thread_id in 0..3 {
        let collector = collector.clone();
        let counter = Arc::clone(&allocation_counter);
        let handle = thread::spawn(move || {
            let mut local_objects = Vec::new();
            let mut iteration = 0;

            while start_time.elapsed() < duration {
                // Allocate some objects
                for i in 0..20 {
                    let data = format!("t{thread_id}-i{iteration}-o{i}");
                    local_objects.push(Gc::with_collector(&collector, data));
                    counter.fetch_add(1, Ordering::Relaxed);
                }

                // Clear some objects (making them unreachable)
                if local_objects.len() > 100 {
                    local_objects.truncate(50);
                }

                iteration += 1;
                thread::sleep(Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }

    // GC trigger thread
    let gc_handle = thread::spawn(move || {
        while start_time.elapsed() < duration {
            collector.collect();
            thread::sleep(Duration::from_millis(30));
        }
    });

    for handle in handles {
        handle.join().unwrap();
    }
    gc_handle.join().unwrap();

    let total_allocations = allocation_counter.load(Ordering::Relaxed);
    assert!(total_allocations > 0, "Should have allocated some objects");
}

#[test_log::test]
fn test_concurrent_roots_management() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    #[derive(Debug)]
    struct RootObject {
        id: usize,
        child: Option<Gc<RootObject>>,
    }

    impl Trace for RootObject {
        fn trace(&self, collector: &GarbageCollector) {
            if let Some(ref child) = self.child {
                child.trace(collector);
            }
        }
    }

    let objects: Vec<_> = (0..5)
        .map(|i| Gc::with_collector(&collector, RootObject { id: i, child: None }))
        .collect();

    let mut handles = vec![];

    // Threads that add/remove roots concurrently
    for i in 0..3 {
        let collector = collector.clone();
        let thread_objects = objects.clone();
        let handle = thread::spawn(move || -> Result<()> {
            for j in 0..10 {
                let obj_idx = (i + j) % thread_objects.len();
                thread_objects[obj_idx]
                    .as_root(&collector)
                    .expect("root failed");
                thread::sleep(Duration::from_millis(5));
            }
            Ok(())
        });
        handles.push(handle);
    }

    // Collection thread
    let collection_handle = thread::spawn(move || {
        for _ in 0..15 {
            collector.collect();
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap()?;
    }
    collection_handle.join().unwrap();

    // All objects should remain accessible
    for object in &objects {
        assert!(object.id < 5);
    }

    Ok(())
}

#[test_log::test]
fn test_concurrent_complex_graph() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    #[derive(Debug)]
    struct GraphNode {
        id: usize,
        connections: Mutex<Vec<Gc<GraphNode>>>,
    }

    impl Trace for GraphNode {
        fn trace(&self, collector: &GarbageCollector) {
            if let Ok(connections) = self.connections.lock() {
                connections.trace(collector);
            }
        }
    }

    // Create a set of nodes
    let nodes: Vec<_> = (0..20)
        .map(|i| {
            Gc::with_collector(
                &collector,
                GraphNode {
                    id: i,
                    connections: Mutex::new(Vec::new()),
                },
            )
        })
        .collect();

    // Add some nodes as roots
    nodes[0].as_root(&collector)?;
    nodes[10].as_root(&collector)?;

    let mut handles = vec![];

    // Threads that create connections between nodes
    for thread_id in 0..4 {
        let thread_nodes = nodes.clone();
        let handle = thread::spawn(move || {
            for i in 0..30 {
                let from_idx = (thread_id * 30 + i) % thread_nodes.len();
                let to_idx = (from_idx + thread_id + 1) % thread_nodes.len();

                if let Ok(mut connections) = thread_nodes[from_idx].connections.lock() {
                    connections.push(thread_nodes[to_idx].clone_gc());
                }

                thread::sleep(Duration::from_millis(2));
            }
        });
        handles.push(handle);
    }

    // Background collection
    let gc_handle = thread::spawn(move || {
        for _ in 0..25 {
            collector.collect();
            thread::sleep(Duration::from_millis(12));
        }
    });

    for handle in handles {
        handle.join().unwrap();
    }
    gc_handle.join().unwrap();

    // Verify graph integrity
    for node in &nodes {
        assert!(node.id < 20);
        // Check that connections are accessible
        if let Ok(connections) = node.connections.lock() {
            for connection in connections.iter() {
                assert!(connection.id < 20);
            }
        }
    }

    Ok(())
}

#[test_log::test]
fn test_high_frequency_allocation_deallocation() {
    let collector = GarbageCollector::new();
    collector.start();

    let iterations = 100;
    let objects_per_iteration = 50;

    let mut handles = vec![];

    for thread_id in 0..3 {
        let collector = collector.clone();
        let handle = thread::spawn(move || {
            for iteration in 0..iterations {
                let mut local_objects = Vec::new();

                // Rapid allocation
                for i in 0..objects_per_iteration {
                    let data = vec![thread_id, iteration, i];
                    local_objects.push(Gc::with_collector(&collector, data));
                }

                // Use the objects briefly
                for object in &local_objects {
                    let _sum: usize = object.iter().sum();
                }

                // Objects become unreachable when local_objects is dropped

                // Occasional GC trigger
                if iteration % 20 == 0 {
                    collector.collect();
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Final collection
    collector.collect();
    thread::sleep(Duration::from_millis(100));
}

#[test_log::test]
fn test_concurrent_statistics() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();
    let initial_stats = collector.statistics()?;

    let mut handles = vec![];

    // Threads that allocate and trigger collection
    for thread_id in 0..2 {
        let collector = collector.clone();
        let handle = thread::spawn(move || -> Result<()> {
            for i in 0..20 {
                let _objects: Vec<_> = (0..10)
                    .map(|j| Gc::with_collector(&collector, format!("t{thread_id}-i{i}-o{j}")))
                    .collect();

                if i % 5 == 0 {
                    collector.collect();
                }

                thread::sleep(Duration::from_millis(10));
            }
            Ok(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()?;
    }

    let final_stats = collector.statistics()?;

    // Should show increased activity
    assert!(final_stats.collections_started >= initial_stats.collections_started);
    assert!(final_stats.bytes_allocated >= initial_stats.bytes_allocated);

    Ok(())
}
