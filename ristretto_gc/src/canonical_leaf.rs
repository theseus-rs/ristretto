//! Concrete private leaf ownership for the canonical catalog experiment.
//! No public Gc allocation or legacy registration selects this adapter.
use super::{Owner, Publication};
use crate::canonical_catalog as gc_catalog;
use crate::metadata::MetadataRef;
use crate::{GarbageCollector, ObjectId};
use gc_catalog::bounded::RootHandle;
use gc_catalog::{Entered, Record, RecordError};
use std::sync::Weak;

#[path = "canonical_diagnostics/canonical_root_owner.rs"]
mod root_owner;

#[path = "canonical_diagnostics/canonical_leaf_allocation.rs"]
pub(super) mod allocation;

#[path = "canonical_diagnostics/canonical_leaf_maintenance.rs"]
pub(super) mod maintenance;

#[path = "canonical_diagnostics/canonical_leaf_runtime.rs"]
pub(super) mod runtime;

#[path = "canonical_diagnostics/canonical_graph.rs"]
pub(super) mod graph;

#[path = "canonical_graph_benchmark.rs"]
pub(super) mod graph_benchmark;
use std::sync::atomic::{AtomicU8, Ordering};

/// Unique payload ownership moves into one catalog cell before announcement.
/// Inert copied identities and publication diagnostics grant no payload access.
pub(super) struct LeafRecord {
    metadata: MetadataRef,
    published: AtomicU8,
    // Initialized while private. Child roots recover the same layout from their record.
    graph_edges: u8,
    #[cfg(test)]
    after_admission: std::sync::Mutex<Option<Box<dyn FnOnce() + Send + Sync>>>,
}

pub(super) struct LeafGuard<'a> {
    _record: &'a LeafRecord,
    _collector: &'a GarbageCollector,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum LeafAccessError {
    NotLive,
    NotPublished,
    WrongType,
    Admission(RecordError),
}

/// A typed read retains the exact root token, admitted record and matching heap.
/// It owns no extra pool share, storage lease, or independently clonable root.
pub(super) struct LeafView<'a, T> {
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

impl<T> Publication<'_, T> {
    /// Failure preserves every original owner. Success never enters legacy indexes.
    #[expect(clippy::expect_used, reason = "validated unique private publication")]
    pub(super) fn try_into_canonical_leaf(mut self) -> Result<LeafRecord, Self> {
        let Some(metadata) = &self.metadata else {
            return Err(self);
        };
        if !metadata.canonical_leaf_eligible()
            || self.registration.is_some()
            || !self.construction.is_empty()
            || !matches!(
                self.root.as_ref().map(|root| &root.owner),
                Some(Owner::Edge(_))
            )
        {
            return Err(self);
        }
        let metadata = self.metadata.take().expect("private metadata");
        let mut root = self.root.take().expect("private pointer");
        // This pointer was never published in the legacy catalog. Disable its
        // legacy deletion notification before dropping it, including inside an
        // unrelated ambient construction scope. Metadata retains the allocation.
        root.owner = Owner::Edge(Weak::new());
        drop(root);
        Ok(LeafRecord {
            metadata,
            published: AtomicU8::new(0),
            graph_edges: 0,
            #[cfg(test)]
            after_admission: std::sync::Mutex::new(None),
        })
    }
}

impl LeafRecord {
    pub(super) fn borrow_root<'a, T: 'static>(
        root: &'a RootHandle<Self>,
        collector: &'a GarbageCollector,
    ) -> Result<LeafView<'a, T>, LeafAccessError> {
        let record = root.borrow_live_record().ok_or(LeafAccessError::NotLive)?;
        let admission = record
            .enter(collector)
            .map_err(LeafAccessError::Admission)?;
        if record.published.load(Ordering::Acquire) == 0 {
            return Err(LeafAccessError::NotPublished);
        }
        if !record.metadata.is_type::<T>() {
            return Err(LeafAccessError::WrongType);
        }
        // SAFETY: immutable TypeId pairs this pointer with initialized T. The
        // root token owns this exact cell, and all three borrows outlive the
        // returned reference. This sealed adapter exports no independent cleanup
        // owner and never registers in a legacy index. No relocation or mutable
        // payload access is provided. The publication latch is read with acquire
        // ordering; a Live root observation alone would not certify publication.
        let value = unsafe { &*record.metadata.trace.ptr.cast::<T>() };
        Ok(LeafView {
            _root: root,
            _admission: admission,
            value,
        })
    }

    pub(super) fn identity(&self) -> ObjectId {
        self.metadata.id
    }

    pub(super) fn address(&self) -> usize {
        self.metadata.ptr().0
    }

    #[cfg(test)]
    pub(super) fn published_for_test(&self) -> bool {
        self.published.load(Ordering::Acquire) != 0
    }

    /// Only for the synthetic Live-without-certificate access fixture. Real
    /// publication writes this monotonic latch through the catalog controller.
    #[cfg(test)]
    pub(super) fn set_certificate_for_test(&self, published: bool) {
        self.published.store(u8::from(published), Ordering::Release);
    }

    #[cfg(test)]
    pub(super) fn pause_after_admission(&self, hook: impl FnOnce() + Send + Sync + 'static) {
        *self.after_admission.lock().unwrap() = Some(Box::new(hook));
    }
}

impl Record for LeafRecord {
    type Context = GarbageCollector;
    type Guard<'a> = LeafGuard<'a>;

    fn enter<'a>(
        &'a self,
        collector: &'a GarbageCollector,
    ) -> Result<Entered<'a, LeafGuard<'a>>, RecordError> {
        if !std::ptr::eq(self.metadata.collector.as_ptr(), collector) {
            return Err(RecordError::Foreign);
        }
        // This sealed record is the sole payload cleanup owner: it was never
        // registered elsewhere and exports no metadata owners or storage leases.
        // Borrowing it excludes its destructor. Leaf eligibility excludes any
        // outgoing graph that would require separate admission. A general
        // MetadataRef or published Gc view cannot use this shortcut.
        #[cfg(test)]
        {
            let hook = self
                .after_admission
                .lock()
                .map_err(|_| RecordError::Closed)?
                .take();
            if let Some(hook) = hook {
                hook();
            }
        }
        Ok(Entered::new(
            LeafGuard {
                _record: self,
                _collector: collector,
            },
            &self.published,
        ))
    }
}

impl Drop for LeafRecord {
    fn drop(&mut self) {
        // A unique record dies after private cancellation or removal of its
        // canonical cell and all retained cell/root/controller owners. No storage
        // leases escape this adapter. Descriptor-owned credit remains charged
        // until the real allocation destructor and free have finished.
        self.metadata.drop_object();
    }
}

#[path = "canonical_leaf_benchmark.rs"]
pub(super) mod benchmark;

#[path = "canonical_leaf_contention.rs"]
pub(super) mod contention;
