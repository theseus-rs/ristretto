//! A discoverable bounded operation couples one cell effect and both phase domains.
//! Every cell mutation must go through this controller. A fixed-capacity registry
//! discovers prepared cells and durable work. Payload tracing, storage ownership,
//! conditional-reference decisions and actual collector integration remain unimplemented.
//! Tracing membership couples retention, births, generation and durable work.
#![expect(
    clippy::unreachable,
    reason = "private controller variants are inspected only after their phase certificate"
)]
#![expect(
    clippy::expect_used,
    reason = "fresh validated control backing has a free initial slot"
)]
use super::{
    Arc, AtomicU8, AtomicU64, AtomicUsize, Contended, Domain, DomainMode, OptionalLink, Ordering,
    Phase, Pool, Pooled, PooledLink, TraceState, allocate,
};
use crate::canonical_pool as pool;
const UNDECIDED: u8 = 0;
const COMMIT: u8 = 1;
const CANCEL: u8 = 2;

struct Owner {
    vacancy: super::vacancy::Preference,
}

/// Private graph admission errors. Closed is terminal; contention is retryable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordError {
    Closed,
    Contended,
    Foreign,
}

/// A helper retains this guard through the cell effect and durable certificate.
/// The optional latch informs resource cleanup only, never catalog membership.
pub struct Entered<'a, G> {
    _guard: G,
    published: Option<&'a AtomicU8>,
}
impl<'a, G> Entered<'a, G> {
    pub fn new(guard: G, published: &'a AtomicU8) -> Self {
        Self {
            _guard: guard,
            published: Some(published),
        }
    }
    fn certified(&self) {
        if let Some(latch) = self.published {
            latch.store(COMMIT, Ordering::Release);
        }
    }
}

