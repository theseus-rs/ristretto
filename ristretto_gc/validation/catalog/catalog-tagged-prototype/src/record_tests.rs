use super::record_fixture::Fixture;
use super::*;

fn catalog() -> Catalog<Fixture> {
    Catalog::with_record_capacities_and_limit(1, 64, 64, 32, pool::MAX_GENERATION)
}
fn snapshot(catalog: &Catalog<Fixture>) -> Snapshot<Fixture> {
    for _ in 0..16 {
        if let Some(s) = catalog.snapshot_in(&7) {
            return s;
        }
    }
    panic!("record controller did not quiesce")
}
fn cell(catalog: &Catalog<Fixture>, record: Fixture) -> Pooled<Cell<Fixture>> {
    let mut creation = catalog.begin_create_with(0, 8, record).unwrap();
    for _ in 0..16 {
        if let Some(cell) = creation.poll(catalog).unwrap() {
            return cell;
        }
    }
    panic!("record creation did not settle")
}
fn publish(catalog: &Catalog<Fixture>, cell: &Pooled<Cell<Fixture>>) -> Attempt<Fixture> {
    let mut prep = catalog
        .begin_publish(snapshot(catalog), cell.try_clone().unwrap())
        .unwrap();
    for _ in 0..16 {
        if let Some(a) = prep.poll(catalog).unwrap() {
            return a;
        }
    }
    panic!("record preparation did not settle")
}

#[test]
fn closed_and_foreign_records_cancel_without_membership_or_work_effects() {
    for closed in [false, true] {
        let catalog = catalog();
        let record = Fixture::new();
        let cell = cell(&catalog, record.clone());
        let attempt = publish(&catalog, &cell);
        if closed {
            record.release_origin();
        }
        assert_eq!(
            catalog.commit_in(&attempt, if closed { &7 } else { &9 }),
            Outcome::Cancelled
        );
        assert_eq!(cell.membership(), Some(Membership::Prepared));
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [0, 0]
        );
        assert_eq!(record.0.published.load(Ordering::Acquire), 0);
        if !closed {
            record.release_origin();
        }
        assert_eq!(record.0.last_release.load(Ordering::Acquire), 0);
    }
}

#[test]
fn contended_record_admission_leaves_installed_work_cancellable() {
    let catalog = catalog();
    let record = Fixture::new();
    let cell = cell(&catalog, record.clone());
    let attempt = publish(&catalog, &cell);
    record.0.contended.store(true, Ordering::Release);
    assert_eq!(catalog.commit_in(&attempt, &7), Outcome::Pending);
    assert!(
        attempt.cancel(),
        "graph admission made an irreversible commit decision"
    );
    assert_eq!(catalog.commit_in(&attempt, &7), Outcome::Cancelled);
    assert_eq!(
        snapshot(&catalog).phase().domains.map(|d| d.pending),
        [0, 0]
    );
    record.release_origin();
    assert_eq!(record.0.last_release.load(Ordering::Acquire), 0);
}

#[test]
fn publication_latch_precedes_final_graph_release_and_closed_head_help() {
    let catalog = catalog();
    let record = Fixture::new();
    let cell = cell(&catalog, record.clone());
    let attempt = publish(&catalog, &cell);
    // Busy owner and effect clone succeed; final stable-head clone fails.
    FAIL_SHARE.with(|n| n.set(Some(2)));
    assert_eq!(catalog.commit_in(&attempt, &7), Outcome::Committed);
    assert_eq!(record.0.published.load(Ordering::Acquire), 1);
    record.release_origin();
    assert_eq!(
        record.0.last_release.load(Ordering::Acquire),
        1,
        "cleanup missed canonical ownership"
    );
    let enters = record.0.enters.load(Ordering::Acquire);
    assert!(catalog.snapshot_in(&9).is_none());
    assert_eq!(
        record.0.enters.load(Ordering::Acquire),
        enters,
        "certified head release reacquired a closed graph"
    );
    assert_eq!(
        snapshot(&catalog).phase().domains.map(|d| d.pending),
        [1, 1]
    );
}

#[test]
fn abandoned_prepared_records_need_no_graph_admission() {
    let catalog = catalog();
    let record = Fixture::new();
    let cell = cell(&catalog, record.clone());
    record.release_origin();
    let mut prep = catalog
        .begin_abandon(snapshot(&catalog), cell.try_clone().unwrap())
        .unwrap();
    let mut ready = None;
    for _ in 0..16 {
        if let Some(a) = prep.poll(&catalog).unwrap() {
            ready = Some(a);
            break;
        }
    }
    assert_eq!(catalog.commit_in(&ready.unwrap(), &9), Outcome::Committed);
    assert_eq!(cell.membership(), Some(Membership::Retired));
    assert_eq!(record.0.enters.load(Ordering::Acquire), 0);
    assert_eq!(record.0.published.load(Ordering::Acquire), 0);
    assert!(catalog.release_retired(&cell));
}

