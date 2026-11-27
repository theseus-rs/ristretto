//! Managed mutation demotes owning clones into traced cycle edges.
#![cfg(not(target_family = "wasm"))]
#![expect(clippy::panic_in_result_fn)]

use ristretto_gc::cell::RwLock;
use ristretto_gc::{GarbageCollector, Gc, Result, Trace};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Node {
    id: usize,
    children: Vec<Gc<RwLock<Node>>>,
    parent: Option<Gc<RwLock<Node>>>,
    drops: Arc<AtomicUsize>,
}

// SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Node {
    fn trace(&self, collector: &GarbageCollector) {
        self.children.trace(collector);
        self.parent.trace(collector);
    }
    fn root(&mut self) {
        self.children.root();
        self.parent.root();
    }
    unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
        unsafe { Trace::unroot(&mut self.children, owner) };
        unsafe { Trace::unroot(&mut self.parent, owner) };
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}

fn node(id: usize, drops: &Arc<AtomicUsize>) -> RwLock<Node> {
    RwLock::new(Node {
        id,
        children: Vec::new(),
        parent: None,
        drops: drops.clone(),
    })
}

#[test_log::test]
fn test_cyclic_gc_collection() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();
    let drops = Arc::new(AtomicUsize::new(0));
    let a = Gc::try_new(&collector, node(1, &drops))?;
    let b = Gc::try_new(&collector, node(2, &drops))?;
    a.write().parent = Some(b.clone_gc());
    b.write().parent = Some(a.clone_gc());
    drop(b);
    collector.collect_full()?;
    assert_eq!(a.read().parent.as_ref().expect("b").read().id, 2);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(a);
    collector.stop()?;
    assert_eq!(drops.load(Ordering::Relaxed), 2);
    assert_eq!(collector.managed_bytes(), 0);
    Ok(())
}

#[test_log::test]
fn test_complex_cyclic_structure() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();
    let drops = Arc::new(AtomicUsize::new(0));
    let root = Gc::try_new(&collector, node(0, &drops))?;
    for id in 1..=2 {
        let child = Gc::try_new(&collector, node(id, &drops))?;
        child.write().parent = Some(root.clone_gc());
        root.write().children.push(child.clone_gc());
    }
    collector.collect_full()?;
    assert_eq!(root.read().children.len(), 2);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(root);
    collector.stop()?;
    assert_eq!(drops.load(Ordering::Relaxed), 3);
    assert_eq!(collector.managed_bytes(), 0);
    Ok(())
}

#[test_log::test]
fn test_self_referencing_object() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.start();
    let drops = Arc::new(AtomicUsize::new(0));
    let object = Gc::try_new(&collector, node(42, &drops))?;
    object.write().parent = Some(object.clone_gc());
    collector.collect_full()?;
    {
        let read = object.read();
        assert_eq!(read.id, 42);
        assert!(Gc::ptr_eq(
            &object,
            read.parent.as_ref().expect("self edge")
        ));
    }
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(object);
    collector.stop()?;
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(collector.managed_bytes(), 0);
    Ok(())
}
