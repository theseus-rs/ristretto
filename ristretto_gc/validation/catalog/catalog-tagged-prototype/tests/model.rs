//! Models the exact transition source with a linearizable owning pointer CAS.
//! Mutex-backed owning loads model the link API; pointer protection, capacities,
//! generation exhaustion, byte charges and refund order are excluded.
#![allow(dead_code)]
#![cfg(not(target_family = "wasm"))]
use loom::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, AtomicUsize, Ordering};
use loom::sync::{Arc, Mutex};
use loom::thread;
#[path = "../src/trace.rs"]
mod trace;
#[path = "../src/vacancy.rs"]
mod vacancy;
use trace::{
    DomainMode, Generation, RETIREMENT_FLOOR, TraceError, TraceMark, TraceRetention, TraceState,
};
#[path = "../src/reference.rs"]
mod reference;
#[path = "../src/state.rs"]
mod state;
use state::*;

struct PooledLink<T>(Mutex<Arc<T>>);
impl<T> PooledLink<T> {
    fn points_to(&self, value: &Arc<T>) -> bool {
        Arc::ptr_eq(&self.0.lock().unwrap(), value)
    }
    fn new(_: Arc<Pool<T>>, value: Arc<T>) -> Self {
        Self(Mutex::new(value))
    }
    fn load(&self) -> Option<Arc<T>> {
        if fail_load() {
            return None;
        }
        Some(self.0.lock().unwrap().clone())
    }
    fn compare(&self, expected: &Arc<T>, proposed: Arc<T>) -> bool {
        let mut current = self.0.lock().unwrap();
        if Arc::ptr_eq(&current, expected) {
            *current = proposed;
            true
        } else {
            false
        }
    }
}
#[derive(Debug)]
struct Contended;
std::thread_local! {
    static FAIL_SHARE: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
    static FAIL_LOAD: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
    static FAIL_ALLOCATION: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
}
fn fault(counter: &'static std::thread::LocalKey<std::cell::Cell<Option<usize>>>) -> bool {
    counter.with(|remaining| match remaining.get() {
        Some(0) => {
            remaining.set(None);
            true
        }
        Some(n) => {
            remaining.set(Some(n - 1));
            false
        }
        None => false,
    })
}
fn fail_load() -> bool {
    fault(&FAIL_LOAD)
}
fn share<T>(value: &Pooled<T>) -> Option<Pooled<T>> {
    if fault(&FAIL_SHARE) {
        None
    } else {
        Some(value.clone())
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
        assert!(matches!(cell.membership().unwrap(), Membership::Live(_)));
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
        assert_eq!(
            cell.membership().unwrap() == Membership::Prepared,
            cancelled
        );
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
        let advance = advance_cycle(&catalog, 0).unwrap();
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
        let Membership::Live(work) = cell.membership().unwrap() else {
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
        let Membership::Live(work) = cell.membership().unwrap() else {
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
        assert_eq!(cell.membership().unwrap(), Membership::Live([0, work[1]]));
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [0, 1]
        );
        assert_eq!(catalog.commit(&insert), Outcome::Committed);
    });
}

fn fixture_cell(catalog: &Catalog, address: usize) -> Arc<Cell> {
    for slot in 0..catalog.capacity() {
        if let Some(old) = catalog.discover(slot).unwrap() {
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
    fn empty(_: Arc<Pool<T>>) -> Self {
        Self(Mutex::new(None))
    }
    fn is_occupied(&self) -> bool {
        self.0.lock().unwrap().is_some()
    }
    fn load(&self) -> Result<Option<Arc<T>>, Contended> {
        if fail_load() {
            return Err(Contended);
        }
        Ok(self.0.lock().unwrap().clone())
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
        assert_eq!(cell.membership().unwrap() == Membership::Retired, abandoned);
        assert_eq!(
            snapshot(&catalog)
                .phase()
                .domains
                .map(|domain| domain.pending),
            [usize::from(published); 2]
        );
        assert_eq!(catalog.release_retired(&cell), abandoned);
        assert_eq!(catalog.discover(0).unwrap().is_none(), abandoned);
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
        assert!(Arc::ptr_eq(
            &catalog.discover(0).unwrap().unwrap(),
            &current
        ));
        assert!(current.identity > old.identity);
    });
}

// The protocol models retain linearizable owning pointers. The pool's own reuse
// atomics have separate exact-state models. One selected control-allocation
// failure can be injected; pool occupancy, selection and byte credits are not
// simulated by this adapter.
type Pooled<T> = Arc<T>;
struct Pool<T>(std::marker::PhantomData<fn() -> T>);
impl<T> Pool<T> {
    // Generation exhaustion is checked in the exact pool model/native facade;
    // this owning-pointer adapter models selected transient allocation failure.
    fn with_generation_limit(capacity: usize, _: u64) -> Arc<Self> {
        Self::new(capacity)
    }
    fn try_allocate_at_status(
        &self,
        index: usize,
        value: T,
    ) -> Result<Pooled<T>, pool::AllocationFailure<T>> {
        self.try_allocate_at(index, value)
            .map_err(|value| pool::AllocationFailure {
                value,
                error: pool::AllocationError::Unavailable,
            })
    }
    fn generation_exhaustion_observed(&self) -> bool {
        false
    }
    fn try_allocate_status(&self, value: T) -> Result<Pooled<T>, pool::AllocationFailure<T>> {
        self.try_allocate(value)
            .map_err(|value| pool::AllocationFailure {
                value,
                error: pool::AllocationError::Unavailable,
            })
    }

    // Layout queries are outside this pointer-transition model.
    fn requested_backing_bytes(_: usize) -> Option<usize> {
        Some(0)
    }

    fn new(_: usize) -> Arc<Self> {
        Arc::new(Self(std::marker::PhantomData))
    }
    fn try_allocate(&self, value: T) -> Result<Pooled<T>, T> {
        if fault(&FAIL_ALLOCATION) {
            Err(value)
        } else {
            Ok(Arc::new(value))
        }
    }
    fn try_allocate_at(&self, _: usize, value: T) -> Result<Pooled<T>, T> {
        Ok(Arc::new(value))
    }
    fn occupied(&self) -> usize {
        panic!("pool accounting is not modeled by this adapter")
    }
}
fn allocate<T>(_: &Arc<Pool<T>>, value: T) -> Option<Pooled<T>> {
    Some(Arc::new(value))
}

#[test]
fn pending_request_recovers_after_effect_acquisition_failure() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = fixture_cell(&catalog, 8);
        let attempt = Arc::new(catalog.insert(snapshot(&catalog), cell.clone()).unwrap());
        FAIL_SHARE.with(|remaining| remaining.set(Some(1)));
        assert_eq!(catalog.commit(&attempt), Outcome::Pending);
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
    });
}

#[test]
fn failed_head_observation_does_not_discard_an_installed_request() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = fixture_cell(&catalog, 8);
        let attempt = Arc::new(catalog.insert(snapshot(&catalog), cell.clone()).unwrap());
        assert!(catalog.install_and_stop(&attempt));
        FAIL_LOAD.with(|remaining| remaining.set(Some(0)));
        assert_eq!(catalog.commit(&attempt), Outcome::Pending);
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
    });
}

#[test]
fn discovery_failure_reports_contention_without_hiding_membership() {
    loom::model(|| {
        let catalog = Catalog::new();
        let cell = fixture_cell(&catalog, 8);
        FAIL_LOAD.with(|remaining| remaining.set(Some(0)));
        assert!(matches!(catalog.discover(cell.slot()), Err(Contended)));
        assert!(Arc::ptr_eq(
            &catalog.discover(cell.slot()).unwrap().unwrap(),
            &cell
        ));
    });
}

#[test]
fn an_applied_effect_needs_no_fallible_owning_reread() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = fixture_cell(&catalog, 8);
        let attempt = Arc::new(catalog.insert(snapshot(&catalog), cell.clone()).unwrap());
        catalog.effect_and_stop(&attempt);
        FAIL_LOAD.with(|remaining| remaining.set(Some(1)));
        assert_eq!(catalog.commit(&attempt), Outcome::Committed);
        assert_eq!(FAIL_LOAD.with(|remaining| remaining.replace(None)), Some(0));
        assert!(!attempt.cancel());
        let helper = {
            let catalog = catalog.clone();
            let attempt = attempt.clone();
            thread::spawn(move || catalog.commit(&attempt))
        };
        assert_eq!(helper.join().unwrap(), Outcome::Committed);
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [1, 1]
        );
    });
}

#[test]
fn failed_effect_preparation_can_cancel_before_a_later_helper_commits() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = fixture_cell(&catalog, 8);
        let attempt = Arc::new(catalog.insert(snapshot(&catalog), cell.clone()).unwrap());
        assert!(catalog.install_and_stop(&attempt));
        FAIL_SHARE.with(|remaining| remaining.set(Some(0)));
        assert!(catalog.snapshot().is_none());
        assert!(
            attempt.cancel(),
            "effect preparation made an irreversible commit decision"
        );
        let helper = {
            let catalog = catalog.clone();
            let attempt = attempt.clone();
            thread::spawn(move || catalog.commit(&attempt))
        };
        assert_eq!(catalog.commit(&attempt), Outcome::Cancelled);
        assert_eq!(helper.join().unwrap(), Outcome::Cancelled);
        assert_eq!(cell.membership(), Some(Membership::Prepared));
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [0, 0]
        );
    });
}

#[test]
fn partial_preparation_cannot_refresh_its_expected_controller() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = fixture_cell(&catalog, 8);
        let mut preparation = catalog
            .begin_insert(snapshot(&catalog), cell.clone())
            .unwrap();
        assert!(preparation.poll(&catalog).unwrap().is_none());
        let other = advance_cycle(&catalog, 0).unwrap();
        let competitor = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&other))
        };
        assert!(preparation.poll(&catalog).unwrap().is_none());
        let attempt = preparation.poll(&catalog).unwrap().unwrap();
        let committed = catalog.commit(&attempt) == Outcome::Committed;
        assert_ne!(committed, competitor.join().unwrap() == Outcome::Committed);
        let phase = snapshot(&catalog).phase();
        assert_eq!(phase.domains[0].floor, u8::from(!committed));
        assert_eq!(
            phase.domains.map(|d| d.pending),
            [usize::from(committed); 2]
        );
        assert_eq!(
            matches!(cell.membership(), Some(Membership::Live(_))),
            committed
        );
    });
}

#[test]
fn a_failed_final_allocation_keeps_private_outcomes_for_handoff() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = fixture_cell(&catalog, 8);
        let mut preparation = catalog
            .begin_insert(snapshot(&catalog), cell.clone())
            .unwrap();
        FAIL_ALLOCATION.with(|remaining| remaining.set(Some(2)));
        for _ in 0..3 {
            assert!(preparation.poll(&catalog).unwrap().is_none());
        }
        let helper = {
            let catalog = catalog.clone();
            thread::spawn(move || preparation.poll(&catalog).unwrap().unwrap())
        };
        let attempt = helper.join().unwrap();
        assert_eq!(catalog.commit(&attempt), Outcome::Committed);
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [1, 1]
        );
    });
}

#[test]
fn two_resumable_creations_announce_only_one_exact_record() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let mut a = catalog.begin_create(0, 8).unwrap();
        let mut b = catalog.begin_create(0, 16).unwrap();
        for _ in 0..2 {
            assert!(a.poll(&catalog).unwrap().is_none());
            assert!(b.poll(&catalog).unwrap().is_none());
        }
        let worker = {
            let catalog = catalog.clone();
            thread::spawn(move || a.poll(&catalog))
        };
        let right = b.poll(&catalog);
        let left = worker.join().unwrap();
        let winner = match (left, right) {
            (Ok(Some(cell)), Err(CreationError::Occupied))
            | (Err(CreationError::Occupied), Ok(Some(cell))) => cell,
            _ => panic!("announcements did not have exactly one winner"),
        };
        assert_eq!(
            catalog.discover(0).unwrap().unwrap().identity,
            winner.identity
        );
        assert_eq!(winner.membership(), Some(Membership::Prepared));
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [0, 0]
        );
    });
}

#[test]
fn resumable_creation_retries_a_failed_private_allocation_without_announcing() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let mut creation = catalog.begin_create(0, 8).unwrap();
        assert!(creation.poll(&catalog).unwrap().is_none());
        FAIL_ALLOCATION.with(|counter| counter.set(Some(0)));
        assert!(creation.poll(&catalog).unwrap().is_none());
        assert!(catalog.discover(0).unwrap().is_none());
        let worker = {
            let catalog = catalog.clone();
            thread::spawn(move || {
                assert!(creation.poll(&catalog).unwrap().is_none());
                creation.poll(&catalog).unwrap().unwrap()
            })
        };
        let cell = worker.join().unwrap();
        assert_eq!(
            catalog.discover(0).unwrap().unwrap().identity,
            cell.identity
        );
        assert_eq!(cell.membership(), Some(Membership::Prepared));
    });
}

fn trace_prepared(catalog: &Catalog, mut preparation: Preparation) -> Attempt {
    for _ in 0..4 {
        if let Some(attempt) = preparation.poll(catalog).unwrap() {
            return attempt;
        }
    }
    panic!("quiescent tracing preparation did not settle");
}
fn trace_commit(catalog: &Catalog, preparation: Preparation) {
    assert_eq!(
        catalog.commit(&trace_prepared(catalog, preparation)),
        Outcome::Committed
    );
}
fn traced_cell(catalog: &Catalog) -> Pooled<Cell> {
    let cell = fixture_cell(catalog, 8);
    trace_commit(
        catalog,
        catalog
            .begin_trace_insert(snapshot(catalog), cell.clone())
            .unwrap(),
    );
    cell
}
fn trace_marks(cell: &Pooled<Cell>) -> [TraceMark; 2] {
    let Membership::Tracing(marks) = cell.membership().unwrap() else {
        panic!()
    };
    marks.marks
}

