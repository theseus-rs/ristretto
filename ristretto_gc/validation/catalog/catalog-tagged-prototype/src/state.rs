//! A discoverable bounded operation couples one cell effect and both phase domains.
//! Every cell mutation must go through this controller. A fixed-capacity registry
//! discovers prepared cells and durable work. Payload tracing, storage ownership,
//! conditional-reference decisions and actual collector integration remain unimplemented.
//! Tracing membership couples retention, births, generation and durable work.
use super::{
    Arc, AtomicU8, AtomicU64, AtomicUsize, Contended, DomainMode, OptionalLink, Ordering, Pool,
    Pooled, PooledLink, RETIREMENT_FLOOR, TraceRetention, TraceState, allocate, share,
};
use pool::AllocationError;
#[path = "catalog_roots.rs"]
mod roots;
pub(crate) use roots::root_barrier_plan;
pub use roots::{ReferenceRootKind, Root, RootCreation, RootPrepareError, RootStatus, RootToken};
use roots::{RootEffect, RootRegistry};
#[path = "catalog_references.rs"]
mod references;
pub use references::{
    ReferenceCreation, ReferenceMembership, ReferenceNotification, ReferencePrepareError,
    ReferenceScan, ReferenceSlot, ReferenceToken,
};
use references::{ReferenceEffect, ReferenceRegistry};
#[path = "catalog_edges.rs"]
mod edges;
pub use edges::{EdgeCreation, EdgeMembership, EdgePrepareError, EdgeSlot, EdgeState, EdgeToken};
use edges::{EdgeEffect, EdgeRegistry};
#[path = "catalog_edge_traversal.rs"]
mod edge_traversal;
pub use edge_traversal::{EdgeTraversal, EdgeTraversalError, EdgeTraversalProgress};

const UNDECIDED: u8 = 0;
const COMMIT: u8 = 1;
const CANCEL: u8 = 2;

