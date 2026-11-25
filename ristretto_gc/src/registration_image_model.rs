//! Source-shared owning-image transitions. Loom replaces only the linearizable
//! owning head and completion atomics. Directory paths are the actual immutable
//! production implementation; `ArcSwap` internals/allocator latency are excluded.
#![cfg(not(target_family = "wasm"))]
#![expect(
    dead_code,
    reason = "selected production image APIs under the model adapter"
)]
use loom::sync::atomic::{AtomicBool, Ordering};
use loom::sync::{Arc, Mutex, mpsc};
use loom::thread;

use crate::publication_directory::Directory;
use crate::registration_key::{Conflict, Key, Observed};
struct Link<T>(Mutex<Arc<T>>);
#[expect(
    clippy::expect_used,
    reason = "a poisoned model link fails the schedule"
)]
impl<T> Link<T> {
    fn new(value: Arc<T>) -> Self {
        Self(Mutex::new(value))
    }
    fn load(&self) -> Arc<T> {
        self.0.lock().expect("unpoisoned owning-link model").clone()
    }
    fn compare(&self, expected: &Arc<T>, proposed: Arc<T>) -> bool {
        let old = {
            let mut head = self.0.lock().expect("unpoisoned owning-link model");
            if !Arc::ptr_eq(&head, expected) {
                return false;
            }
            std::mem::replace(&mut *head, proposed)
        };
        drop(old);
        true
    }
}
mod state {
    include!("registration_image_state.rs");
}
use state::{Registry, Status};

fn model(action: impl Fn() + Send + Sync + 'static) {
    let mut builder = loom::model::Builder::new();
    builder.preemption_bound = Some(2);
    builder.max_permutations = Some(10_000);
    builder.max_branches = 10_000;
    builder.check(action);
}
fn key(address: usize, id: u64) -> Key {
    Key {
        address,
        id: crate::ObjectId::from_i64(id.cast_signed()),
    }
}

#[test]
fn competing_images_never_publish_a_mixed_write_set() {
    model(|| {
        let registry = Arc::new(Registry::new());
        let a = key(8, 1);
        let b = key(16, 2);
        let first = registry.insert(a, 11).unwrap();
        let second = registry.insert(b, 22).unwrap();
        let worker = {
            let registry = registry.clone();
            thread::spawn(move || (registry.commit(&first), first))
        };
        let second_status = registry.commit(&second);
        let (first_status, first) = worker.join().unwrap();
        let view = registry.view();
        assert_eq!(view.len(), 1);
        for (candidate, value, status) in [(a, 11, first_status), (b, 22, second_status)] {
            let present = view.get(candidate).is_some();
            assert_eq!(present, status == Status::Installed);
            assert_eq!(
                view.by_identity(candidate.id).map(|(_, v)| *v),
                present.then_some(value)
            );
            assert_eq!(view.is_young(candidate), present);
        }
        let (late, candidate, value) = if first_status == Status::Installed {
            (&second, b, 22)
        } else {
            (&first, a, 11)
        };
        assert_eq!(registry.commit(late), Status::Stale);
        assert_eq!(
            registry.commit(&registry.insert(candidate, value).unwrap()),
            Status::Installed
        );
        assert_eq!(registry.view().len(), 2);
    });
}

#[test]
fn parked_success_is_recovered_after_removal_and_address_reuse() {
    model(|| {
        let registry = Arc::new(Registry::new());
        let old = key(8, 1);
        let fresh = key(8, 2);
        let operation = Arc::new(registry.insert(old, 11).unwrap());
        let (entered, observed) = mpsc::channel();
        let (resume, resumed) = mpsc::channel();
        let worker = {
            let registry = registry.clone();
            let operation = operation.clone();
            thread::spawn(move || {
                registry.commit_observed(
                    &operation,
                    || {},
                    || {
                        entered.send(()).unwrap();
                        resumed.recv().unwrap();
                    },
                )
            })
        };
        observed.recv().unwrap();
        let parked_view = registry.view();
        let remove = registry.remove(old).unwrap();
        assert_eq!(registry.commit(&remove), Status::Installed);
        assert_eq!(
            registry.commit(&registry.insert(fresh, 22).unwrap()),
            Status::Installed
        );
        // Do not resume or explicitly help the original CAS winner first: its
        // successor must have left a durable installation certificate.
        let recovered = registry.commit(&operation);
        let view = registry.view();
        let current = view.get(fresh).copied();
        let stale = view.get(old).copied();
        let old_identity = view.by_identity(old.id).is_some();
        resume.send(()).unwrap();
        assert_eq!(worker.join().unwrap(), Status::Installed);
        assert_eq!(
            recovered,
            Status::Installed,
            "success was lost after head replacement"
        );
        assert_eq!(current, Some(22));
        assert_eq!(stale, None, "old exact key followed the reused address");
        assert!(!old_identity);
        assert!(view.is_young(fresh));
        assert_eq!(parked_view.get(old), Some(&11));
    });
}

#[test]
fn parked_precompare_cannot_overwrite_a_reused_address() {
    model(|| {
        let registry = Arc::new(Registry::new());
        let old = key(8, 1);
        let fresh = key(8, 2);
        let operation = Arc::new(registry.insert(old, 11).unwrap());
        let (entered, observed) = mpsc::channel();
        let (resume, resumed) = mpsc::channel();
        let worker = {
            let registry = registry.clone();
            let operation = operation.clone();
            thread::spawn(move || {
                registry.commit_observed(
                    &operation,
                    || {
                        entered.send(()).unwrap();
                        resumed.recv().unwrap();
                    },
                    || {},
                )
            })
        };
        observed.recv().unwrap();
        assert_eq!(
            registry.commit(&registry.insert(fresh, 22).unwrap()),
            Status::Installed
        );
        resume.send(()).unwrap();
        assert_eq!(worker.join().unwrap(), Status::Retry);
        assert_eq!(registry.commit(&operation), Status::Stale);
        let view = registry.view();
        assert_eq!(view.get(old), None);
        assert_eq!(view.get(fresh), Some(&22));
        assert_eq!(view.by_identity(old.id), None);
    });
}

#[test]
fn parked_removal_cannot_erase_a_later_registration() {
    model(|| {
        let registry = Arc::new(Registry::new());
        let old = key(8, 1);
        let fresh = key(8, 2);
        assert_eq!(
            registry.commit(&registry.insert(old, 11).unwrap()),
            Status::Installed
        );
        let operation = Arc::new(registry.remove(old).unwrap());
        let (entered, observed) = mpsc::channel();
        let (resume, resumed) = mpsc::channel();
        let worker = {
            let registry = registry.clone();
            let operation = operation.clone();
            thread::spawn(move || {
                registry.commit_observed(
                    &operation,
                    || {},
                    || {
                        entered.send(()).unwrap();
                        resumed.recv().unwrap();
                    },
                )
            })
        };
        observed.recv().unwrap();
        assert_eq!(
            registry.commit(&registry.insert(fresh, 22).unwrap()),
            Status::Installed
        );
        let recovered = registry.commit(&operation);
        resume.send(()).unwrap();
        assert_eq!(worker.join().unwrap(), Status::Installed);
        assert_eq!(recovered, Status::Installed);
        assert_eq!(registry.view().get(fresh), Some(&22));
    });
}
