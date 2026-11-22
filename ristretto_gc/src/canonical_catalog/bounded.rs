//! Bounded participant admission and logical backing charges.
//! Request storage is introduced separately; no owning catalog state escapes.
#![expect(
    clippy::expect_used,
    reason = "admitted capacities and an unconsumed private handle remain valid until Drop"
)]
#![expect(
    clippy::unreachable,
    reason = "private request variants retain a same-catalog unfinished operation"
)]
use super::{
    Arc, AtomicU64, AtomicUsize, Attempt, Catalog, Cell, Creation, CreationError, Membership,
    Ordering, Outcome, Owned, Phase, Preparation, PreparationError, PrepareError,
    Record as Payload,
};
use crate::canonical_catalog::{OLD_DOMAIN, YOUNG_DOMAIN};
use crate::canonical_pool as pool;
use std::ops::Deref;
mod allocations;
mod edges;
pub use edges::*;
mod guarded;
pub use guarded::GuardedCommand;
mod roots;
use roots::RootLease;
pub use roots::*;
mod budget;
mod quota;
use budget::Charge;
pub use budget::{Budget, Footprint};

// Bounds on backing retained by root, reference and edge owners.
const RETAINED_PER_ROOT: usize = 2;
const RETAINED_PER_REFERENCE: usize = 2;
const RETAINED_PER_EDGE: usize = 2;
const TARGETS_PER_REFERENCE: usize = 6;

// Each pool also needs temporary backing for participants and fixed slack for
// in-flight operations. Reference and edge transactions use the same reserves.
struct PoolReserve {
    per_participant: usize,
    slack: usize,
}

impl PoolReserve {
    const CONTROLS: Self = Self::new(8, 8);
    const ORDINARY_CELLS: Self = Self::new(2, 2);
    const ROOT_CELLS: Self = Self::new(2, 2);
    const CELLS_WITH_ROOTS: Self = Self::new(6, 4);
    const MEMBERSHIPS_WITH_ROOTS: Self = Self::new(10, 8);
    const LINK_CELLS: Self = Self::new(6, 4);
    const LINK_STATES: Self = Self::new(20, 8);
    const ROOT_CELLS_WITH_LINKS: Self = Self::new(24, 16);
    const CELLS_WITH_LINKS: Self = Self::new(40, 24);
    const MEMBERSHIPS_WITH_LINKS: Self = Self::new(56, 32);

    const fn new(per_participant: usize, slack: usize) -> Self {
        Self {
            per_participant,
            slack,
        }
    }

    fn capacity(&self, participants: usize, retained: usize) -> Result<usize, ConfigError> {
        participants
            .checked_mul(self.per_participant)
            .and_then(|slots| slots.checked_add(self.slack))
            .and_then(|slots| slots.checked_add(retained))
            .and_then(usize::checked_next_power_of_two)
            .filter(|slots| *slots <= pool::MAX_CAPACITY)
            .ok_or(ConfigError::CapacityOverflow)
    }
}

