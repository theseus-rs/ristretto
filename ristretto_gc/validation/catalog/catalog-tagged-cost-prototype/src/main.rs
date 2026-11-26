//! Allocation-count design screen, not a collector performance comparison.
#[cfg(feature = "counts")]
use std::alloc::{GlobalAlloc, Layout, System};
#[cfg(feature = "counts")]
use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(feature = "counts")]
struct Counting;
#[cfg(feature = "counts")]
static CALLS: AtomicU64 = AtomicU64::new(0);
#[cfg(feature = "counts")]
static BYTES: AtomicU64 = AtomicU64::new(0);
#[cfg(feature = "counts")]
static FREES: AtomicU64 = AtomicU64::new(0);
#[cfg(feature = "counts")]
static RELEASED: AtomicU64 = AtomicU64::new(0);
#[cfg(feature = "counts")]
fn allocated(pointer: *mut u8, bytes: usize) {
    if !pointer.is_null() {
        CALLS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(bytes as u64, Ordering::Relaxed);
    }
}
// SAFETY: forward the allocator's original pointer/layout/size, unchanged.
// Counter updates neither allocate nor change storage ownership or unwinding.
#[cfg(feature = "counts")]
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        allocated(pointer, layout.size());
        pointer
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc_zeroed(layout) };
        allocated(pointer, layout.size());
        pointer
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) };
        FREES.fetch_add(1, Ordering::Relaxed);
        RELEASED.fetch_add(layout.size() as u64, Ordering::Relaxed);
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, bytes: usize) -> *mut u8 {
        let result = unsafe { System.realloc(pointer, layout, bytes) };
        if !result.is_null() {
            FREES.fetch_add(1, Ordering::Relaxed);
            RELEASED.fetch_add(layout.size() as u64, Ordering::Relaxed);
            allocated(result, bytes);
        }
        result
    }
}
#[cfg(feature = "counts")]
#[global_allocator]
static ALLOCATOR: Counting = Counting;

#[derive(Clone, Copy, Default)]
struct Count {
    calls: u64,
    bytes: u64,
    frees: u64,
    released: u64,
}
#[cfg(feature = "counts")]
fn count() -> Count {
    Count {
        calls: CALLS.load(Ordering::Relaxed),
        bytes: BYTES.load(Ordering::Relaxed),
        frees: FREES.load(Ordering::Relaxed),
        released: RELEASED.load(Ordering::Relaxed),
    }
}
impl Count {
    fn add_since(&mut self, before: Self) {
        let after = count();
        self.calls += after.calls - before.calls;
        self.bytes += after.bytes - before.bytes;
        self.frees += after.frees - before.frees;
        self.released += after.released - before.released;
    }
}

const STAGES: [&str; 9] = [
    "record",
    "insert_prepare",
    "insert_commit",
    "acknowledge",
    "retire",
    "release",
    "cycle",
    "setup",
    "teardown",
];

#[cfg(not(feature = "counts"))]
fn count() -> Count {
    Count::default()
}

