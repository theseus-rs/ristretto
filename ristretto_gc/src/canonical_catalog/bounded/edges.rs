//! Quota-bound creation owners and source-owned, non-owning edge identities.
use super::{
    Attempt, BoundedCatalog, CatalogHandle, Commands, CreationError, Membership, Ordering, Outcome,
    Participant, Payload, Phase, Pooled, Preparation, PreparationError, Progress, RecordKey,
    RequestError, RequestState, RootHandle, RootKey, roots, trace_error,
};
use crate::canonical_catalog::reference::ObjectKey;
use crate::canonical_catalog::{
    EdgeCreation, EdgeMembership, EdgePrepareError, EdgeSlot, EdgeToken, EdgeTraversal,
    EdgeTraversalError, EdgeTraversalProgress, Root, RootStatus,
};

#[cfg(test)]
thread_local! {
    static AFTER_SOURCE_OBSERVATION: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = const {
        std::cell::RefCell::new(None)
    };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EdgeKey {
    catalog: u64,
    identity: u64,
    slot: usize,
}
impl EdgeKey {
    pub fn identity(self) -> u64 {
        self.identity
    }
    pub fn slot(self) -> usize {
        self.slot
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EdgeObservation {
    pub key: EdgeKey,
    pub state: EdgeMembership,
    pub removal_requested: bool,
}
/// A temporary external creation owner, never a payload field. Publication can
/// transfer it into a non-owning `EdgeKey`; otherwise `Drop` requests removal.
pub struct EdgeHandle<R: Payload = ()> {
    token: EdgeToken,
    lease: EdgeLease<R>,
}
impl<R: Payload> EdgeHandle<R> {
    pub fn key(&self) -> EdgeKey {
        key(&self.lease.owner, &self.token)
    }
}
pub struct EdgeReleaseFailure<R: Payload> {
    pub error: RequestError,
    pub edge: EdgeHandle<R>,
}
pub(super) struct EdgeLease<R: Payload> {
    owner: CatalogHandle<R>,
}
impl<R: Payload> EdgeLease<R> {
    fn reserve(owner: &CatalogHandle<R>) -> Result<Self, RequestError> {
        let before = owner.edge_tokens.load(Ordering::Relaxed);
        if before >= owner.capacities.edge_slots {
            return Err(RequestError::EdgeLimit);
        }
        owner
            .edge_tokens
            .compare_exchange(before, before + 1, Ordering::Acquire, Ordering::Relaxed)
            .map_err(|_| RequestError::Contended)?;
        Ok(Self {
            owner: owner.clone(),
        })
    }
}
impl<R: Payload> Drop for EdgeLease<R> {
    fn drop(&mut self) {
        let previous = self.owner.edge_tokens.fetch_sub(1, Ordering::Release);
        debug_assert!(previous > 0, "duplicate edge quota refund");
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EdgeCommand {
    Publish {
        edge: EdgeKey,
        source: RootKey,
        target: Option<RootKey>,
    },
    Replace {
        edge: EdgeKey,
        source: RootKey,
        target: Option<RootKey>,
    },
    RootTarget {
        edge: EdgeKey,
        source: RootKey,
        child: RootKey,
    },
    RemoveRequested(EdgeKey),
    RemoveOrphan(EdgeKey),
}
impl EdgeCommand {
    fn key(self) -> EdgeKey {
        match self {
            Self::Publish { edge, .. }
            | Self::Replace { edge, .. }
            | Self::RootTarget { edge, .. }
            | Self::RemoveRequested(edge)
            | Self::RemoveOrphan(edge) => edge,
        }
    }
}
pub(super) enum Request<R: Payload> {
    CreatingStart {
        slot: usize,
        lease: Option<EdgeLease<R>>,
    },
    Creating {
        creation: EdgeCreation,
        lease: Option<EdgeLease<R>>,
    },
    Ready(EdgeHandle<R>),
    Start(EdgeCommand),
    Preparing(EdgeCommand, Preparation<R>),
    Attempt(EdgeCommand, Attempt<R>),
    Traverse {
        key: RecordKey,
        domain: usize,
        ticket: u64,
    },
    Traversing(EdgeTraversal<R>),
}
pub(super) enum Step {
    Pending,
    Ready(EdgeKey),
    Complete(Progress),
}
fn rejected(error: RequestError) -> Step {
    if error == RequestError::Contended {
        Step::Pending
    } else {
        Step::Complete(Progress::Rejected(error))
    }
}
fn retry_error<R: Payload>(owner: &BoundedCatalog<R>) -> RequestError {
    if owner.raw.generation_exhaustion_observed() {
        RequestError::GenerationExhausted
    } else {
        RequestError::Contended
    }
}
impl<R: Payload> Request<R> {
    pub(super) fn cancel(&self) -> bool {
        match self {
            Self::Attempt(_, attempt) => attempt.cancel(),
            Self::Traversing(traversal) => traversal.cancel(),
            _ => true,
        }
    }
    pub(super) fn poll(&mut self, owner: &CatalogHandle<R>, context: &R::Context) -> Step {
        match self {
            Self::CreatingStart { slot, lease } => match owner.raw.begin_create_edge(*slot) {
                Ok(creation) => {
                    *self = Self::Creating {
                        creation,
                        lease: lease.take(),
                    };
                    Step::Pending
                }
                Err(error) => rejected(match error {
                    EdgePrepareError::InvalidSlot => RequestError::InvalidSlot,
                    EdgePrepareError::Occupied => RequestError::Conflict,
                    EdgePrepareError::Contended => RequestError::Contended,
                    EdgePrepareError::IdentityExhausted => RequestError::Exhausted,
                }),
            },
            Self::Creating { creation, lease } => match creation.poll(&owner.raw) {
                Ok(Some(token)) => {
                    let handle = EdgeHandle {
                        token,
                        lease: lease.take().expect("edge reservation"),
                    };
                    let key = handle.key();
                    *self = Self::Ready(handle);
                    Step::Ready(key)
                }
                Ok(None) => Step::Pending,
                Err(CreationError::Occupied) => rejected(RequestError::Conflict),
                Err(CreationError::GenerationExhausted) => {
                    rejected(RequestError::GenerationExhausted)
                }
                Err(_) => unreachable!("same unfinished edge creation"),
            },
            Self::Ready(handle) => Step::Ready(handle.key()),
            Self::Start(command) => match start(owner, context, *command) {
                Ok(preparation) => {
                    *self = Self::Preparing(*command, preparation);
                    Step::Pending
                }
                Err(error) => rejected(error),
            },
            Self::Preparing(command, preparation) => {
                match preparation.poll(&owner.raw) {
                    Ok(Some(attempt)) => *self = Self::Attempt(*command, attempt),
                    Ok(None) => {}
                    Err(PreparationError::GenerationExhausted) => {
                        return rejected(RequestError::GenerationExhausted);
                    }
                    Err(_) => unreachable!("same unfinished edge preparation"),
                }
                Step::Pending
            }
            Self::Attempt(command, attempt) => match owner.raw.commit_in(attempt, context) {
                Outcome::Pending => Step::Pending,
                Outcome::Committed => Step::Complete(Progress::Committed),
                Outcome::Cancelled => Step::Complete(Progress::Cancelled),
                Outcome::Retry => {
                    *self = Self::Start(*command);
                    Step::Pending
                }
            },
            Self::Traverse {
                key,
                domain,
                ticket,
            } => match traversal(owner, context, *key, *domain, *ticket) {
                Ok(traversal) => {
                    *self = Self::Traversing(traversal);
                    Step::Pending
                }
                Err(error) => rejected(error),
            },
            Self::Traversing(traversal) => match traversal.poll(&owner.raw, context) {
                Ok(EdgeTraversalProgress::Pending) => Step::Pending,
                Ok(EdgeTraversalProgress::Complete) => Step::Complete(Progress::Committed),
                Ok(EdgeTraversalProgress::Superseded) => rejected(RequestError::Conflict),
                Err(EdgeTraversalError::GenerationExhausted) => {
                    rejected(RequestError::GenerationExhausted)
                }
                Err(EdgeTraversalError::CounterExhausted) => rejected(RequestError::Exhausted),
                Err(_) => unreachable!("same unfinished traversal"),
            },
        }
    }
}

#[expect(
    private_bounds,
    reason = "sealed command dispatch keeps owning state within bounded participant custody"
)]
impl<R: Payload, C: Commands<R>> Participant<R, C> {
    pub fn submit_edge_creation(&mut self, slot: usize) -> Result<(), RequestError> {
        self.idle()?;
        if slot >= self.lease.owner.capacities.edge_slots {
            return Err(RequestError::InvalidSlot);
        }
        let lease = EdgeLease::reserve(&self.lease.owner)?;
        self.state = RequestState::Edge(Request::CreatingStart {
            slot,
            lease: Some(lease),
        });
        Ok(())
    }
    pub fn take_edge(&mut self) -> Option<EdgeHandle<R>> {
        if !matches!(self.state, RequestState::Edge(Request::Ready(_))) {
            return None;
        }
        let RequestState::Edge(Request::Ready(handle)) =
            std::mem::replace(&mut self.state, RequestState::Idle)
        else {
            unreachable!("ready edge")
        };
        Some(handle)
    }
    pub fn submit_edge(&mut self, command: EdgeCommand) -> Result<(), RequestError> {
        self.idle()?;
        if command.key().catalog != self.lease.owner.identity {
            return Err(RequestError::Foreign);
        }
        self.state = RequestState::Edge(Request::Start(command));
        Ok(())
    }
    pub fn observe_edge(&mut self, key: EdgeKey) -> Result<EdgeObservation, RequestError> {
        self.idle()?;
        let edge = lookup(&self.lease.owner, key)?;
        observation(&self.lease.owner, &edge)
    }
    /// A copied edge observation validated with its live source under one stable
    /// controller revision. In particular, null is a committed graph read, not
    /// merely a transient edge effect seen before its controller certificate.
    /// Non-null identities still require `RootTarget` before payload access.
    pub fn observe_source_edge(
        &mut self,
        source: RootKey,
        slot: usize,
        context: &R::Context,
    ) -> Result<EdgeObservation, RequestError> {
        self.read_source_edge(slot, context, |owner, phase| {
            live_root(owner, source, phase).map(|root| root_key(&root))
        })
    }
    /// A retained token already pins its registry entry. Check its visibility
    /// and target under the same stable revision as the edge observation.
    pub fn observe_rooted_edge(
        &mut self,
        source: &RootHandle<R>,
        slot: usize,
        context: &R::Context,
    ) -> Result<EdgeObservation, RequestError> {
        self.read_source_edge(slot, context, |owner, phase| {
            if !source.belongs_to(owner) {
                return Err(RequestError::Foreign);
            }
            let root = source.entry();
            validate_live_root(root, phase)?;
            Ok(root_key(root))
        })
    }
    fn read_source_edge(
        &mut self,
        slot: usize,
        context: &R::Context,
        source: impl FnOnce(&CatalogHandle<R>, Phase) -> Result<ObjectKey, RequestError>,
    ) -> Result<EdgeObservation, RequestError> {
        self.idle()?;
        let owner = &self.lease.owner;
        if slot >= owner.capacities.edge_slots {
            return Err(RequestError::InvalidSlot);
        }
        let snapshot = owner
            .raw
            .snapshot_in(context)
            .ok_or(RequestError::Contended)?;
        let phase = snapshot.phase();
        let result = (|| {
            let source_key = source(owner, phase)?;
            let edge = owner
                .raw
                .discover_edge(slot)
                .map_err(|_| RequestError::Contended)?
                .ok_or(RequestError::Stale)?;
            let observed = observation(owner, &edge)?;
            if !matches!(observed.state, EdgeMembership::Live(state)
                if state.source == source_key)
            {
                return Err(RequestError::Conflict);
            }
            Ok(observed)
        })();
        #[cfg(test)]
        {
            let hook = AFTER_SOURCE_OBSERVATION.with(|hook| hook.borrow_mut().take());
            if let Some(hook) = hook {
                hook();
            }
        }
        if !owner.raw.validates_snapshot_in(&snapshot, context) {
            return Err(RequestError::Contended);
        }
        result
    }
    pub fn discover_edge(&mut self, slot: usize) -> Result<Option<EdgeObservation>, RequestError> {
        self.idle()?;
        let owner = &self.lease.owner;
        if slot >= owner.capacities.edge_slots {
            return Err(RequestError::InvalidSlot);
        }
        owner
            .raw
            .discover_edge(slot)
            .map_err(|_| RequestError::Contended)?
            .map(|edge| observation(owner, &edge))
            .transpose()
    }
    pub fn release_retired_edge(&mut self, key: EdgeKey) -> Result<bool, RequestError> {
        self.idle()?;
        let owner = &self.lease.owner;
        Ok(owner.raw.release_retired_edge(&lookup(owner, key)?))
    }
    /// Consume the external creation owner after a stable controller observation.
    /// Payloads retain only the returned key; it owns neither catalog nor Cells.
    pub fn release_edge_to_source(
        &mut self,
        edge: EdgeHandle<R>,
        context: &R::Context,
    ) -> Result<EdgeKey, EdgeReleaseFailure<R>> {
        let preflight = || {
            self.idle()?;
            if edge.lease.owner.identity != self.lease.owner.identity {
                return Err(RequestError::Foreign);
            }
            self.lease
                .owner
                .raw
                .snapshot_in(context)
                .ok_or(RequestError::Contended)?;
            Ok(())
        };
        if let Err(error) = preflight() {
            return Err(EdgeReleaseFailure { error, edge });
        }
        let key = edge.key();
        let EdgeHandle { token, lease } = edge;
        match token.release_to_source() {
            Ok(()) => {
                drop(lease);
                Ok(key)
            }
            Err(token) => {
                let error = match token.membership() {
                    Some(EdgeMembership::Prepared | EdgeMembership::Retired) => {
                        RequestError::Conflict
                    }
                    _ => RequestError::Contended,
                };
                Err(EdgeReleaseFailure {
                    error,
                    edge: EdgeHandle { token, lease },
                })
            }
        }
    }
}
fn key<R: Payload>(owner: &BoundedCatalog<R>, edge: &EdgeSlot) -> EdgeKey {
    EdgeKey {
        catalog: owner.identity,
        identity: edge.identity,
        slot: edge.slot,
    }
}
fn observation<R: Payload>(
    owner: &BoundedCatalog<R>,
    edge: &EdgeSlot,
) -> Result<EdgeObservation, RequestError> {
    Ok(EdgeObservation {
        key: key(owner, edge),
        state: edge.membership().ok_or(RequestError::Contended)?,
        removal_requested: edge.removal_requested(),
    })
}
fn lookup<R: Payload>(
    owner: &BoundedCatalog<R>,
    key: EdgeKey,
) -> Result<Pooled<EdgeSlot>, RequestError> {
    if key.catalog != owner.identity {
        return Err(RequestError::Foreign);
    }
    let edge = owner
        .raw
        .discover_edge(key.slot)
        .map_err(|_| RequestError::Contended)?
        .ok_or(RequestError::Stale)?;
    if edge.identity != key.identity {
        return Err(RequestError::Stale);
    }
    Ok(edge)
}
fn root_key<R: Payload>(root: &Root<R>) -> ObjectKey {
    ObjectKey {
        identity: root.target_identity(),
        slot: root.target_slot(),
    }
}
fn live_root<R: Payload>(
    owner: &BoundedCatalog<R>,
    key: RootKey,
    phase: Phase,
) -> Result<Pooled<Root<R>>, RequestError> {
    let root = roots::lookup(owner, key)?;
    validate_live_root(&root, phase)?;
    Ok(root)
}
fn validate_live_root<R: Payload>(root: &Root<R>, phase: Phase) -> Result<(), RequestError> {
    if root.is_internal() || root.status() != RootStatus::Live {
        return Err(RequestError::Conflict);
    }
    let Membership::Tracing(marks) = root.target_membership().ok_or(RequestError::Contended)?
    else {
        return Err(RequestError::Conflict);
    };
    if marks.logically_retired(phase) {
        return Err(RequestError::Conflict);
    }
    Ok(())
}
fn deletion<R: Payload>(
    owner: &BoundedCatalog<R>,
    target: Option<ObjectKey>,
    phase: Phase,
) -> Result<Phase, RequestError> {
    let Some(target) = target else {
        return Ok(phase);
    };
    let cell = owner
        .raw
        .discover(target.slot)
        .map_err(|_| RequestError::Contended)?;
    let Some(cell) = cell.filter(|cell| cell.identity == target.identity) else {
        return Ok(phase);
    };
    crate::canonical_catalog::state::root_barrier_plan(
        cell.membership().ok_or(RequestError::Contended)?,
        phase,
        true,
    )
    .map(|(_, phase)| phase)
    .map_err(trace_error)
}
fn insertion<R: Payload>(
    target: Option<&Pooled<Root<R>>>,
    phase: Phase,
) -> Result<(), RequestError> {
    if let Some(root) = target {
        roots::barrier_valid(root, phase, false)?;
    }
    Ok(())
}
pub(super) fn stable_failure<R: Payload, T>(
    owner: &BoundedCatalog<R>,
    context: &R::Context,
    revision: u64,
    result: Result<T, RequestError>,
) -> Result<T, RequestError> {
    match result {
        Err(
            error @ (RequestError::Conflict | RequestError::Stale | RequestError::AlreadyMarked),
        ) => {
            let current = owner.raw.phase_in(context).ok_or(RequestError::Contended)?;
            if current.revision == revision {
                Err(error)
            } else {
                Err(RequestError::Contended)
            }
        }
        result => result,
    }
}
fn start<R: Payload>(
    owner: &BoundedCatalog<R>,
    context: &R::Context,
    command: EdgeCommand,
) -> Result<Preparation<R>, RequestError> {
    let snapshot = owner
        .raw
        .snapshot_in(context)
        .ok_or(RequestError::Contended)?;
    let revision = snapshot.phase().revision;
    stable_failure(owner, context, revision, prepare(owner, snapshot, command))
}
fn prepare<R: Payload>(
    owner: &BoundedCatalog<R>,
    snapshot: crate::canonical_catalog::Snapshot<R>,
    command: EdgeCommand,
) -> Result<Preparation<R>, RequestError> {
    let phase = snapshot.phase();
    if phase.revision == u64::MAX {
        return Err(RequestError::Exhausted);
    }
    let edge = lookup(owner, command.key())?;
    let state = edge.membership().ok_or(RequestError::Contended)?;
    let preparation = match command {
        EdgeCommand::Publish { source, target, .. } => {
            if state != EdgeMembership::Prepared {
                return Err(RequestError::Conflict);
            }
            let source = live_root(owner, source, phase)?;
            if !owner
                .raw
                .edge_slot_matches_source(edge.slot, source.target_slot())
            {
                return Err(RequestError::InvalidSlot);
            }
            let target = target
                .map(|root| live_root(owner, root, phase))
                .transpose()?;
            insertion(target.as_ref(), phase)?;
            owner.raw.begin_publish_edge(snapshot, edge, source, target)
        }
        EdgeCommand::Replace { source, target, .. } => {
            let EdgeMembership::Live(state) = state else {
                return Err(RequestError::Conflict);
            };
            let source = live_root(owner, source, phase)?;
            if root_key(&source) != state.source {
                return Err(RequestError::Conflict);
            }
            let target = target
                .map(|root| live_root(owner, root, phase))
                .transpose()?;
            let next = target.as_ref().map(|root| root_key(&**root));
            let changed = if state.target == next {
                phase
            } else {
                deletion(owner, state.target, phase)?
            };
            insertion(target.as_ref(), changed)?;
            owner.raw.begin_replace_edge(snapshot, edge, source, target)
        }
        EdgeCommand::RootTarget { source, child, .. } => {
            let EdgeMembership::Live(state) = state else {
                return Err(RequestError::Conflict);
            };
            let source = live_root(owner, source, phase)?;
            let child = roots::lookup(owner, child)?;
            if root_key(&source) != state.source
                || state.target != Some(root_key(&child))
                || child.is_internal()
                || child.status() != RootStatus::Prepared
            {
                return Err(RequestError::Conflict);
            }
            insertion(Some(&child), phase)?;
            owner
                .raw
                .begin_root_edge_target(snapshot, edge, source, child)
        }
        EdgeCommand::RemoveRequested(_) => {
            if !edge.removal_requested() || state == EdgeMembership::Retired {
                return Err(RequestError::Conflict);
            }
            if let EdgeMembership::Live(state) = state {
                deletion(owner, state.target, phase)?;
            }
            owner.raw.begin_requested_edge_removal(snapshot, edge)
        }
        EdgeCommand::RemoveOrphan(_) => {
            let EdgeMembership::Live(state) = state else {
                return Err(RequestError::Conflict);
            };
            if let Some(source) = owner
                .raw
                .discover(state.source.slot)
                .map_err(|_| RequestError::Contended)?
                && source.identity == state.source.identity
                && source.membership().ok_or(RequestError::Contended)? != Membership::Retired
            {
                return Err(RequestError::Conflict);
            }
            deletion(owner, state.target, phase)?;
            owner.raw.begin_orphan_edge_removal(snapshot, edge)
        }
    };
    preparation.ok_or_else(|| retry_error(owner))
}
#[expect(
    clippy::indexing_slicing,
    reason = "domain is checked before fixed trace-mark access"
)]
fn traversal<R: Payload>(
    owner: &BoundedCatalog<R>,
    context: &R::Context,
    key: RecordKey,
    domain: usize,
    ticket: u64,
) -> Result<EdgeTraversal<R>, RequestError> {
    let snapshot = owner
        .raw
        .snapshot_in(context)
        .ok_or(RequestError::Contended)?;
    let phase = snapshot.phase();
    let result = || {
        let active = phase
            .domains
            .get(domain)
            .ok_or(RequestError::InvalidDomain)?;
        let cell = super::lookup(owner, key)?;
        let Membership::Tracing(marks) = cell.membership().ok_or(RequestError::Contended)? else {
            return Err(RequestError::Conflict);
        };
        if active.mode != crate::canonical_catalog::DomainMode::Full
            || ticket == 0
            || marks[domain].epoch != active.epoch
            || marks[domain].ticket != ticket
        {
            return Err(RequestError::Conflict);
        }
        owner
            .raw
            .begin_edge_traversal(snapshot, cell, domain, ticket)
            .ok_or_else(|| retry_error(owner))
    };
    stable_failure(owner, context, phase.revision, result())
}

#[cfg(test)]
#[path = "edge_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "edge_admission_tests.rs"]
mod admission_tests;
