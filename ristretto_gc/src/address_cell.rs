//! Address reservations admit one installer or remover per slot. Each change
//! happens before birth or after revocation; no visible object needs a writer.
use crate::metadata::MetadataRef;
use arc_swap::ArcSwapAny;
use once_cell::race::OnceBox;
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

type Shared = ArcSwapAny<Option<MetadataRef>>;
#[path = "address_gate.rs"]
mod address_gate;
use address_gate::{Gate, ReadMode, WriteMode};

pub(super) struct Cell {
    state: Gate,
    primary: UnsafeCell<Option<MetadataRef>>,
    shared: OnceBox<Shared>,
}

// SAFETY: admitted readers share primary. An exclusive writer or the unique
// cleanup claimant takes it only after excluding every admitted reader.
unsafe impl Sync for Cell {}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cell")
            .field("state", &self.state)
            .finish_non_exhaustive()
    }
}

pub(super) enum Read<'a> {
    Primary(&'a Cell),
    Shared(arc_swap::Guard<Option<MetadataRef>>),
    Vacant,
}

impl Deref for Read<'_> {
    type Target = Option<MetadataRef>;
    fn deref(&self) -> &Self::Target {
        match self {
            // SAFETY: this guard owns one reader admission on primary.
            Self::Primary(cell) => unsafe { &*cell.primary.get() },
            Self::Shared(guard) => guard,
            Self::Vacant => &None,
        }
    }
}
impl Drop for Read<'_> {
    fn drop(&mut self) {
        if let Self::Primary(cell) = self {
            cell.release();
        }
    }
}

impl From<Option<MetadataRef>> for Cell {
    fn from(value: Option<MetadataRef>) -> Self {
        Self {
            state: Gate::default(),
            primary: UnsafeCell::new(value),
            shared: OnceBox::new(),
        }
    }
}

impl Cell {
    fn cleanup(&self) {
        // SAFETY: only the gate's unique cleanup claimant calls this. Shared
        // mode permanently excludes new primary readers and the gate acquired
        // the last reader's release before granting cleanup ownership.
        drop(unsafe { (*self.primary.get()).take() });
    }

    fn release(&self) {
        if self.state.release() {
            self.cleanup();
        }
    }

    pub(super) fn load(&self) -> Read<'_> {
        let mode = self.state.read();
        if mode == ReadMode::Primary {
            return Read::Primary(self);
        }
        self.release();
        if mode == ReadMode::Vacant {
            return Read::Vacant;
        }
        self.shared
            .get()
            .map_or(Read::Vacant, |shared| Read::Shared(shared.load()))
    }

    pub(super) fn load_full(&self) -> Option<MetadataRef> {
        self.load().as_ref().cloned()
    }

    /// The live shard excludes retirement and the address reservation excludes
    /// replacement. Decide while the exact entry is readable, then take it.
    pub(super) fn take_if(&self, revoke: impl FnOnce(&MetadataRef) -> bool) -> Option<MetadataRef> {
        let read = self.load();
        if !revoke(read.as_ref()?) {
            return None;
        }
        if matches!(read, Read::Primary(_)) && self.state.take_reader() {
            // The CAS consumed this reader as the unique writer. Its normal
            // Drop would incorrectly release a reader that no longer exists.
            std::mem::forget(read);
            // SAFETY: the writer excludes every reader of this primary slot.
            let value = unsafe { (*self.primary.get()).take() };
            self.state.finish_exclusive();
            value
        } else {
            drop(read);
            self.swap(None)
        }
    }

    pub(super) fn store(&self, value: Option<MetadataRef>) {
        drop(self.swap(value));
    }

    pub(super) fn swap(&self, value: Option<MetadataRef>) -> Option<MetadataRef> {
        match self.state.write() {
            WriteMode::Shared => return self.shared.get()?.swap(value),
            WriteMode::Exclusive => {
                // SAFETY: the gate excludes both earlier and new primary readers.
                let old = unsafe { std::mem::replace(&mut *self.primary.get(), value) };
                self.state.finish_exclusive();
                return old;
            }
            WriteMode::Promote => {}
        }
        // SAFETY: primary is immutable throughout this transition. Existing
        // readers may park indefinitely without blocking the new generation.
        let old = unsafe { (&*self.primary.get()).clone() };
        assert!(self.shared.set(Box::new(Shared::from(value))).is_ok());
        if self.state.publish_shared() {
            self.cleanup();
        }
        old
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::Cell;
    use crate::{GarbageCollector, Gc};

    #[test]
    #[expect(clippy::panic_in_result_fn, reason = "assert exact handoff ownership")]
    fn parked_primary_reader_preserves_only_its_original_descriptor() -> crate::Result<()> {
        let gc = GarbageCollector::new();
        let first = Gc::try_new(&gc, 42_u64)?.into_rooted();
        let next = Gc::try_new(&gc, 43_u64)?.into_rooted();
        let first_id = first.identity();
        let next_id = next.identity();
        let lookup = |root: &Gc<u64>| {
            gc.traced_edge(root.as_ptr().addr(), root.identity())
                .expect("descriptor")
        };
        let cell = Cell::from(Some(lookup(&first)));
        let reader = cell.load();
        let replacement = lookup(&next);
        std::thread::scope(|scope| {
            scope
                .spawn(|| {
                    assert_eq!(cell.swap(None).expect("retired owner").id, first_id);
                    cell.store(Some(replacement));
                    assert_eq!(cell.load_full().expect("new slot").id, next_id);
                    assert_eq!(cell.swap(None).expect("new owner").id, next_id);
                })
                .join()
                .expect("publisher");
        });
        drop((first, next));
        gc.collect_full()?;
        assert_eq!(gc.statistics()?.bytes_managed, 0);
        assert_eq!(reader.as_ref().expect("old snapshot").id, first_id);
        assert!(!cell.state.cleaned());
        drop(reader);
        assert!(cell.state.cleaned());
        assert!(cell.load_full().is_none());
        Ok(())
    }
}
