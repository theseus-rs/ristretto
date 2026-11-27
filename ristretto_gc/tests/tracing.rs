//! `GcTrace` trait and object tracing tests
//!
//! Tests custom `GcTrace` implementations, object graphs, and tracing correctness.
#![cfg(not(target_family = "wasm"))]
#![allow(unsafe_code)]
#![expect(
    clippy::indexing_slicing,
    clippy::items_after_statements,
    clippy::panic_in_result_fn,
    reason = "integration tests use local fixture types, direct fixture indexing, and assertions with Result-returning tests"
)]

use ristretto_gc::cell::RwLock;
use ristretto_gc::{GarbageCollector, Gc, GcRootGuard, Result, Trace};
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
fn test_option_trace() {
    #[derive(Debug)]
    struct TestNode {
        value: i32,
        next: Option<Gc<TestNode>>,
    }

    // SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for TestNode {
        fn root(&mut self) {
            self.next.root();
        }
        unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
            unsafe { Trace::unroot(&mut self.next, owner) };
        }
        fn trace(&self, collector: &GarbageCollector) {
            self.next.trace(collector);
        }
    }

    let collector = GarbageCollector::new();
    collector.start();

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
            next: Some(node1.clone_gc()),
        },
    );

    // Should trace without issues
    node2.trace(&collector);
    assert_eq!(node2.value, 2);
    assert_eq!(node1.value, 1);
}

#[test_log::test]
fn test_vec_trace() {
    #[derive(Debug)]
    struct Container {
        items: Vec<Gc<i32>>,
    }

    // SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for Container {
        fn root(&mut self) {
            self.items.root();
        }
        unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
            unsafe { Trace::unroot(&mut self.items, owner) };
        }
        fn trace(&self, collector: &GarbageCollector) {
            self.items.trace(collector);
        }
    }

    let collector = GarbageCollector::new();
    collector.start();

    let item1 = Gc::with_collector(&collector, 10);
    let item2 = Gc::with_collector(&collector, 20);
    let item3 = Gc::with_collector(&collector, 30);

    let container = Gc::with_collector(
        &collector,
        Container {
            items: vec![item1.clone_gc(), item2.clone_gc(), item3.clone_gc()],
        },
    );

    // Should trace all items in the vector
    container.trace(&collector);
    assert_eq!(container.items.len(), 3);
}

#[test_log::test]
fn test_custom_trace_implementation() -> Result<()> {
    #[derive(Debug)]
    struct TreeNode {
        value: String,
        children: Vec<Gc<RwLock<TreeNode>>>,
        parent: Option<Gc<RwLock<TreeNode>>>,
    }

    // SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for TreeNode {
        fn root(&mut self) {
            self.children.root();
            self.parent.root();
        }
        unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
            unsafe { Trace::unroot(&mut self.children, owner) };
            unsafe { Trace::unroot(&mut self.parent, owner) };
        }
        fn trace(&self, collector: &GarbageCollector) {
            // Trace all children
            for child in &self.children {
                child.trace(collector);
            }
            self.parent.trace(collector);
        }
    }

    let collector = GarbageCollector::new();
    collector.start();

    let root = Gc::with_collector(
        &collector,
        RwLock::new(TreeNode {
            value: "root".to_string(),
            children: Vec::new(),
            parent: None,
        }),
    );

    let child1 = Gc::with_collector(
        &collector,
        RwLock::new(TreeNode {
            value: "child1".to_string(),
            children: Vec::new(),
            parent: Some(root.clone_gc()),
        }),
    );

    let child2 = Gc::with_collector(
        &collector,
        RwLock::new(TreeNode {
            value: "child2".to_string(),
            children: Vec::new(),
            parent: Some(root.clone_gc()),
        }),
    );

    root.write()
        .children
        .extend([child1.clone_gc(), child2.clone_gc()]);

    // Add root to keep tree reachable
    root.as_root(&collector)?;

    // Should trace the entire tree structure
    root.trace(&collector);
    assert_eq!(root.read().value, "root");
    assert_eq!(root.read().children.len(), 2);

    drop((root, child1, child2));
    collector.stop()?;
    assert_eq!(collector.managed_bytes(), 0);
    Ok(())
}

