//! Numeric root IDs address slots within pages. Directory changes publish a
//! new immutable path. A parked reader holds storage containing numbers only.
use super::{MAX_TREE_GENERATION, OWNED_TREE_ROOT, TreeRoot};
use crate::publication_directory::Directory;
use arc_swap::ArcSwap;
use portable_atomic::AtomicU64;
use std::num::NonZeroU64;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

const PAGE_BITS: usize = 5;
const SLOTS: usize = 1 << PAGE_BITS;
#[path = "tree_root_page_state.rs"]
mod state;
use state::{Claim, Entry, Occupancy};

type BackingCache = Arc<Mutex<Option<Box<[Entry]>>>>;

#[derive(Debug)]
struct Page {
    occupied: Occupancy,
    slots: Box<[Entry]>,
    cache: BackingCache,
}

impl Page {
    fn new(cache: &BackingCache) -> Self {
        let slots = cache
            .try_lock()
            .ok()
            .and_then(|mut entry| entry.take())
            .unwrap_or_else(|| Box::new(std::array::from_fn::<_, SLOTS, _>(|_| Entry::default())));
        Self {
            occupied: Occupancy::default(),
            slots,
            cache: cache.clone(),
        }
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        // Last page owner: a stalled scan or insertion keeps its own backing.
        // The cache may miss under contention and never delays a root operation.
        if self.occupied.slots() == 0
            && let Ok(mut cache) = self.cache.try_lock()
            && cache.is_none()
        {
            *cache = Some(std::mem::take(&mut self.slots));
        }
    }
}

#[derive(Debug, Default)]
struct Pages {
    older: Directory<Arc<Page>>,
    recent: Option<(u64, Arc<Page>)>,
}

impl Pages {
    fn get(&self, key: u64) -> Option<&Arc<Page>> {
        if let Some((recent, page)) = &self.recent
            && *recent == key
        {
            Some(page)
        } else {
            self.older.get(key)
        }
    }

    fn next_after(&self, cursor: Option<u64>) -> Option<(u64, &Arc<Page>)> {
        let older = self.older.next_after(cursor);
        if let Some((key, page)) = &self.recent
            && cursor.is_none_or(|last| *key > last)
            && older.is_none_or(|(next, _)| *key < next)
        {
            return Some((*key, page));
        }
        older
    }

    fn is_recent(&self, key: u64) -> bool {
        self.recent
            .as_ref()
            .is_some_and(|(current, _)| *current == key)
    }
}

#[derive(Debug, Default)]
pub(crate) struct TreeRoots {
    pages: ArcSwap<Pages>,
    // The lowest numeric page usually contains all short-lived roots. Install
    // its directory membership first, then cache that permanent identity.
    first: OnceLock<Arc<Page>>,
    cache: BackingCache,
    generation: AtomicU64,
    // A hint only. Occupancy and the checked generation decide whether a
    // private registration can use an ID; manual IDs still use next_root_id.
    last_owned_vacancy: AtomicUsize,
    live: AtomicUsize,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TreeScan {
    Root(usize, usize),
    Advance(usize),
    Done,
}

#[expect(
    clippy::indexing_slicing,
    reason = "page slots derive from a five-bit index or a bitmap of those slots"
)]
impl TreeRoots {
    #[cfg(test)]
    pub(crate) fn read_view(&self) -> impl std::fmt::Debug {
        self.pages.load_full()
    }

    #[cfg(test)]
    pub(super) fn set_generation(&self, value: u64) {
        self.generation.store(value, Ordering::Relaxed);
    }

    pub(crate) fn len(&self) -> usize {
        self.live.load(Ordering::Relaxed)
    }

    pub(crate) fn insert(&self, next: &AtomicUsize, address: usize) -> TreeRoot {
        self.insert_kind(next, address, true)
    }

    pub(crate) fn insert_manual(&self, next: &AtomicUsize, address: usize) -> TreeRoot {
        self.insert_kind(next, address, false)
    }