static NEXT_CATALOG: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigError {
    EmptyRegistry,
    NoHelpers,
    CapacityOverflow,
    IdentityExhausted,
    BudgetExceeded,
    BudgetContended,
    InvalidGenerationLimit,
    InvalidEdgeLayout,
    UnsupportedReferences,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Capacities {
    registry: usize,
    mutators: usize,
    helpers: usize,
    controls: usize,
    memberships: usize,
    cells: usize,
    generation_limit: u64,
    root_slots: usize,
    root_cells: usize,
    reference_slots: usize,
    reference_cells: usize,
    reference_states: usize,
    edge_slots: usize,
    edge_cells: usize,
    edge_states: usize,
}
impl Capacities {
    pub fn derive(registry: usize, mutators: usize, helpers: usize) -> Result<Self, ConfigError> {
        if registry == 0 {
            return Err(ConfigError::EmptyRegistry);
        }
        if helpers == 0 {
            return Err(ConfigError::NoHelpers);
        }
        let overflow = ConfigError::CapacityOverflow;
        let participants = mutators.checked_add(helpers).ok_or(overflow)?;
        Ok(Self {
            registry,
            mutators,
            helpers,
            controls: PoolReserve::CONTROLS.capacity(participants, 0)?,
            memberships: PoolReserve::CONTROLS.capacity(participants, registry)?,
            cells: PoolReserve::ORDINARY_CELLS.capacity(participants, registry)?,
            generation_limit: pool::MAX_GENERATION,
            root_slots: 0,
            root_cells: 0,
            reference_slots: 0,
            reference_cells: 0,
            reference_states: 0,
            edge_slots: 0,
            edge_cells: 0,
            edge_states: 0,
        })
    }

    /// Reserve M independent root tokens and M discoverable slots. Root ownership
    /// can retain target cells after their object-registry slots are released.
    /// Each destructor stays charged to its participant or root-token quota.
    pub fn with_roots(mut self, roots: usize) -> Result<Self, ConfigError> {
        // Derive afresh so removing/changing root capacity is deterministic.
        let ordinary = Self::derive(self.registry, self.mutators, self.helpers)?;
        self.controls = ordinary.controls;
        self.memberships = ordinary.memberships;
        self.cells = ordinary.cells;
        self.root_slots = roots;
        self.root_cells = 0;
        if roots == 0 {
            return self.derive_link_pools();
        }
        let overflow = ConfigError::CapacityOverflow;
        let retained = roots.checked_mul(RETAINED_PER_ROOT).ok_or(overflow)?;
        let participants = self.participants();
        self.root_cells = PoolReserve::ROOT_CELLS.capacity(participants, retained)?;
        let retained = retained.checked_add(self.registry).ok_or(overflow)?;
        self.cells = PoolReserve::CELLS_WITH_ROOTS.capacity(participants, retained)?;
        self.memberships = PoolReserve::MEMBERSHIPS_WITH_ROOTS.capacity(participants, retained)?;
        self.derive_link_pools()
    }

    pub fn with_references(mut self, references: usize) -> Result<Self, ConfigError> {
        self.reference_slots = references;
        self.with_roots(self.root_slots)
    }

    /// E permanent edge slots and at most E independent creation owners. Live
    /// source-owned edges retain no Cell or catalog owner through their keys.
    pub fn with_edges(mut self, edges: usize) -> Result<Self, ConfigError> {
        self.edge_slots = edges;
        self.with_roots(self.root_slots)
    }

    fn derive_link_pools(mut self) -> Result<Self, ConfigError> {
        let participants = self.participants();
        let link_pools = |slots: usize, retained_per_slot: usize| {
            if slots == 0 {
                return Ok((0, 0));
            }
            let retained = slots
                .checked_mul(retained_per_slot)
                .ok_or(ConfigError::CapacityOverflow)?;
            Ok((
                PoolReserve::LINK_CELLS.capacity(participants, retained)?,
                PoolReserve::LINK_STATES.capacity(participants, retained)?,
            ))
        };
        (self.reference_cells, self.reference_states) =
            link_pools(self.reference_slots, RETAINED_PER_REFERENCE)?;
        (self.edge_cells, self.edge_states) = link_pools(self.edge_slots, RETAINED_PER_EDGE)?;
        if self.reference_slots != 0 || self.edge_slots != 0 {
            self.reserve_link_targets()?;
        }
        Ok(self)
    }

    // Reference and edge operations can retain target cells and memberships
    // through temporary root reads, traversals and stopped destructors.
    fn reserve_link_targets(&mut self) -> Result<(), ConfigError> {
        let overflow = ConfigError::CapacityOverflow;
        let roots = self
            .root_slots
            .checked_mul(RETAINED_PER_ROOT)
            .ok_or(overflow)?;
        let references = self
            .reference_slots
            .checked_mul(TARGETS_PER_REFERENCE)
            .ok_or(overflow)?;
        let retained = roots.checked_add(references).ok_or(overflow)?;
        let participants = self.participants();
        self.root_cells = self
            .root_cells
            .max(PoolReserve::ROOT_CELLS_WITH_LINKS.capacity(participants, retained)?);
        let retained = retained.checked_add(self.registry).ok_or(overflow)?;
        self.cells = self
            .cells
            .max(PoolReserve::CELLS_WITH_LINKS.capacity(participants, retained)?);
        self.memberships = self
            .memberships
            .max(PoolReserve::MEMBERSHIPS_WITH_LINKS.capacity(participants, retained)?);
        Ok(())
    }

    pub fn edge_slots(self) -> usize {
        self.edge_slots
    }

    pub fn edge_pools(self) -> (usize, usize) {
        (self.edge_cells, self.edge_states)
    }

    pub fn reference_slots(self) -> usize {
        self.reference_slots
    }

    pub fn reference_pools(self) -> (usize, usize) {
        (self.reference_cells, self.reference_states)
    }

    pub fn root_slots(self) -> usize {
        self.root_slots
    }

    pub fn root_pool(self) -> usize {
        self.root_cells
    }

    pub fn registry(self) -> usize {
        self.registry
    }

    pub fn participants(self) -> usize {
        self.mutators + self.helpers
    }

    pub fn mutators(self) -> usize {
        self.mutators
    }

    pub fn helpers(self) -> usize {
        self.helpers
    }

    pub fn pools(self) -> (usize, usize, usize) {
        (self.controls, self.memberships, self.cells)
    }

    /// Limit successful allocations per slot for validation. Setup encodes this
    /// budget in opaque generations. The default retains the full token width.
    pub fn with_generation_limit(mut self, limit: u64) -> Result<Self, ConfigError> {
        if !(1..=pool::MAX_GENERATION).contains(&limit) {
            return Err(ConfigError::InvalidGenerationLimit);
        }
        self.generation_limit = limit;
        Ok(self)
    }

    pub fn generation_limit(self) -> u64 {
        self.generation_limit
    }
}

pub struct BoundedCatalog<R: Payload = ()> {
    raw: Catalog<R>,
    capacities: Capacities,
    identity: u64,
    mutators: AtomicUsize,
    helpers: AtomicUsize,
    root_tokens: AtomicUsize,
    edge_tokens: AtomicUsize,
    // Last: catalog backing is destroyed before its charge is refunded.
    charge: Charge,
}
impl BoundedCatalog {
    #[expect(
        clippy::new_ret_no_self,
        reason = "opaque handles release their Arc before backing and charge"
    )]
    pub fn new(capacities: Capacities) -> Result<CatalogHandle, ConfigError> {
        Self::new_in(capacities, &Budget::new(usize::MAX))
    }

    pub fn new_in(
        capacities: Capacities,
        budget: &Arc<Budget>,
    ) -> Result<CatalogHandle, ConfigError> {
        Self::with_records_in(capacities, budget)
    }
}
impl<R: Payload> BoundedCatalog<R> {
    pub fn with_records_in(
        capacities: Capacities,
        budget: &Arc<Budget>,
    ) -> Result<CatalogHandle<R>, ConfigError> {
        Self::with_records_and_storage_in(capacities, budget, 0)
    }

