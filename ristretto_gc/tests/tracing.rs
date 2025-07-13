//! GcTrace trait and object tracing tests
//!
//! Tests custom GcTrace implementations, object graphs, and tracing correctness.

use ristretto_gc::{GarbageCollector, Gc, Result, Trace};
use std::collections::{HashMap, HashSet};

#[test_log::test]
fn test_primitive_types_trace() {
    let collector = GarbageCollector::new();
    collector.start();

    // Test that primitive types implement GcTrace with no-op
    let gc_u32 = Gc::with_collector(&collector, 42u32);
    let gc_string = Gc::with_collector(&collector, "test".to_string());
    let gc_bool = Gc::with_collector(&collector, true);
    let gc_float = Gc::with_collector(&collector, 1.23f64);

    // These should not panic when traced
    gc_u32.trace(&collector);
    gc_string.trace(&collector);
    gc_bool.trace(&collector);
    gc_float.trace(&collector);
}

#[test_log::test]
fn test_option_trace() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    #[derive(Debug)]
    struct TestNode {
        value: i32,
        next: Option<Gc<TestNode>>,
    }

    impl Trace for TestNode {
        fn trace(&self, collector: &GarbageCollector) {
            self.next.trace(collector);
        }
    }

    let node1 = Gc::with_collector(
        &collector,
        TestNode {
            value: 1,
            next: None,
        },
    );
    let node2 = Gc::with_collector(
        &collector,
        TestNode {
            value: 2,
            next: Some(node1.clone()),
        },
    );

    // Should trace without issues
    node2.trace(&collector);
    assert_eq!(node2.value, 2);
    assert_eq!(node1.value, 1);
    Ok(())
}

#[test_log::test]
fn test_vec_trace() {
    let collector = GarbageCollector::new();
    collector.start();

    #[derive(Debug)]
    struct Container {
        items: Vec<Gc<i32>>,
    }

    impl Trace for Container {
        fn trace(&self, collector: &GarbageCollector) {
            self.items.trace(collector);
        }
    }

    let item1 = Gc::with_collector(&collector, 10);
    let item2 = Gc::with_collector(&collector, 20);
    let item3 = Gc::with_collector(&collector, 30);

    let container = Gc::with_collector(
        &collector,
        Container {
            items: vec![item1, item2, item3],
        },
    );

    // Should trace all items in the vector
    container.trace(&collector);
    assert_eq!(container.items.len(), 3);
}

#[test_log::test]
fn test_custom_trace_implementation() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    #[derive(Debug)]
    #[expect(dead_code)]
    struct TreeNode {
        value: String,
        children: Vec<Gc<TreeNode>>,
        parent: Option<Gc<TreeNode>>,
    }

    impl Trace for TreeNode {
        fn trace(&self, collector: &GarbageCollector) {
            // Trace all children
            for child in &self.children {
                child.trace(collector);
            }
            // Note: We don't trace parent to avoid cycles in this simple example
            // In a real implementation, we'd handle cycles properly
        }
    }

    let mut root = Gc::with_collector(
        &collector,
        TreeNode {
            value: "root".to_string(),
            children: Vec::new(),
            parent: None,
        },
    );

    let child1 = Gc::with_collector(
        &collector,
        TreeNode {
            value: "child1".to_string(),
            children: Vec::new(),
            parent: Some(root.clone()),
        },
    );

    let child2 = Gc::with_collector(
        &collector,
        TreeNode {
            value: "child2".to_string(),
            children: Vec::new(),
            parent: Some(root.clone()),
        },
    );

    // Unsafe modification to create tree structure
    // Safety: This is safe because:
    // 1. We have exclusive access to the test environment
    // 2. No other threads are accessing this object
    // 3. This is a controlled test for tree structure tracing
    // 4. The mutation happens before any concurrent access
    unsafe {
        let root_mut = root.get_mut_unchecked();
        root_mut.children.push(child1.clone());
        root_mut.children.push(child2.clone());
    }

    // Add root to keep tree reachable
    root.as_root(&collector)?;

    // Should trace the entire tree structure
    root.trace(&collector);
    assert_eq!(root.value, "root");
    assert_eq!(root.children.len(), 2);

    Ok(())
}