#[test]
fn trace_discovery_and_phase_closure_cannot_both_win() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = traced_cell(&catalog);
        let shade = trace_prepared(
            &catalog,
            catalog
                .begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Strong)
                .unwrap(),
        );
        let advance = advance_cycle(&catalog, 0).unwrap();
        let worker = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&shade))
        };
        let closed = catalog.commit(&advance) == Outcome::Committed;
        let marked = worker.join().unwrap() == Outcome::Committed;
        assert_ne!(closed, marked);
        let phase = snapshot(&catalog).phase();
        assert_eq!(phase.domains[0].floor, u8::from(closed));
        assert_eq!(phase.domains[0].pending, usize::from(marked));
        let mark = trace_marks(&cell)[0];
        assert_eq!(mark.retention, marked.then_some(TraceRetention::Strong));
        assert_eq!(mark.ticket != 0, marked);
    });
}

#[test]
fn stronger_canonical_trace_work_excludes_an_old_acknowledgement() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = traced_cell(&catalog);
        trace_commit(
            &catalog,
            catalog
                .begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Phantom)
                .unwrap(),
        );
        let ticket = trace_marks(&cell)[0].ticket;
        let ack = trace_prepared(
            &catalog,
            catalog
                .begin_trace_acknowledge(snapshot(&catalog), cell.clone(), 0, ticket)
                .unwrap(),
        );
        let shade = trace_prepared(
            &catalog,
            catalog
                .begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Strong)
                .unwrap(),
        );
        let worker = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&ack))
        };
        let stronger = catalog.commit(&shade) == Outcome::Committed;
        let acknowledged = worker.join().unwrap() == Outcome::Committed;
        assert_ne!(stronger, acknowledged);
        let mark = trace_marks(&cell)[0];
        assert_eq!(
            mark.retention,
            Some(if stronger {
                TraceRetention::Strong
            } else {
                TraceRetention::Phantom
            })
        );
        assert_eq!(
            snapshot(&catalog).phase().domains[0].pending,
            usize::from(stronger)
        );
        assert_eq!(mark.ticket != 0, stronger);
        if stronger {
            assert_ne!(mark.ticket, ticket);
        }
    });
}

#[test]
fn old_trace_helper_cannot_overwrite_new_epoch_reachability() {
    let mut builder = loom::model::Builder::new();
    // Six real floor transitions precede the race; allow that deterministic setup.
    builder.max_branches = 10000;
    builder.check(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = traced_cell(&catalog);
        let old = trace_prepared(
            &catalog,
            catalog
                .begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Strong)
                .unwrap(),
        );
        catalog.effect_and_stop(&old);
        assert_eq!(catalog.commit(&old), Outcome::Committed);
        let ticket = trace_marks(&cell)[0].ticket;
        trace_commit(
            &catalog,
            catalog
                .begin_trace_acknowledge(snapshot(&catalog), cell.clone(), 0, ticket)
                .unwrap(),
        );
        for _ in 0..=RETIREMENT_FLOOR {
            assert_eq!(
                catalog.commit(&advance_cycle(&catalog, 0).unwrap()),
                Outcome::Committed
            );
        }
        let next = trace_prepared(
            &catalog,
            catalog
                .begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Final)
                .unwrap(),
        );
        let worker = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.late_help(&old))
        };
        assert_eq!(catalog.commit(&next), Outcome::Committed);
        worker.join().unwrap();
        let mark = trace_marks(&cell)[0];
        assert_eq!(mark.epoch, 2);
        assert_eq!(mark.retention, Some(TraceRetention::Final));
        assert_ne!(mark.ticket, 0);
        assert_eq!(snapshot(&catalog).phase().domains[0].pending, 1);
    });
}

#[test]
fn trace_domains_keep_independent_epochs_floors_and_work() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::with_phase(Phase {
            root_cursors: [0; 2],
            reference_cursors: [0; 2],
            revision: 1,
            domains: [
                Domain {
                    mode: DomainMode::Full,
                    epoch: 1,
                    floor: 0,
                    pending: 0,
                },
                Domain {
                    mode: DomainMode::Full,
                    epoch: 7,
                    floor: 3,
                    pending: 0,
                },
            ],
        }));
        let cell = traced_cell(&catalog);
        let first = trace_prepared(
            &catalog,
            catalog
                .begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Strong)
                .unwrap(),
        );
        let second = trace_prepared(
            &catalog,
            catalog
                .begin_shade(snapshot(&catalog), cell.clone(), 1, TraceRetention::Strong)
                .unwrap(),
        );
        let worker = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&second))
        };
        let won = catalog.commit(&first) == Outcome::Committed;
        assert_ne!(worker.join().unwrap() == Outcome::Committed, won);
        let loser = usize::from(won);
        trace_commit(
            &catalog,
            catalog
                .begin_shade(
                    snapshot(&catalog),
                    cell.clone(),
                    loser,
                    TraceRetention::Strong,
                )
                .unwrap(),
        );
        let marks = trace_marks(&cell);
        assert_eq!(marks.map(|m| m.epoch), [1, 7]);
        assert_eq!(
            marks.map(|m| m.retention),
            [Some(TraceRetention::Strong), Some(TraceRetention::Phantom)]
        );
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [1, 1]
        );
        trace_commit(
            &catalog,
            catalog
                .begin_trace_acknowledge(snapshot(&catalog), cell.clone(), 0, marks[0].ticket)
                .unwrap(),
        );
        assert_eq!(
            catalog.commit(&advance_cycle(&catalog, 0).unwrap()),
            Outcome::Committed
        );
        assert_eq!(trace_marks(&cell)[1], marks[1]);
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [0, 1]
        );
    });
}

#[test]
fn a_helper_stopped_before_its_cas_cannot_rewind_a_later_membership() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = fixture_cell(&catalog, 8);
        let insert = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
        assert!(catalog.install_and_stop(&insert));
        let (entered, entry) = loom::sync::mpsc::channel();
        let (resume, resumed) = loom::sync::mpsc::channel();
        let stopped = {
            let catalog = catalog.clone();
            thread::spawn(move || {
                catalog.help_at(0, || {
                    entered.send(()).unwrap();
                    resumed.recv().unwrap();
                    FAIL_LOAD.with(|remaining| remaining.set(Some(0)));
                });
                assert_eq!(FAIL_LOAD.with(|remaining| remaining.replace(None)), Some(0));
            })
        };
        entry.recv().unwrap();
        assert_eq!(catalog.commit(&insert), Outcome::Committed);
        let Membership::Live(work) = cell.membership().unwrap() else {
            unreachable!()
        };
        let ack = catalog
            .acknowledge(snapshot(&catalog), cell.clone(), 0, work[0])
            .unwrap();
        assert_eq!(catalog.commit(&ack), Outcome::Committed);
        resume.send(()).unwrap();
        stopped.join().unwrap();
        assert_eq!(
            cell.membership(),
            Some(Membership::Live([0, work[1]])),
            "late effect rewound membership"
        );
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [0, 1]
        );
    });
}

// Existing lifecycle fixtures explicitly end and restart a terminal cycle.
fn advance_cycle(catalog: &Catalog, domain: usize) -> Option<Attempt> {
    let before = snapshot(catalog);
    let entry = before.phase().domains[domain];
    if entry.floor == RETIREMENT_FLOOR {
        let finish = trace_prepared(catalog, catalog.begin_finish(before, domain)?);
        assert_eq!(catalog.commit(&finish), Outcome::Committed);
        Some(trace_prepared(
            catalog,
            catalog.begin_start(snapshot(catalog), domain, entry.mode)?,
        ))
    } else {
        catalog.advance(before, domain)
    }
}

#[test]
fn publication_and_strong_closure_cannot_both_commit() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let cell = fixture_cell(&catalog, 8);
        let publish = trace_prepared(
            &catalog,
            catalog
                .begin_publish(snapshot(&catalog), cell.clone())
                .unwrap(),
        );
        let close = catalog.advance(snapshot(&catalog), 0).unwrap();
        let worker = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&publish))
        };
        let closed = catalog.commit(&close) == Outcome::Committed;
        let published = worker.join().unwrap() == Outcome::Committed;
        assert_ne!(closed, published);
        let phase = snapshot(&catalog).phase();
        if published {
            let Membership::Tracing(state) = cell.membership().unwrap() else {
                panic!()
            };
            assert_eq!(state.generation, Generation::Young);
            assert_eq!(phase.domains.map(|d| d.pending), [1, 1]);
            assert!(state.marks.iter().all(|m| m.ticket != 0));
            assert_eq!(phase.domains[0].floor, 0);
        } else {
            assert_eq!(cell.membership(), Some(Membership::Prepared));
            assert_eq!(phase.domains.map(|d| d.pending), [0, 0]);
            assert_eq!(phase.domains[0].floor, 1);
        }
    });
}

#[test]
fn idle_publication_and_domain_start_have_one_exact_phase_winner() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::with_phase(Phase {
            root_cursors: [0; 2],
            reference_cursors: [0; 2],
            revision: 1,
            domains: [Domain {
                mode: DomainMode::Inactive,
                epoch: 1,
                floor: 0,
                pending: 0,
            }; 2],
        }));
        let cell = fixture_cell(&catalog, 8);
        let publish = trace_prepared(
            &catalog,
            catalog
                .begin_publish(snapshot(&catalog), cell.clone())
                .unwrap(),
        );
        let start = trace_prepared(
            &catalog,
            catalog
                .begin_start(snapshot(&catalog), 1, DomainMode::Full)
                .unwrap(),
        );
        let worker = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&publish))
        };
        let started = catalog.commit(&start) == Outcome::Committed;
        let published = worker.join().unwrap() == Outcome::Committed;
        assert_ne!(started, published);
        let phase = snapshot(&catalog).phase();
        assert_eq!(phase.domains.map(|d| d.pending), [0, 0]);
        if published {
            let Membership::Tracing(state) = cell.membership().unwrap() else {
                panic!()
            };
            assert!(!state[1].marked(1));
            assert_eq!(phase.domains[1].mode, DomainMode::Inactive);
        } else {
            assert_eq!(cell.membership(), Some(Membership::Prepared));
            assert_eq!(phase.domains[1].epoch, 2);
            assert_eq!(phase.domains[1].mode, DomainMode::Full);
        }
    });
}

#[test]
fn old_promotion_and_old_closure_cannot_lose_handoff_work() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::with_phase(Phase {
            root_cursors: [0; 2],
            reference_cursors: [0; 2],
            revision: 1,
            domains: [
                Domain {
                    mode: DomainMode::Young,
                    epoch: 1,
                    floor: 0,
                    pending: 0,
                },
                Domain {
                    mode: DomainMode::Old,
                    epoch: 1,
                    floor: 0,
                    pending: 0,
                },
            ],
        }));
        let cell = fixture_cell(&catalog, 8);
        trace_commit(
            &catalog,
            catalog
                .begin_publish(snapshot(&catalog), cell.clone())
                .unwrap(),
        );
        let promote = trace_prepared(
            &catalog,
            catalog
                .begin_promote(snapshot(&catalog), cell.clone())
                .unwrap(),
        );
        let close = catalog.advance(snapshot(&catalog), 1).unwrap();
        let worker = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&promote))
        };
        let closed = catalog.commit(&close) == Outcome::Committed;
        let promoted = worker.join().unwrap() == Outcome::Committed;
        assert_ne!(closed, promoted);
        let Membership::Tracing(state) = cell.membership().unwrap() else {
            panic!()
        };
        let phase = snapshot(&catalog).phase();
        assert_eq!(state.old_birth, 1);
        assert_ne!(state[0].ticket, 0, "promotion erased pending young work");
        assert_eq!(phase.domains[0].pending, 1);
        assert_eq!(phase.domains[1].pending, usize::from(promoted));
        assert_eq!(state[1].ticket != 0, promoted);
        assert_eq!(state.generation == Generation::Old, promoted);
        assert_eq!(state[1].born, !promoted);
    });
}

#[test]
fn late_publication_helpers_cannot_restore_pre_promotion_membership() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::with_phase(Phase {
            root_cursors: [0; 2],
            reference_cursors: [0; 2],
            revision: 1,
            domains: [
                Domain {
                    mode: DomainMode::Inactive,
                    epoch: 1,
                    floor: 0,
                    pending: 0,
                },
                Domain {
                    mode: DomainMode::Old,
                    epoch: 1,
                    floor: 0,
                    pending: 0,
                },
            ],
        }));
        let cell = fixture_cell(&catalog, 8);
        let publish = Arc::new(trace_prepared(
            &catalog,
            catalog
                .begin_publish(snapshot(&catalog), cell.clone())
                .unwrap(),
        ));
        assert!(catalog.install_and_stop(&publish));
        let (entered, entry) = loom::sync::mpsc::channel();
        let (resume, resumed) = loom::sync::mpsc::channel();
        let late = {
            let catalog = catalog.clone();
            thread::spawn(move || {
                catalog.help_at(0, || {
                    entered.send(()).unwrap();
                    resumed.recv().unwrap();
                })
            })
        };
        entry.recv().unwrap();
        assert_eq!(catalog.commit(&publish), Outcome::Committed);
        trace_commit(
            &catalog,
            catalog
                .begin_promote(snapshot(&catalog), cell.clone())
                .unwrap(),
        );
        resume.send(()).unwrap();
        late.join().unwrap();
        let Membership::Tracing(state) = cell.membership().unwrap() else {
            panic!()
        };
        assert_eq!(state.generation, Generation::Old);
        assert!(!state[1].born);
        assert_ne!(state[1].ticket, 0);
        assert_eq!(snapshot(&catalog).phase().domains[1].pending, 1);
    });
}

#[test]
fn retirement_birth_publication_and_domain_finish_cannot_share_a_stale_phase() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::with_phase(Phase {
            root_cursors: [0; 2],
            reference_cursors: [0; 2],
            revision: 1,
            domains: [
                Domain {
                    mode: DomainMode::Inactive,
                    epoch: 1,
                    floor: 0,
                    pending: 0,
                },
                Domain {
                    mode: DomainMode::Full,
                    epoch: 1,
                    floor: RETIREMENT_FLOOR,
                    pending: 0,
                },
            ],
        }));
        let cell = fixture_cell(&catalog, 8);
        let publish = trace_prepared(
            &catalog,
            catalog
                .begin_publish(snapshot(&catalog), cell.clone())
                .unwrap(),
        );
        let finish = trace_prepared(
            &catalog,
            catalog.begin_finish(snapshot(&catalog), 1).unwrap(),
        );
        let worker = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&publish))
        };
        let finished = catalog.commit(&finish) == Outcome::Committed;
        let published = worker.join().unwrap() == Outcome::Committed;
        assert_ne!(finished, published);
        let phase = snapshot(&catalog).phase();
        assert_eq!(phase.domains.map(|d| d.pending), [0, 0]);
        if published {
            let Membership::Tracing(state) = cell.membership().unwrap() else {
                panic!()
            };
            assert!(state[1].born);
            assert!(!state[1].marked_before(1, true));
            assert_eq!(state.can_retire(phase, 1), Err(TraceError::Conflict));
        } else {
            assert_eq!(cell.membership(), Some(Membership::Prepared));
            assert_eq!(phase.domains[1].mode, DomainMode::Inactive);
        }
    });
}

