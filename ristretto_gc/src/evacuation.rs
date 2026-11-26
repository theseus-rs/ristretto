//! Bounded region selection. All methods run under the region registry mutex.
//! Destination roles prevent evacuation from cycling within a selection epoch.
use super::{AtomicU64, AtomicUsize, BTreeMap, Ordering, Region};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::Arc;

const SOURCE: u64 = 1;
const DESTINATION: u64 = 2;
const SCALE: usize = 1024;
const MAX_COST: usize = SCALE * 2 / 3;
// Preserve ordinary allocation holes while a bounded evacuation set is active.
pub(super) const MAX_SOURCES: usize = 4;

// Role tags occupy the low two bits of each region's epoch. The heap word
// uses bit zero for activation and retains its high bits between cycles.
#[derive(Debug, Default)]
pub(super) struct Selection {
    generation: AtomicU64,
    pub(super) remaining: AtomicUsize,
}

impl Selection {
    // Only the collection domain that owns relocation starts or ends a selection.
    pub(super) fn begin(&self) -> bool {
        let previous = self.generation.load(Ordering::Relaxed) & !3;
        let Some(next) = previous.checked_add(4) else {
            self.end();
            return false;
        };
        self.remaining.store(MAX_SOURCES, Ordering::Relaxed);
        self.generation.store(next | 1, Ordering::Release);
        true
    }

    pub(super) fn end(&self) {
        self.generation.fetch_and(!1, Ordering::Release);
    }

    pub(super) fn epoch(&self) -> Option<u64> {
        let word = self.generation.load(Ordering::Acquire);
        (word & 1 != 0).then_some(word & !3)
    }
}

#[derive(Debug)]
pub(super) struct Evacuation {
    pub(super) selection: Arc<Selection>,
    prepared: u64,
    candidates: BinaryHeap<Reverse<(usize, u64)>>,
    building: bool,
    cursor: u64,
    pub(super) selected_regions: u64,
    pub(super) selected_bytes: u64,
    pub(super) selection_skips: u64,
    pub(super) pin_skips: u64,
    pub(super) hot_skips: u64,
}

fn cost(region: &Region, occupied: usize) -> usize {
    // Quantized occupancy estimates copy cost per byte of backing. The wide
    // multiply handles valid layouts near the address-space limit without wrap.
    ((occupied as u128 * SCALE as u128) / region.layout.size() as u128) as usize
}

impl Evacuation {
    pub(super) fn new(selection: Arc<Selection>) -> Self {
        Self {
            selection,
            prepared: 0,
            candidates: BinaryHeap::new(),
            building: false,
            cursor: 0,
            selected_regions: 0,
            selected_bytes: 0,
            selection_skips: 0,
            pin_skips: 0,
            hot_skips: 0,
        }
    }

    pub(super) fn building(&self) -> bool {
        self.building
    }

    pub(super) fn next_cost(&self) -> Option<usize> {
        self.candidates.peek().map(|Reverse((cost, _))| *cost)
    }

    pub(super) fn capacity_bytes(&self) -> usize {
        self.candidates.capacity() * size_of::<Reverse<(usize, u64)>>()
    }

    /// One registry entry or candidate per unit. Build the cost index on the
    /// collector, avoiding a sorted-tree update on every mutator allocation/free.
    /// Recheck roles and occupancy because this snapshot is only advisory.
    pub(super) fn select(&mut self, regions: &BTreeMap<u64, Arc<Region>>, epoch: u64) -> bool {
        if self.prepared != epoch {
            // The heap publishes role identity before any shard is visited.
            // Only collector-owned scratch is initialized lazily here.
            self.candidates.clear();
            self.cursor = 0;
            self.building = true;
            self.prepared = epoch;
        }
        if self.building {
            let Some((&id, region)) = regions.range((Excluded(self.cursor), Unbounded)).next()
            else {
                self.building = false;
                return false;
            };
            self.cursor = id;
            let score = cost(region, region.occupied());
            if score <= MAX_COST && !self.is_destination(region) {
                if self.candidates.try_reserve(1).is_err() {
                    // No sources have been selected yet; safely use legacy packing.
                    return true;
                }
                self.candidates.push(Reverse((score, id)));
            }
            return false;
        }
        if self.selection.remaining.load(Ordering::Acquire) == 0 {
            return true;
        }
        let Some(Reverse((_, id))) = self.candidates.pop() else {
            return true;
        };
        let Some(region) = regions.get(&id) else {
            self.selection_skips += 1;
            return false;
        };
        let occupied = region.occupied();
        if occupied == 0 || cost(region, occupied) > MAX_COST || self.is_destination(region) {
            self.selection_skips += 1;
            return false;
        }
        let mutations = region.mutations.load(Ordering::Relaxed);
        let previous = region.sampled_mutations.swap(mutations, Ordering::Relaxed);
        let pinned = region.pin_epoch.load(Ordering::Relaxed);
        if pinned != 0 && pinned == epoch - 4 {
            self.pin_skips += 1;
            self.destination(region);
            return false;
        }
        if mutations.wrapping_sub(previous) > 8 {
            self.hot_skips += 1;
            self.destination(region);
            return false;
        }
        if self
            .selection
            .remaining
            .try_update(Ordering::AcqRel, Ordering::Acquire, |remaining| {
                remaining.checked_sub(1)
            })
            .is_err()
        {
            return true;
        }
        region
            .relocation_role
            .store(epoch | SOURCE, Ordering::Relaxed);
        self.selected_regions += 1;
        self.selected_bytes += occupied as u64;
        false
    }

    pub(super) fn source(&self, region: &Region) -> bool {
        self.selection
            .epoch()
            .is_some_and(|epoch| Self::source_at(region, epoch))
    }

    pub(super) fn source_at(region: &Region, epoch: u64) -> bool {
        region.relocation_role.load(Ordering::Relaxed) == epoch | SOURCE
    }

    pub(super) fn destination(&self, region: &Region) {
        if let Some(epoch) = self.selection.epoch() {
            region
                .relocation_role
                .store(epoch | DESTINATION, Ordering::Relaxed);
        }
    }

    pub(super) fn is_destination(&self, region: &Region) -> bool {
        self.selection
            .epoch()
            .is_some_and(|epoch| Self::destination_at(region, epoch))
    }

    pub(super) fn destination_at(region: &Region, epoch: u64) -> bool {
        region.relocation_role.load(Ordering::Relaxed) == epoch | DESTINATION
    }

    pub(super) fn pin(&self, region: &Region) {
        if let Some(epoch) = self.selection.epoch() {
            region.pin_epoch.store(epoch, Ordering::Relaxed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exhausted_epoch_never_reuses_a_source_or_destination_role() {
        let selection = Selection {
            generation: AtomicU64::new(u64::MAX - 7),
            ..Selection::default()
        };
        assert!(selection.begin());
        assert_eq!(selection.epoch(), Some(u64::MAX - 3));
        assert!(!selection.begin());
        assert_eq!(selection.epoch(), None);
        assert!(!selection.begin());
    }
}
