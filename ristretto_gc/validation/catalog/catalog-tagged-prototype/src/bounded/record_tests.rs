use super::*;
use crate::record_fixture::{Fixture, Guard};
use crate::{Entered, FAIL_LOAD, FAIL_SHARE, RecordError};

// Deliberately non-cloneable and larger than the numeric record.
struct Token {
    fixture: Fixture,
    identity: u64,
    drops: Arc<AtomicUsize>,
    padding: [u64; 64],
    on_drop: Option<Box<dyn FnOnce() + Send + Sync>>,
}
impl Token {
    fn new(identity: u64) -> Self {
        Self {
            fixture: Fixture::new(),
            identity,
            drops: Arc::new(AtomicUsize::new(0)),
            padding: [identity; 64],
            on_drop: None,
        }
    }
}
impl Drop for Token {
    fn drop(&mut self) {
        assert_eq!(self.padding, [self.identity; 64]);
        self.drops.fetch_add(1, Ordering::Release);
        if let Some(callback) = self.on_drop.take() {
            callback();
        }
    }
}
impl Payload for Token {
    type Context = usize;
    type Guard<'a> = Guard<'a>;
    fn enter<'a>(&'a self, context: &'a usize) -> Result<Entered<'a, Guard<'a>>, RecordError> {
        self.fixture.enter(context)
    }
}
type Worker = Participant<Token, GuardedCommand>;
fn setup(mutators: usize) -> (CatalogHandle<Token>, Arc<Budget>) {
    let capacity = Capacities::derive(2, mutators, 1).unwrap();
    let footprint = Footprint::for_records::<Token>(capacity).unwrap();
    let budget = Budget::new(footprint.total());
    let catalog = BoundedCatalog::<Token>::with_records_in(capacity, &budget).unwrap();
    assert_eq!(budget.used(), footprint.total());
    assert_eq!(catalog.footprint(), footprint);
    (catalog, budget)
}
fn finish(worker: &mut Worker, context: usize) -> Progress {
    for _ in 0..4096 {
        let progress = worker.poll_in(&context);
        if progress != Progress::Pending {
            return progress;
        }
    }
    panic!("record request did not settle");
}
fn create(worker: &mut Worker, slot: usize, token: Token) -> RecordKey {
    worker.submit_record(slot, 8 + slot * 8, token).unwrap();
    let Progress::Created(key) = finish(worker, 7) else {
        panic!("creation failed")
    };
    key
}

#[test]
fn transient_preflight_and_announcement_misses_preserve_noncloneable_record() {
    let (catalog, budget) = setup(1);
    let mut worker = catalog.register_record_mutator().unwrap();
    let token = Token::new(91);
    let drops = token.drops.clone();
    let fixture = token.fixture.clone();
    worker.submit_record(0, 8, token).unwrap();
    for _ in 0..8 {
        FAIL_LOAD.with(|fail| fail.set(Some(0)));
        assert_eq!(worker.poll_in(&7), Progress::Pending);
        assert_eq!(drops.load(Ordering::Acquire), 0);
        assert!(
            matches!(&worker.state, RequestState::StartingRecord { record: Some(token), .. } if token.identity == 91)
        );
    }
    // Reach the private cell, then fail its owning share before announcement.
    assert_eq!(worker.poll_in(&7), Progress::Pending);
    assert_eq!(worker.poll_in(&7), Progress::Pending);
    FAIL_SHARE.with(|fail| fail.set(Some(0)));
    assert_eq!(worker.poll_in(&7), Progress::Pending);
    assert_eq!(worker.poll_in(&7), Progress::Pending);
    assert_eq!(drops.load(Ordering::Acquire), 0);
    let Progress::Created(key) = finish(&mut worker, 7) else {
        panic!("creation failed")
    };
    worker.submit(GuardedCommand::Publish(key)).unwrap();
    assert_eq!(finish(&mut worker, 7), Progress::Committed);
    assert_eq!(fixture.0.published.load(Ordering::Acquire), 1);
    drop(worker);
    drop(catalog);
    assert_eq!(drops.load(Ordering::Acquire), 1);
    assert_eq!(budget.used(), 0);
    fixture.release_origin();
}

#[test]
fn busy_returns_record_and_cancellation_drops_private_state_before_quota() {
    let (catalog, _) = setup(1);
    let mut worker = catalog.register_record_mutator().unwrap();
    let first = Token::new(12);
    let drops = first.drops.clone();
    worker.submit_record(0, 8, first).unwrap();
    let second = Token::new(13);
    let other_drops = second.drops.clone();
    let failure = worker.submit_record(1, 16, second).unwrap_err();
    assert_eq!(failure.error, RequestError::Busy);
    assert_eq!(failure.record.identity, 13);
    assert_eq!(other_drops.load(Ordering::Acquire), 0);
    drop(failure);
    assert_eq!(other_drops.load(Ordering::Acquire), 1);
    worker.lease.before_release = Some(Box::new(move || {
        assert_eq!(drops.load(Ordering::Acquire), 1)
    }));
    drop(worker);
    assert_eq!(catalog.registered(), (0, 0));
    assert!(catalog.register_record_mutator().is_ok());
}

