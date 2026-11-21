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
    reason = "validated backing and private creation stages retain each required owner"
)]
use super::{
    Arc, AtomicU8, AtomicU64, AtomicUsize, Contended, Domain, DomainMode, OptionalLink, Ordering,
    Phase, Pool, Pooled, PooledLink, TraceState, allocate, share,
};
use crate::canonical_pool as pool;
use pool::AllocationError;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrepareError {
    InvalidSlot,
    InvalidAddress,
    Occupied,
    Contended,
    IdentityExhausted,
    BackingUnavailable,
}

/// Rejected creation retains the exact caller-owned record for retry or cancellation.
pub struct RecordFailure<R> {
    pub error: PrepareError,
    pub record: R,
}
impl<R> std::fmt::Debug for RecordFailure<R> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RecordFailure")
            .field("error", &self.error)
            .finish_non_exhaustive()
    }
}

/// Private cell creation. One selected backing slot is tried per poll. A cell
/// becomes discoverable only after both backing allocations have succeeded.
pub struct Creation<R: Record = ()> {
    record: Option<R>,
    owner: Arc<Owner>,
    slot: usize,
    scan_slot: usize,
    misses: usize,
    nominated: bool,
    address: usize,
    identity: u64,
    membership: Option<Pooled<Membership>>,
    private_cell: Option<Cell<R>>,
    cell: Option<Pooled<Cell<R>>>,
    finished: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CreationError {
    Foreign,
    Finished,
    Occupied,
    GenerationExhausted,
    FixedLocation,
}

pub(crate) fn following_slot(slot: usize, capacity: usize) -> usize {
    if slot + 1 == capacity { 0 } else { slot + 1 }
}

impl<R: Record> Creation<R> {
    pub fn poll(&mut self, catalog: &Catalog<R>) -> Result<Option<Pooled<Cell<R>>>, CreationError> {
        self.poll_at(catalog, false)
    }
    /// Retain this exact private record after a lost announcement and select the
    /// next discovery slot. Full arrays remain pending; no scan or collection is
    /// performed. A successful announcement permanently seals the location.
    pub fn poll_rotating(
        &mut self,
        catalog: &Catalog<R>,
    ) -> Result<Option<Pooled<Cell<R>>>, CreationError> {
        self.poll_at(catalog, true)
    }
    #[expect(
        clippy::indexing_slicing,
        reason = "preflight validates the slot and rotation remains inside catalog capacity"
    )]
    fn poll_at(
        &mut self,
        catalog: &Catalog<R>,
        rotate: bool,
    ) -> Result<Option<Pooled<Cell<R>>>, CreationError> {
        if !Arc::ptr_eq(&self.owner, &catalog.owner) {
            return Err(CreationError::Foreign);
        }
        if self.finished {
            return Err(CreationError::Finished);
        }
        if self.cell.is_none() && self.private_cell.is_none() && self.membership.is_none() {
            match catalog
                .memberships
                .try_allocate_status(Membership::Prepared)
            {
                Ok(value) => self.membership = Some(value),
                Err(failure) if failure.error == AllocationError::GenerationExhausted => {
                    return Err(CreationError::GenerationExhausted);
                }
                Err(_) => {}
            }
            return Ok(None);
        }
        if self.cell.is_none() {
            let cell = self.private_cell.take().unwrap_or_else(|| Cell {
                record: self.record.take().expect("private prepared record"),
                owner: self.owner.clone(),
                identity: self.identity,
                address: self.address,
                slot: AtomicUsize::new(self.slot),
                abandon_requested: AtomicU8::new(0),
                state: PooledLink::new(
                    catalog.memberships.clone(),
                    self.membership.take().expect("prepared membership"),
                ),
            });
            match catalog.cells.try_allocate_status(cell) {
                Ok(cell) => self.cell = Some(cell),
                // Retain the exact private owning link after a selected-slot
                // miss; rebuilding membership here can repeat fragmented misses.
                Err(failure) => {
                    self.private_cell = Some(failure.value);
                    if failure.error == AllocationError::GenerationExhausted {
                        return Err(CreationError::GenerationExhausted);
                    }
                }
            }
            return Ok(None);
        }
        let cell = self.cell.as_ref().expect("prepared cell");
        let Some(proposed) = share(cell) else {
            return Ok(None);
        };
        if (!rotate || !catalog.owner.vacancy.defer(self.slot, self.identity))
            && catalog.slots[self.slot].compare(None, Some(proposed))
        {
            if self.nominated {
                catalog.owner.vacancy.finish(self.identity);
                self.nominated = false;
            }
            self.finished = true;
            Ok(self.cell.take())
        } else if rotate {
            // Failed CAS never exposed this value. The temporary proposal owner
            // has been dropped and only this private Creation retains the cell.
            if self.nominated {
                self.owner.vacancy.missed(self.identity, self.slot);
            }
            self.misses = self.misses.wrapping_add(1);
            if self.misses.is_multiple_of(super::vacancy::PROBE_INTERVAL)
                && catalog.owner.vacancy.nominate(self.identity)
            {
                self.nominated = true;
            }
            self.slot =
                catalog.next_slot_after_conflict(self.slot, &mut self.scan_slot, self.identity);
            cell.slot.store(self.slot, Ordering::Relaxed);
            Ok(None)
        } else {
            self.finished = true;
            self.cell = None;
            Err(CreationError::Occupied)
        }
    }
}

