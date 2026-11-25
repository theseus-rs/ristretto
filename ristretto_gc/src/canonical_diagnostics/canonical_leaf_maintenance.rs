//! Incremental policy for the internal conditional-reference-free catalog. One
//! helper owns one request; root/edge/object/phase lanes rotate without retaining pooled
//! observations. A call can still execute arbitrary payload Drop or host allocator
//! work. There is no wall-time bound or public collector selection here.
use super::LeafRecord;
use crate::GarbageCollector;
use crate::canonical_catalog as gc_catalog;
use gc_catalog::bounded::{
    CatalogHandle, EdgeCommand, EdgeKey, GuardedCommand, Participant, Progress, RecordKey,
    RegisterError, RequestError, RootCommand, RootKey,
};
use gc_catalog::{DomainMode, EdgeMembership, Membership, Phase, RETIREMENT_FLOOR, RootStatus};

type Worker = Participant<LeafRecord, GuardedCommand>;

const EMPTY_SCAN_BATCH: usize = 8;

/// Skip only confirmed empty positions, keeping the final position for the
/// caller's normal completion path. An occupied slot or failed observation
/// ends the batch. No owning observation is retained between probes.
fn scan_slots<T>(
    cursor: &mut usize,
    slots: usize,
    mut discover: impl FnMut(usize) -> Result<Option<T>, RequestError>,
) -> (usize, Result<Option<T>, RequestError>) {
    debug_assert!(slots != 0 && *cursor < slots);
    let limit = EMPTY_SCAN_BATCH.min(slots);
    let mut skipped = 0;
    let mut observation = discover(*cursor);
    while matches!(observation, Ok(None)) && skipped + 1 < limit {
        *cursor = (*cursor + 1) % slots;
        skipped += 1;
        observation = discover(*cursor);
    }
    (skipped, observation)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::gc::publication_owner) enum MaintenanceError {
    ConditionalReferences,
    Register(RegisterError),
    Catalog(RequestError),
    Protocol,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::gc::publication_owner) enum MaintenanceStep {
    Pending,
    Visited,
    Committed,
    ReleasedObjectSlot,
    ReleasedRootSlot,
    ReleasedEdgeSlot,
}
#[derive(Clone, Copy)]
enum Lane {
    Root,
    Edge,
    Object,
    Phase,
}
#[derive(Clone, Copy)]
enum Pending {
    None,
    Command,
    ReleaseRoot(RootKey),
    ReleaseObject(RecordKey),
    ReleaseEdge(EdgeKey),
}

pub(in crate::gc::publication_owner) struct LeafMaintenance<'gc> {
    // Destroy outstanding request storage before returning its helper quota.
    worker: Worker,
    collector: &'gc GarbageCollector,
    registry: usize,
    roots: usize,
    edges: usize,
    root_cursor: usize,
    edge_cursor: usize,
    object_cursor: usize,
    domain_cursor: usize,
    trace_cursor: usize,
    lane: Lane,
    pending: Pending,
    phase: Option<Phase>,
    visits_since_phase: usize,
    swept: [usize; 2],
}
impl<'gc> LeafMaintenance<'gc> {
    pub(in crate::gc::publication_owner) fn new(
        catalog: &CatalogHandle<LeafRecord>,
        collector: &'gc GarbageCollector,
    ) -> Result<Self, MaintenanceError> {
        let capacities = catalog.capacities();
        if capacities.reference_slots() != 0 {
            return Err(MaintenanceError::ConditionalReferences);
        }
        let worker = catalog
            .register_record_helper()
            .map_err(MaintenanceError::Register)?;
        Ok(Self {
            worker,
            collector,
            registry: capacities.registry(),
            roots: capacities.root_slots(),
            edges: capacities.edge_slots(),
            root_cursor: 0,
            edge_cursor: 0,
            object_cursor: 0,
            domain_cursor: 0,
            trace_cursor: 0,
            lane: Lane::Root,
            pending: Pending::None,
            phase: None,
            visits_since_phase: 0,
            swept: [0; 2],
        })
    }

