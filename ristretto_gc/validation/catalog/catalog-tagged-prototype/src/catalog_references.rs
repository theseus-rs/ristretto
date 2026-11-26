//! Exact conditional-slot versions, construction custody and phase-coupled
//! notification/root transfer. Runtime payload traversal is a separate adapter.
use super::super::reference::{
    ObjectKey, ReferenceKind, ReferenceRoot, ReferenceState, ReferenceTarget,
};
use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceMembership {
    Prepared,
    Live(ReferenceState),
    Retired,
}
pub struct ReferenceSlot<R: Record = ()> {
    pub(super) owner: Arc<Owner>,
    pub(super) roots: [OptionalLink<Root<R>>; 2],
    pub identity: u64,
    pub slot: usize,
    state: PooledLink<ReferenceMembership>,
    removal_requested: AtomicU8,
    #[cfg(test)]
    before_effect: super::super::Mutex<Option<Box<dyn FnOnce() + Send>>>,
}
impl<R: Record> ReferenceSlot<R> {
    #[cfg(all(test, not(target_family = "wasm")))]
    pub(crate) fn pause_next_effect(&self, hook: impl FnOnce() + Send + 'static) {
        *self.before_effect.lock().unwrap() = Some(Box::new(hook));
    }
    pub fn membership(&self) -> Option<ReferenceMembership> {
        self.state.load().map(|state| *state)
    }
    pub fn removal_requested(&self) -> bool {
        self.removal_requested.load(Ordering::Acquire) != 0
    }
}
pub struct ReferenceToken<R: Record = ()> {
    slot: Pooled<ReferenceSlot<R>>,
    #[cfg(test)]
    after_request: Option<Box<dyn FnOnce() + Send + Sync>>,
}
impl<R: Record> std::ops::Deref for ReferenceToken<R> {
    type Target = ReferenceSlot<R>;
    fn deref(&self) -> &Self::Target {
        &self.slot
    }
}
impl<R: Record> ReferenceToken<R> {
    #[cfg(test)]
    pub(crate) fn after_drop_request(&mut self, hook: impl FnOnce() + Send + Sync + 'static) {
        self.after_request = Some(Box::new(hook));
    }
    /// Raw experiment only. A bounded facade must not let owning shares escape.
    pub fn try_share(&self) -> Option<Pooled<ReferenceSlot<R>>> {
        share(&self.slot)
    }
}
impl<R: Record> Drop for ReferenceToken<R> {
    fn drop(&mut self) {
        self.slot.removal_requested.store(1, Ordering::Release);
        #[cfg(test)]
        if let Some(hook) = self.after_request.take() {
            hook();
        }
    }
}
pub(super) struct ReferenceRegistry<R: Record = ()> {
    pub(super) slots_pool: Arc<Pool<ReferenceSlot<R>>>,
    pub(super) states: Arc<Pool<ReferenceMembership>>,
    slots: Box<[OptionalLink<ReferenceSlot<R>>]>,
}
pub(super) struct ReferenceEffect<R: Record = ()> {
    slot: Pooled<ReferenceSlot<R>>,
    expected: Pooled<ReferenceMembership>,
    pub(super) proposed: Pooled<ReferenceMembership>,
}
impl<R: Record> ReferenceEffect<R> {
    pub(super) fn apply(&self, proposed: Pooled<ReferenceMembership>) {
        // Only canonical controllers replace these never-reinstalled versions.
        // Strong CAS failure means this exact effect has already been superseded.
        #[cfg(test)]
        {
            let hook = self.slot.before_effect.lock().unwrap().take();
            if let Some(hook) = hook {
                hook();
            }
        }
        self.slot.state.compare(&self.expected, proposed);
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferencePrepareError {
    InvalidSlot,
    Occupied,
    Contended,
    IdentityExhausted,
}
pub struct ReferenceCreation<R: Record = ()> {
    owner: Arc<Owner>,
    identity: u64,
    slot: usize,
    membership: Option<Pooled<ReferenceMembership>>,
    private: Option<ReferenceSlot<R>>,
    prepared: Option<Pooled<ReferenceSlot<R>>>,
    finished: bool,
}
impl<R: Record> ReferenceCreation<R> {
    /// One selected pool probe or announcement attempt per poll.
    pub fn poll(
        &mut self,
        catalog: &Catalog<R>,
    ) -> Result<Option<ReferenceToken<R>>, CreationError> {
        if !Arc::ptr_eq(&self.owner, &catalog.owner) {
            return Err(CreationError::Foreign);
        }
        if self.finished {
            return Err(CreationError::Finished);
        }
        let refs = catalog
            .references
            .as_ref()
            .expect("same catalog reference backing");
        if self.membership.is_none() && self.private.is_none() && self.prepared.is_none() {
            match refs
                .states
                .try_allocate_status(ReferenceMembership::Prepared)
            {
                Ok(state) => self.membership = Some(state),
                Err(failure) if failure.error == AllocationError::GenerationExhausted => {
                    return Err(CreationError::GenerationExhausted);
                }
                Err(_) => {}
            }
            return Ok(None);
        }
        if self.prepared.is_none() {
            let private = self.private.take().unwrap_or_else(|| ReferenceSlot {
                owner: self.owner.clone(),
                identity: self.identity,
                slot: self.slot,
                state: PooledLink::new(
                    refs.states.clone(),
                    self.membership.take().expect("private state"),
                ),
                roots: std::array::from_fn(|_| {
                    OptionalLink::empty(
                        catalog
                            .roots
                            .as_ref()
                            .expect("reference root pool")
                            .pool
                            .clone(),
                    )
                }),
                removal_requested: AtomicU8::new(0),
                #[cfg(test)]
                before_effect: super::super::Mutex::new(None),
            });
            match refs.slots_pool.try_allocate_status(private) {
                Ok(slot) => self.prepared = Some(slot),
                Err(failure) => {
                    self.private = Some(failure.value);
                    if failure.error == AllocationError::GenerationExhausted {
                        return Err(CreationError::GenerationExhausted);
                    }
                }
            }
            return Ok(None);
        }
        let slot = self.prepared.as_ref().expect("private slot");
        let Some(proposed) = share(slot) else {
            return Ok(None);
        };
        self.finished = true;
        if refs.slots[self.slot].compare(None, Some(proposed)) {
            Ok(Some(ReferenceToken {
                slot: self.prepared.take().expect("announced slot"),
                #[cfg(test)]
                after_request: None,
            }))
        } else {
            self.prepared = None;
            Err(CreationError::Occupied)
        }
    }
}
impl<R: Record> Catalog<R> {
    /// Consumes an unshared setup value. Existing reference backing cannot be replaced.
    pub fn with_references(
        mut self,
        slots: usize,
        entries: usize,
        states: usize,
        limit: u64,
    ) -> Self {
        assert!(
            self.references.is_none(),
            "reference backing already configured"
        );
        assert_eq!(
            Arc::strong_count(&self.owner),
            1,
            "setup owner already shared"
        );
        assert_eq!(
            self.next_identity.load(Ordering::Relaxed),
            1,
            "setup identity already used"
        );
        assert!(
            matches!(&*self.head.load().expect("fresh setup head"), Control::Stable(phase) if phase.revision == 1),
            "setup phase already used"
        );
        if slots > 0 {
            assert!(
                self.roots.is_some(),
                "references require a configured root pool"
            );
            self.root_capacity()
                .checked_add(slots.checked_mul(2).expect("reference roots overflow"))
                .expect("root capacity overflow");
            let slots_pool = Pool::with_generation_limit(entries, limit);
            let states = Pool::with_generation_limit(states, limit);
            let slots = (0..slots)
                .map(|_| OptionalLink::empty(slots_pool.clone()))
                .collect();
            self.references = Some(ReferenceRegistry {
                slots_pool,
                states,
                slots,
            });
        }
        self
    }
    /// Additional requested backing; inline Catalog/Control growth is charged by
    /// their existing generic layout calculation. Runtime quotas are not derived yet.
    pub fn reference_backing_bytes(slots: usize, entries: usize, states: usize) -> Option<usize> {
        if slots == 0 {
            return Some(0);
        }
        std::alloc::Layout::array::<OptionalLink<ReferenceSlot<R>>>(slots)
            .ok()?
            .size()
            .checked_add(Pool::<ReferenceSlot<R>>::requested_backing_bytes(entries)?)?
            .checked_add(Pool::<ReferenceMembership>::requested_backing_bytes(
                states,
            )?)
    }
    pub fn reference_capacity(&self) -> usize {
        self.references.as_ref().map_or(0, |refs| refs.slots.len())
    }
    pub fn discover_reference(
        &self,
        slot: usize,
    ) -> Result<Option<Pooled<ReferenceSlot<R>>>, Contended> {
        self.references
            .as_ref()
            .and_then(|refs| refs.slots.get(slot))
            .map_or(Ok(None), OptionalLink::load)
    }
    pub fn begin_create_reference(
        &self,
        slot: usize,
    ) -> Result<ReferenceCreation<R>, ReferencePrepareError> {
        let entry = self
            .references
            .as_ref()
            .and_then(|refs| refs.slots.get(slot))
            .ok_or(ReferencePrepareError::InvalidSlot)?;
        if entry
            .load()
            .map_err(|_| ReferencePrepareError::Contended)?
            .is_some()
        {
            return Err(ReferencePrepareError::Occupied);
        }
        let identity = self.next_identity.load(Ordering::Relaxed);
        let next = identity
            .checked_add(1)
            .ok_or(ReferencePrepareError::IdentityExhausted)?;
        self.next_identity
            .compare_exchange(identity, next, Ordering::Relaxed, Ordering::Relaxed)
            .map_err(|_| ReferencePrepareError::Contended)?;
        Ok(ReferenceCreation {
            owner: self.owner.clone(),
            identity,
            slot,
            membership: None,
            private: None,
            prepared: None,
            finished: false,
        })
    }
    fn reference_effect(
        &self,
        mut preparation: Preparation<R>,
        slot: Pooled<ReferenceSlot<R>>,
        expected: Pooled<ReferenceMembership>,
        proposed: ReferenceMembership,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&slot.owner, &self.owner) {
            return None;
        }
        preparation.reference = Some(ReferenceEffect {
            slot,
            expected,
            proposed: allocate(&self.references.as_ref()?.states, proposed)?,
        });
        Some(preparation)
    }
    fn reference_target(&self, key: ObjectKey) -> Option<ReferenceTarget> {
        let cell = self.discover(key.slot).ok()??;
        if cell.identity != key.identity {
            return None;
        }
        Some(ReferenceTarget {
            key,
            membership: cell.membership()?,
        })
    }
    /// Publish a new host reference from an exact currently live root. Owner
    /// binding requires the later construction-reservation protocol.
    pub fn begin_publish_reference(
        &self,
        snapshot: Snapshot<R>,
        slot: Pooled<ReferenceSlot<R>>,
        source: Pooled<Root<R>>,
        kind: ReferenceKind,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&slot.owner, &self.owner)
            || !Arc::ptr_eq(&source.owner, &self.owner)
            || source.status() != RootStatus::Live
            || source.is_internal()
        {
            return None;
        }
        let expected = slot.state.load()?;
        if *expected != ReferenceMembership::Prepared {
            return None;
        }
        let phase = snapshot.phase();
        let target = ObjectKey {
            identity: source.target_identity(),
            slot: source.target_slot(),
        };
        let observed = self.reference_target(target)?;
        let Membership::Tracing(marks) = observed.membership else {
            return None;
        };
        if marks.logically_retired(phase) {
            return None;
        }
        let proposed = ReferenceState::created(kind, target, None, phase);
        let preparation = self.begin_prepare(snapshot, phase, None)?;
        self.reference_effect(
            preparation,
            slot,
            expected,
            ReferenceMembership::Live(proposed),
        )
    }
    /// Preserve source decisions. Missing targets produce a cleared clone only
    /// after exact absence is observed; failed owning acquisition must retry.
    pub fn begin_clone_reference(
        &self,
        snapshot: Snapshot<R>,
        source: Pooled<ReferenceSlot<R>>,
        destination: Pooled<ReferenceSlot<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&source.owner, &self.owner) || !Arc::ptr_eq(&destination.owner, &self.owner)
        {
            return None;
        }
        let source_state = source.state.load()?;
        let ReferenceMembership::Live(source_state) = *source_state else {
            return None;
        };
        let expected = destination.state.load()?;
        if *expected != ReferenceMembership::Prepared {
            return None;
        }
        let phase = snapshot.phase();
        let target = if let Some(key) = source_state.target {
            self.observe_reference_target(key)?
        } else {
            None
        };
        let proposed = source_state.cloned_at(phase, target);
        let preparation = self.begin_prepare(snapshot, phase, None)?;
        self.reference_effect(
            preparation,
            destination,
            expected,
            ReferenceMembership::Live(proposed),
        )
    }
    // Outer None means contention; inner None means a missing/replaced target.
    fn observe_reference_target(&self, key: ObjectKey) -> Option<Option<ReferenceTarget>> {
        let Some(cell) = self.discover(key.slot).ok()? else {
            return Some(None);
        };
        if cell.identity != key.identity {
            return Some(None);
        }
        Some(Some(ReferenceTarget {
            key,
            membership: cell.membership()?,
        }))
    }
    pub fn begin_clear_reference(
        &self,
        snapshot: Snapshot<R>,
        slot: Pooled<ReferenceSlot<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&slot.owner, &self.owner) {
            return None;
        }
        let expected = slot.state.load()?;
        let ReferenceMembership::Live(state) = *expected else {
            return None;
        };
        state.target?;
        let phase = snapshot.phase();
        let preparation = self.begin_prepare(snapshot, phase, None)?;
        self.reference_effect(
            preparation,
            slot,
            expected,
            ReferenceMembership::Live(state.cleared()),
        )
    }
    /// Eligibility, target shading and activation of the supplied prepared root
    /// use one expected controller. A competing clear/closure invalidates it.
    pub fn begin_upgrade_reference(
        &self,
        snapshot: Snapshot<R>,
        slot: Pooled<ReferenceSlot<R>>,
        root: Pooled<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&slot.owner, &self.owner) || !Arc::ptr_eq(&root.owner, &self.owner) {
            return None;
        }
        let state = slot.state.load()?;
        let ReferenceMembership::Live(state) = *state else {
            return None;
        };
        let key = ObjectKey {
            identity: root.target_identity(),
            slot: root.target_slot(),
        };
        if state.target != Some(key) {
            return None;
        }
        let target = ReferenceTarget {
            key,
            membership: root.target_membership()?,
        };
        if !state.upgrade_eligible(snapshot.phase(), Some(target)) {
            return None;
        }
        self.begin_activate_root(snapshot, root)
    }
    /// A dropped slot remains discoverable until pending notifications are dealt
    /// with. Notification processing itself is not implemented in this candidate.
    pub fn begin_retire_reference(
        &self,
        snapshot: Snapshot<R>,
        slot: Pooled<ReferenceSlot<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&slot.owner, &self.owner) || !slot.removal_requested() {
            return None;
        }
        let expected = slot.state.load()?;
        if matches!(
            *expected,
            ReferenceMembership::Retired
                | ReferenceMembership::Live(ReferenceState { pending: true, .. })
                | ReferenceMembership::Live(ReferenceState {
                    queued_root: Some(_),
                    ..
                })
                | ReferenceMembership::Live(ReferenceState {
                    construction_root: Some(_),
                    ..
                })
        ) {
            return None;
        }
        let phase = snapshot.phase();
        let preparation = self.begin_prepare(snapshot, phase, None)?;
        self.reference_effect(preparation, slot, expected, ReferenceMembership::Retired)
    }
    pub fn release_retired_reference(&self, slot: &Pooled<ReferenceSlot<R>>) -> bool {
        if !Arc::ptr_eq(&slot.owner, &self.owner)
            || !slot.removal_requested()
            || slot.membership() != Some(ReferenceMembership::Retired)
        {
            return false;
        }
        for root in &slot.roots {
            if !matches!(root.load(), Ok(None)) {
                return false;
            }
        }
        self.references
            .as_ref()
            .and_then(|refs| refs.slots.get(slot.slot))
            .is_some_and(|entry| entry.compare(Some(slot), None))
    }
}