#[test]
fn discovery_collision_destroys_only_losing_private_record() {
    let (catalog, _) = setup(2);
    let mut first = catalog.register_record_mutator().unwrap();
    let mut second = catalog.register_record_mutator().unwrap();
    let winner = Token::new(20);
    let loser = Token::new(21);
    let winner_drops = winner.drops.clone();
    let loser_drops = loser.drops.clone();
    first.submit_record(0, 8, winner).unwrap();
    second.submit_record(0, 8, loser).unwrap();
    // Both preflights finish before either discovery announcement.
    assert_eq!(first.poll_in(&7), Progress::Pending);
    assert_eq!(second.poll_in(&7), Progress::Pending);
    let Progress::Created(key) = finish(&mut first, 7) else {
        panic!("creation failed")
    };
    assert_eq!(
        finish(&mut second, 7),
        Progress::Rejected(RequestError::Conflict)
    );
    assert_eq!(loser_drops.load(Ordering::Acquire), 1);
    assert_eq!(winner_drops.load(Ordering::Acquire), 0);
    assert_eq!(second.discover(0).unwrap().unwrap().key, key);
    first.submit(GuardedCommand::Publish(key)).unwrap();
    assert_eq!(finish(&mut first, 7), Progress::Committed);
    drop(first);
    drop(second);
    drop(catalog);
    assert_eq!(winner_drops.load(Ordering::Acquire), 1);
}

#[test]
fn reserved_helper_finishes_stopped_guarded_publication() {
    let (catalog, _) = setup(1);
    let mut origin = catalog.register_record_mutator().unwrap();
    assert!(matches!(
        catalog.register_record_mutator(),
        Err(RegisterError::Limit)
    ));
    let mut helper = catalog.register_record_helper().unwrap();
    let token = Token::new(22);
    let fixture = token.fixture.clone();
    let key = create(&mut origin, 0, token);
    origin.submit(GuardedCommand::Publish(key)).unwrap();
    for _ in 0..4096 {
        assert_eq!(origin.poll_in(&7), Progress::Pending);
        if matches!(origin.state, RequestState::Attempt(..)) {
            break;
        }
    }
    let RequestState::Attempt(_, attempt) = &origin.state else {
        panic!("preparation failed")
    };
    assert!(catalog.raw.install_and_stop(attempt));
    fixture.0.contended.store(true, Ordering::Release);
    assert_eq!(helper.phase_in(&7), Err(RequestError::Contended));
    fixture.0.contended.store(false, Ordering::Release);
    assert_eq!(helper.phase_in(&7), Err(RequestError::Contended));
    assert_eq!(
        helper.phase_in(&7).unwrap().domains.map(|d| d.pending),
        [1, 1]
    );
    assert_eq!(fixture.0.published.load(Ordering::Acquire), 1);
    assert_eq!(finish(&mut origin, 7), Progress::Committed);
    assert!(matches!(
        helper.observe(key).unwrap().membership,
        Membership::Tracing(_)
    ));
    fixture.release_origin();
}

#[test]
fn closed_or_foreign_context_cancels_without_effects() {
    for closed in [false, true] {
        let (catalog, _) = setup(1);
        let mut worker = catalog.register_record_mutator().unwrap();
        let token = Token::new(23);
        let fixture = token.fixture.clone();
        let key = create(&mut worker, 0, token);
        if closed {
            fixture.release_origin();
        }
        worker.submit(GuardedCommand::Publish(key)).unwrap();
        assert_eq!(
            finish(&mut worker, if closed { 7 } else { 9 }),
            Progress::Cancelled
        );
        assert_eq!(
            worker.observe(key).unwrap().membership,
            Membership::Prepared
        );
        assert_eq!(
            worker.phase_in(&7).unwrap().domains.map(|d| d.pending),
            [0, 0]
        );
        assert_eq!(fixture.0.published.load(Ordering::Acquire), 0);
        if !closed {
            fixture.release_origin();
        }
    }
}

#[test]
fn actual_record_and_participant_layouts_are_charged_before_allocation() {
    let capacities = Capacities::derive(2, 2, 1).unwrap();
    let numeric = Footprint::for_capacities(capacities).unwrap();
    let record = Footprint::for_records::<Token>(capacities).unwrap();
    assert!(record.backing() > numeric.backing());
    assert!(record.participants() > numeric.participants());
    assert_eq!(
        record.participants(),
        capacities.participants()
            * std::mem::size_of::<Worker>().max(std::mem::size_of::<Participant<Token>>())
    );
    let budget = Budget::new(record.total() - 1);
    assert!(matches!(
        BoundedCatalog::<Token>::with_records_in(capacities, &budget),
        Err(ConfigError::BudgetExceeded)
    ));
    assert_eq!(budget.used(), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn stopped_private_record_destruction_retains_quota_while_helper_progresses() {
    use std::sync::mpsc;
    let (catalog, budget) = setup(1);
    let charge = budget.used();
    let mut origin = catalog.register_record_mutator().unwrap();
    let mut helper = catalog.register_record_helper().unwrap();
    let mut token = Token::new(24);
    let drops = token.drops.clone();
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    // Receiver is Send but not Sync; the callback is invoked once at drop.
    let release_rx = std::sync::Mutex::new(release_rx);
    token.on_drop = Some(Box::new(move || {
        entered_tx.send(()).unwrap();
        release_rx.lock().unwrap().recv().unwrap();
    }));
    origin.submit_record(0, 8, token).unwrap();
    let stopped = std::thread::spawn(move || drop(origin));
    entered_rx.recv().unwrap();
    assert_eq!(drops.load(Ordering::Acquire), 1);
    assert_eq!(catalog.registered(), (1, 1));
    assert_eq!(budget.used(), charge);
    assert!(matches!(
        catalog.register_record_mutator(),
        Err(RegisterError::Limit)
    ));
    let key = create(&mut helper, 1, Token::new(25));
    helper.submit(GuardedCommand::Publish(key)).unwrap();
    assert_eq!(finish(&mut helper, 7), Progress::Committed);
    release_tx.send(()).unwrap();
    stopped.join().unwrap();
    assert_eq!(catalog.registered(), (0, 1));
    drop(helper);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}
