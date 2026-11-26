use super::*;
use crate::record_fixture::{Fixture, Guard};
use crate::{Entered, FAIL_LOAD, FAIL_SHARE, RecordError};

struct Value {
    fixture: Fixture,
    on_drop: Option<Box<dyn FnOnce() + Send + Sync>>,
}
impl Value {
    fn new() -> Self {
        Self {
            fixture: Fixture::new(),
            on_drop: None,
        }
    }
}
impl Payload for Value {
    type Context = usize;
    type Guard<'a> = Guard<'a>;
    fn enter<'a>(&'a self, context: &'a usize) -> Result<Entered<'a, Guard<'a>>, RecordError> {
        self.fixture.enter(context)
    }
}
impl Drop for Value {
    fn drop(&mut self) {
        if let Some(hook) = self.on_drop.take() {
            hook();
        }
    }
}
type Worker = Participant<Value, GuardedCommand>;
fn setup(roots: usize, mutators: usize) -> (CatalogHandle<Value>, Arc<Budget>) {
    let capacities = Capacities::derive(3, mutators, 1)
        .unwrap()
        .with_roots(roots)
        .unwrap();
    let footprint = Footprint::for_records::<Value>(capacities).unwrap();
    let budget = Budget::new(footprint.total());
    let catalog = BoundedCatalog::with_records_in(capacities, &budget).unwrap();
    (catalog, budget)
}
fn finish(worker: &mut Worker) -> Progress {
    for _ in 0..4096 {
        match worker.poll_in(&7) {
            Progress::Pending => {}
            result => return result,
        }
    }
    panic!("root request did not settle");
}
fn create(worker: &mut Worker, slot: usize, value: Value) -> RecordKey {
    worker.submit_record(slot, (slot + 1) * 8, value).unwrap();
    let Progress::Created(key) = finish(worker) else {
        panic!("record creation failed");
    };
    key
}
fn root(worker: &mut Worker, slot: usize, target: RecordKey) -> RootHandle<Value> {
    worker.submit_root_creation(slot, target).unwrap();
    let Progress::RootCreated(key) = finish(worker) else {
        panic!("root creation failed");
    };
    let root = worker.take_root().expect("unique ready root");
    assert_eq!(root.key(), key);
    assert!(worker.take_root().is_none());
    root
}
fn run(worker: &mut Worker, command: GuardedCommand) -> Progress {
    worker.submit(command).unwrap();
    finish(worker)
}
fn roots(worker: &mut Worker, command: RootCommand) -> Progress {
    worker.submit_root(command).unwrap();
    finish(worker)
}

#[test]
fn unique_root_holds_quota_and_catalog_charge_after_all_participants_leave() {
    let (catalog, budget) = setup(1, 1);
    let mut worker = catalog.register_record_mutator().unwrap();
    let target = create(&mut worker, 0, Value::new());
    let root = root(&mut worker, 0, target);
    assert_eq!(root.observe().target, target);
    assert_eq!(catalog.reserved_root_tokens(), 1);
    assert_eq!(
        worker.submit_root_creation(0, target),
        Err(RequestError::RootLimit)
    );
    let charged = catalog.charged_bytes();
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), charged);
    drop(root);
    assert_eq!(budget.used(), 0);
}

#[test]
fn cancellation_releases_every_private_and_ready_root_state_in_order() {
    for steps in 0..=3 {
        let (catalog, budget) = setup(1, 1);
        let mut worker = catalog.register_record_mutator().unwrap();
        let target = create(&mut worker, 0, Value::new());
        worker.submit_root_creation(0, target).unwrap();
        for _ in 0..steps {
            worker.poll_in(&7);
        }
        assert_eq!(catalog.reserved_root_tokens(), 1);
        assert!(worker.cancel());
        assert_eq!(catalog.reserved_root_tokens(), 0);
        if let Some(observation) = worker.discover_root(0).unwrap() {
            assert_eq!(steps, 3);
            assert_eq!(observation.status, RootStatus::Prepared);
            assert!(observation.removal_requested);
            assert_eq!(
                roots(&mut worker, RootCommand::RemoveRequested(observation.key)),
                Progress::Committed
            );
            assert!(worker.release_retired_root(observation.key).unwrap());
        } else {
            assert!(steps < 3);
        }
        assert_eq!(catalog.raw.root_pool_occupancy(), 0);
        drop(worker);
        drop(catalog);
        assert_eq!(budget.used(), 0);
    }
}