/// A private scan keeps its exact snapshot and referent proposal while a
/// reserved owner root is being prepared. It grants no completed-scan authority.
#[allow(
    clippy::large_enum_variant,
    reason = "bounded inline work avoids an allocation per reference scan"
)]
pub enum ReferenceScan<R: Record = ()> {
    Ready(Preparation<R>),
    NeedsRoot(ReferenceNotification<R>),
}
pub struct ReferenceNotification<R: Record = ()> {
    snapshot: Snapshot<R>,
    phase: Phase,
    slot: Pooled<ReferenceSlot<R>>,
    expected: Pooled<ReferenceMembership>,
    proposed: ReferenceState,
    effect: Option<Effect<R>>,
}
impl<R: Record> ReferenceNotification<R> {
    pub fn expected_revision(&self) -> u64 {
        self.snapshot.phase().revision
    }
    pub fn reference_identity(&self) -> u64 {
        self.slot.identity
    }
    pub fn reference_slot(&self) -> usize {
        self.slot.slot
    }
    pub fn owner(&self) -> ObjectKey {
        self.proposed.owner.expect("notification owner")
    }
    pub fn discover_root(&self) -> Result<Option<Pooled<Root<R>>>, Contended> {
        self.slot.roots[1].load()
    }
    pub fn begin_root_creation(
        &self,
        catalog: &Catalog<R>,
    ) -> Result<RootCreation<R>, RootPrepareError> {
        if !Arc::ptr_eq(&self.slot.owner, &catalog.owner) {
            return Err(RootPrepareError::Foreign);
        }
        let key = self.owner();
        let owner = catalog
            .discover(key.slot)
            .map_err(|_| RootPrepareError::Contended)?
            .filter(|owner| owner.identity == key.identity)
            .ok_or(RootPrepareError::InvalidSlot)?;
        let slot = share(&self.slot).ok_or(RootPrepareError::Contended)?;
        catalog.begin_create_reference_root(slot, ReferenceRootKind::Notification, owner)
    }
    /// Failed owning acquisition retains this plan. A later phase change still
    /// makes its eventual attempt Retry; the expected controller is never refreshed.
    pub fn try_finish(
        &self,
        catalog: &Catalog<R>,
        root: Pooled<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&self.slot.owner, &catalog.owner) {
            return None;
        }
        let snapshot = Snapshot {
            owner: self.snapshot.owner.clone(),
            head: share(&self.snapshot.head)?,
        };
        let effect = match &self.effect {
            Some(effect) => Some(Effect {
                cell: share(&effect.cell)?,
                expected: share(&effect.expected)?,
                proposed: share(&effect.proposed)?,
            }),
            None => None,
        };
        let mut state = self.proposed;
        let held = catalog.notification_root(&self.slot, &mut state, Some(root))?;
        let preparation = catalog.begin_hold_root_after(snapshot, held, self.phase, effect)?;
        catalog.reference_effect(
            preparation,
            share(&self.slot)?,
            share(&self.expected)?,
            ReferenceMembership::Live(state),
        )
    }
}

