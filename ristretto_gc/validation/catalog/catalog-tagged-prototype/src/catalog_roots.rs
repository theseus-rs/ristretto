//! Canonical root identity/visibility; this owns no runtime payload-access right.
use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum RootStatus {
    Prepared = 0,
    Live = 1,
    Retired = 2,
}

// Custody and visibility share one canonical atomic state. Token Drop only posts
// intent; it cannot race this effect as a competing status writer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub(super) enum RootState {
    Prepared = 0,
    Live = 1,
    Retired = 2,
    InternalLive = 3,
    InternalRetired = 4,
    ReservedPrepared = 5,
}

/// Reserved roles are independent so a stopped construction token cannot
/// consume a reference's notification location.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceRootKind {
    Construction,
    Notification,
}
impl ReferenceRootKind {
    fn index(self) -> usize {
        match self {
            Self::Construction => 0,
            Self::Notification => 1,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RootOrigin {
    External,
    Reference {
        identity: u64,
        slot: usize,
        kind: ReferenceRootKind,
    },
}

pub struct Root<R: Record = ()> {
    pub(super) owner: Arc<Owner>,
    pub identity: u64,
    slot: AtomicUsize,
    target: Pooled<Cell<R>>,
    origin: RootOrigin,
    status: AtomicU8,
    removal_requested: AtomicU8,
    #[cfg(test)]
    before_effect: super::super::Mutex<Option<Box<dyn FnOnce() + Send>>>,
}
impl<R: Record> Root<R> {
    /// Discovery location, immutable once announced. Reference-role roots never
    /// rotate, including while private.
    pub fn slot(&self) -> usize {
        self.slot.load(Ordering::Relaxed)
    }

    #[cfg(test)]
    pub(crate) fn pause_next_effect(&self, hook: impl FnOnce() + Send + 'static) {
        *self.before_effect.lock().unwrap() = Some(Box::new(hook));
    }

    pub fn status(&self) -> RootStatus {
        match self.state() {
            RootState::Prepared | RootState::ReservedPrepared => RootStatus::Prepared,
            RootState::Live | RootState::InternalLive => RootStatus::Live,
            RootState::Retired | RootState::InternalRetired => RootStatus::Retired,
        }
    }
    pub fn is_internal(&self) -> bool {
        self.origin != RootOrigin::External
            || matches!(
                self.state(),
                RootState::InternalLive | RootState::InternalRetired
            )
    }
    pub(super) fn state(&self) -> RootState {
        match self.status.load(Ordering::Acquire) {
            0 => RootState::Prepared,
            1 => RootState::Live,
            2 => RootState::Retired,
            3 => RootState::InternalLive,
            4 => RootState::InternalRetired,
            5 => RootState::ReservedPrepared,
            _ => unreachable!("invalid monotonic root state"),
        }
    }
    pub(super) fn matches_reference(
        &self,
        reference: &ReferenceSlot<R>,
        kind: ReferenceRootKind,
    ) -> bool {
        self.origin == RootOrigin::External
            || self.origin
                == RootOrigin::Reference {
                    identity: reference.identity,
                    slot: reference.slot,
                    kind,
                }
    }
    pub fn removal_requested(&self) -> bool {
        self.removal_requested.load(Ordering::Acquire) != 0
    }
    pub fn target_membership(&self) -> Option<Membership> {
        self.target.membership()
    }
    pub fn target_identity(&self) -> u64 {
        self.target.identity
    }
    pub fn target_slot(&self) -> usize {
        self.target.slot()
    }

    /// Borrow the exact target already owned by this entry, without acquiring an
    /// additional pool owner. Visibility and runtime access are checked separately.
    pub(crate) fn record(&self) -> &R {
        self.target.record()
    }
}

/// Unique root-token ownership. Drop posts a durable intent; helpers perform the
/// phase-coupled removal. A registry slot cannot be reused while this token lives,
/// including after explicit revocation. This raw token carries no payload access.
/// The bounded facade must additionally retain catalog charge ownership.
pub struct RootToken<R: Record = ()> {
    root: Pooled<Root<R>>,
    #[cfg(test)]
    after_request: Option<Box<dyn FnOnce() + Send + Sync>>,
}
impl<R: Record> std::ops::Deref for RootToken<R> {
    type Target = Root<R>;
    fn deref(&self) -> &Self::Target {
        &self.root
    }
}
impl<R: Record> RootToken<R> {
    #[cfg(test)]
    pub(crate) fn after_drop_request(&mut self, hook: impl FnOnce() + Send + Sync + 'static) {
        self.after_request = Some(Box::new(hook));
    }
    /// Raw experiment only; bounded handles do not expose owning root shares.
    pub fn try_share(&self) -> Option<Pooled<Root<R>>> {
        share(&self.root)
    }
}
impl<R: Record> Drop for RootToken<R> {
    fn drop(&mut self) {
        self.root.removal_requested.store(1, Ordering::Release);
        #[cfg(test)]
        if let Some(hook) = self.after_request.take() {
            hook();
        }
    }
}

pub(super) struct RootRegistry<R: Record> {
    pub(super) pool: Arc<Pool<Root<R>>>,
    slots: Box<[OptionalLink<Root<R>>]>,
}
pub(super) struct RootEffect<R: Record> {
    root: Pooled<Root<R>>,
    expected: RootState,
    proposed: RootState,
}
impl<R: Record> RootEffect<R> {
    pub(super) fn apply(&self) {
        #[cfg(test)]
        {
            let hook = self.root.before_effect.lock().unwrap().take();
            if let Some(hook) = hook {
                hook();
            }
        }
        let _ = self.root.status.compare_exchange(
            self.expected as u8,
            self.proposed as u8,
            Ordering::AcqRel,
            Ordering::Acquire,
        );
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RootPrepareError {
    Foreign,
    InvalidSlot,
    Occupied,
    Contended,
    IdentityExhausted,
}

enum RootLocation<R: Record> {
    External(usize),
    Reference {
        reference: Pooled<ReferenceSlot<R>>,
        kind: ReferenceRootKind,
    },
}
impl<R: Record> RootLocation<R> {
    fn link<'a>(&'a self, catalog: &'a Catalog<R>) -> &'a OptionalLink<Root<R>> {
        match self {
            Self::External(slot) => &catalog.roots.as_ref().expect("root pool").slots[*slot],
            Self::Reference { reference, kind } => &reference.roots[kind.index()],
        }
    }
}

/// Exactly one root-pool probe or announcement attempt per poll. No phase effect
/// occurs before announcement; after announcement only controllers change status.
pub struct RootCreation<R: Record = ()> {
    owner: Arc<Owner>,
    location: RootLocation<R>,
    private: Option<Root<R>>,
    root: Option<Pooled<Root<R>>>,
    finished: bool,
}
impl<R: Record> RootCreation<R> {
    pub fn poll(&mut self, catalog: &Catalog<R>) -> Result<Option<RootToken<R>>, CreationError> {
        self.poll_at(catalog, false)
    }
    /// Rotate only private external roots after a lost discovery announcement.
    /// Reference construction/notification roots retain their exact fixed role.
    pub fn poll_rotating(
        &mut self,
        catalog: &Catalog<R>,
    ) -> Result<Option<RootToken<R>>, CreationError> {
        self.poll_at(catalog, true)
    }
    fn poll_at(
        &mut self,
        catalog: &Catalog<R>,
        rotate: bool,
    ) -> Result<Option<RootToken<R>>, CreationError> {
        if !Arc::ptr_eq(&self.owner, &catalog.owner) {
            return Err(CreationError::Foreign);
        }
        if self.finished {
            return Err(CreationError::Finished);
        }
        if rotate && !matches!(self.location, RootLocation::External(_)) {
            return Err(CreationError::FixedLocation);
        }
        let registry = catalog.roots.as_ref().expect("same catalog root backing");
        if self.root.is_none() {
            let value = self.private.take().expect("unfinished private root");
            match registry.pool.try_allocate_status(value) {
                Ok(root) => self.root = Some(root),
                Err(failure) => {
                    self.private = Some(failure.value);
                    if failure.error == AllocationError::GenerationExhausted {
                        return Err(CreationError::GenerationExhausted);
                    }
                }
            }
            return Ok(None);
        }
        let root = self.root.as_ref().expect("prepared root");
        let Some(proposed) = share(root) else {
            return Ok(None);
        };
        if self.location.link(catalog).compare(None, Some(proposed)) {
            self.finished = true;
            Ok(Some(RootToken {
                root: self.root.take().expect("announced root token"),
                #[cfg(test)]
                after_request: None,
            }))
        } else if rotate {
            let RootLocation::External(slot) = &mut self.location else {
                unreachable!("rotating roots have external locations")
            };
            // No reference root or observer has ever owned this proposal.
            *slot = following_slot(*slot, catalog.external_root_capacity());
            root.slot.store(*slot, Ordering::Relaxed);
            Ok(None)
        } else {
            self.finished = true;
            self.root = None;
            Err(CreationError::Occupied)
        }
    }
}

impl<R: Record> Catalog<R> {
    pub fn with_record_root_capacities_and_limit(
        capacity: usize,
        controls: usize,
        memberships: usize,
        cells: usize,
        root_slots: usize,
        root_cells: usize,
        limit: u64,
    ) -> Self {
        assert!(
            root_slots == 0 || root_cells > 0,
            "root slots require root backing"
        );
        let mut catalog =
            Self::with_record_capacities_and_limit(capacity, controls, memberships, cells, limit);
        if root_cells > 0 {
            let pool = Pool::with_generation_limit(root_cells, limit);
            let slots = (0..root_slots)
                .map(|_| OptionalLink::empty(pool.clone()))
                .collect();
            catalog.roots = Some(RootRegistry { pool, slots });
        }
        catalog
    }
    pub fn record_root_backing_bytes(
        capacity: usize,
        controls: usize,
        memberships: usize,
        cells: usize,
        root_slots: usize,
        root_cells: usize,
    ) -> Option<usize> {
        let ordinary = Self::record_backing_bytes(capacity, controls, memberships, cells)?;
        if root_cells == 0 {
            return (root_slots == 0).then_some(ordinary);
        }
        ordinary
            .checked_add(
                std::alloc::Layout::array::<OptionalLink<Root<R>>>(root_slots)
                    .ok()?
                    .size(),
            )?
            .checked_add(Pool::<Root<R>>::requested_backing_bytes(root_cells)?)
    }
    pub fn external_root_capacity(&self) -> usize {
        self.roots.as_ref().map_or(0, |r| r.slots.len())
    }
    pub(crate) fn external_root_slot_occupied(&self, slot: usize) -> bool {
        self.roots
            .as_ref()
            .and_then(|registry| registry.slots.get(slot))
            .is_some_and(OptionalLink::is_occupied)
    }
    pub fn root_capacity(&self) -> usize {
        self.external_root_capacity() + self.reference_capacity() * 2
    }
    fn root_location(&self, slot: usize) -> Result<Option<RootLocation<R>>, Contended> {
        let external = self.external_root_capacity();
        if slot < external {
            return Ok(Some(RootLocation::External(slot)));
        }
        let offset = slot - external;
        let Some(reference) = self.discover_reference(offset / 2)? else {
            return Ok(None);
        };
        let kind = if offset.is_multiple_of(2) {
            ReferenceRootKind::Construction
        } else {
            ReferenceRootKind::Notification
        };
        Ok(Some(RootLocation::Reference { reference, kind }))
    }
    pub fn discover_root(&self, slot: usize) -> Result<Option<Pooled<Root<R>>>, Contended> {
        match self.root_location(slot)? {
            Some(location) => location.link(self).load(),
            None => Ok(None),
        }
    }
    pub fn begin_create_root(
        &self,
        slot: usize,
        target: Pooled<Cell<R>>,
    ) -> Result<RootCreation<R>, RootPrepareError> {
        if !Arc::ptr_eq(&target.owner, &self.owner) {
            return Err(RootPrepareError::Foreign);
        }
        let entry = self
            .roots
            .as_ref()
            .and_then(|r| r.slots.get(slot))
            .ok_or(RootPrepareError::InvalidSlot)?;
        if entry
            .load()
            .map_err(|_| RootPrepareError::Contended)?
            .is_some()
        {
            return Err(RootPrepareError::Occupied);
        }
        let identity = self.next_identity.load(Ordering::Relaxed);
        let next = identity
            .checked_add(1)
            .ok_or(RootPrepareError::IdentityExhausted)?;
        self.next_identity
            .compare_exchange(identity, next, Ordering::Relaxed, Ordering::Relaxed)
            .map_err(|_| RootPrepareError::Contended)?;
        Ok(RootCreation {
            owner: self.owner.clone(),
            location: RootLocation::External(slot),
            finished: false,
            root: None,
            private: Some(Root {
                owner: self.owner.clone(),
                identity,
                slot: AtomicUsize::new(slot),
                target,
                origin: RootOrigin::External,
                status: AtomicU8::new(RootState::Prepared as u8),
                removal_requested: AtomicU8::new(0),
                #[cfg(test)]
                before_effect: super::super::Mutex::new(None),
            }),
        })
    }
    pub fn release_retired_root(&self, root: &Pooled<Root<R>>) -> bool {
        if !Arc::ptr_eq(&root.owner, &self.owner)
            || root.status() != RootStatus::Retired
            || (root.origin == RootOrigin::External && !root.removal_requested())
        {
            return false;
        }
        self.root_location(root.slot())
            .ok()
            .flatten()
            .is_some_and(|location| location.link(self).compare(Some(root), None))
    }
    pub(super) fn attach_root_effect(
        &self,
        mut preparation: Preparation<R>,
        root: Pooled<Root<R>>,
        expected: RootState,
        proposed: RootState,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&root.owner, &self.owner) || root.state() != expected {
            return None;
        }
        preparation.root = Some(RootEffect {
            root,
            expected,
            proposed,
        });
        Some(preparation)
    }
    /// Observe a retained root under one stable controller revision. Failed
    /// owning loads or a changed revision are inconclusive. Root visibility by
    /// itself can precede publication certification; this check helps the
    /// controller and validates its completed state before reporting Live.
    /// The caller must separately establish runtime payload admission.
    pub fn root_is_stably_live_in(
        &self,
        root: &RootToken<R>,
        context: &R::Context,
    ) -> Option<bool> {
        if !Arc::ptr_eq(&root.owner, &self.owner) {
            return None;
        }
        let head = self.stable_head_in(context)?;
        let live = root.status() == RootStatus::Live;
        self.validates_head_in(&head, context).then_some(live)
    }

    /// Initial object publication and its root become visible under one decision.
    pub fn begin_publish_root(
        &self,
        snapshot: Snapshot<R>,
        root: Pooled<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&root.owner, &self.owner) || root.state() != RootState::Prepared {
            return None;
        }
        let preparation = self.begin_publish(snapshot, share(&root.target)?)?;
        self.attach_root_effect(preparation, root, RootState::Prepared, RootState::Live)
    }
    /// Cancel a prepared root independently of its target. A later publication
    /// attempt cannot activate it once this canonical effect wins.
    pub fn begin_cancel_root(
        &self,
        snapshot: Snapshot<R>,
        root: Pooled<Root<R>>,
    ) -> Option<Preparation<R>> {
        let expected = root.state();
        let proposed = match expected {
            RootState::Prepared => RootState::Retired,
            RootState::ReservedPrepared => RootState::InternalRetired,
            _ => return None,
        };
        let phase = snapshot.phase();
        let preparation = self.begin_prepare(snapshot, phase, None)?;
        self.attach_root_effect(preparation, root, expected, proposed)
    }
}

impl<R: Record> Catalog<R> {
    // Match the collector's owning-root/deletion barrier: request Strong, clamp
    // to each active domain's current floor, preserve births and exclusions, and
    // never generate ordinary work for logically retired targets. This grants no
    // conditional-reference upgrade or payload-access authority.
    pub(super) fn root_barrier(
        &self,
        target: Pooled<Cell<R>>,
        phase: Phase,
        deleting: bool,
    ) -> Option<(Option<Effect<R>>, Phase)> {
        self.barrier_effect(target.state.load()?, phase, deleting, || Some(target))
    }

    fn rooted_barrier(
        &self,
        root: &Root<R>,
        phase: Phase,
        deleting: bool,
    ) -> Option<(Option<Effect<R>>, Phase)> {
        // The root pins its target throughout planning. Acquire a separate
        // cell owner only if the resulting operation must retain an effect.
        self.barrier_effect(root.target.state.load()?, phase, deleting, || {
            share(&root.target)
        })
    }

    fn barrier_effect(
        &self,
        expected: Pooled<Membership>,
        phase: Phase,
        deleting: bool,
        retain: impl FnOnce() -> Option<Pooled<Cell<R>>>,
    ) -> Option<(Option<Effect<R>>, Phase)> {
        let (proposed, phase) = root_barrier_plan(*expected, phase, deleting).ok()?;
        let effect = if proposed != *expected {
            Some(self.effect(retain()?, expected, proposed)?)
        } else {
            None
        };
        Some((effect, phase))
    }

    /// A separately prepared root can activate only for an already tracing,
    /// non-retired target. Both domains' required work shares its visibility CAS.
    pub fn begin_activate_root(
        &self,
        snapshot: Snapshot<R>,
        root: Pooled<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&root.owner, &self.owner) || root.state() != RootState::Prepared {
            return None;
        }
        let (effect, phase) = self.rooted_barrier(&root, snapshot.phase(), false)?;
        let preparation = self.begin_prepare(snapshot, phase, effect)?;
        self.attach_root_effect(preparation, root, RootState::Prepared, RootState::Live)
    }

    /// Root removal preserves its deletion edge before exposing a later phase.
    /// Revoked/stale roots can be removed after target retirement without revival.
    pub fn begin_remove_root(
        &self,
        snapshot: Snapshot<R>,
        root: Pooled<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&root.owner, &self.owner) || root.state() != RootState::Live {
            return None;
        }
        let (effect, phase) = self.rooted_barrier(&root, snapshot.phase(), true)?;
        let preparation = self.begin_prepare(snapshot, phase, effect)?;
        self.attach_root_effect(preparation, root, RootState::Live, RootState::Retired)
    }

    /// One bounded root-slot observation. Failed owning acquisition cannot move
    /// the cursor; the existing single-slot API preserves its exact semantics.
    pub fn begin_scan_root(&self, snapshot: Snapshot<R>, domain: usize) -> Option<Preparation<R>> {
        self.begin_scan_roots(snapshot, domain, 1)
    }

    /// Inspect at most eight consecutive slots, stopping at the first effect.
    /// Cursor and at most one root/target effect share the ordinary controller
    /// decision. Any failed observation abandons the entire private preparation.
    pub fn begin_scan_root_batch(
        &self,
        snapshot: Snapshot<R>,
        domain: usize,
    ) -> Option<Preparation<R>> {
        self.begin_scan_roots(snapshot, domain, 8)
    }

    fn begin_scan_roots(
        &self,
        snapshot: Snapshot<R>,
        domain: usize,
        limit: usize,
    ) -> Option<Preparation<R>> {
        let mut phase = snapshot.phase();
        let entry = phase.domains.get(domain)?;
        if entry.mode == DomainMode::Inactive || entry.floor != 0 {
            return None;
        }
        let slot = phase.root_cursors[domain];
        if slot >= self.root_capacity() {
            return None;
        }
        let mut cursor = slot;
        let mut root_effect = None;
        let mut effect = None;
        for _ in 0..limit {
            let root = self.discover_root(cursor).ok()?;
            effect = if let Some(root) = root {
                match (root.state(), root.removal_requested()) {
                    (RootState::Live, true) => {
                        let (effect, next) = self.rooted_barrier(&root, phase, true)?;
                        phase = next;
                        root_effect = Some(RootEffect {
                            root,
                            expected: RootState::Live,
                            proposed: RootState::Retired,
                        });
                        effect
                    }
                    (state @ (RootState::Prepared | RootState::ReservedPrepared), true) => {
                        root_effect = Some(RootEffect {
                            root,
                            expected: state,
                            proposed: if state == RootState::Prepared {
                                RootState::Retired
                            } else {
                                RootState::InternalRetired
                            },
                        });
                        None
                    }
                    (RootState::Live, false) | (RootState::InternalLive, _) => {
                        let (effect, next) = self.rooted_barrier(&root, phase, false)?;
                        phase = next;
                        effect
                    }
                    _ => None,
                }
            } else {
                None
            };
            cursor = cursor.checked_add(1)?;
            // Retain at most the existing single root/membership effect. Empty,
            // retired and already-covered slots need no independent commit.
            if effect.is_some() || root_effect.is_some() || cursor == self.root_capacity() {
                break;
            }
        }
        phase.root_cursors[domain] = cursor;
        let mut preparation = self.begin_prepare(snapshot, phase, effect)?;
        preparation.root = root_effect;
        Some(preparation)
    }

    /// Complete a durable drop request without requiring an active collection.
    /// Callers still need a fair bounded scan/help policy and exact slot release.
    pub fn begin_requested_root_removal(
        &self,
        snapshot: Snapshot<R>,
        root: Pooled<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !root.removal_requested() {
            return None;
        }
        match root.state() {
            RootState::Prepared | RootState::ReservedPrepared => {
                self.begin_cancel_root(snapshot, root)
            }
            RootState::Live => self.begin_remove_root(snapshot, root),
            RootState::Retired | RootState::InternalLive | RootState::InternalRetired => None,
        }
    }

    #[cfg(test)]
    pub fn root_pool_occupancy(&self) -> usize {
        self.roots.as_ref().map_or(0, |roots| roots.pool.occupied())
    }
}

// Shared pure validation for owning preparation and copied facade diagnostics.
pub(crate) fn root_barrier_plan(
    membership: Membership,
    mut phase: Phase,
    deleting: bool,
) -> Result<(Membership, Phase), super::super::TraceError> {
    use super::super::{Generation, TraceError};
    let Membership::Tracing(original) = membership else {
        return if deleting && membership == Membership::Retired {
            Ok((membership, phase))
        } else {
            Err(TraceError::Conflict)
        };
    };
    if original.logically_retired(phase) {
        return if deleting {
            Ok((membership, phase))
        } else {
            Err(TraceError::Conflict)
        };
    }
    let mut marks = original;
    for index in 0..2 {
        let domain = phase.domains[index];
        if domain.mode == DomainMode::Inactive
            || domain.floor >= RETIREMENT_FLOOR
            || (domain.mode == DomainMode::Young && marks.generation == Generation::Old)
            || (domain.mode == DomainMode::Old
                && marks.generation == Generation::Young
                && marks[index].epoch == domain.epoch
                && marks[index].born)
        {
            continue;
        }
        match marks.shade(phase, index, TraceRetention::Strong) {
            Ok((next, changed)) => {
                marks = next;
                phase = changed;
            }
            Err(TraceError::AlreadyMarked) => {}
            Err(error) => return Err(error),
        }
    }
    Ok((Membership::Tracing(marks), phase))
}

impl<R: Record> Catalog<R> {
    /// Combine an already prepared referent effect with notification-owner
    /// rooting. When both name the same cell, merge into one exact membership
    /// replacement, preserving its original expectation and all counted work.
    pub(super) fn begin_hold_root_after(
        &self,
        snapshot: Snapshot<R>,
        root: Pooled<Root<R>>,
        phase: Phase,
        effect: Option<Effect<R>>,
    ) -> Option<Preparation<R>> {
        let root_state = root.state();
        if !Arc::ptr_eq(&root.owner, &self.owner)
            || !matches!(
                root_state,
                RootState::Prepared | RootState::ReservedPrepared
            )
        {
            return None;
        }
        let (effect, secondary, phase) = match effect {
            Some(effect) if Pooled::ptr_eq(&effect.cell, &root.target) => {
                let (proposed, phase) = root_barrier_plan(*effect.proposed, phase, false).ok()?;
                let effect = self.effect(effect.cell, effect.expected, proposed)?;
                (Some(effect), None, phase)
            }
            effect => {
                let (owner_effect, phase) = self.rooted_barrier(&root, phase, false)?;
                if effect.is_some() {
                    (effect, owner_effect, phase)
                } else {
                    (owner_effect, None, phase)
                }
            }
        };
        let mut preparation = self.begin_prepare(snapshot, phase, effect)?;
        preparation.secondary_effect = secondary;
        self.attach_root_effect(preparation, root, root_state, RootState::InternalLive)
    }
    /// Transfer a held root to a fresh caller root. Both roots have the same
    /// target, so activation's strongest barrier also preserves deletion work.
    pub(super) fn begin_transfer_held_root(
        &self,
        snapshot: Snapshot<R>,
        held: Pooled<Root<R>>,
        caller: Pooled<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&held.owner, &self.owner)
            || !Arc::ptr_eq(&caller.owner, &self.owner)
            || held.state() != RootState::InternalLive
            || caller.state() != RootState::Prepared
            || held.target.identity != caller.target.identity
        {
            return None;
        }
        let mut preparation = self.begin_activate_root(snapshot, caller)?;
        preparation.secondary_root = Some(RootEffect {
            root: held,
            expected: RootState::InternalLive,
            proposed: RootState::InternalRetired,
        });
        Some(preparation)
    }
}