    fn finish_lane(&mut self, observed: bool) {
        self.pending = Pending::None;
        self.lane = match self.lane {
            Lane::Root => {
                if self.roots != 0 {
                    self.root_cursor = (self.root_cursor + 1) % self.roots;
                }
                if self.edges == 0 {
                    Lane::Object
                } else {
                    Lane::Edge
                }
            }
            Lane::Edge => {
                self.edge_cursor = (self.edge_cursor + 1) % self.edges;
                Lane::Object
            }
            Lane::Object => {
                self.object_cursor = (self.object_cursor + 1) % self.registry;
                if observed {
                    self.visits_since_phase =
                        self.visits_since_phase.saturating_add(1).min(self.registry);
                } else {
                    // Rotate past a contended slot, but do not certify a sweep
                    // containing a failed discovery. A later complete pass is
                    // required; counting just successful visits could count the
                    // same slots repeatedly while always skipping another one.
                    self.visits_since_phase = 0;
                    self.swept = [0; 2];
                }
                Lane::Phase
            }
            Lane::Phase => Lane::Root,
        };
    }
    fn command(&mut self, command: GuardedCommand) -> Result<MaintenanceStep, MaintenanceError> {
        self.worker
            .submit(command)
            .map_err(MaintenanceError::Catalog)?;
        self.pending = Pending::Command;
        Ok(MaintenanceStep::Pending)
    }
    fn root_command(&mut self, command: RootCommand) -> Result<MaintenanceStep, MaintenanceError> {
        self.worker
            .submit_root(command)
            .map_err(MaintenanceError::Catalog)?;
        self.pending = Pending::Command;
        Ok(MaintenanceStep::Pending)
    }
    #[expect(
        clippy::indexing_slicing,
        reason = "fixed two-element domain arrays indexed by 0..2"
    )]
    fn observed_phase(&mut self, phase: Phase) {
        for domain in 0..2 {
            let current = phase.domains[domain];
            let continuous = self.phase.is_some_and(|previous| {
                let previous = previous.domains[domain];
                current.mode != DomainMode::Inactive
                    && previous.mode == current.mode
                    && previous.epoch == current.epoch
                    && previous.floor == RETIREMENT_FLOOR
                    && current.floor == RETIREMENT_FLOOR
            });
            self.swept[domain] = if continuous {
                self.swept[domain]
                    .saturating_add(self.visits_since_phase)
                    .min(self.registry)
            } else {
                0
            };
        }
        self.phase = Some(phase);
        self.visits_since_phase = 0;
    }

    /// One request step or a scan of up to eight slots, stopping at the first
    /// occupied or contended slot. A live edge also checks its source.
    /// Pending preserves preparation;
    /// Drop never loops to finish a cycle. Host callbacks remain unbounded.
    pub(in crate::gc::publication_owner) fn poll(
        &mut self,
    ) -> Result<MaintenanceStep, MaintenanceError> {
        match self.pending {
            Pending::Command => match self.worker.poll_in(self.collector) {
                Progress::Pending => return Ok(MaintenanceStep::Pending),
                Progress::Committed => {
                    self.finish_lane(true);
                    return Ok(MaintenanceStep::Committed);
                }
                Progress::Rejected(
                    RequestError::Conflict | RequestError::Stale | RequestError::AlreadyMarked,
                ) => {
                    self.finish_lane(true);
                    return Ok(MaintenanceStep::Visited);
                }
                Progress::Rejected(error) => {
                    self.finish_lane(false);
                    return Err(MaintenanceError::Catalog(error));
                }
                Progress::Cancelled => {
                    self.finish_lane(false);
                    return Ok(MaintenanceStep::Visited);
                }
                _ => return Err(MaintenanceError::Protocol),
            },
            Pending::ReleaseRoot(key) => {
                let result = self.worker.release_retired_root(key);
                self.finish_lane(true);
                return release_result(result, MaintenanceStep::ReleasedRootSlot);
            }
            Pending::ReleaseObject(key) => {
                let result = self.worker.release_retired(key);
                self.finish_lane(true);
                return release_result(result, MaintenanceStep::ReleasedObjectSlot);
            }
            Pending::ReleaseEdge(key) => {
                let result = self.worker.release_retired_edge(key);
                self.finish_lane(true);
                return release_result(result, MaintenanceStep::ReleasedEdgeSlot);
            }
            Pending::None => {}
        }
        match self.lane {
            Lane::Root => self.root(),
            Lane::Edge => self.edge(),
            Lane::Object => self.object(),
            Lane::Phase => self.phase(),
        }
    }
    fn edge(&mut self) -> Result<MaintenanceStep, MaintenanceError> {
        let (_, observation) = scan_slots(&mut self.edge_cursor, self.edges, |slot| {
            self.worker.discover_edge(slot)
        });
        let edge = match observation {
            Ok(Some(edge)) => edge,
            Ok(None) | Err(RequestError::Contended) => {
                self.finish_lane(true);
                return Ok(MaintenanceStep::Visited);
            }
            Err(error) => {
                self.finish_lane(false);
                return Err(MaintenanceError::Catalog(error));
            }
        };
        if edge.state == EdgeMembership::Retired {
            self.pending = Pending::ReleaseEdge(edge.key);
            return Ok(MaintenanceStep::Pending);
        }
        let command = if edge.removal_requested {
            EdgeCommand::RemoveRequested(edge.key)
        } else if let EdgeMembership::Live(state) = edge.state {
            // A live source rules out useful cleanup at this observation. If
            // it retires immediately afterwards, the next sweep sees it. An
            // uncertain or stale observation still goes through RemoveOrphan,
            // which revalidates the exact edge and source before any change.
            if let Ok(Some(source)) = self.worker.discover(state.source.slot)
                && source.key.identity() == state.source.identity
                && source.membership != Membership::Retired
            {
                self.finish_lane(true);
                return Ok(MaintenanceStep::Visited);
            }
            EdgeCommand::RemoveOrphan(edge.key)
        } else {
            self.finish_lane(true);
            return Ok(MaintenanceStep::Visited);
        };
        self.worker
            .submit_edge(command)
            .map_err(MaintenanceError::Catalog)?;
        self.pending = Pending::Command;
        Ok(MaintenanceStep::Pending)
    }
    fn root(&mut self) -> Result<MaintenanceStep, MaintenanceError> {
        if self.roots == 0 {
            self.finish_lane(true);
            return Ok(MaintenanceStep::Visited);
        }
        let (_, observation) = scan_slots(&mut self.root_cursor, self.roots, |slot| {
            self.worker.discover_root(slot)
        });
        match observation {
            Ok(Some(root)) if root.removal_requested => {
                if root.status == RootStatus::Retired {
                    self.pending = Pending::ReleaseRoot(root.key);
                    Ok(MaintenanceStep::Pending)
                } else {
                    self.root_command(RootCommand::RemoveRequested(root.key))
                }
            }
            Ok(Some(root)) if !root.internal && root.status == RootStatus::Prepared => {
                // A prepared allocation root already owns a fully initialized
                // record. Help its coupled publication before more phase work
                // can repeatedly supersede the origin's private preparation.
                // Roots for already-tracing targets fail Publish's precondition;
                // in particular this never substitutes for child-edge activation.
                self.root_command(RootCommand::Publish(root.key))
            }
            Ok(_) | Err(RequestError::Contended) => {
                self.finish_lane(true);
                Ok(MaintenanceStep::Visited)
            }
            Err(error) => {
                self.finish_lane(false);
                Err(MaintenanceError::Catalog(error))
            }
        }
    }
    #[expect(
        clippy::indexing_slicing,
        reason = "domain indices are reduced modulo the fixed two-element domain count"
    )]
    fn object(&mut self) -> Result<MaintenanceStep, MaintenanceError> {
        let (skipped, observation) = scan_slots(&mut self.object_cursor, self.registry, |slot| {
            self.worker.discover(slot)
        });
        // Every skipped position was observed empty in consecutive cursor
        // order. The final position is counted by finish_lane; any failed
        // discovery or command still resets the entire sweep certificate.
        self.visits_since_phase = self
            .visits_since_phase
            .saturating_add(skipped)
            .min(self.registry);
        let record = match observation {
            Ok(Some(record)) => record,
            Ok(None) => {
                self.finish_lane(true);
                return Ok(MaintenanceStep::Visited);
            }
            Err(RequestError::Contended) => {
                self.finish_lane(false);
                return Ok(MaintenanceStep::Visited);
            }
            Err(error) => {
                self.finish_lane(false);
                return Err(MaintenanceError::Catalog(error));
            }
        };
        match record.membership {
            Membership::Prepared if record.abandon_requested => {
                self.command(GuardedCommand::Abandon(record.key))
            }
            Membership::Retired => {
                self.pending = Pending::ReleaseObject(record.key);
                Ok(MaintenanceStep::Pending)
            }
            Membership::Tracing(marks) => {
                // Alternate domains so a stream of replacement tickets in one
                // domain cannot always outrank the other's existing ticket.
                for offset in 0..2 {
                    let domain = (self.trace_cursor + offset) % 2;
                    let ticket = marks.marks[domain].ticket;
                    if ticket != 0 {
                        self.trace_cursor = 1 - domain;
                        return self.command(GuardedCommand::Acknowledge {
                            key: record.key,
                            domain,
                            ticket,
                        });
                    }
                }
                if let Some(phase) = self.phase {
                    for domain in 0..2 {
                        if marks.can_retire(phase, domain).is_ok() {
                            return self.command(GuardedCommand::Retire {
                                key: record.key,
                                domain,
                            });
                        }
                    }
                }
                self.finish_lane(true);
                Ok(MaintenanceStep::Visited)
            }
            Membership::Prepared => {
                self.finish_lane(true);
                Ok(MaintenanceStep::Visited)
            }
            Membership::Live(_) => Err(MaintenanceError::Protocol),
        }
    }
    #[expect(
        clippy::indexing_slicing,
        reason = "domain indices are reduced modulo the fixed two-element domain count"
    )]
    fn phase(&mut self) -> Result<MaintenanceStep, MaintenanceError> {
        let phase = match self.worker.phase_in(self.collector) {
            Ok(phase) => phase,
            Err(RequestError::Contended) => {
                self.finish_lane(true);
                return Ok(MaintenanceStep::Visited);
            }
            Err(error) => {
                self.finish_lane(false);
                return Err(MaintenanceError::Catalog(error));
            }
        };
        self.observed_phase(phase);
        for offset in 0..2 {
            let domain = (self.domain_cursor + offset) % 2;
            let state = phase.domains[domain];
            let command = if state.mode == DomainMode::Inactive {
                if domain != 0 {
                    continue;
                }
                GuardedCommand::Start {
                    domain,
                    mode: DomainMode::Full,
                }
            } else if state.floor == 0 && phase.root_cursors[domain] < self.roots {
                self.domain_cursor = 1 - domain;
                return self.root_command(RootCommand::ScanBatch { domain });
            } else if state.pending != 0 {
                continue;
            } else if state.floor < RETIREMENT_FLOOR {
                GuardedCommand::Advance { domain }
            } else if self.swept[domain] == self.registry {
                GuardedCommand::FinishEpoch {
                    domain,
                    epoch: state.epoch,
                }
            } else {
                continue;
            };
            self.domain_cursor = 1 - domain;
            return self.command(command);
        }
        self.finish_lane(true);
        Ok(MaintenanceStep::Visited)
    }
}
fn release_result(
    result: Result<bool, RequestError>,
    released: MaintenanceStep,
) -> Result<MaintenanceStep, MaintenanceError> {
    match result {
        Ok(true) => Ok(released),
        Ok(false) | Err(RequestError::Stale | RequestError::Contended) => {
            Ok(MaintenanceStep::Visited)
        }
        Err(error) => Err(MaintenanceError::Catalog(error)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gc_catalog::bounded::{BoundedCatalog, Budget, Capacities};

    #[test]
    fn a_retained_source_skips_orphan_work_and_its_edges_are_reclaimed_after_drop() {
        use super::super::graph::GraphRuntime;

        let gc = GarbageCollector::new();
        let budget = Budget::new(usize::MAX);
        let caps = Capacities::derive(2, 1, 1)
            .unwrap()
            .with_roots(4)
            .unwrap()
            .with_edges(2)
            .unwrap();
        let runtime = GraphRuntime::new(&gc, caps, 1, &budget).unwrap();
        let mut worker = runtime.mutator().unwrap();
        let root = {
            let mut allocation = worker.allocate(17_u64).unwrap();
            (0..256)
                .find_map(|_| allocation.poll().unwrap())
                .expect("uncontended source allocation")
        };
        let mut helper = runtime.maintenance().unwrap();
        let edge = helper.worker.discover_edge(0).unwrap().unwrap();
        assert!(matches!(edge.state, EdgeMembership::Live(_)));
        helper.lane = Lane::Edge;
        assert_eq!(helper.poll().unwrap(), MaintenanceStep::Visited);
        assert!(matches!(helper.lane, Lane::Object));
        assert!(matches!(helper.pending, Pending::None));
        assert_eq!(helper.worker.discover_edge(0).unwrap().unwrap(), edge);

        drop(root);
        assert!((0..10_000).any(|_| helper.poll().unwrap() == MaintenanceStep::ReleasedEdgeSlot));
        assert!(helper.worker.discover_edge(0).unwrap().is_none());
        drop((helper, worker, runtime));
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(budget.used(), 0);
    }

    #[test]
    fn empty_batch_is_bounded_and_wraps_without_duplicate_visits() {
        for slots in [1, 2, 7, 8, 9, 17] {
            for start in [0, slots / 2, slots - 1] {
                let mut cursor = start;
                let mut visited = Vec::new();
                let (skipped, result) = scan_slots(&mut cursor, slots, |slot| {
                    visited.push(slot);
                    Ok(None::<()>)
                });
                let expected: Vec<_> = (0..slots.min(8))
                    .map(|offset| (start + offset) % slots)
                    .collect();
                assert_eq!(visited, expected, "empty batch probe bound or cursor wrap");
                assert_eq!(skipped, expected.len() - 1);
                assert_eq!(
                    cursor,
                    *expected.last().unwrap(),
                    "last probe must await lane completion"
                );
                assert_eq!(result, Ok(None));
            }
        }
    }
    #[test]
    fn empty_batch_stops_at_each_occupied_or_failed_observation() {
        for stop in 0..8 {
            for terminal in [
                Ok(Some(71)),
                Err(RequestError::Contended),
                Err(RequestError::InvalidSlot),
            ] {
                let mut cursor = 13;
                let mut visited = Vec::new();
                let (skipped, result) = scan_slots(&mut cursor, 17, |slot| {
                    let offset = visited.len();
                    visited.push(slot);
                    if offset == stop { terminal } else { Ok(None) }
                });
                assert_eq!(
                    visited.len(),
                    stop + 1,
                    "batch crossed an occupied or failed observation"
                );
                assert_eq!(skipped, stop);
                assert_eq!(cursor, (13 + stop) % 17);
                assert_eq!(result, terminal, "batch lost the final observation");
            }
        }
    }
    #[test]
    fn empty_lanes_rotate_and_only_complete_sweeps_certify_retirement() {
        let gc = GarbageCollector::new();
        let capacities = Capacities::derive(17, 0, 1)
            .unwrap()
            .with_roots(17)
            .unwrap()
            .with_edges(17)
            .unwrap();
        let catalog =
            BoundedCatalog::with_records_in(capacities, &Budget::new(usize::MAX)).unwrap();
        let mut helper = LeafMaintenance::new(&catalog, &gc).unwrap();
        let phase = phase(7, RETIREMENT_FLOOR, DomainMode::Full);
        helper.observed_phase(phase);
        assert_eq!(helper.poll().unwrap(), MaintenanceStep::Visited);
        assert_eq!(helper.root_cursor, 8);
        assert!(matches!(helper.lane, Lane::Edge));
        assert_eq!(helper.poll().unwrap(), MaintenanceStep::Visited);
        assert_eq!(helper.edge_cursor, 8);
        assert!(matches!(helper.lane, Lane::Object));
        assert_eq!(helper.poll().unwrap(), MaintenanceStep::Visited);
        assert_eq!(helper.object_cursor, 8);
        assert_eq!(
            helper.visits_since_phase, 8,
            "batch must count exactly the empty positions observed"
        );
        assert_eq!(
            helper.swept[0], 0,
            "uncertified batch cannot finish an epoch"
        );
        helper.observed_phase(phase);
        assert_eq!(helper.swept[0], 8);
        helper.lane = Lane::Object;
        helper.poll().unwrap();
        helper.observed_phase(phase);
        assert_eq!(helper.object_cursor, 16);
        assert_eq!(
            helper.swept[0], 16,
            "incomplete batch sweep cannot finish an epoch"
        );
        helper.lane = Lane::Object;
        helper.poll().unwrap();
        helper.observed_phase(phase);
        assert_eq!(helper.object_cursor, 7);
        assert_eq!(helper.swept[0], 17);
    }
    #[test]
    fn batches_stop_at_live_root_and_object_before_counting_later_slots() {
        use super::super::allocation::LeafAllocation;
        let gc = GarbageCollector::new();
        let capacities = Capacities::derive(17, 1, 1)
            .unwrap()
            .with_roots(17)
            .unwrap();
        let catalog =
            BoundedCatalog::with_records_in(capacities, &Budget::new(usize::MAX)).unwrap();
        let mut worker = catalog.register_record_mutator().unwrap();
        let mut roots = Vec::new();
        for slot in [7, 8] {
            let mut request = LeafAllocation::begin(&mut worker, &gc, slot, slot, 71_u64).unwrap();
            roots.push((0..128).find_map(|_| request.poll().unwrap()).unwrap());
        }
        let mut helper = LeafMaintenance::new(&catalog, &gc).unwrap();
        assert_eq!(helper.root().unwrap(), MaintenanceStep::Visited);
        assert_eq!(
            helper.root_cursor, 8,
            "root scan crossed first occupied position"
        );
        helper.lane = Lane::Root;
        assert_eq!(helper.root().unwrap(), MaintenanceStep::Visited);
        assert_eq!(
            helper.root_cursor, 9,
            "root scan crossed next occupied position"
        );
        for slot in [7, 8] {
            helper.lane = Lane::Object;
            assert_eq!(helper.object().unwrap(), MaintenanceStep::Pending);
            assert_eq!(
                helper.object_cursor, slot,
                "occupied object awaits request completion"
            );
            assert_eq!(
                helper.visits_since_phase, slot,
                "uncommitted object inflated sweep credit"
            );
            let terminal = (0..128)
                .map(|_| helper.poll().unwrap())
                .find(|step| *step != MaintenanceStep::Pending)
                .unwrap();
            assert_eq!(terminal, MaintenanceStep::Committed);
            assert_eq!(helper.object_cursor, slot + 1);
            assert_eq!(
                helper.visits_since_phase,
                slot + 1,
                "occupied position inflated complete-sweep credit"
            );
        }
        assert_eq!(*roots[0].borrow(&gc).unwrap(), 71);
        assert_eq!(*roots[1].borrow(&gc).unwrap(), 71);
    }
    #[test]
    fn helper_publishes_an_announced_allocation_before_its_owner_resumes() {
        use super::super::allocation::LeafAllocation;
        let gc = GarbageCollector::new();
        let capacities = Capacities::derive(9, 1, 1).unwrap().with_roots(9).unwrap();
        let catalog =
            BoundedCatalog::with_records_in(capacities, &Budget::new(usize::MAX)).unwrap();
        let mut worker = catalog.register_record_mutator().unwrap();
        let mut helper = LeafMaintenance::new(&catalog, &gc).unwrap();
        let root = {
            let mut request = LeafAllocation::begin(&mut worker, &gc, 7, 7, 71_u64).unwrap();
            assert!((0..128).any(|_| {
                assert!(request.poll().unwrap().is_none());
                helper
                    .worker
                    .discover_root(7)
                    .unwrap()
                    .is_some_and(|root| root.status == RootStatus::Prepared)
            }));
            assert!(
                (0..128).any(|_| {
                    helper.poll().unwrap() == MaintenanceStep::Committed
                        && helper
                            .worker
                            .discover_root(7)
                            .unwrap()
                            .is_some_and(|root| root.status == RootStatus::Live)
                }),
                "helper did not publish the stopped allocation"
            );
            request
                .poll()
                .unwrap()
                .expect("owner did not recover helper publication")
        };
        assert_eq!(*root.borrow(&gc).unwrap(), 71);
        drop(root);
        assert!(
            (0..4096).any(|_| {
                helper.poll().unwrap();
                gc.managed_bytes() == 0
            }),
            "helped publication did not reclaim"
        );
    }
    fn phase(epoch: u64, floor: u8, mode: DomainMode) -> Phase {
        Phase {
            root_cursors: [0; 2],
            reference_cursors: [0; 2],
            revision: 0,
            domains: [
                gc_catalog::Domain {
                    mode,
                    epoch,
                    floor,
                    pending: 0,
                },
                gc_catalog::Domain {
                    mode: DomainMode::Inactive,
                    epoch: 1,
                    floor: RETIREMENT_FLOOR,
                    pending: 0,
                },
            ],
        }
    }
    fn visited(helper: &mut LeafMaintenance<'_>, success: bool) {
        helper.lane = Lane::Object;
        helper.finish_lane(success);
    }
    #[test]
    fn sweep_counts_consecutive_visits_across_delayed_phase_observations() {
        let gc = GarbageCollector::new();
        let catalog = BoundedCatalog::with_records_in(
            Capacities::derive(3, 0, 1).unwrap(),
            &Budget::new(usize::MAX),
        )
        .unwrap();
        let mut helper = LeafMaintenance::new(&catalog, &gc).unwrap();
        let phase = phase(7, RETIREMENT_FLOOR, DomainMode::Full);
        helper.observed_phase(phase);
        for _ in 0..3 {
            visited(&mut helper, true);
        }
        assert_eq!(
            helper.swept[0], 0,
            "uncertified interval cannot finish a cycle"
        );
        helper.observed_phase(phase);
        assert_eq!(helper.swept[0], 3);
        assert_eq!(helper.visits_since_phase, 0);
        assert_eq!(helper.swept[1], 0);
    }
    #[test]
    fn a_failed_object_discovery_requires_a_new_complete_sweep() {
        let gc = GarbageCollector::new();
        let catalog = BoundedCatalog::with_records_in(
            Capacities::derive(3, 0, 1).unwrap(),
            &Budget::new(usize::MAX),
        )
        .unwrap();
        let mut helper = LeafMaintenance::new(&catalog, &gc).unwrap();
        let phase = phase(9, RETIREMENT_FLOOR, DomainMode::Full);
        helper.observed_phase(phase);
        for _ in 0..3 {
            visited(&mut helper, true);
        }
        helper.observed_phase(phase);
        assert_eq!(helper.swept[0], 3);
        visited(&mut helper, false);
        for _ in 0..2 {
            visited(&mut helper, true);
        }
        helper.observed_phase(phase);
        assert_eq!(helper.swept[0], 2);
        visited(&mut helper, true);
        helper.observed_phase(phase);
        assert_eq!(helper.swept[0], 3);
    }
    #[test]
    fn a_changed_epoch_mode_or_floor_cannot_certify_prior_visits() {
        let gc = GarbageCollector::new();
        let catalog = BoundedCatalog::with_records_in(
            Capacities::derive(3, 0, 1).unwrap(),
            &Budget::new(usize::MAX),
        )
        .unwrap();
        let mut helper = LeafMaintenance::new(&catalog, &gc).unwrap();
        let baseline = phase(11, RETIREMENT_FLOOR, DomainMode::Full);
        for changed in [
            phase(12, RETIREMENT_FLOOR, DomainMode::Full),
            phase(11, RETIREMENT_FLOOR, DomainMode::Inactive),
            phase(11, RETIREMENT_FLOOR - 1, DomainMode::Full),
        ] {
            helper.observed_phase(baseline);
            for _ in 0..3 {
                visited(&mut helper, true);
            }
            helper.observed_phase(baseline);
            assert_eq!(helper.swept[0], 3, "establish a complete certified sweep");
            visited(&mut helper, true);
            helper.observed_phase(changed);
            assert_eq!(helper.swept[0], 0);
            assert_eq!(helper.visits_since_phase, 0);
        }
    }
    #[test]
    fn stale_queued_finish_cannot_close_a_later_unswept_epoch() {
        fn run(worker: &mut Worker, gc: &GarbageCollector, command: GuardedCommand) {
            worker.submit(command).unwrap();
            let result = (0..128)
                .map(|_| worker.poll_in(gc))
                .find(|r| *r != Progress::Pending)
                .unwrap();
            assert_eq!(result, Progress::Committed);
        }
        let gc = GarbageCollector::new();
        let catalog = BoundedCatalog::with_records_in(
            Capacities::derive(3, 1, 1).unwrap(),
            &Budget::new(usize::MAX),
        )
        .unwrap();
        let mut external = catalog.register_record_mutator().unwrap();
        let mut helper = LeafMaintenance::new(&catalog, &gc).unwrap();
        let mut old_epoch = None;
        for _ in 0..4096 {
            helper.poll().unwrap();
            if matches!(helper.lane, Lane::Phase)
                && matches!(helper.pending, Pending::Command)
                && helper.swept[0] == helper.registry
                && let Some(phase) = helper.phase
                && phase.domains[0].floor == RETIREMENT_FLOOR
                && phase.domains[0].mode != DomainMode::Inactive
                && phase.domains[1].mode == DomainMode::Inactive
            {
                old_epoch = Some(phase.domains[0].epoch);
                break;
            }
        }
        let old_epoch = old_epoch.expect("queued finish after a complete local sweep");
        // Another policy owner completes this epoch, then reaches retirement in
        // a new epoch while this helper's Finish request remains unpolled.
        run(&mut external, &gc, GuardedCommand::Finish { domain: 0 });
        run(
            &mut external,
            &gc,
            GuardedCommand::Start {
                domain: 0,
                mode: DomainMode::Full,
            },
        );
        for _ in 0..RETIREMENT_FLOOR {
            run(&mut external, &gc, GuardedCommand::Advance { domain: 0 });
        }
        let current = external.phase_in(&gc).unwrap();
        assert_eq!(current.domains[0].epoch, old_epoch + 1);
        let result = (0..128)
            .map(|_| helper.poll().unwrap())
            .find(|r| *r != MaintenanceStep::Pending)
            .unwrap();
        assert_eq!(
            result,
            MaintenanceStep::Visited,
            "stale Finish must be rejected before closing another epoch"
        );
        let after = external.phase_in(&gc).unwrap();
        assert_eq!(after.domains[0].mode, DomainMode::Full);
        assert_eq!(after.domains[0].epoch, current.domains[0].epoch);
        assert_eq!(after.domains[0].floor, RETIREMENT_FLOOR);
    }
}