    #[expect(
        clippy::expect_used,
        reason = "root generations fail closed before reuse"
    )]
    fn insert_kind(&self, next: &AtomicUsize, address: usize, owned: bool) -> TreeRoot {
        debug_assert_ne!(address, 0);
        let previous = self
            .generation
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
                (value < MAX_TREE_GENERATION).then(|| value + 1)
            })
            .expect("tree root generation exhausted");
        let generation = NonZeroU64::new((previous + 1) | if owned { OWNED_TREE_ROOT } else { 0 })
            .expect("nonzero tree root generation");
        if owned {
            let id = self.last_owned_vacancy.load(Ordering::Relaxed);
            if id != 0
                && let Some(root) = self.with_page((id >> PAGE_BITS) as u64, |page| {
                    let base = id & !(SLOTS - 1);
                    let slot = id & (SLOTS - 1);
                    let slot = match page.occupied.claim(1 << slot) {
                        Claim::Reserved => slot,
                        Claim::InUse => {
                            // Reuse another issued position on the hinted page. A
                            // racing claimant just sends this insertion to the cursor.
                            let issued =
                                next.load(Ordering::Relaxed).saturating_sub(base).min(SLOTS);
                            let available = ((1u64 << issued) - 1)
                                & !page.occupied.slots()
                                & !u64::from(base == 0);
                            let bit = available.isolate_lowest_one();
                            if bit == 0 || page.occupied.claim(bit) != Claim::Reserved {
                                return None;
                            }
                            bit.trailing_zeros() as usize
                        }
                        Claim::Sealed => return None,
                    };
                    self.live.fetch_add(1, Ordering::Relaxed);
                    page.slots[slot].publish(generation, address);
                    Some(TreeRoot {
                        id: base | slot,
                        generation,
                    })
                })
            {
                return root;
            }
        }
        loop {
            let id = next.fetch_add(1, Ordering::Relaxed);
            if id == 0 {
                continue;
            }
            let key = (id >> PAGE_BITS) as u64;
            let slot = id & (SLOTS - 1);
            let bit = 1 << slot;
            loop {
                let view = self.pages.load();
                let Some(page) = view.get(key) else {
                    self.install_page(key, &view);
                    continue;
                };
                match page.occupied.claim(bit) {
                    Claim::Reserved => {
                        self.live.fetch_add(1, Ordering::Relaxed);
                        page.slots[slot].publish(generation, address);
                        return TreeRoot { id, generation };
                    }
                    Claim::Sealed => self.install_page(key, &view),
                    Claim::InUse => break, // This numeric ID is still in use.
                }
            }
        }
    }

    fn with_page<T>(&self, key: u64, visit: impl FnOnce(&Arc<Page>) -> Option<T>) -> Option<T> {
        if key == 0
            && let Some(page) = self.first.get()
        {
            return visit(page);
        }
        visit(self.pages.load().get(key)?)
    }

    #[expect(
        clippy::expect_used,
        reason = "at most one page per machine-word ID prefix"
    )]
    fn install_page(&self, key: u64, view: &Arc<Pages>) {
        // Replacing a sealed page is safe even if a writer still holds its old
        // directory view. Its membership bits cannot be reopened.
        let mut older = view.older.remove(key).0;
        let previous = view.recent.as_ref().filter(|(old, _)| *old != key);
        if let Some((old, page)) = previous
            && (*old == 0 || (!page.occupied.seal_empty() && !page.occupied.is_sealed()))
        {
            older = older
                .insert(*old, page.clone())
                .expect("finite page directory")
                .0;
        }
        let fresh = Arc::new(Page::new(&self.cache));
        let proposed = Arc::new(Pages {
            older,
            recent: Some((key, fresh.clone())),
        });
        if Arc::ptr_eq(&self.pages.compare_and_swap(view, proposed), view) {
            if key == 0 {
                let _ = self.first.set(fresh);
            }
            if let Some((old, page)) = previous {
                self.retire_empty(*old, page);
            }
        }
    }

    fn retire_empty(&self, key: u64, page: &Arc<Page>) {
        if key == 0 || (!page.occupied.seal_empty() && !page.occupied.is_sealed()) {
            return;
        }
        loop {
            let view = self.pages.load();
            if !view
                .older
                .get(key)
                .is_some_and(|current| Arc::ptr_eq(current, page))
            {
                return;
            }
            let proposed = Arc::new(Pages {
                older: view.older.remove(key).0,
                recent: view.recent.clone(),
            });
            if Arc::ptr_eq(&self.pages.compare_and_swap(&*view, proposed), &view) {
                return;
            }
        }
    }

    pub(crate) fn remove(&self, id: usize, expected: Option<NonZeroU64>) -> Option<usize> {
        let key = (id >> PAGE_BITS) as u64;
        self.with_page(key, |page| {
            let slot = id & (SLOTS - 1);
            let address = page.slots[slot].remove(expected)?;
            self.live.fetch_sub(1, Ordering::Relaxed);
            let bit = 1u64 << slot;
            if page.occupied.release(bit) && key != 0 && !self.pages.load().is_recent(key) {
                self.retire_empty(key, page);
            } else if expected.is_some_and(|generation| generation.get() & OWNED_TREE_ROOT != 0) {
                self.last_owned_vacancy.store(id, Ordering::Relaxed);
            }
            Some(address)
        })
    }

    /// Inspect at most one page per collector work unit. Re-reading a numeric
    /// cursor after a shading retry does not retain a page or a mutation claim.
    #[expect(
        clippy::expect_used,
        reason = "page keys contain only usize ID prefixes"
    )]
    pub(crate) fn scan(&self, after: usize) -> TreeScan {
        let Some(first) = after.checked_add(1) else {
            return TreeScan::Done;
        };
        let key = (first >> PAGE_BITS) as u64;
        let view = self.pages.load();
        let (key, page, start) = if let Some(page) = view.get(key) {
            (key, page, first & (SLOTS - 1))
        } else if let Some((next, page)) = view.next_after(Some(key)) {
            (next, page, 0)
        } else {
            return TreeScan::Done;
        };
        let base = usize::try_from(key).expect("page prefix") << PAGE_BITS;
        let mut bits = page.occupied.slots() & (u64::MAX << start);
        while bits != 0 {
            let slot = bits.trailing_zeros() as usize;
            bits &= bits - 1;
            if let Some((_, address)) = page.slots[slot].read() {
                return TreeScan::Root(base | slot, address);
            }
        }
        TreeScan::Advance(base | (SLOTS - 1))
    }

    #[expect(
        clippy::expect_used,
        reason = "page keys contain only usize ID prefixes"
    )]
    pub(crate) fn matching(&self, address: usize) -> Vec<TreeRoot> {
        let view = self.pages.load();
        let mut found = Vec::new();
        let mut cursor = None;
        while let Some((key, page)) = view.next_after(cursor) {
            let base = usize::try_from(key).expect("page prefix") << PAGE_BITS;
            for (slot, entry) in page.slots.iter().enumerate() {
                if let Some((generation, current)) = entry.read()
                    && address == current
                {
                    found.push(TreeRoot {
                        id: base | slot,
                        generation,
                    });
                }
            }
            cursor = Some(key);
        }
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bursts_reuse_owned_holes_while_a_manual_root_remains_live() {
        let roots = TreeRoots::default();
        let next = AtomicUsize::new(1);
        let manual = roots.insert_manual(&next, 99);
        for batch in 0..16 {
            let registrations: Vec<_> = (1..=12)
                .map(|address| roots.insert(&next, address))
                .collect();
            assert_eq!(next.load(Ordering::Relaxed), 14, "batch {batch}");
            for (i, registration) in registrations.into_iter().enumerate() {
                assert_eq!(
                    roots.remove(registration.id, Some(registration.generation)),
                    Some(i + 1)
                );
                assert!(
                    roots
                        .remove(registration.id, Some(registration.generation))
                        .is_none()
                );
            }
        }
        assert_eq!(roots.remove(manual.id, None), Some(99));
    }

    #[test]
    fn owned_roots_reuse_a_vacancy_without_rewinding_manual_ids() {
        let roots = TreeRoots::default();
        let next = AtomicUsize::new(1);
        let first = roots.insert(&next, 10);
        assert_eq!(roots.remove(first.id, Some(first.generation)), Some(10));
        let page = roots.read_view();
        for address in 1..=128 {
            let current = roots.insert(&next, address);
            assert_eq!(current.id, first.id);
            assert_ne!(current.generation, first.generation);
            assert_eq!(roots.remove(first.id, Some(first.generation)), None);
            assert_eq!(roots.remove(current.id, None), None);
            assert_eq!(
                roots.remove(current.id, Some(current.generation)),
                Some(address)
            );
        }
        let manual = roots.insert_manual(&next, 300);
        assert_ne!(manual.id, first.id);
        assert_eq!(next.load(Ordering::Relaxed), 3);
        let current = roots.insert(&next, 400);
        let other = roots.insert(&next, 500);
        assert_ne!(current.id, other.id);
        assert_ne!(other.id, manual.id);
        assert_eq!(roots.remove(manual.id, None), Some(300));
        assert_eq!(
            roots.remove(current.id, Some(current.generation)),
            Some(400)
        );
        assert_eq!(roots.remove(other.id, Some(other.generation)), Some(500));
        drop(page);
        assert_eq!(roots.len(), 0);
    }

    #[test]
    fn a_private_slot_claim_cannot_pin_unrelated_root_membership() {
        let roots = TreeRoots::default();
        let next = AtomicUsize::new(1);
        let seed = roots.insert(&next, 111);
        let view = roots.pages.load_full();
        let parked = view.get(0).unwrap();
        assert_eq!(parked.occupied.claim(1 << 2), Claim::Reserved);
        let other = roots.insert(&next, 333);
        assert_eq!(other.id, 3);
        assert_eq!(roots.remove(seed.id, Some(seed.generation)), Some(111));
        assert_eq!(roots.scan(0), TreeScan::Root(3, 333));
        assert_eq!(roots.remove(other.id, Some(other.generation)), Some(333));
        assert_eq!(roots.len(), 0);
        assert!(parked.occupied.release(1 << 2));
    }

    #[test]
    fn a_parked_first_page_is_still_scannable_after_installing_later_pages() {
        let roots = TreeRoots::default();
        let next = AtomicUsize::new(1);
        let original = roots.insert(&next, 11);
        let view = roots.pages.load_full();
        let first = view.get(0).unwrap();
        next.store(SLOTS, Ordering::Relaxed);
        let another = roots.insert_manual(&next, 22);
        assert_eq!(
            roots.remove(original.id, Some(original.generation)),
            Some(11)
        );
        let replacement = roots.insert(&next, 33);
        assert_eq!(replacement.id, original.id);
        assert_ne!(replacement.generation, original.generation);
        assert!(Arc::ptr_eq(roots.first.get().unwrap(), first));
        assert!(Arc::ptr_eq(roots.pages.load().get(0).unwrap(), first));
        assert_eq!(roots.scan(0), TreeScan::Root(replacement.id, 33));
        assert!(
            roots
                .remove(original.id, Some(original.generation))
                .is_none()
        );
        assert_eq!(roots.remove(another.id, None), Some(22));
        assert_eq!(
            roots.remove(replacement.id, Some(replacement.generation)),
            Some(33)
        );
    }

    #[test]
    fn vacated_page_identity_cannot_unlink_an_id_reused_by_a_new_page() {
        let roots = TreeRoots::default();
        let next = AtomicUsize::new(SLOTS + 1);
        let first = roots.insert(&next, 10);
        next.store(2 * SLOTS, Ordering::Relaxed);
        let later = roots.insert(&next, 20);
        let view = roots.pages.load_full();
        let retired = view.get(1).unwrap();
        assert_eq!(roots.remove(first.id, Some(first.generation)), Some(10));
        assert_eq!(retired.occupied.claim(2), Claim::Sealed);
        next.store(first.id, Ordering::Relaxed);
        let reused = roots.insert(&next, 30);
        assert_eq!(first.id, reused.id);
        assert_ne!(first.generation, reused.generation);
        assert_eq!(roots.remove(first.id, Some(first.generation)), None);
        assert_eq!(roots.scan(SLOTS), TreeScan::Root(reused.id, 30));
        assert_eq!(roots.remove(later.id, Some(later.generation)), Some(20));
        assert_eq!(roots.remove(reused.id, Some(reused.generation)), Some(30));
        assert_eq!(roots.len(), 0);
        assert_eq!(roots.pages.load().older.len(), 0);
        assert!(roots.pages.load().is_recent(1));
        assert_eq!(retired.occupied.slots(), 0);
    }
}
