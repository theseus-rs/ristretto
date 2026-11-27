//! Ownership context for fields bound to a traced allocation.
use crate::GarbageCollector;
use std::sync::Weak;

/// The heap and descriptor whose tracing protects embedded edges.
///
/// Hooks forward this borrowed context to embedded values. A context does not
/// itself retain the allocation or authorize an edge to escape into a host local.
#[derive(Clone, Copy, Debug)]
pub struct TraceOwner<'a> {
    collector: &'a Weak<GarbageCollector>,
    address: usize,
}

impl<'a> TraceOwner<'a> {
    /// Describes the allocation receiving attached or demoted fields.
    ///
    /// # Safety
    /// `address` must identify the actual live descriptor that owns those fields
    /// in `collector`. The caller must protect that descriptor and its graph for
    /// the hook call, exclude conflicting access, and publish the required
    /// barriers before exposing a mutation. Unpublished descriptors additionally
    /// require construction roots until their registration commits. A context
    /// must not be used to demote unrelated host values.
    #[must_use]
    pub unsafe fn new_unchecked(collector: &'a Weak<GarbageCollector>, address: usize) -> Self {
        Self { collector, address }
    }

    /// The collector whose tracing protects same-heap edges.
    #[must_use]
    pub fn collector(self) -> &'a Weak<GarbageCollector> {
        self.collector
    }

    /// The stable descriptor address used for owner mutation notifications.
    #[must_use]
    pub fn address(self) -> usize {
        self.address
    }
}