#[test_log::test]
fn test_complex_nested_trace() -> Result<()> {
    #[derive(Debug)]
    struct GraphNode {
        id: usize,
        neighbors: Vec<Gc<RwLock<GraphNode>>>,
    }

    // SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for GraphNode {
        fn root(&mut self) {
            self.neighbors.root();
        }
        unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
            unsafe { Trace::unroot(&mut self.neighbors, owner) };
        }
        fn trace(&self, collector: &GarbageCollector) {
            for neighbor in &self.neighbors {
                neighbor.trace(collector);
            }
        }
    }

    let collector = GarbageCollector::new();
    collector.start();

    // Create a simple graph with cycles
    let node_a = Gc::with_collector(
        &collector,
        RwLock::new(GraphNode {
            id: 1,
            neighbors: Vec::new(),
        }),
    );

    let node_b = Gc::with_collector(
        &collector,
        RwLock::new(GraphNode {
            id: 2,
            neighbors: Vec::new(),
        }),
    );

    node_a.write().neighbors.push(node_b.clone_gc());
    node_b.write().neighbors.push(node_a.clone_gc());

    // Add one as root to keep graph reachable
    node_a.as_root(&collector)?;

    // Tracing should handle the cycle properly due to the marking mechanism
    node_a.trace(&collector);
    assert_eq!(node_a.read().id, 1);
    assert_eq!(node_b.read().id, 2);

    drop((node_a, node_b));
    collector.stop()?;
    assert_eq!(collector.managed_bytes(), 0);
    Ok(())
}

#[test_log::test]
fn test_mixed_types_trace() {
    #[derive(Debug)]
    struct MixedContainer {
        number: Gc<i32>,
        text: Gc<String>,
        optional: Option<Gc<f64>>,
        list: Vec<Gc<bool>>,
    }

    // SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for MixedContainer {
        fn root(&mut self) {
            self.number.root();
            self.text.root();
            self.optional.root();
            self.list.root();
        }
        unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
            unsafe { Trace::unroot(&mut self.number, owner) };
            unsafe { Trace::unroot(&mut self.text, owner) };
            unsafe { Trace::unroot(&mut self.optional, owner) };
            unsafe { Trace::unroot(&mut self.list, owner) };
        }
        fn trace(&self, collector: &GarbageCollector) {
            self.number.trace(collector);
            self.text.trace(collector);
            self.optional.trace(collector);
            self.list.trace(collector);
        }
    }

    let collector = GarbageCollector::new();
    collector.start();

    let container = Gc::with_collector(
        &collector,
        MixedContainer {
            number: Gc::with_collector(&collector, 42).clone_gc(),
            text: Gc::with_collector(&collector, "hello".to_string()).clone_gc(),
            optional: Some(Gc::with_collector(&collector, 1.23).clone_gc()),
            list: vec![
                Gc::with_collector(&collector, true).clone_gc(),
                Gc::with_collector(&collector, false).clone_gc(),
            ],
        },
    );

    // Should trace all nested Gc objects
    container.trace(&collector);
    assert_eq!(*container.number, 42);
    assert_eq!(*container.text, "hello");
    assert!((**container.optional.as_ref().unwrap() - 1.23f64).abs() < f64::EPSILON);
    assert_eq!(container.list.len(), 2);
}

#[test_log::test]
fn test_deep_nesting_trace() {
    #[derive(Debug)]
    struct Nested {
        level: usize,
        inner: Option<Gc<Nested>>,
    }

    // SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for Nested {
        fn root(&mut self) {
            self.inner.root();
        }
        unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
            unsafe { Trace::unroot(&mut self.inner, owner) };
        }
        fn trace(&self, collector: &GarbageCollector) {
            if let Some(ref inner) = self.inner {
                inner.trace(collector);
            }
        }
    }

    let collector = GarbageCollector::new();
    collector.start();

    // Create deeply nested structure
    let mut current: Option<GcRootGuard<Nested>> = None;
    for level in (0..10).rev() {
        let inner = current.as_ref().map(GcRootGuard::clone_gc);
        current = Some(Gc::with_collector(&collector, Nested { level, inner }));
    }

    let root = current.unwrap();

    // Should trace the entire chain
    root.trace(&collector);
    assert_eq!(root.level, 0);

    // Verify the nested structure
    let mut current_node: &Nested = &root;
    for expected_level in 0..10 {
        assert_eq!(current_node.level, expected_level);
        if let Some(ref inner) = current_node.inner {
            current_node = inner;
        } else {
            assert_eq!(expected_level, 9); // Should be the last level
        }
    }
}