    pub fn with_records_and_storage_in(
        capacities: Capacities,
        budget: &Arc<Budget>,
        inline_storage: usize,
    ) -> Result<CatalogHandle<R>, ConfigError> {
        Self::with_record_layout_in(capacities, budget, inline_storage, None)
    }

    pub fn with_indexed_records_and_storage_in(
        capacities: Capacities,
        budget: &Arc<Budget>,
        inline_storage: usize,
        degree: usize,
    ) -> Result<CatalogHandle<R>, ConfigError> {
        if degree == 0 || capacities.registry.checked_mul(degree) != Some(capacities.edge_slots) {
            return Err(ConfigError::InvalidEdgeLayout);
        }
        Self::with_record_layout_in(capacities, budget, inline_storage, Some(degree))
    }

    fn with_record_layout_in(
        capacities: Capacities,
        budget: &Arc<Budget>,
        inline_storage: usize,
        degree: Option<usize>,
    ) -> Result<CatalogHandle<R>, ConfigError> {
        let footprint = Footprint::for_records::<R>(capacities)?;
        let total = footprint
            .total()
            .checked_add(inline_storage)
            .ok_or(ConfigError::CapacityOverflow)?;
        let charge = budget.reserve(total)?;
        let identity = NEXT_CATALOG
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
                value.checked_add(1)
            })
            .map_err(|_| ConfigError::IdentityExhausted)?;
        let raw = Catalog::with_record_root_capacities_and_limit(
            capacities.registry,
            capacities.controls,
            capacities.memberships,
            capacities.cells,
            capacities.root_slots,
            capacities.root_cells,
            capacities.generation_limit,
        )
        .with_edge_capacities(
            capacities.edge_slots,
            capacities.edge_cells,
            capacities.edge_states,
            capacities.generation_limit,
        );
        let raw = match degree {
            Some(degree) => raw.with_source_edge_degree(degree),
            None => raw,
        };
        Ok(CatalogHandle {
            inner: Some(Arc::new(Self {
                raw,
                capacities,
                identity,
                mutators: AtomicUsize::new(0),
                helpers: AtomicUsize::new(0),
                root_tokens: AtomicUsize::new(0),
                edge_tokens: AtomicUsize::new(0),
                charge,
            })),
        })
    }

    pub fn footprint(&self) -> Footprint {
        Footprint::for_records::<R>(self.capacities).expect("validated capacities")
    }

    pub fn charged_bytes(&self) -> usize {
        self.charge.bytes()
    }

    pub fn reserved_storage_bytes(&self) -> usize {
        self.charge.bytes() - self.footprint().total()
    }

    pub fn capacities(&self) -> Capacities {
        self.capacities
    }

    pub fn reserved_edge_tokens(&self) -> usize {
        self.edge_tokens.load(Ordering::Acquire)
    }

    pub fn reserved_root_tokens(&self) -> usize {
        self.root_tokens.load(Ordering::Acquire)
    }

    pub fn registered(&self) -> (usize, usize) {
        (
            self.mutators.load(Ordering::Acquire),
            self.helpers.load(Ordering::Acquire),
        )
    }

    fn counter(&self, role: Role) -> (&AtomicUsize, usize) {
        match role {
            Role::Mutator => (&self.mutators, self.capacities.mutators),
            Role::Helper => (&self.helpers, self.capacities.helpers),
        }
    }
}

/// Final release frees the outer `Arc` before catalog backing, then refunds charge.
pub struct CatalogHandle<R: Payload = ()> {
    inner: Option<Arc<BoundedCatalog<R>>>,
}
impl<R: Payload> Clone for CatalogHandle<R> {
    fn clone(&self) -> Self {
        Self {
            inner: Some(self.inner.as_ref().expect("live handle").clone()),
        }
    }
}
impl<R: Payload> Deref for CatalogHandle<R> {
    type Target = BoundedCatalog<R>;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().expect("live handle")
    }
}
impl<R: Payload> Drop for CatalogHandle<R> {
    fn drop(&mut self) {
        if let Some(catalog) = Arc::into_inner(self.inner.take().expect("live handle")) {
            drop(catalog);
        }
    }
}
impl CatalogHandle {
    pub fn register_mutator(&self) -> Result<Participant, RegisterError> {
        self.register(Role::Mutator)
    }