#[test]
#[cfg(all(not(target_family = "wasm"), not(miri)))]
fn another_helper_finishes_while_the_first_guards_a_stopped_effect() {
    use std::sync::mpsc;
    use std::time::Duration;
    for point in 0..3 {
        let catalog = Arc::new(catalog());
        let record = Fixture::new();
        let cell = cell(&catalog, record.clone());
        let attempt = publish(&catalog, &cell);
        assert!(catalog.install_and_stop(&attempt));
        let (entered, entry) = mpsc::channel();
        let (resume, resumed) = mpsc::channel();
        let stalled = {
            let catalog = catalog.clone();
            std::thread::spawn(move || {
                catalog.help_record_at(&7, point, || {
                    entered.send(()).unwrap();
                    resumed.recv_timeout(Duration::from_secs(10)).unwrap();
                });
            })
        };
        entry.recv_timeout(Duration::from_secs(10)).unwrap();
        assert_eq!(record.0.handles.load(Ordering::Acquire), 2);
        record.release_origin();
        assert_eq!(record.0.handles.load(Ordering::Acquire), 1);
        assert_eq!(catalog.commit_in(&attempt, &7), Outcome::Committed);
        assert_eq!(
            snapshot(&catalog).phase().domains.map(|d| d.pending),
            [1, 1]
        );
        assert_eq!(record.0.handles.load(Ordering::Acquire), 1);
        resume.send(()).unwrap();
        stalled.join().unwrap();
        assert_eq!(record.0.handles.load(Ordering::Acquire), 0);
        assert_eq!(record.0.last_release.load(Ordering::Acquire), 1);
    }
}

#[test]
fn cancellation_during_graph_admission_keeps_private_cleanup_eligible() {
    let catalog = catalog();
    let record = Fixture::new();
    let cell = cell(&catalog, record.clone());
    let attempt = Arc::new(publish(&catalog, &cell));
    let cancel = attempt.clone();
    *record.0.after_admission.lock().unwrap() = Some(Box::new(move || {
        assert!(cancel.cancel());
    }));
    assert_eq!(catalog.commit_in(&attempt, &7), Outcome::Cancelled);
    assert_eq!(cell.membership(), Some(Membership::Prepared));
    assert_eq!(
        record.0.published.load(Ordering::Acquire),
        0,
        "cancelled graph was labelled published"
    );
    record.release_origin();
    assert_eq!(record.0.last_release.load(Ordering::Acquire), 0);
    assert_eq!(
        snapshot(&catalog).phase().domains.map(|d| d.pending),
        [0, 0]
    );
}

#[test]
fn creation_preflight_returns_the_exact_noncloneable_record_on_every_rejection() {
    struct Token {
        inner: Fixture,
        drops: Arc<AtomicUsize>,
    }
    impl Drop for Token {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::Relaxed);
        }
    }
    impl Record for Token {
        type Context = usize;
        type Guard<'a> = crate::record_fixture::Guard<'a>;
        fn enter<'a>(
            &'a self,
            context: &'a usize,
        ) -> Result<Entered<'a, Self::Guard<'a>>, RecordError> {
            self.inner.enter(context)
        }
    }
    let catalog =
        Catalog::<Token>::with_record_capacities_and_limit(1, 64, 64, 32, pool::MAX_GENERATION);
    let origin = Fixture::new();
    let drops = Arc::new(AtomicUsize::new(0));
    let mut record = Token {
        inner: origin.clone(),
        drops: drops.clone(),
    };
    for (slot, address, error, contended) in [
        (1, 8, PrepareError::InvalidSlot, false),
        (0, 0, PrepareError::InvalidAddress, false),
        (0, 8, PrepareError::Contended, true),
    ] {
        if contended {
            FAIL_LOAD.with(|n| n.set(Some(0)));
        }
        let failure = match catalog.begin_create_with(slot, address, record) {
            Err(failure) => failure,
            Ok(_) => panic!("rejected preflight unexpectedly created a record"),
        };
        assert_eq!(failure.error, error);
        record = failure.record;
        assert!(Arc::ptr_eq(&record.inner.0, &origin.0));
        assert_eq!(
            drops.load(Ordering::Acquire),
            0,
            "preflight destroyed the unique record"
        );
    }
    let mut creation = catalog.begin_create_with(0, 8, record).unwrap();
    let cell = loop {
        if let Some(cell) = creation.poll(&catalog).unwrap() {
            break cell;
        }
    };
    let other = Fixture::new();
    let second = Token {
        inner: other.clone(),
        drops: drops.clone(),
    };
    let failure = match catalog.begin_create_with(0, 8, second) {
        Err(e) => e,
        Ok(_) => panic!("occupied slot"),
    };
    assert_eq!(failure.error, PrepareError::Occupied);
    assert!(Arc::ptr_eq(&failure.record.inner.0, &other.0));
    assert_eq!(drops.load(Ordering::Acquire), 0);
    drop(failure);
    assert_eq!(drops.load(Ordering::Acquire), 1);
    drop(cell);
    drop(creation);
    drop(catalog);
    assert_eq!(drops.load(Ordering::Acquire), 2);
    origin.release_origin();
    other.release_origin();
}
