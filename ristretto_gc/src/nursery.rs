//! Readable nursery membership. The object's address shard serializes all writes
//! to one directory; scanners take an owning view without acquiring that shard.
use super::{Cursor, Scan, Shards, address_bucket, shard_count};
use crate::metadata::MetadataRef;
use crate::publication_directory::Directory;
use arc_swap::ArcSwap;
use portable_atomic::AtomicU64;
use std::collections::BTreeMap;
use std::ops::Deref;
#[cfg(test)]
use std::ops::DerefMut;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::{Arc, MutexGuard};

const WORDS: usize = 8;
const PAGE_SIZE: usize = WORDS * 64;
const PAGE_MASK: usize = PAGE_SIZE - 1;

// Group the eight possible low-bit patterns separately within a 4 KiB range.
// An aligned nursery uses one 64-byte bitmap. Unaligned byte allocations remain
// individually representable, without ever interpreting a scan key as a pointer.
fn position(address: usize) -> usize {
    (address & !(PAGE_SIZE * 8 - 1)) | ((address & 7) * PAGE_SIZE) | ((address >> 3) & PAGE_MASK)
}

fn address_of(key: usize) -> usize {
    (key & !(PAGE_SIZE * 8 - 1)) | ((key & PAGE_MASK) << 3) | ((key / PAGE_SIZE) & 7)
}

#[derive(Debug)]
struct Page {
    nonempty: AtomicU64,
    words: [AtomicU64; WORDS],
}

#[expect(
    clippy::indexing_slicing,
    reason = "a masked page offset or nonempty word bit selects one of WORDS words"
)]
impl Page {
    fn new() -> Self {
        Self {
            nonempty: AtomicU64::new(0),
            words: [const { AtomicU64::new(0) }; WORDS],
        }
    }

    fn contains(&self, key: usize) -> bool {
        let offset = key & PAGE_MASK;
        self.words[offset / 64].load(Acquire) & (1 << (offset % 64)) != 0
    }

    // Only the writer holding the matching object shard changes this page.
    fn set(&self, key: usize, present: bool) {
        let offset = key & PAGE_MASK;
        let index = offset / 64;
        let bit = 1 << (offset % 64);
        let word = &self.words[index];
        let previous = word.load(Acquire);
        let next = if present {
            previous | bit
        } else {
            previous & !bit
        };
        word.store(next, Release);
        if (previous == 0) != (next == 0) {
            let mask = self.nonempty.load(Acquire);
            let bit = 1 << index;
            self.nonempty
                .store(if next == 0 { mask & !bit } else { mask | bit }, Release);
        }
    }

    fn next(&self, start: usize) -> Option<usize> {
        let first = start / 64;
        let mut possible = self.nonempty.load(Acquire) & (u64::MAX << first);
        while possible != 0 {
            let index = possible.trailing_zeros() as usize;
            let mut bits = self.words[index].load(Acquire);
            if index == first {
                bits &= u64::MAX << (start % 64);
            }
            if bits != 0 {
                return Some(index * 64 + bits.trailing_zeros() as usize);
            }
            possible &= possible - 1;
        }
        None
    }
}

type Map = BTreeMap<usize, MetadataRef>;
type Pages = Directory<Arc<Page>>;

#[derive(Debug, Default)]
pub(crate) struct YoungCursor {
    position: Cursor,
    // A retained page contains numeric membership hints only. Probe a fresh
    // directory when it runs out, including when the page key was reused.
    page: Option<(usize, usize, Arc<Page>)>,
}

impl YoungCursor {
    pub(crate) fn retry_entry(&mut self) {
        self.position.retry_entry();
    }
}

#[derive(Debug, Default)]
pub(crate) struct ObjectMap {
    objects: Map,
    // The shard lock protects the writer's reference to the published directory.
    // Only a page appearing or disappearing needs to publish a new root.
    pages: Arc<Pages>,
    last: Option<(u64, Arc<Page>)>,
}

impl ObjectMap {
    // Match BTreeMap lookup syntax so all descriptor lookups share this check.
    #[expect(
        clippy::trivially_copy_pass_by_ref,
        reason = "the shard exposes the existing map lookup interface"
    )]
    pub(crate) fn get(&self, address: &usize) -> Option<&MetadataRef> {
        self.objects
            .get(address)
            .filter(|metadata| metadata.birth.visible())
    }

    pub(crate) fn len(&self) -> usize {
        self.objects.len()
    }

    pub(crate) fn live_values(&self) -> impl Iterator<Item = &MetadataRef> {
        self.objects
            .values()
            .filter(|metadata| metadata.birth.is_live())
    }
}

