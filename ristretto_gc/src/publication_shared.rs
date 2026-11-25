//! Shared headers are inert after their last heap-borrowing handle disappears.
//! This owns prepared graphs; it does not make registration effects replay-safe.
use super::Publication;
use crate::GarbageCollector;
use portable_atomic::AtomicU64;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Weak};

#[path = "publication_handle_state.rs"]
mod state;
pub(super) use state::AcquireError;

#[path = "publication_record_state.rs"]
mod binding;
pub(super) use binding::ReserveError;

struct Resources<T> {
    root: Option<super::Gc<T>>,
    registration: Option<super::Registration>,
    metadata: Option<super::MetadataRef>,
    construction: crate::construction::OwnedTokens,
}

struct Core<T> {
    collector: Weak<GarbageCollector>,
    handles: state::State,
    records: binding::State,
    // Immutable after construction. Only an admitted handle may dereference it.
    // The owning MetadataRef stays in resources until handles reaches zero.
    descriptor: AtomicPtr<crate::metadata::ObjectMetadata>,
    resources: Mutex<Option<Resources<T>>>,
}

impl<T> Core<T> {
    fn cleanup(&self, collector: &GarbageCollector) {
        let publication = self.take(collector);
        if self.handles.announced() {
            // SAFETY: announcement atomically forbids exclusive restoration into
            // legacy registration. No canonical registration API exists yet.
            // The final handle closes every future view/publication admission;
            // all earlier views borrow handles and have ended before this release.
            // Retained PreparedStorage values expose no payload without a handle.
            // This private interface creates no descriptor storage leases:
            // PreparedStorage clones metadata ownership, not storage admission.
            unsafe { publication.cancel_private() };
        } else {
            drop(publication);
        }
    }

    #[expect(
        clippy::expect_used,
        reason = "closure extracts private resources once"
    )]
    fn take<'gc>(&self, collector: &'gc GarbageCollector) -> Publication<'gc, T> {
        // Only the unique final handle accesses this slot. try_lock enforces
        // that invariant without waiting. No user callback runs with its guard.
        let resources = self
            .resources
            .try_lock()
            .expect("exclusive private resource extraction")
            .take()
            .expect("closed publication resources");
        Publication {
            root: resources.root,
            registration: resources.registration,
            metadata: resources.metadata,
            construction: resources.construction.attach(collector),
        }
    }
}

impl<T> Drop for Core<T> {
    #[expect(
        clippy::expect_used,
        reason = "private extraction never poisons the slot"
    )]
    fn drop(&mut self) {
        debug_assert!(
            self.resources
                .get_mut()
                .expect("private resource slot")
                .is_none()
        );
    }
}

/// May be retained in a collector registry without creating a collector cycle.
/// A raw header grants neither payload access nor delayed private destruction.
pub(super) struct Header<T>(Arc<Core<T>>);

impl<T> Clone for Header<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub(super) struct Handle<'gc, T> {
    header: Header<T>,
    collector: &'gc GarbageCollector,
    active: bool,
}

/// A borrowed descriptor keeps its admitted handle and private graph protection
/// live. This is neither an owning `MetadataRef` nor a publication certificate.
pub(super) struct View<'handle> {
    metadata: &'handle crate::metadata::ObjectMetadata,
}

impl View<'_> {
    pub(super) fn metadata(&self) -> &crate::metadata::ObjectMetadata {
        self.metadata
    }
}

impl<'handle> View<'handle> {
    /// The caller must protect this exact metadata and its transitive graph
    /// through the supplied guard's borrow, including against collector shutdown.
    unsafe fn from_guard(
        descriptor: *const crate::metadata::ObjectMetadata,
        _guard: &'handle impl Sized,
    ) -> Self {
        // SAFETY: the admitted handle supplied by each caller provides the
        // descriptor, graph and heap lifetime required by this function.
        Self {
            metadata: unsafe { &*descriptor },
        }
    }
}

/// One heterogeneous header representation for a collector's prepared records.
/// Erasing an existing `Arc` changes its pointer metadata, with no second allocation.
pub(super) struct ErasedHeader(Arc<dyn ErasedResources>);

trait ErasedResources: Send + Sync {
    fn collector(&self) -> &Weak<GarbageCollector>;
    fn handles(&self) -> &state::State;
    fn records(&self) -> &binding::State;
    fn descriptor(&self) -> *const crate::metadata::ObjectMetadata;
    fn cleanup(&self, collector: &GarbageCollector);
}

