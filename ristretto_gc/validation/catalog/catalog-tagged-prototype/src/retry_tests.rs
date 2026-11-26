//! Forced private announcement collisions preserve unique ownership.
use crate::*;
struct Unique {
    drops: Arc<AtomicUsize>,
    number: usize,
}
impl Record for Unique {
    type Context = ();
    type Guard<'a> = ();
    fn enter<'a>(&'a self, context: &'a ()) -> Result<Entered<'a, ()>, RecordError> {
        ().enter(context)
    }
}
impl Drop for Unique {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}
fn setup() -> Catalog<Unique> {
    Catalog::with_record_root_capacities_and_limit(2, 128, 64, 64, 2, 64, pool::MAX_GENERATION)
}
fn begin(c: &Catalog<Unique>, drops: &Arc<AtomicUsize>, number: usize) -> Creation<Unique> {
    c.begin_create_with(
        0,
        number * 8,
        Unique {
            drops: drops.clone(),
            number,
        },
    )
    .unwrap()
}
fn finish(c: &Catalog<Unique>, creation: &mut Creation<Unique>) -> Pooled<Cell<Unique>> {
    (0..128)
        .find_map(|_| creation.poll_rotating(c).unwrap())
        .expect("creation settled")
}
#[test]
fn retry_rotating_cell_collision_keeps_unique_record_and_seals_announced_location() {
    let c = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    let mut first = begin(&c, &drops, 1);
    let mut second = begin(&c, &drops, 2);
    for creation in [&mut first, &mut second] {
        assert!(creation.poll_rotating(&c).unwrap().is_none());
        assert!(creation.poll_rotating(&c).unwrap().is_none());
    }
    let first_cell = finish(&c, &mut first);
    assert!(
        second.poll_rotating(&c).unwrap().is_none(),
        "forced losing CAS"
    );
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    assert!(c.discover(1).unwrap().is_none());
    let second_cell = second
        .poll_rotating(&c)
        .unwrap()
        .expect("same backing, next slot");
    assert_eq!(
        (
            second_cell.slot(),
            second_cell.address,
            second_cell.record().number
        ),
        (1, 16, 2)
    );
    assert_ne!(first_cell.identity, second_cell.identity);
    assert!(Pooled::ptr_eq(
        &second_cell,
        &c.discover(1).unwrap().unwrap()
    ));
    for creation in [&mut first, &mut second] {
        assert!(matches!(
            creation.poll_rotating(&c),
            Err(CreationError::Finished)
        ));
        assert!(matches!(creation.poll(&c), Err(CreationError::Finished)));
    }
    drop(c);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(first_cell);
    drop(second_cell);
    assert_eq!(drops.load(Ordering::Relaxed), 2);
}
#[test]
fn retry_full_rotating_creation_stays_pending_and_cancellation_drops_only_private_record() {
    let c = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    let mut waiting = begin(&c, &drops, 3);
    let first = finish(&c, &mut begin(&c, &drops, 1));
    let mut second = c
        .begin_create_with(
            1,
            16,
            Unique {
                drops: drops.clone(),
                number: 2,
            },
        )
        .unwrap();
    let second = finish(&c, &mut second);
    for _ in 0..32 {
        assert!(waiting.poll_rotating(&c).unwrap().is_none());
    }
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(waiting);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(c.discover(0).unwrap().unwrap().identity, first.identity);
    assert_eq!(c.discover(1).unwrap().unwrap().identity, second.identity);
    drop((c, first, second));
    assert_eq!(drops.load(Ordering::Relaxed), 3);
}
#[test]
fn retry_explicit_collision_remains_terminal_and_drops_private_record() {
    let c = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    let mut loser = begin(&c, &drops, 2);
    let winner = finish(&c, &mut begin(&c, &drops, 1));
    assert!(loser.poll(&c).unwrap().is_none());
    assert!(loser.poll(&c).unwrap().is_none());
    assert!(matches!(loser.poll(&c), Err(CreationError::Occupied)));
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert!(matches!(
        loser.poll_rotating(&c),
        Err(CreationError::Finished)
    ));
    drop((c, winner));
    assert_eq!(drops.load(Ordering::Relaxed), 2);
}
#[test]
fn retry_rotating_root_collision_retains_target_and_cannot_move_after_announcement() {
    let c = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    let cell = finish(&c, &mut begin(&c, &drops, 1));
    let mut first = c.begin_create_root(0, cell.try_clone().unwrap()).unwrap();
    let mut second = c.begin_create_root(0, cell.try_clone().unwrap()).unwrap();
    assert!(first.poll_rotating(&c).unwrap().is_none());
    assert!(second.poll_rotating(&c).unwrap().is_none());
    let first_root = first.poll_rotating(&c).unwrap().unwrap();
    assert!(
        second.poll_rotating(&c).unwrap().is_none(),
        "forced root collision"
    );
    let second_root = second
        .poll_rotating(&c)
        .unwrap()
        .expect("same root backing");
    assert_eq!((first_root.slot(), second_root.slot()), (0, 1));
    assert_eq!(second_root.target_identity(), cell.identity);
    assert_ne!(first_root.identity, second_root.identity);
    assert!(matches!(
        second.poll_rotating(&c),
        Err(CreationError::Finished)
    ));
    assert_eq!(
        c.discover_root(1).unwrap().unwrap().identity,
        second_root.identity
    );
    drop((c, cell, first_root));
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(second_root);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
}
#[test]
fn retry_foreign_rotating_poll_preserves_private_creation_for_its_catalog() {
    let c = setup();
    let other = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    let mut creation = begin(&c, &drops, 1);
    assert!(matches!(
        creation.poll_rotating(&other),
        Err(CreationError::Foreign)
    ));
    let cell = finish(&c, &mut creation);
    let mut root = c.begin_create_root(0, cell.try_clone().unwrap()).unwrap();
    assert!(matches!(
        root.poll_rotating(&other),
        Err(CreationError::Foreign)
    ));
    let root = (0..128)
        .find_map(|_| root.poll_rotating(&c).unwrap())
        .unwrap();
    assert_eq!(root.target_identity(), cell.identity);
    drop((c, cell, root));
    assert_eq!(drops.load(Ordering::Relaxed), 1);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn retry_concurrent_announcements_choose_distinct_object_and_root_locations() {
    let c = Arc::new(setup());
    let drops = Arc::new(AtomicUsize::new(0));
    let mut a = begin(&c, &drops, 1);
    let mut b = begin(&c, &drops, 2);
    for creation in [&mut a, &mut b] {
        assert!(creation.poll_rotating(&c).unwrap().is_none());
        assert!(creation.poll_rotating(&c).unwrap().is_none());
    }
    let barrier = std::sync::Barrier::new(2);
    let (a, b) = std::thread::scope(|scope| {
        let other = scope.spawn(|| {
            barrier.wait();
            finish(&c, &mut b)
        });
        barrier.wait();
        (finish(&c, &mut a), other.join().unwrap())
    });
    assert_ne!(a.slot(), b.slot());
    let mut ra = c.begin_create_root(0, a.try_clone().unwrap()).unwrap();
    let mut rb = c.begin_create_root(0, b.try_clone().unwrap()).unwrap();
    assert!(ra.poll_rotating(&c).unwrap().is_none());
    assert!(rb.poll_rotating(&c).unwrap().is_none());
    let (ra, rb) = std::thread::scope(|scope| {
        let other = scope.spawn(|| {
            barrier.wait();
            (0..128)
                .find_map(|_| rb.poll_rotating(&c).unwrap())
                .unwrap()
        });
        barrier.wait();
        let ra = (0..128)
            .find_map(|_| ra.poll_rotating(&c).unwrap())
            .unwrap();
        (ra, other.join().unwrap())
    });
    assert_ne!(ra.slot(), rb.slot());
    assert_eq!(ra.target_identity(), a.identity);
    assert_eq!(rb.target_identity(), b.identity);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop((c, a, b, ra, rb));
    assert_eq!(drops.load(Ordering::Relaxed), 2);
}
