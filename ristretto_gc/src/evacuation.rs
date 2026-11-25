//! Bounded region selection. All methods run under the region registry mutex.
//! Destination roles prevent evacuation from cycling within a selection epoch.
use super::{BTreeMap, Ordering, Region};
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

#[derive(Debug, Default)]
pub(super) struct Evacuation {
    pub(super) active: bool,
    quota: Option<Arc<super::AtomicUsize>>,
    epoch: u64,
    candidates: BinaryHeap<Reverse<(usize, u64)>>,
    building: bool,
    cursor: u64,
    sources: usize,
    pub(super) selected_regions: u64,
    pub(super) selected_bytes: u64,
    pub(super) selection_skips: u64,
    pub(super) released_regions: u64,
    pub(super) released_bytes: u64,
    pub(super) pin_skips: u64,
    pub(super) hot_skips: u64,
}

fn cost(region: &Region, occupied: usize) -> usize {
    // Quantized occupancy estimates copy cost per byte of backing. The wide
    // multiply handles valid layouts near the address-space limit without wrap.
    ((occupied as u128 * SCALE as u128) / region.layout.size() as u128) as usize
}

impl Evacuation {
    pub(super) fn with_quota(quota: Arc<super::AtomicUsize>) -> Self {
        Self {
            quota: Some(quota),
            ..Self::default()
        }
    }

    pub(super) fn building(&self) -> bool {
        self.building
    }

    pub(super) fn next_cost(&self) -> Option<usize> {
        self.candidates.peek().map(|Reverse((cost, _))| *cost)
    }

    pub(super) fn begin(&mut self) -> bool {
        self.active = false;
        let Some(epoch) = self.epoch.checked_add(4) else {
            // Never reuse an epoch: falling back to legacy packing is safe.
            return false;
        };
        self.epoch = epoch;
        // Entries contain no owned resources; clearing retains the vector capacity.
        self.candidates.clear();
        self.cursor = 0;
        self.building = true;
        self.sources = 0;
        self.active = true;
        true
    }

    pub(super) fn capacity_bytes(&self) -> usize {
        self.candidates.capacity() * size_of::<Reverse<(usize, u64)>>()
    }

    /// One registry entry or candidate per unit. Build the cost index on the
    /// collector, avoiding a sorted-tree update on every mutator allocation/free.
    /// Recheck roles and occupancy because this snapshot is only advisory.
    pub(super) fn select(&mut self, regions: &BTreeMap<u64, Arc<Region>>) -> bool {
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
                    self.active = false;
                    return true;
                }
                self.candidates.push(Reverse((score, id)));
            }
            return false;
        }
        if self.sources == MAX_SOURCES
            || self
                .quota
                .as_ref()
                .is_some_and(|quota| quota.load(Ordering::Acquire) == 0)
        {
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
        if pinned != 0 && pinned == self.epoch - 4 {
            self.pin_skips += 1;
            self.destination(region);
            return false;
        }
        if mutations.wrapping_sub(previous) > 8 {
            self.hot_skips += 1;
            self.destination(region);
            return false;
        }
        if let Some(quota) = &self.quota
            && quota
                .try_update(Ordering::AcqRel, Ordering::Acquire, |remaining| {
                    remaining.checked_sub(1)
                })
                .is_err()
        {
            return true;
        }
        region
            .relocation_role
            .store(self.epoch | SOURCE, Ordering::Relaxed);
        self.sources += 1;
        self.selected_regions += 1;
        self.selected_bytes += occupied as u64;
        false
    }

    pub(super) fn source(&self, region: &Region) -> bool {
        self.active && region.relocation_role.load(Ordering::Relaxed) == self.epoch | SOURCE
    }

    pub(super) fn destination(&self, region: &Region) {
        if self.active {
            region
                .relocation_role
                .store(self.epoch | DESTINATION, Ordering::Relaxed);
        }
    }

    pub(super) fn is_destination(&self, region: &Region) -> bool {
        self.active && region.relocation_role.load(Ordering::Relaxed) == (self.epoch | DESTINATION)
    }

    pub(super) fn pin(&self, region: &Region) {
        if self.active {
            region.pin_epoch.store(self.epoch, Ordering::Relaxed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exhausted_epoch_never_reuses_a_source_or_destination_role() {
        let mut plan = Evacuation {
            epoch: u64::MAX - 7,
            ..Evacuation::default()
        };
        assert!(plan.begin());
        assert_eq!(plan.epoch, u64::MAX - 3);
        assert!(!plan.begin());
        assert!(!plan.active);
        assert_eq!(plan.epoch, u64::MAX - 3);
        assert!(!plan.begin());
    }
}