#[path = "../src/record_fixture.rs"]
mod record_fixture;

fn record_catalog() -> Catalog<record_fixture::Fixture> {
    Catalog::with_record_capacities_and_limit(1, 64, 64, 32, pool::MAX_GENERATION)
}
fn record_snapshot(
    catalog: &Catalog<record_fixture::Fixture>,
) -> Snapshot<record_fixture::Fixture> {
    for _ in 0..8 {
        if let Some(s) = catalog.snapshot_in(&7) {
            return s;
        }
    }
    panic!("record controller did not settle")
}
fn record_cell(
    catalog: &Catalog<record_fixture::Fixture>,
    record: record_fixture::Fixture,
) -> Pooled<Cell<record_fixture::Fixture>> {
    let mut creation = catalog.begin_create_with(0, 8, record).unwrap();
    for _ in 0..8 {
        if let Some(c) = creation.poll(catalog).unwrap() {
            return c;
        }
    }
    panic!("record creation did not settle")
}
fn record_publish(
    catalog: &Catalog<record_fixture::Fixture>,
    cell: &Pooled<Cell<record_fixture::Fixture>>,
) -> Attempt<record_fixture::Fixture> {
    let mut preparation = catalog
        .begin_publish(record_snapshot(catalog), cell.clone())
        .unwrap();
    for _ in 0..8 {
        if let Some(a) = preparation.poll(catalog).unwrap() {
            return a;
        }
    }
    panic!("record preparation did not settle")
}

#[test]
fn final_private_closure_cannot_race_an_unprotected_canonical_commit() {
    loom::model(|| {
        let catalog = record_catalog();
        let record = record_fixture::Fixture::new();
        let cell = record_cell(&catalog, record.clone());
        let publish = record_publish(&catalog, &cell);
        let closer = {
            let record = record.clone();
            thread::spawn(move || record.release_origin())
        };
        let first = catalog.commit_in(&publish, &7);
        closer.join().unwrap();
        let outcome = if first == Outcome::Pending {
            catalog.commit_in(&publish, &7)
        } else {
            first
        };
        assert!(matches!(outcome, Outcome::Committed | Outcome::Cancelled));
        assert_eq!(record.0.handles.load(Ordering::Acquire), 0);
        assert_eq!(
            record.0.last_release.load(Ordering::Acquire),
            u8::from(outcome == Outcome::Committed),
            "closed graph cleanup lost canonical ownership"
        );
        assert_eq!(
            record_snapshot(&catalog).phase().domains.map(|d| d.pending),
            [usize::from(outcome == Outcome::Committed); 2]
        );
        assert_eq!(
            matches!(cell.membership(), Some(Membership::Tracing(_))),
            outcome == Outcome::Committed
        );
    });
}

#[test]
fn a_guarded_helper_can_finish_after_origin_closure_and_another_helper_certificate() {
    loom::model(|| {
        let catalog = Arc::new(record_catalog());
        let record = record_fixture::Fixture::new();
        let cell = record_cell(&catalog, record.clone());
        let publish = record_publish(&catalog, &cell);
        assert!(catalog.install_and_stop(&publish));
        let (entered, entry) = loom::sync::mpsc::channel();
        let (resume, resumed) = loom::sync::mpsc::channel();
        let delayed = {
            let catalog = catalog.clone();
            thread::spawn(move || {
                catalog.help_record_at(&7, 0, || {
                    entered.send(()).unwrap();
                    resumed.recv().unwrap();
                });
            })
        };
        entry.recv().unwrap();
        record.release_origin();
        assert_eq!(record.0.handles.load(Ordering::Acquire), 1);
        assert_eq!(catalog.commit_in(&publish, &7), Outcome::Committed);
        assert_eq!(
            record_snapshot(&catalog).phase().domains.map(|d| d.pending),
            [1, 1]
        );
        resume.send(()).unwrap();
        delayed.join().unwrap();
        assert_eq!(record.0.handles.load(Ordering::Acquire), 0);
        assert_eq!(record.0.last_release.load(Ordering::Acquire), 1);
        assert_eq!(record.0.published.load(Ordering::Acquire), 1);
    });
}

#[test]
fn certified_controller_release_needs_no_private_record_admission() {
    loom::model(|| {
        let catalog = record_catalog();
        let record = record_fixture::Fixture::new();
        let cell = record_cell(&catalog, record.clone());
        let publish = record_publish(&catalog, &cell);
        FAIL_SHARE.with(|n| n.set(Some(2)));
        assert_eq!(catalog.commit_in(&publish, &7), Outcome::Committed);
        record.release_origin();
        assert_eq!(record.0.handles.load(Ordering::Acquire), 0);
        let enters = record.0.enters.load(Ordering::Acquire);
        assert!(catalog.snapshot_in(&9).is_none());
        assert_eq!(record.0.enters.load(Ordering::Acquire), enters);
        assert_eq!(
            record_snapshot(&catalog).phase().domains.map(|d| d.pending),
            [1, 1]
        );
        assert_eq!(record.0.last_release.load(Ordering::Acquire), 1);
    });
}

#[test]
fn cancellation_and_record_publication_agree_with_final_cleanup_ownership() {
    loom::model(|| {
        let catalog = record_catalog();
        let record = record_fixture::Fixture::new();
        let cell = record_cell(&catalog, record.clone());
        let publish = Arc::new(record_publish(&catalog, &cell));
        let cancel = {
            let publish = publish.clone();
            thread::spawn(move || publish.cancel())
        };
        let outcome = catalog.commit_in(&publish, &7);
        let cancelled = cancel.join().unwrap();
        assert_eq!(outcome == Outcome::Cancelled, cancelled);
        record.release_origin();
        assert_eq!(
            record.0.last_release.load(Ordering::Acquire),
            u8::from(outcome == Outcome::Committed),
            "cancelled graph was labelled published"
        );
        assert_eq!(
            record.0.published.load(Ordering::Acquire),
            u8::from(outcome == Outcome::Committed)
        );
    });
}

fn root_catalog() -> Catalog {
    Catalog::with_record_root_capacities_and_limit(1, 64, 64, 32, 1, 16, pool::MAX_GENERATION)
}
fn model_root(catalog: &Catalog, cell: &Pooled<Cell>) -> RootToken {
    assert!(matches!(
        catalog.begin_create_root(1, cell.clone()),
        Err(RootPrepareError::InvalidSlot)
    ));
    let mut creation: RootCreation = catalog.begin_create_root(0, cell.clone()).unwrap();
    for _ in 0..8 {
        if let Some(root) = creation.poll(catalog).unwrap() {
            let observed: &Root = &root;
            assert_eq!(observed.status(), RootStatus::Prepared);
            return root;
        }
    }
    panic!("root creation did not settle")
}
fn root_commit(catalog: &Catalog, preparation: Option<Preparation>) {
    let attempt = trace_prepared(catalog, preparation.expect("valid root preparation"));
    assert_eq!(catalog.commit(&attempt), Outcome::Committed);
}
fn root_drain(catalog: &Catalog, cell: &Pooled<Cell>) {
    let Some(Membership::Tracing(marks)) = cell.membership() else {
        panic!();
    };
    for domain in 0..2 {
        if marks[domain].ticket != 0 {
            root_commit(
                catalog,
                catalog.begin_trace_acknowledge(
                    snapshot(catalog),
                    cell.clone(),
                    domain,
                    marks[domain].ticket,
                ),
            );
        }
    }
}
fn root_new_epoch(catalog: &Catalog, cell: &Pooled<Cell>) {
    root_drain(catalog, cell);
    root_commit(catalog, catalog.begin_scan_root(snapshot(catalog), 0));
    for _ in 0..RETIREMENT_FLOOR {
        root_commit(catalog, catalog.begin_advance(snapshot(catalog), 0));
    }
    root_commit(catalog, catalog.begin_finish(snapshot(catalog), 0));
    root_commit(
        catalog,
        catalog.begin_start(snapshot(catalog), 0, DomainMode::Full),
    );
}

#[test]
fn root_and_object_effects_finish_before_a_later_phase_is_observable() {
    loom::model(|| {
        let catalog = Arc::new(root_catalog());
        let cell = fixture_cell(&catalog, 8);
        let root = model_root(&catalog, &cell);
        let attempt = trace_prepared(
            &catalog,
            catalog
                .begin_publish_root(snapshot(&catalog), root.try_share().unwrap())
                .unwrap(),
        );
        assert!(catalog.install_and_stop(&attempt));
        let (entered, entry) = loom::sync::mpsc::channel();
        let (resume, resumed) = loom::sync::mpsc::channel();
        let helper = {
            let catalog = catalog.clone();
            thread::spawn(move || {
                catalog.help_record_at(&(), 1, || {
                    entered.send(()).unwrap();
                    resumed.recv().unwrap();
                })
            })
        };
        entry.recv().unwrap();
        assert_eq!(root.status(), RootStatus::Prepared);
        assert_eq!(catalog.commit(&attempt), Outcome::Committed);
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [1, 1]
        );
        assert_eq!(root.status(), RootStatus::Live);
        resume.send(()).unwrap();
        helper.join().unwrap();
    });
}

#[test]
fn root_cancellation_and_publication_share_the_same_irreversible_decision() {
    loom::model(|| {
        let catalog = root_catalog();
        let cell = fixture_cell(&catalog, 8);
        let root = model_root(&catalog, &cell);
        let attempt = Arc::new(trace_prepared(
            &catalog,
            catalog
                .begin_publish_root(snapshot(&catalog), root.try_share().unwrap())
                .unwrap(),
        ));
        let canceller = {
            let attempt = attempt.clone();
            thread::spawn(move || attempt.cancel())
        };
        let outcome = catalog.commit(&attempt);
        let cancelled = canceller.join().unwrap();
        assert_eq!(outcome == Outcome::Cancelled, cancelled);
        assert_eq!(root.status() == RootStatus::Prepared, cancelled);
        assert_eq!(cell.membership() == Some(Membership::Prepared), cancelled);
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [usize::from(!cancelled); 2]
        );
    });
}

#[test]
fn root_drop_and_scan_preserve_the_deletion_edge_before_phase_closure() {
    loom::model(|| {
        let catalog = root_catalog();
        let cell = fixture_cell(&catalog, 8);
        let root = model_root(&catalog, &cell);
        root_commit(
            &catalog,
            catalog.begin_publish_root(snapshot(&catalog), root.try_share().unwrap()),
        );
        root_new_epoch(&catalog, &cell);
        let retained = root.try_share().unwrap();
        let closer = thread::spawn(move || drop(root));
        root_commit(&catalog, catalog.begin_scan_root(snapshot(&catalog), 0));
        closer.join().unwrap();
        let phase = snapshot(&catalog).phase();
        assert_eq!(phase.root_cursors[0], 1);
        assert_eq!(phase.domains[0].pending, 1, "drop lost root deletion work");
        assert!(catalog.begin_advance(snapshot(&catalog), 0).is_none());
        if retained.status() == RootStatus::Live {
            root_commit(
                &catalog,
                catalog.begin_requested_root_removal(snapshot(&catalog), retained.clone()),
            );
        }
        assert_eq!(retained.status(), RootStatus::Retired);
        assert!(catalog.release_retired_root(&retained));
        assert_eq!(snapshot(&catalog).phase().domains[0].pending, 1);
    });
}

#[test]
fn root_revocation_does_not_reuse_a_slot_before_unique_token_drop() {
    loom::model(|| {
        let catalog = root_catalog();
        let cell = fixture_cell(&catalog, 8);
        let root = model_root(&catalog, &cell);
        let retained = root.try_share().unwrap();
        root_commit(
            &catalog,
            catalog.begin_cancel_root(snapshot(&catalog), retained.clone()),
        );
        assert!(!catalog.release_retired_root(&retained));
        assert!(matches!(
            catalog.begin_create_root(0, cell.clone()),
            Err(RootPrepareError::Occupied)
        ));
        drop(root);
        assert!(catalog.release_retired_root(&retained));
        let replacement = model_root(&catalog, &cell);
        assert_ne!(replacement.identity, retained.identity);
        assert!(!catalog.release_retired_root(&retained));
        assert!(
            catalog
                .begin_publish_root(snapshot(&catalog), retained.clone())
                .is_none()
        );
        root_commit(
            &catalog,
            catalog.begin_publish_root(snapshot(&catalog), replacement.try_share().unwrap()),
        );
        assert_eq!(replacement.status(), RootStatus::Live);
        assert_eq!(retained.status(), RootStatus::Retired);
    });
}

#[test]
fn root_discovery_failure_cannot_complete_the_scan_or_close_strong() {
    loom::model(|| {
        let catalog = root_catalog();
        let current = snapshot(&catalog);
        FAIL_LOAD.with(|n| n.set(Some(0)));
        assert!(catalog.begin_scan_root(current, 0).is_none());
        assert_eq!(snapshot(&catalog).phase().root_cursors, [0, 0]);
        assert!(catalog.begin_advance(snapshot(&catalog), 0).is_none());
        root_commit(&catalog, catalog.begin_scan_root(snapshot(&catalog), 0));
        root_commit(&catalog, catalog.begin_advance(snapshot(&catalog), 0));
        assert_eq!(snapshot(&catalog).phase().domains[0].floor, 1);
    });
}

#[test]
fn root_barrier_never_revives_retired_membership() {
    loom::model(|| {
        let catalog = root_catalog();
        let phase = snapshot(&catalog).phase();
        assert_eq!(
            root_barrier_plan(Membership::Retired, phase, false),
            Err(TraceError::Conflict)
        );
        assert_eq!(
            root_barrier_plan(Membership::Retired, phase, true),
            Ok((Membership::Retired, phase))
        );
        assert_eq!(
            root_barrier_plan(Membership::Prepared, phase, true),
            Err(TraceError::Conflict)
        );
    });
}

