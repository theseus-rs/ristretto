#![expect(
    clippy::panic_in_result_fn,
    reason = "tests propagate setup errors and assert admission"
)]
use super::*;
use crate::{Configuration, HeapOwner};
use std::sync::Arc;

fn heap(limit: usize) -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .threads(1)
            .max_heap_bytes(limit)
            .build(),
    )
}

struct Uncertified;
impl Trace for Uncertified {
    fn trace(&self, _: &GarbageCollector) {}

    fn may_contain_gc_references() -> bool {
        false
    }
}

#[test]
fn safe_trace_hint_does_not_grant_canonical_authority() -> crate::Result<()> {
    let collector = heap(1024);
    assert!(matches!(
        PreparedPayload::new(&collector, Uncertified),
        Err(PrepareError::Unsupported)
    ));
    assert_eq!(collector.managed_bytes(), 0);
    let ordinary = Gc::try_new(&collector, Uncertified)?;
    drop(ordinary);
    Ok(())
}

#[test]
fn scalar_proof_preserves_exact_type_and_heap_identity() -> crate::Result<()> {
    let collector = heap(size_of::<u64>());
    let foreign = heap(size_of::<u64>());
    let payload = PreparedPayload::new(&collector, 19_u64).map_err(|_| Error::OutOfMemory)?;
    assert_eq!(payload.borrow::<u64>(), Some(&19));
    assert!(payload.borrow::<i64>().is_none());
    assert!(payload.belongs_to(&collector));
    assert!(!payload.belongs_to(&foreign));
    assert_eq!(collector.managed_bytes(), size_of::<u64>());
    drop(payload);
    assert_eq!(collector.managed_bytes(), 0);
    Ok(())
}

struct Backing(Box<[u8]>);
impl Trace for Backing {
    fn trace(&self, _: &GarbageCollector) {}

    fn may_contain_gc_references() -> bool {
        false
    }

    fn canonical_payload() -> Option<CanonicalPayload<Self>> {
        // SAFETY: immutable byte storage has no GC edges or required callbacks.
        Some(unsafe { CanonicalPayload::new() })
    }

    fn external_size(&self) -> usize {
        self.0.len()
    }
}

#[test]
fn backing_is_reserved_before_payload_allocation_and_refunded_on_drop() -> crate::Result<()> {
    let bytes = size_of::<Backing>() + 17;
    let collector = heap(bytes);
    let payload = PreparedPayload::new(&collector, Backing(vec![3; 17].into_boxed_slice()))
        .map_err(|_| Error::OutOfMemory)?;
    assert_eq!(collector.managed_bytes(), bytes);
    assert!(matches!(
        PreparedPayload::new(&collector, 1_u8),
        Err(PrepareError::Heap(Error::OutOfMemory))
    ));
    drop(payload);
    assert_eq!(collector.managed_bytes(), 0);
    Ok(())
}

struct Owner;
impl HeapOwner for Owner {
    fn release_heap_roots(&self) {}
}
impl Trace for Owner {
    fn trace(&self, _: &GarbageCollector) {}

    fn may_contain_gc_references() -> bool {
        false
    }

    fn canonical_payload() -> Option<CanonicalPayload<Self>> {
        // SAFETY: this empty payload has no edges. Its reported owner is still
        // rejected independently of the type certificate.
        Some(unsafe { CanonicalPayload::new() })
    }

    fn heap_owner(&self) -> Option<Arc<dyn HeapOwner>> {
        Some(Arc::new(Self))
    }
}

#[test]
fn reported_owner_is_rejected_even_for_an_empty_certified_type() {
    let collector = heap(1024);
    assert!(matches!(
        PreparedPayload::new(&collector, Owner),
        Err(PrepareError::Unsupported)
    ));
    assert_eq!(collector.managed_bytes(), 0);
}

#[test]
fn typed_view_uses_owning_storage_and_exact_type_id() -> crate::Result<()> {
    let budget = Arc::new(crate::allocation_budget::AllocationBudget::new(size_of::<
        u64,
    >()));
    let (allocation, _) = Allocation::new(43_u64, budget.reserve(size_of::<u64>())?)?;
    let payload = PreparedPayload {
        allocation,
        collector: Weak::new(),
        type_id: TypeId::of::<u64>(),
    };
    assert_eq!(payload.borrow::<u64>(), Some(&43));
    assert!(payload.borrow::<i64>().is_none());
    drop(payload);
    assert_eq!(budget.used(), 0);
    Ok(())
}