#[cfg(test)]
impl Deref for ObjectMap {
    type Target = Map;
    fn deref(&self) -> &Map {
        &self.objects
    }
}
#[cfg(test)]
impl DerefMut for ObjectMap {
    fn deref_mut(&mut self) -> &mut Map {
        &mut self.objects
    }
}

#[derive(Debug)]
pub(crate) struct Objects {
    descriptors: Shards<ObjectMap>,
    nursery: Box<[ArcSwap<Pages>]>,
}

impl Deref for Objects {
    type Target = Shards<ObjectMap>;
    fn deref(&self) -> &Self::Target {
        &self.descriptors
    }
}

/// An exact address write permit. There is no independently held young writer.
pub(crate) struct Membership<'a> {
    objects: MutexGuard<'a, ObjectMap>,
    pages: &'a ArcSwap<Pages>,
    key: usize,
}

#[cfg(test)]
impl Deref for Membership<'_> {
    type Target = Map;
    fn deref(&self) -> &Self::Target {
        &self.objects
    }
}
impl Membership<'_> {
    pub(crate) fn is_young(&self) -> bool {
        let base = (self.key & !PAGE_MASK) as u64;
        self.objects
            .last
            .as_ref()
            .filter(|(key, _)| *key == base)
            .map(|(_, page)| page)
            .or_else(|| self.objects.pages.get(base))
            .is_some_and(|p| p.contains(self.key))
    }

    pub(crate) fn split(&mut self) -> (&mut Map, YoungWriter<'_>) {
        let ObjectMap {
            objects,
            pages,
            last,
        } = &mut *self.objects;
        (
            objects,
            YoungWriter {
                current: pages,
                last,
                published: self.pages,
                key: self.key,
            },
        )
    }
}

pub(crate) struct YoungWriter<'a> {
    current: &'a mut Arc<Pages>,
    last: &'a mut Option<(u64, Arc<Page>)>,
    published: &'a ArcSwap<Pages>,
    key: usize,
}

impl YoungWriter<'_> {
    fn replace(&mut self, pages: Pages) {
        let pages = Arc::new(pages);
        self.published.store(pages.clone());
        *self.current = pages;
    }

    #[expect(
        clippy::expect_used,
        reason = "page keys cannot exhaust usize address-space capacity"
    )]
    pub(crate) fn insert(&mut self) {
        let base = (self.key & !PAGE_MASK) as u64;
        if let Some((key, page)) = &*self.last
            && *key == base
        {
            page.set(self.key, true);
            return;
        }
        let page = if let Some(page) = self.current.get(base) {
            page.set(self.key, true);
            page.clone()
        } else {
            let page = Arc::new(Page::new());
            page.set(self.key, true);
            let (updated, _) = self
                .current
                .insert(base, page.clone())
                .expect("address page capacity");
            self.replace(updated);
            page
        };
        *self.last = Some((base, page));
    }

    pub(crate) fn remove(&mut self) {
        let base = (self.key & !PAGE_MASK) as u64;
        if self.last.as_ref().is_none_or(|(key, _)| *key != base) {
            *self.last = self.current.get(base).map(|p| (base, p.clone()));
        }
        let Some((_, page)) = self.last.as_ref() else {
            return;
        };
        page.set(self.key, false);
        if page.nonempty.load(Acquire) == 0 {
            *self.last = None;
            self.replace(self.current.remove(base).0);
        }
    }
}

#[expect(
    clippy::indexing_slicing,
    reason = "normalized shard counts bound cursor and bucket indices"
)]
impl Objects {
    pub(crate) fn new(count: usize) -> Self {
        Self {
            descriptors: Shards::new(count),
            nursery: (0..shard_count(count))
                .map(|_| ArcSwap::from_pointee(Pages::default()))
                .collect(),
        }
    }