#[test]
fn ready_root_is_busy_until_transfer_and_participant_drop_posts_intent() {
    let (catalog, _) = setup(1, 1);
    let mut worker = catalog.register_record_mutator().unwrap();
    let target = create(&mut worker, 0, Value::new());
    worker.submit_root_creation(0, target).unwrap();
    let Progress::RootCreated(key) = finish(&mut worker) else {
        panic!("root not ready");
    };
    assert_eq!(worker.poll_in(&7), Progress::RootCreated(key));
    assert_eq!(
        worker.submit_root(RootCommand::Publish(key)),
        Err(RequestError::Busy)
    );
    assert_eq!(worker.observe(target), Err(RequestError::Busy));
    drop(worker);
    assert_eq!(catalog.reserved_root_tokens(), 0);
    let mut helper = catalog.register_record_helper().unwrap();
    assert!(helper.observe_root(key).unwrap().removal_requested);
    assert_eq!(
        roots(&mut helper, RootCommand::RemoveRequested(key)),
        Progress::Committed
    );
    assert!(helper.release_retired_root(key).unwrap());
}

#[test]
fn transient_preflight_and_share_misses_retain_one_root_reservation() {
    let (catalog, _) = setup(1, 1);
    let mut worker = catalog.register_record_mutator().unwrap();
    let target = create(&mut worker, 0, Value::new());
    worker.submit_root_creation(0, target).unwrap();
    for _ in 0..8 {
        FAIL_LOAD.with(|fail| fail.set(Some(0)));
        assert_eq!(worker.poll_in(&7), Progress::Pending);
        assert_eq!(catalog.reserved_root_tokens(), 1);
    }
    assert_eq!(worker.poll_in(&7), Progress::Pending);
    assert_eq!(worker.poll_in(&7), Progress::Pending);
    for _ in 0..8 {
        FAIL_SHARE.with(|fail| fail.set(Some(0)));
        assert_eq!(worker.poll_in(&7), Progress::Pending);
        assert_eq!(catalog.reserved_root_tokens(), 1);
        assert!(catalog.raw.discover_root(0).unwrap().is_none());
    }
    assert!(matches!(finish(&mut worker), Progress::RootCreated(_)));
    drop(worker.take_root());
    assert_eq!(catalog.reserved_root_tokens(), 0);
}

#[test]
fn announcement_collision_refunds_only_the_losing_root_reservation() {
    let (catalog, _) = setup(2, 2);
    let mut first = catalog.register_record_mutator().unwrap();
    let mut second = catalog.register_record_mutator().unwrap();
    let target = create(&mut first, 0, Value::new());
    first.submit_root_creation(0, target).unwrap();
    second.submit_root_creation(0, target).unwrap();
    assert_eq!(first.poll_in(&7), Progress::Pending);
    assert_eq!(second.poll_in(&7), Progress::Pending);
    assert!(matches!(finish(&mut first), Progress::RootCreated(_)));
    assert_eq!(
        finish(&mut second),
        Progress::Rejected(RequestError::Conflict)
    );
    assert_eq!(catalog.reserved_root_tokens(), 1);
    let root = first.take_root().unwrap();
    assert!(!root.observe().removal_requested);
    assert_eq!(
        roots(&mut second, RootCommand::Publish(root.key())),
        Progress::Committed
    );
    assert_eq!(root.observe().status, RootStatus::Live);
}

