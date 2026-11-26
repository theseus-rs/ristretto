use super::{
    GarbageCollector, MarkItem, ObjectMetadata, Ordering, ReservedFullScan, Retention, SCAN_YOUNG,
    ScanYoung, TraceProgress, TraceRetention, WorkCounters,
};

impl GarbageCollector {
    /// Finish each claim directly from its queue item. No callback runs under
    /// a phase lock, and the existing reservation covers every publication.
    #[expect(
        clippy::too_many_lines,
        reason = "packet and scalar callbacks share a single claim and tracing budget"
    )]
    pub(super) fn trace_mark_batch(
        &self,
        claim: &mut crate::work::Claim,
        started: Option<std::time::Instant>,
        counters: &mut WorkCounters,
    ) -> bool {
        let index = self.domain_index();
        let domain = self.domain_at(index);
        let epoch = domain.epoch.load(Ordering::Acquire);
        let lane = self.local_queue();
        let ordinary = !self.configuration.overlapping_generations
            && self.configuration.old_collection_interval == 0;
        let full = (ordinary && index == 1).then_some(ReservedFullScan {
            epoch,
            mature: self.configuration.promotion_age.max(1),
        });
        let may_prune = self.remembered.may_prune();
        let mut executed = 0_u64;
        let mut deferred = 0_u64;
        let mut busy = false;
        let verified = full.is_some() && claim.strong_packet(epoch);
        let expired = || {
            self.configuration.max_pause_time_us != 0
                && started.is_some_and(|time| {
                    time.elapsed().as_micros() >= u128::from(self.configuration.max_pause_time_us)
                })
        };
        if verified
            && !may_prune
            && let Some((packet, last_budget)) = claim.take_packet()
        {
            let _young = ScanYoung::enter(false);
            let _strong = TraceRetention::callback(self, Retention::Strong);
            packet.for_each(last_budget, |metadata, budget| {
                if busy || deferred != 0 || (executed != 0 && expired()) {
                    deferred += 1;
                    domain
                        .mark_queue
                        .push(lane, MarkItem::fresh(metadata.clone()));
                    return;
                }
                // Verified packets contain only fresh work. Dirty state, offset and
                // remembered-version lookup become relevant only on a continuation.
                let progress =
                    ObjectMetadata::trace_ordinary(metadata, self, full.as_ref(), true, 0, budget);
                executed += 1;
                if progress == TraceProgress::Complete {
                    counters.marked += 1;
                } else {
                    let mut next = MarkItem::fresh(metadata.clone());
                    next.version = Some(usize::MAX);
                    next.has_young = SCAN_YOUNG.get();
                    if let TraceProgress::Continue(offset) = progress {
                        next.offset = offset;
                    }
                    busy = progress == TraceProgress::Busy;
                    domain.mark_queue.push(lane, next);
                }
            });
        } else {
            claim.for_each(|mut item, budget| {
                if busy || deferred != 0 || (executed != 0 && expired()) {
                    deferred += 1;
                    domain.mark_queue.push(lane, item.owned());
                    return;
                }
                debug_assert!(
                    index != 1
                        || !self.old_collection_active()
                        || item.metadata.age.load(Ordering::Acquire)
                            >= self.configuration.promotion_age.max(1)
                );
                if item.dirty_scan && item.version.is_none() {
                    item.metadata.begin_dirty_scan();
                }
                let version = item.version.unwrap_or_else(|| {
                    if may_prune {
                        item.metadata.mutation.load(Ordering::Acquire)
                    } else {
                        usize::MAX
                    }
                });
                let progress = if item.metadata.trace.step_fn.is_some() {
                    let _young = ScanYoung::enter(item.has_young);
                    let _retention = {
                        let strength = if verified {
                            Retention::Strong
                        } else {
                            self.visible_retention(&item.metadata, index, epoch)
                                .unwrap_or(Retention::Phantom)
                        };
                        TraceRetention::callback(self, strength)
                    };
                    let result = if ordinary {
                        ObjectMetadata::trace_ordinary(
                            &item.metadata,
                            self,
                            full.as_ref(),
                            verified,
                            item.offset,
                            budget,
                        )
                    } else {
                        ObjectMetadata::trace_step(&item.metadata, self, item.offset, budget)
                    };
                    item.has_young = SCAN_YOUNG.get();
                    result
                } else {
                    TraceProgress::Complete
                };
                executed += 1;
                if progress == TraceProgress::Complete {
                    counters.marked += 1;
                    if version != usize::MAX {
                        self.prune_remembered(
                            item.metadata.ptr().0,
                            item.metadata.id,
                            version,
                            item.has_young,
                        );
                    }
                    if item.dirty_scan && item.metadata.finish_dirty_scan() {
                        domain
                            .mark_queue
                            .push(lane, MarkItem::dirty(item.metadata.into_owned()));
                    }
                } else {
                    if let TraceProgress::Continue(offset) = progress {
                        item.offset = offset;
                    }
                    busy = progress == TraceProgress::Busy;
                    item.version = Some(version);
                    domain.mark_queue.push(lane, item.owned());
                }
            });
        }
        if self.diagnostics_enabled() {
            self.diagnostics
                .mark_batches
                .fetch_add(1, Ordering::Relaxed);
            self.diagnostics
                .trace_tasks
                .fetch_add(executed, Ordering::Relaxed);
            if index == 1 && self.old_collection_active() {
                self.diagnostics
                    .old_trace_tasks
                    .fetch_add(executed, Ordering::Relaxed);
            }
            if deferred != 0 {
                self.diagnostics
                    .trace_deferred
                    .fetch_add(deferred, Ordering::Relaxed);
            }
            if busy {
                self.diagnostics.trace_busy.fetch_add(1, Ordering::Relaxed);
            }
        }
        busy || deferred != 0
    }
}
