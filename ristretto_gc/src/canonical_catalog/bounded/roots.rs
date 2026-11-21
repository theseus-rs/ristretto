//! Separately quota-bound root ownership. Keys and observations carry no access.
use super::super::{Root, RootPrepareError, RootStatus, RootToken};
use super::{
    BoundedCatalog, CatalogHandle, Commands, Membership, Ordering, Participant, Payload, Pooled,
    Preparation, RecordKey, RequestError, RequestState, trace_error,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RootKey {
    catalog: u64,
    identity: u64,
    slot: usize,
}
impl RootKey {
    pub fn identity(self) -> u64 {
        self.identity
    }
    pub fn slot(self) -> usize {
        self.slot
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RootObservation {
    pub key: RootKey,
    pub target: RecordKey,
    pub status: RootStatus,
    pub removal_requested: bool,
    pub internal: bool,
}

/// This is a logical root in the experiment, not a runtime payload-access guard.
/// No clone or raw owning share escapes. Token destruction (including destruction
/// of its retained value) precedes quota release and final catalog-charge refund.
pub struct RootHandle<R: Payload = ()> {
    token: RootToken<R>,
    lease: RootLease<R>,
}
impl<R: Payload> RootHandle<R> {
    pub(super) fn new(token: RootToken<R>, lease: RootLease<R>) -> Self {
        Self { token, lease }
    }
    pub fn key(&self) -> RootKey {
        key(&self.lease.owner, &self.token)
    }
    /// Compare the exact non-reused catalog identity without acquiring an owner.
    pub fn belongs_to(&self, catalog: &CatalogHandle<R>) -> bool {
        self.lease.owner.identity == catalog.identity
    }
    /// The token retains this exact root and target through slot reuse.
    pub fn target_slot(&self) -> usize {
        self.token.target_slot()
    }
    pub(super) fn entry(&self) -> &Root<R> {
        &self.token
    }
    pub(super) fn stably_live_in(&self, context: &R::Context) -> Option<bool> {
        self.lease
            .owner
            .raw
            .root_is_stably_live_in(&self.token, context)
    }
    pub fn observe(&self) -> RootObservation {
        observation(&self.lease.owner, &self.token)
    }

    /// Borrow storage retained by this exact live token. This is not a completed
    /// publication or runtime payload-access certificate: R's adapter must check
    /// its own heap, publication and type requirements before exposing a value.
    ///
    /// Explicit revocation can follow the Live observation. The existing borrow
    /// still retains the token/target; a subsequent call observes Retired. No
    /// owning membership load or pool share escapes, so this adds no independent
    /// retained owner to the root/participant capacity bound.
    pub fn borrow_live_record(&self) -> Option<&R> {
        (self.token.status() == RootStatus::Live).then(|| self.token.record())
    }
}
pub(super) struct RootLease<R: Payload> {
    owner: CatalogHandle<R>,
}
impl<R: Payload> RootLease<R> {
    pub(super) fn reserve(owner: &CatalogHandle<R>) -> Result<Self, RequestError> {
        let before = owner.root_tokens.load(Ordering::Relaxed);
        if before >= owner.capacities.root_slots {
            return Err(RequestError::RootLimit);
        }
        owner
            .root_tokens
            .compare_exchange(before, before + 1, Ordering::Acquire, Ordering::Relaxed)
            .map_err(|_| RequestError::Contended)?;
        Ok(Self {
            owner: owner.clone(),
        })
    }
}
impl<R: Payload> Drop for RootLease<R> {
    fn drop(&mut self) {
        let before = self.owner.root_tokens.fetch_sub(1, Ordering::Release);
        debug_assert!(before > 0, "duplicate root quota refund");
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RootCommand {
    Publish(RootKey),
    Cancel(RootKey),
}
impl RootCommand {
    fn key(self) -> RootKey {
        match self {
            Self::Publish(key) | Self::Cancel(key) => key,
        }
    }
}

#[expect(
    private_bounds,
    reason = "sealed commands keep raw owners inside participants"
)]
impl<R: Payload, C: Commands<R>> Participant<R, C> {
    /// Reserve before creating private ownership. Completion leaves a unique
    /// ready token in this participant until `take_root` or cancellation/drop.
    pub fn submit_root_creation(
        &mut self,
        slot: usize,
        target: RecordKey,
    ) -> Result<(), RequestError> {
        self.idle()?;
        let catalog = &self.lease.owner;
        if target.catalog != catalog.identity {
            return Err(RequestError::Foreign);
        }
        if slot >= catalog.capacities.root_slots {
            return Err(RequestError::InvalidSlot);
        }
        let lease = RootLease::reserve(catalog)?;
        self.state = RequestState::StartingRootCreation {
            slot,
            target,
            lease: Some(lease),
        };
        Ok(())
    }
    pub fn take_root(&mut self) -> Option<RootHandle<R>> {
        if !matches!(self.state, RequestState::RootReady(_)) {
            return None;
        }
        let RequestState::RootReady(root) = std::mem::replace(&mut self.state, RequestState::Idle)
        else {
            unreachable!("ready root")
        };
        Some(root)
    }
    pub fn submit_root(&mut self, command: RootCommand) -> Result<(), RequestError> {
        self.idle()?;
        if command.key().catalog != self.lease.owner.identity {
            return Err(RequestError::Foreign);
        }
        self.state = RequestState::StartingRoot(command);
        Ok(())
    }
    pub fn observe_root(&mut self, key: RootKey) -> Result<RootObservation, RequestError> {
        self.idle()?;
        let catalog = &self.lease.owner;
        lookup(catalog, key).map(|root| observation(catalog, &root))
    }
    /// Failed owning discovery is Contended, never an empty-slot observation.
    pub fn discover_root(&mut self, slot: usize) -> Result<Option<RootObservation>, RequestError> {
        self.idle()?;
        let catalog = &self.lease.owner;
        if slot >= catalog.capacities.root_slots {
            return Err(RequestError::InvalidSlot);
        }
        Ok(catalog
            .raw
            .discover_root(slot)
            .map_err(|_| RequestError::Contended)?
            .map(|root| observation(catalog, &root)))
    }
    pub fn release_retired_root(&mut self, key: RootKey) -> Result<bool, RequestError> {
        self.idle()?;
        let catalog = &self.lease.owner;
        Ok(catalog.raw.release_retired_root(&lookup(catalog, key)?))
    }
}
pub(super) fn key<R: Payload>(catalog: &BoundedCatalog<R>, root: &Root<R>) -> RootKey {
    RootKey {
        catalog: catalog.identity,
        identity: root.identity,
        slot: root.slot(),
    }
}
pub(super) fn observation<R: Payload>(
    catalog: &BoundedCatalog<R>,
    root: &Root<R>,
) -> RootObservation {
    RootObservation {
        key: key(catalog, root),
        target: RecordKey {
            catalog: catalog.identity,
            identity: root.target_identity(),
            slot: root.target_slot(),
        },
        status: root.status(),
        removal_requested: root.removal_requested(),
        internal: root.is_internal(),
    }
}
pub(super) fn lookup<R: Payload>(
    catalog: &BoundedCatalog<R>,
    key: RootKey,
) -> Result<Pooled<Root<R>>, RequestError> {
    if key.catalog != catalog.identity {
        return Err(RequestError::Foreign);
    }
    let root = catalog
        .raw
        .discover_root(key.slot)
        .map_err(|_| RequestError::Contended)?
        .ok_or(RequestError::Stale)?;
    if root.identity != key.identity {
        return Err(RequestError::Stale);
    }
    Ok(root)
}

pub(super) fn start<R: Payload>(
    catalog: &BoundedCatalog<R>,
    context: &R::Context,
    command: RootCommand,
) -> Result<Preparation<R>, RequestError> {
    let snapshot = catalog
        .raw
        .snapshot_in(context)
        .ok_or(RequestError::Contended)?;
    let phase = snapshot.phase();
    if phase.revision == u64::MAX {
        return Err(RequestError::Exhausted);
    }
    let root = lookup(catalog, command.key())?;
    let preparation = match command {
        RootCommand::Publish(_) => catalog.raw.begin_publish_root(snapshot, root),
        RootCommand::Cancel(_) => catalog.raw.begin_cancel_root(snapshot, root),
    };
    if let Some(preparation) = preparation {
        return Ok(preparation);
    }
    let root = lookup(catalog, command.key())?;
    if root.status() != RootStatus::Prepared || root.is_internal() {
        return Err(RequestError::Conflict);
    }
    if matches!(command, RootCommand::Publish(_)) {
        if root.target_membership().ok_or(RequestError::Contended)? != Membership::Prepared {
            return Err(RequestError::Conflict);
        }
        super::super::TraceState::publication(phase).map_err(trace_error)?;
    }
    if catalog.raw.generation_exhaustion_observed() {
        Err(RequestError::GenerationExhausted)
    } else {
        Err(RequestError::Contended)
    }
}
pub(super) fn prepare_error(error: RootPrepareError) -> RequestError {
    match error {
        RootPrepareError::Foreign => RequestError::Foreign,
        RootPrepareError::InvalidSlot => RequestError::InvalidSlot,
        RootPrepareError::Occupied => RequestError::Conflict,
        RootPrepareError::Contended => RequestError::Contended,
        RootPrepareError::IdentityExhausted => RequestError::Exhausted,
    }
}

#[cfg(test)]
#[path = "root_admission_tests.rs"]
mod root_admission_tests;