fn pooled_prepare(
    catalog: &pooled::Catalog,
    slot: usize,
    retries: &mut u64,
) -> std::sync::Arc<pooled::Cell> {
    for _ in 0..(catalog.capacity() + 256).next_power_of_two() {
        match catalog.prepare_cell(slot, (slot + 1) * 8) {
            Ok(cell) => return cell,
            Err(pooled::PrepareError::BackingUnavailable | pooled::PrepareError::Contended) => {
                *retries += 1
            }
            Err(error) => panic!("unexpected preparation error: {error:?}"),
        }
    }
    panic!("quiescent pool did not find a free record");
}
fn pooled_snapshot(catalog: &pooled::Catalog) -> pooled::Snapshot {
    catalog.snapshot().unwrap()
}
fn pooled_ack(catalog: &pooled::Catalog, cell: &std::sync::Arc<pooled::Cell>) {
    for domain in 0..2 {
        let pooled::Membership::Live(work) = cell.membership() else {
            panic!("missing membership")
        };
        let attempt = catalog
            .acknowledge(pooled_snapshot(catalog), cell.clone(), domain, work[domain])
            .unwrap();
        assert_eq!(catalog.commit(&attempt), pooled::Outcome::Committed);
    }
}
fn pooled_run(resident: usize, iterations: u64) -> ([Count; 9], u128, u64) {
    let setup = count();
    let catalog = pooled::Catalog::with_capacity(resident + 1);
    let mut retries = 0;
    for slot in 0..resident {
        let cell = pooled_prepare(&catalog, slot, &mut retries);
        assert_eq!(
            catalog.commit(
                &catalog
                    .insert(pooled_snapshot(&catalog), cell.clone())
                    .unwrap()
            ),
            pooled::Outcome::Committed
        );
        pooled_ack(&catalog, &cell);
    }
    let mut totals = [Count::default(); 9];
    totals[7].add_since(setup);
    retries = 0;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let cycle = count();
        let before = count();
        let cell = pooled_prepare(&catalog, resident, &mut retries);
        totals[0].add_since(before);
        let before = count();
        let attempt = catalog
            .insert(pooled_snapshot(&catalog), cell.clone())
            .unwrap();
        totals[1].add_since(before);
        let before = count();
        assert_eq!(catalog.commit(&attempt), pooled::Outcome::Committed);
        drop(attempt);
        totals[2].add_since(before);
        let before = count();
        pooled_ack(&catalog, &cell);
        totals[3].add_since(before);
        let before = count();
        assert_eq!(
            catalog.commit(
                &catalog
                    .retire(pooled_snapshot(&catalog), cell.clone())
                    .unwrap()
            ),
            pooled::Outcome::Committed
        );
        totals[4].add_since(before);
        let before = count();
        assert!(catalog.release_retired(&cell));
        drop(cell);
        totals[5].add_since(before);
        totals[6].add_since(cycle);
    }
    let nanos = start.elapsed().as_nanos();
    let teardown = count();
    assert_eq!(
        pooled_snapshot(&catalog)
            .phase()
            .domains
            .map(|domain| domain.pending),
        [0, 0]
    );
    assert!(catalog.discover(resident).is_none());
    for slot in 0..resident {
        let cell = catalog.discover(slot).unwrap();
        assert_eq!(
            catalog.commit(
                &catalog
                    .retire(pooled_snapshot(&catalog), cell.clone())
                    .unwrap()
            ),
            pooled::Outcome::Committed
        );
        assert!(catalog.release_retired(&cell));
    }
    drop(catalog);
    totals[8].add_since(teardown);
    (totals, nanos, retries)
}

