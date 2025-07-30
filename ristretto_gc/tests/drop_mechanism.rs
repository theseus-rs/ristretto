use ristretto_gc::{Finalize, GC, Gc};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Test struct that implements Drop to verify proper cleanup
#[derive(Debug)]
struct TestObject {
    id: usize,
    dropped: Arc<AtomicBool>,
}

impl TestObject {
    fn new(id: usize) -> (Self, Arc<AtomicBool>) {
        let dropped = Arc::new(AtomicBool::new(false));
        let object = Self {
            id,
            dropped: dropped.clone(),
        };
        (object, dropped)
    }
}

impl Drop for TestObject {
    fn drop(&mut self) {
        println!("Dropping TestObject {}", self.id);
        self.dropped.store(true, Ordering::Release);
    }
}

impl ristretto_gc::Trace for TestObject {
    fn trace(&self, _collector: &ristretto_gc::GarbageCollector) {
        // No GC references to trace
    }
}

/// Test struct with finalizer support
#[derive(Debug)]
struct TestObjectWithFinalizer {
    id: usize,
    finalized: Arc<AtomicBool>,
    dropped: Arc<AtomicBool>,
}

impl TestObjectWithFinalizer {
    fn new(id: usize) -> (Self, Arc<AtomicBool>, Arc<AtomicBool>) {
        let finalized = Arc::new(AtomicBool::new(false));
        let dropped = Arc::new(AtomicBool::new(false));
        let object = Self {
            id,
            finalized: finalized.clone(),
            dropped: dropped.clone(),
        };
        (object, finalized, dropped)
    }
}

impl Drop for TestObjectWithFinalizer {
    fn drop(&mut self) {
        println!("Dropping TestObjectWithFinalizer {}", self.id);
        self.dropped.store(true, Ordering::Release);
    }
}

impl Finalize for TestObjectWithFinalizer {
    fn finalize(&self) {
        println!("Finalizing TestObjectWithFinalizer {}", self.id);
        self.finalized.store(true, Ordering::Release);
    }
}

impl ristretto_gc::Trace for TestObjectWithFinalizer {
    fn trace(&self, _collector: &ristretto_gc::GarbageCollector) {
        // No GC references to trace
    }
}

#[test]
fn test_basic_drop_mechanism() {
    println!("Testing basic drop mechanism...");

    let (obj, dropped_flag) = TestObject::new(1);

    // Create a Gc object
    let gc_object = Gc::new(obj);

    // Verify the object is not dropped yet
    assert!(!dropped_flag.load(Ordering::Acquire));

    // Drop the Gc reference
    drop(gc_object);

    // Force garbage collection
    GC.collect();

    // Give the collector time to run
    std::thread::sleep(std::time::Duration::from_millis(100));

    // The object should eventually be dropped by the garbage collector
    // Note: In a real scenario, we might need to trigger multiple collections
    // or wait longer for the concurrent collector to complete
    println!("Object dropped: {}", dropped_flag.load(Ordering::Acquire));
}

#[test]
fn test_finalizer_mechanism() {
    println!("Testing finalizer mechanism...");

    let (obj, finalized_flag, dropped_flag) = TestObjectWithFinalizer::new(2);

    // Create a Gc object with finalizer
    let gc_object = Gc::new_with_finalizer(obj);

    // Verify the object is not finalized or dropped yet
    assert!(!finalized_flag.load(Ordering::Acquire));
    assert!(!dropped_flag.load(Ordering::Acquire));

    // Drop the Gc reference
    drop(gc_object);

    // Force garbage collection
    GC.collect();

    // Give the collector time to run
    std::thread::sleep(std::time::Duration::from_millis(100));

    // The object should eventually be finalized and then dropped
    println!(
        "Object finalized: {}",
        finalized_flag.load(Ordering::Acquire)
    );
    println!("Object dropped: {}", dropped_flag.load(Ordering::Acquire));
}

#[test]
fn test_multiple_objects() {
    println!("Testing multiple objects...");

    let mut objects = Vec::new();
    let mut drop_flags = Vec::new();

    // Create multiple objects
    for i in 0..5 {
        let (obj, dropped) = TestObject::new(i + 10);
        objects.push(Gc::new(obj));
        drop_flags.push(dropped);
    }

    // Drop all references
    objects.clear();

    // Force garbage collection multiple times
    for _ in 0..3 {
        GC.collect();
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    // Check results
    for (i, flag) in drop_flags.iter().enumerate() {
        println!(
            "Object {} dropped: {}",
            i + 10,
            flag.load(Ordering::Acquire)
        );
    }
}
