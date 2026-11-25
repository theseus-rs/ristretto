//! A safe direct `Gc::trace` call has no mark-task reservation. Publish its
//! traversal to the owning epoch so suspending the caller cannot pin an epoch
//! or let it modify a later epoch's mark word.
use super::{
    GarbageCollector, MetadataRef, ObjectMetadata, Ordering, Retention, SCAN_YOUNG,
    TRACE_RETENTION, publication_state,
};
use std::sync::Arc;

impl GarbageCollector {
    pub(super) fn visible_mark(&self, metadata: &ObjectMetadata, index: usize, epoch: u64) -> bool {
        metadata.marked(index, epoch)
            || metadata
                .direct_marks
                .get(index, epoch, &self.domain_at(index).mark_queue)
                .is_some()
    }

    pub(super) fn visible_retention(
        &self,
        metadata: &ObjectMetadata,
        index: usize,
        epoch: u64,
    ) -> Option<Retention> {
        let physical = metadata.retention(index, epoch);
        if physical == Some(Retention::Strong) {
            return physical;
        }
        match metadata
            .direct_marks
            .get(index, epoch, &self.domain_at(index).mark_queue)
        {
            Some(committed) => Some(physical.map_or(committed, |v| v.min(committed))),
            None => physical,
        }
    }

    pub(super) fn trace_needs_epoch_proposal(&self) -> bool {
        // Old handoff uses per-edge cross-domain admission and has its own
        // semantics for a young birth. Reserved callbacks already pin closure.
        self.configuration.old_collection_interval == 0
            && !TRACE_RETENTION.get().is_some_and(|(gc, _, reserved)| {
                reserved && gc == std::ptr::from_ref(self) as usize
            })
    }

    pub(super) fn publish_external_trace(&self, metadata: &MetadataRef) {
        let source = self.domain_index();
        let mature = self.configuration.promotion_age.max(1);
        let inherited = TRACE_RETENTION
            .get()
            .filter(|(heap, _, _)| *heap == std::ptr::from_ref(self) as usize)
            .map_or(Retention::Strong, |(_, strength, _)| strength);

        loop {
            let snapshot = match self.phase_clock.indexed_root() {
                publication_state::Birth::Idle => return,
                publication_state::Birth::Inline(snapshot)
                | publication_state::Birth::Epochs(snapshot) => snapshot,
            };
            let Some(source_stage) = snapshot.stage(source) else {
                return;
            };
            if (0..2).all(|i| snapshot.stage(i).is_none_or(|stage| stage >= 5))
                || !metadata.birth.is_live()
            {
                return;
            }
            let young = metadata.age.load(Ordering::Acquire) < mature;
            if young {
                SCAN_YOUNG.set(true);
            }
            if source == 0 && !young {
                return;
            }

            let epochs = [0, 1].map(|i| {
                snapshot
                    .stage(i)
                    .and_then(|_| self.domain_at(i).mark_queue.birth_epoch())
            });
            if epochs
                .iter()
                .enumerate()
                .any(|(i, epoch)| snapshot.stage(i).is_some() && epoch.is_none())
            {
                continue;
            }
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            super::publication_tests::pause_after_external_trace_epoch();
            let dead = self.configuration.overlapping_generations
                && epochs.iter().enumerate().any(|(i, epoch)| {
                    snapshot.stage(i) == Some(5)
                        && (i == 1 || young)
                        && epoch
                            .as_ref()
                            .is_some_and(|e| !metadata.marked(i, e.number))
                });
            let mut intent = None;
            let mut strengths = [None; 2];
            if !dead {
                for (i, (epoch, proposed)) in epochs.iter().zip(&mut strengths).enumerate() {
                    let (Some(stage), Some(epoch)) = (snapshot.stage(i), epoch) else {
                        continue;
                    };
                    if stage >= 5 || (i == 0 && !young) {
                        continue;
                    }
                    let strength = inherited
                        .max(Retention::at_phase(source_stage))
                        .max(Retention::at_phase(stage));
                    if self
                        .visible_retention(metadata, i, epoch.number)
                        .is_some_and(|current| current <= strength)
                    {
                        continue;
                    }
                    *proposed = Some(strength);
                    let record = intent.get_or_insert_with(|| {
                        Arc::new(crate::roots::Record::typed(metadata.ptr().0))
                    });
                    self.domain_at(i).mark_queue.propose_root(
                        epoch,
                        crate::work::RootProposal {
                            metadata: metadata.clone(),
                            record: record.clone(),
                            index: i,
                            strength,
                            promotion_age: mature,
                        },
                    );
                }
            }
            if self.phase_clock.certify_epochs(snapshot, intent.is_some())
                && epochs.iter().enumerate().all(|(i, epoch)| {
                    epoch
                        .as_ref()
                        .is_none_or(|epoch| self.domain_at(i).mark_queue.current_birth_epoch(epoch))
                })
                && intent.is_none_or(|record| record.commit_typed())
            {
                self.expose_committed_trace(metadata, &epochs, strengths);
                return;
            }
        }
    }
    fn expose_committed_trace(
        &self,
        metadata: &MetadataRef,
        epochs: &[Option<Arc<crate::work::BirthEpoch>>; 2],
        strengths: [Option<Retention>; 2],
    ) {
        for (i, (strength, epoch)) in strengths.into_iter().zip(epochs).enumerate() {
            if let (Some(strength), Some(epoch)) = (strength, epoch) {
                metadata
                    .direct_marks
                    .publish(i, epoch, strength, &self.domain_at(i).mark_queue);
            }
        }
    }
}
