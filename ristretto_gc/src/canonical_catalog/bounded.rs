//! Bounded participant admission and logical backing charges.
//! Request storage is introduced separately; no owning catalog state escapes.
#![expect(
    clippy::expect_used,
    reason = "admitted capacities and an unconsumed private handle remain valid until Drop"
)]
use super::{Arc, AtomicU64, AtomicUsize, Catalog, Ordering, Record as Payload};
use crate::canonical_pool as pool;
use std::ops::Deref;
mod budget;
mod quota;
use budget::Charge;
pub use budget::{Budget, Footprint};
const MAX_POOL: usize = 1 << 24;
static NEXT_CATALOG: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigError {
    EmptyRegistry,
    NoHelpers,
    CapacityOverflow,
    IdentityExhausted,
    BudgetExceeded,
    BudgetContended,
    InvalidGenerationLimit,
    InvalidEdgeLayout,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Capacities {
    registry: usize,
    mutators: usize,
    helpers: usize,
    controls: usize,
    memberships: usize,
    cells: usize,
    generation_limit: u64,
}
impl Capacities {
    pub fn derive(registry: usize, mutators: usize, helpers: usize) -> Result<Self, ConfigError> {
        fn rounded(value: usize) -> Result<usize, ConfigError> {
            value
                .checked_next_power_of_two()
                .filter(|v| *v <= MAX_POOL)
                .ok_or(ConfigError::CapacityOverflow)
        }
        if registry == 0 {
            return Err(ConfigError::EmptyRegistry);
        }
        if helpers == 0 {
            return Err(ConfigError::NoHelpers);
        }
        let overflow = ConfigError::CapacityOverflow;
        let participants = mutators.checked_add(helpers).ok_or(overflow)?;
        let reserve = participants
            .checked_mul(8)
            .and_then(|v| v.checked_add(8))
            .ok_or(overflow)?;

        Ok(Self {
            registry,
            mutators,
            helpers,
            controls: rounded(reserve)?,
            memberships: rounded(registry.checked_add(reserve).ok_or(overflow)?)?,
            cells: rounded(
                participants
                    .checked_mul(2)
                    .and_then(|v| v.checked_add(2))
                    .and_then(|v| v.checked_add(registry))
                    .ok_or(overflow)?,
            )?,
            generation_limit: pool::MAX_GENERATION,
        })
    }
}
impl Capacities {
    pub fn registry(self) -> usize {
        self.registry
    }
    pub fn mutators(self) -> usize {
        self.mutators
    }
    pub fn helpers(self) -> usize {
        self.helpers
    }
    pub fn participants(self) -> usize {
        self.mutators + self.helpers
    }
    pub fn pools(self) -> (usize, usize, usize) {
        (self.controls, self.memberships, self.cells)
    }
    pub fn generation_limit(self) -> u64 {
        self.generation_limit
    }
    pub fn with_generation_limit(mut self, limit: u64) -> Result<Self, ConfigError> {
        if !(1..=pool::MAX_GENERATION).contains(&limit) {
            return Err(ConfigError::InvalidGenerationLimit);
        }
        self.generation_limit = limit;
        Ok(self)
    }
}

pub struct BoundedCatalog<R: Payload = ()> {
    raw: Catalog<R>,
    capacities: Capacities,
    identity: u64,
    mutators: AtomicUsize,
    helpers: AtomicUsize,
    // Last: catalog backing is destroyed before its charge is refunded.
    charge: Charge,
}
impl BoundedCatalog {
    #[expect(
        clippy::new_ret_no_self,
        reason = "opaque handles release their Arc before backing and charge"
    )]
    pub fn new(capacities: Capacities) -> Result<CatalogHandle, ConfigError> {
        Self::new_in(capacities, &Budget::new(usize::MAX))
    }
    pub fn new_in(
        capacities: Capacities,
        budget: &Arc<Budget>,
    ) -> Result<CatalogHandle, ConfigError> {
        Self::with_records_in(capacities, budget)
    }
}
impl<R: Payload> BoundedCatalog<R> {
    pub fn with_records_in(
        capacities: Capacities,
        budget: &Arc<Budget>,
    ) -> Result<CatalogHandle<R>, ConfigError> {
        Self::with_records_and_storage_in(capacities, budget, 0)
    }
    pub fn with_records_and_storage_in(
        capacities: Capacities,
        budget: &Arc<Budget>,
        inline_storage: usize,
    ) -> Result<CatalogHandle<R>, ConfigError> {
        let footprint = Footprint::for_records::<R>(capacities)?;
        let total = footprint
            .total()
            .checked_add(inline_storage)
            .ok_or(ConfigError::CapacityOverflow)?;
        let charge = budget.reserve(total)?;
        let identity = NEXT_CATALOG
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
                value.checked_add(1)
            })
            .map_err(|_| ConfigError::IdentityExhausted)?;
        let raw = Catalog::with_record_capacities_and_limit(
            capacities.registry,
            capacities.controls,
            capacities.memberships,
            capacities.cells,
            capacities.generation_limit,
        );
        Ok(CatalogHandle {
            inner: Some(Arc::new(Self {
                raw,
                capacities,
                identity,
                mutators: AtomicUsize::new(0),
                helpers: AtomicUsize::new(0),
                charge,
            })),
        })
    }
    pub fn footprint(&self) -> Footprint {
        Footprint::for_records::<R>(self.capacities).expect("validated capacities")
    }
    pub fn charged_bytes(&self) -> usize {
        self.charge.bytes()
    }
    pub fn reserved_storage_bytes(&self) -> usize {
        self.charge.bytes() - self.footprint().total()
    }
    pub fn capacities(&self) -> Capacities {
        self.capacities
    }
    pub fn registered(&self) -> (usize, usize) {
        (
            self.mutators.load(Ordering::Acquire),
            self.helpers.load(Ordering::Acquire),
        )
    }
    fn counter(&self, role: Role) -> (&AtomicUsize, usize) {
        match role {
            Role::Mutator => (&self.mutators, self.capacities.mutators),
            Role::Helper => (&self.helpers, self.capacities.helpers),
        }
    }
}