impl<T: Send + Sync> ErasedResources for Core<T> {
    fn collector(&self) -> &Weak<GarbageCollector> {
        &self.collector
    }
    fn handles(&self) -> &state::State {
        &self.handles
    }
    fn records(&self) -> &binding::State {
        &self.records
    }
    fn descriptor(&self) -> *const crate::metadata::ObjectMetadata {
        self.descriptor.load(Ordering::Relaxed)
    }
    fn cleanup(&self, collector: &GarbageCollector) {
        Core::cleanup(self, collector);
    }
}

impl Clone for ErasedHeader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub(super) struct ErasedHandle<'gc> {
    header: ErasedHeader,
    collector: &'gc GarbageCollector,
}

/// Owns descriptor storage, while a matching admitted handle protects its graph.
/// After cancellation the descriptor is inert even if this storage owner survives.
#[derive(Clone)]
pub(super) struct PreparedStorage {
    metadata: super::MetadataRef,
    header: ErasedHeader,
}

impl PreparedStorage {
    pub(super) fn identity(&self) -> crate::ObjectId {
        self.metadata.id
    }

    pub(super) fn view<'handle>(
        &'handle self,
        handle: &'handle ErasedHandle<'_>,
    ) -> Option<View<'handle>> {
        Arc::ptr_eq(&self.header.0, &handle.header.0).then_some(View {
            metadata: &self.metadata,
        })
    }
}

/// One candidate canonical record for one exact private payload. This is not yet
/// collector membership, a published cleanup flag, or ownership of its result.
/// Losing a private creation may discard an unbound record; a bound record's
/// identity is permanent and cannot be replaced by preparing another token.
pub(super) struct PreparedRecord {
    storage: PreparedStorage,
    ticket: binding::Ticket,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum EnterError {
    Admission(AcquireError),
    BoundElsewhere,
}

/// The record and heap borrows outlive every descriptor view. The handle keeps
/// the exact private graph alive; record storage by itself grants no such lease.
pub(super) struct RecordGuard<'record, 'gc> {
    record: &'record PreparedRecord,
    handle: ErasedHandle<'gc>,
}
impl PreparedRecord {
    pub(super) fn identity(&self) -> crate::ObjectId {
        self.storage.identity()
    }

    pub(super) fn enter<'record, 'gc>(
        &'record self,
        collector: &'gc GarbageCollector,
    ) -> Result<RecordGuard<'record, 'gc>, EnterError> {
        let handle = self
            .storage
            .header
            .acquire(collector)
            .map_err(EnterError::Admission)?;
        if !self.storage.header.0.records().bind(&self.ticket) {
            return Err(EnterError::BoundElsewhere);
        }
        Ok(RecordGuard {
            record: self,
            handle,
        })
    }
}
impl RecordGuard<'_, '_> {
    #[expect(
        clippy::expect_used,
        reason = "admission retains this exact record header"
    )]
    pub(super) fn view(&self) -> View<'_> {
        self.record
            .storage
            .view(&self.handle)
            .expect("matching prepared record")
    }
}

impl<'gc, T> Publication<'gc, T> {
    #[expect(
        clippy::expect_used,
        reason = "private publication owns its descriptor"
    )]
    pub(super) fn share(mut self) -> (Header<T>, Handle<'gc, T>) {
        let collector = self.construction.collector();
        let descriptor =
            super::MetadataRef::as_ptr(self.metadata.as_ref().expect("private descriptor"));
        let header = Header(Arc::new(Core {
            collector: collector.weak(),
            handles: state::State::new(),
            records: binding::State::new(),
            descriptor: AtomicPtr::new(descriptor.cast_mut()),
            resources: Mutex::new(Some(Resources {
                root: self.root.take(),
                registration: self.registration.take(),
                metadata: self.metadata.take(),
                construction: self.construction.take_owned(),
            })),
        }));
        let handle = Handle {
            header: header.clone(),
            collector,
            active: true,
        };
        (header, handle)
    }
}

impl<T> Header<T> {
    /// A matching live heap and one successful count CAS authorize one handle.
    /// Foreign heaps are rejected before touching admission or private resources.
    pub(super) fn acquire<'gc>(
        &self,
        collector: &'gc GarbageCollector,
    ) -> Result<Handle<'gc, T>, AcquireError> {
        if !std::ptr::eq(self.0.collector.as_ptr(), collector) {
            return Err(AcquireError::Foreign);
        }
        self.0.handles.acquire()?;
        Ok(Handle {
            header: self.clone(),
            collector,
            active: true,
        })
    }
}