fn tagged_prepare(
    catalog: &tagged::Catalog,
    slot: usize,
    retries: &mut u64,
) -> pool::Owned<tagged::Cell> {
    for _ in 0..(catalog.capacity() + 256).next_power_of_two() {
        match catalog.prepare_cell(slot, (slot + 1) * 8) {
            Ok(cell) => return cell,
            Err(tagged::PrepareError::BackingUnavailable | tagged::PrepareError::Contended) => {
                *retries += 1
            }
            Err(error) => panic!("unexpected preparation error: {error:?}"),
        }
    }
    panic!("quiescent pool did not find a free record");
}
fn tagged_snapshot(catalog: &tagged::Catalog) -> tagged::Snapshot {
    catalog.snapshot().unwrap()
}
fn tagged_ack(catalog: &tagged::Catalog, cell: &pool::Owned<tagged::Cell>) {
    for domain in 0..2 {
        let tagged::Membership::Live(work) = cell.membership().unwrap() else {
            panic!("missing membership")
        };
        let attempt = catalog
            .acknowledge(
                tagged_snapshot(catalog),
                cell.try_clone().unwrap(),
                domain,
                work[domain],
            )
            .unwrap();
        assert_eq!(catalog.commit(&attempt), tagged::Outcome::Committed);
    }
}
fn tagged_run(resident: usize, iterations: u64) -> ([Count; 9], u128, u64) {
    let setup = count();
    let limits = tagged::bounded::Capacities::derive(resident + 1, 1, 1).unwrap();
    let (controls, memberships, cells) = limits.pools();
    let catalog = tagged::Catalog::with_pool_capacities(resident + 1, controls, memberships, cells);
    let mut retries = 0;
    for slot in 0..resident {
        let cell = tagged_prepare(&catalog, slot, &mut retries);
        assert_eq!(
            catalog.commit(
                &catalog
                    .insert(tagged_snapshot(&catalog), cell.try_clone().unwrap())
                    .unwrap()
            ),
            tagged::Outcome::Committed
        );
        tagged_ack(&catalog, &cell);
    }
    let mut totals = [Count::default(); 9];
    totals[7].add_since(setup);
    retries = 0;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let cycle = count();
        let before = count();
        let cell = tagged_prepare(&catalog, resident, &mut retries);
        totals[0].add_since(before);
        let before = count();
        let attempt = catalog
            .insert(tagged_snapshot(&catalog), cell.try_clone().unwrap())
            .unwrap();
        totals[1].add_since(before);
        let before = count();
        assert_eq!(catalog.commit(&attempt), tagged::Outcome::Committed);
        drop(attempt);
        totals[2].add_since(before);
        let before = count();
        tagged_ack(&catalog, &cell);
        totals[3].add_since(before);
        let before = count();
        assert_eq!(
            catalog.commit(
                &catalog
                    .retire(tagged_snapshot(&catalog), cell.try_clone().unwrap())
                    .unwrap()
            ),
            tagged::Outcome::Committed
        );
        totals[4].add_since(before);
        let before = count();
        assert!(catalog.release_retired(&cell));
        drop(cell);
        totals[5].add_since(before);
        totals[6].add_since(cycle);
    }
    let nanos = start.elapsed().as_nanos();
    let teardown = count();
    assert_eq!(
        tagged_snapshot(&catalog)
            .phase()
            .domains
            .map(|domain| domain.pending),
        [0, 0]
    );
    assert!(catalog.discover(resident).unwrap().is_none());
    for slot in 0..resident {
        let cell = catalog.discover(slot).unwrap().unwrap();
        assert_eq!(
            catalog.commit(
                &catalog
                    .retire(tagged_snapshot(&catalog), cell.try_clone().unwrap())
                    .unwrap()
            ),
            tagged::Outcome::Committed
        );
        assert!(catalog.release_retired(&cell));
    }
    drop(catalog);
    totals[8].add_since(teardown);
    assert_eq!(
        totals[7].calls + totals[6].calls + totals[8].calls,
        totals[7].frees + totals[6].frees + totals[8].frees
    );
    assert_eq!(
        totals[7].bytes + totals[6].bytes + totals[8].bytes,
        totals[7].released + totals[6].released + totals[8].released
    );
    (totals, nanos, retries)
}