    pub fn register_helper(&self) -> Result<Participant, RegisterError> {
        self.register(Role::Helper)
    }
}
impl<R: Payload> CatalogHandle<R> {
    fn register<C: Copy>(&self, role: Role) -> Result<Participant<R, C>, RegisterError> {
        let owner = self.clone();
        let (count, limit) = self.counter(role);
        let before = quota::acquire(count, limit)?;
        Ok(Participant {
            state: RequestState::Idle,
            next_object_slot: before % self.capacities.registry,
            next_root_slot: before.checked_rem(self.capacities.root_slots).unwrap_or(0),
            lease: Lease {
                owner,
                role,
                #[cfg(test)]
                before_release: None,
            },
        })
    }
}
#[derive(Clone, Copy)]
enum Role {
    Mutator,
    Helper,
}
struct Lease<R: Payload> {
    owner: CatalogHandle<R>,
    role: Role,
    #[cfg(test)]
    before_release: Option<Box<dyn FnOnce() + Send + Sync>>,
}
impl<R: Payload> Drop for Lease<R> {
    fn drop(&mut self) {
        #[cfg(test)]
        if let Some(hook) = self.before_release.take() {
            hook();
        }
        quota::release(self.owner.counter(self.role).0);
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegisterError {
    Limit,
    Contended,
}
#[cfg(test)]
mod admission_tests;
#[cfg(all(test, not(target_family = "wasm")))]
mod quota_models;

impl<R: Payload> CatalogHandle<R> {
    pub fn register_record_mutator(&self) -> Result<Participant<R, GuardedCommand>, RegisterError> {
        self.register(Role::Mutator)
    }

    pub fn register_record_helper(&self) -> Result<Participant<R, GuardedCommand>, RegisterError> {
        self.register(Role::Helper)
    }
}
/// Numeric identity only; retaining/copying keys retains no backing or value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecordKey {
    catalog: u64,
    identity: u64,
    slot: usize,
}
impl RecordKey {
    pub fn identity(self) -> u64 {
        self.identity
    }