#[test_log::test]
fn test_complex_nested_trace() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    #[derive(Debug)]
    struct GraphNode {
        id: usize,
        neighbors: Vec<Gc<GraphNode>>,
    }

    impl Trace for GraphNode {
        fn trace(&self, collector: &GarbageCollector) {
            for neighbor in &self.neighbors {
                neighbor.trace(collector);
            }
        }
    }

    // Create a simple graph with cycles
    let mut node_a = Gc::with_collector(
        &collector,
        GraphNode {
            id: 1,
            neighbors: Vec::new(),
        },
    );

    let mut node_b = Gc::with_collector(
        &collector,
        GraphNode {
            id: 2,
            neighbors: Vec::new(),
        },
    );

    // Create bidirectional references (cycle)
    // Safety: This is safe because:
    // 1. We have exclusive access to the test environment
    // 2. No other threads are accessing these objects
    // 3. This is a controlled test for graph structure with cycles
    // 4. The mutations happen in a single-threaded test context
    unsafe {
        let a_mut = node_a.get_mut_unchecked();
        a_mut.neighbors.push(node_b.clone());

        let b_mut = node_b.get_mut_unchecked();
        b_mut.neighbors.push(node_a.clone());
    }

    // Add one as root to keep graph reachable
    node_a.as_root(&collector)?;

    // Tracing should handle the cycle properly due to the marking mechanism
    node_a.trace(&collector);
    assert_eq!(node_a.id, 1);
    assert_eq!(node_b.id, 2);

    Ok(())
}

#[test_log::test]
fn test_mixed_types_trace() {
    let collector = GarbageCollector::new();
    collector.start();

    #[derive(Debug)]
    struct MixedContainer {
        number: Gc<i32>,
        text: Gc<String>,
        optional: Option<Gc<f64>>,
        list: Vec<Gc<bool>>,
    }

    impl Trace for MixedContainer {
        fn trace(&self, collector: &GarbageCollector) {
            self.number.trace(collector);
            self.text.trace(collector);
            self.optional.trace(collector);
            self.list.trace(collector);
        }
    }

    let container = Gc::with_collector(
        &collector,
        MixedContainer {
            number: Gc::with_collector(&collector, 42),
            text: Gc::with_collector(&collector, "hello".to_string()),
            optional: Some(Gc::with_collector(&collector, 1.23)),
            list: vec![
                Gc::with_collector(&collector, true),
                Gc::with_collector(&collector, false),
            ],
        },
    );

    // Should trace all nested Gc objects
    container.trace(&collector);
    assert_eq!(*container.number, 42);
    assert_eq!(*container.text, "hello");
    assert_eq!(**container.optional.as_ref().unwrap(), 1.23);
    assert_eq!(container.list.len(), 2);
}

#[test_log::test]
fn test_deep_nesting_trace() {
    let collector = GarbageCollector::new();
    collector.start();

    #[derive(Debug)]
    struct Nested {
        level: usize,
        inner: Option<Gc<Nested>>,
    }

    impl Trace for Nested {
        fn trace(&self, collector: &GarbageCollector) {
            if let Some(ref inner) = self.inner {
                inner.trace(collector);
            }
        }
    }

    // Create deeply nested structure
    let mut current = None;
    for level in (0..10).rev() {
        current = Some(Gc::with_collector(
            &collector,
            Nested {
                level,
                inner: current,
            },
        ));
    }

    let root = current.unwrap();

    // Should trace the entire chain
    root.trace(&collector);
    assert_eq!(root.level, 0);

    // Verify the nested structure
    let mut current_ref = &root;
    for expected_level in 0..10 {
        assert_eq!(current_ref.level, expected_level);
        if let Some(ref inner) = current_ref.inner {
            current_ref = inner;
        } else {
            assert_eq!(expected_level, 9); // Should be the last level
        }
    }
}