fn bounded_finish(p: &mut tagged::bounded::Participant, limit: usize) -> tagged::bounded::Progress {
    for _ in 0..limit {
        let outcome = p.poll();
        if outcome != tagged::bounded::Progress::Pending {
            return outcome;
        }
    }
    panic!("quiescent bounded request did not finish");
}
fn bounded_command(
    p: &mut tagged::bounded::Participant,
    command: tagged::bounded::Command,
    limit: usize,
) {
    p.submit(command).unwrap();
    assert_eq!(
        bounded_finish(p, limit),
        tagged::bounded::Progress::Committed
    );
}
fn bounded_create(
    p: &mut tagged::bounded::Participant,
    slot: usize,
    limit: usize,
) -> tagged::bounded::RecordKey {
    p.submit(tagged::bounded::Command::Create {
        slot,
        address: (slot + 1) * 8,
    })
    .unwrap();
    let tagged::bounded::Progress::Created(record) = bounded_finish(p, limit) else {
        panic!("creation failed")
    };
    record
}
fn bounded_ack(
    p: &mut tagged::bounded::Participant,
    key: tagged::bounded::RecordKey,
    limit: usize,
) {
    for domain in 0..2 {
        let tagged::Membership::Live(tickets) = p.observe(key).unwrap().membership else {
            panic!("missing membership")
        };
        bounded_command(
            p,
            tagged::bounded::Command::Acknowledge {
                key,
                domain,
                ticket: tickets[domain],
            },
            limit,
        );
    }
}
fn bounded_run(resident: usize, iterations: u64) -> ([Count; 9], u128, u64) {
    use tagged::bounded::{BoundedCatalog, Capacities, Command, Progress};
    let setup = count();
    let limits = Capacities::derive(resident + 1, 1, 1).unwrap();
    let (controls, memberships, cells) = limits.pools();
    let limit = 4 * (controls + memberships + cells) + 64;
    let catalog = BoundedCatalog::new(limits).unwrap();
    let mut p = catalog.register_mutator().unwrap();
    // Reserve the helper registration throughout the screen. It retains no raw
    // request, but its quota and pool reserve are present in both representations.
    let helper = catalog.register_helper().unwrap();
    for slot in 0..resident {
        let key = bounded_create(&mut p, slot, limit);
        bounded_command(&mut p, Command::Insert(key), limit);
        bounded_ack(&mut p, key, limit);
    }
    let mut totals = [Count::default(); 9];
    totals[7].add_since(setup);
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let cycle = count();
        let before = count();
        let key = bounded_create(&mut p, resident, limit);
        totals[0].add_since(before);
        let before = count();
        p.submit(Command::Insert(key)).unwrap();
        totals[1].add_since(before);
        let before = count();
        assert_eq!(bounded_finish(&mut p, limit), Progress::Committed);
        totals[2].add_since(before);
        let before = count();
        bounded_ack(&mut p, key, limit);
        totals[3].add_since(before);
        let before = count();
        bounded_command(&mut p, Command::Retire(key), limit);
        totals[4].add_since(before);
        let before = count();
        assert!(p.release_retired(key).unwrap());
        totals[5].add_since(before);
        totals[6].add_since(cycle);
    }
    let nanos = start.elapsed().as_nanos();
    let teardown = count();
    assert_eq!(
        p.phase().unwrap().domains.map(|domain| domain.pending),
        [0, 0]
    );
    assert!(p.discover(resident).unwrap().is_none());
    for slot in 0..resident {
        let key = p.discover(slot).unwrap().unwrap().key;
        bounded_command(&mut p, Command::Retire(key), limit);
        assert!(p.release_retired(key).unwrap());
    }
    drop((p, helper));
    assert_eq!(catalog.registered(), (0, 0));
    drop(catalog);
    totals[8].add_since(teardown);
    assert_eq!(
        totals[7].calls + totals[6].calls + totals[8].calls,
        totals[7].frees + totals[6].frees + totals[8].frees
    );
    assert_eq!(
        totals[7].bytes + totals[6].bytes + totals[8].bytes,
        totals[7].released + totals[6].released + totals[8].released
    );
    // Preparation retries are not comparable to mandatory cooperative polls.
    // This mode reports zero for that legacy column; only full lifecycle cost
    // and aggregate allocation counts are compared between these modes.
    (totals, nanos, 0)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mode = args.get(1).expect("pooled|tagged");
    let resident: usize = args.get(2).expect("resident records").parse().unwrap();
    let iterations: u64 = args.get(3).expect("iterations").parse().unwrap();
    assert!(resident > 0 && resident <= 65536 && iterations > 0 && iterations <= 1000000);
    let (counts, nanos, retries) = match mode.as_str() {
        "pooled" => pooled_run(resident, iterations),
        "tagged" => tagged_run(resident, iterations),
        "bounded" => bounded_run(resident, iterations),
        _ => panic!("unknown mode"),
    };
    let instrumented = cfg!(feature = "counts");
    println!(
        "mode,resident,iterations,instrumented,stage,allocations,requested_bytes,deallocations,released_bytes,cycle_nanoseconds,prepare_retries"
    );
    for (stage, count) in STAGES.into_iter().zip(counts) {
        println!(
            "{mode},{resident},{iterations},{instrumented},{stage},{},{},{},{},{nanos},{retries}",
            count.calls, count.bytes, count.frees, count.released
        );
    }
}