#[test]
fn foreign_and_stale_root_keys_cannot_change_reused_slots() {
    let (catalog, _) = setup(1, 1);
    let (foreign, _) = setup(1, 1);
    let mut worker = catalog.register_record_mutator().unwrap();
    let mut other = foreign.register_record_mutator().unwrap();
    let target = create(&mut worker, 0, Value::new());
    let old = root(&mut worker, 0, target);
    let key = old.key();
    assert_eq!(
        other.submit_root(RootCommand::Cancel(key)),
        Err(RequestError::Foreign)
    );
    assert_eq!(
        other.submit_root_creation(0, target),
        Err(RequestError::Foreign)
    );
    assert_eq!(other.observe_root(key), Err(RequestError::Foreign));
    assert_eq!(
        roots(&mut worker, RootCommand::Cancel(key)),
        Progress::Committed
    );
    assert!(!worker.release_retired_root(key).unwrap());
    drop(old);
    assert!(worker.release_retired_root(key).unwrap());
    let new = root(&mut worker, 0, target);
    assert_ne!(key, new.key());
    assert_eq!(worker.observe_root(key), Err(RequestError::Stale));
    assert_eq!(
        roots(&mut worker, RootCommand::Publish(key)),
        Progress::Rejected(RequestError::Stale)
    );
    assert_eq!(
        roots(&mut worker, RootCommand::Publish(new.key())),
        Progress::Committed
    );
    assert_eq!(new.observe().status, RootStatus::Live);
}

#[test]
fn reserved_helper_finishes_installed_root_publication_and_drop_removal() {
    let (catalog, _) = setup(1, 1);
    let mut origin = catalog.register_record_mutator().unwrap();
    let mut helper = catalog.register_record_helper().unwrap();
    let target = create(&mut origin, 0, Value::new());
    let root = root(&mut origin, 0, target);
    origin
        .submit_root(RootCommand::Publish(root.key()))
        .unwrap();
    for _ in 0..4096 {
        assert_eq!(origin.poll_in(&7), Progress::Pending);
        if matches!(origin.state, RequestState::AttemptRoot(..)) {
            break;
        }
    }
    let RequestState::AttemptRoot(_, attempt) = &origin.state else {
        panic!("not prepared");
    };
    assert!(catalog.raw.install_and_stop(attempt));
    let phase = (0..16)
        .find_map(|_| helper.phase_in(&7).ok())
        .expect("helped phase");
    assert_eq!(phase.domains.map(|domain| domain.pending), [1, 1]);
    assert_eq!(root.observe().status, RootStatus::Live);
    let key = root.key();
    drop(root);
    assert_eq!(
        roots(&mut helper, RootCommand::RemoveRequested(key)),
        Progress::Committed
    );
    assert!(helper.release_retired_root(key).unwrap());
    assert_eq!(finish(&mut origin), Progress::Committed);
    assert_eq!(helper.observe_root(key), Err(RequestError::Stale));
}

#[test]
fn failed_scan_observation_cannot_skip_a_slot_and_incomplete_scan_rejects_advance() {
    let (catalog, _) = setup(1, 1);
    let mut worker = catalog.register_record_mutator().unwrap();
    assert_eq!(
        run(&mut worker, GuardedCommand::Advance { domain: 0 }),
        Progress::Rejected(RequestError::Conflict)
    );
    worker.submit_root(RootCommand::Scan { domain: 0 }).unwrap();
    // snapshot is the first owning load; root discovery is the second.
    FAIL_LOAD.with(|fail| fail.set(Some(1)));
    assert_eq!(worker.poll_in(&7), Progress::Pending);
    assert!(matches!(worker.state, RequestState::StartingRoot(_)));
    assert_eq!(
        catalog.raw.snapshot_in(&7).unwrap().phase().root_cursors,
        [0, 0]
    );
    assert_eq!(finish(&mut worker), Progress::Committed);
    assert_eq!(worker.phase_in(&7).unwrap().root_cursors, [1, 0]);
    assert_eq!(
        run(&mut worker, GuardedCommand::Advance { domain: 0 }),
        Progress::Committed
    );
    assert_eq!(
        roots(&mut worker, RootCommand::Scan { domain: 0 }),
        Progress::Rejected(RequestError::Conflict)
    );
    assert_eq!(
        roots(&mut worker, RootCommand::Scan { domain: 2 }),
        Progress::Rejected(RequestError::InvalidDomain)
    );
}