/// Retained descriptor ownership. Runtime implementations must protect their
/// exact graph through Guard and permanently exclude admission after closure.
/// Each helper enters before its own decision attempt and effect; another helper
/// may already have decided COMMIT. No record callback runs inside the effect span.
/// This trait alone does not implement roots, traversal or reclamation.
pub trait Record {
    type Context: ?Sized;
    type Guard<'a>
    where
        Self: 'a;
    fn enter<'a>(
        &'a self,
        context: &'a Self::Context,
    ) -> Result<Entered<'a, Self::Guard<'a>>, RecordError>;
}
impl Record for () {
    type Context = ();
    type Guard<'a> = ();
    fn enter<'a>(&'a self, (): &'a ()) -> Result<Entered<'a, ()>, RecordError> {
        Ok(Entered {
            _guard: (),
            published: None,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Membership {
    Prepared,
    Live([u64; 2]),
    Tracing(TraceState),
    Retired,
}

pub struct Cell<R: Record = ()> {
    record: R,
    owner: Arc<Owner>,
    pub identity: u64,
    pub address: usize,
    slot: AtomicUsize,
    state: PooledLink<Membership>,
    abandon_requested: AtomicU8,
}
impl<R: Record> Cell<R> {
    /// Fixed after successful announcement. Only its private creator can rotate
    /// this location before any owning observation can discover the cell.
    pub fn slot(&self) -> usize {
        self.slot.load(Ordering::Relaxed)
    }

    /// Durable intent from a cancelled rooted-allocation owner. Helpers may
    /// abandon only Prepared membership through the canonical controller.
    pub fn abandon_requested(&self) -> bool {
        self.abandon_requested.load(Ordering::Acquire) != 0
    }
    pub(crate) fn request_abandon(&self) {
        self.abandon_requested.store(1, Ordering::Release);
    }
    pub fn record(&self) -> &R {
        &self.record
    }
    pub fn membership(&self) -> Option<Membership> {
        self.state.load().map(|state| *state)
    }
}

struct Effect<R: Record = ()> {
    cell: Pooled<Cell<R>>,
    expected: Pooled<Membership>,
    proposed: Pooled<Membership>,
}
struct Operation<R: Record = ()> {
    effect: Option<Effect<R>>,

    successor: Pooled<Control<R>>,
    decision: AtomicU8,
}
impl<R: Record> Operation<R> {
    // Both public outcomes share one private successor. Its decision is the
    // completion certificate, published only after all chosen effects finish.
    fn outcome(&self) -> &PhaseOperation {
        let Control::Phase(outcome) = &*self.successor else {
            unreachable!("effect successor holds both prepared phases")
        };
        outcome
    }
}
// No external effects or predecessor ownership: selecting an outcome also
// completes this phase change. The installed node can remain the stable head.
struct PhaseOperation {
    committed: Phase,
    cancelled: Phase,
    decision: AtomicU8,
}
enum Control<R: Record = ()> {
    Stable(Phase),
    Busy(Operation<R>),
    Phase(PhaseOperation),
}
impl<R: Record> Control<R> {
    fn is_stable(&self) -> bool {
        match self {
            Self::Stable(_) => true,
            Self::Busy(_) => false,
            Self::Phase(operation) => operation.decision.load(Ordering::Acquire) != UNDECIDED,
        }
    }
    fn phase(&self) -> Phase {
        match self {
            Control::Stable(phase) => *phase,
            Control::Phase(operation) => match operation.decision.load(Ordering::Acquire) {
                COMMIT => operation.committed,
                CANCEL => operation.cancelled,
                _ => unreachable!("undecided phase snapshot"),
            },
            Control::Busy(_) => unreachable!("effect-bearing snapshot"),
        }
    }
}

pub struct Catalog<R: Record = ()> {
    owner: Arc<Owner>,
    head: PooledLink<Control<R>>,
    controls: Arc<Pool<Control<R>>>,
    memberships: Arc<Pool<Membership>>,
    slots: Box<[OptionalLink<Cell<R>>]>,
    cells: Arc<Pool<Cell<R>>>,
    next_identity: AtomicU64,
    released_slot: AtomicUsize,
}
impl<R: Record> Catalog<R> {
    /// Requested heap backing; the inline Catalog value belongs to its caller.
    /// Arc headers use the pool's frozen-toolchain layout calculation.
    pub fn record_backing_bytes(
        capacity: usize,
        controls: usize,
        memberships: usize,
        cells: usize,
    ) -> Option<usize> {
        if capacity == 0 {
            return None;
        }
        std::alloc::Layout::array::<OptionalLink<Cell<R>>>(capacity)
            .ok()?
            .size()
            .checked_add(pool::arc_requested_bytes::<Owner>()?)?
            .checked_add(Pool::<Control<R>>::requested_backing_bytes(controls)?)?
            .checked_add(Pool::<Membership>::requested_backing_bytes(memberships)?)?
            .checked_add(Pool::<Cell<R>>::requested_backing_bytes(cells)?)
    }
    pub fn with_record_capacities_and_limit(
        capacity: usize,
        control_capacity: usize,
        state_capacity: usize,
        cell_capacity: usize,
        generation_limit: u64,
    ) -> Self {
        assert!(capacity > 0);
        let controls = Pool::with_generation_limit(control_capacity, generation_limit);
        let memberships = Pool::with_generation_limit(state_capacity, generation_limit);
        let cells = Pool::with_generation_limit(cell_capacity, generation_limit);
        let phase = Phase {
            root_cursors: [0; 2],
            reference_cursors: [0; 2],
            revision: 1,
            domains: [Domain {
                mode: DomainMode::Full,
                epoch: 1,
                floor: 0,
                pending: 0,
            }; 2],
        };
        let head = PooledLink::new(
            controls.clone(),
            allocate(&controls, Control::Stable(phase)).expect("fresh control pool"),
        );
        Self {
            owner: Arc::new(Owner {
                vacancy: super::vacancy::Preference::default(),
            }),
            head,
            controls,
            memberships,
            slots: (0..capacity)
                .map(|_| OptionalLink::empty(cells.clone()))
                .collect(),
            cells,
            next_identity: AtomicU64::new(1),
            released_slot: AtomicUsize::new(usize::MAX),
        }
    }
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }
    pub fn generation_exhaustion_observed(&self) -> bool {
        self.controls.generation_exhaustion_observed()
            || self.memberships.generation_exhaustion_observed()
            || self.cells.generation_exhaustion_observed()
    }
    pub fn discover(&self, slot: usize) -> Result<Option<Pooled<Cell<R>>>, Contended> {
        self.slots.get(slot).map_or(Ok(None), OptionalLink::load)
    }
}

#[cfg(test)]
#[path = "backing_tests.rs"]
mod backing_tests;
