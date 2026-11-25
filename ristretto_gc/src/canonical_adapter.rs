//! Private typed adapter for the optional canonical catalog. No ordinary GC
//! registration enters this backend. A view borrows its exact record and heap.
use crate::GarbageCollector;
use crate::canonical_catalog as gc_catalog;
use crate::canonical_payload::PreparedPayload;
use gc_catalog::bounded::RootHandle;
use gc_catalog::{Entered, Record, RecordError, UNPUBLISHED};
use std::sync::atomic::{AtomicU8, Ordering};

#[path = "canonical_leaf_allocation.rs"]
mod allocation;
#[path = "canonical_api.rs"]
pub mod api;
#[path = "canonical_graph.rs"]
mod graph;
#[path = "canonical_leaf_maintenance.rs"]
mod maintenance;
#[path = "canonical_root_owner.rs"]
mod root_owner;
#[path = "canonical_leaf_runtime.rs"]
mod runtime;

struct LeafRecord {
    payload: PreparedPayload,
    published: AtomicU8,
    graph_edges: u8,
}

struct LeafGuard<'a> {
    _record: &'a LeafRecord,
    _collector: &'a GarbageCollector,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LeafAccessError {
    NotLive,
    NotPublished,
    WrongType,
    Admission(RecordError),
}

struct LeafView<'a, T> {
    _root: &'a RootHandle<LeafRecord>,
    _admission: Entered<'a, LeafGuard<'a>>,
    value: &'a T,
}

impl<T> std::ops::Deref for LeafView<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value
    }
}

impl LeafRecord {
    fn new(payload: PreparedPayload) -> Self {
        Self {
            payload,
            published: AtomicU8::new(UNPUBLISHED),
            graph_edges: 0,
        }
    }

    fn address(&self) -> usize {
        self.payload.address()
    }

    fn borrow_root<'a, T: 'static>(
        root: &'a RootHandle<Self>,
        collector: &'a GarbageCollector,
    ) -> Result<LeafView<'a, T>, LeafAccessError> {
        let record = root.borrow_live_record().ok_or(LeafAccessError::NotLive)?;
        let admission = record
            .enter(collector)
            .map_err(LeafAccessError::Admission)?;
        if record.published.load(Ordering::Acquire) == UNPUBLISHED {
            return Err(LeafAccessError::NotPublished);
        }
        let value = record.payload.borrow().ok_or(LeafAccessError::WrongType)?;
        Ok(LeafView {
            _root: root,
            _admission: admission,
            value,
        })
    }
}

impl Record for LeafRecord {
    type Context = GarbageCollector;
    type Guard<'a> = LeafGuard<'a>;

    fn enter<'a>(
        &'a self,
        collector: &'a GarbageCollector,
    ) -> Result<Entered<'a, LeafGuard<'a>>, RecordError> {
        if !self.payload.belongs_to(collector) {
            return Err(RecordError::Foreign);
        }
        // The sealed record owns certified payload storage exclusively. Borrowing
        // the record excludes destruction; no legacy descriptor/storage view exists.
        Ok(Entered::new(
            LeafGuard {
                _record: self,
                _collector: collector,
            },
            &self.published,
        ))
    }
}

#[cfg(test)]
#[path = "canonical_adapter_tests.rs"]
mod tests;