impl<R: Record> Drop for Creation<R> {
    fn drop(&mut self) {
        if self.nominated {
            self.owner.vacancy.finish(self.identity);
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

impl<R: Record> Catalog<R> {
    pub fn begin_create_with(
        &self,
        slot: usize,
        address: usize,
        record: R,
    ) -> Result<Creation<R>, RecordFailure<R>> {
        self.prepare_private_creation(slot, address, record, true)
    }
    pub(crate) fn prepare_private_creation(
        &self,
        slot: usize,
        address: usize,
        record: R,
        require_vacancy: bool,
    ) -> Result<Creation<R>, RecordFailure<R>> {
        let preflight = || {
            let entry = self.slots.get(slot).ok_or(PrepareError::InvalidSlot)?;
            if address == 0 {
                return Err(PrepareError::InvalidAddress);
            }
            if require_vacancy && entry.load().map_err(|_| PrepareError::Contended)?.is_some() {
                return Err(PrepareError::Occupied);
            }
            let identity = self.next_identity.load(Ordering::Relaxed);
            let next = identity
                .checked_add(1)
                .ok_or(PrepareError::IdentityExhausted)?;
            self.next_identity
                .compare_exchange(identity, next, Ordering::Relaxed, Ordering::Relaxed)
                .map_err(|_| PrepareError::Contended)?;
            Ok(identity)
        };
        let identity = match preflight() {
            Ok(identity) => identity,
            Err(error) => return Err(RecordFailure { error, record }),
        };
        Ok(Creation {
            record: Some(record),
            owner: self.owner.clone(),
            slot,
            scan_slot: slot,
            misses: 0,
            nominated: false,
            address,
            identity,
            membership: None,
            private_cell: None,
            cell: None,
            finished: false,
        })
    }
    pub(crate) fn object_slot_occupied(&self, slot: usize) -> bool {
        self.slots.get(slot).is_some_and(OptionalLink::is_occupied)
    }
    pub(crate) fn next_slot_after_conflict(
        &self,
        slot: usize,
        scan: &mut usize,
        identity: u64,
    ) -> usize {
        if slot == *scan {
            let hint = self
                .owner
                .vacancy
                .slot_for(identity)
                .unwrap_or_else(|| self.released_slot.load(Ordering::Relaxed));
            if hint < self.capacity() && hint != slot {
                return hint;
            }
        }
        *scan = following_slot(*scan, self.capacity());
        *scan
    }
}

#[cfg(test)]
#[path = "backing_tests.rs"]
mod backing_tests;

#[cfg(test)]
#[path = "creation_tests.rs"]
mod creation_tests;
