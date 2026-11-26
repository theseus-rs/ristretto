//! Canonical root identity/visibility; this owns no runtime payload-access right.
use super::super::{DomainMode, Phase, RETIREMENT_FLOOR, TraceRetention};
use super::{
    AllocationError, Arc, AtomicU8, AtomicUsize, Catalog, Cell, Contended, CreationError, Effect,
    Membership, OptionalLink, Ordering, Owned, Owner, Pool, Preparation, Record, Snapshot,
    following_slot, share,
};
use crate::canonical_catalog::{DOMAIN_COUNT, STRONG_FLOOR};
use std::sync::atomic::AtomicBool;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum RootStatus {
    Prepared = 0,
    Live = 1,
    Retired = 2,
}

const ROOT_SCAN_BATCH: usize = 8;

pub struct Root<R: Record = ()> {
    pub(super) owner: Arc<Owner>,
    pub identity: u64,
    slot: AtomicUsize,
    target: Owned<Cell<R>>,
    status: AtomicU8,
    removal_requested: AtomicBool,
    #[cfg(test)]
    before_effect: std::sync::Mutex<Option<Box<dyn FnOnce() + Send>>>,
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
        match self.status.load(Ordering::Acquire) {
            state if state == RootStatus::Prepared as u8 => RootStatus::Prepared,
            state if state == RootStatus::Live as u8 => RootStatus::Live,
            state if state == RootStatus::Retired as u8 => RootStatus::Retired,
            _ => unreachable!("invalid monotonic root state"),
        }
    }

    pub fn is_internal(&self) -> bool {
        false
    }

    pub fn removal_requested(&self) -> bool {
        self.removal_requested.load(Ordering::Acquire)
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
    root: Owned<Root<R>>,
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
    #[must_use]
    pub fn try_share(&self) -> Option<Owned<Root<R>>> {
        share(&self.root)
    }
}
impl<R: Record> Drop for RootToken<R> {
    fn drop(&mut self) {
        self.root.removal_requested.store(true, Ordering::Release);
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
    root: Owned<Root<R>>,
    expected: RootStatus,
    proposed: RootStatus,
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

enum RootLocation {
    External(usize),
}
impl RootLocation {
    #[expect(
        clippy::indexing_slicing,
        reason = "root locations are created only after fixed registry bounds validation"
    )]
    fn link<'a, R: Record>(&'a self, catalog: &'a Catalog<R>) -> &'a OptionalLink<Root<R>> {
        let Self::External(slot) = self;
        &catalog.roots.as_ref().expect("root pool").slots[*slot]
    }
}

/// Exactly one root-pool probe or announcement attempt per poll. No phase effect
/// occurs before announcement; after announcement only controllers change status.
pub struct RootCreation<R: Record = ()> {
    owner: Arc<Owner>,
    location: RootLocation,
    private: Option<Root<R>>,
    root: Option<Owned<Root<R>>>,
    finished: bool,
}
impl<R: Record> RootCreation<R> {
    pub fn poll(&mut self, catalog: &Catalog<R>) -> Result<Option<RootToken<R>>, CreationError> {
        self.poll_at(catalog, false)
    }

    /// Rotate only private external roots after a lost discovery announcement.
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
            let RootLocation::External(slot) = &mut self.location;
            // No other root or observer has ever owned this private proposal.
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
    #[must_use]
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

    #[must_use]
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
        self.external_root_capacity()
    }

    fn root_location(&self, slot: usize) -> Option<RootLocation> {
        (slot < self.external_root_capacity()).then_some(RootLocation::External(slot))
    }

    pub fn discover_root(&self, slot: usize) -> Result<Option<Owned<Root<R>>>, Contended> {
        match self.root_location(slot) {
            Some(location) => location.link(self).load(),
            None => Ok(None),
        }
    }

    pub fn begin_create_root(
        &self,
        slot: usize,
        target: Owned<Cell<R>>,
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
                status: AtomicU8::new(RootStatus::Prepared as u8),
                removal_requested: AtomicBool::new(false),
                #[cfg(test)]
                before_effect: std::sync::Mutex::new(None),
            }),
        })
    }

    pub fn release_retired_root(&self, root: &Owned<Root<R>>) -> bool {
        if !Arc::ptr_eq(&root.owner, &self.owner)
            || root.status() != RootStatus::Retired
            || !root.removal_requested()
        {
            return false;
        }
        self.root_location(root.slot())
            .is_some_and(|location| location.link(self).compare(Some(root), None))
    }
}

#[cfg(test)]
#[path = "root_backing_tests.rs"]
mod root_backing_tests;