#[test_log::test]
fn test_trace_with_collections() {
    #[derive(Debug)]
    struct CollectionContainer {
        map: HashMap<String, Gc<i32>>,
        set: HashSet<Gc<String>>,
    }

    // SAFETY: outgoing fields remain independently rooted and are included in tracing.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for CollectionContainer {
        fn trace(&self, collector: &GarbageCollector) {
            for value in self.map.values() {
                value.trace(collector);
            }
            for item in &self.set {
                item.trace(collector);
            }
        }
    }

    let collector = GarbageCollector::new();
    collector.start();

    let mut map = HashMap::new();
    map.insert(
        "one".to_string(),
        Gc::with_collector(&collector, 1).clone_gc(),
    );
    map.insert(
        "two".to_string(),
        Gc::with_collector(&collector, 2).clone_gc(),
    );

    let mut set = HashSet::new();
    set.insert(Gc::with_collector(&collector, "hello".to_string()).clone_gc());
    set.insert(Gc::with_collector(&collector, "world".to_string()).clone_gc());

    let container = Gc::with_collector(&collector, CollectionContainer { map, set });

    // Should trace all items in collections
    container.trace(&collector);
    assert_eq!(container.map.len(), 2);
    assert_eq!(container.set.len(), 2);
}

#[test_log::test]
fn test_circular_reference_trace() -> Result<()> {
    #[derive(Debug)]
    struct CircularNode {
        id: usize,
        partner: Option<Gc<RwLock<CircularNode>>>,
    }

    // SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for CircularNode {
        fn root(&mut self) {
            self.partner.root();
        }
        unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
            unsafe { Trace::unroot(&mut self.partner, owner) };
        }
        fn trace(&self, collector: &GarbageCollector) {
            if let Some(ref partner) = self.partner {
                partner.trace(collector);
            }
        }
    }

    let collector = GarbageCollector::new();
    collector.start();

    let node_a = Gc::with_collector(
        &collector,
        RwLock::new(CircularNode {
            id: 1,
            partner: None,
        }),
    );

    let node_b = Gc::with_collector(
        &collector,
        RwLock::new(CircularNode {
            id: 2,
            partner: Some(node_a.clone_gc()),
        }),
    );

    node_a.write().partner = Some(node_b.clone_gc());

    // Add as root
    node_a.as_root(&collector)?;

    // Should handle circular tracing properly
    node_a.trace(&collector);
    assert_eq!(node_a.read().id, 1);
    assert_eq!(node_b.read().id, 2);

    drop((node_a, node_b));
    collector.stop()?;
    assert_eq!(collector.managed_bytes(), 0);
    Ok(())
}

#[test_log::test]
fn test_trace_performance() {
    #[derive(Debug)]
    struct PerfNode {
        id: usize,
        children: Vec<Gc<PerfNode>>,
    }

    // SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for PerfNode {
        fn root(&mut self) {
            self.children.root();
        }
        unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
            unsafe { Trace::unroot(&mut self.children, owner) };
        }
        fn trace(&self, collector: &GarbageCollector) {
            self.children.trace(collector);
        }
    }

    let collector = GarbageCollector::new();
    collector.start();

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
        level1_nodes.push(
            Gc::with_collector(
                &collector,
                PerfNode {
                    id: 100 + i,
                    children: level2_nodes[start..end]
                        .iter()
                        .map(GcRootGuard::clone_gc)
                        .collect(),
                },
            )
            .clone_gc(),
        );
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

#[test]
fn reference_free_buffers_do_not_walk_primitive_elements_but_drop_values_in_batches() {
    use ristretto_gc::{GarbageCollector, HeapBuffer, TraceProgress};
    use std::sync::atomic::{AtomicUsize, Ordering};
    let gc = GarbageCollector::new();
    let mut bytes: HeapBuffer<u8> = vec![0; 1024 * 1024].into();
    assert_eq!(bytes.trace_work(), 1);
    assert_eq!(bytes.trace_step(&gc, 0, 1), TraceProgress::Complete);
    assert_eq!(bytes.clear_step(0, 1), TraceProgress::Complete);
    assert!(bytes.is_empty());
    let mut words = vec![0_u64; 1024 * 1024];
    assert_eq!(words.trace_step(&gc, 0, 1), TraceProgress::Complete);
    assert_eq!(words.clear_step(0, 1), TraceProgress::Complete);
    assert_eq!(words, [] as [u64; 0]);
    struct Dropped(std::sync::Arc<AtomicUsize>);
    // SAFETY: this fixture contains no outgoing GC references.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for Dropped {
        fn may_contain_gc_references() -> bool {
            false
        }
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for Dropped {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    let drops = std::sync::Arc::new(AtomicUsize::new(0));
    let mut values: HeapBuffer<_> = (0..10).map(|_| Dropped(drops.clone())).collect();
    assert_eq!(values.trace_step(&gc, 0, 1), TraceProgress::Complete);
    assert_eq!(values.clear_step(0, 2), TraceProgress::Continue(0));
    assert_eq!(drops.load(Ordering::Relaxed), 2);
    drop(values);
    assert_eq!(drops.load(Ordering::Relaxed), 10);
}
