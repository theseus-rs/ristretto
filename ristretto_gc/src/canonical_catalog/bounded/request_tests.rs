use super::*;

fn catalog() -> (CatalogHandle, Arc<Budget>) {
    let budget = Budget::new(usize::MAX);
    let catalog = BoundedCatalog::new_in(Capacities::derive(2, 1, 1).unwrap(), &budget).unwrap();
    (catalog, budget)
}
fn created(worker: &mut Participant) -> RecordKey {
    worker.submit_record(0, 8, ()).unwrap();
    for _ in 0..8 {
        if let Progress::Created(key) = worker.poll() {
            return key;
        }
    }
    panic!("finite creation did not finish");
}
fn reach_attempt(worker: &mut Participant, key: RecordKey) {
    worker.submit(Command::Publish(key)).unwrap();
    for _ in 0..8 {
        assert_eq!(worker.poll(), Progress::Pending);
        if matches!(worker.state, RequestState::Attempt(..)) {
            return;
        }
    }
    panic!("finite proposal preparation did not finish");
}

#[test]
fn stopped_participant_request_remains_helpable_and_quota_is_reusable() {
    let (catalog, budget) = catalog();
    let mut worker = catalog.register_mutator().unwrap();
    let key = created(&mut worker);
    reach_attempt(&mut worker, key);
    let RequestState::Attempt(_, attempt) = &worker.state else {
        panic!("prepared attempt")
    };
    catalog.raw.effect_and_stop(attempt);
    drop(worker);
    let mut helper = catalog.register_helper().unwrap();
    for _ in 0..4 {
        let _ = helper.phase();
    }
    let mut replacement = catalog.register_mutator().unwrap();
    assert_eq!(replacement.phase().unwrap().domains[0].pending, 1);
    assert!(matches!(
        replacement.observe(key).unwrap().membership,
        Membership::Tracing(_)
    ));
    drop((helper, replacement, catalog));
    assert_eq!(budget.used(), 0);
}

#[test]
fn cancellation_before_selection_preserves_prepared_record_for_abandonment() {
    let (catalog, budget) = catalog();
    let mut worker = catalog.register_mutator().unwrap();
    let key = created(&mut worker);
    reach_attempt(&mut worker, key);
    drop(worker);
    let mut replacement = catalog.register_mutator().unwrap();
    assert_eq!(
        replacement.observe(key).unwrap().membership,
        Membership::Prepared
    );
    replacement.submit(Command::Abandon(key)).unwrap();
    for _ in 0..8 {
        if replacement.poll() == Progress::Committed {
            break;
        }
    }
    assert_eq!(
        replacement.observe(key).unwrap().membership,
        Membership::Retired
    );
    assert!(replacement.release_retired(key).unwrap());
    assert!(replacement.discover(0).unwrap().is_none());
    drop((replacement, catalog));
    assert_eq!(budget.used(), 0);
}

struct Payload {
    identity: usize,
    drops: Arc<AtomicUsize>,
    published: std::sync::atomic::AtomicU8,
}
impl super::Payload for Payload {
    type Context = usize;
    type Guard<'a> = ();
    fn enter<'a>(
        &'a self,
        context: &'a usize,
    ) -> Result<super::super::Entered<'a, ()>, super::super::RecordError> {
        if *context != self.identity {
            return Err(super::super::RecordError::Foreign);
        }
        Ok(super::super::Entered::new((), &self.published))
    }
}
impl Drop for Payload {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}
fn payload(drops: &Arc<AtomicUsize>) -> Payload {
    Payload {
        identity: 42,
        drops: drops.clone(),
        published: std::sync::atomic::AtomicU8::new(0),
    }
}
fn record_catalog() -> (CatalogHandle<Payload>, Arc<Budget>, Arc<AtomicUsize>) {
    let budget = Budget::new(usize::MAX);
    let catalog =
        BoundedCatalog::<Payload>::with_records_in(Capacities::derive(1, 1, 1).unwrap(), &budget)
            .unwrap();
    (catalog, budget, Arc::new(AtomicUsize::new(0)))
}
fn record_created(worker: &mut Participant<Payload, GuardedCommand>, value: Payload) -> RecordKey {
    worker.submit_record(0, 8, value).unwrap();
    for _ in 0..8 {
        if let Progress::Created(key) = worker.poll_in(&42) {
            return key;
        }
    }
    panic!("finite record creation did not finish");
}
fn record_attempt(worker: &mut Participant<Payload, GuardedCommand>, key: RecordKey) {
    worker.submit(GuardedCommand::Publish(key)).unwrap();
    for _ in 0..8 {
        assert_eq!(worker.poll_in(&42), Progress::Pending);
        if matches!(worker.state, RequestState::Attempt(..)) {
            return;
        }
    }
    panic!("finite record proposal did not finish");
}

