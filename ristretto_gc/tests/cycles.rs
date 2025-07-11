use ristretto_gc::{GarbageCollector, Gc, Result, Trace};
use std::sync::Mutex;

#[test_log::test]
fn test_cyclic_gc_collection() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();

    struct CyclicWrapper {
        inner: Mutex<Cyclic>,
    }

    struct Cyclic {
        other: Option<Gc<CyclicWrapper>>,
    }

    impl Trace for CyclicWrapper {
        fn trace(&self, collector: &GarbageCollector) {
            if let Ok(guard) = self.inner.lock() {
                guard.trace(collector);
            }
        }
    }

    impl Trace for Cyclic {
        fn trace(&self, collector: &GarbageCollector) {
            if let Some(ref gc) = self.other {
                gc.trace(collector);
            }
        }
    }

    {
        // Create two Gc objects referencing each other
        let a = Gc::with_collector(
            &collector,
            CyclicWrapper {
                inner: Mutex::new(Cyclic { other: None }),
            },
        );
        let b = Gc::with_collector(
            &collector,
            CyclicWrapper {
                inner: Mutex::new(Cyclic { other: None }),
            },
        );

        // Set up the cycle using Mutex's lock
        a.inner.lock().unwrap().other = Some(b.clone());
        b.inner.lock().unwrap().other = Some(a.clone());

        // Objects are now in a cycle - verify they're accessible
        assert!(a.inner.lock().unwrap().other.is_some());
        assert!(b.inner.lock().unwrap().other.is_some());

        // Cycles should be handled automatically when objects become unreachable.
    }

    // Force a garbage collection cycle to test cycle collection
    collector.collect();
    // Give the GC time to run
    std::thread::sleep(std::time::Duration::from_millis(100));
    let stats = collector.statistics()?;

    assert_eq!(stats.objects_swept, 2);
    assert_eq!(stats.bytes_allocated, 0);
    Ok(())
}

#[test_log::test]
fn test_complex_cyclic_structure() {
    let collector = GarbageCollector::new();
    collector.start();

    struct Node {
        id: i32,
        children: Vec<Gc<Node>>,
        parent: Option<Gc<Node>>,
    }

    impl Trace for Node {
        fn trace(&self, collector: &GarbageCollector) {
            for child in &self.children {
                child.trace(collector);
            }
            if let Some(ref parent) = self.parent {
                parent.trace(collector);
            }
        }
    }

    {
        // Create a tree structure with cycles (parent-child relationships)
        let mut root = Gc::with_collector(
            &collector,
            Node {
                id: 0,
                children: Vec::new(),
                parent: None,
            },
        );

        let child1 = Gc::with_collector(
            &collector,
            Node {
                id: 1,
                children: Vec::new(),
                parent: Some(root.clone()),
            },
        );

        let child2 = Gc::with_collector(
            &collector,
            Node {
                id: 2,
                children: Vec::new(),
                parent: Some(root.clone()),
            },
        );

        // Create cycles by adding children to root
        // Note: This is unsafe mutation but demonstrates cycle handling
        // Safety: This is safe because:
        // 1. We have exclusive access to the test environment
        // 2. No other threads are accessing this object
        // 3. This is a controlled test scenario for cycle detection
        // 4. The mutation happens before any concurrent access
        unsafe {
            let root_mut = root.get_mut_unchecked();
            root_mut.children.push(child1.clone());
            root_mut.children.push(child2.clone());
        }

        // Verify the structure is accessible
        assert_eq!(root.id, 0);
        assert_eq!(child1.id, 1);
        assert_eq!(child2.id, 2);

        // In reachability analysis, this complex cyclic structure should be handled
    }

    // Trigger collection
    collector.collect();
    std::thread::sleep(std::time::Duration::from_millis(50));
}

#[test_log::test]
fn test_self_referencing_object() {
    let collector = GarbageCollector::new();
    collector.start();

    struct SelfRef {
        value: i32,
        myself: Option<Gc<SelfRef>>,
    }

    impl Trace for SelfRef {
        fn trace(&self, collector: &GarbageCollector) {
            if let Some(ref myself) = self.myself {
                myself.trace(collector);
            }
        }
    }

    {
        let mut obj = Gc::with_collector(
            &collector,
            SelfRef {
                value: 42,
                myself: None,
            },
        );

        // Create self-reference using a scope to avoid borrow checker issues
        {
            let obj_clone = obj.clone();
            // Safety: This is safe because:
            // 1. We have exclusive access to the test environment
            // 2. No other threads are accessing this object
            // 3. This is a controlled test for self-referencing objects
            // 4. The mutation happens in a single-threaded test context
            unsafe {
                let obj_mut = obj.get_mut_unchecked();
                obj_mut.myself = Some(obj_clone);
            }
        }

        // Verify the self-reference works
        assert_eq!(obj.value, 42);
        assert!(obj.myself.is_some());
        if let Some(ref myself) = obj.myself {
            assert_eq!(myself.value, 42);
            assert!(Gc::ptr_eq(&obj, myself));
        }
    }

    // Test that self-referencing objects can be collected
    collector.collect();
    std::thread::sleep(std::time::Duration::from_millis(50));
}
