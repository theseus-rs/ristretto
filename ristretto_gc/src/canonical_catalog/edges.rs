//! Exact non-owning edge inventory slots. These transactions do not yet provide
//! complete source traversal or a runtime payload/child-access API.
use super::super::reference::ObjectKey;
use super::{
    AllocationError, Arc, AtomicU8, Catalog, Contended, CreationError, OptionalLink, Ordering,
    Owner, Pool, Pooled, PooledLink, Record, share,
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