impl<T: Send + Sync + 'static> Header<T> {
    pub(super) fn erase(&self) -> ErasedHeader {
        ErasedHeader(self.0.clone())
    }

    #[cfg(test)]
    pub(super) fn shares_backing_with(&self, erased: &ErasedHeader) -> bool {
        std::ptr::addr_eq(Arc::as_ptr(&self.0), Arc::as_ptr(&erased.0))
    }
}

impl ErasedHeader {
    pub(super) fn acquire<'gc>(
        &self,
        collector: &'gc GarbageCollector,
    ) -> Result<ErasedHandle<'gc>, AcquireError> {
        if !std::ptr::eq(self.0.collector().as_ptr(), collector) {
            return Err(AcquireError::Foreign);
        }
        self.0.handles().acquire()?;
        Ok(ErasedHandle {
            header: self.clone(),
            collector,
        })
    }
}

impl ErasedHandle<'_> {
    /// Reserve identity before irreversible announcement. Reservation failure
    /// grants no record and does not change restoration eligibility.
    pub(super) fn prepare_record(&self) -> Result<PreparedRecord, ReserveError> {
        let ticket = self.header.0.records().reserve()?;
        Ok(PreparedRecord {
            storage: self.prepare_storage(),
            ticket,
        })
    }

    #[cfg(test)]
    pub(super) fn exhaust_record_nonces_for_test(&self) {
        self.header.0.records().exhaust_for_test();
    }

    /// Prepared storage is permanently excluded from the legacy registration path.
    /// Actual canonical membership transfer is a separate, unimplemented step.
    pub(super) fn prepare_storage(&self) -> PreparedStorage {
        self.header.0.handles().announce();
        // SAFETY: this pointer was obtained from the exact MetadataRef::as_ptr,
        // retaining reference-count provenance. This admitted handle keeps that
        // owner and its graph live throughout cloning. The opaque result exposes
        // payload metadata only with a matching live handle, and final announced
        // cleanup destroys the private payload before releasing child protection.
        let metadata = unsafe { super::MetadataRef::clone_from_ptr(self.header.0.descriptor()) };
        PreparedStorage {
            metadata,
            header: self.header.clone(),
        }
    }

    pub(super) fn view(&self) -> View<'_> {
        // SAFETY: this admitted handle holds the same count as typed handles.
        // That count excludes extraction of metadata and construction roots;
        // this borrow also retains the matching collector. Erasure adds no
        // payload cast and cannot change the descriptor's actual Rust type.
        unsafe { View::from_guard(self.header.0.descriptor(), self) }
    }
}

impl Drop for ErasedHandle<'_> {
    fn drop(&mut self) {
        if self.header.0.handles().release() {
            self.header.0.cleanup(self.collector);
        }
    }
}

impl<'gc, T> Handle<'gc, T> {
    /// Read immutable descriptor identity without locking the private result slot.
    /// Any phase observations still require the canonical operation's validation.
    pub(super) fn view(&self) -> View<'_> {
        // SAFETY: share initializes this pointer from the exact owning MetadataRef
        // before the header is exposed; it is never changed. This active handle
        // prevents both final release and exclusive restoration from reaching
        // zero. The resources therefore retain the metadata and construction roots
        // for this entire borrow. No owning metadata or private root can escape
        // through View. Its shared heap borrow also excludes collector shutdown.
        unsafe { View::from_guard(self.header.0.descriptor.load(Ordering::Relaxed), self) }
    }

    /// Registration still needs exclusive private ownership. Retained helpers
    /// therefore prevent this conversion; this is not shared publication helping.
    pub(super) fn restore(mut self) -> Result<Publication<'gc, T>, Self> {
        if !self.header.0.handles.close_if_unique() {
            return Err(self);
        }
        self.active = false;
        // This handle won the unique one-to-zero transition.
        Ok(self.header.0.take(self.collector))
    }

    #[cfg(test)]
    pub(super) fn with_result_slot_locked(&self, operation: impl FnOnce()) {
        let _slot = self.header.0.resources.try_lock().unwrap();
        operation();
    }
}

impl<T> Drop for Handle<'_, T> {
    fn drop(&mut self) {
        if self.active && self.header.0.handles.release() {
            // The final release uniquely owns extraction. Its heap borrow
            // remains live through ordinary Publication's ordered cleanup.
            self.header.0.cleanup(self.collector);
        }
    }
}