impl<R: Record> Catalog<R> {
    /// Bind an already published owner from its caller-owned root. Construction
    /// reservations and rebinding are separate operations, not inferred here.
    pub fn begin_bind_reference_owner(
        &self,
        snapshot: Snapshot<R>,
        slot: Pooled<ReferenceSlot<R>>,
        owner: Pooled<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&slot.owner, &self.owner)
            || !Arc::ptr_eq(&owner.owner, &self.owner)
            || owner.status() != RootStatus::Live
            || owner.is_internal()
        {
            return None;
        }
        let expected = slot.state.load()?;
        let ReferenceMembership::Live(mut state) = *expected else {
            return None;
        };
        if state.owner.is_some()
            || state.activated
            || state.pending
            || state.queued_root.is_some()
            || state.construction_root.is_some()
        {
            return None;
        }
        let key = ObjectKey {
            identity: owner.target_identity(),
            slot: owner.target_slot(),
        };
        let Membership::Tracing(marks) = self.reference_target(key)?.membership else {
            return None;
        };
        let phase = snapshot.phase();
        if marks.logically_retired(phase) {
            return None;
        }
        state.owner = Some(key);
        let preparation = self.begin_prepare(snapshot, phase, None)?;
        self.reference_effect(
            preparation,
            slot,
            expected,
            ReferenceMembership::Live(state),
        )
    }

    /// Consume one exact notification by transferring its internal root into a
    /// fresh caller root. Pending state, both root statuses and target work are
    /// certified together; a stopped helper cannot own the only handoff step.
    pub fn begin_take_reference(
        &self,
        snapshot: Snapshot<R>,
        slot: Pooled<ReferenceSlot<R>>,
        caller: Pooled<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&slot.owner, &self.owner) {
            return None;
        }
        let expected = slot.state.load()?;
        let ReferenceMembership::Live(mut state) = *expected else {
            return None;
        };
        let key = state.queued_root?;
        let held = self.discover_root(key.slot).ok()??;
        if held.identity != key.identity {
            return None;
        }
        let preparation = self.begin_transfer_held_root(snapshot, held, caller)?;
        state.queued_root = None;
        self.reference_effect(
            preparation,
            slot,
            expected,
            ReferenceMembership::Live(state),
        )
    }

    fn reference_owner_live(
        &self,
        state: ReferenceState,
        phase: Phase,
        domain: usize,
    ) -> Option<bool> {
        let Some(key) = state.owner else {
            return Some(true);
        };
        let Some(observed) = self.observe_reference_target(key)? else {
            return Some(false);
        };
        let Membership::Tracing(marks) = observed.membership else {
            return Some(false);
        };
        let entry = phase.domains[domain];
        Some(
            !marks.logically_retired(phase)
                && (marks[domain].marked(entry.epoch) || !entry.mode.collects(marks.generation)),
        )
    }

    // Returns only proposed state and counted work. The outer controller couples
    // this with any owner root, slot version and cursor before exposing a phase.
    fn scan_reference_target(
        &self,
        mut state: ReferenceState,
        mut phase: Phase,
        domain: usize,
    ) -> Option<(ReferenceState, Phase, Option<Effect<R>>)> {
        let entry = phase.domains[domain];
        if !state.relevant_stage(entry.floor) || state.born_after[domain] == entry.epoch {
            return Some((state, phase, None));
        }
        let Some(key) = state.target else {
            return Some((state, phase, None));
        };
        let target = self.discover(key.slot).ok()?;
        let target = target.filter(|cell| cell.identity == key.identity);
        let mut effect = None;
        let mut retained = false;
        if let Some(target) = target {
            let expected = target.state.load()?;
            if let Membership::Tracing(marks) = *expected
                && !marks.logically_retired(phase)
            {
                let group = marks[domain].epoch == entry.epoch && marks[domain].phantom_decided;
                if marks.reference_live(entry, domain, state.kind.includes_final()) && !group {
                    return Some((state, phase, None));
                }
                if state.kind.retaining() {
                    let retention = TraceRetention::at_floor(entry.floor)?;
                    let (mut changed, next) = match marks.shade(phase, domain, retention) {
                        Ok(changed) => changed,
                        Err(super::super::TraceError::AlreadyMarked) => (marks, phase),
                        Err(_) => return None,
                    };
                    if state.kind == ReferenceKind::PhantomRetained && entry.floor == 3 {
                        changed.marks[domain].phantom_decided = true;
                    }
                    phase = next;
                    if changed != marks {
                        effect =
                            Some(self.effect(target, expected, Membership::Tracing(changed))?);
                    }
                    retained = true;
                }
            }
        }
        if !retained {
            state.target = None;
        }
        if !state.activated && state.owner.is_some() {
            state.pending = true;
        }
        state.activated = true;
        Some((state, phase, effect))
    }

    /// Compatibility preparation without a supplied notification root. For
    /// resumable root preparation, retain begin_reference_scan's NeedsRoot plan.
    pub fn begin_scan_reference(
        &self,
        snapshot: Snapshot<R>,
        domain: usize,
    ) -> Option<Preparation<R>> {
        match self.begin_reference_scan(snapshot, domain)? {
            ReferenceScan::Ready(preparation) => Some(preparation),
            ReferenceScan::NeedsRoot(_) => None,
        }
    }
    pub fn begin_scan_reference_with_root(
        &self,
        snapshot: Snapshot<R>,
        domain: usize,
        notification: Pooled<Root<R>>,
    ) -> Option<Preparation<R>> {
        match self.begin_reference_scan(snapshot, domain)? {
            ReferenceScan::Ready(preparation) => Some(preparation),
            ReferenceScan::NeedsRoot(plan) => plan.try_finish(self, notification),
        }
    }
    pub fn begin_reference_scan(
        &self,
        snapshot: Snapshot<R>,
        domain: usize,
    ) -> Option<ReferenceScan<R>> {
        let mut phase = snapshot.phase();
        let entry = *phase.domains.get(domain)?;
        if entry.mode == DomainMode::Inactive || !(1..RETIREMENT_FLOOR).contains(&entry.floor) {
            return None;
        }
        let cursor = phase.reference_cursors[domain];
        if cursor >= self.reference_capacity() {
            return None;
        }
        let slot = self.discover_reference(cursor).ok()?;
        let mut reference = None;
        let mut effect = None;
        let mut needs_root = false;
        if let Some(slot) = slot {
            let expected = slot.state.load()?;
            if let ReferenceMembership::Live(state) = *expected
                && state.construction_root.is_some()
            {
                phase.reference_cursors[domain] = cursor.checked_add(1)?;
                return self
                    .scan_reference_reservation(snapshot, slot, expected, state, phase)
                    .map(ReferenceScan::Ready);
            }
            let proposed = match *expected {
                ReferenceMembership::Prepared if slot.removal_requested() => {
                    ReferenceMembership::Retired
                }
                ReferenceMembership::Live(state)
                    if slot.removal_requested()
                        && !state.pending
                        && state.queued_root.is_none() =>
                {
                    ReferenceMembership::Retired
                }
                ReferenceMembership::Live(mut state) => {
                    let owner_live = self.reference_owner_live(state, phase, domain)?;
                    if entry.floor == 4 {
                        if state.pending {
                            state.pending = false;
                            if owner_live && state.owner.is_some() {
                                needs_root = true;
                            }
                        }
                    } else if !state.kind.retaining() || owner_live {
                        (state, phase, effect) =
                            self.scan_reference_target(state, phase, domain)?;
                        if state.pending && state.kind == ReferenceKind::Final {
                            state.pending = false;
                            needs_root = true;
                        }
                    }
                    ReferenceMembership::Live(state)
                }
                other => other,
            };
            if proposed != *expected {
                reference = Some((slot, expected, proposed));
            }
        }
        phase.reference_cursors[domain] = cursor.checked_add(1)?;
        if needs_root {
            let (slot, expected, ReferenceMembership::Live(proposed)) = reference? else {
                return None;
            };
            return Some(ReferenceScan::NeedsRoot(ReferenceNotification {
                snapshot,
                phase,
                slot,
                expected,
                proposed,
                effect,
            }));
        }
        let preparation = self.begin_prepare(snapshot, phase, effect)?;
        let preparation = if let Some((slot, expected, proposed)) = reference {
            self.reference_effect(preparation, slot, expected, proposed)?
        } else {
            preparation
        };
        Some(ReferenceScan::Ready(preparation))
    }
    fn notification_root(
        &self,
        slot: &ReferenceSlot<R>,
        state: &mut ReferenceState,
        root: Option<Pooled<Root<R>>>,
    ) -> Option<Pooled<Root<R>>> {
        let root = root?;
        let owner = state.owner?;
        if !Arc::ptr_eq(&root.owner, &self.owner)
            || !root.matches_reference(slot, ReferenceRootKind::Notification)
            || root.target_identity() != owner.identity
            || root.target_slot() != owner.slot
            || state.queued_root.is_some()
        {
            return None;
        }
        state.queued_root = Some(ReferenceRoot {
            identity: root.identity,
            slot: root.slot(),
        });
        Some(root)
    }
}

