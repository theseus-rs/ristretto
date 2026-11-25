//! Full and nursery reference cursors share a single immutable snapshot. An
//! interrupted insertion has no lock that a collection needs to acquire.
use crate::publication_directory::Directory;
use crate::weak::WeakSlot;
use arc_swap::ArcSwap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};

#[derive(Debug, Default)]
struct Indexes {
    full: Directory<Weak<WeakSlot>>,
    young: Directory<Weak<WeakSlot>>,
}

#[derive(Debug, Default)]
pub(crate) struct WeakRegistry(ArcSwap<Indexes>);

impl WeakRegistry {
    #[expect(
        clippy::expect_used,
        reason = "an unused nonzero usize ID proves there is space in both directories"
    )]
    pub(crate) fn insert(&self, next_id: &AtomicUsize, slot: &Arc<WeakSlot>, young: bool) {
        let weak = Arc::downgrade(slot);
        let mut id = next_id.fetch_add(1, Ordering::Relaxed);
        loop {
            let current = self.0.load_full();
            while id == 0 || current.full.get(id as u64).is_some() {
                id = next_id.fetch_add(1, Ordering::Relaxed);
            }
            let key = id as u64;
            let proposed = Arc::new(Indexes {
                // A nonzero unused usize key leaves room in both indexes.
                full: current
                    .full
                    .insert(key, weak.clone())
                    .expect("unused weak id")
                    .0,
                young: if young {
                    current
                        .young
                        .insert(key, weak.clone())
                        .expect("unused weak id")
                        .0
                } else {
                    current.young.clone()
                },
            });
            if Arc::ptr_eq(&self.0.compare_and_swap(&current, proposed), &current) {
                return;
            }
        }
    }

    pub(crate) fn next(&self, young: bool, after: usize) -> Option<(usize, Weak<WeakSlot>)> {
        let current = self.0.load();
        let index = if young { &current.young } else { &current.full };
        let (key, slot) = index.next_after(Some(after as u64))?;
        Some((usize::try_from(key).ok()?, slot.clone()))
    }

    /// Pruning is optional. A conflicting publication can leave an expired or
    /// mature entry for the next pass. A stale cursor must never prune a new
    /// generation that reused the same numeric ID.
    pub(crate) fn prune(&self, id: usize, observed: &Weak<WeakSlot>, young_only: bool) {
        let current = self.0.load_full();
        let key = id as u64;
        let index = if young_only {
            &current.young
        } else {
            &current.full
        };
        if !index
            .get(key)
            .is_some_and(|entry| Weak::ptr_eq(entry, observed))
        {
            return;
        }
        let proposed = Arc::new(Indexes {
            full: if young_only {
                current.full.clone()
            } else {
                current.full.remove(key).0
            },
            young: current.young.remove(key).0,
        });
        drop(self.0.compare_and_swap(&current, proposed));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GarbageCollector, Gc, GcWeak, ReferenceKind};

    #[test]
    fn stale_snapshot_and_expired_cursor_cannot_remove_reused_id() {
        let gc = GarbageCollector::new();
        let target = Gc::new(&gc, 7_u64);
        let first = GcWeak::new(&target, ReferenceKind::Weak, 0);
        let second = GcWeak::new(&target, ReferenceKind::Soft, 0);
        let registry = WeakRegistry::default();
        let ids = AtomicUsize::new(usize::MAX);
        registry.insert(&ids, &first.slot, true);
        let stale = registry.0.load_full();
        let (id, expired) = registry.next(true, 0).expect("first slot");
        assert_eq!(usize::MAX, id);
        drop(first);
        assert!(expired.upgrade().is_none());
        registry.prune(id, &expired, false);
        ids.store(id, Ordering::Relaxed);
        registry.insert(&ids, &second.slot, true);
        let replacement = Arc::downgrade(&second.slot);
        registry.prune(id, &expired, true);
        registry.prune(id, &expired, false);
        for young in [true, false] {
            assert!(
                registry
                    .next(young, 0)
                    .is_some_and(|(key, slot)| key == id && slot.ptr_eq(&replacement))
            );
        }
        assert!(
            stale
                .full
                .get(id as u64)
                .is_some_and(|slot| slot.ptr_eq(&expired))
        );
        registry.prune(id, &replacement, true);
        assert!(registry.next(true, 0).is_none());
        assert!(registry.next(false, 0).is_some());
    }
}
