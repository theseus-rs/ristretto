//! Incremental policy for the internal conditional-reference-free catalog. One
//! helper owns one request; root/edge/object/phase lanes rotate without retaining pooled
//! observations. A call can still execute arbitrary payload Drop or host allocator
//! work. There is no wall-time bound or public collector selection here.
use super::LeafRecord;
use crate::GarbageCollector;
use crate::canonical_catalog as gc_catalog;
use crate::canonical_catalog::{DOMAIN_COUNT, NO_TICKET, STRONG_FLOOR, YOUNG_DOMAIN};
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
pub(in crate::canonical_adapter) enum MaintenanceError {
    ConditionalReferences,
    Register(RegisterError),
    Catalog(RequestError),
    Protocol,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::canonical_adapter) enum MaintenanceStep {
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

pub(in crate::canonical_adapter) struct LeafMaintenance<'gc> {
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
    swept: [usize; DOMAIN_COUNT],
}
impl<'gc> LeafMaintenance<'gc> {
    pub(in crate::canonical_adapter) fn new(
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
            swept: [0; DOMAIN_COUNT],
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
                    self.swept = [0; DOMAIN_COUNT];
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
        for domain in 0..DOMAIN_COUNT {
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
    pub(in crate::canonical_adapter) fn poll(
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
                for offset in 0..DOMAIN_COUNT {
                    let domain = (self.trace_cursor + offset) % DOMAIN_COUNT;
                    let ticket = marks.marks[domain].ticket;
                    if ticket != NO_TICKET {
                        self.trace_cursor = (domain + 1) % DOMAIN_COUNT;
                        return self.command(GuardedCommand::Acknowledge {
                            key: record.key,
                            domain,
                            ticket,
                        });
                    }
                }
                if let Some(phase) = self.phase {
                    for domain in 0..DOMAIN_COUNT {
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
        for offset in 0..DOMAIN_COUNT {
            let domain = (self.domain_cursor + offset) % DOMAIN_COUNT;
            let state = phase.domains[domain];
            let command = if state.mode == DomainMode::Inactive {
                if domain != YOUNG_DOMAIN {
                    continue;
                }
                GuardedCommand::Start {
                    domain,
                    mode: DomainMode::Full,
                }
            } else if state.floor == STRONG_FLOOR && phase.root_cursors[domain] < self.roots {
                self.domain_cursor = (domain + 1) % DOMAIN_COUNT;
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
            self.domain_cursor = (domain + 1) % DOMAIN_COUNT;
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
#[path = "canonical_maintenance_tests.rs"]
mod tests;