#[test]
fn dropping_creator_at_each_private_boundary_releases_one_exact_payload() {
    for polls in 0..=4 {
        let (catalog, budget, drops) = record_catalog();
        let mut worker = catalog.register_record_mutator().unwrap();
        worker.submit_record(0, 8, payload(&drops)).unwrap();
        for _ in 0..polls {
            let _ = worker.poll_in(&42);
        }
        drop(worker);
        let mut replacement = catalog.register_record_mutator().unwrap();
        if let Some(record) = replacement.discover(0).unwrap() {
            assert_eq!(drops.load(Ordering::Relaxed), 0);
            replacement
                .submit(GuardedCommand::Abandon(record.key))
                .unwrap();
            for _ in 0..8 {
                if replacement.poll_in(&42) == Progress::Committed {
                    break;
                }
            }
            assert!(replacement.release_retired(record.key).unwrap());
        }
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        drop((replacement, catalog));
        assert_eq!(budget.used(), 0);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
    }
}

#[test]
fn dropping_creator_after_commit_selection_cannot_cancel_or_double_drop() {
    for stop in 0..=3 {
        let (catalog, budget, drops) = record_catalog();
        let mut worker = catalog.register_record_mutator().unwrap();
        let key = record_created(&mut worker, payload(&drops));
        record_attempt(&mut worker, key);
        let RequestState::Attempt(_, attempt) = &worker.state else {
            panic!("prepared attempt")
        };
        assert!(catalog.raw.install_and_stop(attempt));
        catalog.raw.help_record_at(&42, stop, || drop(worker));
        let mut replacement = catalog.register_record_mutator().unwrap();
        assert_eq!(replacement.phase_in(&42).unwrap().domains[0].pending, 1);
        assert!(matches!(
            replacement.observe(key).unwrap().membership,
            Membership::Tracing(_)
        ));
        assert_eq!(
            catalog
                .raw
                .discover(0)
                .unwrap()
                .unwrap()
                .record()
                .published
                .load(Ordering::Acquire),
            1
        );
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        drop((replacement, catalog));
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(budget.used(), 0);
    }
}

#[test]
fn dropping_selected_undecided_request_cancels_and_refunds_after_abandonment() {
    let (catalog, budget, drops) = record_catalog();
    let mut worker = catalog.register_record_mutator().unwrap();
    let key = record_created(&mut worker, payload(&drops));
    record_attempt(&mut worker, key);
    let RequestState::Attempt(_, attempt) = &worker.state else {
        panic!("prepared attempt")
    };
    assert!(catalog.raw.install_and_stop(attempt));
    drop(worker);
    let mut replacement = catalog.register_record_mutator().unwrap();
    for _ in 0..4 {
        let _ = replacement.phase_in(&42);
    }
    assert_eq!(replacement.phase_in(&42).unwrap().domains[0].pending, 0);
    assert_eq!(
        replacement.observe(key).unwrap().membership,
        Membership::Prepared
    );
    replacement.submit(GuardedCommand::Abandon(key)).unwrap();
    for _ in 0..8 {
        if replacement.poll_in(&42) == Progress::Committed {
            break;
        }
    }
    assert!(replacement.release_retired(key).unwrap());
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    drop((replacement, catalog));
    assert_eq!(budget.used(), 0);
}

#[test]
fn busy_submission_returns_the_exact_unconsumed_record() {
    let (catalog, budget, drops) = record_catalog();
    let mut worker = catalog.register_record_mutator().unwrap();
    worker.submit_record(0, 8, payload(&drops)).unwrap();
    let mut second = payload(&drops);
    second.identity = 99;
    let failure = worker.submit_record(0, 16, second).unwrap_err();
    assert_eq!(failure.error, RequestError::Busy);
    assert_eq!(failure.record.identity, 99);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(failure);
    drop((worker, catalog));
    assert_eq!(drops.load(Ordering::Relaxed), 2);
    assert_eq!(budget.used(), 0);
}

#[test]
fn private_record_drops_before_its_participant_quota_and_catalog_charge() {
    let (catalog, budget, drops) = record_catalog();
    let called = Arc::new(AtomicUsize::new(0));
    let mut worker = catalog.register_record_mutator().unwrap();
    worker.submit_record(0, 8, payload(&drops)).unwrap();
    let observed_drops = drops.clone();
    let observed_budget = budget.clone();
    let observed_called = called.clone();
    worker.lease.before_release = Some(Box::new(move || {
        assert_eq!(observed_drops.load(Ordering::Relaxed), 1);
        assert!(observed_budget.used() > 0);
        observed_called.fetch_add(1, Ordering::Relaxed);
    }));
    drop(catalog);
    drop(worker);
    assert_eq!(called.load(Ordering::Relaxed), 1);
    assert_eq!(budget.used(), 0);
}
