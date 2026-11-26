//! Resumable ordinary allocation. A cursor owns search keys and no physical storage.
use super::{Block, RegionHeap, try_lock};
use crate::{Error, Result};
use std::alloc::Layout;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::task::Poll;

#[derive(Default)]
pub(crate) struct AllocationProbe {
    first: Option<usize>,
    visited: usize,
    region: Option<(usize, u64)>,
    extent: Option<(usize, usize)>,
    busy: bool,
}

impl AllocationProbe {
    fn next_shard(&mut self) {
        self.visited += 1;
        self.region = None;
        self.extent = None;
    }

    /// Probe at most 32 candidates per call. A parked or fragmented region does
    /// not prevent allocation from another shard or from fresh backing.
    pub(crate) fn poll(&mut self, heap: &RegionHeap, layout: Layout) -> Poll<Result<Block>> {
        let first = *self.first.get_or_insert_with(|| heap.preferred_shard());
        for _ in 0..32 {
            if self.visited == heap.states.len() {
                let busy = self.busy;
                *self = Self::default();
                return Self::open_backing(heap, layout, first, busy);
            }
            let index = (first + self.visited) & (heap.states.len() - 1);
            let Some(mut state) = try_lock(heap.shard(index)) else {
                self.busy = true;
                self.next_shard();
                continue;
            };
            let key = match (self.region, self.extent) {
                (Some(key), Some(_)) if state.available.contains(&key) => key,
                _ => {
                    self.extent = None;
                    let lower = self.region.map_or(Included((layout.size(), 0)), Excluded);
                    let Some(key) = state.available.range((lower, Unbounded)).next().copied()
                    else {
                        self.next_shard();
                        continue;
                    };
                    self.region = Some(key);
                    key
                }
            };
            let previous_extent = self.extent.take();
            let Some(region) = state.regions.get(&key.1) else {
                continue;
            };
            if state
                .evacuation
                .as_ref()
                .is_some_and(|plan| plan.source(region))
            {
                continue;
            }
            let Some(mut free) = try_lock(&region.free) else {
                self.busy = true;
                continue;
            };
            let lower = previous_extent.map_or(Included((layout.size(), 0)), Excluded);
            let Some(&(length, start)) = free.by_size.range((lower, Unbounded)).next() else {
                continue;
            };
            let Some(at) = region.aligned_offset(layout, start, length) else {
                self.extent = Some((length, start));
                continue;
            };
            let block = region.take_range(&mut free, layout, start, length, at);
            let remaining = free.by_size.last().map_or(0, |(size, _)| *size);
            drop(free);
            state.record_allocation(&block, key.0, remaining);
            *self = Self::default();
            return Poll::Ready(Ok(block));
        }
        Poll::Pending
    }

    fn open_backing(
        heap: &RegionHeap,
        layout: Layout,
        first: usize,
        mut busy: bool,
    ) -> Poll<Result<Block>> {
        for offset in 0..heap.states.len() {
            let index = (first + offset) & (heap.states.len() - 1);
            let Some(state) = try_lock(heap.shard(index)) else {
                busy = true;
                continue;
            };
            if let Ok(fresh) = heap.prepare_region(layout, false, index, state) {
                #[cfg(all(test, not(target_family = "wasm")))]
                super::shard_tests::before_probe_publication();
                let Some(mut state) = try_lock(heap.shard(index)) else {
                    // No Block exists yet. Drop rolls back the unpublished backing
                    // and both credits without waiting for the parked registry.
                    return Poll::Pending;
                };
                return Poll::Ready(fresh.publish(layout, &mut state));
            }
        }
        if busy {
            Poll::Pending
        } else {
            Poll::Ready(Err(Error::OutOfMemory))
        }
    }
}