fn reference_catalog() -> Catalog {
    Catalog::with_record_root_capacities_and_limit(1, 128, 128, 64, 2, 32, pool::MAX_GENERATION)
        .with_references(2, 32, 128, pool::MAX_GENERATION)
}
fn reference_root(catalog: &Catalog, cell: &Pooled<Cell>, slot: usize) -> RootToken {
    let mut creation = catalog.begin_create_root(slot, cell.clone()).unwrap();
    (0..8)
        .find_map(|_| creation.poll(catalog).unwrap())
        .expect("reference source root")
}
fn conditional_slot(catalog: &Catalog, slot: usize) -> ReferenceToken {
    let mut creation: ReferenceCreation = catalog.begin_create_reference(slot).unwrap();
    let token = (0..8)
        .find_map(|_| creation.poll(catalog).unwrap())
        .expect("conditional slot");
    let observed: &ReferenceSlot = &token;
    assert_eq!(observed.membership(), Some(ReferenceMembership::Prepared));
    token
}
fn publish_conditional(catalog: &Catalog, root: &RootToken, reference: &ReferenceToken) {
    root_commit(
        catalog,
        catalog.begin_publish_root(snapshot(catalog), root.try_share().unwrap()),
    );
    root_commit(
        catalog,
        catalog.begin_publish_reference(
            snapshot(catalog),
            reference.try_share().unwrap(),
            root.try_share().unwrap(),
            reference::ReferenceKind::Weak,
        ),
    );
}

#[test]
fn conditional_clear_and_upgrade_have_one_phase_winner() {
    loom::model(|| {
        let catalog = Arc::new(reference_catalog());
        assert!(matches!(
            catalog.begin_create_reference(2),
            Err(ReferencePrepareError::InvalidSlot)
        ));
        let cell = fixture_cell(&catalog, 8);
        let source = reference_root(&catalog, &cell, 0);
        let reference = conditional_slot(&catalog, 0);
        publish_conditional(&catalog, &source, &reference);
        let acquired = reference_root(&catalog, &cell, 1);
        let upgrade = trace_prepared(
            &catalog,
            catalog
                .begin_upgrade_reference(
                    snapshot(&catalog),
                    reference.try_share().unwrap(),
                    acquired.try_share().unwrap(),
                )
                .unwrap(),
        );
        let clear = trace_prepared(
            &catalog,
            catalog
                .begin_clear_reference(snapshot(&catalog), reference.try_share().unwrap())
                .unwrap(),
        );
        let clearing = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&clear))
        };
        let upgraded = catalog.commit(&upgrade) == Outcome::Committed;
        let cleared = clearing.join().unwrap() == Outcome::Committed;
        assert_ne!(upgraded, cleared);
        assert_eq!(acquired.status() == RootStatus::Live, upgraded);
        let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
            panic!();
        };
        assert_eq!(state.target.is_none(), cleared);
    });
}

#[test]
fn conditional_clone_cannot_copy_a_cleared_source_using_an_old_phase() {
    loom::model(|| {
        let catalog = Arc::new(reference_catalog());
        let cell = fixture_cell(&catalog, 8);
        let root = reference_root(&catalog, &cell, 0);
        let source = conditional_slot(&catalog, 0);
        let destination = conditional_slot(&catalog, 1);
        publish_conditional(&catalog, &root, &source);
        let clone = trace_prepared(
            &catalog,
            catalog
                .begin_clone_reference(
                    snapshot(&catalog),
                    source.try_share().unwrap(),
                    destination.try_share().unwrap(),
                )
                .unwrap(),
        );
        let clear = trace_prepared(
            &catalog,
            catalog
                .begin_clear_reference(snapshot(&catalog), source.try_share().unwrap())
                .unwrap(),
        );
        let clearing = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&clear))
        };
        let cloned = catalog.commit(&clone) == Outcome::Committed;
        let cleared = clearing.join().unwrap() == Outcome::Committed;
        assert_ne!(cloned, cleared);
        if cleared {
            assert_eq!(
                destination.membership(),
                Some(ReferenceMembership::Prepared)
            );
            root_commit(
                &catalog,
                catalog.begin_clone_reference(
                    snapshot(&catalog),
                    source.try_share().unwrap(),
                    destination.try_share().unwrap(),
                ),
            );
        }
        let ReferenceMembership::Live(state) = destination.membership().unwrap() else {
            panic!();
        };
        assert_eq!(state.target.is_some(), cloned);
    });
}

#[test]
fn delayed_conditional_cas_cannot_restore_a_retired_version() {
    loom::model(|| {
        let catalog = Arc::new(reference_catalog());
        let cell = fixture_cell(&catalog, 8);
        let root = reference_root(&catalog, &cell, 0);
        let token = conditional_slot(&catalog, 0);
        let retained = token.try_share().unwrap();
        root_commit(
            &catalog,
            catalog.begin_publish_root(snapshot(&catalog), root.try_share().unwrap()),
        );
        let publish = trace_prepared(
            &catalog,
            catalog
                .begin_publish_reference(
                    snapshot(&catalog),
                    token.try_share().unwrap(),
                    root.try_share().unwrap(),
                    reference::ReferenceKind::Weak,
                )
                .unwrap(),
        );
        assert!(catalog.install_and_stop(&publish));
        let (entered, entry) = loom::sync::mpsc::channel();
        let (resume, resumed) = loom::sync::mpsc::channel();
        token.pause_next_effect(move || {
            entered.send(()).unwrap();
            resumed.recv().unwrap();
        });
        let helper = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.help_record_at(&(), 1, || {}))
        };
        entry.recv().unwrap();
        assert_eq!(catalog.commit(&publish), Outcome::Committed);
        root_commit(
            &catalog,
            catalog.begin_clear_reference(snapshot(&catalog), retained.clone()),
        );
        drop(token);
        root_commit(
            &catalog,
            catalog.begin_retire_reference(snapshot(&catalog), retained.clone()),
        );
        assert!(catalog.release_retired_reference(&retained));
        let replacement = conditional_slot(&catalog, 0);
        root_commit(
            &catalog,
            catalog.begin_publish_reference(
                snapshot(&catalog),
                replacement.try_share().unwrap(),
                root.try_share().unwrap(),
                reference::ReferenceKind::Final,
            ),
        );
        resume.send(()).unwrap();
        helper.join().unwrap();
        assert_eq!(retained.membership(), Some(ReferenceMembership::Retired));
        let ReferenceMembership::Live(state) = replacement.membership().unwrap() else {
            panic!();
        };
        assert_eq!(state.kind, reference::ReferenceKind::Final);
    });
}

// The larger setup exceeds Loom's default 32 KiB coroutine stack in debug
// builds. Give only this model body an explicit stack; do not change explored
// schedules or cap permutations. The extra outer thread only joins the body.
fn notification_model(body: fn()) {
    let mut builder = loom::model::Builder::new();
    builder.max_branches = 20_000;
    builder.check(move || {
        thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(body)
            .unwrap()
            .join()
            .unwrap();
    });
}

fn managed_final() -> (
    Catalog,
    Pooled<Cell>,
    Pooled<Cell>,
    ReferenceToken,
    RootToken,
) {
    let c = Catalog::with_record_root_capacities_and_limit(
        2,
        256,
        256,
        128,
        4,
        64,
        pool::MAX_GENERATION,
    )
    .with_references(1, 32, 128, pool::MAX_GENERATION);
    let target = fixture_cell(&c, 8);
    let owner = fixture_cell(&c, 16);
    let target_root = reference_root(&c, &target, 0);
    let owner_root = reference_root(&c, &owner, 1);
    root_commit(
        &c,
        c.begin_publish_root(snapshot(&c), target_root.try_share().unwrap()),
    );
    root_commit(
        &c,
        c.begin_publish_root(snapshot(&c), owner_root.try_share().unwrap()),
    );
    let reference = conditional_slot(&c, 0);
    root_commit(
        &c,
        c.begin_publish_reference(
            snapshot(&c),
            reference.try_share().unwrap(),
            target_root.try_share().unwrap(),
            reference::ReferenceKind::Final,
        ),
    );
    root_commit(
        &c,
        c.begin_bind_reference_owner(
            snapshot(&c),
            reference.try_share().unwrap(),
            owner_root.try_share().unwrap(),
        ),
    );
    for token in [target_root, owner_root] {
        let root = token.try_share().unwrap();
        root_commit(&c, c.begin_remove_root(snapshot(&c), root.clone()));
        drop(token);
        assert!(c.release_retired_root(&root));
    }
    for domain in 0..2 {
        while snapshot(&c).phase().domains[domain].floor < RETIREMENT_FLOOR {
            root_drain(&c, &target);
            root_drain(&c, &owner);
            if snapshot(&c).phase().domains[domain].floor == 0 {
                for _ in 0..c.root_capacity() {
                    root_commit(&c, c.begin_scan_root(snapshot(&c), domain));
                }
            } else {
                root_commit(&c, c.begin_scan_reference(snapshot(&c), domain));
            }
            root_commit(&c, c.begin_advance(snapshot(&c), domain));
        }
        root_commit(&c, c.begin_finish(snapshot(&c), domain));
    }
    for domain in 0..2 {
        root_commit(&c, c.begin_start(snapshot(&c), domain, DomainMode::Full));
        for _ in 0..c.root_capacity() {
            root_commit(&c, c.begin_scan_root(snapshot(&c), domain));
        }
    }
    root_commit(
        &c,
        c.begin_shade(snapshot(&c), owner.clone(), 0, TraceRetention::Final),
    );
    root_drain(&c, &owner);
    root_commit(&c, c.begin_advance(snapshot(&c), 0));
    root_commit(&c, c.begin_scan_reference(snapshot(&c), 0));
    root_commit(&c, c.begin_advance(snapshot(&c), 0));
    let notification = reference_root(&c, &owner, 2);
    (c, target, owner, reference, notification)
}

#[test]
fn notification_effects_and_both_root_transfers_are_helpable_after_each_prefix() {
    notification_model(|| {
        let (catalog, target, owner, reference, notification) = managed_final();
        let catalog = Arc::new(catalog);
        let attempt = trace_prepared(
            &catalog,
            catalog
                .begin_scan_reference_with_root(
                    snapshot(&catalog),
                    0,
                    notification.try_share().unwrap(),
                )
                .unwrap(),
        );
        assert!(catalog.install_and_stop(&attempt));
        let (entered, entry) = loom::sync::mpsc::channel();
        let (resume, resumed) = loom::sync::mpsc::channel();
        let helper = {
            let catalog = catalog.clone();
            thread::spawn(move || {
                catalog.help_at(1, || {
                    entered.send(()).unwrap();
                    resumed.recv().unwrap();
                })
            })
        };
        entry.recv().unwrap();
        assert_eq!(
            trace_marks(&target)[0].retention,
            Some(TraceRetention::Final)
        );
        assert!(!trace_marks(&owner)[1].marked(2));
        assert_eq!(notification.status(), RootStatus::Prepared);
        assert_eq!(catalog.commit(&attempt), Outcome::Committed);
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [1, 1]
        );
        let held = notification.try_share().unwrap();
        drop(notification);
        assert!(held.is_internal());
        assert!(
            catalog
                .begin_requested_root_removal(snapshot(&catalog), held.clone())
                .is_none()
        );
        let caller = reference_root(&catalog, &owner, 3);
        root_commit(
            &catalog,
            catalog.begin_take_reference(
                snapshot(&catalog),
                reference.try_share().unwrap(),
                caller.try_share().unwrap(),
            ),
        );
        assert_eq!(caller.status(), RootStatus::Live);
        assert_eq!(held.status(), RootStatus::Retired);
        assert!(catalog.release_retired_root(&held));
        resume.send(()).unwrap();
        helper.join().unwrap();
        assert_eq!(held.status(), RootStatus::Retired);
        let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
            panic!()
        };
        assert!(state.queued_root.is_none());
    });
}

#[test]
fn notification_proposed_ownership_failures_leave_all_effects_cancellable() {
    notification_model(|| {
        for failure in 1..=3 {
            let (catalog, target, owner, reference, notification) = managed_final();
            let before = (
                target.membership(),
                owner.membership(),
                reference.membership(),
            );
            let attempt = trace_prepared(
                &catalog,
                catalog
                    .begin_scan_reference_with_root(
                        snapshot(&catalog),
                        0,
                        notification.try_share().unwrap(),
                    )
                    .unwrap(),
            );
            FAIL_SHARE.with(|counter| counter.set(Some(failure)));
            assert_eq!(catalog.commit(&attempt), Outcome::Pending);
            assert!(attempt.cancel());
            assert_eq!(catalog.commit(&attempt), Outcome::Cancelled);
            assert_eq!(
                before,
                (
                    target.membership(),
                    owner.membership(),
                    reference.membership()
                )
            );
            assert_eq!(notification.status(), RootStatus::Prepared);
            assert_eq!(snapshot(&catalog).phase().reference_cursors[0], 0);
        }
    });
}

