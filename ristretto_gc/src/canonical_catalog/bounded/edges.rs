//! Bounded edge creation ownership and logical backing charges.
use super::super::{EdgeCreation, EdgeMembership, EdgePrepareError, EdgeSlot, EdgeToken};
use super::{
    BoundedCatalog, CatalogHandle, Commands, CreationError, Ordering, Participant, Payload, Pooled,
    Progress, RequestError, RequestState,
};
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
/// transfer it into a non-owning `EdgeKey`; otherwise Drop requests removal.
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
impl<R: Payload> Request<R> {
    #[expect(
        clippy::unused_self,
        reason = "private creation is unselected; the shared facade cancellation API returns success"
    )]
    pub(super) fn cancel(&self) -> bool {
        true
    }
    pub(super) fn poll(&mut self, owner: &CatalogHandle<R>, _context: &R::Context) -> Step {
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
        }
    }
}
#[expect(
    private_bounds,
    reason = "sealed command dispatch keeps raw owners inside the bounded participant"
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
    pub fn observe_edge(&mut self, key: EdgeKey) -> Result<EdgeObservation, RequestError> {
        self.idle()?;
        let edge = lookup(&self.lease.owner, key)?;
        observation(&self.lease.owner, &edge)
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

#[cfg(test)]
#[path = "edge_admission_tests.rs"]
mod tests;