impl<R: Record> Catalog<R> {
    pub(super) fn begin_release_held_root(
        &self,
        snapshot: Snapshot<R>,
        held: Pooled<Root<R>>,
        phase: Phase,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&held.owner, &self.owner) || held.state() != RootState::InternalLive {
            return None;
        }
        let (effect, phase) = self.rooted_barrier(&held, phase, true)?;
        let preparation = self.begin_prepare(snapshot, phase, effect)?;
        self.attach_root_effect(
            preparation,
            held,
            RootState::InternalLive,
            RootState::InternalRetired,
        )
    }

    /// Initial owner publication remains the primary guarded record effect.
    /// The secondary effect only preserves deletion work for its tracing referent.
    pub(super) fn begin_publish_root_releasing(
        &self,
        snapshot: Snapshot<R>,
        owner: Pooled<Root<R>>,
        held: Pooled<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&owner.owner, &self.owner)
            || !Arc::ptr_eq(&held.owner, &self.owner)
            || owner.state() != RootState::Prepared
            || held.state() != RootState::InternalLive
            || Pooled::ptr_eq(&owner.target, &held.target)
        {
            return None;
        }
        let expected = owner.target.state.load()?;
        if *expected != Membership::Prepared {
            return None;
        }
        let (marks, phase) = TraceState::publication(snapshot.phase()).ok()?;
        let effect = self.effect(share(&owner.target)?, expected, Membership::Tracing(marks))?;
        let (secondary, phase) = self.rooted_barrier(&held, phase, true)?;
        let mut preparation = self.begin_prepare(snapshot, phase, Some(effect))?;
        preparation.secondary_effect = secondary;
        preparation.secondary_root = Some(RootEffect {
            root: held,
            expected: RootState::InternalLive,
            proposed: RootState::InternalRetired,
        });
        self.attach_root_effect(preparation, owner, RootState::Prepared, RootState::Live)
    }
}

