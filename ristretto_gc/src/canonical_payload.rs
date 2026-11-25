//! Opt-in type authority for reference-free payloads; independent of ordinary Trace.
#[cfg(test)]
use crate::Gc;
use crate::private_allocation::Allocation;
use crate::{Error, GarbageCollector, Trace};
use std::any::TypeId;
use std::marker::PhantomData;
use std::sync::Weak;

/// An audited, type-invariant certificate for catalog-owned graph payloads.
///
/// Implementing [`Trace`] alone leaves a type uncertified. Returning
/// `false` from a tracing hint alone does not grant this authority.
///
/// ```compile_fail
/// use ristretto_gc::CanonicalPayload;
/// fn change_type(proof: CanonicalPayload<u64>) -> CanonicalPayload<String> { proof }
/// ```
#[derive(Debug)]
pub struct CanonicalPayload<T>(PhantomData<fn(T) -> T>);

impl<T> CanonicalPayload<T> {
    /// Inline bytes and separately owned backing reserved by the canonical adapter.
    ///
    /// # Errors
    /// Returns an allocation error when the total cannot be represented.
    pub fn allocation_size(value: &T) -> crate::Result<usize>
    where
        T: Trace,
    {
        size_of::<T>()
            .checked_add(value.external_size())
            .ok_or(Error::OutOfMemory)
    }

    /// Certifies every value of exactly T, including values after interior mutation.
    ///
    /// # Safety
    /// T must contain no managed strong, weak, conditional, or owning heap edges.
    /// Its lifetime must not depend on tracing, attachment, relocation, or Java
    /// finalization callbacks. All graph edges must live in the canonical catalog.
    /// Ordinary Rust Drop remains supported and is outside bounded poll-work claims.
    /// Owned external backing must be reported by [`Trace::external_size`] and
    /// must not grow without a separately reserved charge.
    #[must_use]
    pub const unsafe fn new() -> Self {
        Self(PhantomData)
    }
}

#[derive(Debug)]
pub(crate) enum PrepareError {
    Unsupported,
    Heap(Error),
}

/// The sole private payload owner. This type has no legacy registration/finalizer
/// constructor and cannot be built by extracting an ordinary descriptor.
pub(crate) struct PreparedPayload {
    allocation: Allocation,
    collector: Weak<GarbageCollector>,
    type_id: TypeId,
}

impl PreparedPayload {
    pub(crate) fn new<T: Trace + Send + Sync + 'static>(
        collector: &GarbageCollector,
        value: T,
    ) -> Result<Self, PrepareError> {
        if T::canonical_payload().is_none()
            || T::may_contain_gc_references()
            || value.heap_owner().is_some()
        {
            return Err(PrepareError::Unsupported);
        }
        let bytes = CanonicalPayload::allocation_size(&value).map_err(PrepareError::Heap)?;
        let reservation = collector
            .reserve_private_bytes(bytes)
            .map_err(PrepareError::Heap)?;
        let (allocation, _) = Allocation::new(value, reservation).map_err(PrepareError::Heap)?;
        Ok(Self {
            allocation,
            collector: collector.weak(),
            type_id: TypeId::of::<T>(),
        })
    }

    pub(crate) fn belongs_to(&self, collector: &GarbageCollector) -> bool {
        std::ptr::eq(self.collector.as_ptr(), collector)
    }

    pub(crate) fn address(&self) -> usize {
        self.allocation.pointer().get() as usize
    }

    pub(crate) fn borrow<T: 'static>(&self) -> Option<&T> {
        if self.type_id != TypeId::of::<T>() {
            return None;
        }
        // SAFETY: immutable TypeId was paired with initialized T at construction.
        // The owning allocation is borrowed throughout the returned reference.
        Some(unsafe { &*self.allocation.pointer().get().cast::<T>() })
    }
}

#[cfg(test)]
#[path = "canonical_payload_tests.rs"]
mod tests;
