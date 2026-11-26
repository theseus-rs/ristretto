//! Models the exact transition source with a linearizable owning pointer CAS.
//! The mutex is a model adapter; ArcSwap internals and object storage are excluded.
#![allow(dead_code)]
#![cfg(not(target_family = "wasm"))]
use loom::sync::atomic::{AtomicU8, AtomicU64, Ordering};
use loom::sync::{Arc, Mutex};
use loom::thread;
#[path = "../src/state.rs"]
mod state;
use state::*;

struct PooledLink<T>(Mutex<Arc<T>>);
impl<T> PooledLink<T> {
    fn new(value: Arc<T>) -> Self {
        Self(Mutex::new(value))
    }
    fn load(&self) -> Arc<T> {
        self.0.lock().unwrap().clone()
    }
    fn compare(&self, expected: &Arc<T>, proposed: Arc<T>) -> Arc<T> {
        let mut current = self.0.lock().unwrap();
        let previous = current.clone();
        if Arc::ptr_eq(&previous, expected) {
            *current = proposed;
        }
        previous
    }
}
fn snapshot(catalog: &Catalog) -> Snapshot {
    for _ in 0..5 {
        if let Some(snapshot) = catalog.snapshot() {
            return snapshot;
        }
    }
    panic!("bounded operations failed to quiesce");
}

#[test]
fn helpers_complete_a_stopped_winner_once() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = fixture_cell(&catalog, 8);
        let attempt = Arc::new(catalog.insert(snapshot(&catalog), cell.clone()).unwrap());
        assert!(catalog.install_and_stop(&attempt));
        let helper = {
            let catalog = catalog.clone();
            let attempt = attempt.clone();
            thread::spawn(move || catalog.commit(&attempt))
        };
        assert_eq!(catalog.commit(&attempt), Outcome::Committed);
        assert_eq!(helper.join().unwrap(), Outcome::Committed);
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [1, 1]
        );
        assert!(matches!(cell.membership(), Membership::Live(_)));
    });
}

#[test]
fn cancellation_and_commit_have_one_decision() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = fixture_cell(&catalog, 8);
        let attempt = Arc::new(catalog.insert(snapshot(&catalog), cell.clone()).unwrap());
        assert!(catalog.install_and_stop(&attempt));
        let canceller = {
            let attempt = attempt.clone();
            thread::spawn(move || attempt.cancel())
        };
        let outcome = catalog.commit(&attempt);
        let cancelled = canceller.join().unwrap();
        assert_eq!(outcome == Outcome::Cancelled, cancelled);
        assert_eq!(cell.membership() == Membership::Prepared, cancelled);
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [usize::from(!cancelled); 2]
        );
    });
}

#[test]
fn an_installed_effect_is_certified_before_another_phase_is_observed() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = fixture_cell(&catalog, 8);
        let attempt = Arc::new(catalog.insert(snapshot(&catalog), cell.clone()).unwrap());
        catalog.effect_and_stop(&attempt);
        let helper = {
            let catalog = catalog.clone();
            let attempt = attempt.clone();
            thread::spawn(move || catalog.commit(&attempt))
        };
        let phase = snapshot(&catalog).phase();
        assert_eq!(phase.domains.map(|d| d.pending), [1, 1]);
        assert_eq!(helper.join().unwrap(), Outcome::Committed);
    });
}

#[test]
fn a_phase_transition_excludes_an_old_announcement() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = fixture_cell(&catalog, 8);
        let insert = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
        let advance = catalog.advance(snapshot(&catalog), 0).unwrap();
        let publisher = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&insert))
        };
        let advanced = catalog.commit(&advance) == Outcome::Committed;
        let inserted = publisher.join().unwrap() == Outcome::Committed;
        assert_ne!(advanced, inserted);
        let phase = snapshot(&catalog).phase();
        assert_eq!(phase.domains[0].floor, u8::from(advanced));
        assert_eq!(phase.domains.map(|d| d.pending), [usize::from(inserted); 2]);
    });
}

#[test]
fn acknowledging_old_work_cannot_erase_new_work() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = fixture_cell(&catalog, 8);
        assert_eq!(
            catalog.commit(&catalog.insert(snapshot(&catalog), cell.clone()).unwrap()),
            Outcome::Committed
        );
        let Membership::Live(work) = cell.membership() else {
            unreachable!()
        };
        let ack = catalog
            .acknowledge(snapshot(&catalog), cell.clone(), 0, work[0])
            .unwrap();
        let require = catalog
            .require(snapshot(&catalog), cell.clone(), 0)
            .unwrap();
        let helper = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&ack))
        };
        let replaced = catalog.commit(&require) == Outcome::Committed;
        let acknowledged = helper.join().unwrap() == Outcome::Committed;
        assert_ne!(replaced, acknowledged);
        let phase = snapshot(&catalog).phase();
        assert_eq!(phase.domains[0].pending, usize::from(replaced));
        assert_eq!(phase.domains[1].pending, 1);
    });
}

