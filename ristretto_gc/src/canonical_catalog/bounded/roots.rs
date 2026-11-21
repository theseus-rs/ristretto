//! Separately quota-bound root ownership. Keys and observations carry no access.
use super::super::{Root, RootPrepareError, RootStatus, RootToken};
use super::{
    BoundedCatalog, CatalogHandle, Commands, Membership, Ordering, Participant, Payload, Phase,
    Pooled, Preparation, RecordKey, RequestError, RequestState, trace_error,
};

#[cfg(test)]
thread_local! {
    pub(super) static BEFORE_PUBLICATION_PREPARATION: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = const {
        std::cell::RefCell::new(None)
    };
}

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
    /// Coupled initial target publication and root activation.
    Publish(RootKey),
    Activate(RootKey),
    Cancel(RootKey),
    Remove(RootKey),
    RemoveRequested(RootKey),
    Scan {
        domain: usize,
    },
    /// At most eight observations, stopping at the first root/target effect.
    ScanBatch {
        domain: usize,
    },
}
impl RootCommand {
    fn key(self) -> Option<RootKey> {
        match self {
            Self::Publish(key)
            | Self::Activate(key)
            | Self::Cancel(key)
            | Self::Remove(key)
            | Self::RemoveRequested(key) => Some(key),
            Self::Scan { .. } | Self::ScanBatch { .. } => None,
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
        if let RequestState::Allocation(request) = &mut self.state {
            let root = request.take_root()?;
            self.next_object_slot = super::super::following_slot(
                root.token.target_slot(),
                self.lease.owner.capacities.registry,
            );
            self.next_root_slot = super::super::following_slot(
                root.token.slot(),
                self.lease.owner.capacities.root_slots,
            );
            self.state = RequestState::Idle;
            return Some(root);
        }
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
        if command
            .key()
            .is_some_and(|key| key.catalog != self.lease.owner.identity)
        {
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

#[expect(
    clippy::too_many_lines,
    reason = "preserve the exhaustive frozen root command diagnosis"
)]
#[expect(
    clippy::indexing_slicing,
    reason = "the domain is checked by get before fixed-array access"
)]
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
    let root = command.key().map(|key| lookup(catalog, key)).transpose()?;
    #[cfg(test)]
    if matches!(command, RootCommand::Publish(_)) {
        let hook = BEFORE_PUBLICATION_PREPARATION.with(|hook| hook.borrow_mut().take());
        if let Some(hook) = hook {
            hook();
        }
    }
    let preparation = match command {
        RootCommand::Publish(_) => catalog.raw.begin_publish_root(
            snapshot,
            root.expect("root command retains its required root"),
        ),
        RootCommand::Activate(_) => catalog.raw.begin_activate_root(
            snapshot,
            root.expect("root command retains its required root"),
        ),
        RootCommand::Cancel(_) => catalog.raw.begin_cancel_root(
            snapshot,
            root.expect("root command retains its required root"),
        ),
        RootCommand::Remove(_) => catalog.raw.begin_remove_root(
            snapshot,
            root.expect("root command retains its required root"),
        ),
        RootCommand::RemoveRequested(_) => catalog.raw.begin_requested_root_removal(
            snapshot,
            root.expect("root command retains its required root"),
        ),
        RootCommand::Scan { domain } => catalog.raw.begin_scan_root(snapshot, domain),
        RootCommand::ScanBatch { domain } => catalog.raw.begin_scan_root_batch(snapshot, domain),
    };
    if let Some(preparation) = preparation {
        return Ok(preparation);
    }
    // Diagnose copied preconditions only. Valid observations retry; they never
    // stand in for the exact owning state used by a successful preparation.
    let root = command.key().map(|key| lookup(catalog, key)).transpose()?;
    match command {
        RootCommand::Publish(_) => {
            let root = root.expect("root command retains its required root");
            require_status(&root, RootStatus::Prepared)?;
            if root.target_membership().ok_or(RequestError::Contended)? != Membership::Prepared {
                return Err(RequestError::Conflict);
            }
            crate::canonical_catalog::TraceState::publication(phase).map_err(trace_error)?;
        }
        RootCommand::Activate(_) => {
            let root = root.expect("root command retains its required root");
            require_status(&root, RootStatus::Prepared)?;
            barrier_valid(&root, phase, false)?;
        }
        RootCommand::Cancel(_) => {
            if root
                .expect("root command retains its required root")
                .status()
                != RootStatus::Prepared
            {
                return Err(RequestError::Conflict);
            }
        }
        RootCommand::Remove(_) => {
            let root = root.expect("root command retains its required root");
            require_status(&root, RootStatus::Live)?;
            barrier_valid(&root, phase, true)?;
        }
        RootCommand::RemoveRequested(_) => {
            let root = root.expect("root command retains its required root");
            if !root.removal_requested()
                || (root.is_internal() && root.status() != RootStatus::Prepared)
            {
                return Err(RequestError::Conflict);
            }
            match root.status() {
                RootStatus::Prepared => {}
                RootStatus::Live => barrier_valid(&root, phase, true)?,
                RootStatus::Retired => return Err(RequestError::Conflict),
            }
        }
        RootCommand::Scan { domain } | RootCommand::ScanBatch { domain } => {
            let entry = phase
                .domains
                .get(domain)
                .ok_or(RequestError::InvalidDomain)?;
            if entry.mode == crate::canonical_catalog::DomainMode::Inactive
                || entry.floor != 0
                || phase.root_cursors[domain] >= catalog.raw.root_capacity()
            {
                return Err(RequestError::Conflict);
            }
            if let Some(root) = catalog
                .raw
                .discover_root(phase.root_cursors[domain])
                .map_err(|_| RequestError::Contended)?
                && root.status() == RootStatus::Live
            {
                barrier_valid(
                    &root,
                    phase,
                    root.removal_requested() && !root.is_internal(),
                )?;
            }
        }
    }
    if catalog.raw.generation_exhaustion_observed() {
        Err(RequestError::GenerationExhausted)
    } else {
        Err(RequestError::Contended)
    }
}
fn require_status<R: Payload>(root: &Root<R>, expected: RootStatus) -> Result<(), RequestError> {
    if !root.is_internal() && root.status() == expected {
        Ok(())
    } else {
        Err(RequestError::Conflict)
    }
}
pub(super) fn barrier_valid<R: Payload>(
    root: &Root<R>,
    phase: Phase,
    deleting: bool,
) -> Result<(), RequestError> {
    crate::canonical_catalog::state::root_barrier_plan(
        root.target_membership().ok_or(RequestError::Contended)?,
        phase,
        deleting,
    )
    .map(|_| ())
    .map_err(trace_error)
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
