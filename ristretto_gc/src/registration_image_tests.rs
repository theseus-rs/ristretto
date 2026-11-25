//! Real descriptor ownership plus forced schedules for the migration component.
#![expect(
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    reason = "finite ownership fixture assertions"
)]
use super::*;
use crate::{Configuration, GarbageCollector, Gc, GcRootGuard, HeapOwner, Trace};
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;

fn heap() -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .allocation_threshold(usize::MAX)
            .build(),
    )
}

fn descriptor<T: Trace>(gc: &GarbageCollector, root: &GcRootGuard<T>) -> MetadataRef {
    gc.metadata_for_address(root.as_ptr() as usize)
        .expect("ordinary descriptor")
}

#[test]
fn real_descriptors_are_selected_together_and_foreign_records_fail_closed() -> crate::Result<()> {
    let gc = heap();
    let other = heap();
    let registry = Registry::new(Arc::downgrade(&gc));
    let foreign = Registry::new(Arc::downgrade(&other));
    let root = Gc::try_new(&gc, 7_u64)?;
    let metadata = descriptor(&gc, &root);
    let key = Key {
        address: metadata.ptr().0,
        id: metadata.id,
    };
    assert!(matches!(
        foreign.insert(&metadata),
        Err(Error::ForeignCollector)
    ));
    let old = registry.view();
    let prepared = registry.insert(&metadata).expect("vacant image");
    assert_eq!(registry.view().len(), 0, "preparation published nothing");
    assert_eq!(foreign.commit(&prepared), Status::Foreign);
    assert_eq!(registry.commit(&prepared), Status::Installed);
    assert_eq!(
        foreign.commit(&prepared),
        Status::Foreign,
        "completion is registry scoped"
    );
    assert_eq!(old.len(), 0);
    let live = registry.view();
    assert_eq!(live.len(), 1);
    assert_eq!(
        live.by_identity(key.id).map(|(actual, _)| actual),
        Some(key)
    );
    assert_eq!(live.next(None).map(|(actual, _)| actual), Some(key));
    assert!(live.next(Some(key.address)).is_none());
    assert!(live.is_young(key));
    assert!(matches!(
        registry.insert(&metadata),
        Err(Error::Conflict(Conflict::AlreadyPresent))
    ));
    let promoted = registry.promote(key).expect("young identity");
    assert_eq!(registry.commit(&promoted), Status::Installed);
    assert!(!registry.view().is_young(key));
    assert!(live.is_young(key), "old image was changed in place");
    let removed = registry.remove(key).expect("exact identity");
    assert_eq!(registry.commit(&removed), Status::Installed);
    assert_eq!(registry.view().len(), 0);
    assert_eq!(
        registry.commit(&prepared),
        Status::Installed,
        "old outcome is not a replay"
    );
    assert_eq!(registry.view().len(), 0);
    drop(root);
    gc.collect_full()?;
    assert_eq!(
        gc.managed_bytes(),
        0,
        "old views retained allocation credit"
    );
    assert_eq!(live.get(key).map(|value| value.id), Some(key.id));
    Ok(())
}