#[test]
fn a_late_helper_cannot_replace_a_newer_cell_state() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = fixture_cell(&catalog, 8);
        let insert = Arc::new(catalog.insert(snapshot(&catalog), cell.clone()).unwrap());
        assert_eq!(catalog.commit(&insert), Outcome::Committed);
        let Membership::Live(work) = cell.membership() else {
            unreachable!()
        };
        let late = {
            let catalog = catalog.clone();
            let insert = insert.clone();
            thread::spawn(move || catalog.late_help(&insert))
        };
        assert_eq!(
            catalog.commit(
                &catalog
                    .acknowledge(snapshot(&catalog), cell.clone(), 0, work[0])
                    .unwrap()
            ),
            Outcome::Committed
        );
        late.join().unwrap();
        assert_eq!(cell.membership(), Membership::Live([0, work[1]]));
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [0, 1]
        );
        assert_eq!(catalog.commit(&insert), Outcome::Committed);
    });
}

fn fixture_cell(catalog: &Catalog, address: usize) -> Arc<Cell> {
    for slot in 0..catalog.capacity() {
        if let Some(old) = catalog.discover(slot) {
            catalog.release_retired(&old);
        }
        match catalog.prepare_cell(slot, address) {
            Ok(cell) => return cell,
            Err(
                PrepareError::Occupied | PrepareError::Contended | PrepareError::BackingUnavailable,
            ) => {}
            Err(error) => panic!("cell preparation failed: {error:?}"),
        }
    }
    panic!("fixture registry exhausted");
}

struct OptionalLink<T>(Mutex<Option<Arc<T>>>);
impl<T> OptionalLink<T> {
    fn empty() -> Self {
        Self(Mutex::new(None))
    }
    fn load(&self) -> Option<Arc<T>> {
        self.0.lock().unwrap().clone()
    }
    fn compare(&self, expected: Option<&Arc<T>>, proposed: Option<Arc<T>>) -> bool {
        let mut current = self.0.lock().unwrap();
        let same = match (current.as_ref(), expected) {
            (None, None) => true,
            (Some(current), Some(expected)) => Arc::ptr_eq(current, expected),
            _ => false,
        };
        if same {
            *current = proposed;
        }
        same
    }
}

#[test]
fn registration_cancellation_and_publication_have_one_canonical_winner() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::with_capacity(1));
        let cell = catalog.prepare_cell(0, 8).unwrap();
        let insert = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
        let abandon = catalog.abandon(snapshot(&catalog), cell.clone()).unwrap();
        let publisher = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&insert))
        };
        let abandoned = catalog.commit(&abandon) == Outcome::Committed;
        let published = publisher.join().unwrap() == Outcome::Committed;
        assert_ne!(abandoned, published);
        assert_eq!(cell.membership() == Membership::Retired, abandoned);
        assert_eq!(
            snapshot(&catalog)
                .phase()
                .domains
                .map(|domain| domain.pending),
            [usize::from(published); 2]
        );
        assert_eq!(catalog.release_retired(&cell), abandoned);
        assert_eq!(catalog.discover(0).is_none(), abandoned);
    });
}

#[test]
fn late_slot_removal_does_not_remove_a_replacement() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::with_capacity(1));
        let old = catalog.prepare_cell(0, 8).unwrap();
        assert_eq!(
            catalog.commit(&catalog.abandon(snapshot(&catalog), old.clone()).unwrap()),
            Outcome::Committed
        );
        let cleaner = {
            let catalog = catalog.clone();
            let old = old.clone();
            thread::spawn(move || catalog.release_retired(&old))
        };
        catalog.release_retired(&old);
        let current = catalog.prepare_cell(0, 8).unwrap();
        cleaner.join().unwrap();
        assert!(Arc::ptr_eq(&catalog.discover(0).unwrap(), &current));
        assert!(current.identity > old.identity);
    });
}

// The protocol models retain linearizable owning pointers. The pool's own reuse
// atomics have separate exact-state models; allocation failure/credits are not
// simulated by this adapter.
type Pooled<T> = Arc<T>;
struct Pool<T>(std::marker::PhantomData<fn() -> T>);
impl<T> Pool<T> {
    fn new(_: usize) -> Arc<Self> {
        Arc::new(Self(std::marker::PhantomData))
    }
    fn occupied(&self) -> usize {
        panic!("pool accounting is not modeled by this adapter")
    }
}
fn allocate<T>(_: &Arc<Pool<T>>, value: T) -> Option<Pooled<T>> {
    Some(Arc::new(value))
}
