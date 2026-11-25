//! Payload lifetime owns VM metadata; stale descriptors own only its discovery link.
#![expect(
    clippy::panic_in_result_fn,
    reason = "assert finite GC lifetime fixtures"
)]

use crate::metadata::{MetadataRef, ObjectMetadata};
use crate::{Configuration, Finalize, GarbageCollector, Gc, GcRootGuard, HeapOwner, Result, Trace};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Weak};

#[derive(Default)]
struct Counts {
    children: AtomicUsize,
    owners: AtomicUsize,
    unlocked_owner_drops: AtomicUsize,
    releases: AtomicUsize,
    unlocked_releases: AtomicUsize,
    payloads: AtomicUsize,
    payload_owner_live: AtomicUsize,
    finalizers: AtomicUsize,
    finalizer_owner_live: AtomicUsize,
}

struct Child(Arc<Counts>);
// SAFETY: only host counters are present, with no managed graph edges.
unsafe impl Trace for Child {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Child {
    fn drop(&mut self) {
        self.0.children.fetch_add(1, Ordering::Relaxed);
    }
}

struct Owner {
    root: Mutex<Option<GcRootGuard<Child>>>,
    counts: Arc<Counts>,
    collector: Weak<GarbageCollector>,
}

fn callback_locks_available(collector: &Weak<GarbageCollector>) -> bool {
    let Some(gc) = collector.upgrade() else {
        return false;
    };
    // The fixture fixes metadata_shards=1; this probes every configured bucket.
    gc.objects.try_lock_bucket(0).is_some()
        && gc.roots.try_lock().is_ok()
        && gc
            .domains
            .iter()
            .all(|domain| domain.cycle.try_lock().is_ok())
}

impl HeapOwner for Owner {
    fn release_heap_roots(&self) {
        self.counts.releases.fetch_add(1, Ordering::Relaxed);
        if callback_locks_available(&self.collector) {
            self.counts
                .unlocked_releases
                .fetch_add(1, Ordering::Relaxed);
        }
        let root = self
            .root
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
        drop(root);
    }
}
impl Drop for Owner {
    fn drop(&mut self) {
        if callback_locks_available(&self.collector) {
            self.counts
                .unlocked_owner_drops
                .fetch_add(1, Ordering::Relaxed);
        }
        self.counts.owners.fetch_add(1, Ordering::Relaxed);
    }
}

struct Owned {
    // Deliberately weak: the metadata must retain the returned HeapOwner for
    // the full payload lifetime even when T itself does not retain that Arc.
    owner: Weak<Owner>,
    counts: Arc<Counts>,
}
// SAFETY: Owner's child has an independent owning root; no unrooted managed
// references are omitted. This payload itself contains only host state.
unsafe impl Trace for Owned {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
    fn heap_owner(&self) -> Option<Arc<dyn HeapOwner>> {
        self.owner
            .upgrade()
            .map(|owner| owner as Arc<dyn HeapOwner>)
    }
}
impl Finalize for Owned {
    fn finalize(&self) {
        self.counts.finalizers.fetch_add(1, Ordering::Relaxed);
        if self.owner.upgrade().is_some() {
            self.counts
                .finalizer_owner_live
                .fetch_add(1, Ordering::Relaxed);
        }
    }
}
impl Drop for Owned {
    fn drop(&mut self) {
        if self.owner.upgrade().is_some() {
            self.counts
                .payload_owner_live
                .fetch_add(1, Ordering::Relaxed);
        }
        self.counts.payloads.fetch_add(1, Ordering::Relaxed);
    }
}

fn heap() -> Arc<GarbageCollector> {
    GarbageCollector::with_config(Configuration {
        allocation_threshold: usize::MAX,
        metadata_shards: 1,
        ..Configuration::default()
    })
}

fn owner(gc: &Arc<GarbageCollector>, counts: &Arc<Counts>) -> Result<Arc<Owner>> {
    Ok(Arc::new(Owner {
        root: Mutex::new(Some(Gc::try_new(gc, Child(counts.clone()))?)),
        counts: counts.clone(),
        collector: Arc::downgrade(gc),
    }))
}

fn value(owner: &Arc<Owner>) -> Owned {
    Owned {
        owner: Arc::downgrade(owner),
        counts: owner.counts.clone(),
    }
}

fn descriptor(gc: &GarbageCollector, root: &GcRootGuard<Owned>) -> MetadataRef {
    gc.metadata_for_address(root.as_ptr() as usize)
        .expect("live fixture descriptor")
}

fn collect(gc: &GarbageCollector) -> Result<()> {
    for _ in 0..3 {
        gc.collect_full()?;
    }
    Ok(())
}

fn assert_reclaimed(gc: &GarbageCollector, counts: &Counts, payloads: usize) {
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(counts.payloads.load(Ordering::Relaxed), payloads);
    assert_eq!(counts.payload_owner_live.load(Ordering::Relaxed), payloads);
    assert_eq!(counts.owners.load(Ordering::Relaxed), 1);
    assert_eq!(counts.unlocked_owner_drops.load(Ordering::Relaxed), 1);
    assert_eq!(counts.children.load(Ordering::Relaxed), 1);
}

#[test]
fn retired_descriptor_does_not_retain_owner_managed_roots() -> Result<()> {
    let gc = heap();
    let counts = Arc::new(Counts::default());
    let owner = owner(&gc, &counts)?;
    let root = Gc::try_new(&gc, value(&owner))?;
    let parked = descriptor(&gc, &root);
    drop((root, owner));
    collect(&gc)?;
    let observed = (
        gc.managed_bytes(),
        counts.owners.load(Ordering::Relaxed),
        counts.children.load(Ordering::Relaxed),
    );
    // Retain the descriptor throughout the progress assertion's observation,
    // then clean even a failing pre-fix implementation before asserting it.
    drop(parked);
    collect(&gc)?;
    gc.stop()?;
    assert_reclaimed(&gc, &counts, 1);
    assert_eq!(observed, (0, 1, 1));
    Ok(())
}

#[test]
fn live_payload_owner_remains_discoverable_and_shutdown_releases_roots() -> Result<()> {
    let gc = heap();
    let counts = Arc::new(Counts::default());
    let owner = owner(&gc, &counts)?;
    let root = Gc::try_new(&gc, value(&owner))?;
    let parked = descriptor(&gc, &root);
    let payload_bytes = parked.size();
    let weak = Arc::downgrade(&owner);
    drop(owner);
    collect(&gc)?;
    assert!(weak.upgrade().is_some());
    assert_eq!(counts.children.load(Ordering::Relaxed), 0);
    gc.release_vm_roots();
    collect(&gc)?;
    assert_eq!(counts.releases.load(Ordering::Relaxed), 1);
    assert_eq!(counts.unlocked_releases.load(Ordering::Relaxed), 1);
    assert_eq!(counts.children.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), payload_bytes);
    assert_eq!(counts.owners.load(Ordering::Relaxed), 0);
    drop(root);
    collect(&gc)?;
    assert_reclaimed(&gc, &counts, 1);
    assert!(weak.upgrade().is_none());
    drop(parked);
    gc.stop()?;
    assert_reclaimed(&gc, &counts, 1);
    Ok(())
}