#[test]
fn notification_delivery_cannot_reactivate_a_root_after_a_late_actual_cas() {
    notification_model(|| {
        for first_wins in [false, true] {
            let (catalog, _, owner, reference, notification) = managed_final();
            let catalog = Arc::new(catalog);
            root_commit(
                &catalog,
                catalog.begin_scan_reference_with_root(
                    snapshot(&catalog),
                    0,
                    notification.try_share().unwrap(),
                ),
            );
            let first = reference_root(&catalog, &owner, 0);
            let second = reference_root(&catalog, &owner, 1);
            let first_take = trace_prepared(
                &catalog,
                catalog
                    .begin_take_reference(
                        snapshot(&catalog),
                        reference.try_share().unwrap(),
                        first.try_share().unwrap(),
                    )
                    .unwrap(),
            );
            let second_take = trace_prepared(
                &catalog,
                catalog
                    .begin_take_reference(
                        snapshot(&catalog),
                        reference.try_share().unwrap(),
                        second.try_share().unwrap(),
                    )
                    .unwrap(),
            );
            let (winner, loser, token, rejected) = if first_wins {
                (first_take, second_take, first, second)
            } else {
                (second_take, first_take, second, first)
            };
            assert!(catalog.install_and_stop(&winner));
            let (entered, entry) = loom::sync::mpsc::channel();
            let (resume, resumed) = loom::sync::mpsc::channel();
            token.pause_next_effect(move || {
                entered.send(()).unwrap();
                resumed.recv().unwrap();
            });
            let helper = {
                let catalog = catalog.clone();
                thread::spawn(move || catalog.help_at(1, || {}))
            };
            entry.recv().unwrap();
            // The helper already passed its completion check and is stopped
            // immediately before the actual caller-root status CAS.
            assert_eq!(catalog.commit(&loser), Outcome::Retry);
            assert_eq!(catalog.commit(&winner), Outcome::Committed);
            assert_eq!(token.status(), RootStatus::Live);
            assert_eq!(rejected.status(), RootStatus::Prepared);
            assert_eq!(notification.status(), RootStatus::Retired);
            let old = token.try_share().unwrap();
            root_commit(
                &catalog,
                catalog.begin_remove_root(snapshot(&catalog), old.clone()),
            );
            drop(token);
            assert!(catalog.release_retired_root(&old));
            let replacement = reference_root(&catalog, &owner, old.slot());
            assert_ne!(replacement.identity, old.identity);
            resume.send(()).unwrap();
            helper.join().unwrap();
            assert_eq!(old.status(), RootStatus::Retired);
            assert_eq!(replacement.status(), RootStatus::Prepared);
            let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
                panic!()
            };
            assert!(state.queued_root.is_none());
        }
    });
}

fn construction_model_fixture() -> (Catalog, Pooled<Cell>, ReferenceToken, RootToken) {
    let catalog = Catalog::with_record_root_capacities_and_limit(
        2,
        128,
        128,
        64,
        4,
        32,
        pool::MAX_GENERATION,
    )
    .with_references(2, 32, 128, pool::MAX_GENERATION);
    let target = fixture_cell(&catalog, 8);
    let source = reference_root(&catalog, &target, 0);
    let reference = conditional_slot(&catalog, 0);
    publish_conditional(&catalog, &source, &reference);
    let held = reference_root(&catalog, &target, 1);
    root_commit(
        &catalog,
        catalog.begin_reserve_reference_owner(
            snapshot(&catalog),
            reference.try_share().unwrap(),
            held.try_share().unwrap(),
        ),
    );
    let source_root = source.try_share().unwrap();
    root_commit(
        &catalog,
        catalog.begin_remove_root(snapshot(&catalog), source_root.clone()),
    );
    drop(source);
    assert!(catalog.release_retired_root(&source_root));
    (catalog, target, reference, held)
}

#[test]
fn construction_cancellation_and_owner_publication_cannot_split_their_root_effects() {
    notification_model(|| {
        for publish_first in [false, true] {
            let (catalog, _, reference, reservation) = construction_model_fixture();
            let catalog = Arc::new(catalog);
            let owner = fixture_cell(&catalog, 16);
            let owner_root = reference_root(&catalog, &owner, 2);
            let held = reservation.try_share().unwrap();
            let publication = trace_prepared(
                &catalog,
                catalog
                    .begin_publish_reference_owner(
                        snapshot(&catalog),
                        reference.try_share().unwrap(),
                        owner_root.try_share().unwrap(),
                    )
                    .unwrap(),
            );
            drop(reservation);
            let cancellation = trace_prepared(
                &catalog,
                catalog
                    .begin_cancel_reference_owner(
                        snapshot(&catalog),
                        reference.try_share().unwrap(),
                    )
                    .unwrap(),
            );
            let (winner, loser) = if publish_first {
                (publication, cancellation)
            } else {
                (cancellation, publication)
            };
            assert!(catalog.install_and_stop(&winner));
            let (entered, entry) = loom::sync::mpsc::channel();
            let (resume, resumed) = loom::sync::mpsc::channel();
            let helper = {
                let catalog = catalog.clone();
                thread::spawn(move || {
                    catalog.help_at(1, || {
                        entered.send(()).unwrap();
                        resumed.recv().unwrap();
                    })
                })
            };
            entry.recv().unwrap();
            assert_eq!(catalog.commit(&loser), Outcome::Retry);
            assert_eq!(catalog.commit(&winner), Outcome::Committed);
            let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
                panic!()
            };
            assert!(state.construction_root.is_none());
            assert_eq!(state.owner.is_some(), publish_first);
            assert_eq!(owner_root.status() == RootStatus::Live, publish_first);
            assert_eq!(
                matches!(owner.membership(), Some(Membership::Tracing(_))),
                publish_first
            );
            assert_eq!(held.status(), RootStatus::Retired);
            assert!(catalog.release_retired_root(&held));
            resume.send(()).unwrap();
            helper.join().unwrap();
            assert_eq!(held.status(), RootStatus::Retired);
        }
    });
}

#[test]
fn prebound_reference_cannot_finish_before_exact_owner_publication_is_certified() {
    notification_model(|| {
        let (catalog, _, reference, reservation) = construction_model_fixture();
        let catalog = Arc::new(catalog);
        let owner = fixture_cell(&catalog, 16);
        root_commit(
            &catalog,
            catalog.begin_prepare_reference_owner(
                snapshot(&catalog),
                reference.try_share().unwrap(),
                owner.clone(),
            ),
        );
        assert!(
            catalog
                .begin_finish_reference_owner(snapshot(&catalog), reference.try_share().unwrap())
                .is_none()
        );
        let owner_root = reference_root(&catalog, &owner, 2);
        let publication = trace_prepared(
            &catalog,
            catalog
                .begin_publish_root(snapshot(&catalog), owner_root.try_share().unwrap())
                .unwrap(),
        );
        assert!(catalog.install_and_stop(&publication));
        let (entered, entry) = loom::sync::mpsc::channel();
        let (resume, resumed) = loom::sync::mpsc::channel();
        let helper = {
            let catalog = catalog.clone();
            thread::spawn(move || {
                catalog.help_at(1, || {
                    entered.send(()).unwrap();
                    resumed.recv().unwrap();
                })
            })
        };
        entry.recv().unwrap();
        assert!(matches!(owner.membership(), Some(Membership::Tracing(_))));
        assert_eq!(owner_root.status(), RootStatus::Prepared);
        // Snapshot acquisition finishes the publication first; observing the
        // first object CAS alone never authorizes the construction-root release.
        root_commit(
            &catalog,
            catalog
                .begin_finish_reference_owner(snapshot(&catalog), reference.try_share().unwrap()),
        );
        assert_eq!(owner_root.status(), RootStatus::Live);
        assert_eq!(reservation.status(), RootStatus::Retired);
        let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
            panic!()
        };
        assert!(state.construction_root.is_none());
        assert_eq!(
            state.owner,
            Some(reference::ObjectKey {
                identity: owner.identity,
                slot: owner.slot()
            })
        );
        resume.send(()).unwrap();
        helper.join().unwrap();
    });
}

#[test]
fn late_reserved_announcement_uses_the_old_reference_object_after_registry_reuse() {
    notification_model(|| {
        let catalog = Arc::new(reference_catalog());
        let target = fixture_cell(&catalog, 8);
        let source = reference_root(&catalog, &target, 0);
        let old = conditional_slot(&catalog, 0);
        publish_conditional(&catalog, &source, &old);
        let old_slot = old.try_share().unwrap();
        let mut creation = catalog
            .begin_create_reference_root(
                old.try_share().unwrap(),
                ReferenceRootKind::Construction,
                target.clone(),
            )
            .unwrap();
        let (entered, entry) = loom::sync::mpsc::channel();
        let (resume, resumed) = loom::sync::mpsc::channel();
        let helper = {
            let catalog = catalog.clone();
            thread::spawn(move || {
                assert!(creation.poll(&catalog).unwrap().is_none());
                entered.send(()).unwrap();
                resumed.recv().unwrap();
                creation.poll(&catalog).unwrap().unwrap()
            })
        };
        entry.recv().unwrap();
        drop(old);
        root_commit(
            &catalog,
            catalog.begin_retire_reference(snapshot(&catalog), old_slot.clone()),
        );
        assert!(catalog.release_retired_reference(&old_slot));
        let replacement = conditional_slot(&catalog, 0);
        root_commit(
            &catalog,
            catalog.begin_publish_reference(
                snapshot(&catalog),
                replacement.try_share().unwrap(),
                source.try_share().unwrap(),
                reference::ReferenceKind::Weak,
            ),
        );
        resume.send(()).unwrap();
        let late = helper.join().unwrap();
        assert!(catalog.discover_root(late.slot()).unwrap().is_none());
        let mut current = catalog
            .begin_create_reference_root(
                replacement.try_share().unwrap(),
                ReferenceRootKind::Construction,
                target.clone(),
            )
            .unwrap();
        assert!(current.poll(&catalog).unwrap().is_none());
        let current = current.poll(&catalog).unwrap().unwrap();
        assert_eq!(late.slot(), current.slot());
        assert_ne!(late.identity, current.identity);
        assert_eq!(
            catalog
                .discover_root(current.slot())
                .unwrap()
                .unwrap()
                .identity,
            current.identity
        );
        assert!(
            catalog
                .begin_reserve_reference_owner(
                    snapshot(&catalog),
                    replacement.try_share().unwrap(),
                    late.try_share().unwrap()
                )
                .is_none()
        );
        root_commit(
            &catalog,
            catalog.begin_reserve_reference_owner(
                snapshot(&catalog),
                replacement.try_share().unwrap(),
                current.try_share().unwrap(),
            ),
        );
        assert_eq!(current.status(), RootStatus::Live);
        assert_eq!(late.status(), RootStatus::Prepared);
    });
}

#[test]
fn reserved_root_retirement_reuses_only_its_link_while_the_old_token_remains_owned() {
    notification_model(|| {
        let catalog = reference_catalog();
        let target = fixture_cell(&catalog, 8);
        let source = reference_root(&catalog, &target, 0);
        let reference = conditional_slot(&catalog, 0);
        publish_conditional(&catalog, &source, &reference);
        let mut creation = catalog
            .begin_create_reference_root(
                reference.try_share().unwrap(),
                ReferenceRootKind::Notification,
                target.clone(),
            )
            .unwrap();
        assert!(creation.poll(&catalog).unwrap().is_none());
        let token = creation.poll(&catalog).unwrap().unwrap();
        let old = token.try_share().unwrap();
        root_commit(
            &catalog,
            catalog.begin_cancel_root(snapshot(&catalog), old.clone()),
        );
        assert!(!token.removal_requested());
        assert!(catalog.release_retired_root(&old));
        let mut replacement = catalog
            .begin_create_reference_root(
                reference.try_share().unwrap(),
                ReferenceRootKind::Notification,
                target.clone(),
            )
            .unwrap();
        assert!(replacement.poll(&catalog).unwrap().is_none());
        let replacement = replacement.poll(&catalog).unwrap().unwrap();
        assert_ne!(replacement.identity, token.identity);
        assert_eq!(old.status(), RootStatus::Retired);
        assert!(!catalog.release_retired_root(&old));
        assert!(
            catalog
                .begin_activate_root(snapshot(&catalog), old.clone())
                .is_none()
        );
        assert_eq!(
            catalog
                .discover_root(replacement.slot())
                .unwrap()
                .unwrap()
                .identity,
            replacement.identity
        );
    });
}

#[test]
fn deferred_notification_plans_retain_work_but_never_refresh_the_expected_controller() {
    notification_model(|| {
        for supersede in [false, true] {
            let (catalog, target, owner, reference, _unused) = managed_final();
            let ReferenceScan::NeedsRoot(plan) =
                catalog.begin_reference_scan(snapshot(&catalog), 0).unwrap()
            else {
                panic!()
            };
            let typed: &ReferenceNotification = &plan;
            assert_eq!(
                typed.owner(),
                reference::ObjectKey {
                    identity: owner.identity,
                    slot: owner.slot()
                }
            );
            let mut creation = plan.begin_root_creation(&catalog).unwrap();
            assert!(creation.poll(&catalog).unwrap().is_none());
            let root = creation.poll(&catalog).unwrap().unwrap();
            let proposed_root = root.try_share().unwrap();
            FAIL_SHARE.with(|counter| counter.set(Some(0)));
            assert!(plan.try_finish(&catalog, proposed_root).is_none());
            assert_eq!(snapshot(&catalog).phase().reference_cursors[0], 0);
            if supersede {
                root_commit(
                    &catalog,
                    catalog
                        .begin_clear_reference(snapshot(&catalog), reference.try_share().unwrap()),
                );
            }
            let attempt = trace_prepared(
                &catalog,
                plan.try_finish(&catalog, root.try_share().unwrap())
                    .unwrap(),
            );
            assert_eq!(
                catalog.commit(&attempt),
                if supersede {
                    Outcome::Retry
                } else {
                    Outcome::Committed
                }
            );
            assert_eq!(root.status() == RootStatus::Live, !supersede);
            assert_eq!(trace_marks(&target)[0].marked(2), !supersede);
            let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
                panic!()
            };
            assert_eq!(state.queued_root.is_some(), !supersede);
        }
    });
}

#[test]
fn allocation_abandonment_racing_publication_preserves_the_winning_decision() {
    loom::model(|| {
        let catalog = Arc::new(root_catalog());
        let cell = fixture_cell(&catalog, 8);
        let token = model_root(&catalog, &cell);
        let root = token.try_share().unwrap();
        let attempt = Arc::new(trace_prepared(
            &catalog,
            catalog
                .begin_publish_root(snapshot(&catalog), root.clone())
                .unwrap(),
        ));
        assert!(catalog.install_and_stop(&attempt));
        let canceller = {
            let attempt = attempt.clone();
            let cell = cell.clone();
            thread::spawn(move || {
                let won = attempt.cancel();
                cell.request_abandon();
                drop(token);
                won
            })
        };
        let outcome = catalog.commit(&attempt);
        let cancelled = canceller.join().unwrap();
        assert_eq!(outcome == Outcome::Cancelled, cancelled);
        assert!(cell.abandon_requested());
        assert!(root.removal_requested());
        root_commit(
            &catalog,
            catalog.begin_requested_root_removal(snapshot(&catalog), root.clone()),
        );
        assert_eq!(root.status(), RootStatus::Retired);
        if cancelled {
            assert_eq!(cell.membership(), Some(Membership::Prepared));
            root_commit(
                &catalog,
                catalog.begin_abandon(snapshot(&catalog), cell.clone()),
            );
            assert_eq!(cell.membership(), Some(Membership::Retired));
            assert!(catalog.release_retired(&cell));
        } else {
            assert!(matches!(cell.membership(), Some(Membership::Tracing(_))));
            assert!(
                catalog
                    .begin_abandon(snapshot(&catalog), cell.clone())
                    .is_none()
            );
        }
        assert!(catalog.release_retired_root(&root));
    });
}

