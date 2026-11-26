//! Allocation-count design screen, not a collector performance comparison.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering};

struct Counting;
static CALLS: AtomicU64 = AtomicU64::new(0);
static BYTES: AtomicU64 = AtomicU64::new(0);
static FREES: AtomicU64 = AtomicU64::new(0);
static RELEASED: AtomicU64 = AtomicU64::new(0);
fn allocated(pointer: *mut u8, bytes: usize) {
    if !pointer.is_null() {
        CALLS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(bytes as u64, Ordering::Relaxed);
    }
}
// SAFETY: forward the allocator's original pointer/layout/size, unchanged.
// Counter updates neither allocate nor change storage ownership or unwinding.
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
#[global_allocator]
static ALLOCATOR: Counting = Counting;

#[derive(Clone, Copy, Default)]
struct Count {
    calls: u64,
    bytes: u64,
    frees: u64,
    released: u64,
}
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

const STAGES: [&str; 7] = [
    "record",
    "insert_prepare",
    "insert_commit",
    "acknowledge",
    "retire",
    "release",
    "cycle",
];

fn descriptor_snapshot(catalog: &descriptor::Catalog) -> descriptor::Snapshot {
    catalog.snapshot().unwrap()
}
fn descriptor_ack(catalog: &descriptor::Catalog, cell: &std::sync::Arc<descriptor::Cell>) {
    for domain in 0..2 {
        let descriptor::Membership::Live(work) = cell.membership() else {
            panic!("missing membership")
        };
        let attempt = catalog
            .acknowledge(
                descriptor_snapshot(catalog),
                cell.clone(),
                domain,
                work[domain],
            )
            .unwrap();
        assert_eq!(catalog.commit(&attempt), descriptor::Outcome::Committed);
    }
}
fn descriptor_run(resident: usize, iterations: u64) -> [Count; 7] {
    let catalog = descriptor::Catalog::with_capacity(resident + 1);
    for slot in 0..resident {
        let cell = catalog.prepare_cell(slot, (slot + 1) * 8).unwrap();
        assert_eq!(
            catalog.commit(
                &catalog
                    .insert(descriptor_snapshot(&catalog), cell.clone())
                    .unwrap()
            ),
            descriptor::Outcome::Committed
        );
        descriptor_ack(&catalog, &cell);
    }
    let mut totals = [Count::default(); 7];
    for _ in 0..iterations {
        let cycle = count();
        let before = count();
        let cell = catalog.prepare_cell(resident, (resident + 1) * 8).unwrap();
        totals[0].add_since(before);
        let before = count();
        let attempt = catalog
            .insert(descriptor_snapshot(&catalog), cell.clone())
            .unwrap();
        totals[1].add_since(before);
        let before = count();
        assert_eq!(catalog.commit(&attempt), descriptor::Outcome::Committed);
        drop(attempt);
        totals[2].add_since(before);
        let before = count();
        descriptor_ack(&catalog, &cell);
        totals[3].add_since(before);
        let before = count();
        assert_eq!(
            catalog.commit(
                &catalog
                    .retire(descriptor_snapshot(&catalog), cell.clone())
                    .unwrap()
            ),
            descriptor::Outcome::Committed
        );
        totals[4].add_since(before);
        let before = count();
        assert!(catalog.release_retired(&cell));
        drop(cell);
        totals[5].add_since(before);
        totals[6].add_since(cycle);
    }
    assert_eq!(
        descriptor_snapshot(&catalog)
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
                    .retire(descriptor_snapshot(&catalog), cell.clone())
                    .unwrap()
            ),
            descriptor::Outcome::Committed
        );
        assert!(catalog.release_retired(&cell));
    }
    totals
}

fn tree_ack(catalog: &tree::Catalog, identity: u64) {
    for domain in 0..2 {
        let snapshot = catalog.snapshot();
        let ticket = snapshot.entries.get(identity).unwrap().work[domain];
        assert_eq!(
            catalog
                .commit(&tree::Catalog::acknowledge(snapshot, identity, domain, ticket).unwrap()),
            tree::Outcome::Committed
        );
    }
}
fn tree_run(resident: usize, iterations: u64) -> [Count; 7] {
    let catalog = tree::Catalog::new();
    for index in 0..resident {
        let identity = ((index + 1) as u64) << 32;
        assert_eq!(
            catalog.commit(
                &tree::Catalog::insert(catalog.snapshot(), identity, (index + 1) * 8).unwrap()
            ),
            tree::Outcome::Committed
        );
        tree_ack(&catalog, identity);
    }
    let mut totals = [Count::default(); 7];
    for iteration in 0..iterations {
        let identity = (((resident / 2 + 1) as u64) << 32) + iteration + 1;
        let address = (resident + 1) * 8;
        let cycle = count();
        let before = count();
        let attempt = tree::Catalog::insert(catalog.snapshot(), identity, address).unwrap();
        totals[1].add_since(before);
        let before = count();
        assert_eq!(catalog.commit(&attempt), tree::Outcome::Committed);
        drop(attempt);
        totals[2].add_since(before);
        let before = count();
        tree_ack(&catalog, identity);
        totals[3].add_since(before);
        let before = count();
        assert_eq!(
            catalog.commit(&tree::Catalog::retire(catalog.snapshot(), identity, address).unwrap()),
            tree::Outcome::Committed
        );
        totals[4].add_since(before);
        totals[6].add_since(cycle);
    }
    assert_eq!(catalog.snapshot().entries.count(), resident);
    for domain in 0..2 {
        assert_eq!(catalog.snapshot().entries.pending_count(domain), 0);
    }
    for index in 0..resident {
        assert_eq!(
            catalog.commit(
                &tree::Catalog::retire(
                    catalog.snapshot(),
                    ((index + 1) as u64) << 32,
                    (index + 1) * 8
                )
                .unwrap()
            ),
            tree::Outcome::Committed
        );
    }
    assert_eq!(catalog.snapshot().entries.count(), 0);
    totals
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mode = args.get(1).expect("tree|descriptor");
    let resident: usize = args.get(2).expect("resident records").parse().unwrap();
    let iterations: u64 = args.get(3).expect("iterations").parse().unwrap();
    assert!(resident > 0 && resident <= 65536 && iterations > 0 && iterations <= 65536);
    let counts = match mode.as_str() {
        "tree" => tree_run(resident, iterations),
        "descriptor" => descriptor_run(resident, iterations),
        _ => panic!("unknown mode"),
    };
    println!(
        "mode,resident,iterations,stage,allocations,requested_bytes,deallocations,released_bytes"
    );
    for (stage, count) in STAGES.into_iter().zip(counts) {
        println!(
            "{mode},{resident},{iterations},{stage},{},{},{},{}",
            count.calls, count.bytes, count.frees, count.released
        );
    }
}