    pub fn slot(self) -> usize {
        self.slot
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Record {
    pub key: RecordKey,
    pub address: usize,
    pub membership: Membership,
    pub abandon_requested: bool,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    Publish(RecordKey),
    Abandon(RecordKey),
    Promote(RecordKey),
    Advance {
        domain: usize,
    },
    Start {
        domain: usize,
        mode: super::DomainMode,
    },
    Finish {
        domain: usize,
    },
    Shade {
        key: RecordKey,
        domain: usize,
        retention: super::TraceRetention,
    },
    TraceAcknowledge {
        key: RecordKey,
        domain: usize,
        ticket: u64,
    },
    TraceRetire {
        key: RecordKey,
        domain: usize,
    },
}
impl Command {
    fn key(self) -> Option<RecordKey> {
        match self {
            Self::Publish(key)
            | Self::Abandon(key)
            | Self::Promote(key)
            | Self::Shade { key, .. }
            | Self::TraceAcknowledge { key, .. }
            | Self::TraceRetire { key, .. } => Some(key),
            Self::Advance { .. } | Self::Start { .. } | Self::Finish { .. } => None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestError {
    Busy,
    RootLimit,
    ReferenceLimit,
    EdgeLimit,
    ConstructionLimit,
    Foreign,
    Stale,
    InvalidSlot,
    InvalidAddress,
    InvalidDomain,
    Conflict,
    /// A copied observation already has at least the requested retention. This
    /// is not a new committed root, payload access or phase-stability certificate.
    AlreadyMarked,
    Exhausted,
    Contended,
    /// Preparation observed permanently exhausted slot backing. This is not a
    /// whole-pool exhaustion certificate, nor cancellation of an installed effect.
    GenerationExhausted,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Progress {
    Idle,
    Pending,
    Committed,
    Created(RecordKey),
    RootCreated(RootKey),
    EdgeCreated(EdgeKey),
    Cancelled,
    Rejected(RequestError),
}

// A phase-only proposal uses one backing slot. Keep the two intervening poll
// returns that effect-bearing preparation needs, so fast phase changes do not
// repeatedly supersede those requests. This private window never waits for an
// owner or retains a global gate; its next two polls always exhaust it.
struct PhaseYield<R: Payload> {
    attempt: Option<Attempt<R>>,
    first: bool,
}
impl<R: Payload> PhaseYield<R> {
    fn new(attempt: Attempt<R>) -> Self {
        Self {
            attempt: Some(attempt),
            first: true,
        }
    }

    #[expect(
        clippy::unwrap_in_result,
        reason = "the private second phase poll consumes its one retained attempt"
    )]
    fn poll(&mut self) -> Option<Attempt<R>> {
        if self.first {
            self.first = false;
            None
        } else {
            Some(self.attempt.take().expect("private phase proposal"))
        }
    }

    fn cancel(&self) -> bool {
        self.attempt
            .as_ref()
            .expect("private phase proposal")
            .cancel()
    }
}

enum RequestState<R: Payload = (), C: Copy = Command> {
    Idle,
    Allocation(allocations::Request<R>),
    Edge(Request<R>),
    Starting(C),
    StartingRecord {
        slot: usize,
        address: usize,
        record: Option<R>,
    },
    Creating(Creation<R>),
    Preparing(C, Preparation<R>),
    Yielding(C, PhaseYield<R>),
    Attempt(C, Attempt<R>),
    StartingRootCreation {
        slot: usize,
        target: RecordKey,
        lease: Option<RootLease<R>>,
    },
    CreatingRoot {
        creation: super::RootCreation<R>,
        lease: Option<RootLease<R>>,
    },
    RootReady(RootHandle<R>),
    StartingRoot(RootCommand),
    PreparingRoot(RootCommand, Preparation<R>),
    YieldingRoot(RootCommand, PhaseYield<R>),
    AttemptRoot(RootCommand, Attempt<R>),
}
/// Request state drops before the lease releases its registration quota.
pub struct Participant<R: Payload = (), C: Copy = Command> {
    state: RequestState<R, C>,
    next_object_slot: usize,
    next_root_slot: usize,
    lease: Lease<R>,
}
impl Participant {
    pub fn poll(&mut self) -> Progress {
        self.poll_in(&())
    }

    pub fn phase(&mut self) -> Result<Phase, RequestError> {
        self.phase_in(&())
    }
}
#[expect(
    private_bounds,
    reason = "sealed commands prevent escaping quota-owned raw state"
)]
impl<R: Payload, C: Commands<R>> Participant<R, C> {
    fn idle(&self) -> Result<(), RequestError> {
        if matches!(self.state, RequestState::Idle) {
            Ok(())
        } else {
            Err(RequestError::Busy)
        }
    }

    pub fn submit(&mut self, command: C) -> Result<(), RequestError> {
        self.idle()?;
        if command
            .numeric()
            .key()
            .is_some_and(|key| key.catalog != self.lease.owner.identity)
        {
            return Err(RequestError::Foreign);
        }
        if self.lease.owner.capacities.edge_slots != 0 {
            match command.numeric() {
                Command::TraceAcknowledge {
                    key,
                    domain,
                    ticket,
                } => {
                    self.state = RequestState::Edge(Request::Traverse {
                        key,
                        domain,
                        ticket,
                    });
                    return Ok(());
                }
                Command::Start { mode, .. } if mode != super::DomainMode::Full => {
                    return Err(RequestError::Conflict);
                }
                _ => {}
            }
        }
        self.state = RequestState::Starting(command);
        Ok(())
    }

    pub fn submit_record(
        &mut self,
        slot: usize,
        address: usize,
        record: R,
    ) -> Result<(), SubmitFailure<R>> {
        if let Err(error) = self.idle() {
            return Err(SubmitFailure { error, record });
        }
        self.state = RequestState::StartingRecord {
            slot,
            address,
            record: Some(record),
        };
        Ok(())
    }

    #[expect(
        clippy::too_many_lines,
        reason = "keep the frozen bounded state transitions in one exhaustive dispatcher"
    )]
    pub fn poll_in(&mut self, context: &R::Context) -> Progress {
        let catalog = &self.lease.owner;
        match &mut self.state {
            RequestState::Idle => Progress::Idle,
            RequestState::Edge(request) => match request.poll(catalog, context) {
                Step::Pending => Progress::Pending,
                Step::Ready(key) => Progress::EdgeCreated(key),
                Step::Complete(progress) => {
                    self.state = RequestState::Idle;
                    progress
                }
            },
            RequestState::Allocation(request) => match request.poll(catalog, context) {
                allocations::Step::Pending => Progress::Pending,
                allocations::Step::Ready(key) => Progress::RootCreated(key),
                allocations::Step::Complete(progress) => {
                    self.state = RequestState::Idle;
                    progress
                }
            },
            RequestState::StartingRootCreation {
                slot,
                target,
                lease,
            } => {
                let creation = lookup(catalog, *target).and_then(|target| {
                    catalog
                        .raw
                        .begin_create_root(*slot, target)
                        .map_err(roots::prepare_error)
                });
                match creation {
                    Ok(creation) => {
                        self.state = RequestState::CreatingRoot {
                            creation,
                            lease: lease.take(),
                        }
                    }
                    Err(RequestError::Contended) => {}
                    Err(error) => {
                        self.state = RequestState::Idle;
                        return Progress::Rejected(error);
                    }
                }
                Progress::Pending
            }
            RequestState::CreatingRoot { creation, lease } => match creation.poll(&catalog.raw) {
                Ok(Some(token)) => {
                    let root = RootHandle::new(token, lease.take().expect("root reservation"));
                    let key = root.key();
                    self.state = RequestState::RootReady(root);
                    Progress::RootCreated(key)
                }
                Ok(None) => Progress::Pending,
                Err(CreationError::Occupied) => {
                    self.state = RequestState::Idle;
                    Progress::Rejected(RequestError::Conflict)
                }
                Err(CreationError::GenerationExhausted) => {
                    self.state = RequestState::Idle;
                    Progress::Rejected(RequestError::GenerationExhausted)
                }
                Err(_) => unreachable!("same unfinished root creation"),
            },
            RequestState::RootReady(root) => Progress::RootCreated(root.key()),
            RequestState::StartingRoot(command) => match roots::start(catalog, context, *command) {
                Ok(preparation) => {
                    self.state = RequestState::PreparingRoot(*command, preparation);
                    Progress::Pending
                }
                Err(RequestError::Contended) => Progress::Pending,
                Err(error) => {
                    self.state = RequestState::Idle;
                    Progress::Rejected(error)
                }
            },
            RequestState::PreparingRoot(command, preparation) => {
                match preparation.poll(&catalog.raw) {
                    Ok(Some(attempt)) => {
                        self.state = if attempt.is_phase_only() {
                            RequestState::YieldingRoot(*command, PhaseYield::new(attempt))
                        } else {
                            RequestState::AttemptRoot(*command, attempt)
                        };
                    }
                    Ok(None) => {}
                    Err(PreparationError::GenerationExhausted) => {
                        self.state = RequestState::Idle;
                        return Progress::Rejected(RequestError::GenerationExhausted);
                    }
                    Err(_) => unreachable!("same unfinished root preparation"),
                }
                Progress::Pending
            }
            RequestState::YieldingRoot(command, yielding) => {
                if let Some(attempt) = yielding.poll() {
                    self.state = RequestState::AttemptRoot(*command, attempt);
                }
                Progress::Pending
            }
            RequestState::AttemptRoot(command, attempt) => {
                match catalog.raw.commit_in(attempt, context) {
                    Outcome::Committed => {
                        self.state = RequestState::Idle;
                        Progress::Committed
                    }
                    Outcome::Cancelled => {
                        self.state = RequestState::Idle;
                        Progress::Cancelled
                    }
                    Outcome::Pending => Progress::Pending,
                    Outcome::Retry => {
                        self.state = RequestState::StartingRoot(*command);
                        Progress::Pending
                    }
                }
            }
            RequestState::Starting(command) => match command.start(catalog, context) {
                Ok(next) => {
                    self.state = next;
                    Progress::Pending
                }
                Err(RequestError::Contended) => Progress::Pending,
                Err(error) => {
                    self.state = RequestState::Idle;
                    Progress::Rejected(error)
                }
            },
            RequestState::StartingRecord {
                slot,
                address,
                record,
            } => {
                match catalog.raw.begin_create_with(
                    *slot,
                    *address,
                    record.take().expect("retained record"),
                ) {
                    Ok(creation) => self.state = RequestState::Creating(creation),
                    Err(failure) => {
                        let error = prepare_error(failure.error);
                        if error == RequestError::Contended {
                            *record = Some(failure.record);
                        } else {
                            self.state = RequestState::Idle;
                            return Progress::Rejected(error);
                        }
                    }
                }
                Progress::Pending
            }
            RequestState::Creating(creation) => match creation.poll(&catalog.raw) {
                Ok(Some(cell)) => {
                    let record = key(catalog, &cell);
                    self.state = RequestState::Idle;
                    Progress::Created(record)
                }
                Ok(None) => Progress::Pending,
                Err(CreationError::Occupied) => {
                    self.state = RequestState::Idle;
                    Progress::Rejected(RequestError::Conflict)
                }
                Err(CreationError::GenerationExhausted) => {
                    self.state = RequestState::Idle;
                    Progress::Rejected(RequestError::GenerationExhausted)
                }
                Err(_) => unreachable!("participant owns an unfinished same-catalog creation"),
            },
            RequestState::Preparing(command, preparation) => {
                match preparation.poll(&catalog.raw) {
                    Ok(Some(attempt)) => {
                        self.state = if attempt.is_phase_only() {
                            RequestState::Yielding(*command, PhaseYield::new(attempt))
                        } else {
                            RequestState::Attempt(*command, attempt)
                        };
                    }
                    Ok(None) => {}
                    Err(PreparationError::GenerationExhausted) => {
                        self.state = RequestState::Idle;
                        return Progress::Rejected(RequestError::GenerationExhausted);
                    }
                    Err(_) => unreachable!("same unfinished preparation"),
                }
                Progress::Pending
            }
            RequestState::Yielding(command, yielding) => {
                if let Some(attempt) = yielding.poll() {
                    self.state = RequestState::Attempt(*command, attempt);
                }
                Progress::Pending
            }
            RequestState::Attempt(command, attempt) => {
                match catalog.raw.commit_in(attempt, context) {
                    Outcome::Committed => {
                        self.state = RequestState::Idle;
                        Progress::Committed
                    }
                    Outcome::Cancelled => {
                        self.state = RequestState::Idle;
                        Progress::Cancelled
                    }
                    Outcome::Pending => Progress::Pending,
                    Outcome::Retry => {
                        self.state = RequestState::Starting(*command);
                        Progress::Pending
                    }
                }
            }
        }
    }