#[test]
fn late_allocation_abandonment_changes_only_its_retained_cell_after_slot_reuse() {
    loom::model(|| {
        let catalog = Arc::new(Catalog::new());
        let old = fixture_cell(&catalog, 8);
        root_commit(
            &catalog,
            catalog.begin_abandon(snapshot(&catalog), old.clone()),
        );
        assert!(catalog.release_retired(&old));
        let old_owner = {
            let old = old.clone();
            thread::spawn(move || old.request_abandon())
        };
        let replacement = fixture_cell(&catalog, 16);
        old_owner.join().unwrap();
        assert!(old.abandon_requested());
        assert!(!replacement.abandon_requested());
        assert_ne!(old.identity, replacement.identity);
        assert_eq!(replacement.membership(), Some(Membership::Prepared));
        assert!(!catalog.release_retired(&old));
        assert_eq!(
            catalog
                .discover(replacement.slot())
                .unwrap()
                .unwrap()
                .identity,
            replacement.identity
        );
    });
}

#[test]
fn a_finish_epoch_request_cannot_rebase_after_another_owner_restarts_the_domain() {
    loom::model(|| {
        let catalog = Catalog::new();
        for _ in 0..RETIREMENT_FLOOR {
            root_commit(&catalog, catalog.begin_advance(snapshot(&catalog), 0));
        }
        let epoch = snapshot(&catalog).phase().domains[0].epoch;
        let stale = trace_prepared(
            &catalog,
            catalog
                .begin_finish_epoch(snapshot(&catalog), 0, epoch)
                .unwrap(),
        );
        root_commit(&catalog, catalog.begin_finish(snapshot(&catalog), 0));
        root_commit(
            &catalog,
            catalog.begin_start(snapshot(&catalog), 0, DomainMode::Full),
        );
        for _ in 0..RETIREMENT_FLOOR {
            root_commit(&catalog, catalog.begin_advance(snapshot(&catalog), 0));
        }
        assert_eq!(catalog.commit(&stale), Outcome::Retry);
        assert!(
            catalog
                .begin_finish_epoch(snapshot(&catalog), 0, epoch)
                .is_none()
        );
        let phase = snapshot(&catalog).phase();
        assert_eq!(phase.domains[0].epoch, epoch + 1);
        assert_eq!(phase.domains[0].mode, DomainMode::Full);
        root_commit(
            &catalog,
            catalog.begin_finish_epoch(snapshot(&catalog), 0, epoch + 1),
        );
    });
}

#[test]
fn a_racing_old_epoch_finish_never_closes_the_restarted_full_epoch() {
    loom::model(|| {
        // Model the deciding boundary directly: an old queued command races the
        // last advance of the new epoch. The separate stale-preparation fixture
        // executes Finish/Start/all advances and checks expected-head rejection.
        let old_epoch = 7;
        let catalog = Arc::new(Catalog::with_phase(Phase {
            root_cursors: [0; 2],
            reference_cursors: [0; 2],
            revision: 20,
            domains: [
                Domain {
                    mode: DomainMode::Full,
                    epoch: old_epoch + 1,
                    floor: RETIREMENT_FLOOR - 1,
                    pending: 0,
                },
                Domain {
                    mode: DomainMode::Inactive,
                    epoch: 1,
                    floor: RETIREMENT_FLOOR,
                    pending: 0,
                },
            ],
        }));
        let old_finisher = {
            let catalog = catalog.clone();
            thread::spawn(move || {
                if let Some(preparation) =
                    catalog.begin_finish_epoch(snapshot(&catalog), 0, old_epoch)
                {
                    let attempt = trace_prepared(&catalog, preparation);
                    assert!(matches!(
                        catalog.commit(&attempt),
                        Outcome::Committed | Outcome::Retry
                    ));
                }
            })
        };
        root_commit(&catalog, catalog.begin_advance(snapshot(&catalog), 0));
        old_finisher.join().unwrap();
        let current = snapshot(&catalog).phase();
        assert_eq!(current.domains[0].epoch, old_epoch + 1);
        assert_eq!(current.domains[0].floor, RETIREMENT_FLOOR);
        assert_eq!(current.domains[0].mode, DomainMode::Full);
    });
}

#[test]
fn rotating_cell_announcements_preserve_exact_identity_under_collision() {
    loom::model(|| {
        let c = Arc::new(Catalog::with_capacity(2));
        let mut a = c.begin_create(0, 8).unwrap();
        let mut b = c.begin_create(0, 16).unwrap();
        for creation in [&mut a, &mut b] {
            assert!(creation.poll_rotating(&c).unwrap().is_none());
            assert!(creation.poll_rotating(&c).unwrap().is_none());
        }
        let other = {
            let c = c.clone();
            thread::spawn(move || {
                let result = b.poll_rotating(&c).unwrap();
                (b, result)
            })
        };
        let first = a.poll_rotating(&c).unwrap();
        let (mut b, second) = other.join().unwrap();
        assert!(first.is_some() || second.is_some());
        let first = first
            .or_else(|| a.poll_rotating(&c).unwrap())
            .expect("next free slot");
        let second = second
            .or_else(|| b.poll_rotating(&c).unwrap())
            .expect("next free slot");
        assert_ne!(first.identity, second.identity);
        assert_ne!(first.slot(), second.slot());
        assert_eq!((first.address, second.address), (8, 16));
        assert!(Pooled::ptr_eq(
            &first,
            &c.discover(first.slot()).unwrap().unwrap()
        ));
        assert!(Pooled::ptr_eq(
            &second,
            &c.discover(second.slot()).unwrap().unwrap()
        ));
        assert!(matches!(a.poll_rotating(&c), Err(CreationError::Finished)));
        assert!(matches!(b.poll_rotating(&c), Err(CreationError::Finished)));
    });
}
#[test]
fn rotating_root_announcements_preserve_target_and_location_under_collision() {
    loom::model(|| {
        let c = Arc::new(reference_catalog());
        let cell = fixture_cell(&c, 8);
        let mut a = c.begin_create_root(0, cell.clone()).unwrap();
        let mut b = c.begin_create_root(0, cell.clone()).unwrap();
        assert!(a.poll_rotating(&c).unwrap().is_none());
        assert!(b.poll_rotating(&c).unwrap().is_none());
        let other = {
            let c = c.clone();
            thread::spawn(move || {
                let result = b.poll_rotating(&c).unwrap();
                (b, result)
            })
        };
        let first = a.poll_rotating(&c).unwrap();
        let (mut b, second) = other.join().unwrap();
        assert!(first.is_some() || second.is_some());
        let first = first.or_else(|| a.poll_rotating(&c).unwrap()).unwrap();
        let second = second.or_else(|| b.poll_rotating(&c).unwrap()).unwrap();
        assert_ne!(first.identity, second.identity);
        assert_ne!(first.slot(), second.slot());
        assert_eq!(first.target_identity(), cell.identity);
        assert_eq!(second.target_identity(), cell.identity);
        assert_eq!(
            c.discover_root(first.slot()).unwrap().unwrap().identity,
            first.identity
        );
        assert_eq!(
            c.discover_root(second.slot()).unwrap().unwrap().identity,
            second.identity
        );
        assert!(matches!(a.poll_rotating(&c), Err(CreationError::Finished)));
        assert!(matches!(b.poll_rotating(&c), Err(CreationError::Finished)));
    });
}
#[test]
fn rotating_root_rejection_keeps_reserved_reference_role_usable() {
    loom::model(|| {
        let c = reference_catalog();
        let cell = fixture_cell(&c, 8);
        let reference = conditional_slot(&c, 0);
        for kind in [
            ReferenceRootKind::Construction,
            ReferenceRootKind::Notification,
        ] {
            let mut creation = c
                .begin_create_reference_root(reference.try_share().unwrap(), kind, cell.clone())
                .unwrap();
            assert!(matches!(
                creation.poll_rotating(&c),
                Err(CreationError::FixedLocation)
            ));
            assert!(creation.poll(&c).unwrap().is_none());
            assert!(matches!(
                creation.poll_rotating(&c),
                Err(CreationError::FixedLocation)
            ));
            let root = creation.poll(&c).unwrap().unwrap();
            assert!(root.is_internal());
            assert_eq!(
                c.discover_root(root.slot()).unwrap().unwrap().identity,
                root.identity
            );
        }
    });
}

fn batch_root_catalog(capacity: usize) -> Catalog {
    Catalog::with_record_root_capacities_and_limit(
        1,
        64,
        64,
        32,
        capacity,
        16,
        pool::MAX_GENERATION,
    )
}
fn batch_model_root(catalog: &Catalog, cell: &Pooled<Cell>) -> RootToken {
    let mut creation = catalog.begin_create_root(1, cell.clone()).unwrap();
    for _ in 0..8 {
        if let Some(root) = creation.poll(catalog).unwrap() {
            return root;
        }
    }
    panic!("batch model root did not announce");
}
fn batch_root_new_epoch(catalog: &Catalog, cell: &Pooled<Cell>) {
    root_drain(catalog, cell);
    root_commit(catalog, catalog.begin_scan_root_batch(snapshot(catalog), 0));
    for _ in 0..RETIREMENT_FLOOR {
        root_commit(catalog, catalog.begin_advance(snapshot(catalog), 0));
    }
    root_commit(catalog, catalog.begin_finish(snapshot(catalog), 0));
    root_commit(
        catalog,
        catalog.begin_start(snapshot(catalog), 0, DomainMode::Full),
    );
}

#[test]
fn scan_batch_racing_publication_preserves_work_on_either_side_of_cursor_commit() {
    loom::model(|| {
        let catalog = Arc::new(batch_root_catalog(2));
        let cell = fixture_cell(&catalog, 8);
        let root = batch_model_root(&catalog, &cell);
        let publish = trace_prepared(
            &catalog,
            catalog
                .begin_publish_root(snapshot(&catalog), root.try_share().unwrap())
                .unwrap(),
        );
        let scan = trace_prepared(
            &catalog,
            catalog
                .begin_scan_root_batch(snapshot(&catalog), 0)
                .unwrap(),
        );
        let publisher = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&publish))
        };
        let scanned = catalog.commit(&scan);
        let published = publisher.join().unwrap();
        assert!(matches!(scanned, Outcome::Committed | Outcome::Retry));
        assert!(matches!(published, Outcome::Committed | Outcome::Retry));
        if published == Outcome::Retry {
            root_commit(
                &catalog,
                catalog.begin_publish_root(snapshot(&catalog), root.try_share().unwrap()),
            );
        }
        if scanned == Outcome::Retry {
            root_commit(
                &catalog,
                catalog.begin_scan_root_batch(snapshot(&catalog), 0),
            );
        }
        assert_eq!(snapshot(&catalog).phase().root_cursors[0], 2);
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [1, 1]
        );
        assert_eq!(root.status(), RootStatus::Live);
        assert!(catalog.begin_advance(snapshot(&catalog), 0).is_none());
    });
}

#[test]
fn scan_batch_root_drop_cannot_lose_new_epoch_deletion_work() {
    loom::model(|| {
        let catalog = batch_root_catalog(3);
        let cell = fixture_cell(&catalog, 8);
        let root = batch_model_root(&catalog, &cell);
        root_commit(
            &catalog,
            catalog.begin_publish_root(snapshot(&catalog), root.try_share().unwrap()),
        );
        batch_root_new_epoch(&catalog, &cell);
        let retained = root.try_share().unwrap();
        let closer = thread::spawn(move || drop(root));
        root_commit(
            &catalog,
            catalog.begin_scan_root_batch(snapshot(&catalog), 0),
        );
        closer.join().unwrap();
        assert_eq!(snapshot(&catalog).phase().root_cursors[0], 2);
        assert_eq!(
            snapshot(&catalog).phase().domains[0].pending,
            1,
            "batch scan lost deletion work"
        );
        assert!(catalog.begin_advance(snapshot(&catalog), 0).is_none());
        if retained.status() == RootStatus::Live {
            root_commit(
                &catalog,
                catalog.begin_requested_root_removal(snapshot(&catalog), retained.clone()),
            );
        }
        assert_eq!(retained.status(), RootStatus::Retired);
        assert_eq!(snapshot(&catalog).phase().domains[0].pending, 1);
    });
}

#[test]
fn scan_batch_cursor_cannot_pass_stopped_shading_effect() {
    loom::model(|| {
        let catalog = Arc::new(batch_root_catalog(3));
        let cell = fixture_cell(&catalog, 8);
        let root = batch_model_root(&catalog, &cell);
        root_commit(
            &catalog,
            catalog.begin_publish_root(snapshot(&catalog), root.try_share().unwrap()),
        );
        batch_root_new_epoch(&catalog, &cell);
        let scan = trace_prepared(
            &catalog,
            catalog
                .begin_scan_root_batch(snapshot(&catalog), 0)
                .unwrap(),
        );
        assert!(catalog.install_and_stop(&scan));
        let (entered, entry) = loom::sync::mpsc::channel();
        let (resume, resumed) = loom::sync::mpsc::channel();
        let helper = {
            let catalog = catalog.clone();
            thread::spawn(move || {
                catalog.help_record_at(&(), 1, || {
                    entered.send(()).unwrap();
                    resumed.recv().unwrap();
                })
            })
        };
        entry.recv().unwrap();
        // A second helper must finish the original membership effect before
        // exposing the batched cursor, even while the first remains stopped.
        let phase = snapshot(&catalog).phase();
        assert_eq!(phase.root_cursors[0], 2);
        assert_eq!(phase.domains[0].pending, 1);
        let Some(Membership::Tracing(marks)) = cell.membership() else {
            panic!()
        };
        assert_eq!(marks[0].epoch, phase.domains[0].epoch);
        assert_ne!(marks[0].ticket, 0);
        resume.send(()).unwrap();
        helper.join().unwrap();
        assert_eq!(catalog.commit(&scan), Outcome::Committed);
    });
}