#[test_log::test]
fn test_trace_with_collections() {
    let collector = GarbageCollector::new();
    collector.start();

    #[derive(Debug)]
    struct CollectionContainer {
        map: HashMap<String, Gc<i32>>,
        set: HashSet<Gc<String>>,
    }

    impl Trace for CollectionContainer {
        fn trace(&self, collector: &GarbageCollector) {
            for value in self.map.values() {
                value.trace(collector);
            }
            for item in &self.set {
                item.trace(collector);
            }
        }
    }

    let mut map = HashMap::new();
    map.insert("one".to_string(), Gc::with_collector(&collector, 1));
    map.insert("two".to_string(), Gc::with_collector(&collector, 2));

    let mut set = HashSet::new();
    set.insert(Gc::with_collector(&collector, "hello".to_string()));
    set.insert(Gc::with_collector(&collector, "world".to_string()));

    let container = Gc::with_collector(&collector, CollectionContainer { map, set });

    // Should trace all items in collections
    container.trace(&collector);
    assert_eq!(container.map.len(), 2);
    assert_eq!(container.set.len(), 2);
}

#[test_log::test]
fn test_circular_reference_trace() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    #[derive(Debug)]
    struct CircularNode {
        id: usize,
        partner: Option<Gc<CircularNode>>,
    }

    impl Trace for CircularNode {
        fn trace(&self, collector: &GarbageCollector) {
            if let Some(ref partner) = self.partner {
                partner.trace(collector);
            }
        }
    }

    let mut node_a = Gc::with_collector(
        &collector,
        CircularNode {
            id: 1,
            partner: None,
        },
    );

    let node_b = Gc::with_collector(
        &collector,
        CircularNode {
            id: 2,
            partner: Some(node_a.clone()),
        },
    );

    // Create circular reference
    // Safety: This is safe because:
    // 1. We have exclusive access to the test environment
    // 2. No other threads are accessing this object
    // 3. This is a controlled test for circular reference tracing
    // 4. The mutation happens in a single-threaded test context
    unsafe {
        let a_mut = node_a.get_mut_unchecked();
        a_mut.partner = Some(node_b.clone());
    }

    // Add as root
    node_a.as_root(&collector)?;

    // Should handle circular tracing properly
    node_a.trace(&collector);
    assert_eq!(node_a.id, 1);
    assert_eq!(node_b.id, 2);

    Ok(())
}

#[test_log::test]
fn test_trace_performance() {
    let collector = GarbageCollector::new();
    collector.start();

    #[derive(Debug)]
    #[expect(dead_code)]
    struct PerfNode {
        id: usize,
        children: Vec<Gc<PerfNode>>,
    }

    impl Trace for PerfNode {
        fn trace(&self, collector: &GarbageCollector) {
            self.children.trace(collector);
        }
    }

    // Create a wide tree (10 children per node, 3 levels deep)
    let mut level2_nodes = Vec::new();
    for i in 0..100 {
        level2_nodes.push(Gc::with_collector(
            &collector,
            PerfNode {
                id: 200 + i,
                children: vec![],
            },
        ));
    }

    let mut level1_nodes = Vec::new();
    for i in 0..10 {
        let start = i * 10;
        let end = start + 10;
        level1_nodes.push(Gc::with_collector(
            &collector,
            PerfNode {
                id: 100 + i,
                children: level2_nodes[start..end].to_vec(),
            },
        ));
    }

    let root = Gc::with_collector(
        &collector,
        PerfNode {
            id: 0,
            children: level1_nodes,
        },
    );

    // Should be able to trace efficiently
    let start = std::time::Instant::now();
    root.trace(&collector);
    let duration = start.elapsed();

    // Should complete reasonably quickly (this is a rough check)
    assert!(
        duration.as_millis() < 100,
        "Tracing took too long: {duration:?}"
    );
}