#[test]
fn repeated_cancel_and_object_slot_reuse_does_not_accumulate_root_backing() {
    let (catalog, _) = setup(1, 1);
    let mut worker = catalog.register_record_mutator().unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    for iteration in 0..64 {
        let count = drops.clone();
        let mut value = Value::new();
        value.on_drop = Some(Box::new(move || {
            count.fetch_add(1, Ordering::Release);
        }));
        let target = create(&mut worker, 0, value);
        let root = root(&mut worker, 0, target);
        let key = root.key();
        assert_eq!(
            roots(&mut worker, RootCommand::Cancel(key)),
            Progress::Committed
        );
        assert_eq!(
            run(&mut worker, GuardedCommand::Abandon(target)),
            Progress::Committed
        );
        assert!(worker.release_retired(target).unwrap());
        assert_eq!(drops.load(Ordering::Acquire), iteration);
        drop(root);
        assert!(worker.release_retired_root(key).unwrap());
        assert_eq!(catalog.raw.root_pool_occupancy(), 0);
        assert_eq!(drops.load(Ordering::Acquire), iteration + 1);
        assert_eq!(catalog.reserved_root_tokens(), 0);
    }
}

#[test]
fn root_capacity_derivation_is_checked_and_zero_roots_restore_original_pool_counts() {
    let ordinary = Capacities::derive(3, 2, 1)
        .unwrap()
        .with_generation_limit(17)
        .unwrap();
    let roots = ordinary.with_roots(2).unwrap();
    assert_eq!(roots.pools(), (32, 64, 32));
    assert_eq!(roots.root_pool(), 16);
    assert_eq!(roots.with_roots(0).unwrap(), ordinary);
    assert_eq!(
        ordinary.with_roots(usize::MAX),
        Err(ConfigError::CapacityOverflow)
    );
    assert_eq!(
        ordinary.with_roots(MAX_POOL),
        Err(ConfigError::CapacityOverflow)
    );
    let footprint = Footprint::for_records::<Value>(roots).unwrap();
    assert_eq!(
        footprint.root_tokens(),
        2 * std::mem::size_of::<RootHandle<Value>>()
    );
    assert_eq!(
        footprint.total(),
        footprint.backing() + footprint.participants() + footprint.root_tokens()
    );
}

#[test]
#[cfg(not(target_family = "wasm"))]
fn stalled_last_root_destructor_keeps_quota_and_budget_until_storage_is_destroyed() {
    use std::sync::Barrier;
    let (catalog, budget) = setup(1, 1);
    let mut worker = catalog.register_record_mutator().unwrap();
    let requested = Arc::new(Barrier::new(2));
    let release_slot = Arc::new(Barrier::new(2));
    let destroying = Arc::new(Barrier::new(2));
    let finish_destroy = Arc::new(Barrier::new(2));
    let mut value = Value::new();
    let reached = destroying.clone();
    let resume = finish_destroy.clone();
    value.on_drop = Some(Box::new(move || {
        reached.wait();
        resume.wait();
    }));
    let target = create(&mut worker, 0, value);
    let other = create(&mut worker, 1, Value::new());
    let mut root = root(&mut worker, 0, target);
    let key = root.key();
    assert_eq!(
        roots(&mut worker, RootCommand::Cancel(key)),
        Progress::Committed
    );
    assert_eq!(
        run(&mut worker, GuardedCommand::Abandon(target)),
        Progress::Committed
    );
    assert!(worker.release_retired(target).unwrap());
    let reached = requested.clone();
    let resume = release_slot.clone();
    root.token.after_drop_request(move || {
        reached.wait();
        resume.wait();
    });
    let charged = catalog.charged_bytes();
    let thread = std::thread::spawn(move || drop(root));
    requested.wait();
    assert_eq!(catalog.reserved_root_tokens(), 1);
    assert!(worker.release_retired_root(key).unwrap());
    release_slot.wait();
    destroying.wait();
    assert_eq!(catalog.reserved_root_tokens(), 1);
    assert_eq!(
        worker.submit_root_creation(0, other),
        Err(RequestError::RootLimit)
    );
    // Unrelated retirement still completes with this last-value destructor held.
    assert_eq!(
        run(&mut worker, GuardedCommand::Abandon(other)),
        Progress::Committed
    );
    assert!(worker.release_retired(other).unwrap());
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), charged);
    finish_destroy.wait();
    thread.join().unwrap();
    assert_eq!(budget.used(), 0);
}