#[test]
fn scan_batch_failure_and_cancellation_cannot_skip_unobserved_slots() {
    loom::model(|| {
        let catalog = batch_root_catalog(10);
        for failure in [0, 3, 7] {
            let current = snapshot(&catalog);
            FAIL_LOAD.with(|n| n.set(Some(failure)));
            assert!(catalog.begin_scan_root_batch(current, 0).is_none());
            assert_eq!(snapshot(&catalog).phase().root_cursors[0], 0);
        }
        let scan = trace_prepared(
            &catalog,
            catalog
                .begin_scan_root_batch(snapshot(&catalog), 0)
                .unwrap(),
        );
        assert!(scan.cancel());
        assert_eq!(catalog.commit(&scan), Outcome::Cancelled);
        assert_eq!(snapshot(&catalog).phase().root_cursors[0], 0);
        root_commit(
            &catalog,
            catalog.begin_scan_root_batch(snapshot(&catalog), 0),
        );
        assert_eq!(snapshot(&catalog).phase().root_cursors[0], 8);
        root_commit(
            &catalog,
            catalog.begin_scan_root_batch(snapshot(&catalog), 0),
        );
        assert_eq!(snapshot(&catalog).phase().root_cursors[0], 10);
    });
}

fn edge_catalog() -> Catalog {
    Catalog::with_record_root_capacities_and_limit(1, 128, 128, 64, 2, 32, pool::MAX_GENERATION)
        .with_edge_capacities(1, 32, 64, pool::MAX_GENERATION)
}
fn model_edge(catalog: &Catalog) -> EdgeToken {
    assert!(matches!(
        catalog.begin_create_edge(1),
        Err(EdgePrepareError::InvalidSlot)
    ));
    let mut creation: EdgeCreation = catalog.begin_create_edge(0).unwrap();
    let edge = (0..8)
        .find_map(|_| creation.poll(catalog).unwrap())
        .expect("edge slot");
    let observed: &EdgeSlot = &edge;
    assert_eq!(observed.membership(), Some(EdgeMembership::Prepared));
    edge
}

#[test]
fn edge_replacement_and_child_root_acquisition_have_one_snapshot_winner() {
    loom::model(|| {
        let catalog = Arc::new(edge_catalog());
        let cell = fixture_cell(&catalog, 8);
        let source = reference_root(&catalog, &cell, 0);
        root_commit(
            &catalog,
            catalog.begin_publish_root(snapshot(&catalog), source.try_share().unwrap()),
        );
        let edge = model_edge(&catalog);
        root_commit(
            &catalog,
            catalog.begin_publish_edge(
                snapshot(&catalog),
                edge.try_share().unwrap(),
                source.try_share().unwrap(),
                Some(source.try_share().unwrap()),
            ),
        );
        let state = EdgeState {
            source: reference::ObjectKey {
                identity: cell.identity,
                slot: cell.slot(),
            },
            target: Some(reference::ObjectKey {
                identity: cell.identity,
                slot: cell.slot(),
            }),
        };
        assert_eq!(edge.membership(), Some(EdgeMembership::Live(state)));
        let child = reference_root(&catalog, &cell, 1);
        let read = trace_prepared(
            &catalog,
            catalog
                .begin_root_edge_target(
                    snapshot(&catalog),
                    edge.try_share().unwrap(),
                    source.try_share().unwrap(),
                    child.try_share().unwrap(),
                )
                .unwrap(),
        );
        let replace = trace_prepared(
            &catalog,
            catalog
                .begin_replace_edge(
                    snapshot(&catalog),
                    edge.try_share().unwrap(),
                    source.try_share().unwrap(),
                    None,
                )
                .unwrap(),
        );
        let writer = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&replace))
        };
        let read = catalog.commit(&read) == Outcome::Committed;
        let replaced = writer.join().unwrap() == Outcome::Committed;
        assert_ne!(read, replaced);
        assert_eq!(child.status() == RootStatus::Live, read);
        let Some(EdgeMembership::Live(after)) = edge.membership() else {
            panic!("lost edge")
        };
        assert_eq!(after.target.is_none(), replaced);
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [1; 2]
        );
    });
}

#[test]
fn edge_effect_finishes_before_a_later_phase_can_be_observed() {
    loom::model(|| {
        let catalog = edge_catalog();
        let cell = fixture_cell(&catalog, 8);
        let source = reference_root(&catalog, &cell, 0);
        root_commit(
            &catalog,
            catalog.begin_publish_root(snapshot(&catalog), source.try_share().unwrap()),
        );
        let edge = model_edge(&catalog);
        let publish = trace_prepared(
            &catalog,
            catalog
                .begin_publish_edge(
                    snapshot(&catalog),
                    edge.try_share().unwrap(),
                    source.try_share().unwrap(),
                    Some(source.try_share().unwrap()),
                )
                .unwrap(),
        );
        assert!(catalog.install_and_stop(&publish));
        catalog.help_at(1, || {
            assert_eq!(edge.membership(), Some(EdgeMembership::Prepared));
            snapshot(&catalog);
            assert!(
                matches!(edge.membership(), Some(EdgeMembership::Live(_))),
                "phase escaped an incomplete edge effect"
            );
        });
        assert_eq!(catalog.commit(&publish), Outcome::Committed);
    });
}

#[test]
fn edge_failed_proposed_owner_leaves_the_request_cancellable() {
    loom::model(|| {
        let catalog = edge_catalog();
        let cell = fixture_cell(&catalog, 8);
        let source = reference_root(&catalog, &cell, 0);
        root_commit(
            &catalog,
            catalog.begin_publish_root(snapshot(&catalog), source.try_share().unwrap()),
        );
        let edge = model_edge(&catalog);
        let publish = trace_prepared(
            &catalog,
            catalog
                .begin_publish_edge(
                    snapshot(&catalog),
                    edge.try_share().unwrap(),
                    source.try_share().unwrap(),
                    None,
                )
                .unwrap(),
        );
        assert!(catalog.install_and_stop(&publish));
        FAIL_SHARE.with(|n| n.set(Some(0)));
        assert!(catalog.snapshot().is_none());
        assert!(publish.cancel(), "edge proposal acquired after COMMIT");
        assert_eq!(catalog.commit(&publish), Outcome::Cancelled);
        assert_eq!(edge.membership(), Some(EdgeMembership::Prepared));
    });
}

#[test]
fn edge_traversal_acknowledgement_is_helped_after_the_scanner_stops() {
    loom::model(|| {
        let catalog = Arc::new(edge_catalog());
        let cell = fixture_cell(&catalog, 8);
        let source = reference_root(&catalog, &cell, 0);
        root_commit(
            &catalog,
            catalog.begin_publish_root(snapshot(&catalog), source.try_share().unwrap()),
        );
        let edge = model_edge(&catalog);
        root_commit(
            &catalog,
            catalog.begin_publish_edge(
                snapshot(&catalog),
                edge.try_share().unwrap(),
                source.try_share().unwrap(),
                Some(source.try_share().unwrap()),
            ),
        );
        let Some(Membership::Tracing(marks)) = cell.membership() else {
            panic!()
        };
        let mut traversal: EdgeTraversal = catalog
            .begin_edge_traversal(snapshot(&catalog), cell, 0, marks[0].ticket)
            .unwrap();
        assert_eq!(traversal.scanned_slots(), 0);
        for _ in 0..16 {
            if traversal.install_pending_for_test(&catalog) {
                break;
            }
            assert_eq!(
                traversal.poll(&catalog, &()).unwrap(),
                EdgeTraversalProgress::Pending
            );
        }
        assert_eq!(traversal.scanned_slots(), 1);
        let helper = {
            let catalog = catalog.clone();
            thread::spawn(move || snapshot(&catalog).phase())
        };
        assert_eq!(
            helper.join().unwrap().domains[0].pending,
            0,
            "stopped scanner retained indispensable acknowledgement work"
        );
        assert_eq!(
            traversal.poll(&catalog, &()).unwrap(),
            EdgeTraversalProgress::Complete
        );
        assert_eq!(
            traversal.poll(&catalog, &()),
            Err(EdgeTraversalError::Finished)
        );
    });
}

#[test]
fn edge_traversal_cannot_acknowledge_a_stronger_replacement_ticket() {
    loom::model(|| {
        let catalog = edge_catalog();
        let cell = fixture_cell(&catalog, 8);
        let source = reference_root(&catalog, &cell, 0);
        root_commit(
            &catalog,
            catalog.begin_publish_root(snapshot(&catalog), source.try_share().unwrap()),
        );
        let _edge = model_edge(&catalog);
        batch_root_new_epoch(&catalog, &cell);
        root_commit(
            &catalog,
            catalog.begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Phantom),
        );
        let Some(Membership::Tracing(marks)) = cell.membership() else {
            panic!()
        };
        let mut traversal = catalog
            .begin_edge_traversal(snapshot(&catalog), cell.clone(), 0, marks[0].ticket)
            .unwrap();
        assert_eq!(
            traversal.poll(&catalog, &()).unwrap(),
            EdgeTraversalProgress::Pending
        );
        assert_eq!(traversal.scanned_slots(), 1);
        root_commit(
            &catalog,
            catalog.begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Strong),
        );
        assert_eq!(
            traversal.poll(&catalog, &()).unwrap(),
            EdgeTraversalProgress::Superseded
        );
        assert_eq!(
            snapshot(&catalog).phase().domains[0].pending,
            1,
            "old traversal acknowledged replacement work"
        );
    });
}

#[test]
fn edge_two_target_effects_finish_before_the_pending_count_is_visible() {
    loom::model(|| {
        let catalog = Catalog::with_record_root_capacities_and_limit(
            2,
            128,
            128,
            64,
            2,
            32,
            pool::MAX_GENERATION,
        )
        .with_edge_capacities(1, 32, 64, pool::MAX_GENERATION);
        let a = fixture_cell(&catalog, 8);
        let b = fixture_cell(&catalog, 16);
        let ar = reference_root(&catalog, &a, 0);
        let br = reference_root(&catalog, &b, 1);
        for root in [&ar, &br] {
            root_commit(
                &catalog,
                catalog.begin_publish_root(snapshot(&catalog), root.try_share().unwrap()),
            );
        }
        let edge = model_edge(&catalog);
        root_commit(
            &catalog,
            catalog.begin_publish_edge(
                snapshot(&catalog),
                edge.try_share().unwrap(),
                ar.try_share().unwrap(),
                Some(ar.try_share().unwrap()),
            ),
        );
        root_drain(&catalog, &b);
        batch_root_new_epoch(&catalog, &a);
        let replace = trace_prepared(
            &catalog,
            catalog
                .begin_replace_edge(
                    snapshot(&catalog),
                    edge.try_share().unwrap(),
                    ar.try_share().unwrap(),
                    Some(br.try_share().unwrap()),
                )
                .unwrap(),
        );
        assert!(catalog.install_and_stop(&replace));
        catalog.help_at(1, || {
            let phase = snapshot(&catalog).phase();
            assert_eq!(phase.domains[0].pending, 2);
            for cell in [&a, &b] {
                let Some(Membership::Tracing(marks)) = cell.membership() else {
                    panic!()
                };
                assert_eq!(marks[0].epoch, phase.domains[0].epoch);
                assert_ne!(
                    marks[0].ticket, 0,
                    "pending count escaped an incomplete target effect"
                );
            }
            let Some(EdgeMembership::Live(state)) = edge.membership() else {
                panic!()
            };
            assert_eq!(state.target.unwrap().identity, b.identity);
        });
        assert_eq!(catalog.commit(&replace), Outcome::Committed);
    });
}

#[test]
fn edge_source_retirement_allows_orphan_cleanup_without_late_replay() {
    loom::model(|| {
        let catalog = edge_catalog();
        let cell = fixture_cell(&catalog, 8);
        let root = reference_root(&catalog, &cell, 0);
        root_commit(
            &catalog,
            catalog.begin_publish_root(snapshot(&catalog), root.try_share().unwrap()),
        );
        let edge = model_edge(&catalog);
        let old = edge.try_share().unwrap();
        root_commit(
            &catalog,
            catalog.begin_publish_edge(
                snapshot(&catalog),
                edge.try_share().unwrap(),
                root.try_share().unwrap(),
                Some(root.try_share().unwrap()),
            ),
        );
        assert!(edge.release_to_source().is_ok());
        assert!(!old.removal_requested());
        assert!(
            catalog
                .begin_orphan_edge_removal(snapshot(&catalog), old.clone())
                .is_none()
        );
        root_commit(
            &catalog,
            catalog.begin_remove_root(snapshot(&catalog), root.try_share().unwrap()),
        );
        batch_root_new_epoch(&catalog, &cell);
        root_commit(
            &catalog,
            catalog.begin_scan_root_batch(snapshot(&catalog), 0),
        );
        for _ in 0..RETIREMENT_FLOOR {
            root_commit(&catalog, catalog.begin_advance(snapshot(&catalog), 0));
        }
        let retire = trace_prepared(
            &catalog,
            catalog
                .begin_trace_retire(snapshot(&catalog), cell.clone(), 0)
                .unwrap(),
        );
        assert!(catalog.install_and_stop(&retire));
        catalog.help_at(1, || {
            root_commit(
                &catalog,
                catalog.begin_orphan_edge_removal(snapshot(&catalog), old.clone()),
            );
            assert!(catalog.release_retired_edge(&old));
            let replacement = model_edge(&catalog);
            assert_ne!(old.identity, replacement.identity);
            assert!(!catalog.release_retired_edge(&old));
        });
        assert_eq!(catalog.commit(&retire), Outcome::Committed);
        assert_eq!(old.membership(), Some(EdgeMembership::Retired));
        assert!(matches!(
            catalog.discover_edge(0).unwrap().unwrap().membership(),
            Some(EdgeMembership::Prepared)
        ));
    });
}