struct Owner {
    vacancy: crate::vacancy::Preference,
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
    fn enter<'a>(&'a self, _: &'a ()) -> Result<Entered<'a, ()>, RecordError> {
        Ok(Entered {
            _guard: (),
            published: None,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Domain {
    pub mode: DomainMode,
    pub epoch: u64,
    pub floor: u8,
    pub pending: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase {
    /// Per-domain root scan cursor; only a canonical scan operation advances it.
    pub root_cursors: [usize; 2],
    /// Per-domain cursor for the current conditional-reference stage.
    pub reference_cursors: [usize; 2],
    pub revision: u64,
    pub domains: [Domain; 2],
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
    secondary_effect: Option<Effect<R>>,
    root: Option<RootEffect<R>>,
    secondary_root: Option<RootEffect<R>>,
    reference: Option<ReferenceEffect<R>>,
    edge: Option<EdgeEffect>,
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
#[allow(
    clippy::large_enum_variant,
    reason = "controller backing is prepaid; boxing would allocate in bounded polls"
)]
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

pub struct Snapshot<R: Record = ()> {
    owner: Arc<Owner>,
    head: Pooled<Control<R>>,
}
impl<R: Record> Snapshot<R> {
    pub fn phase(&self) -> Phase {
        self.head.phase()
    }
}
pub struct Attempt<R: Record = ()> {
    owner: Arc<Owner>,
    expected: Pooled<Control<R>>,
    busy: Pooled<Control<R>>,
}

/// Private, resumable backing preparation. Dropping this value cancels only the
/// private proposal; no controller or membership effect has been installed.
pub struct Preparation<R: Record = ()> {
    owner: Arc<Owner>,
    expected: Option<Pooled<Control<R>>>,
    committed_phase: Phase,
    cancelled_phase: Phase,
    effect: Option<Effect<R>>,
    secondary_effect: Option<Effect<R>>,
    root: Option<RootEffect<R>>,
    secondary_root: Option<RootEffect<R>>,
    reference: Option<ReferenceEffect<R>>,
    edge: Option<EdgeEffect>,
    successor: Option<Pooled<Control<R>>>,
    operation: Option<Operation<R>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreparationError {
    Foreign,
    Finished,
    GenerationExhausted,
}

impl<R: Record> Preparation<R> {
    /// At most one selected-slot allocation attempt. Partial successes remain
    /// owned here across polls, so a later miss cannot repeatedly erase them.
    pub fn poll(&mut self, catalog: &Catalog<R>) -> Result<Option<Attempt<R>>, PreparationError> {
        if !Arc::ptr_eq(&self.owner, &catalog.owner) {
            return Err(PreparationError::Foreign);
        }
        if self.expected.is_none() {
            return Err(PreparationError::Finished);
        }
        if self.successor.is_none() && self.operation.is_none() {
            let outcome = PhaseOperation {
                committed: self.committed_phase,
                cancelled: self.cancelled_phase,
                decision: AtomicU8::new(UNDECIDED),
            };
            let successor = match catalog
                .controls
                .try_allocate_status(Control::Phase(outcome))
            {
                Ok(value) => value,
                Err(failure) if failure.error == AllocationError::GenerationExhausted => {
                    return Err(PreparationError::GenerationExhausted);
                }
                Err(_) => return Ok(None),
            };
            if self.effect.is_none()
                && self.secondary_effect.is_none()
                && self.root.is_none()
                && self.secondary_root.is_none()
                && self.reference.is_none()
                && self.edge.is_none()
            {
                return Ok(Some(Attempt {
                    owner: self.owner.clone(),
                    expected: self.expected.take().expect("unconsumed phase preparation"),
                    busy: successor,
                }));
            }
            self.successor = Some(successor);
            return Ok(None);
        }
        let operation = self.operation.take().unwrap_or_else(|| Operation {
            effect: self.effect.take(),
            secondary_effect: self.secondary_effect.take(),
            root: self.root.take(),
            secondary_root: self.secondary_root.take(),
            reference: self.reference.take(),
            edge: self.edge.take(),
            successor: self.successor.take().expect("prepared phase outcomes"),
            decision: AtomicU8::new(UNDECIDED),
        });
        match catalog
            .controls
            .try_allocate_status(Control::Busy(operation))
        {
            Ok(busy) => Ok(Some(Attempt {
                owner: self.owner.clone(),
                expected: self.expected.take().expect("unconsumed preparation"),
                busy,
            })),
            Err(failure) => {
                let Control::Busy(operation) = failure.value else {
                    unreachable!("allocation returned a different proposal")
                };
                self.operation = Some(operation);
                if failure.error == AllocationError::GenerationExhausted {
                    Err(PreparationError::GenerationExhausted)
                } else {
                    Ok(None)
                }
            }
        }
    }
}
impl<R: Record> Attempt<R> {
    pub(crate) fn is_phase_only(&self) -> bool {
        matches!(*self.busy, Control::Phase(_))
    }
    #[cfg(test)]
    fn operation(&self) -> &Operation<R> {
        let Control::Busy(operation) = &*self.busy else {
            unreachable!()
        };
        operation
    }
    fn decision(&self) -> &AtomicU8 {
        match &*self.busy {
            Control::Busy(operation) => &operation.decision,
            Control::Phase(operation) => &operation.decision,
            Control::Stable(_) => unreachable!("stable proposal"),
        }
    }
    fn completed(&self) -> u8 {
        match &*self.busy {
            Control::Busy(operation) => operation.outcome().decision.load(Ordering::Acquire),
            Control::Phase(operation) => operation.decision.load(Ordering::Acquire),
            Control::Stable(_) => unreachable!("stable proposal"),
        }
    }
    /// The immutable decision is shared by all helpers. Cancellation does not
    /// release an already installed controller; a subsequent helper does that.
    pub fn cancel(&self) -> bool {
        match self.decision().compare_exchange(
            UNDECIDED,
            CANCEL,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => true,
            Err(value) => value == CANCEL,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Outcome {
    Committed,
    Cancelled,
    /// This exact request may still commit. Retain and poll it, or cancel it;
    /// preparing a replacement here can lose the original completion result.
    Pending,
    /// The expected controller was superseded without installing this request.
    Retry,
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
            if self.misses.is_multiple_of(crate::vacancy::PROBE_INTERVAL)
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
    roots: Option<RootRegistry<R>>,
    references: Option<ReferenceRegistry<R>>,
    edges: Option<EdgeRegistry>,
}
impl Default for Catalog {
    fn default() -> Self {
        Self::new()
    }
}
impl Catalog<()> {
    pub fn requested_backing_bytes(
        capacity: usize,
        controls: usize,
        memberships: usize,
        cells: usize,
    ) -> Option<usize> {
        Self::record_backing_bytes(capacity, controls, memberships, cells)
    }
    pub fn new() -> Self {
        Self::with_capacity(8)
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_backing_capacity(
            capacity,
            256,
            // Experimental reserve, not a production participant/budget policy.
            // Runtime integration must bound each participant's retained state
            // and reserve independent progress capacity before admitting it.
            capacity.checked_add(256).unwrap().next_power_of_two(),
        )
    }
    pub fn with_backing_capacity(
        capacity: usize,
        control_capacity: usize,
        state_capacity: usize,
    ) -> Self {
        Self::with_pool_capacities(
            capacity,
            control_capacity,
            state_capacity,
            capacity.checked_add(256).unwrap().next_power_of_two(),
        )
    }
    pub fn with_pool_capacities(
        capacity: usize,
        control_capacity: usize,
        state_capacity: usize,
        cell_capacity: usize,
    ) -> Self {
        Self::with_pool_capacities_and_limit(
            capacity,
            control_capacity,
            state_capacity,
            cell_capacity,
            pool::MAX_GENERATION,
        )
    }
    pub fn with_pool_capacities_and_limit(
        capacity: usize,
        control_capacity: usize,
        state_capacity: usize,
        cell_capacity: usize,
        generation_limit: u64,
    ) -> Self {
        Self::with_record_capacities_and_limit(
            capacity,
            control_capacity,
            state_capacity,
            cell_capacity,
            generation_limit,
        )
    }
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
                vacancy: crate::vacancy::Preference::default(),
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
            roots: None,
            references: None,
            edges: None,
        }
    }
    /// Retain an opaque record before slot announcement. A rejected preflight
    /// returns that exact value; no phase reservation survives failure. Record
    /// validity, payload uniqueness and graph ownership remain caller obligations.
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

    /// Try a recently freed object slot between sequential probes. Even if
    /// releases continuously change the hint, at least every second collision
    /// advances the scan. A hint conveys no ownership or publication rights.
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

    pub fn capacity(&self) -> usize {
        self.slots.len()
    }
    pub fn generation_exhaustion_observed(&self) -> bool {
        self.controls.generation_exhaustion_observed()
            || self.memberships.generation_exhaustion_observed()
            || self.cells.generation_exhaustion_observed()
            || self
                .roots
                .as_ref()
                .is_some_and(|roots| roots.pool.generation_exhaustion_observed())
            || self.references.as_ref().is_some_and(|refs| {
                refs.slots_pool.generation_exhaustion_observed()
                    || refs.states.generation_exhaustion_observed()
            })
            || self.edges.as_ref().is_some_and(|edges| {
                edges.slots_pool.generation_exhaustion_observed()
                    || edges.states.generation_exhaustion_observed()
            })
    }
    /// One owning slot observation; a scan cursor visits one slot per poll.
    pub fn discover(&self, slot: usize) -> Result<Option<Pooled<Cell<R>>>, Contended> {
        self.slots.get(slot).map_or(Ok(None), OptionalLink::load)
    }
    /// Terminal membership is irreversible; cleanup never authorizes a payload
    /// destructor, and compares the exact old owner so it cannot clear slot reuse.
    pub fn release_retired(&self, cell: &Pooled<Cell<R>>) -> bool {
        if !Arc::ptr_eq(&cell.owner, &self.owner) || cell.membership() != Some(Membership::Retired)
        {
            return false;
        }
        let slot = cell.slot();
        let released = self.slots[slot].compare(Some(cell), None);
        if released {
            self.owner.vacancy.offer(slot);
            self.released_slot.store(slot, Ordering::Relaxed);
        }
        released
    }
    /// Insertion cancellation is a canonical Prepared -> Retired operation.
    /// Attempt::cancel only cancels that particular immutable proposal.
    pub fn abandon(&self, snapshot: Snapshot<R>, cell: Pooled<Cell<R>>) -> Option<Attempt<R>> {
        let preparation = self.begin_abandon(snapshot, cell)?;
        self.prepare_now(preparation)
    }

    pub fn begin_abandon(
        &self,
        snapshot: Snapshot<R>,
        cell: Pooled<Cell<R>>,
    ) -> Option<Preparation<R>> {
        let expected = cell.state.load()?;
        if *expected != Membership::Prepared {
            return None;
        }
        let phase = snapshot.phase();
        let effect = self.effect(cell, expected, Membership::Retired)?;
        self.begin_prepare(snapshot, phase, Some(effect))
    }
    /// Copies possible phase outcomes without deciding or helping an operation.
    /// The one temporary owner is released before return; its Drop can destroy
    /// otherwise unowned records. These copies are not linearizable snapshots:
    /// both proposed outcomes can have a revision newer than the predecessor.
    pub(crate) fn possible_phase_outcomes(&self) -> Option<[Phase; 2]> {
        let head = self.head.load()?;
        #[cfg(test)]
        observation_tests::after_acquire();
        match &*head {
            Control::Stable(phase) => Some([*phase; 2]),
            Control::Phase(operation) => match operation.decision.load(Ordering::Acquire) {
                COMMIT => Some([operation.committed; 2]),
                CANCEL => Some([operation.cancelled; 2]),
                UNDECIDED => Some([operation.cancelled, operation.committed]),
                _ => None,
            },
            Control::Busy(operation) => {
                // Until effects are certified either immutable outcome is
                // a conservative observation, including after a decision.
                let outcome = operation.outcome();
                Some([outcome.cancelled, outcome.committed])
            }
        }
    }
    /// A single bounded help attempt. Seeing Busy cannot yield a phase snapshot.
    fn stable_head_in(&self, context: &R::Context) -> Option<Pooled<Control<R>>> {
        let head = self.head.load()?;
        if !head.is_stable() {
            self.finish(&head, context);
            None
        } else {
            Some(head)
        }
    }

    pub fn snapshot_in(&self, context: &R::Context) -> Option<Snapshot<R>> {
        Some(Snapshot {
            head: self.stable_head_in(context)?,
            owner: self.owner.clone(),
        })
    }

    pub(crate) fn phase_in(&self, context: &R::Context) -> Option<Phase> {
        Some(self.stable_head_in(context)?.phase())
    }

    // A retained stable controller's generation cannot be recycled. If it
    // changed, the ordinary observation still helps an installed operation
    // and checks the revision before accepting the read.
    fn validates_head_in(&self, head: &Pooled<Control<R>>, context: &R::Context) -> bool {
        self.head.points_to(head)
            || self
                .phase_in(context)
                .is_some_and(|current| current.revision == head.phase().revision)
    }

    pub(crate) fn validates_snapshot_in(
        &self,
        snapshot: &Snapshot<R>,
        context: &R::Context,
    ) -> bool {
        Arc::ptr_eq(&snapshot.owner, &self.owner) && self.validates_head_in(&snapshot.head, context)
    }
    fn begin_prepare(
        &self,
        snapshot: Snapshot<R>,
        mut phase: Phase,
        effect: Option<Effect<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&snapshot.owner, &self.owner) {
            return None;
        }
        let mut cancelled = snapshot.phase();
        cancelled.revision = cancelled.revision.checked_add(1)?;
        phase.revision = cancelled.revision;
        Some(Preparation {
            owner: snapshot.owner,
            expected: Some(snapshot.head),
            committed_phase: phase,
            cancelled_phase: cancelled,
            effect,
            root: None,
            secondary_effect: None,
            secondary_root: None,
            reference: None,
            edge: None,
            successor: None,
            operation: None,
        })
    }

    // Compatibility convenience: a finite attempt, not a progress guarantee.
    // Progress-sensitive callers must retain Preparation across their polls.
    fn prepare_now(&self, mut preparation: Preparation<R>) -> Option<Attempt<R>> {
        for _ in 0..24 {
            match preparation.poll(self) {
                Ok(Some(attempt)) => return Some(attempt),
                Ok(None) => {}
                Err(PreparationError::GenerationExhausted) => return None,
                Err(_) => unreachable!("same unfinished preparation"),
            }
        }
        None
    }
    fn effect(
        &self,
        cell: Pooled<Cell<R>>,
        expected: Pooled<Membership>,
        proposed: Membership,
    ) -> Option<Effect<R>> {
        if !Arc::ptr_eq(&cell.owner, &self.owner) {
            return None;
        }
        Some(Effect {
            cell,
            expected,
            proposed: allocate(&self.memberships, proposed)?,
        })
    }

    /// Initialized numeric allocation state, with both domain effects coupled.
    /// Payload graph/root/reference ownership is still the runtime's obligation.
    pub fn begin_publish(
        &self,
        snapshot: Snapshot<R>,
        cell: Pooled<Cell<R>>,
    ) -> Option<Preparation<R>> {
        let expected = cell.state.load()?;
        if *expected != Membership::Prepared {
            return None;
        }
        let (marks, phase) = TraceState::publication(snapshot.phase()).ok()?;
        let effect = self.effect(cell, expected, Membership::Tracing(marks))?;
        self.begin_prepare(snapshot, phase, Some(effect))
    }

    pub fn begin_shade(
        &self,
        snapshot: Snapshot<R>,
        cell: Pooled<Cell<R>>,
        domain: usize,
        requested: TraceRetention,
    ) -> Option<Preparation<R>> {
        let expected = cell.state.load()?;
        let Membership::Tracing(marks) = *expected else {
            return None;
        };
        let (marks, phase) = marks.shade(snapshot.phase(), domain, requested).ok()?;
        let effect = self.effect(cell, expected, Membership::Tracing(marks))?;
        self.begin_prepare(snapshot, phase, Some(effect))
    }

    pub fn begin_trace_acknowledge(
        &self,
        snapshot: Snapshot<R>,
        cell: Pooled<Cell<R>>,
        domain: usize,
        ticket: u64,
    ) -> Option<Preparation<R>> {
        let expected = cell.state.load()?;
        let Membership::Tracing(marks) = *expected else {
            return None;
        };
        let (marks, phase) = marks.acknowledge(snapshot.phase(), domain, ticket).ok()?;
        let effect = self.effect(cell, expected, Membership::Tracing(marks))?;
        self.begin_prepare(snapshot, phase, Some(effect))
    }

    pub fn begin_trace_retire(
        &self,
        snapshot: Snapshot<R>,
        cell: Pooled<Cell<R>>,
        domain: usize,
    ) -> Option<Preparation<R>> {
        let expected = cell.state.load()?;
        let Membership::Tracing(marks) = *expected else {
            return None;
        };
        let phase = snapshot.phase();
        marks.can_retire(phase, domain).ok()?;
        let effect = self.effect(cell, expected, Membership::Retired)?;
        self.begin_prepare(snapshot, phase, Some(effect))
    }

    pub fn begin_promote(
        &self,
        snapshot: Snapshot<R>,
        cell: Pooled<Cell<R>>,
    ) -> Option<Preparation<R>> {
        let expected = cell.state.load()?;
        let Membership::Tracing(marks) = *expected else {
            return None;
        };
        let (marks, phase) = marks.promote(snapshot.phase()).ok()?;
        let effect = self.effect(cell, expected, Membership::Tracing(marks))?;
        self.begin_prepare(snapshot, phase, Some(effect))
    }

    /// Numeric phase activation. Root enumeration and callback obligations are
    /// separate protocols; this operation is not a certificate for either.
    pub fn begin_start(
        &self,
        snapshot: Snapshot<R>,
        domain: usize,
        mode: DomainMode,
    ) -> Option<Preparation<R>> {
        let mut phase = snapshot.phase();
        let entry = phase.domains.get_mut(domain)?;
        if mode == DomainMode::Inactive
            || (mode == DomainMode::Old && domain != 1)
            || (mode == DomainMode::Young && domain != 0)
            || entry.mode != DomainMode::Inactive
            || entry.pending != 0
        {
            return None;
        }
        entry.mode = mode;
        entry.epoch = entry.epoch.checked_add(1)?;
        entry.floor = 0;
        phase.root_cursors[domain] = 0;
        phase.reference_cursors[domain] = 0;
        self.begin_prepare(snapshot, phase, None)
    }

    /// Bind a policy's completed sweep to its exact collection epoch. A queued
    /// request must not rebase its Finish onto a later, unswept collection.
    pub fn begin_finish_epoch(
        &self,
        snapshot: Snapshot<R>,
        domain: usize,
        epoch: u64,
    ) -> Option<Preparation<R>> {
        if snapshot.phase().domains.get(domain)?.epoch != epoch {
            return None;
        }
        self.begin_finish(snapshot, domain)
    }
    pub fn begin_finish(&self, snapshot: Snapshot<R>, domain: usize) -> Option<Preparation<R>> {
        let mut phase = snapshot.phase();
        let entry = phase.domains.get_mut(domain)?;
        if entry.mode == DomainMode::Inactive
            || entry.floor != RETIREMENT_FLOOR
            || entry.pending != 0
            || phase.root_cursors[domain] != self.root_capacity()
        {
            return None;
        }
        entry.mode = DomainMode::Inactive;
        entry.floor = 0;
        self.begin_prepare(snapshot, phase, None)
    }

    pub fn advance(&self, snapshot: Snapshot<R>, domain: usize) -> Option<Attempt<R>> {
        let preparation = self.begin_advance(snapshot, domain)?;
        self.prepare_now(preparation)
    }

    pub fn begin_advance(&self, snapshot: Snapshot<R>, domain: usize) -> Option<Preparation<R>> {
        let mut phase = snapshot.phase();
        let entry = phase.domains.get_mut(domain)?;
        if entry.mode == DomainMode::Inactive
            || entry.pending != 0
            || entry.floor >= RETIREMENT_FLOOR
            || phase.root_cursors[domain] != self.root_capacity()
            || (entry.floor != 0 && phase.reference_cursors[domain] != self.reference_capacity())
        {
            return None;
        }
        entry.floor += 1;
        phase.reference_cursors[domain] = 0;
        self.begin_prepare(snapshot, phase, None)
    }
    /// One help attempt over already prepared owning state. Acquisition failure
    /// returns without a completion certificate. No exclusive helper claim exists.
    fn finish(&self, busy: &Pooled<Control<R>>, context: &R::Context) {
        self.finish_at(busy, context, || {}, || {}, || {}, || {});
    }
    fn finish_at(
        &self,
        busy: &Pooled<Control<R>>,
        context: &R::Context,
        before_effect: impl FnOnce(),
        after_effect: impl FnOnce(),
        after_certificate: impl FnOnce(),
        after_release: impl FnOnce(),
    ) {
        if let Control::Phase(operation) = &**busy {
            // Called only for a successfully installed or observed head. This
            // single decision certifies all immutable phase data; no record,
            // root, reference or edge effect can be attached to this variant.
            let _ = operation.decision.compare_exchange(
                UNDECIDED,
                COMMIT,
                Ordering::AcqRel,
                Ordering::Acquire,
            );
            after_effect();
            after_certificate();
            after_release();
            return;
        }
        let Control::Busy(operation) = &**busy else {
            return;
        };
        let completed = &operation.outcome().decision;
        // Prepare every fallible effect acquisition before deciding COMMIT.
        // A caller supplying a private-graph guard may release it on this early
        // return: no irreversible commit decision has been made by this helper.
        let proposed = if completed.load(Ordering::Acquire) == UNDECIDED
            && operation.decision.load(Ordering::Acquire) != CANCEL
        {
            if let Some(effect) = &operation.effect {
                let Some(proposed) = share(&effect.proposed) else {
                    return;
                };
                Some(proposed)
            } else {
                None
            }
        } else {
            None
        };
        let secondary_proposed = if let Some(effect) = &operation.secondary_effect
            && completed.load(Ordering::Acquire) == UNDECIDED
            && operation.decision.load(Ordering::Acquire) != CANCEL
        {
            let Some(proposed) = share(&effect.proposed) else {
                return;
            };
            Some(proposed)
        } else {
            None
        };
        let reference_proposed = if let Some(effect) = &operation.reference
            && completed.load(Ordering::Acquire) == UNDECIDED
            && operation.decision.load(Ordering::Acquire) != CANCEL
        {
            let Some(proposed) = share(&effect.proposed) else {
                return;
            };
            Some(proposed)
        } else {
            None
        };
        let edge_proposed = if let Some(effect) = &operation.edge
            && completed.load(Ordering::Acquire) == UNDECIDED
            && operation.decision.load(Ordering::Acquire) != CANCEL
        {
            let Some(proposed) = share(&effect.proposed) else {
                return;
            };
            Some(proposed)
        } else {
            None
        };
        // Real-record publication must retain its private graph before the
        // irreversible decision. Already certified work only needs head release.
        let mut entered = None;
        if let Some(effect) = &operation.effect
            && *effect.expected == Membership::Prepared
            && matches!(*effect.proposed, Membership::Tracing(_))
            && completed.load(Ordering::Acquire) == UNDECIDED
            && operation.decision.load(Ordering::Acquire) != CANCEL
        {
            match effect.cell.record.enter(context) {
                Ok(guard) => entered = Some(guard),
                Err(RecordError::Contended) => return,
                Err(RecordError::Closed | RecordError::Foreign) => {
                    match operation.decision.compare_exchange(
                        UNDECIDED,
                        CANCEL,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    ) {
                        Ok(_) | Err(CANCEL) => {}
                        Err(COMMIT) => {
                            // A prior committer owns its own graph guard until
                            // certification. A terminally closed record therefore
                            // cannot authorize any new uncertified effect here.
                            if completed.load(Ordering::Acquire) == UNDECIDED {
                                return;
                            }
                        }
                        Err(_) => unreachable!("invalid immutable decision"),
                    }
                }
            }
        }
        let decision = match operation.decision.compare_exchange(
            UNDECIDED,
            COMMIT,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => COMMIT,
            Err(decision) => decision,
        };
        if completed.load(Ordering::Acquire) == UNDECIDED
            && decision == COMMIT
            && let Some(effect) = &operation.effect
        {
            before_effect();
            let proposed = proposed.expect("effect prepared before commit decision");
            // This exact membership version is changed only by this installed
            // controller and is never reinstalled. A strong CAS failure therefore
            // proves this effect already happened, possibly followed by later
            // certified controllers. No owning reread is required to certify it.
            // There is no fallible acquisition between COMMIT and its certificate.
            effect.cell.state.compare(&effect.expected, proposed);
        }
        after_effect();
        // Only already-tracing owner barriers use the second effect. It never
        // publishes a Prepared record and therefore needs no private-graph guard.
        if let Some(effect) = &operation.secondary_effect
            && decision == COMMIT
            && completed.load(Ordering::Acquire) == UNDECIDED
        {
            effect.cell.state.compare(
                &effect.expected,
                secondary_proposed.expect("secondary effect prepared before commit decision"),
            );
        }
        // Root visibility/removal shares this controller with the object effect.
        // Exact monotonic status versions never reset. A strong CAS failure can
        // only mean this effect already happened (possibly followed by retirement).
        if let Some(root) = &operation.root
            && decision == COMMIT
            && completed.load(Ordering::Acquire) == UNDECIDED
        {
            root.apply();
        }
        if let Some(root) = &operation.secondary_root
            && decision == COMMIT
            && completed.load(Ordering::Acquire) == UNDECIDED
        {
            root.apply();
        }
        if let Some(reference) = &operation.reference
            && decision == COMMIT
            && completed.load(Ordering::Acquire) == UNDECIDED
        {
            reference.apply(
                reference_proposed.expect("reference effect prepared before commit decision"),
            );
        }
        if let Some(edge) = &operation.edge
            && decision == COMMIT
            && completed.load(Ordering::Acquire) == UNDECIDED
        {
            edge.apply(edge_proposed.expect("edge effect prepared before commit decision"));
        }
        if decision == COMMIT
            && let Some(guard) = &entered
        {
            // This is a concrete atomic store, not a user callback. A completed
            // certificate observes cleanup ownership before any guard can end.
            guard.certified();
        }
        completed.store(decision, Ordering::Release);
        after_certificate();
        let Some(after) = share(&operation.successor) else {
            return;
        };
        self.head.compare(busy, after);
        after_release();
    }
    /// One install/help attempt. Failed owning acquisition leaves Pending;
    /// it cannot prove that an effect was applied or a request was superseded.
    pub fn commit_in(&self, attempt: &Attempt<R>, context: &R::Context) -> Outcome {
        assert!(
            Arc::ptr_eq(&attempt.owner, &self.owner),
            "foreign operation"
        );
        let mut superseded = false;
        if let Some(proposed) = share(&attempt.busy) {
            if self.head.compare(&attempt.expected, proposed) {
                self.finish(&attempt.busy, context);
            } else if let Some(current) = self.head.load() {
                // Exact owning controller versions are never reinstalled. An
                // observed different controller excludes a future install of
                // this request; completed certificates still take priority below.
                superseded = !Pooled::ptr_eq(&current, &attempt.busy)
                    && !Pooled::ptr_eq(&current, &attempt.expected);
                self.finish(&current, context);
            }
        }
        match attempt.completed() {
            COMMIT => Outcome::Committed,
            CANCEL => Outcome::Cancelled,
            _ if attempt.decision().load(Ordering::Acquire) == CANCEL => Outcome::Cancelled,
            _ if superseded => Outcome::Retry,
            _ => Outcome::Pending,
        }
    }
    #[cfg(test)]
    pub fn install_and_stop(&self, attempt: &Attempt<R>) -> bool {
        self.head.compare(
            &attempt.expected,
            share(&attempt.busy).expect("quiescent fixture clone"),
        )
    }
    #[cfg(test)]
    pub fn effect_and_stop(&self, attempt: &Attempt<R>) {
        assert!(self.install_and_stop(attempt));
        if matches!(*attempt.busy, Control::Phase(_)) {
            assert_eq!(
                attempt.decision().compare_exchange(
                    UNDECIDED,
                    COMMIT,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                ),
                Ok(UNDECIDED)
            );
            return;
        }
        let operation = attempt.operation();
        assert_eq!(
            operation.decision.compare_exchange(
                UNDECIDED,
                COMMIT,
                Ordering::AcqRel,
                Ordering::Acquire
            ),
            Ok(UNDECIDED)
        );
        if let Some(effect) = &operation.effect {
            assert!(effect.cell.state.compare(
                &effect.expected,
                share(&effect.proposed).expect("quiescent fixture clone")
            ));
        }
    }
}

#[cfg(test)]
impl Catalog<()> {
    pub fn help_at(&self, point: usize, stop: impl FnOnce()) {
        self.help_observed_at(point, || {}, stop);
    }
    pub fn help_observed_at(&self, point: usize, observed: impl FnOnce(), stop: impl FnOnce()) {
        let busy = self.head.load().expect("quiescent fixture load");
        assert!(matches!(*busy, Control::Busy(_) | Control::Phase(_)));
        observed();
        match point {
            0 => self.finish_at(&busy, &(), stop, || {}, || {}, || {}),
            1 => self.finish_at(&busy, &(), || {}, stop, || {}, || {}),
            2 => self.finish_at(&busy, &(), || {}, || {}, stop, || {}),
            3 => self.finish_at(&busy, &(), || {}, || {}, || {}, stop),
            _ => panic!("unknown stop point"),
        }
    }
    pub fn late_help(&self, attempt: &Attempt<()>) {
        assert_ne!(attempt.completed(), UNDECIDED);
        self.finish(&attempt.busy, &());
    }
    pub fn with_phase(phase: Phase) -> Self {
        let mut catalog = Self::new();
        catalog.head = PooledLink::new(
            catalog.controls.clone(),
            allocate(&catalog.controls, Control::Stable(phase)).expect("fresh control pool"),
        );
        catalog
    }
    pub fn exhaust_identities(&self) {
        self.next_identity.store(u64::MAX, Ordering::Relaxed);
    }
    pub fn prepare_and_stop(
        &self,
        slot: usize,
        address: usize,
        after: bool,
        stop: impl FnOnce(),
    ) -> Result<Pooled<Cell<()>>, PrepareError> {
        if after {
            self.prepare_cell_at(slot, address, || {}, stop)
        } else {
            self.prepare_cell_at(slot, address, stop, || {})
        }
    }
}

#[cfg(test)]
impl Catalog<()> {
    pub fn pool_occupancy(&self) -> (usize, usize) {
        (self.controls.occupied(), self.memberships.occupied())
    }

    pub fn cell_pool_occupancy(&self) -> usize {
        self.cells.occupied()
    }

    pub fn hold_control_slots(&self, slots: &[usize]) -> impl Sized + use<> {
        let phase = self.snapshot().expect("quiescent fixture").phase();
        slots
            .iter()
            .map(|index| {
                self.controls
                    .try_allocate_at(*index, Control::Stable(phase))
                    .unwrap_or_else(|_| panic!("fixture slot already occupied"))
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
impl Catalog<()> {
    pub fn probe_control_generation(&self, index: usize, phase: Phase) -> Option<AllocationError> {
        self.controls
            .try_allocate_at_status(index, Control::Stable(phase))
            .err()
            .map(|failure| failure.error)
    }
}

impl Catalog<()> {
    pub fn begin_create(&self, slot: usize, address: usize) -> Result<Creation<()>, PrepareError> {
        self.begin_create_with(slot, address, ())
            .map_err(|failure| failure.error)
    }
    pub fn snapshot(&self) -> Option<Snapshot<()>> {
        self.snapshot_in(&())
    }
    pub fn commit(&self, attempt: &Attempt<()>) -> Outcome {
        self.commit_in(attempt, &())
    }
    pub fn prepare_cell(
        &self,
        slot: usize,
        address: usize,
    ) -> Result<Pooled<Cell<()>>, PrepareError> {
        self.prepare_cell_at(slot, address, || {}, || {})
    }
    fn prepare_cell_at(
        &self,
        slot: usize,
        address: usize,
        before: impl FnOnce(),
        after: impl FnOnce(),
    ) -> Result<Pooled<Cell<()>>, PrepareError> {
        let entry = self.slots.get(slot).ok_or(PrepareError::InvalidSlot)?;
        if address == 0 {
            return Err(PrepareError::InvalidAddress);
        }
        if entry.load().map_err(|_| PrepareError::Contended)?.is_some() {
            return Err(PrepareError::Occupied);
        }
        let identity = self.next_identity.load(Ordering::Relaxed);
        let next = identity
            .checked_add(1)
            .ok_or(PrepareError::IdentityExhausted)?;
        self.next_identity
            .compare_exchange(identity, next, Ordering::Relaxed, Ordering::Relaxed)
            .map_err(|_| PrepareError::Contended)?;
        let cell = allocate(
            &self.cells,
            Cell {
                record: (),
                owner: self.owner.clone(),
                identity,
                address,
                slot: AtomicUsize::new(slot),
                abandon_requested: AtomicU8::new(0),
                state: PooledLink::new(
                    self.memberships.clone(),
                    allocate(&self.memberships, Membership::Prepared)
                        .ok_or(PrepareError::BackingUnavailable)?,
                ),
            },
        )
        .ok_or(PrepareError::BackingUnavailable)?;
        before();
        let proposed = share(&cell).ok_or(PrepareError::Contended)?;
        if entry.compare(None, Some(proposed)) {
            after();
            Ok(cell)
        } else {
            Err(PrepareError::Occupied)
        }
    }
}

impl Catalog<()> {
    pub fn insert(&self, snapshot: Snapshot<()>, cell: Pooled<Cell<()>>) -> Option<Attempt<()>> {
        let preparation = self.begin_insert(snapshot, cell)?;
        self.prepare_now(preparation)
    }
    pub fn begin_insert(
        &self,
        snapshot: Snapshot<()>,
        cell: Pooled<Cell<()>>,
    ) -> Option<Preparation<()>> {
        let expected = cell.state.load()?;
        if *expected != Membership::Prepared {
            return None;
        }
        let mut phase = snapshot.phase();
        let ticket = phase.revision.checked_add(1)?;
        for domain in &mut phase.domains {
            if domain.mode == DomainMode::Inactive {
                return None;
            }
            domain.pending = domain.pending.checked_add(1)?;
        }
        let effect = self.effect(cell, expected, Membership::Live([ticket; 2]))?;
        self.begin_prepare(snapshot, phase, Some(effect))
    }
    pub fn acknowledge(
        &self,
        snapshot: Snapshot<()>,
        cell: Pooled<Cell<()>>,
        domain: usize,
        ticket: u64,
    ) -> Option<Attempt<()>> {
        let preparation = self.begin_acknowledge(snapshot, cell, domain, ticket)?;
        self.prepare_now(preparation)
    }
    pub fn begin_acknowledge(
        &self,
        snapshot: Snapshot<()>,
        cell: Pooled<Cell<()>>,
        domain: usize,
        ticket: u64,
    ) -> Option<Preparation<()>> {
        let expected = cell.state.load()?;
        let Membership::Live(mut work) = *expected else {
            return None;
        };
        if ticket == 0 || *work.get(domain)? != ticket {
            return None;
        }
        work[domain] = 0;
        let mut phase = snapshot.phase();
        phase.domains[domain].pending = phase.domains[domain].pending.checked_sub(1)?;
        let effect = self.effect(cell, expected, Membership::Live(work))?;
        self.begin_prepare(snapshot, phase, Some(effect))
    }
    pub fn require(
        &self,
        snapshot: Snapshot<()>,
        cell: Pooled<Cell<()>>,
        domain: usize,
    ) -> Option<Attempt<()>> {
        let preparation = self.begin_require(snapshot, cell, domain)?;
        self.prepare_now(preparation)
    }
    pub fn begin_require(
        &self,
        snapshot: Snapshot<()>,
        cell: Pooled<Cell<()>>,
        domain: usize,
    ) -> Option<Preparation<()>> {
        let expected = cell.state.load()?;
        let Membership::Live(mut work) = *expected else {
            return None;
        };
        let old = *work.get(domain)?;
        let mut phase = snapshot.phase();
        if phase.domains[domain].mode == DomainMode::Inactive {
            return None;
        }
        work[domain] = phase.revision.checked_add(1)?;
        if old == 0 {
            phase.domains[domain].pending = phase.domains[domain].pending.checked_add(1)?;
        }
        let effect = self.effect(cell, expected, Membership::Live(work))?;
        self.begin_prepare(snapshot, phase, Some(effect))
    }
    pub fn retire(&self, snapshot: Snapshot<()>, cell: Pooled<Cell<()>>) -> Option<Attempt<()>> {
        let preparation = self.begin_retire(snapshot, cell)?;
        self.prepare_now(preparation)
    }
    pub fn begin_retire(
        &self,
        snapshot: Snapshot<()>,
        cell: Pooled<Cell<()>>,
    ) -> Option<Preparation<()>> {
        let expected = cell.state.load()?;
        if *expected != Membership::Live([0; 2]) {
            return None;
        }
        let phase = snapshot.phase();
        let effect = self.effect(cell, expected, Membership::Retired)?;
        self.begin_prepare(snapshot, phase, Some(effect))
    }
    /// Establish unmarked numeric tracing membership. This is not allocation
    /// publication: no payload, construction root or birth exemption is supplied.
    pub fn begin_trace_insert(
        &self,
        snapshot: Snapshot<()>,
        cell: Pooled<Cell<()>>,
    ) -> Option<Preparation<()>> {
        let expected = cell.state.load()?;
        if *expected != Membership::Prepared {
            return None;
        }
        let phase = snapshot.phase();
        let marks = TraceState::unmarked(phase);
        let effect = self.effect(cell, expected, Membership::Tracing(marks))?;
        self.begin_prepare(snapshot, phase, Some(effect))
    }
}

#[cfg(test)]
impl<R: Record> Catalog<R> {
    pub fn help_record_at(&self, context: &R::Context, point: usize, stop: impl FnOnce()) {
        let busy = self.head.load().expect("quiescent record fixture");
        assert!(matches!(*busy, Control::Busy(_) | Control::Phase(_)));
        match point {
            0 => self.finish_at(&busy, context, stop, || {}, || {}, || {}),
            1 => self.finish_at(&busy, context, || {}, stop, || {}, || {}),
            2 => self.finish_at(&busy, context, || {}, || {}, stop, || {}),
            3 => self.finish_at(&busy, context, || {}, || {}, || {}, stop),
            _ => panic!("invalid record stop point"),
        }
    }
}

#[cfg(test)]
#[path = "observation_tests.rs"]
pub(crate) mod observation_tests;