#[test]
fn shared_owner_survives_one_payload_and_ends_with_the_last_payload() -> Result<()> {
    let gc = heap();
    let counts = Arc::new(Counts::default());
    let owner = owner(&gc, &counts)?;
    let first = Gc::try_new(&gc, value(&owner))?;
    let second = Gc::try_new(&gc, value(&owner))?;
    let parked = descriptor(&gc, &first);
    drop((first, owner));
    collect(&gc)?;
    assert_eq!(counts.payloads.load(Ordering::Relaxed), 1);
    assert_eq!(counts.owners.load(Ordering::Relaxed), 0);
    assert_eq!(counts.children.load(Ordering::Relaxed), 0);
    drop(second);
    collect(&gc)?;
    assert_reclaimed(&gc, &counts, 2);
    drop(parked);
    gc.stop()?;
    assert_reclaimed(&gc, &counts, 2);
    Ok(())
}

#[test]
fn storage_lease_preserves_payload_owner_until_destruction() -> Result<()> {
    let gc = heap();
    let counts = Arc::new(Counts::default());
    let owner = owner(&gc, &counts)?;
    let root = Gc::try_new(&gc, value(&owner))?;
    let parked = descriptor(&gc, &root);
    let lease = ObjectMetadata::lease(&parked).expect("live storage");
    let all_bytes = gc.managed_bytes();
    drop((root, owner));
    collect(&gc)?;
    assert_eq!(gc.managed_bytes(), all_bytes);
    assert_eq!(counts.payloads.load(Ordering::Relaxed), 0);
    assert_eq!(counts.owners.load(Ordering::Relaxed), 0);
    assert_eq!(counts.children.load(Ordering::Relaxed), 0);
    assert!(ObjectMetadata::lease(&parked).is_none());
    drop(lease);
    collect(&gc)?;
    assert_reclaimed(&gc, &counts, 1);
    drop(parked);
    gc.stop()?;
    assert_reclaimed(&gc, &counts, 1);
    Ok(())
}

