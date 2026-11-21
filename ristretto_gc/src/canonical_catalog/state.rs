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
#[path = "edges.rs"]
mod edges;
#[path = "phase_ops.rs"]
mod phase_ops;
#[path = "roots.rs"]
mod roots;
use edges::EdgeRegistry;
pub(crate) use roots::root_barrier_plan;
pub use roots::{Root, RootCreation, RootPrepareError, RootStatus, RootToken};
use roots::{RootEffect, RootRegistry};
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
    root: Option<RootEffect<R>>,

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
    root: Option<RootEffect<R>>,

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
            if self.effect.is_none() && self.root.is_none() {
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
            root: self.root.take(),

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
        match decision::choose(self.decision(), CANCEL) {
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
    roots: Option<RootRegistry<R>>,
    edges: Option<EdgeRegistry>,
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
            roots: None,
            edges: None,
        }
    }
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }
    pub fn generation_exhaustion_observed(&self) -> bool {
        self.edges.as_ref().is_some_and(|edges| {
            edges.slots_pool.generation_exhaustion_observed()
                || edges.states.generation_exhaustion_observed()
        }) || self.controls.generation_exhaustion_observed()
            || self.memberships.generation_exhaustion_observed()
            || self.cells.generation_exhaustion_observed()
            || self
                .roots
                .as_ref()
                .is_some_and(|roots| roots.pool.generation_exhaustion_observed())
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

impl<R: Record> Catalog<R> {
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
            successor: None,
            operation: None,
        })
    }
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
}

#[cfg(test)]
#[path = "preparation_tests.rs"]
mod preparation_tests;

impl<R: Record> Catalog<R> {
    pub(crate) fn possible_phase_outcomes(&self) -> Option<[Phase; 2]> {
        let head = self.head.load()?;
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
    fn stable_head_in(&self, context: &R::Context) -> Option<Pooled<Control<R>>> {
        let head = self.head.load()?;
        if head.is_stable() {
            Some(head)
        } else {
            self.finish(&head, context);
            None
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
            let _ = decision::choose(&operation.decision, COMMIT);
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
                    match decision::choose(&operation.decision, CANCEL) {
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
        let decision = match decision::choose(&operation.decision, COMMIT) {
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
        if let Some(root) = &operation.root
            && decision == COMMIT
            && completed.load(Ordering::Acquire) == UNDECIDED
        {
            root.apply();
        }
        if decision == COMMIT
            && let Some(guard) = &entered
        {
            // This is a concrete atomic store, not a user callback. A completed
            // certificate observes cleanup ownership before any guard can end.
            guard.certified();
        }
        decision::certify(completed, decision);
        after_certificate();
        let Some(after) = share(&operation.successor) else {
            return;
        };
        self.head.compare(busy, after);
        after_release();
    }
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
        assert_eq!(decision::choose(&operation.decision, COMMIT), Ok(UNDECIDED));
        if let Some(effect) = &operation.effect {
            assert!(effect.cell.state.compare(
                &effect.expected,
                share(&effect.proposed).expect("quiescent fixture clone")
            ));
        }
    }
}

#[cfg(test)]
#[path = "commit_tests.rs"]
mod commit_tests;

#[path = "decision.rs"]
mod decision;
#[cfg(all(test, not(target_family = "wasm")))]
#[path = "decision_models.rs"]
mod decision_models;

impl<R: Record> Catalog<R> {
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
    #[expect(
        clippy::indexing_slicing,
        reason = "an exact same-catalog cell owns its validated immutable discovery slot"
    )]
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
}

impl<R: Record> Catalog<R> {
    #[cfg(test)]
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
