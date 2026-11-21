//! Exact non-owning edge inventory slots. These transactions do not yet provide
//! complete source traversal or a runtime payload/child-access API.
use super::super::reference::ObjectKey;
use super::{
    AllocationError, Arc, AtomicU8, Catalog, Cell, Contended, CreationError, Membership,
    OptionalLink, Ordering, Owner, Phase, Pool, Pooled, PooledLink, Preparation, Record, Root,
    RootStatus, Snapshot, allocate, share,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EdgeState {
    pub source: ObjectKey,
    pub target: Option<ObjectKey>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EdgeMembership {
    Prepared,
    Live(EdgeState),
    Retired,
}
pub struct EdgeSlot {
    owner: Arc<Owner>,
    pub identity: u64,
    pub slot: usize,
    state: PooledLink<EdgeMembership>,
    removal_requested: AtomicU8,
    #[cfg(test)]
    before_effect: std::sync::Mutex<Option<Box<dyn FnOnce() + Send>>>,
}
impl EdgeSlot {
    pub fn membership(&self) -> Option<EdgeMembership> {
        self.state.load().map(|state| *state)
    }
    pub fn removal_requested(&self) -> bool {
        self.removal_requested.load(Ordering::Acquire) != 0
    }
    #[cfg(all(test, not(target_family = "wasm")))]
    pub(crate) fn pause_next_effect(&self, hook: impl FnOnce() + Send + 'static) {
        *self.before_effect.lock().expect("unpoisoned test hook") = Some(Box::new(hook));
    }
}
pub struct EdgeToken {
    slot: Pooled<EdgeSlot>,
    remove_on_drop: bool,
}
impl std::ops::Deref for EdgeToken {
    type Target = EdgeSlot;
    fn deref(&self) -> &Self::Target {
        &self.slot
    }
}
impl EdgeToken {
    /// Raw experiment only; bounded facade ownership must not escape its quota.
    pub fn try_share(&self) -> Option<Pooled<EdgeSlot>> {
        share(&self.slot)
    }

    /// Relinquish the creation owner after publication. The live inventory owns
    /// only source/target identities; a payload may retain an edge key without
    /// creating a strong catalog cycle. This grants no payload-access authority.
    /// The bounded adapter must call this only after its publication certificate.
    pub fn release_to_source(mut self) -> Result<(), Self> {
        if !matches!(self.slot.membership(), Some(EdgeMembership::Live(_))) {
            return Err(self);
        }
        self.remove_on_drop = false;
        Ok(())
    }
}
impl Drop for EdgeToken {
    fn drop(&mut self) {
        if self.remove_on_drop {
            self.slot.removal_requested.store(1, Ordering::Release);
        }
    }
}
pub(super) struct EdgeRegistry {
    pub(super) slots_pool: Arc<Pool<EdgeSlot>>,
    pub(super) states: Arc<Pool<EdgeMembership>>,
    slots: Box<[OptionalLink<EdgeSlot>]>,
    source_degree: Option<std::num::NonZeroUsize>,
}
pub(super) struct EdgeEffect {
    slot: Pooled<EdgeSlot>,
    expected: Pooled<EdgeMembership>,
    pub(super) proposed: Pooled<EdgeMembership>,
}
impl EdgeEffect {
    pub(super) fn apply(&self, proposed: Pooled<EdgeMembership>) {
        #[cfg(test)]
        {
            let hook = self
                .slot
                .before_effect
                .lock()
                .expect("unpoisoned test hook")
                .take();
            if let Some(hook) = hook {
                hook();
            }
        }
        self.slot.state.compare(&self.expected, proposed);
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EdgePrepareError {
    InvalidSlot,
    Occupied,
    Contended,
    IdentityExhausted,
}
pub struct EdgeCreation {
    owner: Arc<Owner>,
    identity: u64,
    slot: usize,
    membership: Option<Pooled<EdgeMembership>>,
    private: Option<EdgeSlot>,
    prepared: Option<Pooled<EdgeSlot>>,
    finished: bool,
}
impl EdgeCreation {
    /// One selected pool probe or announcement attempt; no phase effect yet.
    #[expect(
        clippy::indexing_slicing,
        reason = "edge creation retains its setup-validated fixed slot"
    )]
    pub fn poll<R: Record>(
        &mut self,
        catalog: &Catalog<R>,
    ) -> Result<Option<EdgeToken>, CreationError> {
        if !Arc::ptr_eq(&self.owner, &catalog.owner) {
            return Err(CreationError::Foreign);
        }
        if self.finished {
            return Err(CreationError::Finished);
        }
        let edges = catalog.edges.as_ref().expect("same catalog edge backing");
        if self.membership.is_none() && self.private.is_none() && self.prepared.is_none() {
            match edges.states.try_allocate_status(EdgeMembership::Prepared) {
                Ok(state) => self.membership = Some(state),
                Err(failure) if failure.error == AllocationError::GenerationExhausted => {
                    return Err(CreationError::GenerationExhausted);
                }
                Err(_) => {}
            }
            return Ok(None);
        }
        if self.prepared.is_none() {
            let private = self.private.take().unwrap_or_else(|| EdgeSlot {
                owner: self.owner.clone(),
                identity: self.identity,
                slot: self.slot,
                state: PooledLink::new(
                    edges.states.clone(),
                    self.membership.take().expect("private state"),
                ),
                removal_requested: AtomicU8::new(0),
                #[cfg(test)]
                before_effect: std::sync::Mutex::new(None),
            });
            match edges.slots_pool.try_allocate_status(private) {
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
        let slot = self.prepared.as_ref().expect("prepared edge slot");
        let Some(proposed) = share(slot) else {
            return Ok(None);
        };
        self.finished = true;
        if edges.slots[self.slot].compare(None, Some(proposed)) {
            Ok(Some(EdgeToken {
                slot: self.prepared.take().expect("announced slot"),
                remove_on_drop: true,
            }))
        } else {
            self.prepared = None;
            Err(CreationError::Occupied)
        }
    }
}
impl<R: Record> Catalog<R> {
    /// Setup-only inventory reservation. All inventories are absent by default.
    pub fn with_edge_capacities(
        mut self,
        slots: usize,
        backing: usize,
        states: usize,
        limit: u64,
    ) -> Self {
        assert_eq!(
            Arc::strong_count(&self.owner),
            1,
            "setup owner already shared"
        );
        assert!(self.edges.is_none(), "edge backing already configured");
        assert!(
            slots == 0 || (backing > 0 && states > 0),
            "edge slots require backing"
        );
        if slots != 0 {
            let slots_pool = Pool::with_generation_limit(backing, limit);
            self.edges = Some(EdgeRegistry {
                slots: (0..slots)
                    .map(|_| OptionalLink::empty(slots_pool.clone()))
                    .collect(),
                slots_pool,
                source_degree: None,
                states: Pool::with_generation_limit(states, limit),
            });
        }
        self
    }
    /// Setup-only geometry. Publication enforces placement before traversal can
    /// trust this range; no source or participant can later change the degree.
    pub fn with_source_edge_degree(mut self, degree: usize) -> Self {
        assert_eq!(
            Arc::strong_count(&self.owner),
            1,
            "setup owner already shared"
        );
        let degree = std::num::NonZeroUsize::new(degree).expect("nonzero source degree");
        let edges = self
            .edges
            .as_mut()
            .expect("source degree requires edge backing");
        assert!(
            edges.source_degree.is_none(),
            "source degree already configured"
        );
        assert_eq!(
            self.slots.len().checked_mul(degree.get()),
            Some(edges.slots.len()),
            "source degree does not cover edge inventory"
        );
        edges.source_degree = Some(degree);
        self
    }
    pub(crate) fn source_edge_range(&self, source: usize) -> Option<std::ops::Range<usize>> {
        let edges = self.edges.as_ref()?;
        match edges.source_degree {
            None => Some(0..edges.slots.len()),
            Some(degree) => {
                if source >= self.slots.len() {
                    return None;
                }
                let start = source.checked_mul(degree.get())?;
                Some(start..start.checked_add(degree.get())?)
            }
        }
    }
    pub(crate) fn edge_slot_matches_source(&self, edge: usize, source: usize) -> bool {
        self.source_edge_range(source)
            .is_some_and(|range| range.contains(&edge))
    }
    pub fn edge_backing_bytes(slots: usize, backing: usize, states: usize) -> Option<usize> {
        if slots == 0 {
            return Some(0);
        }
        std::alloc::Layout::array::<OptionalLink<EdgeSlot>>(slots)
            .ok()?
            .size()
            .checked_add(Pool::<EdgeSlot>::requested_backing_bytes(backing)?)?
            .checked_add(Pool::<EdgeMembership>::requested_backing_bytes(states)?)
    }
    pub fn edge_capacity(&self) -> usize {
        self.edges.as_ref().map_or(0, |edges| edges.slots.len())
    }
    pub fn discover_edge(&self, slot: usize) -> Result<Option<Pooled<EdgeSlot>>, Contended> {
        self.edges
            .as_ref()
            .and_then(|edges| edges.slots.get(slot))
            .map_or(Ok(None), OptionalLink::load)
    }
    pub fn begin_create_edge(&self, slot: usize) -> Result<EdgeCreation, EdgePrepareError> {
        let entry = self
            .edges
            .as_ref()
            .and_then(|edges| edges.slots.get(slot))
            .ok_or(EdgePrepareError::InvalidSlot)?;
        if entry
            .load()
            .map_err(|_| EdgePrepareError::Contended)?
            .is_some()
        {
            return Err(EdgePrepareError::Occupied);
        }
        let identity = self.next_identity.load(Ordering::Relaxed);
        let next = identity
            .checked_add(1)
            .ok_or(EdgePrepareError::IdentityExhausted)?;
        self.next_identity
            .compare_exchange(identity, next, Ordering::Relaxed, Ordering::Relaxed)
            .map_err(|_| EdgePrepareError::Contended)?;
        Ok(EdgeCreation {
            owner: self.owner.clone(),
            identity,
            slot,
            membership: None,
            private: None,
            prepared: None,
            finished: false,
        })
    }
}

#[cfg(test)]
#[path = "edge_backing_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "edge_control_tests.rs"]
mod control_tests;

impl<R: Record> Catalog<R> {
    fn edge_effect(
        &self,
        mut preparation: Preparation<R>,
        slot: Pooled<EdgeSlot>,
        expected: Pooled<EdgeMembership>,
        proposed: EdgeMembership,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&slot.owner, &self.owner) {
            return None;
        }
        preparation.edge = Some(EdgeEffect {
            slot,
            expected,
            proposed: allocate(&self.edges.as_ref()?.states, proposed)?,
        });
        Some(preparation)
    }
    fn edge_source(&self, root: &Root<R>, phase: Phase) -> Option<ObjectKey> {
        if !Arc::ptr_eq(&root.owner, &self.owner)
            || root.status() != RootStatus::Live
            || root.is_internal()
        {
            return None;
        }
        let Membership::Tracing(marks) = root.target_membership()? else {
            return None;
        };
        if marks.logically_retired(phase) {
            return None;
        }
        Some(ObjectKey {
            identity: root.target_identity(),
            slot: root.target_slot(),
        })
    }
    // Outer None is failed observation, inner None is absent or a different identity.
    #[expect(
        clippy::option_option,
        reason = "outer absence is contention; inner absence is a missing or replaced identity"
    )]
    fn edge_target(&self, key: ObjectKey) -> Option<Option<Pooled<Cell<R>>>> {
        Some(
            self.discover(key.slot)
                .ok()?
                .filter(|cell| cell.identity == key.identity),
        )
    }
    #[expect(
        clippy::needless_pass_by_value,
        reason = "preserve the frozen owning-preparation API and exact owner destruction boundary"
    )]
    pub fn begin_publish_edge(
        &self,
        snapshot: Snapshot<R>,
        edge: Pooled<EdgeSlot>,
        source: Pooled<Root<R>>,
        target: Option<Pooled<Root<R>>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&edge.owner, &self.owner) {
            return None;
        }
        let expected = edge.state.load()?;
        if *expected != EdgeMembership::Prepared {
            return None;
        }
        let source = self.edge_source(&source, snapshot.phase())?;
        if !self.edge_slot_matches_source(edge.slot, source.slot) {
            return None;
        }
        let target = target
            .as_ref()
            .map(|root| self.edge_source(root, snapshot.phase()))
            .transpose_option()?;
        let (effect, phase) = if let Some(key) = target {
            self.root_barrier(self.edge_target(key)??, snapshot.phase(), false)?
        } else {
            (None, snapshot.phase())
        };
        let preparation = self.begin_prepare(snapshot, phase, effect)?;
        self.edge_effect(
            preparation,
            edge,
            expected,
            EdgeMembership::Live(EdgeState { source, target }),
        )
    }
    #[expect(
        clippy::needless_pass_by_value,
        reason = "preserve the frozen owning-preparation API and exact owner destruction boundary"
    )]
    pub fn begin_replace_edge(
        &self,
        snapshot: Snapshot<R>,
        edge: Pooled<EdgeSlot>,
        source: Pooled<Root<R>>,
        target: Option<Pooled<Root<R>>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&edge.owner, &self.owner) {
            return None;
        }
        let expected = edge.state.load()?;
        let EdgeMembership::Live(mut state) = *expected else {
            return None;
        };
        if self.edge_source(&source, snapshot.phase())? != state.source {
            return None;
        }
        let target = target
            .as_ref()
            .map(|root| self.edge_source(root, snapshot.phase()))
            .transpose_option()?;
        let mut phase = snapshot.phase();
        let mut effect = None;
        // A same-target write needs one insertion barrier, never two proposals
        // from the same expected membership with duplicate counted work.
        if state.target != target
            && let Some(key) = state.target
            && let Some(cell) = self.edge_target(key)?
        {
            (effect, phase) = self.root_barrier(cell, phase, true)?;
        }
        let mut secondary = None;
        if let Some(key) = target {
            (secondary, phase) = self.root_barrier(self.edge_target(key)??, phase, false)?;
        }
        if effect.is_none() {
            effect = secondary.take();
        }
        let mut preparation = self.begin_prepare(snapshot, phase, effect)?;
        preparation.secondary_effect = secondary;
        state.target = target;
        self.edge_effect(preparation, edge, expected, EdgeMembership::Live(state))
    }
    #[expect(
        clippy::needless_pass_by_value,
        reason = "preserve the frozen owning-preparation API and exact owner destruction boundary"
    )]
    pub fn begin_root_edge_target(
        &self,
        snapshot: Snapshot<R>,
        edge: Pooled<EdgeSlot>,
        source: Pooled<Root<R>>,
        child: Pooled<Root<R>>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&edge.owner, &self.owner) {
            return None;
        }
        let expected = edge.state.load()?;
        let EdgeMembership::Live(state) = *expected else {
            return None;
        };
        if self.edge_source(&source, snapshot.phase())? != state.source
            || state.target
                != Some(ObjectKey {
                    identity: child.target_identity(),
                    slot: child.target_slot(),
                })
        {
            return None;
        }
        // Every edge change also changes this exact controller snapshot. No edge
        // mutation or discovery replacement can slip between validation and CAS.
        self.begin_activate_root(snapshot, child)
    }
    pub fn begin_requested_edge_removal(
        &self,
        snapshot: Snapshot<R>,
        edge: Pooled<EdgeSlot>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&edge.owner, &self.owner) || !edge.removal_requested() {
            return None;
        }
        let expected = edge.state.load()?;
        self.prepare_edge_removal(snapshot, edge, expected)
    }

    /// A published source owns its inventory logically, never through a strong
    /// reference cycle. Reclaim it only after the exact source is physically
    /// retired or its discovery identity has been removed/reused. A failed load
    /// cannot be interpreted as an absent source.
    pub fn begin_orphan_edge_removal(
        &self,
        snapshot: Snapshot<R>,
        edge: Pooled<EdgeSlot>,
    ) -> Option<Preparation<R>> {
        if !Arc::ptr_eq(&edge.owner, &self.owner) {
            return None;
        }
        let expected = edge.state.load()?;
        let EdgeMembership::Live(state) = *expected else {
            return None;
        };
        if let Some(source) = self.edge_target(state.source)?
            && source.membership()? != Membership::Retired
        {
            return None;
        }
        self.prepare_edge_removal(snapshot, edge, expected)
    }

    fn prepare_edge_removal(
        &self,
        snapshot: Snapshot<R>,
        edge: Pooled<EdgeSlot>,
        expected: Pooled<EdgeMembership>,
    ) -> Option<Preparation<R>> {
        let mut phase = snapshot.phase();
        let mut effect = None;
        match *expected {
            EdgeMembership::Retired => return None,
            EdgeMembership::Prepared => {}
            EdgeMembership::Live(state) => {
                if let Some(key) = state.target
                    && let Some(cell) = self.edge_target(key)?
                {
                    (effect, phase) = self.root_barrier(cell, phase, true)?;
                }
            }
        }
        let preparation = self.begin_prepare(snapshot, phase, effect)?;
        self.edge_effect(preparation, edge, expected, EdgeMembership::Retired)
    }
    pub fn release_retired_edge(&self, edge: &Pooled<EdgeSlot>) -> bool {
        if !Arc::ptr_eq(&edge.owner, &self.owner)
            || edge.membership() != Some(EdgeMembership::Retired)
        {
            return false;
        }
        self.edges
            .as_ref()
            .and_then(|edges| edges.slots.get(edge.slot))
            .is_some_and(|entry| entry.compare(Some(edge), None))
    }
}

// Preserve the distinction between a null target and failed root validation.
#[expect(
    clippy::option_option,
    reason = "preserve null target versus failed source/target validation"
)]
trait TransposeOption<T> {
    fn transpose_option(self) -> Option<Option<T>>;
}
impl<T> TransposeOption<T> for Option<Option<T>> {
    fn transpose_option(self) -> Option<Option<T>> {
        match self {
            None => Some(None),
            Some(Some(value)) => Some(Some(value)),
            Some(None) => None,
        }
    }
}

#[cfg(test)]
#[path = "edge_transaction_tests.rs"]
pub(crate) mod transaction_tests;
