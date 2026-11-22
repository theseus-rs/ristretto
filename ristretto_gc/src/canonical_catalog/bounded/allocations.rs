//! One participant owns the entire prepared-record to published-root transaction.
//! Cancellation posts durable intent; it never guesses that an installed commit
//! lost. Helpers discover abandoned Prepared cells and requested root removals.
use super::{
    Attempt, CatalogHandle, Cell, Creation, CreationError, GuardedCommand, Outcome, Owned,
    Participant, Payload, Preparation, PreparationError, Progress, RequestError, RequestState,
    RootCommand, RootHandle, RootKey, RootLease, SubmitFailure, prepare_error, roots,
};

#[expect(
    clippy::large_enum_variant,
    reason = "one inline request retains its exact frozen private ownership state"
)]
enum Stage<R: Payload> {
    Starting {
        slot: usize,
        scan_slot: usize,
        misses: usize,
        address: usize,
        record: Option<R>,
    },
    Creating(Creation<R>),
    StartingRoot,
    CreatingRoot(crate::canonical_catalog::RootCreation<R>),
    StartingPublication,
    Preparing(Preparation<R>),
    Attempt(Attempt<R>),
    Ready,
    Taken,
}

// Destruction order retains participant and root quotas through private record,
// cell and root destruction. No owning catalog state escapes except the one
// separately quota-bound completed root.
pub(super) struct Request<R: Payload> {
    stage: Stage<R>,
    cell: Option<Owned<Cell<R>>>,
    root: Option<RootHandle<R>>,
    lease: Option<RootLease<R>>,
    root_slot: usize,
    rotate: bool,
}
pub(super) enum Step {
    Pending,
    Ready(RootKey),
    Complete(Progress),
}
impl<R: Payload> Request<R> {
    pub(super) fn cancel(&self) -> bool {
        match &self.stage {
            Stage::Attempt(attempt) => attempt.cancel(),
            _ => true,
        }
    }
    pub(super) fn take_root(&mut self) -> Option<RootHandle<R>> {
        if !matches!(self.stage, Stage::Ready) {
            return None;
        }
        self.stage = Stage::Taken;
        self.root.take()
    }
    #[expect(
        clippy::too_many_lines,
        reason = "preserve the frozen resumable transaction dispatcher"
    )]
    pub(super) fn poll(&mut self, catalog: &CatalogHandle<R>, context: &R::Context) -> Step {
        let fail = |error| Step::Complete(Progress::Rejected(error));
        match &mut self.stage {
            Stage::Starting {
                slot,
                scan_slot,
                misses,
                address,
                record,
            } => {
                if self.lease.is_none() {
                    // Automatic requests retain the private record even when a
                    // quota CAS loses or every root token is still held. Their
                    // participant already bounds this one unannounced record.
                    match RootLease::reserve(catalog) {
                        Ok(lease) => self.lease = Some(lease),
                        Err(RequestError::RootLimit | RequestError::Contended) => {}
                        Err(_) => unreachable!("root permit reservation"),
                    }
                    return Step::Pending;
                }
                // Backing for one private creation is already charged to this
                // participant. Under prolonged contention, prepare it before
                // the discovery slot becomes vacant so it can take an offer.
                if self.rotate
                    && *misses < crate::canonical_catalog::vacancy::PROBE_INTERVAL
                    && catalog.raw.object_slot_occupied(*slot)
                {
                    *misses += 1;
                    *slot = catalog.raw.next_slot_after_conflict(*slot, scan_slot, 0);
                    return Step::Pending;
                }
                match catalog.raw.prepare_private_creation(
                    *slot,
                    *address,
                    record.take().expect("owned record"),
                    !self.rotate,
                ) {
                    Ok(creation) => self.stage = Stage::Creating(creation),
                    Err(failure) => {
                        let error = prepare_error(failure.error);
                        if error != RequestError::Contended {
                            return fail(error);
                        }
                        *record = Some(failure.record);
                    }
                }
            }
            Stage::Creating(creation) => match if self.rotate {
                creation.poll_rotating(&catalog.raw)
            } else {
                creation.poll(&catalog.raw)
            } {
                Ok(Some(cell)) => {
                    self.cell = Some(cell);
                    self.stage = Stage::StartingRoot;
                }
                Ok(None) => {}
                Err(CreationError::Occupied) => return fail(RequestError::Conflict),
                Err(CreationError::GenerationExhausted) => {
                    return fail(RequestError::GenerationExhausted);
                }
                Err(_) => unreachable!("same unfinished rooted record creation"),
            },
            Stage::StartingRoot => {
                if self.rotate && catalog.raw.external_root_slot_occupied(self.root_slot) {
                    self.root_slot = crate::canonical_catalog::following_slot(
                        self.root_slot,
                        catalog.capacities.root_slots,
                    );
                    return Step::Pending;
                }
                let Some(cell) =
                    crate::canonical_catalog::share(self.cell.as_ref().expect("prepared cell"))
                else {
                    return Step::Pending;
                };
                match catalog.raw.begin_create_root(self.root_slot, cell) {
                    Ok(creation) => self.stage = Stage::CreatingRoot(creation),
                    Err(error) => {
                        let error = roots::prepare_error(error);
                        if self.rotate && error == RequestError::Conflict {
                            self.root_slot = crate::canonical_catalog::following_slot(
                                self.root_slot,
                                catalog.capacities.root_slots,
                            );
                        } else if error != RequestError::Contended {
                            return fail(error);
                        }
                    }
                }
            }
            Stage::CreatingRoot(creation) => match if self.rotate {
                creation.poll_rotating(&catalog.raw)
            } else {
                creation.poll(&catalog.raw)
            } {
                Ok(Some(token)) => {
                    self.root = Some(RootHandle::new(
                        token,
                        self.lease.take().expect("root permit"),
                    ));
                    self.stage = Stage::StartingPublication;
                }
                Ok(None) => {}
                Err(CreationError::Occupied) => return fail(RequestError::Conflict),
                Err(CreationError::GenerationExhausted) => {
                    return fail(RequestError::GenerationExhausted);
                }
                Err(_) => unreachable!("same unfinished allocation root creation"),
            },
            Stage::StartingPublication => {
                let root = self.root.as_ref().expect("prepared root");
                let key = root.key();
                if root.observe().status == crate::canonical_catalog::RootStatus::Live {
                    // Another helper may have published this exact allocation.
                    // Admit its retained record before recovering the result;
                    // the root status alone is not a completion certificate.
                    let Some(record) = root.borrow_live_record() else {
                        return Step::Pending;
                    };
                    let _entered = match record.enter(context) {
                        Ok(entered) => entered,
                        Err(crate::canonical_catalog::RecordError::Contended) => {
                            return Step::Pending;
                        }
                        Err(crate::canonical_catalog::RecordError::Foreign) => {
                            return fail(RequestError::Foreign);
                        }
                        Err(crate::canonical_catalog::RecordError::Closed) => {
                            return fail(RequestError::Stale);
                        }
                    };
                    if root.stably_live_in(context) != Some(true) {
                        return Step::Pending;
                    }
                    self.cell = None;
                    self.stage = Stage::Ready;
                    return Step::Ready(key);
                }
                match roots::start(catalog, context, RootCommand::Publish(key)) {
                    Ok(preparation) => self.stage = Stage::Preparing(preparation),
                    Err(RequestError::Contended) => {}
                    Err(RequestError::Conflict)
                        if matches!(
                            root.observe().status,
                            crate::canonical_catalog::RootStatus::Prepared
                                | crate::canonical_catalog::RootStatus::Live
                        ) =>
                    {
                        // A helper can change the target after our stable
                        // snapshot, then pause before the root effect. Retry
                        // while this exact root remains recoverable; a retired
                        // root must still reject the allocation.
                    }
                    Err(error) => return fail(error),
                }
            }
            Stage::Preparing(preparation) => match preparation.poll(&catalog.raw) {
                Ok(Some(attempt)) => self.stage = Stage::Attempt(attempt),
                Ok(None) => {}
                Err(PreparationError::GenerationExhausted) => {
                    return fail(RequestError::GenerationExhausted);
                }
                Err(_) => unreachable!("same unfinished rooted publication"),
            },
            Stage::Attempt(attempt) => match catalog.raw.commit_in(attempt, context) {
                Outcome::Pending => {}
                Outcome::Retry => self.stage = Stage::StartingPublication,
                Outcome::Cancelled => return Step::Complete(Progress::Cancelled),
                Outcome::Committed => {
                    // The committed root owns the target and the publication
                    // latch is certified. Prepared cancellation no longer applies.
                    self.cell = None;
                    self.stage = Stage::Ready;
                    return Step::Ready(self.root.as_ref().expect("published root").key());
                }
            },
            Stage::Ready => return Step::Ready(self.root.as_ref().expect("completed root").key()),
            Stage::Taken => unreachable!("ready result removed together with request"),
        }
        Step::Pending
    }
}
impl<R: Payload> Drop for Request<R> {
    fn drop(&mut self) {
        self.cancel();
        if let Some(cell) = &self.cell {
            cell.request_abandon();
        }
        // RootHandle drop posts removal even if publication already won. The
        // controller serializes that removal after any committed publication.
        // Field destruction then precedes Participant's registration release.
    }
}

