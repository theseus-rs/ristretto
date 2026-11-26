//! A discoverable bounded operation couples one cell effect and both phase domains.
//! Every cell mutation must go through this controller. A fixed-capacity registry
//! discovers prepared cells and durable work. Payload tracing, storage ownership,
//! byte budgeting and the real reference-floor state machine remain unimplemented.
use super::{Arc, AtomicU8, AtomicU64, OptionalLink, Ordering, Pool, Pooled, PooledLink, allocate};

const UNDECIDED: u8 = 0;
const COMMIT: u8 = 1;
const CANCEL: u8 = 2;

struct Owner;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Domain {
    pub epoch: u64,
    pub floor: u8,
    pub pending: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase {
    pub revision: u64,
    pub domains: [Domain; 2],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Membership {
    Prepared,
    Live([u64; 2]),
    Retired,
}

pub struct Cell {
    owner: Arc<Owner>,
    pub identity: u64,
    pub address: usize,
    pub slot: usize,
    state: PooledLink<Membership>,
}
impl Cell {
    pub fn membership(&self) -> Membership {
        *self.state.load()
    }
}

struct Effect {
    cell: Arc<Cell>,
    expected: Pooled<Membership>,
    proposed: Pooled<Membership>,
}
struct Operation {
    effect: Option<Effect>,
    committed: Pooled<Control>,
    cancelled: Pooled<Control>,
    decision: AtomicU8,
    completed: AtomicU8,
}
enum Control {
    Stable(Phase),
    Busy(Operation),
}

pub struct Snapshot {
    owner: Arc<Owner>,
    head: Pooled<Control>,
}
impl Snapshot {
    pub fn phase(&self) -> Phase {
        let Control::Stable(phase) = &*self.head else {
            unreachable!()
        };
        *phase
    }
}
pub struct Attempt {
    owner: Arc<Owner>,
    expected: Pooled<Control>,
    busy: Pooled<Control>,
}
impl Attempt {
    fn operation(&self) -> &Operation {
        let Control::Busy(operation) = &*self.busy else {
            unreachable!()
        };
        operation
    }
    /// The immutable decision is shared by all helpers. Cancellation does not
    /// release an already installed controller; a subsequent helper does that.
    pub fn cancel(&self) -> bool {
        match self.operation().decision.compare_exchange(
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

pub struct Catalog {
    owner: Arc<Owner>,
    head: PooledLink<Control>,
    controls: Arc<Pool<Control>>,
    memberships: Arc<Pool<Membership>>,
    slots: Box<[OptionalLink<Cell>]>,
    next_identity: AtomicU64,
}
impl Default for Catalog {
    fn default() -> Self {
        Self::new()
    }
}
impl Catalog {
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
        assert!(capacity > 0);
        let controls = Pool::new(control_capacity);
        let memberships = Pool::new(state_capacity);
        let phase = Phase {
            revision: 1,
            domains: [Domain {
                epoch: 1,
                floor: 0,
                pending: 0,
            }; 2],
        };
        let head = PooledLink::new(
            allocate(&controls, Control::Stable(phase)).expect("fresh control pool"),
        );
        Self {
            owner: Arc::new(Owner),
            head,
            controls,
            memberships,
            slots: (0..capacity).map(|_| OptionalLink::empty()).collect(),
            next_identity: AtomicU64::new(1),
        }
    }
    /// Announce a prepared numeric descriptor in one selected slot. An actual
    /// allocation must transfer initialized metadata/credits/initial child roots
    /// into this record before this announcement. This experiment has none yet.
    /// No slot reservation or global admission is retained on failure.
    pub fn prepare_cell(&self, slot: usize, address: usize) -> Result<Arc<Cell>, PrepareError> {
        self.prepare_cell_at(slot, address, || {}, || {})
    }
    fn prepare_cell_at(
        &self,
        slot: usize,
        address: usize,
        before: impl FnOnce(),
        after: impl FnOnce(),
    ) -> Result<Arc<Cell>, PrepareError> {
        let entry = self.slots.get(slot).ok_or(PrepareError::InvalidSlot)?;
        if address == 0 {
            return Err(PrepareError::InvalidAddress);
        }
        if entry.load().is_some() {
            return Err(PrepareError::Occupied);
        }
        let identity = self.next_identity.load(Ordering::Relaxed);
        let next = identity
            .checked_add(1)
            .ok_or(PrepareError::IdentityExhausted)?;
        self.next_identity
            .compare_exchange(identity, next, Ordering::Relaxed, Ordering::Relaxed)
            .map_err(|_| PrepareError::Contended)?;
        let cell = Arc::new(Cell {
            owner: self.owner.clone(),
            identity,
            address,
            slot,
            state: PooledLink::new(
                allocate(&self.memberships, Membership::Prepared)
                    .ok_or(PrepareError::BackingUnavailable)?,
            ),
        });
        before();
        if entry.compare(None, Some(cell.clone())) {
            after();
            Ok(cell)
        } else {
            Err(PrepareError::Occupied)
        }
    }
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }
    /// One owning slot observation; a scan cursor visits one slot per poll.
    pub fn discover(&self, slot: usize) -> Option<Arc<Cell>> {
        self.slots.get(slot)?.load()
    }
    /// Terminal membership is irreversible; cleanup never authorizes a payload
    /// destructor, and compares the exact old owner so it cannot clear slot reuse.
    pub fn release_retired(&self, cell: &Arc<Cell>) -> bool {
        if !Arc::ptr_eq(&cell.owner, &self.owner) || cell.membership() != Membership::Retired {
            return false;
        }
        self.slots[cell.slot].compare(Some(cell), None)
    }
    /// Insertion cancellation is a canonical Prepared -> Retired operation.
    /// Attempt::cancel only cancels that particular immutable proposal.
    pub fn abandon(&self, snapshot: Snapshot, cell: Arc<Cell>) -> Option<Attempt> {
        let expected = cell.state.load();
        if *expected != Membership::Prepared {
            return None;
        }
        let phase = snapshot.phase();
        let effect = self.effect(cell, expected, Membership::Retired)?;
        self.prepare(snapshot, phase, Some(effect))
    }
    /// A single bounded help attempt. Seeing Busy cannot yield a phase snapshot.
    pub fn snapshot(&self) -> Option<Snapshot> {
        let head = self.head.load();
        if matches!(*head, Control::Busy(_)) {
            self.finish(&head);
            None
        } else {
            Some(Snapshot {
                owner: self.owner.clone(),
                head,
            })
        }
    }
    fn prepare(
        &self,
        snapshot: Snapshot,
        mut phase: Phase,
        effect: Option<Effect>,
    ) -> Option<Attempt> {
        if !Arc::ptr_eq(&snapshot.owner, &self.owner) {
            return None;
        }
        let mut cancelled = snapshot.phase();
        cancelled.revision = cancelled.revision.checked_add(1)?;
        phase.revision = cancelled.revision;
        let operation = Operation {
            effect,
            committed: allocate(&self.controls, Control::Stable(phase))?,
            cancelled: allocate(&self.controls, Control::Stable(cancelled))?,
            decision: AtomicU8::new(UNDECIDED),
            completed: AtomicU8::new(UNDECIDED),
        };
        Some(Attempt {
            owner: self.owner.clone(),
            expected: snapshot.head,
            busy: allocate(&self.controls, Control::Busy(operation))?,
        })
    }
    fn effect(
        &self,
        cell: Arc<Cell>,
        expected: Pooled<Membership>,
        proposed: Membership,
    ) -> Option<Effect> {
        if !Arc::ptr_eq(&cell.owner, &self.owner) {
            return None;
        }
        Some(Effect {
            cell,
            expected,
            proposed: allocate(&self.memberships, proposed)?,
        })
    }
    pub fn insert(&self, snapshot: Snapshot, cell: Arc<Cell>) -> Option<Attempt> {
        let expected = cell.state.load();
        if *expected != Membership::Prepared {
            return None;
        }
        let mut phase = snapshot.phase();
        let ticket = phase.revision.checked_add(1)?;
        for domain in &mut phase.domains {
            domain.pending = domain.pending.checked_add(1)?;
        }
        let effect = self.effect(cell, expected, Membership::Live([ticket; 2]))?;
        self.prepare(snapshot, phase, Some(effect))
    }
    pub fn acknowledge(
        &self,
        snapshot: Snapshot,
        cell: Arc<Cell>,
        domain: usize,
        ticket: u64,
    ) -> Option<Attempt> {
        let expected = cell.state.load();
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
        self.prepare(snapshot, phase, Some(effect))
    }
    pub fn require(&self, snapshot: Snapshot, cell: Arc<Cell>, domain: usize) -> Option<Attempt> {
        let expected = cell.state.load();
        let Membership::Live(mut work) = *expected else {
            return None;
        };
        let old = *work.get(domain)?;
        let mut phase = snapshot.phase();
        work[domain] = phase.revision.checked_add(1)?;
        if old == 0 {
            phase.domains[domain].pending = phase.domains[domain].pending.checked_add(1)?;
        }
        let effect = self.effect(cell, expected, Membership::Live(work))?;
        self.prepare(snapshot, phase, Some(effect))
    }
    pub fn retire(&self, snapshot: Snapshot, cell: Arc<Cell>) -> Option<Attempt> {
        let expected = cell.state.load();
        if *expected != Membership::Live([0; 2]) {
            return None;
        }
        let phase = snapshot.phase();
        let effect = self.effect(cell, expected, Membership::Retired)?;
        self.prepare(snapshot, phase, Some(effect))
    }
    pub fn advance(&self, snapshot: Snapshot, domain: usize) -> Option<Attempt> {
        let mut phase = snapshot.phase();
        let entry = phase.domains.get_mut(domain)?;
        if entry.pending != 0 {
            return None;
        }
        if entry.floor == 3 {
            entry.epoch = entry.epoch.checked_add(1)?;
            entry.floor = 0;
        } else {
            entry.floor += 1;
        }
        self.prepare(snapshot, phase, None)
    }
    /// Must only receive a control that was observed installed. Protocol-owned
    /// state is prepared. The ArcSwap adapter can allocate thread bookkeeping,
    /// scan other readers, retry internally and synchronously destroy old Arcs;
    /// its primitive cost is not a bounded clockless-runtime guarantee.
    fn finish(&self, busy: &Pooled<Control>) {
        self.finish_at(busy, || {}, || {}, || {}, || {});
    }
    fn finish_at(
        &self,
        busy: &Pooled<Control>,
        before_effect: impl FnOnce(),
        after_effect: impl FnOnce(),
        after_certificate: impl FnOnce(),
        after_release: impl FnOnce(),
    ) {
        let Control::Busy(operation) = &**busy else {
            return;
        };
        let decision = match operation.decision.compare_exchange(
            UNDECIDED,
            COMMIT,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => COMMIT,
            Err(decision) => decision,
        };
        before_effect();
        if decision == COMMIT
            && let Some(effect) = &operation.effect
        {
            let previous = effect
                .cell
                .state
                .compare(&effect.expected, effect.proposed.clone());
            // Another helper either applied this exact effect, or released
            // the controller after certifying it and a later operation won.
            assert!(
                Pooled::ptr_eq(&previous, &effect.expected)
                    || Pooled::ptr_eq(&previous, &effect.proposed)
                    || operation.completed.load(Ordering::Acquire) == COMMIT,
                "uncertified effect interference"
            );
        }
        after_effect();
        // Every controller replacement certifies completion first. Old owning
        // expected cell pointers forbid replay after a later effect/retirement.
        operation.completed.store(decision, Ordering::Release);
        after_certificate();
        let after = if decision == COMMIT {
            &operation.committed
        } else {
            &operation.cancelled
        };
        self.head.compare(busy, after.clone());
        after_release();
    }
    /// One install/help attempt; contention returns Retry without an admission
    /// count or an exclusive claim that requires its owner to resume.
    pub fn commit(&self, attempt: &Attempt) -> Outcome {
        assert!(
            Arc::ptr_eq(&attempt.owner, &self.owner),
            "foreign operation"
        );
        let operation = attempt.operation();
        let previous = self.head.compare(&attempt.expected, attempt.busy.clone());
        if Pooled::ptr_eq(&previous, &attempt.expected) || Pooled::ptr_eq(&previous, &attempt.busy)
        {
            self.finish(&attempt.busy);
        } else {
            self.finish(&previous);
        }
        match operation.completed.load(Ordering::Acquire) {
            COMMIT => Outcome::Committed,
            CANCEL => Outcome::Cancelled,
            _ if operation.decision.load(Ordering::Acquire) == CANCEL => Outcome::Cancelled,
            _ => Outcome::Retry,
        }
    }
    #[cfg(test)]
    pub fn install_and_stop(&self, attempt: &Attempt) -> bool {
        Pooled::ptr_eq(
            &self.head.compare(&attempt.expected, attempt.busy.clone()),
            &attempt.expected,
        )
    }
    #[cfg(test)]
    pub fn effect_and_stop(&self, attempt: &Attempt) {
        assert!(self.install_and_stop(attempt));
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
            assert!(Pooled::ptr_eq(
                &effect
                    .cell
                    .state
                    .compare(&effect.expected, effect.proposed.clone()),
                &effect.expected
            ));
        }
    }
}

#[cfg(test)]
impl Catalog {
    pub fn help_at(&self, point: usize, stop: impl FnOnce()) {
        let busy = self.head.load();
        assert!(matches!(*busy, Control::Busy(_)));
        match point {
            0 => self.finish_at(&busy, stop, || {}, || {}, || {}),
            1 => self.finish_at(&busy, || {}, stop, || {}, || {}),
            2 => self.finish_at(&busy, || {}, || {}, stop, || {}),
            3 => self.finish_at(&busy, || {}, || {}, || {}, stop),
            _ => panic!("unknown stop point"),
        }
    }
    pub fn late_help(&self, attempt: &Attempt) {
        assert_ne!(
            attempt.operation().completed.load(Ordering::Acquire),
            UNDECIDED
        );
        self.finish(&attempt.busy);
    }
    pub fn with_phase(phase: Phase) -> Self {
        let mut catalog = Self::new();
        catalog.head = PooledLink::new(
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
    ) -> Result<Arc<Cell>, PrepareError> {
        if after {
            self.prepare_cell_at(slot, address, || {}, stop)
        } else {
            self.prepare_cell_at(slot, address, stop, || {})
        }
    }
}

#[cfg(test)]
impl Catalog {
    pub fn pool_occupancy(&self) -> (usize, usize) {
        (self.controls.occupied(), self.memberships.occupied())
    }
}
