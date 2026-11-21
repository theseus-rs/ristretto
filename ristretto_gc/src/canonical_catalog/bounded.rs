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
    Ordering, Outcome, Phase, Pooled, Preparation, PreparationError, PrepareError,
    Record as Payload,
};
use crate::canonical_pool as pool;
use std::ops::Deref;
mod budget;
mod quota;
use budget::Charge;
pub use budget::{Budget, Footprint};
const MAX_POOL: usize = 1 << 24;
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
}
impl Capacities {
    pub fn derive(registry: usize, mutators: usize, helpers: usize) -> Result<Self, ConfigError> {
        fn rounded(value: usize) -> Result<usize, ConfigError> {
            value
                .checked_next_power_of_two()
                .filter(|v| *v <= MAX_POOL)
                .ok_or(ConfigError::CapacityOverflow)
        }
        if registry == 0 {
            return Err(ConfigError::EmptyRegistry);
        }
        if helpers == 0 {
            return Err(ConfigError::NoHelpers);
        }
        let overflow = ConfigError::CapacityOverflow;
        let participants = mutators.checked_add(helpers).ok_or(overflow)?;
        let reserve = participants
            .checked_mul(8)
            .and_then(|v| v.checked_add(8))
            .ok_or(overflow)?;

        Ok(Self {
            registry,
            mutators,
            helpers,
            controls: rounded(reserve)?,
            memberships: rounded(registry.checked_add(reserve).ok_or(overflow)?)?,
            cells: rounded(
                participants
                    .checked_mul(2)
                    .and_then(|v| v.checked_add(2))
                    .and_then(|v| v.checked_add(registry))
                    .ok_or(overflow)?,
            )?,
            generation_limit: pool::MAX_GENERATION,
        })
    }
}
impl Capacities {
    pub fn registry(self) -> usize {
        self.registry
    }
    pub fn mutators(self) -> usize {
        self.mutators
    }
    pub fn helpers(self) -> usize {
        self.helpers
    }
    pub fn participants(self) -> usize {
        self.mutators + self.helpers
    }
    pub fn pools(self) -> (usize, usize, usize) {
        (self.controls, self.memberships, self.cells)
    }
    pub fn generation_limit(self) -> u64 {
        self.generation_limit
    }
    pub fn with_generation_limit(mut self, limit: u64) -> Result<Self, ConfigError> {
        if !(1..=pool::MAX_GENERATION).contains(&limit) {
            return Err(ConfigError::InvalidGenerationLimit);
        }
        self.generation_limit = limit;
        Ok(self)
    }
}

pub struct BoundedCatalog<R: Payload = ()> {
    raw: Catalog<R>,
    capacities: Capacities,
    identity: u64,
    mutators: AtomicUsize,
    helpers: AtomicUsize,
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
        let raw = Catalog::with_record_capacities_and_limit(
            capacities.registry,
            capacities.controls,
            capacities.memberships,
            capacities.cells,
            capacities.generation_limit,
        );
        Ok(CatalogHandle {
            inner: Some(Arc::new(Self {
                raw,
                capacities,
                identity,
                mutators: AtomicUsize::new(0),
                helpers: AtomicUsize::new(0),
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
            next_root_slot: 0,
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
}
impl Command {
    #[expect(
        clippy::unnecessary_wraps,
        reason = "retain the frozen object/phase command lookup interface"
    )]
    fn key(self) -> Option<RecordKey> {
        match self {
            Self::Publish(key) | Self::Abandon(key) => Some(key),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GuardedCommand {
    Publish(RecordKey),
    Abandon(RecordKey),
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

#[expect(
    clippy::large_enum_variant,
    reason = "request storage is prepaid inline; polls must not allocate boxes"
)]
enum RequestState<R: Payload = (), C: Copy = Command> {
    Idle,
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
    pub fn poll_in(&mut self, context: &R::Context) -> Progress {
        let catalog = &self.lease.owner;
        match &mut self.state {
            RequestState::Idle => Progress::Idle,
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
        if let RequestState::Yielding(_, yielding) = &self.state
            && !yielding.cancel()
        {
            return false;
        }
        if let RequestState::Attempt(_, attempt) = &self.state
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
        if let RequestState::Yielding(_, yielding) = &self.state {
            yielding.cancel();
        }
        if let RequestState::Attempt(_, attempt) = &self.state {
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
) -> Result<Pooled<Cell<R>>, RequestError> {
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
    let cell = lookup(catalog, command.key().expect("object command"))?;
    let preparation = match command {
        Command::Publish(_) => catalog.raw.begin_publish(snapshot, cell),
        Command::Abandon(_) => catalog.raw.begin_abandon(snapshot, cell),
    };
    if let Some(preparation) = preparation {
        return Ok(RequestState::Preparing(original, preparation));
    }
    let cell = lookup(catalog, command.key().expect("object command"))?;
    let membership = cell.membership().ok_or(RequestError::Contended)?;
    if membership != Membership::Prepared {
        return Err(RequestError::Conflict);
    }
    if matches!(command, Command::Publish(_)) {
        super::TraceState::publication(phase).map_err(trace_error)?;
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
impl<R: Payload> Commands<R> for GuardedCommand {
    fn numeric(self) -> Command {
        match self {
            Self::Publish(key) => Command::Publish(key),
            Self::Abandon(key) => Command::Abandon(key),
        }
    }
    fn start(
        self,
        catalog: &BoundedCatalog<R>,
        context: &R::Context,
    ) -> Result<RequestState<R, Self>, RequestError> {
        start(catalog, <Self as Commands<R>>::numeric(self), self, context)
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