struct Child(Arc<AtomicUsize>);
// SAFETY: only a host counter, with no managed edges.
unsafe impl Trace for Child {
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Child {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

struct Owner {
    child: Mutex<Option<GcRootGuard<Child>>>,
    dropped: Arc<AtomicUsize>,
}
impl HeapOwner for Owner {
    fn release_heap_roots(&self) {
        let child = self
            .child
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
        drop(child);
    }
}
impl Drop for Owner {
    fn drop(&mut self) {
        self.dropped.fetch_add(1, Ordering::Relaxed);
    }
}

struct Owned {
    owner: Weak<Owner>,
    dropped: Arc<AtomicUsize>,
}
// SAFETY: the owner child has an independent root. The payload holds only host state.
unsafe impl Trace for Owned {
    fn trace(&self, _: &GarbageCollector) {}
    fn heap_owner(&self) -> Option<Arc<dyn HeapOwner>> {
        self.owner
            .upgrade()
            .map(|owner| owner as Arc<dyn HeapOwner>)
    }
}
impl Drop for Owned {
    fn drop(&mut self) {
        self.dropped.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn retained_images_and_discarded_intents_do_not_retain_payload_owner_roots() -> crate::Result<()> {
    for install in [false, true] {
        let gc = heap();
        let registry = Registry::new(Arc::downgrade(&gc));
        let child_drops = Arc::new(AtomicUsize::new(0));
        let owner_drops = Arc::new(AtomicUsize::new(0));
        let payload_drops = Arc::new(AtomicUsize::new(0));
        let owner = Arc::new(Owner {
            child: Mutex::new(Some(Gc::try_new(&gc, Child(child_drops.clone()))?)),
            dropped: owner_drops.clone(),
        });
        let root = Gc::try_new(
            &gc,
            Owned {
                owner: Arc::downgrade(&owner),
                dropped: payload_drops.clone(),
            },
        )?;
        let metadata = descriptor(&gc, &root);
        let key = Key {
            address: metadata.ptr().0,
            id: metadata.id,
        };
        let operation = registry.insert(&metadata).expect("private proposal");
        if install {
            assert_eq!(registry.commit(&operation), Status::Installed);
        }
        let parked = registry.view();
        drop((owner, metadata, root));
        for _ in 0..3 {
            gc.collect_full()?;
        }
        assert_eq!(payload_drops.load(Ordering::Relaxed), 1);
        assert_eq!(owner_drops.load(Ordering::Relaxed), 1);
        assert_eq!(child_drops.load(Ordering::Relaxed), 1);
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(parked.get(key).is_some(), install);
        // These identities remain owned while the transitive managed graph has
        // already disappeared. No descriptor payload access is attempted.
        drop((parked, operation, registry));
        gc.stop()?;
        let gone = Arc::downgrade(&gc);
        drop(gc);
        assert!(gone.upgrade().is_none());
    }
    Ok(())
}

#[test]
fn dropped_and_stale_proposals_do_not_publish_any_subset() {
    let registry = state::Registry::new();
    let first = Key {
        address: 8,
        id: crate::ObjectId::from_i64(1),
    };
    let second = Key {
        address: 16,
        id: crate::ObjectId::from_i64(2),
    };
    let abandoned = registry.insert(first, 11).expect("first proposal");
    drop(abandoned);
    assert_eq!(registry.view().len(), 0);
    let delayed = registry.insert(first, 11).expect("delayed");
    let winner = registry.insert(second, 22).expect("winner");
    assert_eq!(registry.commit(&winner), Status::Installed);
    assert_eq!(registry.commit(&delayed), Status::Stale);
    let view = registry.view();
    assert!(view.get(first).is_none());
    assert!(view.by_identity(first.id).is_none());
    assert!(!view.is_young(first));
    assert_eq!(view.get(second), Some(&22));
    assert!(view.is_young(second));
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn native_owning_link_advances_and_reclaims_with_an_image_writer_parked() -> crate::Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for after_compare in [false, true] {
        let gc = heap();
        let root = Gc::try_new(&gc, 7_u64)?;
        let metadata = descriptor(&gc, &root);
        let key = Key {
            address: metadata.ptr().0,
            id: metadata.id,
        };
        let registry = Arc::new(Registry::new(Arc::downgrade(&gc)));
        let operation = Arc::new(registry.insert(&metadata).expect("image proposal"));
        let (entered, observed) = mpsc::channel();
        let (resume, resumed) = mpsc::channel();
        let worker = {
            let registry = registry.clone();
            let operation = operation.clone();
            std::thread::spawn(move || {
                let pause = || {
                    entered.send(()).expect("parked image writer");
                    resumed
                        .recv_timeout(Duration::from_secs(20))
                        .expect("resume image writer");
                };
                if after_compare {
                    registry.state.commit_observed(&operation, || {}, pause)
                } else {
                    registry.state.commit_observed(&operation, pause, || {})
                }
            })
        };
        let parked = observed.recv_timeout(Duration::from_secs(10)).is_ok();
        let exercise = || -> crate::Result<_> {
            if after_compare {
                let remove = registry.remove(key).expect("selected original");
                assert_eq!(registry.commit(&remove), Status::Installed);
            }
            drop(root);
            gc.collect_full()?;
            assert_eq!(
                gc.managed_bytes(),
                0,
                "parked descriptor retained payload credit"
            );
            let replacement = Gc::try_new(&gc, 19_u64)?;
            let replacement_metadata = descriptor(&gc, &replacement);
            assert_eq!(
                registry.commit(&registry.insert(&replacement_metadata).expect("replacement")),
                Status::Installed
            );
            let recovered = registry.commit(&operation);
            assert_eq!(registry.view().len(), 1);
            assert_eq!(
                registry.view().by_identity(key.id).map(|(key, _)| key),
                None
            );
            assert_eq!(**replacement, 19);
            drop(replacement);
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), 0);
            Ok(recovered)
        };
        let outcome = if parked { Some(exercise()) } else { None };
        let _ = resume.send(());
        let writer = worker.join().expect("image writer");
        assert!(parked, "image writer missed its bounded hook");
        let recovered = outcome.expect("exercise ran")?;
        assert_eq!(
            recovered,
            if after_compare {
                Status::Installed
            } else {
                Status::Stale
            }
        );
        assert_eq!(
            writer,
            if after_compare {
                Status::Installed
            } else {
                Status::Retry
            }
        );
        gc.stop()?;
    }
    Ok(())
}
