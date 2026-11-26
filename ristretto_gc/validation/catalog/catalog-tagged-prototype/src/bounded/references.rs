//! Quota-bound conditional handles and participant-owned reference work.
use super::*;
use crate::reference::{ObjectKey, ReferenceKind, ReferenceState, ReferenceTarget};
use crate::{
    ReferenceCreation, ReferenceMembership, ReferenceNotification, ReferencePrepareError,
    ReferenceRootKind, ReferenceScan, ReferenceSlot, ReferenceToken, RootCreation, RootStatus,
    RootToken,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReferenceKey {
    catalog: u64,
    identity: u64,
    slot: usize,
}
impl ReferenceKey {
    pub fn identity(self) -> u64 {
        self.identity
    }
    pub fn slot(self) -> usize {
        self.slot
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReferenceObservation {
    pub key: ReferenceKey,
    pub state: ReferenceMembership,
    pub removal_requested: bool,
}
pub struct ReferenceHandle<R: Payload = ()> {
    token: ReferenceToken<R>,
    lease: ReferenceLease<R>,
}
impl<R: Payload> ReferenceHandle<R> {
    pub fn key(&self) -> ReferenceKey {
        key(&self.lease.owner, &self.token)
    }
}
pub struct ConstructionHandle<R: Payload = ()> {
    token: RootToken<R>,
    lease: ConstructionLease<R>,
}
impl<R: Payload> ConstructionHandle<R> {
    pub fn key(&self) -> RootKey {
        roots::key(&self.lease.owner, &self.token)
    }
    pub fn observe(&self) -> RootObservation {
        roots::observation(&self.lease.owner, &self.token)
    }
}
pub(super) struct ReferenceLease<R: Payload> {
    owner: CatalogHandle<R>,
}
pub(super) struct ConstructionLease<R: Payload> {
    owner: CatalogHandle<R>,
}
fn reserve(counter: &AtomicUsize, limit: usize, error: RequestError) -> Result<(), RequestError> {
    let before = counter.load(Ordering::Relaxed);
    if before >= limit {
        return Err(error);
    }
    counter
        .compare_exchange(before, before + 1, Ordering::Acquire, Ordering::Relaxed)
        .map(|_| ())
        .map_err(|_| RequestError::Contended)
}
impl<R: Payload> ReferenceLease<R> {
    fn reserve(owner: &CatalogHandle<R>) -> Result<Self, RequestError> {
        reserve(
            &owner.reference_tokens,
            owner.capacities.reference_slots,
            RequestError::ReferenceLimit,
        )?;
        Ok(Self {
            owner: owner.clone(),
        })
    }
}
impl<R: Payload> ConstructionLease<R> {
    fn reserve(owner: &CatalogHandle<R>) -> Result<Self, RequestError> {
        reserve(
            &owner.construction_tokens,
            owner.capacities.reference_slots,
            RequestError::ConstructionLimit,
        )?;
        Ok(Self {
            owner: owner.clone(),
        })
    }
}
impl<R: Payload> Drop for ReferenceLease<R> {
    fn drop(&mut self) {
        let before = self.owner.reference_tokens.fetch_sub(1, Ordering::Release);
        debug_assert!(before > 0);
    }
}
impl<R: Payload> Drop for ConstructionLease<R> {
    fn drop(&mut self) {
        let before = self
            .owner
            .construction_tokens
            .fetch_sub(1, Ordering::Release);
        debug_assert!(before > 0);
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceCommand {
    Publish {
        reference: ReferenceKey,
        source: RootKey,
        kind: ReferenceKind,
    },
    Clone {
        source: ReferenceKey,
        destination: ReferenceKey,
    },
    Clear(ReferenceKey),
    Upgrade {
        reference: ReferenceKey,
        root: RootKey,
    },
    BindOwner {
        reference: ReferenceKey,
        owner: RootKey,
    },
    PrepareOwner {
        reference: ReferenceKey,
        owner: RecordKey,
    },
    PublishOwner {
        reference: ReferenceKey,
        owner: RootKey,
    },
    FinishOwner(ReferenceKey),
    CancelOwner(ReferenceKey),
    Take {
        reference: ReferenceKey,
        root: RootKey,
    },
    Retire(ReferenceKey),
    Scan {
        domain: usize,
    },
}
impl ReferenceCommand {
    fn keys(self) -> [Option<ReferenceKey>; 2] {
        match self {
            Self::Clone {
                source,
                destination,
            } => [Some(source), Some(destination)],
            Self::Publish { reference, .. }
            | Self::Upgrade { reference, .. }
            | Self::BindOwner { reference, .. }
            | Self::PrepareOwner { reference, .. }
            | Self::PublishOwner { reference, .. }
            | Self::Take { reference, .. }
            | Self::Clear(reference)
            | Self::FinishOwner(reference)
            | Self::CancelOwner(reference)
            | Self::Retire(reference) => [Some(reference), None],
            Self::Scan { .. } => [None, None],
        }
    }
}
#[allow(
    clippy::large_enum_variant,
    reason = "bounded inline requests avoid lifecycle allocations"
)]
pub(super) enum Request<R: Payload> {
    StartingCreation {
        slot: usize,
        lease: Option<ReferenceLease<R>>,
    },
    Creating {
        creation: ReferenceCreation<R>,
        lease: Option<ReferenceLease<R>>,
    },
    Ready(ReferenceHandle<R>),
    Command(ReferenceCommand, Work<R>),
    Notification(Notification<R>),
    StartingReservation {
        reference: ReferenceKey,
        lease: Option<ConstructionLease<R>>,
    },
    CreatingReservation {
        reference: ReferenceKey,
        creation: RootCreation<R>,
        lease: Option<ConstructionLease<R>>,
    },
    Reserving {
        reference: ReferenceKey,
        root: ConstructionHandle<R>,
        work: Work<R>,
    },
    ConstructionReady(ConstructionHandle<R>),
}
#[allow(
    clippy::large_enum_variant,
    reason = "one charged inline work state per participant"
)]
pub(super) enum Work<R: Payload> {
    Start,
    Preparing(Preparation<R>),
    Attempt(Attempt<R>),
}
pub(super) struct Notification<R: Payload> {
    command: ReferenceCommand,
    plan: ReferenceNotification<R>,
    token: Option<RootToken<R>>,
    creation: Option<RootCreation<R>>,
    work: Work<R>,
}
pub(super) enum Step {
    Pending,
    Ready(Progress),
    Complete(Progress),
}
fn rejected(error: RequestError) -> Step {
    if error == RequestError::Contended {
        Step::Pending
    } else {
        Step::Complete(Progress::Rejected(error))
    }
}
fn retry_error<R: Payload>(catalog: &BoundedCatalog<R>) -> RequestError {
    if catalog.raw.generation_exhaustion_observed() {
        RequestError::GenerationExhausted
    } else {
        RequestError::Contended
    }
}
fn creation_error(error: CreationError) -> RequestError {
    match error {
        CreationError::Occupied => RequestError::Conflict,
        CreationError::GenerationExhausted => RequestError::GenerationExhausted,
        _ => unreachable!("same unfinished creation"),
    }
}
fn work<R: Payload>(
    catalog: &BoundedCatalog<R>,
    context: &R::Context,
    state: &mut Work<R>,
) -> Result<Outcome, RequestError> {
    match state {
        Work::Start => unreachable!("work must be prepared first"),
        Work::Preparing(preparation) => {
            match preparation.poll(&catalog.raw) {
                Ok(Some(attempt)) => *state = Work::Attempt(attempt),
                Ok(None) => {}
                Err(PreparationError::GenerationExhausted) => {
                    return Err(RequestError::GenerationExhausted);
                }
                Err(_) => unreachable!("same unfinished preparation"),
            }
            Ok(Outcome::Pending)
        }
        Work::Attempt(attempt) => Ok(catalog.raw.commit_in(attempt, context)),
    }
}
impl<R: Payload> Work<R> {
    fn cancel(&self) -> bool {
        match self {
            Self::Attempt(attempt) => attempt.cancel(),
            _ => true,
        }
    }
}
impl<R: Payload> Request<R> {
    pub(super) fn cancel(&self) -> bool {
        match self {
            Self::Command(_, work) | Self::Reserving { work, .. } => work.cancel(),
            Self::Notification(notification) => notification.work.cancel(),
            _ => true,
        }
    }
    pub(super) fn poll(&mut self, catalog: &CatalogHandle<R>, context: &R::Context) -> Step {
        match self {
            Self::StartingCreation { slot, lease } => {
                match catalog.raw.begin_create_reference(*slot) {
                    Ok(creation) => {
                        *self = Self::Creating {
                            creation,
                            lease: lease.take(),
                        }
                    }
                    Err(error) => return rejected(reference_prepare_error(error)),
                }
                Step::Pending
            }
            Self::Creating { creation, lease } => match creation.poll(&catalog.raw) {
                Ok(Some(token)) => {
                    let handle = ReferenceHandle {
                        token,
                        lease: lease.take().expect("reference permit"),
                    };
                    let progress = Progress::ReferenceCreated(handle.key());
                    *self = Self::Ready(handle);
                    Step::Ready(progress)
                }
                Ok(None) => Step::Pending,
                Err(error) => rejected(creation_error(error)),
            },
            Self::Ready(handle) => Step::Ready(Progress::ReferenceCreated(handle.key())),
            Self::ConstructionReady(handle) => {
                Step::Ready(Progress::ConstructionCreated(handle.key()))
            }
            Self::Command(command, state) => {
                if matches!(state, Work::Start) {
                    match start(catalog, context, *command) {
                        Ok(ReferenceScan::Ready(preparation)) => {
                            *state = Work::Preparing(preparation)
                        }
                        Ok(ReferenceScan::NeedsRoot(plan)) => {
                            *self = Self::Notification(Notification {
                                command: *command,
                                plan,
                                token: None,
                                creation: None,
                                work: Work::Start,
                            })
                        }
                        Err(error) => return rejected(error),
                    }
                    return Step::Pending;
                }
                match work(catalog, context, state) {
                    Ok(Outcome::Committed) => Step::Complete(Progress::Committed),
                    Ok(Outcome::Cancelled) => Step::Complete(Progress::Cancelled),
                    Ok(Outcome::Retry) => {
                        *state = Work::Start;
                        Step::Pending
                    }
                    Ok(Outcome::Pending) => Step::Pending,
                    Err(error) => rejected(error),
                }
            }
            Self::Notification(notification) => {
                let result = notification.poll(catalog, context);
                if matches!(result, Ok(Outcome::Retry)) {
                    *self = Self::Command(notification.command, Work::Start);
                    return Step::Pending;
                }
                match result {
                    Ok(Outcome::Committed) => Step::Complete(Progress::Committed),
                    Ok(Outcome::Cancelled) => Step::Complete(Progress::Cancelled),
                    Ok(_) => Step::Pending,
                    Err(error) => rejected(error),
                }
            }
            Self::StartingReservation { reference, lease } => {
                let result = reservation_creation(catalog, context, *reference);
                match result {
                    Ok(creation) => {
                        *self = Self::CreatingReservation {
                            reference: *reference,
                            creation,
                            lease: lease.take(),
                        }
                    }
                    Err(error) => return rejected(error),
                }
                Step::Pending
            }
            Self::CreatingReservation {
                reference,
                creation,
                lease,
            } => match creation.poll(&catalog.raw) {
                Ok(Some(token)) => {
                    *self = Self::Reserving {
                        reference: *reference,
                        root: ConstructionHandle {
                            token,
                            lease: lease.take().expect("construction permit"),
                        },
                        work: Work::Start,
                    };
                    Step::Pending
                }
                Ok(None) => Step::Pending,
                Err(error) => rejected(creation_error(error)),
            },
            Self::Reserving {
                reference,
                root,
                work: state,
            } => {
                if matches!(state, Work::Start) {
                    match reserve_preparation(catalog, context, *reference, &root.token) {
                        Ok(preparation) => *state = Work::Preparing(preparation),
                        Err(error) => return rejected(error),
                    }
                    return Step::Pending;
                }
                match work(catalog, context, state) {
                    Ok(Outcome::Committed) => {
                        let progress = Progress::ConstructionCreated(root.key());
                        let Self::Reserving { root, .. } = std::mem::replace(
                            self,
                            Self::Command(ReferenceCommand::Scan { domain: 0 }, Work::Start),
                        ) else {
                            unreachable!()
                        };
                        *self = Self::ConstructionReady(root);
                        Step::Ready(progress)
                    }
                    Ok(Outcome::Cancelled) => Step::Complete(Progress::Cancelled),
                    Ok(Outcome::Retry) => {
                        *state = Work::Start;
                        Step::Pending
                    }
                    Ok(Outcome::Pending) => Step::Pending,
                    Err(error) => rejected(error),
                }
            }
        }
    }
}
impl<R: Payload> Notification<R> {
    fn poll(
        &mut self,
        catalog: &BoundedCatalog<R>,
        context: &R::Context,
    ) -> Result<Outcome, RequestError> {
        if !matches!(self.work, Work::Start) {
            return work(catalog, context, &mut self.work);
        }
        if let Some(creation) = &mut self.creation {
            match creation.poll(&catalog.raw) {
                Ok(Some(token)) => {
                    self.token = Some(token);
                    self.creation = None;
                }
                Ok(None) => {}
                Err(CreationError::Occupied) => self.creation = None,
                Err(error) => return Err(creation_error(error)),
            }
            return Ok(Outcome::Pending);
        }
        if let Some(root) = self
            .plan
            .discover_root()
            .map_err(|_| RequestError::Contended)?
        {
            if root.status() == RootStatus::Retired {
                catalog.raw.release_retired_root(&root);
            }
            if let Some(preparation) = self.plan.try_finish(&catalog.raw, root) {
                self.work = Work::Preparing(preparation);
                return Ok(Outcome::Pending);
            }
        } else {
            match self.plan.begin_root_creation(&catalog.raw) {
                Ok(creation) => {
                    self.creation = Some(creation);
                    return Ok(Outcome::Pending);
                }
                Err(crate::RootPrepareError::Contended | crate::RootPrepareError::Occupied) => {
                    return Ok(Outcome::Pending);
                }
                Err(crate::RootPrepareError::InvalidSlot) => {}
                Err(error) => return Err(roots::prepare_error(error)),
            }
        }
        let phase = catalog
            .raw
            .snapshot_in(context)
            .ok_or(RequestError::Contended)?
            .phase();
        if phase.revision != self.plan.expected_revision() {
            return Ok(Outcome::Retry);
        }
        Err(retry_error(catalog))
    }
}

#[allow(private_bounds)]
impl<R: Payload, C: Commands<R>> Participant<R, C> {
    pub fn submit_reference_creation(&mut self, slot: usize) -> Result<(), RequestError> {
        self.idle()?;
        let owner = &self.lease.owner;
        if slot >= owner.capacities.reference_slots {
            return Err(RequestError::InvalidSlot);
        }
        let lease = ReferenceLease::reserve(owner)?;
        self.state = RequestState::Reference(Request::StartingCreation {
            slot,
            lease: Some(lease),
        });
        Ok(())
    }
    pub fn take_reference(&mut self) -> Option<ReferenceHandle<R>> {
        if !matches!(self.state, RequestState::Reference(Request::Ready(_))) {
            return None;
        }
        let RequestState::Reference(Request::Ready(handle)) =
            std::mem::replace(&mut self.state, RequestState::Idle)
        else {
            unreachable!()
        };
        Some(handle)
    }
    pub fn submit_reference(&mut self, command: ReferenceCommand) -> Result<(), RequestError> {
        self.idle()?;
        if command
            .keys()
            .iter()
            .flatten()
            .any(|key| key.catalog != self.lease.owner.identity)
        {
            return Err(RequestError::Foreign);
        }
        self.state = RequestState::Reference(Request::Command(command, Work::Start));
        Ok(())
    }
    pub fn submit_reference_reservation(
        &mut self,
        reference: ReferenceKey,
    ) -> Result<(), RequestError> {
        self.idle()?;
        if reference.catalog != self.lease.owner.identity {
            return Err(RequestError::Foreign);
        }
        let lease = ConstructionLease::reserve(&self.lease.owner)?;
        self.state = RequestState::Reference(Request::StartingReservation {
            reference,
            lease: Some(lease),
        });
        Ok(())
    }
    pub fn take_construction(&mut self) -> Option<ConstructionHandle<R>> {
        if !matches!(
            self.state,
            RequestState::Reference(Request::ConstructionReady(_))
        ) {
            return None;
        }
        let RequestState::Reference(Request::ConstructionReady(handle)) =
            std::mem::replace(&mut self.state, RequestState::Idle)
        else {
            unreachable!()
        };
        Some(handle)
    }
    pub fn observe_reference(
        &mut self,
        reference: ReferenceKey,
    ) -> Result<ReferenceObservation, RequestError> {
        self.idle()?;
        let owner = &self.lease.owner;
        let slot = lookup(owner, reference)?;
        observation(owner, &slot)
    }
    pub fn discover_reference(
        &mut self,
        slot: usize,
    ) -> Result<Option<ReferenceObservation>, RequestError> {
        self.idle()?;
        let owner = &self.lease.owner;
        if slot >= owner.capacities.reference_slots {
            return Err(RequestError::InvalidSlot);
        }
        owner
            .raw
            .discover_reference(slot)
            .map_err(|_| RequestError::Contended)?
            .map(|slot| observation(owner, &slot))
            .transpose()
    }
    pub fn discover_reference_root(
        &mut self,
        reference: ReferenceKey,
        kind: ReferenceRootKind,
    ) -> Result<Option<RootObservation>, RequestError> {
        self.idle()?;
        let owner = &self.lease.owner;
        let slot = lookup(owner, reference)?;
        owner
            .raw
            .discover_reference_root(&slot, kind)
            .map_err(roots::prepare_error)
            .map(|root| root.map(|root| roots::observation(owner, &root)))
    }
    pub fn release_retired_reference(
        &mut self,
        reference: ReferenceKey,
    ) -> Result<bool, RequestError> {
        self.idle()?;
        let owner = &self.lease.owner;
        Ok(owner
            .raw
            .release_retired_reference(&lookup(owner, reference)?))
    }
}
fn key<R: Payload>(owner: &BoundedCatalog<R>, slot: &ReferenceSlot<R>) -> ReferenceKey {
    ReferenceKey {
        catalog: owner.identity,
        identity: slot.identity,
        slot: slot.slot,
    }
}
fn lookup<R: Payload>(
    owner: &BoundedCatalog<R>,
    key: ReferenceKey,
) -> Result<Pooled<ReferenceSlot<R>>, RequestError> {
    if key.catalog != owner.identity {
        return Err(RequestError::Foreign);
    }
    let slot = owner
        .raw
        .discover_reference(key.slot)
        .map_err(|_| RequestError::Contended)?
        .ok_or(RequestError::Stale)?;
    if slot.identity != key.identity {
        return Err(RequestError::Stale);
    }
    Ok(slot)
}
fn observation<R: Payload>(
    owner: &BoundedCatalog<R>,
    slot: &ReferenceSlot<R>,
) -> Result<ReferenceObservation, RequestError> {
    Ok(ReferenceObservation {
        key: key(owner, slot),
        state: slot.membership().ok_or(RequestError::Contended)?,
        removal_requested: slot.removal_requested(),
    })
}
fn live<R: Payload>(slot: &ReferenceSlot<R>) -> Result<ReferenceState, RequestError> {
    match slot.membership().ok_or(RequestError::Contended)? {
        ReferenceMembership::Live(state) => Ok(state),
        _ => Err(RequestError::Conflict),
    }
}
fn reference_prepare_error(error: ReferencePrepareError) -> RequestError {
    match error {
        ReferencePrepareError::InvalidSlot => RequestError::InvalidSlot,
        ReferencePrepareError::Occupied => RequestError::Conflict,
        ReferencePrepareError::Contended => RequestError::Contended,
        ReferencePrepareError::IdentityExhausted => RequestError::Exhausted,
    }
}
fn snapshot<R: Payload>(
    owner: &BoundedCatalog<R>,
    context: &R::Context,
) -> Result<crate::Snapshot<R>, RequestError> {
    let snapshot = owner
        .raw
        .snapshot_in(context)
        .ok_or(RequestError::Contended)?;
    if snapshot.phase().revision == u64::MAX {
        return Err(RequestError::Exhausted);
    }
    Ok(snapshot)
}
fn target_key<R: Payload>(root: &crate::Root<R>) -> ObjectKey {
    ObjectKey {
        identity: root.target_identity(),
        slot: root.target_slot(),
    }
}
fn caller_root<R: Payload>(root: &crate::Root<R>, status: RootStatus) -> Result<(), RequestError> {
    if root.is_internal() || root.status() != status {
        Err(RequestError::Conflict)
    } else {
        Ok(())
    }
}
fn unbound(state: ReferenceState) -> Result<(), RequestError> {
    if state.owner.is_some() || state.activated || state.pending || state.queued_root.is_some() {
        Err(RequestError::Conflict)
    } else {
        Ok(())
    }
}
fn held<R: Payload>(
    owner: &BoundedCatalog<R>,
    key: Option<crate::reference::ReferenceRoot>,
) -> Result<Pooled<crate::Root<R>>, RequestError> {
    let key = key.ok_or(RequestError::Conflict)?;
    let root = owner
        .raw
        .discover_root(key.slot)
        .map_err(|_| RequestError::Contended)?
        .ok_or(RequestError::Conflict)?;
    if root.identity != key.identity || !root.is_internal() || root.status() != RootStatus::Live {
        return Err(RequestError::Conflict);
    }
    Ok(root)
}
fn observed_target<R: Payload>(
    owner: &BoundedCatalog<R>,
    key: ObjectKey,
) -> Result<ReferenceTarget, RequestError> {
    let cell = owner
        .raw
        .discover(key.slot)
        .map_err(|_| RequestError::Contended)?
        .ok_or(RequestError::Conflict)?;
    if cell.identity != key.identity {
        return Err(RequestError::Conflict);
    }
    Ok(ReferenceTarget {
        key,
        membership: cell.membership().ok_or(RequestError::Contended)?,
    })
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
            if current.revision != revision {
                Err(RequestError::Contended)
            } else {
                Err(error)
            }
        }
        result => result,
    }
}
fn start<R: Payload>(
    owner: &BoundedCatalog<R>,
    context: &R::Context,
    command: ReferenceCommand,
) -> Result<ReferenceScan<R>, RequestError> {
    let snapshot = snapshot(owner, context)?;
    let revision = snapshot.phase().revision;
    stable_failure(
        owner,
        context,
        revision,
        prepare_command(owner, snapshot, command),
    )
}
fn prepare_command<R: Payload>(
    owner: &BoundedCatalog<R>,
    snapshot: crate::Snapshot<R>,
    command: ReferenceCommand,
) -> Result<ReferenceScan<R>, RequestError> {
    let phase = snapshot.phase();
    let [first, second] = command.keys();
    let slot = first.map(|key| lookup(owner, key)).transpose()?;
    let second = second.map(|key| lookup(owner, key)).transpose()?;
    let preparation = match command {
        ReferenceCommand::Scan { domain } => {
            let entry = phase
                .domains
                .get(domain)
                .ok_or(RequestError::InvalidDomain)?;
            if entry.mode == crate::DomainMode::Inactive
                || !(1..crate::RETIREMENT_FLOOR).contains(&entry.floor)
                || phase.reference_cursors[domain] >= owner.capacities.reference_slots
            {
                return Err(RequestError::Conflict);
            }
            return owner
                .raw
                .begin_reference_scan(snapshot, domain)
                .ok_or_else(|| retry_error(owner));
        }
        ReferenceCommand::Publish { source, kind, .. } => {
            let slot = slot.unwrap();
            if slot.membership().ok_or(RequestError::Contended)? != ReferenceMembership::Prepared {
                return Err(RequestError::Conflict);
            }
            let root = roots::lookup(owner, source)?;
            caller_root(&root, RootStatus::Live)?;
            roots::barrier_valid(&root, phase, false)?;
            owner
                .raw
                .begin_publish_reference(snapshot, slot, root, kind)
        }
        ReferenceCommand::Clone { .. } => {
            let slot = slot.unwrap();
            live(&slot)?;
            let destination = second.unwrap();
            if destination.membership().ok_or(RequestError::Contended)?
                != ReferenceMembership::Prepared
            {
                return Err(RequestError::Conflict);
            }
            owner.raw.begin_clone_reference(snapshot, slot, destination)
        }
        ReferenceCommand::Clear(_) => {
            let slot = slot.unwrap();
            if live(&slot)?.target.is_none() {
                return Err(RequestError::Conflict);
            }
            owner.raw.begin_clear_reference(snapshot, slot)
        }
        ReferenceCommand::Upgrade { root, .. } => {
            let slot = slot.unwrap();
            let state = live(&slot)?;
            let root = roots::lookup(owner, root)?;
            caller_root(&root, RootStatus::Prepared)?;
            let target = ReferenceTarget {
                key: target_key(&root),
                membership: root.target_membership().ok_or(RequestError::Contended)?,
            };
            if !state.upgrade_eligible(phase, Some(target)) {
                return Err(RequestError::Conflict);
            }
            roots::barrier_valid(&root, phase, false)?;
            owner.raw.begin_upgrade_reference(snapshot, slot, root)
        }
        ReferenceCommand::BindOwner { owner: root, .. } => {
            let slot = slot.unwrap();
            let state = live(&slot)?;
            unbound(state)?;
            if state.construction_root.is_some() {
                return Err(RequestError::Conflict);
            }
            let root = roots::lookup(owner, root)?;
            caller_root(&root, RootStatus::Live)?;
            roots::barrier_valid(&root, phase, false)?;
            owner.raw.begin_bind_reference_owner(snapshot, slot, root)
        }
        ReferenceCommand::PrepareOwner { owner: cell, .. } => {
            let slot = slot.unwrap();
            let state = live(&slot)?;
            unbound(state)?;
            let cell = super::lookup(owner, cell)?;
            if cell.membership().ok_or(RequestError::Contended)? != Membership::Prepared
                || (state.target.is_some() && state.construction_root.is_none())
            {
                return Err(RequestError::Conflict);
            }
            if state.construction_root.is_some() {
                held(owner, state.construction_root)?;
            }
            owner
                .raw
                .begin_prepare_reference_owner(snapshot, slot, cell)
        }
        ReferenceCommand::PublishOwner { owner: root, .. } => {
            let slot = slot.unwrap();
            let state = live(&slot)?;
            unbound(state)?;
            let root = roots::lookup(owner, root)?;
            caller_root(&root, RootStatus::Prepared)?;
            if root.target_membership().ok_or(RequestError::Contended)? != Membership::Prepared
                || (state.target.is_some() && state.construction_root.is_none())
            {
                return Err(RequestError::Conflict);
            }
            let (_, after) = crate::TraceState::publication(phase).map_err(trace_error)?;
            if state.construction_root.is_some() {
                let held = held(owner, state.construction_root)?;
                roots::barrier_valid(&held, after, true)?;
            }
            owner
                .raw
                .begin_publish_reference_owner(snapshot, slot, root)
        }
        ReferenceCommand::FinishOwner(_) => {
            let slot = slot.unwrap();
            let state = live(&slot)?;
            held(owner, state.construction_root)?;
            let target = observed_target(owner, state.owner.ok_or(RequestError::Conflict)?)?;
            if !matches!(target.membership, Membership::Tracing(_)) {
                return Err(RequestError::Conflict);
            }
            owner.raw.begin_finish_reference_owner(snapshot, slot)
        }
        ReferenceCommand::CancelOwner(_) => {
            let slot = slot.unwrap();
            let state = live(&slot)?;
            let root = held(owner, state.construction_root)?;
            if !root.removal_requested() {
                return Err(RequestError::Conflict);
            }
            roots::barrier_valid(&root, phase, true)?;
            owner.raw.begin_cancel_reference_owner(snapshot, slot)
        }
        ReferenceCommand::Take { root, .. } => {
            let slot = slot.unwrap();
            let state = live(&slot)?;
            let held = held(owner, state.queued_root)?;
            let root = roots::lookup(owner, root)?;
            caller_root(&root, RootStatus::Prepared)?;
            if target_key(&held) != target_key(&root) {
                return Err(RequestError::Conflict);
            }
            roots::barrier_valid(&root, phase, false)?;
            owner.raw.begin_take_reference(snapshot, slot, root)
        }
        ReferenceCommand::Retire(_) => {
            let slot = slot.unwrap();
            if !slot.removal_requested() {
                return Err(RequestError::Conflict);
            }
            match slot.membership().ok_or(RequestError::Contended)? {
                ReferenceMembership::Retired => return Err(RequestError::Conflict),
                ReferenceMembership::Live(state)
                    if state.pending
                        || state.queued_root.is_some()
                        || state.construction_root.is_some() =>
                {
                    return Err(RequestError::Conflict);
                }
                _ => {}
            }
            owner.raw.begin_retire_reference(snapshot, slot)
        }
    };
    preparation
        .map(ReferenceScan::Ready)
        .ok_or_else(|| retry_error(owner))
}
fn reservation_creation<R: Payload>(
    owner: &BoundedCatalog<R>,
    context: &R::Context,
    key: ReferenceKey,
) -> Result<RootCreation<R>, RequestError> {
    let snapshot = snapshot(owner, context)?;
    let phase = snapshot.phase();
    let result = (|| {
        let slot = lookup(owner, key)?;
        let state = live(&slot)?;
        unbound(state)?;
        if state.construction_root.is_some() {
            return Err(RequestError::Conflict);
        }
        let target = observed_target(owner, state.target.ok_or(RequestError::Conflict)?)?;
        if !state.upgrade_eligible(phase, Some(target)) {
            return Err(RequestError::Conflict);
        }
        let cell = super::lookup(
            owner,
            RecordKey {
                catalog: owner.identity,
                identity: target.key.identity,
                slot: target.key.slot,
            },
        )?;
        owner
            .raw
            .begin_create_reference_root(slot, ReferenceRootKind::Construction, cell)
            .map_err(roots::prepare_error)
    })();
    stable_failure(owner, context, phase.revision, result)
}
fn reserve_preparation<R: Payload>(
    owner: &BoundedCatalog<R>,
    context: &R::Context,
    key: ReferenceKey,
    root: &RootToken<R>,
) -> Result<Preparation<R>, RequestError> {
    let snapshot = snapshot(owner, context)?;
    let phase = snapshot.phase();
    let result = (|| {
        let slot = lookup(owner, key)?;
        let state = live(&slot)?;
        unbound(state)?;
        if state.construction_root.is_some() || root.status() != RootStatus::Prepared {
            return Err(RequestError::Conflict);
        }
        let target = ReferenceTarget {
            key: target_key(root),
            membership: root.target_membership().ok_or(RequestError::Contended)?,
        };
        if !state.upgrade_eligible(phase, Some(target)) {
            return Err(RequestError::Conflict);
        }
        roots::barrier_valid(root, phase, false)?;
        owner
            .raw
            .begin_reserve_reference_owner(
                snapshot,
                slot,
                root.try_share().ok_or(RequestError::Contended)?,
            )
            .ok_or_else(|| retry_error(owner))
    })();
    stable_failure(owner, context, phase.revision, result)
}

#[cfg(test)]
#[path = "reference_tests.rs"]
mod tests;
