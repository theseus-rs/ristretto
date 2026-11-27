#[cfg(test)]
use super::lock;
use super::{
    Context, Domain, GarbageCollector, Gc, MarkItem, MetadataRef, ObjectMetadata, Ordering,
    Retention, SCAN_YOUNG, Trace, TraceRetention, WorkCounters, now, publication_state,
};

impl GarbageCollector {
    pub(super) fn trace_retention(&self) -> Retention {
        let inherited =
            TraceRetention::current(self).map_or(Retention::Strong, |(strength, _)| strength);
        inherited.max(Retention::at_phase(self.domain().weak_phase()))
    }

    pub(super) fn shade_metadata(&self, metadata: &MetadataRef) -> bool {
        let epoch = self.domain().epoch.load(Ordering::Acquire);
        let phase = self.domain().weak_phase();
        // Once retirement is admitted, white objects are logically dead. A
        // deletion barrier from clearing a dead parent must not revive a child
        // whose own graph may already have been swept. Fresh allocation has a
        // separate black-publication path; valid rooted handles are already live.
        if phase >= 5
            || (self.configuration.overlapping_generations && self.logically_retired(metadata))
        {
            return false;
        }
        if self.domain_index() == 1
            && self.old_collection_active()
            && metadata.age.load(Ordering::Acquire) < self.configuration.promotion_age.max(1)
        {
            // Initial young objects are handed off separately. Births are not
            // roots of this snapshot merely because their owning handles change.
            if !metadata.marked(1, epoch) || metadata.retention(1, epoch).is_some() {
                return metadata.mark(1, epoch, self.trace_retention());
            }
            return false;
        }
        if metadata.mark(self.domain_index(), epoch, self.trace_retention()) {
            self.domain()
                .mark_queue
                .push(self.local_queue(), MarkItem::fresh(metadata.clone()));
            return true;
        }
        false
    }

    pub(super) fn shade_all(&self, address: usize) -> bool {
        self.domains.iter().any(Domain::active)
            && self
                .objects
                .lookup(address)
                .is_some_and(|metadata| self.shade_live_domains(&metadata))
    }

    pub(super) fn shade_live_domains(&self, metadata: &MetadataRef) -> bool {
        // The caller holds publication admission through both domain barriers.
        let _retention = TraceRetention::enter(self, Retention::Strong);
        let mut changed = false;
        for index in 0..2 {
            let _context = Context::enter(self, index);
            changed |= self.shade_known(metadata);
        }
        changed
    }