impl<R: Record> Catalog<R> {
    /// Protect an eligible referent before entering an owner-construction callback.
    /// The unique prepared root token becomes the reservation's lifetime token:
    /// dropping it requests cancellation, which another participant can complete.
    /// Only one reservation may exist for an exact reference slot at a time.
    pub fn begin_reserve_reference_owner(
        &self,
        snapshot: Snapshot<R>,
        slot: Pooled<ReferenceSlot<R>>,
        root: Pooled<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&slot.owner, &self.owner)
            || !Arc::ptr_eq(&root.owner, &self.owner)
            || !root.matches_reference(&slot, ReferenceRootKind::Construction)
        {
            return None;
        }
        let expected = slot.state.load()?;
        let ReferenceMembership::Live(mut state) = *expected else {
            return None;
        };
        if state.owner.is_some()
            || state.construction_root.is_some()
            || state.activated
            || state.pending
            || state.queued_root.is_some()
        {
            return None;
        }
        let key = ObjectKey {
            identity: root.target_identity(),
            slot: root.target_slot(),
        };
        let phase = snapshot.phase();
        if !state.upgrade_eligible(
            phase,
            Some(ReferenceTarget {
                key,
                membership: root.target_membership()?,
            }),
        ) {
            return None;
        }
        state.construction_root = Some(ReferenceRoot {
            identity: root.identity,
            slot: root.slot(),
        });
        let preparation = self.begin_hold_root_after(snapshot, root, phase, None)?;
        self.reference_effect(
            preparation,
            slot,
            expected,
            ReferenceMembership::Live(state),
        )
    }

    fn reference_construction_root(&self, state: ReferenceState) -> Option<Pooled<Root<R>>> {
        let key = state.construction_root?;
        let root = self.discover_root(key.slot).ok()??;
        (root.identity == key.identity && root.is_internal() && root.status() == RootStatus::Live)
            .then_some(root)
    }

    /// Bind a newly published owner and release construction protection in the
    /// same guarded publication. Cleared slots need no referent reservation.
    pub fn begin_publish_reference_owner(
        &self,
        snapshot: Snapshot<R>,
        slot: Pooled<ReferenceSlot<R>>,
        owner: Pooled<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&slot.owner, &self.owner) || !Arc::ptr_eq(&owner.owner, &self.owner) {
            return None;
        }
        let expected = slot.state.load()?;
        let ReferenceMembership::Live(mut state) = *expected else {
            return None;
        };
        if state.owner.is_some() || state.activated || state.pending || state.queued_root.is_some()
        {
            return None;
        }
        state.owner = Some(ObjectKey {
            identity: owner.target_identity(),
            slot: owner.target_slot(),
        });
        let preparation = if state.construction_root.is_some() {
            let held = self.reference_construction_root(state)?;
            self.begin_publish_root_releasing(snapshot, owner, held)?
        } else {
            if state.target.is_some() {
                return None;
            }
            self.begin_publish_root(snapshot, owner)?
        };
        state.construction_root = None;
        self.reference_effect(
            preparation,
            slot,
            expected,
            ReferenceMembership::Live(state),
        )
    }

    /// Complete a posted construction-cancellation intent outside a collection.
    /// A live lifetime token cannot be revoked by this operation.
    pub fn begin_cancel_reference_owner(
        &self,
        snapshot: Snapshot<R>,
        slot: Pooled<ReferenceSlot<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&slot.owner, &self.owner) {
            return None;
        }
        let expected = slot.state.load()?;
        let ReferenceMembership::Live(state) = *expected else {
            return None;
        };
        let held = self.reference_construction_root(state)?;
        if !held.removal_requested() {
            return None;
        }
        let phase = snapshot.phase();
        self.cancel_reference_reservation(snapshot, slot, expected, state, held, phase)
    }
    fn cancel_reference_reservation(
        &self,
        snapshot: Snapshot<R>,
        slot: Pooled<ReferenceSlot<R>>,
        expected: Pooled<ReferenceMembership>,
        mut state: ReferenceState,
        held: Pooled<Root<R>>,
        phase: Phase,
    ) -> Option<Preparation<R>> {
        let preparation = self.begin_release_held_root(snapshot, held, phase)?;
        state.construction_root = None;
        let proposed = if slot.removal_requested() {
            ReferenceMembership::Retired
        } else {
            ReferenceMembership::Live(state)
        };
        self.reference_effect(preparation, slot, expected, proposed)
    }
    fn scan_reference_reservation(
        &self,
        snapshot: Snapshot<R>,
        slot: Pooled<ReferenceSlot<R>>,
        expected: Pooled<ReferenceMembership>,
        state: ReferenceState,
        phase: Phase,
    ) -> Option<Preparation<R>> {
        let held = self.reference_construction_root(state)?;
        if held.removal_requested() || self.prepared_reference_owner_published(state)? {
            self.cancel_reference_reservation(snapshot, slot, expected, state, held, phase)
        } else {
            // Its internal root participates in ordinary root scans and barriers;
            // no callback completion or participant acknowledgement is required.
            self.begin_prepare(snapshot, phase, None)
        }
    }
}