    pub(crate) fn scan<R>(
        &self,
        cursor: &mut Cursor,
        read: impl FnOnce(&MetadataRef) -> R,
    ) -> Scan<R> {
        use std::ops::Bound::{Excluded, Unbounded};
        match self.descriptors.scan_with(cursor, |map, after| {
            map.objects
                .range((Excluded(after), Unbounded))
                .next()
                .map(|(&k, v)| (k, v.birth.visible().then(|| read(v))))
        }) {
            Scan::Entry(address, Some(value)) => Scan::Entry(address, value),
            Scan::Entry(_, None) | Scan::Advance => Scan::Advance,
            Scan::Busy => Scan::Busy,
            Scan::Done => Scan::Done,
        }
    }

    pub(crate) fn try_membership(&self, address: usize) -> Option<Membership<'_>> {
        let bucket = address_bucket(address);
        Some(Membership {
            objects: self.descriptors.try_lock_bucket(bucket)?,
            pages: &self.nursery[bucket & (self.nursery.len() - 1)],
            key: position(address),
        })
    }

    pub(crate) fn scan_young(&self, cursor: &mut YoungCursor) -> Scan<()> {
        self.scan_young_with(cursor, || {})
    }

    pub(crate) fn scan_young_with(
        &self,
        young: &mut YoungCursor,
        observed: impl FnOnce(),
    ) -> Scan<()> {
        let cursor = &mut young.position;
        if !cursor.begin(self.nursery.len()) {
            return Scan::Done;
        }
        let Some(start) = cursor.addresses[cursor.shard].checked_add(1) else {
            return cursor.exhausted();
        };
        let base = start & !PAGE_MASK;
        if let Some((shard, key, page)) = &young.page
            && *shard == cursor.shard
            && *key == base
            && let Some(offset) = page.next(start & PAGE_MASK)
        {
            let key = base | offset;
            cursor.selected(key);
            return Scan::Entry(address_of(key), ());
        }
        let pages = self.nursery[cursor.shard].load();
        observed();
        if let Some(page) = pages.get(base as u64)
            && let Some(offset) = page.next(start & PAGE_MASK)
        {
            young.page = Some((cursor.shard, base, page.clone()));
            let key = base | offset;
            cursor.selected(key);
            return Scan::Entry(address_of(key), ());
        }
        let Some((base, page)) = pages.next_after(Some(base as u64)) else {
            return cursor.exhausted();
        };
        #[expect(
            clippy::cast_possible_truncation,
            reason = "keys are zero-extended usize addresses"
        )]
        let base = base as usize;
        if let Some(offset) = page.next(0) {
            young.page = Some((cursor.shard, base, page.clone()));
            let key = base | offset;
            cursor.selected(key);
            Scan::Entry(address_of(key), ())
        } else {
            // A concurrent writer emptied a page in this view. Charge this page
            // to the current unit and resume at the next one.
            cursor.addresses[cursor.shard] = base | PAGE_MASK;
            Scan::Advance
        }
    }

    pub(crate) fn drain(&self) -> Vec<MetadataRef> {
        self.descriptors
            .iter()
            .zip(self.nursery.iter())
            .flat_map(|(mut objects, pages)| {
                pages.store(Arc::new(Pages::default()));
                let old = std::mem::take(&mut *objects);
                old.objects.into_values()
            })
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn contains_young(&self, address: usize) -> bool {
        self.nursery[address_bucket(address) & (self.nursery.len() - 1)]
            .load()
            .get((position(address) & !PAGE_MASK) as u64)
            .is_some_and(|p| p.contains(position(address)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn update(objects: &Objects, address: usize, present: bool) {
        let mut membership = objects.try_membership(address).expect("exclusive fixture");
        if present {
            membership.split().1.insert();
        } else {
            membership.split().1.remove();
        }
    }

    #[test]
    fn numeric_scan_keys_round_trip_every_alignment() {
        let span = PAGE_SIZE * 8;
        for base in [0, span, usize::MAX & !(span - 1)] {
            for offset in 0..span {
                let address = base | offset;
                assert_eq!(address_of(position(address)), address);
                assert_eq!(position(address_of(address)), address);
            }
        }
    }

    #[test]
    fn stale_scan_cannot_republish_a_retired_bitmap_page() {
        let objects = Objects::new(1);
        update(&objects, 8, true);
        let mut cursor = YoungCursor::default();
        // One reader owns the old directory. Retire its last bit and recreate
        // the very same page key before allowing it to inspect its bitmap.
        let old = objects.scan_young_with(&mut cursor, || {
            update(&objects, 8, false);
            update(&objects, 16, true);
        });
        assert!(matches!(old, Scan::Done | Scan::Advance));
        assert!(!objects.contains_young(8));
        assert!(objects.contains_young(16));
        let mut fresh = YoungCursor::default();
        assert!(matches!(
            objects.scan_young(&mut fresh),
            Scan::Entry(16, ())
        ));
        assert!(matches!(objects.scan_young(&mut fresh), Scan::Done));
        update(&objects, 16, false);
        assert!(objects.nursery.iter().all(|slot| slot.load().len() == 0));
    }

    #[test]
    fn a_cached_nursery_page_refreshes_after_unlink_and_cursor_retry() {
        let objects = Objects::new(1);
        let mut cursor = YoungCursor::default();
        update(&objects, 8, true);
        assert!(matches!(
            objects.scan_young(&mut cursor),
            Scan::Entry(8, ())
        ));
        update(&objects, 8, false);
        update(&objects, 16, true);
        cursor.retry_entry();
        assert!(matches!(
            objects.scan_young(&mut cursor),
            Scan::Entry(16, ())
        ));
        update(&objects, 24, true);
        assert!(matches!(
            objects.scan_young(&mut cursor),
            Scan::Entry(24, ())
        ));
        assert!(matches!(objects.scan_young(&mut cursor), Scan::Done));
    }

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn a_peer_observes_replacement_page_and_new_word_through_publication() {
        use std::thread;
        use std::time::{Duration, Instant};

        let objects = Objects::new(1);
        let mut cursor = YoungCursor::default();
        update(&objects, 8, true);
        assert!(matches!(
            objects.scan_young(&mut cursor),
            Scan::Entry(8, ())
        ));
        let slot = objects.nursery.first().expect("one shard");
        let original = slot.load().get(0).expect("first page").clone();
        thread::scope(|scope| {
            scope.spawn(|| {
                update(&objects, 8, false);
                update(&objects, 16, true);
                // Same page, next word. Only the first insertion swaps the root.
                update(&objects, 512, true);
            });
            let deadline = Instant::now() + Duration::from_secs(5);
            // The reader never joins the writer or receives a ready signal.
            // Observe the new directory and the new word's summary directly.
            let page = loop {
                if let Some(p) = slot.load().get(0)
                    && !Arc::ptr_eq(&original, p)
                {
                    break p.clone();
                }
                assert!(Instant::now() < deadline, "replacement was not published");
                thread::yield_now();
            };
            assert!(page.contains(position(16)));
            while page.nonempty.load(Acquire) & 2 == 0 {
                assert!(Instant::now() < deadline, "word was not published");
                thread::yield_now();
            }
            cursor.retry_entry();
            assert!(matches!(
                objects.scan_young(&mut cursor),
                Scan::Entry(16, ())
            ));
            assert!(matches!(
                objects.scan_young(&mut cursor),
                Scan::Entry(512, ())
            ));
            assert!(matches!(objects.scan_young(&mut cursor), Scan::Done));
        });
    }

    #[test]
    fn nursery_cursor_preserves_word_page_and_address_space_boundaries() {
        for shards in [1, 2, 16] {
            let objects = Objects::new(shards);
            let keys = [
                1,
                2,
                63,
                64,
                65,
                PAGE_SIZE - 1,
                PAGE_SIZE,
                PAGE_SIZE + 64,
                2 * PAGE_SIZE - 1,
                4 * PAGE_SIZE + 1,
                usize::MAX - 1,
                usize::MAX,
            ];
            for key in keys {
                let mut held = objects.try_membership(key).expect("exclusive fixture");
                assert!(!held.is_young());
                held.split().1.insert();
                assert!(held.is_young());
            }
            let mut cursor = YoungCursor::default();
            let mut seen = BTreeSet::new();
            loop {
                match objects.scan_young(&mut cursor) {
                    Scan::Entry(key, ()) => {
                        assert!(seen.insert(key));
                        objects
                            .try_membership(key)
                            .expect("exclusive fixture")
                            .split()
                            .1
                            .remove();
                    }
                    Scan::Advance => {}
                    Scan::Done => break,
                    Scan::Busy => panic!("nursery directory required an index lock"),
                }
            }
            assert_eq!(seen, keys.into_iter().collect());
            assert!(objects.nursery.iter().all(|slot| slot.load().len() == 0));
        }
    }
}