    /// False means commit already won; retain the request to observe its result.
    pub fn cancel(&mut self) -> bool {
        if let RequestState::Edge(request) = &self.state
            && !request.cancel()
        {
            return false;
        }
        if let RequestState::Allocation(request) = &self.state
            && !request.cancel()
        {
            return false;
        }
        if let RequestState::Yielding(_, yielding) | RequestState::YieldingRoot(_, yielding) =
            &self.state
            && !yielding.cancel()
        {
            return false;
        }
        if let RequestState::Attempt(_, attempt) | RequestState::AttemptRoot(_, attempt) =
            &self.state
            && !attempt.cancel()
        {
            return false;
        }
        self.state = RequestState::Idle;
        true
    }

    /// Copy possible outcomes without helping or granting an owning snapshot.
    /// The temporary owner can run destruction on drop; this is not a cycle count.
    pub fn possible_phase_outcomes(&mut self) -> Result<[Phase; 2], RequestError> {
        self.idle()?;
        self.lease
            .owner
            .raw
            .possible_phase_outcomes()
            .ok_or(RequestError::Contended)
    }

    pub fn phase_in(&mut self, context: &R::Context) -> Result<Phase, RequestError> {
        self.idle()?;
        self.lease
            .owner
            .raw
            .phase_in(context)
            .ok_or(RequestError::Contended)
    }

