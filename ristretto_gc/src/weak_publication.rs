//! An ownerless non-retaining reference has no graph or owner to publish.
//! A scanner that passes its pending entry vetoes its attempt to become live.
use super::{GarbageCollector, Ordering, publication_state::Birth};
use crate::ReferenceKind;
use crate::metadata::BirthVisibility;
use crate::weak::WeakSlot;
use std::sync::Arc;

impl GarbageCollector {
    pub(crate) fn can_publish_host_weak(&self, kind: ReferenceKind, owner: usize) -> bool {
        owner == 0
            && matches!(kind, ReferenceKind::Weak | ReferenceKind::Soft)
            && self.configuration.old_collection_interval == 0
    }

    pub(crate) fn publish_host_weak(
        &self,
        slot: &Arc<WeakSlot>,
        visibility: &BirthVisibility,
        source: Option<&WeakSlot>,
    ) {
        visibility.index();
        // Age only increases for this exact allocation. A stale young sample
        // may require later pruning, while a mature sample can safely omit it.
        let young = self
            .weak_target_metadata(slot, slot.target.load(Ordering::Acquire))
            .is_some_and(|metadata| self.is_young_metadata(&metadata));
        self.weak_slots.insert(&self.next_weak_id, slot, young);
        loop {
            let phases = self.phase_clock.indexed_root();
            let valid = match phases {
                Birth::Idle => {
                    if let Some(source) = source {
                        // The clock has deactivated both domains. Legacy phase mirrors
                        // can briefly still describe the previous retirement boundary.
                        let address = source.target.load(Ordering::Acquire);
                        let live = self.weak_target_metadata(source, address).is_some();
                        slot.target
                            .store(if live { address } else { 0 }, Ordering::Relaxed);
                    }
                    for stamp in &slot.born_after {
                        stamp.store(0, Ordering::Relaxed);
                    }
                    true
                }
                Birth::Inline(snapshot) | Birth::Epochs(snapshot) => {
                    let epochs = [0, 1].map(|i| {
                        snapshot
                            .stage(i)
                            .and_then(|_| self.domain_at(i).mark_queue.birth_epoch())
                    });
                    // Sample the source inside the interval certified by these owning epochs.
                    let address = source.map_or(0, |s| s.target.load(Ordering::Acquire));
                    let metadata = source.and_then(|s| self.weak_target_metadata(s, address));
                    let mut live = metadata.is_some();
                    let complete = epochs
                        .iter()
                        .enumerate()
                        .all(|(i, epoch)| epoch.is_some() || snapshot.stage(i).is_none());
                    for (i, epoch) in epochs.iter().enumerate() {
                        let closed = snapshot.stage(i).is_some_and(|stage| stage != 0);
                        let number = epoch.as_ref().filter(|_| closed).map_or(0, |e| e.number);
                        let exempt = source
                            .is_none_or(|s| s.birth_stamp(i).load(Ordering::Acquire) == number);
                        if number != 0 && !exempt {
                            live &= metadata.as_ref().is_some_and(|m| {
                                self.reference_metadata_live_at(m, slot.kind, i, number)
                            });
                        }
                        slot.birth_stamp(i)
                            .store(if exempt { number } else { 0 }, Ordering::Relaxed);
                    }
                    if source.is_some() {
                        slot.target
                            .store(if live { address } else { 0 }, Ordering::Relaxed);
                    }
                    complete
                        && self.phase_clock.certify_epochs(snapshot, false)
                        && epochs.iter().enumerate().all(|(i, epoch)| {
                            epoch.as_ref().is_none_or(|epoch| {
                                self.domain_at(i).mark_queue.current_birth_epoch(epoch)
                            })
                        })
                }
            };
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            super::reference_tests::pause_before_host_weak_activation();
            // The source target is monotonic once public. A clear observed here
            // is permanent even when a Final reference has retained its object.
            if source.is_some_and(|s| s.target.load(Ordering::Acquire) == 0) {
                slot.target.store(0, Ordering::Relaxed);
            }
            if valid && visibility.publish_if_unobserved() {
                return;
            }
            visibility.retry();
        }
    }
}
