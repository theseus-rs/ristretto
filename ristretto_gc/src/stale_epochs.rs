//! Disposal of birth proposals after their exact collection epoch has closed.
use crate::publication_directory::Directory;
use crate::work::BirthEpoch;
use arc_swap::ArcSwap;
use once_cell::race::OnceBox;
use portable_atomic::AtomicU64;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

type Epochs = Directory<Arc<BirthEpoch>>;

#[derive(Clone, Debug, Default)]
struct Entries {
    ready: Epochs,
    // A publisher admitted before closure may own a private slot indefinitely.
    // Peers may probe it even after an idle result; its completion signals again.
    waiting: Epochs,
}

#[derive(Debug, Default)]
struct Table {
    entries: ArcSwap<Entries>,
    ready_after: AtomicU64,
    waiting_after: AtomicU64,
    waiting_turn: AtomicBool,
}

#[derive(Debug, Default)]
pub(crate) struct StaleEpochs(OnceBox<Table>);

impl StaleEpochs {
    pub(crate) fn pending(&self) -> bool {
        self.0
            .get()
            .is_some_and(|t| t.entries.load().ready.len() != 0)
    }

    #[expect(
        clippy::expect_used,
        reason = "live Arc addresses fit in the directory"
    )]
    pub(crate) fn signal(&self, epoch: &Arc<BirthEpoch>) {
        let id = Arc::as_ptr(epoch).addr() as u64;
        self.0.get_or_init(Box::default).entries.rcu(|view| {
            let mut next = (**view).clone();
            if next.ready.get(id).is_none() {
                next.waiting = next.waiting.remove(id).0;
                next.ready = next
                    .ready
                    .insert(id, epoch.clone())
                    .expect("live epoch fits")
                    .0;
            }
            // Even a repeat signal invalidates a concurrent empty observation.
            Arc::new(next)
        });
    }

    pub(crate) fn step(&self) -> bool {
        let Some(table) = self.0.get() else {
            return false;
        };
        let epoch = {
            let view = table.entries.load();
            let waiting = view.waiting.len() != 0
                && (view.ready.len() == 0 || table.waiting_turn.fetch_xor(true, Ordering::Relaxed));
            let (entries, cursor) = if waiting {
                (&view.waiting, &table.waiting_after)
            } else {
                (&view.ready, &table.ready_after)
            };
            let Some((id, epoch)) = entries
                .next_after(Some(cursor.load(Ordering::Relaxed)))
                .or_else(|| entries.next_after(None))
            else {
                return false;
            };
            cursor.store(id, Ordering::Relaxed);
            epoch.clone()
        };
        // All peers retain access to this epoch while one proposal is being
        // released, including when its final descriptor invokes user code.
        let discarded = epoch.discard_one();
        let view = table.entries.load_full();
        let id = Arc::as_ptr(&epoch).addr() as u64;
        let ready = view.ready.get(id).is_some();
        let waiting = view.waiting.get(id).is_some();
        if !(ready || waiting) {
            return discarded;
        }
        // Sample writers first. A final completion racing the queue observation
        // also publishes a fresh view; its signal cannot be removed by this CAS.
        let writers = epoch.has_publishers();
        if epoch.has_visible_proposals() {
            if waiting {
                self.signal(&epoch);
            }
            return discarded;
        }
        if waiting && writers {
            return discarded;
        }
        let mut next = (*view).clone();
        next.ready = next.ready.remove(id).0;
        next.waiting = if writers {
            #[expect(clippy::expect_used, reason = "transfer of an existing key")]
            next.waiting.insert(id, epoch).expect("live epoch fits").0
        } else {
            next.waiting.remove(id).0
        };
        let old = table.entries.compare_and_swap(&view, Arc::new(next));
        Arc::ptr_eq(&old, &view) || discarded
    }

    // Shutdown takes one finite sample of dormant publishers. It drains every
    // already-visible sibling without waiting for an unpublished reservation.
    pub(crate) fn wake_visible(&self) {
        if let Some(table) = self.0.get() {
            let view = table.entries.load_full();
            let mut cursor = None;
            while let Some((id, epoch)) = view.waiting.next_after(cursor) {
                if epoch.has_visible_proposals() {
                    self.signal(epoch);
                }
                cursor = Some(id);
            }
        }
    }
}