    /// Drop this observation without waiting; an installed winner stays helpable.
    pub fn discard_request(&mut self) {
        let _ = self.cancel();
        self.state = RequestState::Idle;
    }

    pub fn observe(&mut self, record: RecordKey) -> Result<Record, RequestError> {
        self.idle()?;
        let catalog = &self.lease.owner;
        let cell = lookup(catalog, record)?;
        record_info(catalog, &cell)
    }

    pub fn discover(&mut self, slot: usize) -> Result<Option<Record>, RequestError> {
        self.idle()?;
        let catalog = &self.lease.owner;
        if slot >= catalog.capacities.registry {
            return Err(RequestError::InvalidSlot);
        }
        catalog
            .raw
            .discover(slot)
            .map_err(|_| RequestError::Contended)?
            .map(|cell| record_info(catalog, &cell))
            .transpose()
    }

    pub fn release_retired(&mut self, record: RecordKey) -> Result<bool, RequestError> {
        self.idle()?;
        let catalog = &self.lease.owner;
        let cell = lookup(catalog, record)?;
        Ok(catalog.raw.release_retired(&cell))
    }
}
impl<R: Payload, C: Copy> Drop for Participant<R, C> {
    fn drop(&mut self) {
        if let RequestState::Edge(request) = &self.state {
            request.cancel();
        }
        if let RequestState::Allocation(request) = &self.state {
            request.cancel();
        }
        if let RequestState::Yielding(_, yielding) | RequestState::YieldingRoot(_, yielding) =
            &self.state
        {
            yielding.cancel();
        }
        if let RequestState::Attempt(_, attempt) | RequestState::AttemptRoot(_, attempt) =
            &self.state
        {
            attempt.cancel();
        }
        // Automatic field drop destroys request storage before its lease.
    }
}

fn key<R: Payload>(catalog: &BoundedCatalog<R>, cell: &Cell<R>) -> RecordKey {
    RecordKey {
        catalog: catalog.identity,
        identity: cell.identity,
        slot: cell.slot(),
    }
}

fn record_info<R: Payload>(
    catalog: &BoundedCatalog<R>,
    cell: &Cell<R>,
) -> Result<Record, RequestError> {
    Ok(Record {
        key: key(catalog, cell),
        address: cell.address,
        membership: cell.membership().ok_or(RequestError::Contended)?,
        abandon_requested: cell.abandon_requested(),
    })
}

fn lookup<R: Payload>(
    catalog: &BoundedCatalog<R>,
    record: RecordKey,
) -> Result<Owned<Cell<R>>, RequestError> {
    if record.catalog != catalog.identity {
        return Err(RequestError::Foreign);
    }
    let cell = catalog
        .raw
        .discover(record.slot)
        .map_err(|_| RequestError::Contended)?
        .ok_or(RequestError::Stale)?;
    if cell.identity != record.identity {
        return Err(RequestError::Stale);
    }
    Ok(cell)
}

trait Commands<R: Payload>: Copy {
    fn numeric(self) -> Command;