fn indexed_model_catalog() -> Catalog {
    Catalog::with_record_root_capacities_and_limit(2, 128, 128, 64, 2, 32, pool::MAX_GENERATION)
        .with_edge_capacities(4, 32, 64, pool::MAX_GENERATION)
        .with_source_edge_degree(2)
}
fn indexed_model_edge(catalog: &Catalog, slot: usize) -> EdgeToken {
    let mut creation = catalog.begin_create_edge(slot).unwrap();
    (0..8)
        .find_map(|_| creation.poll(catalog).unwrap())
        .expect("indexed edge")
}

#[test]
fn indexed_publication_cannot_escape_its_source_range() {
    loom::model(|| {
        let catalog = indexed_model_catalog();
        let _unused = fixture_cell(&catalog, 8);
        let cell = fixture_cell(&catalog, 9);
        assert_eq!(cell.slot(), 1);
        let source = reference_root(&catalog, &cell, 1);
        root_commit(
            &catalog,
            catalog.begin_publish_root(snapshot(&catalog), source.try_share().unwrap()),
        );
        let misplaced = indexed_model_edge(&catalog, 0);
        let before = snapshot(&catalog).phase();
        assert!(
            catalog
                .begin_publish_edge(
                    snapshot(&catalog),
                    misplaced.try_share().unwrap(),
                    source.try_share().unwrap(),
                    None
                )
                .is_none(),
            "indexed model accepted misplaced publication"
        );
        assert_eq!(snapshot(&catalog).phase(), before);
        assert_eq!(misplaced.membership(), Some(EdgeMembership::Prepared));
        let last = indexed_model_edge(&catalog, 3);
        root_commit(
            &catalog,
            catalog.begin_publish_edge(
                snapshot(&catalog),
                last.try_share().unwrap(),
                source.try_share().unwrap(),
                None,
            ),
        );
        assert!(matches!(last.membership(), Some(EdgeMembership::Live(_))));
    });
}

#[test]
fn indexed_last_edge_is_traced_before_a_stopped_acknowledgement_is_helped() {
    // This setup needs the existing enlarged coroutine stack; no schedules are capped.
    notification_model(|| {
        let catalog = Arc::new(indexed_model_catalog());
        let child = fixture_cell(&catalog, 8);
        let source = fixture_cell(&catalog, 9);
        let cr = reference_root(&catalog, &child, 0);
        let sr = reference_root(&catalog, &source, 1);
        for root in [&cr, &sr] {
            root_commit(
                &catalog,
                catalog.begin_publish_root(snapshot(&catalog), root.try_share().unwrap()),
            );
        }
        let last = indexed_model_edge(&catalog, 3);
        root_commit(
            &catalog,
            catalog.begin_publish_edge(
                snapshot(&catalog),
                last.try_share().unwrap(),
                sr.try_share().unwrap(),
                Some(cr.try_share().unwrap()),
            ),
        );
        root_commit(
            &catalog,
            catalog.begin_remove_root(snapshot(&catalog), cr.try_share().unwrap()),
        );
        root_drain(&catalog, &child);
        root_drain(&catalog, &source);
        root_commit(
            &catalog,
            catalog.begin_scan_root_batch(snapshot(&catalog), 0),
        );
        for _ in 0..RETIREMENT_FLOOR {
            root_commit(&catalog, catalog.begin_advance(snapshot(&catalog), 0));
        }
        root_commit(&catalog, catalog.begin_finish(snapshot(&catalog), 0));
        root_commit(
            &catalog,
            catalog.begin_start(snapshot(&catalog), 0, DomainMode::Full),
        );
        root_commit(
            &catalog,
            catalog.begin_scan_root_batch(snapshot(&catalog), 0),
        );
        let Some(Membership::Tracing(marks)) = source.membership() else {
            panic!()
        };
        let mut scan = catalog
            .begin_edge_traversal(snapshot(&catalog), source.clone(), 0, marks[0].ticket)
            .unwrap();
        for _ in 0..40 {
            if scan.scanned_slots() == 2 && scan.install_pending_for_test(&catalog) {
                break;
            }
            assert_eq!(
                scan.poll(&catalog, &()).unwrap(),
                EdgeTraversalProgress::Pending
            );
        }
        assert_eq!(
            scan.scanned_slots(),
            2,
            "indexed model skipped a source position"
        );
        let helper = {
            let catalog = catalog.clone();
            thread::spawn(move || snapshot(&catalog).phase())
        };
        assert_eq!(
            helper.join().unwrap().domains[0].pending,
            1,
            "indexed acknowledgement lost last-child work"
        );
        let Some(Membership::Tracing(marks)) = child.membership() else {
            panic!()
        };
        assert_eq!(
            marks[0].retention,
            Some(TraceRetention::Strong),
            "indexed model skipped last child"
        );
        assert_ne!(marks[0].ticket, 0);
        assert_eq!(
            scan.poll(&catalog, &()).unwrap(),
            EdgeTraversalProgress::Complete
        );
    });
}

#[test]
fn stable_root_recovery_observes_publication_certificate() {
    notification_model(|| {
        let catalog = Arc::new(Catalog::with_record_root_capacities_and_limit(
            1,
            64,
            64,
            32,
            1,
            16,
            pool::MAX_GENERATION,
        ));
        let record = record_fixture::Fixture::new();
        let cell = record_cell(&catalog, record.clone());
        let mut creation = catalog.begin_create_root(0, cell.clone()).unwrap();
        let root = (0..8)
            .find_map(|_| creation.poll(&catalog).unwrap())
            .unwrap();
        let mut preparation = catalog
            .begin_publish_root(record_snapshot(&catalog), root.try_share().unwrap())
            .unwrap();
        let attempt = (0..8)
            .find_map(|_| preparation.poll(&catalog).unwrap())
            .unwrap();
        assert!(catalog.install_and_stop(&attempt));
        let (paused, pause) = loom::sync::mpsc::channel();
        let (resume, resumed) = loom::sync::mpsc::channel();
        root.pause_next_effect(move || {
            paused.send(()).unwrap();
            resumed.recv().unwrap();
        });
        let publisher = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit_in(&attempt, &7))
        };
        pause.recv().unwrap();
        assert_eq!(root.status(), RootStatus::Prepared);
        assert_eq!(record.0.published.load(Ordering::Acquire), 0);
        // The first stable-head attempt helps the stopped publication and is
        // inconclusive itself. A subsequent stable observation may return Live
        // only after publication's record certificate has been written.
        assert_eq!(catalog.root_is_stably_live_in(&root, &7), None);
        assert_eq!(catalog.root_is_stably_live_in(&root, &7), Some(true));
        assert_eq!(
            record.0.published.load(Ordering::Acquire),
            1,
            "stable root escaped before publication certificate"
        );
        resume.send(()).unwrap();
        assert_eq!(publisher.join().unwrap(), Outcome::Committed);
        record.release_origin();
    });
}

#[test]
fn stable_root_recovery_rejects_failed_loads_foreign_roots_and_revocation() {
    notification_model(|| {
        let catalog = root_catalog();
        let cell = fixture_cell(&catalog, 8);
        let root = model_root(&catalog, &cell);
        assert_eq!(catalog.root_is_stably_live_in(&root, &()), Some(false));
        root_commit(
            &catalog,
            catalog.begin_publish_root(snapshot(&catalog), root.try_share().unwrap()),
        );
        for index in 0..2 {
            FAIL_LOAD.with(|slot| slot.set(Some(index)));
            assert_eq!(
                catalog.root_is_stably_live_in(&root, &()),
                None,
                "failed root observation became a certificate"
            );
            FAIL_LOAD.with(|slot| slot.set(None));
        }
        assert_eq!(root_catalog().root_is_stably_live_in(&root, &()), None);
        assert_eq!(catalog.root_is_stably_live_in(&root, &()), Some(true));
        root_commit(
            &catalog,
            catalog.begin_remove_root(snapshot(&catalog), root.try_share().unwrap()),
        );
        assert_eq!(catalog.root_is_stably_live_in(&root, &()), Some(false));
    });
}

#[test]
fn phase_only_cancellation_and_help_select_one_immutable_outcome() {
    notification_model(|| {
        let catalog = Arc::new(Catalog::new());
        let before = snapshot(&catalog).phase();
        let attempt = Arc::new(catalog.advance(snapshot(&catalog), 0).unwrap());
        assert!(catalog.install_and_stop(&attempt));
        let canceller = {
            let attempt = attempt.clone();
            thread::spawn(move || attempt.cancel())
        };
        let outcome = catalog.commit(&attempt);
        let cancelled = canceller.join().unwrap();
        assert_eq!(outcome == Outcome::Cancelled, cancelled);
        assert!(matches!(outcome, Outcome::Committed | Outcome::Cancelled));
        let chosen = snapshot(&catalog).phase();
        assert_eq!(chosen.revision, before.revision + 1);
        assert_eq!(chosen.domains[0].floor, u8::from(!cancelled));
        assert_eq!(chosen.domains.map(|d| d.pending), [0, 0]);
        assert_eq!(
            catalog.commit(&catalog.advance(snapshot(&catalog), 0).unwrap()),
            Outcome::Committed
        );
        let later = snapshot(&catalog).phase();
        catalog.late_help(&attempt);
        assert_eq!(catalog.commit(&attempt), outcome);
        assert_eq!(snapshot(&catalog).phase(), later);
    });
}

#[test]
fn phase_only_stopped_installation_preserves_late_result_and_snapshot() {
    notification_model(|| {
        let catalog = Arc::new(Catalog::new());
        let attempt = Arc::new(catalog.advance(snapshot(&catalog), 0).unwrap());
        assert!(catalog.install_and_stop(&attempt));
        assert!(
            catalog.snapshot().is_none(),
            "undecided phase escaped as a snapshot"
        );
        let retained = snapshot(&catalog);
        let committed = retained.phase();
        assert_eq!(committed.domains[0].floor, 1);
        let successor = {
            let catalog = catalog.clone();
            thread::spawn(move || catalog.commit(&catalog.advance(snapshot(&catalog), 0).unwrap()))
        };
        assert_eq!(catalog.commit(&attempt), Outcome::Committed);
        assert_eq!(successor.join().unwrap(), Outcome::Committed);
        assert_eq!(retained.phase(), committed);
        let later = snapshot(&catalog).phase();
        assert_eq!(later.domains[0].floor, 2);
        catalog.late_help(&attempt);
        assert_eq!(
            catalog.commit(&attempt),
            Outcome::Committed,
            "lost completed phase result"
        );
        assert!(!attempt.cancel());
        assert_eq!(
            snapshot(&catalog).phase(),
            later,
            "late phase helper rewound the head"
        );
    });
}

#[test]
fn phase_only_failed_ownership_and_stale_preparation_preserve_current_phase() {
    notification_model(|| {
        let catalog = Catalog::new();
        let mut older = catalog.begin_advance(snapshot(&catalog), 0).unwrap();
        FAIL_ALLOCATION.with(|counter| counter.set(Some(0)));
        assert!(older.poll(&catalog).unwrap().is_none());
        assert_eq!(snapshot(&catalog).phase().domains[0].floor, 0);
        let competing = catalog.advance(snapshot(&catalog), 0).unwrap();
        FAIL_SHARE.with(|counter| counter.set(Some(0)));
        assert_eq!(catalog.commit(&competing), Outcome::Pending);
        assert_eq!(snapshot(&catalog).phase().domains[0].floor, 0);
        assert!(catalog.install_and_stop(&competing));
        FAIL_LOAD.with(|counter| counter.set(Some(0)));
        assert!(catalog.snapshot().is_none());
        assert!(
            catalog.snapshot().is_none(),
            "failed observation certified the phase"
        );
        let current = snapshot(&catalog).phase();
        assert_eq!(current.domains[0].floor, 1);
        let stale = older.poll(&catalog).unwrap().unwrap();
        assert_eq!(catalog.commit(&stale), Outcome::Retry);
        assert_eq!(catalog.commit(&competing), Outcome::Committed);
        assert_eq!(snapshot(&catalog).phase(), current);
    });
}

#[test]
fn cancelled_vacancy_preference_allows_competitors_to_announce() {
    loom::model(|| {
        let preference = Arc::new(vacancy::Preference::default());
        assert!(preference.nominate(1));
        let helper = preference.clone();
        let release = thread::spawn(move || helper.offer(0));
        let owner = preference.clone();
        let cancel = thread::spawn(move || owner.finish(1));
        release.join().unwrap();
        cancel.join().unwrap();
        assert!(!preference.defer(0, 2));
        assert!(preference.nominate(2));
        assert!(!preference.defer(0, 2));
    });
}

#[test]
fn an_effect_successor_exposes_only_certified_membership_while_its_helper_stops() {
    notification_model(|| {
        let catalog = Arc::new(Catalog::new());
        let first = fixture_cell(&catalog, 8);
        let second = fixture_cell(&catalog, 16);
        let before = snapshot(&catalog).phase();
        let attempt = Arc::new(catalog.insert(snapshot(&catalog), first.clone()).unwrap());
        assert!(catalog.install_and_stop(&attempt));
        let cancelling = attempt.clone();
        let cancel = thread::spawn(move || cancelling.cancel());
        // Stop the first helper after its certificate and before it releases
        // the head. A second logical helper must select the same phase and
        // publish an unrelated record while the first still retains Busy.
        catalog.help_at(2, || {
            let phase = snapshot(&catalog).phase();
            assert_eq!(phase.revision, before.revision + 1);
            let committed = matches!(first.membership(), Some(Membership::Live(_)));
            assert_eq!(
                phase.domains.map(|domain| domain.pending),
                [usize::from(committed); 2]
            );
            let other = catalog.insert(snapshot(&catalog), second.clone()).unwrap();
            assert_eq!(catalog.commit(&other), Outcome::Committed);
        });
        let cancelled = cancel.join().unwrap();
        assert_eq!(
            catalog.commit(&attempt),
            if cancelled {
                Outcome::Cancelled
            } else {
                Outcome::Committed
            }
        );
        let current = snapshot(&catalog).phase();
        assert_eq!(current.revision, before.revision + 2);
        assert_eq!(
            current.domains.map(|domain| domain.pending),
            [if cancelled { 1 } else { 2 }; 2]
        );
        assert!(matches!(second.membership(), Some(Membership::Live(_))));
    });
}