#[test]
fn finalizer_and_destructor_run_before_the_owner_is_released() -> Result<()> {
    let gc = heap();
    let counts = Arc::new(Counts::default());
    let owner = owner(&gc, &counts)?;
    let root = Gc::new_with_finalizer(&gc, value(&owner));
    let parked = descriptor(&gc, &root);
    drop((root, owner));
    collect(&gc)?;
    assert_eq!(counts.finalizers.load(Ordering::Relaxed), 1);
    assert_eq!(counts.finalizer_owner_live.load(Ordering::Relaxed), 1);
    assert_reclaimed(&gc, &counts, 1);
    drop(parked);
    gc.stop()?;
    assert_reclaimed(&gc, &counts, 1);
    assert_eq!(counts.finalizers.load(Ordering::Relaxed), 1);
    Ok(())
}

#[test]
fn shutdown_pending_reclaim_releases_cycle_guard_before_owner_drop() -> Result<()> {
    let gc = GarbageCollector::with_config(Configuration {
        allocation_threshold: usize::MAX,
        metadata_shards: 1,
        incremental_step_size: 1,
        max_pause_time_us: 0,
        ..Configuration::default()
    });
    let counts = Arc::new(Counts::default());
    let owner = owner(&gc, &counts)?;
    let root = Gc::new_with_finalizer(&gc, value(&owner));
    let parked = descriptor(&gc, &root);
    // Match shutdown's ownership precondition: no Java task, managed handle,
    // storage borrower, or independent VM root can access a remaining payload.
    gc.release_vm_roots();
    drop((root, owner));
    let roots = gc.root_statistics();
    assert_eq!(roots.compatibility_roots + roots.slot_roots, 0);
    assert_eq!(gc.registered_workers(), 0);
    gc.collect();
    let pending = || {
        gc.domains.iter().any(|domain| {
            domain
                .cycle
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .reclaim
                .as_ref()
                .is_some_and(|task| task.metadata.id == parked.id)
        })
    };
    for _ in 0..4096 {
        if pending() {
            break;
        }
        gc.step();
    }
    assert!(pending(), "real sweep did not park the target reclaim task");
    assert_eq!(counts.payloads.load(Ordering::Relaxed), 0);
    // Invoke the exact teardown routine while one host Arc remains solely to
    // let callbacks probe locks through Weak::upgrade. No live GC roots remain.
    gc.cleanup_remaining_objects();
    drop(parked);
    assert_reclaimed(&gc, &counts, 1);
    assert_eq!(counts.finalizers.load(Ordering::Relaxed), 1);
    assert_eq!(counts.finalizer_owner_live.load(Ordering::Relaxed), 1);
    Ok(())
}

#[cfg(all(panic = "unwind", not(target_family = "wasm")))]
#[test]
fn unpublished_owner_is_released_after_constructor_cancellation() -> Result<()> {
    use super::ClaimPause;
    use std::sync::mpsc;

    for finalizable in [false, true] {
        let gc = heap();
        let counts = Arc::new(Counts::default());
        let owner = owner(&gc, &counts)?;
        let (entered, observed) = mpsc::channel();
        let (resume, receiver) = mpsc::channel();
        drop(resume);
        *gc.metadata_prepare_pause
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ClaimPause {
            entered,
            resume: receiver,
        });
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if finalizable {
                drop(Gc::new_with_finalizer(&gc, value(&owner)));
            } else {
                drop(Gc::try_new(&gc, value(&owner)));
            }
        }));
        assert!(result.is_err());
        assert!(observed.try_recv().is_ok());
        assert_eq!(counts.payloads.load(Ordering::Relaxed), 1);
        assert_eq!(counts.payload_owner_live.load(Ordering::Relaxed), 1);
        assert_eq!(counts.owners.load(Ordering::Relaxed), 0);
        // Only the independently held child root remains after cancellation.
        let roots = gc.root_statistics();
        assert_eq!(roots.compatibility_roots + roots.slot_roots, 1);
        drop(owner);
        collect(&gc)?;
        gc.stop()?;
        assert_reclaimed(&gc, &counts, 1);
    }
    Ok(())
}