    fn start(
        self,
        catalog: &BoundedCatalog<R>,
        context: &R::Context,
    ) -> Result<RequestState<R, Self>, RequestError>;
}

fn start<R: Payload, C: Copy>(
    catalog: &BoundedCatalog<R>,
    command: Command,
    original: C,
    context: &R::Context,
) -> Result<RequestState<R, C>, RequestError> {
    let snapshot = catalog
        .raw
        .snapshot_in(context)
        .ok_or(RequestError::Contended)?;
    let phase = snapshot.phase();
    if phase.revision == u64::MAX {
        return Err(RequestError::Exhausted);
    }
    let cell = command
        .key()
        .map(|record| lookup(catalog, record))
        .transpose()?;
    let preparation = match command {
        Command::Publish(_) => catalog
            .raw
            .begin_publish(snapshot, cell.expect("object command")),
        Command::Abandon(_) => catalog
            .raw
            .begin_abandon(snapshot, cell.expect("object command")),
        Command::Promote(_) => catalog
            .raw
            .begin_promote(snapshot, cell.expect("object command")),
        Command::Advance { domain } => catalog.raw.begin_advance(snapshot, domain),
        Command::Start { domain, mode } => catalog.raw.begin_start(snapshot, domain, mode),
        Command::Finish { domain } => catalog.raw.begin_finish(snapshot, domain),
        Command::Shade {
            domain, retention, ..
        } => catalog
            .raw
            .begin_shade(snapshot, cell.expect("object command"), domain, retention),
        Command::TraceAcknowledge { domain, ticket, .. } => catalog.raw.begin_trace_acknowledge(
            snapshot,
            cell.expect("object command"),
            domain,
            ticket,
        ),
        Command::TraceRetire { domain, .. } => {
            catalog
                .raw
                .begin_trace_retire(snapshot, cell.expect("object command"), domain)
        }
    };
    if let Some(preparation) = preparation {
        return Ok(RequestState::Preparing(original, preparation));
    }
    diagnose_failure(catalog, command, phase)
}

#[expect(
    clippy::indexing_slicing,
    reason = "domain indexes are checked before fixed-array access"
)]
fn diagnose_failure<R: Payload, C: Copy>(
    catalog: &BoundedCatalog<R>,
    command: Command,
    phase: Phase,
) -> Result<RequestState<R, C>, RequestError> {
    let cell = command
        .key()
        .map(|record| lookup(catalog, record))
        .transpose()?;
    let membership = cell
        .as_ref()
        .map(|cell| cell.membership().ok_or(RequestError::Contended))
        .transpose()?;
    match command {
        Command::Advance { domain } => {
            let entry = phase
                .domains
                .get(domain)
                .ok_or(RequestError::InvalidDomain)?;
            if entry.mode == crate::canonical_catalog::DomainMode::Inactive
                || entry.pending != 0
                || entry.floor >= crate::canonical_catalog::RETIREMENT_FLOOR
                || phase.root_cursors[domain] != catalog.raw.root_capacity()
            {
                return Err(RequestError::Conflict);
            }
        }
        Command::Abandon(_) => {
            if membership != Some(Membership::Prepared) {
                return Err(RequestError::Conflict);
            }
        }
        Command::Start { domain, mode } => {
            let entry = phase
                .domains
                .get(domain)
                .ok_or(RequestError::InvalidDomain)?;
            if mode == crate::canonical_catalog::DomainMode::Inactive
                || (mode == crate::canonical_catalog::DomainMode::Old && domain != OLD_DOMAIN)
                || (mode == crate::canonical_catalog::DomainMode::Young && domain != YOUNG_DOMAIN)
                || entry.mode != crate::canonical_catalog::DomainMode::Inactive
                || entry.pending != 0
            {
                return Err(RequestError::Conflict);
            }
            if entry.epoch == u64::MAX {
                return Err(RequestError::Exhausted);
            }
        }
        Command::Finish { domain } => {
            let entry = phase
                .domains
                .get(domain)
                .ok_or(RequestError::InvalidDomain)?;
            if entry.mode == crate::canonical_catalog::DomainMode::Inactive
                || entry.pending != 0
                || entry.floor != crate::canonical_catalog::RETIREMENT_FLOOR
                || phase.root_cursors[domain] != catalog.raw.root_capacity()
            {
                return Err(RequestError::Conflict);
            }
        }
        Command::Publish(_) => {
            if membership != Some(Membership::Prepared) {
                return Err(RequestError::Conflict);
            }
            crate::canonical_catalog::TraceState::publication(phase).map_err(trace_error)?;
        }
        Command::Shade {
            domain, retention, ..
        } => {
            let Some(Membership::Tracing(marks)) = membership else {
                return Err(RequestError::Conflict);
            };
            marks.shade(phase, domain, retention).map_err(trace_error)?;
        }
        Command::TraceAcknowledge { domain, ticket, .. } => {
            let Some(Membership::Tracing(marks)) = membership else {
                return Err(RequestError::Conflict);
            };
            marks
                .acknowledge(phase, domain, ticket)
                .map_err(trace_error)?;
        }
        Command::TraceRetire { domain, .. } => {
            let Some(Membership::Tracing(marks)) = membership else {
                return Err(RequestError::Conflict);
            };
            marks.can_retire(phase, domain).map_err(trace_error)?;
        }
        Command::Promote(_) => {
            let Some(Membership::Tracing(marks)) = membership else {
                return Err(RequestError::Conflict);
            };
            marks.promote(phase).map_err(trace_error)?;
        }
    }
    if catalog.raw.generation_exhaustion_observed() {
        Err(RequestError::GenerationExhausted)
    } else {
        Err(RequestError::Contended)
    }
}

impl Commands<()> for Command {
    fn numeric(self) -> Command {
        self
    }

    fn start(self, catalog: &BoundedCatalog, (): &()) -> Result<RequestState, RequestError> {
        start(catalog, self, self, &())
    }
}
/// Rejection before request ownership transfers retains the caller's record.
pub struct SubmitFailure<R> {
    pub error: RequestError,
    pub record: R,
}
impl<R> std::fmt::Debug for SubmitFailure<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubmitFailure")
            .field("error", &self.error)
            .finish_non_exhaustive()
    }
}

fn prepare_error(error: PrepareError) -> RequestError {
    match error {
        PrepareError::InvalidSlot => RequestError::InvalidSlot,
        PrepareError::InvalidAddress => RequestError::InvalidAddress,
        PrepareError::Occupied => RequestError::Conflict,
        PrepareError::IdentityExhausted => RequestError::Exhausted,
        PrepareError::Contended | PrepareError::BackingUnavailable => RequestError::Contended,
    }
}

fn trace_error(error: super::TraceError) -> RequestError {
    match error {
        super::TraceError::InvalidDomain => RequestError::InvalidDomain,
        super::TraceError::Conflict => RequestError::Conflict,
        super::TraceError::AlreadyMarked => RequestError::AlreadyMarked,
        super::TraceError::Exhausted => RequestError::Exhausted,
    }
}

#[cfg(test)]
#[path = "bounded/request_tests.rs"]
mod request_tests;

#[cfg(test)]
#[path = "bounded/guarded_tests.rs"]
mod guarded_tests;