impl<R: Record> Catalog<R> {
    /// Bind one captured slot before an owner's publication. Repeating this for
    /// every captured slot permits one owner with many conditional fields without
    /// an unbounded effect list in its final publication controller. Eligible
    /// referents remain protected by their independent construction reservations.
    pub fn begin_prepare_reference_owner(
        &self,
        snapshot: Snapshot<R>,
        slot: Pooled<ReferenceSlot<R>>,
        owner: Pooled<Cell<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&slot.owner, &self.owner)
            || !Arc::ptr_eq(&owner.owner, &self.owner)
            || owner.membership()? != Membership::Prepared
        {
            return None;
        }
        let expected = slot.state.load()?;
        let ReferenceMembership::Live(mut state) = *expected else {
            return None;
        };
        if state.owner.is_some()
            || state.activated
            || state.pending
            || state.queued_root.is_some()
            || (state.target.is_some() && state.construction_root.is_none())
        {
            return None;
        }
        if state.construction_root.is_some() {
            self.reference_construction_root(state)?;
        }
        state.owner = Some(ObjectKey {
            identity: owner.identity,
            slot: owner.slot(),
        });
        let phase = snapshot.phase();
        let preparation = self.begin_prepare(snapshot, phase, None)?;
        self.reference_effect(
            preparation,
            slot,
            expected,
            ReferenceMembership::Live(state),
        )
    }
    fn prepared_reference_owner_published(&self, state: ReferenceState) -> Option<bool> {
        let Some(key) = state.owner else {
            return Some(false);
        };
        let observed = self.observe_reference_target(key)?;
        Some(observed.is_some_and(|owner| matches!(owner.membership, Membership::Tracing(_))))
    }
    /// A helper may release a captured slot after observing exact owner publication.
    /// An absent/retired owner alone does not prove the private callback finished;
    /// cancellation still requires the lifetime token's durable drop request.
    pub fn begin_finish_reference_owner(
        &self,
        snapshot: Snapshot<R>,
        slot: Pooled<ReferenceSlot<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&slot.owner, &self.owner) {
            return None;
        }
        let expected = slot.state.load()?;
        let ReferenceMembership::Live(state) = *expected else {
            return None;
        };
        if !self.prepared_reference_owner_published(state)? {
            return None;
        }
        let held = self.reference_construction_root(state)?;
        let phase = snapshot.phase();
        self.cancel_reference_reservation(snapshot, slot, expected, state, held, phase)
    }
}

impl<R: Record> Catalog<R> {
    pub fn discover_reference_root(
        &self,
        reference: &ReferenceSlot<R>,
        kind: ReferenceRootKind,
    ) -> Result<Option<Pooled<Root<R>>>, RootPrepareError> {
        if !Arc::ptr_eq(&reference.owner, &self.owner) {
            return Err(RootPrepareError::Foreign);
        }
        let index = match kind {
            ReferenceRootKind::Construction => 0,
            ReferenceRootKind::Notification => 1,
        };
        reference.roots[index]
            .load()
            .map_err(|_| RootPrepareError::Contended)
    }
}