    /// The phase unit orders its root read and shade against closure. Resolve
    /// only visible metadata; shading never borrows the target payload.
    pub(super) fn shade_registered_root(&self, address: usize) {
        if let Some(metadata) = self.objects.lookup(address) {
            let mature_full = self.domain_index() == 1
                && self.domain().weak_phase() == 0
                && !self.is_young_metadata(&metadata)
                && self.trace_retention() == Retention::Strong;
            if self.shade_known(&metadata) && mature_full {
                // A mature full root has no receiving-young publication.
                self.mature_census.strong.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    /// The caller protects registration/phase ordering; this path does not wait
    /// on the object index while admitted. Metadata owns no payload borrow.
    pub(super) fn shade_known(&self, metadata: &MetadataRef) -> bool {
        if !self.domain().active() {
            return false;
        }
        let young = metadata.age.load(Ordering::Acquire) < self.configuration.promotion_age.max(1);
        if young {
            SCAN_YOUNG.set(true);
        }
        if self.domain_index() == 0 && !young {
            return false;
        }
        let mut changed = self.shade_metadata(metadata);
        let other = self.domain_at(1 - self.domain_index());
        if other.active() {
            let _retention = TraceRetention::enter(self, self.trace_retention());
            let _context = Context::enter(self, 1 - self.domain_index());
            if self.domain_index() == 1 || young {
                changed |= self.shade_metadata(metadata);
            }
        }
        changed
    }

    pub(super) fn prune_remembered(
        &self,
        address: usize,
        identity: crate::ObjectId,
        version: usize,
        has_young: bool,
    ) {
        // Pruning is optional. Keep the entry if either shard is busy, so a
        // suspended shard owner cannot strand an in-flight mark task.
        if has_young || version == usize::MAX {
            return;
        }
        // A write after the scan began either has a newer directory version,
        // or will publish it after our CAS. Ignore a lost optional prune.
        self.remembered
            .dirty
            .try_remove(identity.to_i64().cast_unsigned(), version);
        if !self.remembered.may_contain(address) {
            return;
        }
        let Some(objects) = self.objects.try_lock(address) else {
            return;
        };
        // Verify identity and version under the remembered shard. A later
        // buffered update publishes a fresh insertion after this removal.
        if let Some(_writer) = self.remembered.try_lock(address)
            && objects.get(&address).is_some_and(|metadata| {
                metadata.id == identity && metadata.mutation.load(Ordering::Acquire) == version
            })
        {
            self.remembered.remove(address, identity);
        }
    }

    /// Whether the active cycle collects only young allocations.
    #[must_use]
    pub fn is_young_collection(&self) -> bool {
        self.domain_index() == 0
    }

    /// Queries generation metadata without dereferencing an allocation address.
    #[must_use]
    pub fn is_young_allocation(&self, address: usize) -> bool {
        self.objects
            .lookup(address)
            .is_some_and(|metadata| self.is_young_metadata(&metadata))
    }

    pub(super) fn is_young_metadata(&self, metadata: &ObjectMetadata) -> bool {
        metadata.age.load(Ordering::Acquire) < self.configuration.promotion_age.max(1)
    }

    /// Records mutation of an owner, preserving old-to-young edges across young collections.
    pub fn remember<T>(&self, owner: &Gc<T>) {
        self.remember_address(owner.ptr.as_ptr() as usize);
    }

    /// Shades a reference before it is overwritten. Barrier work is never discarded.
    pub fn write_barrier<T: Trace>(&self, target: &Gc<T>) {
        self.edge_barrier(target);
    }

    /// Certify a no-op against the packed phase/revision clock. Retention only
    /// strengthens within an epoch; all phase transitions stamp the revision.
    /// Metadata ownership permits the check to finish after storage retirement,
    /// but does not access payloads or imply that their graphs remain reachable.
    pub(super) fn edge_already_satisfied(
        &self,
        metadata: &MetadataRef,
        clock: &publication_state::Clock,
        snapshot: publication_state::Snapshot,
        record: crate::context::Record,
    ) -> bool {
        let satisfied = record.epochs.iter().enumerate().all(|(index, epoch)| {
            epoch.is_none()
                || (index == 0
                    && metadata.age.load(Ordering::Acquire)
                        >= self.configuration.promotion_age.max(1))
                || epoch.is_some_and(|epoch| {
                    metadata.retention(index, epoch) == Some(Retention::Strong)
                })
        });
        #[cfg(test)]
        if satisfied {
            let pause = lock(&self.edge_certificate_pause).take();
            if let Some(pause) = pause {
                pause.entered.send(()).expect("certificate checked");
                pause
                    .resume
                    .recv_timeout(std::time::Duration::from_secs(if cfg!(miri) {
                        120
                    } else {
                        10
                    }))
                    .expect("resume certificate");
            }
        }
        if satisfied && clock.unchanged(snapshot) {
            // Cache only after successful certification. Phase changes after this
            // point merely leave a stale epoch key; rollover disables the cache.
            let record = crate::context::Record {
                identity: metadata.id,
                ..record
            };
            crate::context::with_state(self, |state| state.completed(record));
            true
        } else {
            false
        }
    }

    pub(super) fn edge_barrier<T>(&self, target: &Gc<T>) {
        // Keep the common idle deletion out of descriptor resolution. Use the
        // same validated snapshot as the instrumented publication path.
        if !self.diagnostics_enabled()
            && self.phase_clock.snapshot().is_some_and(|snapshot| {
                (snapshot.flags(0) | snapshot.flags(1)) & 1 == 0
                    && self.phase_clock.unchanged(snapshot)
            })
        {
            return;
        }
        self.publish_edge(
            target.ptr.as_ptr().addr(),
            target.identity(),
            target.traced_metadata(),
            target.numeric_edge_for(self),
        );
    }

    pub(super) fn publish_edge(
        &self,
        address: usize,
        identity: crate::ObjectId,
        known: Option<&MetadataRef>,
        numeric_edge: bool,
    ) {
        let _measurement = self
            .diagnostics
            .barrier_record
            .measure(self.diagnostics_enabled());
        let mut certificate = None;
        // A traced edge owns its descriptor. A protected raw address is
        // resolved below while its caller still keeps the original birth live.
        // Numeric managed edges on old-enabled heaps go through exact identity
        // lookup before a committed old mark can be imported by a young scan.
        let numeric_handoff = numeric_edge && self.configuration.old_collection_interval != 0;
        let context_edge = (known.is_some() || identity.to_i64() == 0 || numeric_handoff)
            && self.configuration.barrier_buffer_size != 0
            && crate::context::with_state(self, |_| ()).is_some();
        let clock = self
            .publication_clock
            .as_ref()
            .or(context_edge.then_some(&self.phase_clock));
        let legacy = if let Some(clock) = clock {
            if let Some(snapshot) = clock.snapshot() {
                if (snapshot.flags(0) | snapshot.flags(1)) & 1 == 0 {
                    if clock.unchanged(snapshot) {
                        return;
                    }
                } else {
                    let record = crate::context::Record {
                        address,
                        identity,
                        dirty: false,
                        epochs: std::array::from_fn(|index| {
                            (snapshot.flags(index) & 1 != 0)
                                .then(|| self.domain_at(index).epoch.load(Ordering::Acquire))
                        }),
                    };
                    // Only a concrete birth can have a pending or completed record.
                    // Check it before consulting the address directory.
                    if crate::context::with_state(self, |state| {
                        state.satisfies(&self.contexts, record)
                    }) == Some(true)
                    {
                        if clock.unchanged(snapshot) {
                            return;
                        }
                    } else {
                        certificate = Some((clock, snapshot, record));
                    }
                }
            }
            None
        } else {
            let admission = self.allocation_guard();
            if self.domains.iter().all(|domain| !domain.active()) {
                return;
            }
            Some(admission)
        };
        // A traced edge already owns its original descriptor. An unsafe raw
        // wrapper instead pins its address through the caller's lifetime
        // contract; capture the real identity before any work is deferred.
        let resolved;
        let metadata = if let Some(metadata) = known {
            if !std::ptr::eq(metadata.collector.as_ptr(), self) {
                return;
            }
            metadata
        } else {
            let found = if identity.to_i64() == 0 {
                self.objects.lookup(address)
            } else {
                self.objects.lookup_identity(address, identity)
            };
            let Some(found) = found else { return };
            resolved = found;
            &resolved
        };
        if certificate.is_some_and(|(clock, snapshot, record)| {
            self.edge_already_satisfied(metadata, clock, snapshot, record)
        }) {
            return;
        }
        if context_edge {
            drop(legacy);
            self.shade_context_edge(metadata, numeric_handoff);
            return;
        }
        let publication = legacy.unwrap_or_else(|| self.barrier_guard());
        if self.barrier_inner(metadata, None) {
            publication.changed();
        }
    }

    fn shade_context_edge(&self, metadata: &MetadataRef, numeric_handoff: bool) {
        // The caller holds this exact same-heap birth descriptor. Move
        // its strong-phase traversal onto the epoch queues, where the
        // collector can claim a committed proposal or cancel a pending one.
        let limit = self
            .configuration
            .barrier_buffer_size
            .min(crate::context::MAX_RECORDS);
        for _ in 0..2 {
            let phases = self.phase_clock.indexed_root();
            match &phases {
                publication_state::Birth::Idle => return,
                publication_state::Birth::Inline(s) | publication_state::Birth::Epochs(s)
                    if (0..2).any(|i| s.stage(i).is_some_and(|stage| stage != 0)) =>
                {
                    break;
                }
                _ => {}
            }
            if !metadata.birth.is_live() {
                return;
            }
            let mut debt = None;
            let mut handoff = [None, None];
            let certified = self.certify_root_attempt(phases, metadata, |index, epoch| {
                if !epoch.has_root_capacity(limit) {
                    return None;
                }
                if numeric_handoff
                    && (index == 0 || epoch.old_only)
                    && self.is_young_metadata(metadata)
                {
                    *handoff.get_mut(index)? = Some(epoch.clone());
                }
                Some(
                    debt.get_or_insert_with(|| {
                        std::sync::Arc::new(crate::roots::Record::typed(metadata.trace.ptr.addr()))
                    })
                    .clone(),
                )
            });
            if certified != Some(true)
                || !metadata.birth.is_live()
                || debt.is_some_and(|r| !r.commit_typed())
            {
                break;
            }
            let published_old = handoff[1].is_some();
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            if published_old {
                super::publication_tests::pause_before_old_deletion_visibility();
            }
            for (i, epoch) in handoff.into_iter().enumerate() {
                if let Some(epoch) = epoch {
                    metadata.direct_marks.publish(
                        i,
                        &epoch,
                        Retention::Strong,
                        &self.domain_at(i).mark_queue,
                    );
                }
            }
            if !published_old {
                return;
            }
            // A young snapshot could already have passed this child between
            // committing the old traversal and exposing its retention. Order
            // the visible witness before a fresh receiver sample and cover
            // that young epoch as well. A later young start imports the mark.
        }
        // A phase change or cancelled proposal has no private debt. Use
        // fresh admission and strong retention for every receiving domain.
        let publication = self.barrier_guard();
        // An old-only pass needs visible retention before a young snapshot
        // can start. Apply the ordinary unbuffered handoff under admission.
        if metadata.birth.is_live() && self.shade_live_domains(metadata) {
            publication.changed();
        }
    }

    /// Whether the configured root or barrier policy uses execution contexts.
    #[must_use]
    pub fn execution_contexts_enabled(&self) -> bool {
        self.configuration.root_storage == crate::RootStorage::Contexts
            || self.configuration.barrier_buffer_size != 0
            || self.configuration.local_allocation_buffers
    }

    /// Attach a task-owned root and mutation context. Scoping is explicit; a
    /// suspended context never depends on the OS thread that last polled it.
    #[must_use]
    pub fn register_context(&self) -> crate::ExecutionContext {
        self.contexts.attach(self)
    }

    /// Called under the publication reader. Idle dirty notifications are immediate
    /// so a new young snapshot sees remembered owners even if the context sleeps.
    pub(super) fn barrier(&self, address: usize, owner: Option<(&MetadataRef, usize)>) -> bool {
        let _measurement = self
            .diagnostics
            .barrier_record
            .measure(self.diagnostics_enabled());
        if let Some((metadata, version)) = owner {
            return self.barrier_inner(metadata, Some(version));
        }
        self.domains.iter().any(Domain::active)
            && self
                .objects
                .lookup(address)
                .is_some_and(|metadata| self.barrier_inner(&metadata, None))
    }

    pub(super) fn barrier_inner(&self, metadata: &MetadataRef, version: Option<usize>) -> bool {
        if !metadata.birth.is_live() {
            return false;
        }
        if self.configuration.barrier_buffer_size == 0 {
            debug_assert!(
                version.is_none(),
                "an unbuffered owner is enrolled directly"
            );
            return self.shade_live_domains(metadata);
        }
        let owner = version.map(|version| (metadata, version));
        let epochs = std::array::from_fn(|index| {
            let domain = self.domain_at(index);
            domain
                .active()
                .then(|| domain.epoch.load(Ordering::Acquire))
        });
        let record = crate::context::Record {
            address: metadata.trace.ptr.addr(),
            identity: metadata.id,
            epochs,
            dirty: owner.is_some(),
        };
        // After either domain closes strong tracing, publish immediately. This
        // prevents a stream of already-black deletions from continually refilling
        // buffers ahead of reference processing and sweep.
        if self.domains.iter().all(|domain| domain.weak_phase() == 0)
            && epochs.iter().any(Option::is_some)
            && let Some(context) = crate::context::current(self)
        {
            let mut result = context.push(
                &self.contexts,
                record,
                self.configuration.barrier_buffer_size,
            );
            let overflowed = result == crate::context::Push::Full;
            if overflowed {
                if self.diagnostics_enabled() {
                    self.diagnostics
                        .barrier_overflows
                        .fetch_add(1, Ordering::Relaxed);
                }
                // Overflow is ordinary mutator work under admission. Required
                // records keep their reservations when any preparation lock is
                // busy; collector steps can drain them later. Explicit flush()
                // separately retains its synchronous completion contract.
                for _ in 0..crate::context::MAX_RECORDS {
                    if !self.try_apply_buffered_record(&context) {
                        break;
                    }
                }
                // A shared context may refill concurrently. Never spin on overflow.
                result = context.push(
                    &self.contexts,
                    record,
                    self.configuration.barrier_buffer_size,
                );
            }
            let changed = match result {
                crate::context::Push::Full | crate::context::Push::Busy => {
                    self.apply_direct_barrier(record, owner)
                }
                crate::context::Push::Added => {
                    if self.diagnostics_enabled() {
                        self.diagnostics
                            .barrier_buffered
                            .fetch_add(1, Ordering::Relaxed);
                    }
                    true
                }
                crate::context::Push::Coalesced => {
                    if self.diagnostics_enabled() {
                        self.diagnostics
                            .barrier_coalesced
                            .fetch_add(1, Ordering::Relaxed);
                    }
                    false
                }
            };
            // Overflow draining may itself have changed marks/queues. An exact
            // duplicate either owns the original pending reservation or has
            // already established Strong retention in every tagged epoch.
            changed || overflowed
        } else {
            self.apply_direct_barrier(record, owner)
        }
    }

    pub(super) fn apply_direct_barrier(
        &self,
        record: crate::context::Record,
        owner: Option<(&MetadataRef, usize)>,
    ) -> bool {
        let Some((metadata, version)) = owner else {
            return self.apply_barrier(record);
        };
        // The immediate store already advanced this exact owner's version. All
        // physical retirements revoke its birth while holding the same shard.
        if let Some(remembered) = self.remembered.try_lock(record.address) {
            if !metadata.birth.is_live() {
                return false;
            }
            remembered.insert(record.address, metadata.id);
            drop(remembered);
            self.rescan_dirty_owner(metadata);
        } else {
            self.remember_contended_owner(metadata, Some(version));
        }
        true
    }

    pub(super) fn buffered_target(&self, record: crate::context::Record) -> Option<MetadataRef> {
        self.objects
            .lookup_identity(record.address, record.identity)
    }

    pub(super) fn apply_barrier(&self, record: crate::context::Record) -> bool {
        let _retention = TraceRetention::enter(self, Retention::Strong);
        if record.dirty {
            let Some(metadata) = self.buffered_target(record) else {
                return false;
            };
            if self.logically_retired(&metadata) {
                return false;
            }
            if let Some(remembered) = self.remembered.try_lock(record.address) {
                remembered.insert(record.address, metadata.id);
            } else {
                self.remembered.dirty.publish(
                    metadata.id.to_i64().cast_unsigned(),
                    record.address,
                    metadata.mutation.load(Ordering::Acquire),
                );
            }
            self.rescan_buffered_owner(&metadata);
            true
        } else {
            let mut matching =
                record
                    .epochs
                    .into_iter()
                    .enumerate()
                    .filter_map(|(index, epoch)| {
                        let epoch = epoch?;
                        let domain = self.domain_at(index);
                        (domain.active() && domain.epoch.load(Ordering::Acquire) == epoch)
                            .then_some(index)
                    });
            let Some(first) = matching.next() else {
                return false;
            };
            // Publication admission or the buffered record's outstanding debt
            // covers the epoch. Marking needs a visible descriptor, never a
            // payload lease or exclusive access to its address shard.
            let Some(target) = self.buffered_target(record) else {
                return false;
            };
            let mut changed = false;
            for index in std::iter::once(first).chain(matching) {
                let _context = Context::enter(self, index);
                changed |= self.shade_known(&target);
            }
            changed
        }
    }

    pub(crate) fn flush_context(&self, context: &crate::context::State) {
        // Fixed upper bound even when other scopes keep producing into this context.
        for _ in 0..crate::context::MAX_RECORDS {
            if !self.flush_context_record(context) {
                break;
            }
        }
    }

    pub(super) fn flush_context_record(&self, context: &crate::context::State) -> bool {
        let Some(record) = context.pop(&self.contexts) else {
            return false;
        };
        let _measurement = self
            .diagnostics
            .barrier_drain
            .measure(self.diagnostics_enabled());
        self.apply_buffered_record(context, record);
        true
    }

    pub(super) fn apply_buffered_record(
        &self,
        context: &crate::context::State,
        record: crate::context::Record,
    ) {
        #[cfg(test)]
        if let Some(observer) = lock(&self.barrier_claim_observer).take() {
            observer.send(()).expect("barrier claim observer");
        }
        #[cfg(all(test, not(target_family = "wasm")))]
        super::barrier_progress_tests::pause_after_buffer_claim();
        self.apply_barrier(record);
        self.acknowledge_buffered_record(context, record);
    }

    /// Claim only the sampled record. Dirty-owner enrollment prepares its
    /// required indexes first. Clean edges need only the visible descriptor.
    /// A claimed record keeps its pending reservation until its effects publish.
    // Keep this path out of step_inner's code/stack when no context is ready.
    #[inline(never)]
    pub(super) fn try_apply_buffered_record(&self, context: &crate::context::State) -> bool {
        let Some(record) = context.try_peek() else {
            return false;
        };
        let _measurement = self
            .diagnostics
            .barrier_drain
            .measure(self.diagnostics_enabled());
        #[cfg(test)]
        {
            let pause = record
                .dirty
                .then(|| lock(&self.barrier_prepare_pause).take())
                .flatten();
            if let Some(pause) = pause {
                pause.entered.send(()).expect("barrier prepared");
                pause
                    .resume
                    .recv_timeout(std::time::Duration::from_secs(10))
                    .expect("resume barrier preparation");
            }
        }
        let Some(record) = context.try_pop_matching(&self.contexts, record) else {
            return false;
        };
        self.apply_buffered_record(context, record);
        true
    }

    /// The pending reservation protects this handoff through enqueue. An owner
    /// already marked Strong may have been traced before the buffered write.
    pub(super) fn rescan_buffered_owner(&self, metadata: &MetadataRef) {
        match self.phase_clock.remembered_stage() {
            Some(0) => self.enqueue_dirty_owner(metadata),
            Some(_) => {
                let _context = Context::enter(self, 0);
                self.shade_metadata(metadata);
            }
            None => {}
        }
    }

    pub(super) fn acknowledge_buffered_record(
        &self,
        context: &crate::context::State,
        record: crate::context::Record,
    ) {
        if !record.dirty && record.epochs.iter().any(Option::is_some) {
            // A certificate is valid only for the same birth and tagged epochs.
            let strong = self.buffered_target(record).is_some_and(|metadata| {
                record.epochs.iter().enumerate().all(|(index, epoch)| {
                    epoch.is_none_or(|epoch| {
                        (index == 0 && !self.is_young_metadata(&metadata))
                            || metadata.retention(index, epoch) == Some(Retention::Strong)
                    })
                })
            });
            if strong {
                context.completed(record);
            }
        }
        self.contexts.pending.fetch_sub(1, Ordering::AcqRel);
        if self.diagnostics_enabled() {
            self.diagnostics
                .barrier_flushed
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Pending reservations protect popped records through publication. No phase,
    /// cycle, coordinator or claim gate is held while applying or releasing owners.
    pub(super) fn drain_barrier_step(&self, concurrent: bool) -> Option<bool> {
        let started = now();
        let mut context = self.contexts.try_ready()?;
        let cpu_started = (concurrent && self.diagnostics_enabled())
            .then(crate::diagnostics::thread_cpu_ns)
            .flatten();
        let measurement = if concurrent {
            &self.diagnostics.worker_steps
        } else {
            &self.diagnostics.caller_steps
        }
        .measure_from(self.diagnostics_enabled(), started);
        let mut remaining = self.configuration.incremental_step_size.max(1);
        loop {
            self.try_apply_buffered_record(&context);
            // This can be the final context owner, whose cleanup may wait.
            drop(context);
            remaining -= 1;
            if remaining == 0
                || self.configuration.max_pause_time_us != 0
                    && started.is_some_and(|started| {
                        started.elapsed().as_micros()
                            >= u128::from(self.configuration.max_pause_time_us)
                    })
            {
                break;
            }
            let Some(next) = self.contexts.try_ready() else {
                break;
            };
            context = next;
        }
        let duration = self.record_step(
            started,
            cpu_started,
            concurrent,
            &mut WorkCounters::default(),
        );
        measurement.finish_with(duration);
        self.notify_progress();
        Some(!self.pending())
    }
}