impl<R: Payload> Participant<R, GuardedCommand> {
    /// Discard ownership of this allocation and its result regardless of whether
    /// the commit decision has won. Installed work remains helpable; Prepared
    /// abandonment and root removal are durably requested by the request owner.
    /// Other kinds of participant request are never affected.
    pub fn discard_allocation(&mut self) -> bool {
        if !matches!(self.state, RequestState::Allocation(_)) {
            return false;
        }
        self.state = RequestState::Idle;
        true
    }

    /// Submit a complete allocation using explicit object/root discovery slots.
    /// Success reserves a root permit before accepting the unique record. Polling
    /// returns `RootCreated` only after coupled publication is certified; `take_root`
    /// transfers that exact token once. Cancellation/drop leaves discoverable
    /// cleanup intent for helpers, including the record-before-root interval.
    /// Payload creation, callbacks and destruction are outside bounded poll cost.
    pub fn submit_rooted_record(
        &mut self,
        slot: usize,
        root_slot: usize,
        address: usize,
        record: R,
    ) -> Result<(), SubmitFailure<R>> {
        self.submit_rooted_at(slot, root_slot, address, record, false)
    }

    /// Choose discovery slots incrementally, retaining the exact record across
    /// collisions and transient/full root quota. Full arrays remain Pending until
    /// a helper frees a location or the caller cancels; held roots must be dropped
    /// to return their quota. Preflight rejection returns the supplied record.
    /// This does not bound callbacks, provide scheduling fairness or collect.
    pub fn submit_rooted_record_any(
        &mut self,
        address: usize,
        record: R,
    ) -> Result<(), SubmitFailure<R>> {
        self.submit_rooted_at(
            self.next_object_slot,
            self.next_root_slot,
            address,
            record,
            true,
        )
    }
    fn submit_rooted_at(
        &mut self,
        slot: usize,
        root_slot: usize,
        address: usize,
        record: R,
        rotate: bool,
    ) -> Result<(), SubmitFailure<R>> {
        let reserve = || {
            self.idle()?;
            let catalog = &self.lease.owner;
            if slot >= catalog.capacities.registry || root_slot >= catalog.capacities.root_slots {
                return Err(RequestError::InvalidSlot);
            }
            if address == 0 {
                return Err(RequestError::InvalidAddress);
            }
            match RootLease::reserve(catalog) {
                Ok(lease) => Ok(Some(lease)),
                Err(RequestError::RootLimit | RequestError::Contended) if rotate => Ok(None),
                Err(error) => Err(error),
            }
        };
        let lease = match reserve() {
            Ok(lease) => lease,
            Err(error) => return Err(SubmitFailure { error, record }),
        };
        self.state = RequestState::Allocation(Request {
            stage: Stage::Starting {
                slot,
                scan_slot: slot,
                misses: 0,
                address,
                record: Some(record),
            },
            cell: None,
            root: None,
            lease,
            root_slot,
            rotate,
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{BoundedCatalog, Budget, Capacities, Membership, Ordering};
    use super::*;
    use crate::canonical_catalog::record_fixture::Fixture;
    use std::collections::BTreeSet;

    fn setup() -> (CatalogHandle<Fixture>, Participant<Fixture, GuardedCommand>) {
        let capacities = Capacities::derive(2, 1, 1).unwrap().with_roots(2).unwrap();
        let catalog =
            BoundedCatalog::with_records_in(capacities, &Budget::new(usize::MAX)).unwrap();
        let worker = catalog.register_record_mutator().unwrap();
        (catalog, worker)
    }
    fn stage(worker: &Participant<Fixture, GuardedCommand>) -> &'static str {
        let RequestState::Allocation(request) = &worker.state else {
            panic!("allocation request")
        };
        match request.stage {
            Stage::Starting { .. } => "start",
            Stage::Creating(_) => "create",
            Stage::StartingRoot => "start-root",
            Stage::CreatingRoot(_) => "create-root",
            Stage::StartingPublication => "start-publish",
            Stage::Preparing(_) => "prepare-publish",
            Stage::Attempt(_) => "attempt",
            Stage::Ready => "ready",
            Stage::Taken => "taken",
        }
    }
    fn published_by_helper(
        paused_stage: &str,
    ) -> (
        CatalogHandle<Fixture>,
        Participant<Fixture, GuardedCommand>,
        Fixture,
    ) {
        let (catalog, mut worker) = setup();
        let record = Fixture::new();
        worker
            .submit_rooted_record(0, 0, 8, record.clone())
            .unwrap();
        while stage(&worker) != paused_stage {
            assert_eq!(worker.poll_in(&7), Progress::Pending);
        }
        let mut helper = catalog.register_record_helper().unwrap();
        let key = helper.discover_root(0).unwrap().unwrap().key;
        helper.submit_root(RootCommand::Publish(key)).unwrap();
        assert_eq!(
            (0..128)
                .map(|_| helper.poll_in(&7))
                .find(|p| *p != Progress::Pending),
            Some(Progress::Committed)
        );
        assert_eq!(record.0.published.load(Ordering::Acquire), 1);
        (catalog, worker, record)
    }
    #[test]
    fn allocation_recovers_helper_publication_from_every_private_publication_stage() {
        for paused in ["start-publish", "prepare-publish", "attempt"] {
            let (catalog, mut worker, record) = published_by_helper(paused);
            let root = finish(&mut worker);
            assert_eq!(
                root.observe().status,
                crate::canonical_catalog::RootStatus::Live
            );
            assert_eq!(
                root.borrow_live_record()
                    .unwrap()
                    .0
                    .published
                    .load(Ordering::Acquire),
                1
            );
            assert!(
                worker.take_root().is_none(),
                "helped result transferred twice"
            );
            assert!(!worker.discover(0).unwrap().unwrap().abandon_requested);
            assert_eq!(catalog.reserved_root_tokens(), 1);
            drop(root);
            assert_eq!(catalog.reserved_root_tokens(), 0);
            record.release_origin();
        }
    }

    #[test]
    fn cancelled_owner_of_helper_publication_posts_durable_cleanup() {
        for paused in ["start-publish", "prepare-publish", "attempt"] {
            let (catalog, mut worker, record) = published_by_helper(paused);
            assert!(worker.discard_allocation());
            assert!(worker.take_root().is_none());
            let root = worker.discover_root(0).unwrap().unwrap();
            assert_eq!(root.status, crate::canonical_catalog::RootStatus::Live);
            assert!(
                root.removal_requested,
                "discarded helped publication lost cleanup intent"
            );
            assert_eq!(catalog.reserved_root_tokens(), 0);
            assert!(matches!(
                worker.discover(0).unwrap().unwrap().membership,
                Membership::Tracing(_)
            ));
            record.release_origin();
        }
    }
    #[test]
    fn helper_result_recovery_requires_admission_and_successful_observations() {
        {
            let (_, mut worker, record) = published_by_helper("start-publish");
            crate::canonical_catalog::FAIL_LOAD.with(|slot| slot.set(Some(0)));
            assert_eq!(
                worker.poll_in(&7),
                Progress::Pending,
                "failed stable observation returned a helped result"
            );
            crate::canonical_catalog::FAIL_LOAD.with(|slot| slot.set(None));
            record.0.contended.store(true, Ordering::Release);
            assert_eq!(
                worker.poll_in(&7),
                Progress::Pending,
                "contended admission returned a helped result"
            );
            record.0.contended.store(false, Ordering::Release);
            // Successful recovery needs one owning observation. Its final
            // validation only compares the controller that observation pinned.
            crate::canonical_catalog::FAIL_LOAD.with(|slot| slot.set(Some(1)));
            assert!(matches!(worker.poll_in(&7), Progress::RootCreated(_)));
            assert_eq!(
                crate::canonical_catalog::FAIL_LOAD.with(std::cell::Cell::take),
                Some(0)
            );
            drop(worker.take_root().unwrap());
            record.release_origin();
        }
        let (_, mut worker, record) = published_by_helper("start-publish");
        assert_eq!(
            worker.poll_in(&99),
            Progress::Rejected(RequestError::Foreign),
            "foreign admission recovered a helped result"
        );
        assert!(worker.take_root().is_none());
        record.release_origin();
        let (_, mut worker, record) = published_by_helper("start-publish");
        record.release_origin();
        assert_eq!(
            worker.poll_in(&7),
            Progress::Rejected(RequestError::Stale),
            "closed admission recovered a helped result"
        );
        assert!(worker.take_root().is_none());
    }

    #[test]
    fn cancellation_prefixes_cover_every_transaction_stage_and_leave_no_result() {
        let mut seen = BTreeSet::new();
        for prefix in 0..24 {
            let (catalog, mut worker) = setup();
            let record = Fixture::new();
            worker
                .submit_rooted_record(0, 0, 8, record.clone())
                .unwrap();
            for _ in 0..prefix {
                worker.poll_in(&7);
            }
            seen.insert(stage(&worker));
            assert!(worker.cancel());
            assert!(worker.take_root().is_none());
            assert_eq!(worker.poll_in(&7), Progress::Idle);
            assert_eq!(catalog.reserved_root_tokens(), 0);
            if let Some(cell) = worker.discover(0).unwrap()
                && cell.membership == Membership::Prepared
            {
                assert!(cell.abandon_requested);
            }
            if let Some(root) = worker.discover_root(0).unwrap() {
                assert!(root.removal_requested);
            }
            record.release_origin();
        }
        assert_eq!(
            seen,
            BTreeSet::from([
                "start",
                "create",
                "start-root",
                "create-root",
                "start-publish",
                "prepare-publish",
                "attempt",
                "ready"
            ])
        );
    }

    fn finish(worker: &mut Participant<Fixture, GuardedCommand>) -> RootHandle<Fixture> {
        for _ in 0..128 {
            match worker.poll_in(&7) {
                Progress::Pending => {}
                Progress::RootCreated(_) => return worker.take_root().unwrap(),
                other => panic!("unexpected allocation result {other:?}"),
            }
        }
        panic!("allocation did not settle");
    }
    #[test]
    fn retry_automatic_request_retries_both_private_cell_and_root_collisions() {
        let caps = Capacities::derive(3, 2, 1).unwrap().with_roots(3).unwrap();
        let catalog = BoundedCatalog::with_records_in(caps, &Budget::new(usize::MAX)).unwrap();
        let mut first = catalog.register_record_mutator().unwrap();
        let mut second = catalog.register_record_mutator().unwrap();
        // Registration hints are not authority. Force identical initial probes.
        second.next_object_slot = 0;
        second.next_root_slot = 0;
        first.submit_rooted_record_any(8, Fixture::new()).unwrap();
        second.submit_rooted_record_any(16, Fixture::new()).unwrap();
        for worker in [&mut first, &mut second] {
            for _ in 0..3 {
                assert_eq!(worker.poll_in(&7), Progress::Pending);
            }
            assert_eq!(stage(worker), "create");
        }
        assert_eq!(first.poll_in(&7), Progress::Pending);
        assert_eq!(second.poll_in(&7), Progress::Pending); // lost object CAS
        assert_eq!(second.poll_in(&7), Progress::Pending); // next object CAS
        for worker in [&mut first, &mut second] {
            assert_eq!(worker.poll_in(&7), Progress::Pending); // prepare root
            assert_eq!(worker.poll_in(&7), Progress::Pending); // root backing
            assert_eq!(stage(worker), "create-root");
        }
        assert_eq!(first.poll_in(&7), Progress::Pending);
        assert_eq!(second.poll_in(&7), Progress::Pending); // lost root CAS
        assert_eq!(second.poll_in(&7), Progress::Pending); // next root CAS
        let a = finish(&mut first);
        let b = finish(&mut second);
        assert_eq!((a.key().slot(), b.key().slot()), (0, 1));
        assert_eq!(
            (a.observe().target.slot(), b.observe().target.slot()),
            (0, 1)
        );
        assert_eq!(second.next_object_slot, 2);
        assert_eq!(second.next_root_slot, 2);
        assert_eq!(catalog.reserved_root_tokens(), 2);
        assert_eq!(catalog.raw.discover(1).unwrap().unwrap().address, 16);
    }
    #[test]
    fn retry_automatic_preflight_skips_occupied_slots_without_new_record_preparation() {
        let (catalog, mut worker) = setup();
        let keeper = {
            worker
                .submit_rooted_record(0, 0, 8, Fixture::new())
                .unwrap();
            finish(&mut worker)
        };
        worker.next_object_slot = 0;
        worker.next_root_slot = 0;
        worker.submit_rooted_record_any(16, Fixture::new()).unwrap();
        // Scanning past the occupied object and root must not acquire an
        // owning reference. Leave the next owning load/clone fault armed.
        crate::canonical_catalog::FAIL_LOAD.with(|fault| fault.set(Some(0)));
        assert_eq!(worker.poll_in(&7), Progress::Pending);
        assert_eq!(
            crate::canonical_catalog::FAIL_LOAD.with(std::cell::Cell::take),
            Some(0)
        );
        assert_eq!(stage(&worker), "start");
        for _ in 0..8 {
            if stage(&worker) == "start-root" {
                break;
            }
            assert_eq!(worker.poll_in(&7), Progress::Pending);
        }
        assert_eq!(stage(&worker), "start-root");
        crate::canonical_catalog::FAIL_LOAD.with(|fault| fault.set(Some(0)));
        crate::canonical_catalog::FAIL_SHARE.with(|fault| fault.set(Some(0)));
        assert_eq!(worker.poll_in(&7), Progress::Pending);
        assert_eq!(
            crate::canonical_catalog::FAIL_LOAD.with(std::cell::Cell::take),
            Some(0)
        );
        assert_eq!(
            crate::canonical_catalog::FAIL_SHARE.with(std::cell::Cell::take),
            Some(0)
        );
        assert_eq!(stage(&worker), "start-root");
        let second = finish(&mut worker);
        assert_eq!(
            (second.key().slot(), second.observe().target.slot()),
            (1, 1)
        );
        assert_eq!((worker.next_object_slot, worker.next_root_slot), (0, 0));
        assert_eq!(catalog.reserved_root_tokens(), 2);
        drop((keeper, second));
        assert_eq!(catalog.reserved_root_tokens(), 0);
    }
    #[test]
    #[cfg(not(target_family = "wasm"))]
    fn allocation_retries_partial_helper_publication_after_its_snapshot() {
        let (catalog, mut worker) = setup();
        let record = Fixture::new();
        worker
            .submit_rooted_record(0, 0, 8, record.clone())
            .unwrap();
        while stage(&worker) != "start-publish" {
            assert_eq!(worker.poll_in(&7), Progress::Pending);
        }
        let mut helper = catalog.register_record_helper().unwrap();
        let key = helper.discover_root(0).unwrap().unwrap().key;
        let root = catalog.raw.discover_root(0).unwrap().unwrap();
        let (begin, started) = std::sync::mpsc::channel();
        let (paused, at_effect) = std::sync::mpsc::channel();
        let (resume, released) = std::sync::mpsc::channel();
        root.pause_next_effect(move || {
            paused.send(()).unwrap();
            released.recv().unwrap();
        });
        let publisher = std::thread::spawn(move || {
            started.recv().unwrap();
            helper.submit_root(RootCommand::Publish(key)).unwrap();
            (0..128)
                .map(|_| helper.poll_in(&7))
                .find(|p| *p != Progress::Pending)
        });
        roots::BEFORE_PUBLICATION_PREPARATION.with(|hook| {
            *hook.borrow_mut() = Some(Box::new(move || {
                begin.send(()).unwrap();
                at_effect.recv().unwrap();
            }));
        });
        // The owner has an older stable snapshot; the helper has changed the
        // target but is stopped before changing this same root or certifying it.
        let progress = worker.poll_in(&7);
        let prepared = root.status() == crate::canonical_catalog::RootStatus::Prepared;
        let tracing = matches!(root.target_membership(), Some(Membership::Tracing(_)));
        let unpublished = record.0.published.load(Ordering::Acquire) == 0;
        let absent = worker.take_root().is_none();
        resume.send(()).unwrap();
        assert_eq!(publisher.join().unwrap(), Some(Progress::Committed));
        assert!(prepared && tracing && unpublished && absent);
        assert_eq!(
            progress,
            Progress::Pending,
            "partial helper publication rejected its owner"
        );
        let recovered = finish(&mut worker);
        assert_eq!(recovered.key(), key);
        assert!(worker.take_root().is_none());
        drop(recovered);
        record.release_origin();
    }

    #[test]
    fn a_revoked_helper_publication_cannot_escape_as_an_allocation_result() {
        let (catalog, mut worker, record) = published_by_helper("start-publish");
        let mut helper = catalog.register_record_helper().unwrap();
        let key = helper.discover_root(0).unwrap().unwrap().key;
        helper.submit_root(RootCommand::Remove(key)).unwrap();
        assert_eq!(
            (0..128)
                .map(|_| helper.poll_in(&7))
                .find(|p| *p != Progress::Pending),
            Some(Progress::Committed)
        );
        assert_eq!(
            worker.poll_in(&7),
            Progress::Rejected(RequestError::Conflict),
            "revoked publication returned a result"
        );
        assert!(worker.take_root().is_none());
        record.release_origin();
    }

    #[test]
    fn discard_after_commit_wins_keeps_publication_and_posts_root_removal() {
        let (catalog, mut worker) = setup();
        let record = Fixture::new();
        worker
            .submit_rooted_record(0, 0, 8, record.clone())
            .unwrap();
        while stage(&worker) != "attempt" {
            assert_eq!(worker.poll_in(&7), Progress::Pending);
        }
        let RequestState::Allocation(request) = &worker.state else {
            panic!("allocation")
        };
        let Stage::Attempt(attempt) = &request.stage else {
            panic!("attempt")
        };
        assert_eq!(catalog.raw.commit_in(attempt, &7), Outcome::Committed);
        assert_eq!(record.0.published.load(Ordering::Acquire), 1);
        assert!(
            !worker.cancel(),
            "a committed publication cannot be cancelled"
        );
        assert!(
            worker.discard_allocation(),
            "ownership may still be discarded"
        );
        assert!(!worker.discard_allocation());
        assert_eq!(worker.poll_in(&7), Progress::Idle);
        let cell = worker.discover(0).unwrap().unwrap();
        assert!(matches!(cell.membership, Membership::Tracing(_)));
        assert!(
            cell.abandon_requested,
            "intent is harmless after publication"
        );
        let root = worker.discover_root(0).unwrap().unwrap();
        assert_eq!(root.status, crate::canonical_catalog::RootStatus::Live);
        assert!(root.removal_requested);
        worker
            .submit_root(RootCommand::RemoveRequested(root.key))
            .unwrap();
        let result = (0..128)
            .map(|_| worker.poll_in(&7))
            .find(|r| *r != Progress::Pending)
            .unwrap();
        assert_eq!(result, Progress::Committed);
        assert!(worker.release_retired_root(root.key).unwrap());
        assert_eq!(catalog.reserved_root_tokens(), 0);
        record.release_origin();
    }
}
