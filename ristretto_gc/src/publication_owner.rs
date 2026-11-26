//! A private allocation owns its exact roots and descriptor through publication.
//! This ownership can move. Registration commits the indexes and birth decision once.
use super::{Gc, GcRootGuard, Owner};
use crate::GarbageCollector;
use crate::construction::Detached;
use crate::metadata::MetadataRef;
use crate::roots::Registration;
use std::sync::{Arc, Weak};

pub(super) struct Publication<'gc, T> {
    root: Option<Gc<T>>,
    registration: Option<Registration>,
    metadata: Option<MetadataRef>,
    construction: Detached<'gc>,
}

impl<'gc, T: crate::Trace> Publication<'gc, T> {
    pub(super) fn new(
        collector: &GarbageCollector,
        root: Gc<T>,
        metadata: MetadataRef,
        construction: Detached<'gc>,
        rooted: bool,
    ) -> Self {
        // Establish the complete cleanup owner before installing the parent root.
        // If root installation unwinds, metadata dies before child protection.
        let mut publication = Self {
            root: Some(root),
            registration: None,
            metadata: Some(metadata),
            construction,
        };
        assert!(
            publication.construction.belongs_to(collector),
            "foreign construction protection"
        );
        assert!(
            std::ptr::eq(publication.metadata().collector.as_ptr(), collector),
            "foreign prepared descriptor"
        );
        if rooted {
            publication.registration = Some(collector.add_private_root(publication.metadata()));
        }
        publication
    }

    #[expect(
        clippy::expect_used,
        reason = "private metadata remains owned until finish or drop"
    )]
    fn metadata(&self) -> &MetadataRef {
        self.metadata.as_ref().expect("private descriptor")
    }

    fn register(&self, collector: &GarbageCollector) {
        collector.register_prepared(self.metadata(), &self.construction);
        collector.record_allocation_debt(self.metadata().size());
    }

    #[expect(
        clippy::expect_used,
        reason = "rooted publication transfers both exact owners once"
    )]
    pub(super) fn finish_rooted(mut self, collector: &Arc<GarbageCollector>) -> GcRootGuard<T> {
        assert!(self.registration.is_some(), "unrooted publication");
        self.register(collector);
        let root = self.root.take().expect("private handle");
        let registration = self.registration.take().expect("initial registration");
        GcRootGuard::new(collector.clone(), registration, root)
    }

    #[expect(
        clippy::expect_used,
        reason = "unrooted publication transfers its private handle once"
    )]
    pub(super) fn finish_unrooted(mut self, collector: &GarbageCollector) -> Gc<T> {
        assert!(self.registration.is_none(), "rooted publication");
        self.register(collector);
        self.root.take().expect("private handle")
    }
}

impl<T> Publication<'_, T> {
    fn remove_parent_root(&mut self) {
        if let Some(registration) = self.registration.take() {
            if let Some(root) = &mut self.root
                && let Owner::Edge(owner) = &mut root.owner
            {
                // Exact parent-root removal supplies its deletion barrier.
                *owner = Weak::new();
            }
            self.construction
                .collector()
                .remove_detached_root(&registration);
        }
    }

    /// Cancel a private payload even when announced records retain its metadata.
    ///
    /// # Safety
    /// The allocation must never have been registered, every payload/graph view
    /// and descriptor storage lease must have ended, and all future publication
    /// and view admission must be
    /// permanently excluded. Descriptor clones may retain storage identity only.
    pub(super) unsafe fn cancel_private(mut self) {
        self.remove_parent_root();
        drop(self.root.take());
        if let Some(metadata) = &self.metadata {
            metadata.drop_object();
        }
        // Ordinary Drop still owns metadata and child protection on every unwind.
    }
}

impl<T> Drop for Publication<'_, T> {
    fn drop(&mut self) {
        let unfinished = self.root.is_some();
        self.remove_parent_root();
        drop(self.root.take());
        if unfinished
            && let Some(metadata) = &self.metadata
            && metadata.birth.unpublished()
        {
            // Ordinary registration already rolled back the private indexes.
            // Skipped queue records can retain metadata, but can never trace this
            // payload. Destroy it while its construction roots are still owned.
            metadata.drop_object();
        }
        // Child roots and conditional-owner reservations remain live through all
        // payload destruction, including unwind. Field drop then releases them.
        drop(self.metadata.take());
    }
}

#[cfg(test)]
#[path = "publication_owner_tests.rs"]
mod tests;

// Shared headers carry no strong collector owner. Private resource access requires
// an admitted handle borrowing the matching live collector.
#[path = "publication_shared.rs"]
mod shared;
