use super::*;
use crate::FAIL_LOAD;
use std::cell::RefCell;

thread_local! {
    pub(crate) static AFTER_ACQUIRE: RefCell<Option<Box<dyn FnOnce()>>> = RefCell::new(None);
}
pub(crate) fn after_acquire() {
    let hook = AFTER_ACQUIRE.with(|slot| slot.borrow_mut().take());
    if let Some(hook) = hook {
        hook();
    }
}

fn prepare<R: Record>(catalog: &Catalog<R>, mut preparation: Preparation<R>) -> Attempt<R> {
    for _ in 0..256 {
        if let Some(attempt) = preparation.poll(catalog).unwrap() {
            return attempt;
        }
    }
    panic!("fixture preparation did not finish");
}

pub(crate) fn passive_observation_stable_and_single_acquisition_failure() {
    let catalog = Catalog::new();
    let phase = catalog.snapshot().unwrap().phase();
    let occupancy = catalog.pool_occupancy();
    assert_eq!(catalog.possible_phase_outcomes(), Some([phase; 2]));
    FAIL_LOAD.with(|fail| fail.set(Some(0)));
    assert_eq!(catalog.possible_phase_outcomes(), None);
    assert_eq!(FAIL_LOAD.with(|fail| fail.get()), None);
    assert_eq!(catalog.possible_phase_outcomes(), Some([phase; 2]));
    assert_eq!(catalog.pool_occupancy(), occupancy);
}

pub(crate) fn passive_observation_phase_decisions_are_copied_without_help() {
    for decision in [UNDECIDED, COMMIT, CANCEL] {
        let catalog = Catalog::new();
        let attempt = prepare(
            &catalog,
            catalog
                .begin_advance(catalog.snapshot().unwrap(), 0)
                .unwrap(),
        );
        assert!(catalog.install_and_stop(&attempt));
        let Control::Phase(operation) = &*attempt.busy else {
            panic!("phase fixture")
        };
        operation.decision.store(decision, Ordering::Release);
        let expected = match decision {
            COMMIT => [operation.committed; 2],
            CANCEL => [operation.cancelled; 2],
            _ => [operation.cancelled, operation.committed],
        };
        assert_eq!(catalog.possible_phase_outcomes(), Some(expected));
        assert_eq!(operation.decision.load(Ordering::Acquire), decision);
        assert!(Pooled::ptr_eq(&catalog.head.load().unwrap(), &attempt.busy));
    }
}

struct Tracked {
    enters: Arc<AtomicUsize>,
    drops: Arc<AtomicUsize>,
}
impl Record for Tracked {
    type Context = ();
    type Guard<'a> = ();
    fn enter<'a>(&'a self, _: &'a ()) -> Result<Entered<'a, ()>, RecordError> {
        self.enters.fetch_add(1, Ordering::Relaxed);
        Ok(Entered {
            _guard: (),
            published: None,
        })
    }
}
impl Drop for Tracked {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}
type BusyFixture = (
    Arc<Catalog<Tracked>>,
    Attempt<Tracked>,
    Pooled<Cell<Tracked>>,
    Arc<AtomicUsize>,
    Arc<AtomicUsize>,
);
fn busy_fixture() -> BusyFixture {
    let catalog = Arc::new(Catalog::with_record_capacities_and_limit(
        1,
        32,
        32,
        16,
        pool::MAX_GENERATION,
    ));
    let enters = Arc::new(AtomicUsize::new(0));
    let drops = Arc::new(AtomicUsize::new(0));
    let mut creation = catalog
        .begin_create_with(
            0,
            8,
            Tracked {
                enters: enters.clone(),
                drops: drops.clone(),
            },
        )
        .unwrap();
    let cell = (0..16)
        .find_map(|_| creation.poll(&catalog).unwrap())
        .unwrap();
    let attempt = prepare(
        &catalog,
        catalog
            .begin_publish(catalog.snapshot_in(&()).unwrap(), share(&cell).unwrap())
            .unwrap(),
    );
    assert!(catalog.install_and_stop(&attempt));
    (catalog, attempt, cell, enters, drops)
}

pub(crate) fn passive_observation_busy_never_certifies_effect_completion_or_enters_record() {
    for decision in [UNDECIDED, COMMIT, CANCEL] {
        let (catalog, attempt, cell, enters, drops) = busy_fixture();
        let Control::Busy(operation) = &*attempt.busy else {
            panic!("busy fixture")
        };
        let cancelled = &operation.outcome().cancelled;
        let committed = &operation.outcome().committed;
        operation.decision.store(decision, Ordering::Release);
        let before = (
            catalog.controls.occupied(),
            catalog.memberships.occupied(),
            catalog.cells.occupied(),
        );
        assert_eq!(
            catalog.possible_phase_outcomes(),
            Some([*cancelled, *committed])
        );
        assert_eq!(operation.decision.load(Ordering::Acquire), decision);
        assert_eq!(
            operation.outcome().decision.load(Ordering::Acquire),
            UNDECIDED
        );
        assert_eq!(cell.membership(), Some(Membership::Prepared));
        assert_eq!(enters.load(Ordering::Acquire), 0);
        assert_eq!(
            before,
            (
                catalog.controls.occupied(),
                catalog.memberships.occupied(),
                catalog.cells.occupied()
            )
        );
        drop((attempt, cell, catalog));
        assert_eq!(drops.load(Ordering::Acquire), 1);
    }
}

pub(crate) fn passive_observation_parked_busy_owner_releases_transitive_record_on_return() {
    let (catalog, attempt, cell, enters, drops) = busy_fixture();
    let Control::Busy(operation) = &*attempt.busy else {
        panic!("busy fixture")
    };
    let committed = &operation.outcome().committed;
    let old_phase = *committed;
    let mut later = old_phase;
    later.domains[0].epoch += 1;
    let replacing = catalog.clone();
    let observed_drops = drops.clone();
    AFTER_ACQUIRE.with(|slot| {
        *slot.borrow_mut() = Some(Box::new(move || {
            let current = replacing.head.load().unwrap();
            let replacement =
                allocate(&replacing.controls, Control::Stable(later)).expect("free controller");
            assert!(replacing.head.compare(&current, replacement));
            assert!(replacing.slots[0].compare(Some(&cell), None));
            drop((current, attempt, cell));
            assert_eq!(
                observed_drops.load(Ordering::Acquire),
                0,
                "parked head lost its record"
            );
        }))
    });
    let outcomes = catalog.possible_phase_outcomes().unwrap();
    assert_eq!(
        outcomes[1], old_phase,
        "observation must not refresh after acquiring its head"
    );
    assert_eq!(catalog.possible_phase_outcomes().unwrap(), [later; 2]);
    assert_eq!(enters.load(Ordering::Acquire), 0);
    assert_eq!(
        drops.load(Ordering::Acquire),
        1,
        "temporary Drop can destroy a record"
    );
    assert_eq!(catalog.controls.occupied(), 1);
    assert_eq!(catalog.memberships.occupied(), 0);
    assert_eq!(catalog.cells.occupied(), 0);
}