impl<R: Record> Catalog<R> {
    pub(super) fn attach_root_effect(
        &self,
        mut preparation: Preparation<R>,
        root: Owned<Root<R>>,
        expected: RootStatus,
        proposed: RootStatus,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&root.owner, &self.owner) || root.status() != expected {
            return None;
        }
        preparation.root = Some(RootEffect {
            root,
            expected,
            proposed,
        });
        Some(preparation)
    }

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

    pub fn begin_publish_root(
        &self,
        snapshot: Snapshot<R>,
        root: Owned<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&root.owner, &self.owner) || root.status() != RootStatus::Prepared {
            return None;
        }
        let preparation = self.begin_publish(snapshot, share(&root.target)?)?;
        self.attach_root_effect(preparation, root, RootStatus::Prepared, RootStatus::Live)
    }

    pub fn begin_cancel_root(
        &self,
        snapshot: Snapshot<R>,
        root: Owned<Root<R>>,
    ) -> Option<Preparation<R>> {
        let expected = root.status();
        let proposed = match expected {
            RootStatus::Prepared => RootStatus::Retired,
            _ => return None,
        };
        let phase = snapshot.phase();
        let preparation = self.begin_prepare(snapshot, phase, None)?;
        self.attach_root_effect(preparation, root, expected, proposed)
    }
}

#[cfg(test)]
#[path = "root_publication_tests.rs"]
mod root_publication_tests;

impl<R: Record> Catalog<R> {
    pub(super) fn root_barrier(
        &self,
        target: Owned<Cell<R>>,
        phase: Phase,
        deleting: bool,
    ) -> Option<(Option<Effect<R>>, Phase)> {
        let expected = target.state.load()?;
        let (proposed, phase) = root_barrier_plan(*expected, phase, deleting).ok()?;
        let effect = if proposed == *expected {
            None
        } else {
            Some(self.effect(target, expected, proposed)?)
        };
        Some((effect, phase))
    }

    /// A separately prepared root can activate only for an already tracing,
    /// non-retired target. Both domains' required work shares its visibility CAS.
    pub fn begin_activate_root(
        &self,
        snapshot: Snapshot<R>,
        root: Owned<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&root.owner, &self.owner) || root.status() != RootStatus::Prepared {
            return None;
        }
        let (effect, phase) = self.root_barrier(share(&root.target)?, snapshot.phase(), false)?;
        let preparation = self.begin_prepare(snapshot, phase, effect)?;
        self.attach_root_effect(preparation, root, RootStatus::Prepared, RootStatus::Live)
    }

    /// Root removal preserves its deletion edge before exposing a later phase.
    /// Revoked/stale roots can be removed after target retirement without revival.
    pub fn begin_remove_root(
        &self,
        snapshot: Snapshot<R>,
        root: Owned<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&root.owner, &self.owner) || root.status() != RootStatus::Live {
            return None;
        }
        let (effect, phase) = self.root_barrier(share(&root.target)?, snapshot.phase(), true)?;
        let preparation = self.begin_prepare(snapshot, phase, effect)?;
        self.attach_root_effect(preparation, root, RootStatus::Live, RootStatus::Retired)
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
        self.begin_scan_roots(snapshot, domain, ROOT_SCAN_BATCH)
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "the domain is checked by get before fixed phase-array access"
    )]
    fn begin_scan_roots(
        &self,
        snapshot: Snapshot<R>,
        domain: usize,
        limit: usize,
    ) -> Option<Preparation<R>> {
        let mut phase = snapshot.phase();
        let entry = phase.domains.get(domain)?;
        if entry.mode == DomainMode::Inactive || entry.floor != STRONG_FLOOR {
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
                match (root.status(), root.removal_requested()) {
                    (RootStatus::Live, true) => {
                        let (effect, next) =
                            self.root_barrier(share(&root.target)?, phase, true)?;
                        phase = next;
                        root_effect = Some(RootEffect {
                            root,
                            expected: RootStatus::Live,
                            proposed: RootStatus::Retired,
                        });
                        effect
                    }
                    (state @ RootStatus::Prepared, true) => {
                        root_effect = Some(RootEffect {
                            root,
                            expected: state,
                            proposed: RootStatus::Retired,
                        });
                        None
                    }
                    (RootStatus::Live, false) => {
                        let (effect, next) =
                            self.root_barrier(share(&root.target)?, phase, false)?;
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
        root: Owned<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !root.removal_requested() {
            return None;
        }
        match root.status() {
            RootStatus::Prepared => self.begin_cancel_root(snapshot, root),
            RootStatus::Live => self.begin_remove_root(snapshot, root),
            RootStatus::Retired => None,
        }
    }
}

// Shared pure validation for owning preparation and copied facade diagnostics.
#[expect(
    clippy::indexing_slicing,
    reason = "the loop is bounded by the fixed two-domain arrays"
)]
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
    for index in 0..DOMAIN_COUNT {
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

#[cfg(test)]
#[path = "root_maintenance_tests.rs"]
mod root_maintenance_tests;