/// Final release frees the outer `Arc` before catalog backing, then refunds charge.
pub struct CatalogHandle<R: Payload = ()> {
    inner: Option<Arc<BoundedCatalog<R>>>,
}
impl<R: Payload> Clone for CatalogHandle<R> {
    fn clone(&self) -> Self {
        Self {
            inner: Some(self.inner.as_ref().expect("live handle").clone()),
        }
    }
}
impl<R: Payload> Deref for CatalogHandle<R> {
    type Target = BoundedCatalog<R>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().expect("live handle")
    }
}
impl<R: Payload> Drop for CatalogHandle<R> {
    fn drop(&mut self) {
        if let Some(catalog) = Arc::into_inner(self.inner.take().expect("live handle")) {
            drop(catalog);
        }
    }
}
impl CatalogHandle {
    pub fn register_mutator(&self) -> Result<Participant, RegisterError> {
        self.register(Role::Mutator)
    }
    pub fn register_helper(&self) -> Result<Participant, RegisterError> {
        self.register(Role::Helper)
    }
}
impl<R: Payload> CatalogHandle<R> {
    fn register(&self, role: Role) -> Result<Participant<R>, RegisterError> {
        let owner = self.clone();
        let (count, limit) = self.counter(role);
        quota::acquire(count, limit)?;
        Ok(Participant {
            lease: Lease { owner, role },
        })
    }
}
#[derive(Clone, Copy)]
enum Role {
    Mutator,
    Helper,
}
struct Lease<R: Payload> {
    owner: CatalogHandle<R>,
    role: Role,
}
impl<R: Payload> Drop for Lease<R> {
    fn drop(&mut self) {
        quota::release(self.owner.counter(self.role).0);
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegisterError {
    Limit,
    Contended,
}
/// This unique quota will retain request storage before the lease when added.
pub struct Participant<R: Payload = ()> {
    lease: Lease<R>,
}

#[cfg(test)]
mod admission_tests;
#[cfg(all(test, not(target_family = "wasm")))]
mod quota_models;
