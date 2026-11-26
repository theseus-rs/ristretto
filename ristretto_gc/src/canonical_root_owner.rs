//! One unique catalog token shared by typed runtime aliases. Private backing is
//! allocated before bounded polling; published ownership is immutable.
use super::LeafRecord;
use crate::canonical_catalog as gc_catalog;
use gc_catalog::bounded::RootHandle;
use ristretto_gc_catalog::arc_requested_bytes;
use std::ops::Deref;
use std::sync::Arc;

struct Storage {
    root: Option<RootHandle<LeafRecord>>,
}

pub(super) struct PreparedRoot {
    owner: Arc<Storage>,
}

#[derive(Clone)]
pub(super) struct SharedRoot {
    owner: Option<Arc<Storage>>,
}

/// Requested Arc backing, including its strong/weak counters and alignment.
/// Allocator overhead and caller-owned alias variables are accounted separately.
pub(super) fn backing_bytes() -> Option<usize> {
    arc_requested_bytes::<Storage>()
}

impl PreparedRoot {
    pub(super) fn new() -> Self {
        Self {
            owner: Arc::new(Storage { root: None }),
        }
    }

    #[expect(
        clippy::expect_used,
        reason = "private wrapper has never exposed an alias"
    )]
    pub(super) fn finish(mut self, root: RootHandle<LeafRecord>) -> SharedRoot {
        Arc::get_mut(&mut self.owner)
            .expect("private root owner")
            .root = Some(root);
        SharedRoot {
            owner: Some(self.owner),
        }
    }
}

impl Deref for SharedRoot {
    type Target = RootHandle<LeafRecord>;

    #[expect(clippy::expect_used, reason = "only initialized owners can be shared")]
    fn deref(&self) -> &Self::Target {
        self.owner
            .as_deref()
            .expect("live root owner")
            .root
            .as_ref()
            .expect("initialized root owner")
    }
}

impl Drop for SharedRoot {
    #[expect(
        clippy::expect_used,
        reason = "each alias owns one live Arc until its Drop"
    )]
    fn drop(&mut self) {
        // Every alias takes this path. into_inner frees the final Arc backing
        // before returning its value, so the unique root's lease cannot refund
        // the runtime charge while the outer owner allocation still exists.
        // Ordinary Arc Drop would destroy that lease before freeing its backing.
        if let Some(storage) = Arc::into_inner(self.owner.take().expect("live root owner")) {
            drop(storage);
        }
    }
}