impl<R: Record> Catalog<R> {
    /// Private creation owns this exact reference object through announcement.
    /// A stale announcement can never occupy the replacement reference's link.
    pub fn begin_create_reference_root(
        &self,
        reference: Pooled<ReferenceSlot<R>>,
        kind: ReferenceRootKind,
        target: Pooled<Cell<R>>,
    ) -> Result<RootCreation<R>, RootPrepareError> {
        if !Arc::ptr_eq(&reference.owner, &self.owner) || !Arc::ptr_eq(&target.owner, &self.owner) {
            return Err(RootPrepareError::Foreign);
        }
        let current = self
            .discover_reference(reference.slot)
            .map_err(|_| RootPrepareError::Contended)?
            .ok_or(RootPrepareError::InvalidSlot)?;
        if !Pooled::ptr_eq(&current, &reference) {
            return Err(RootPrepareError::InvalidSlot);
        }
        if reference.roots[kind.index()]
            .load()
            .map_err(|_| RootPrepareError::Contended)?
            .is_some()
        {
            return Err(RootPrepareError::Occupied);
        }
        let identity = self.next_identity.load(Ordering::Relaxed);
        let next = identity
            .checked_add(1)
            .ok_or(RootPrepareError::IdentityExhausted)?;
        self.next_identity
            .compare_exchange(identity, next, Ordering::Relaxed, Ordering::Relaxed)
            .map_err(|_| RootPrepareError::Contended)?;
        let slot = self.external_root_capacity() + reference.slot * 2 + kind.index();
        let origin = RootOrigin::Reference {
            identity: reference.identity,
            slot: reference.slot,
            kind,
        };
        Ok(RootCreation {
            owner: self.owner.clone(),
            location: RootLocation::Reference { reference, kind },
            private: Some(Root {
                owner: self.owner.clone(),
                identity,
                slot: AtomicUsize::new(slot),
                target,
                origin,
                status: AtomicU8::new(RootState::ReservedPrepared as u8),
                removal_requested: AtomicU8::new(0),
                #[cfg(test)]
                before_effect: super::super::Mutex::new(None),
            }),
            root: None,
            finished: false,
        })
    }
}
