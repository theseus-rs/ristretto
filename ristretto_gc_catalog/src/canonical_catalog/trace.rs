//! Canonical numeric reachability. This does not own a payload or its graph.
#![expect(
    clippy::indexing_slicing,
    reason = "parallel two-domain arrays use a checked domain index or fixed enumeration"
)]
use super::{Domain, Phase};

pub const YOUNG_DOMAIN: usize = 0;
pub(crate) const OLD_DOMAIN: usize = 1;
pub const DOMAIN_COUNT: usize = 2;
pub const NO_TICKET: u64 = 0;

pub const STRONG_FLOOR: u8 = 0;
const FINAL_FLOORS: std::ops::RangeInclusive<u8> = 1..=2;
const PHANTOM_FLOORS: std::ops::RangeInclusive<u8> = 3..=4;
pub const RETIREMENT_FLOOR: u8 = 5;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DomainMode {
    Inactive,
    Young,
    Full,
    Old,
}
impl DomainMode {
    #[must_use]
    pub fn collects(self, generation: Generation) -> bool {
        match self {
            Self::Inactive => false,
            Self::Full => true,
            Self::Young => generation == Generation::Young,
            Self::Old => generation == Generation::Old,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Generation {
    Young,
    Old,
}

/// Lower values denote stronger reachability, matching the collector's floors.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TraceRetention {
    Strong,
    Final,
    Phantom,
}
impl TraceRetention {
    #[must_use]
    pub fn at_floor(floor: u8) -> Option<Self> {
        match floor {
            STRONG_FLOOR => Some(Self::Strong),
            floor if FINAL_FLOORS.contains(&floor) => Some(Self::Final),
            floor if PHANTOM_FLOORS.contains(&floor) => Some(Self::Phantom),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TraceMark {
    pub epoch: u64,
    pub retention: Option<TraceRetention>,
    pub ticket: u64,
    pub born: bool,
    /// Group decision belongs to this mark's epoch; reuse the existing epoch.
    pub phantom_decided: bool,
}
impl TraceMark {
    #[must_use]
    pub fn unmarked(epoch: u64) -> Self {
        Self {
            epoch,
            retention: None,
            ticket: NO_TICKET,
            born: false,
            phantom_decided: false,
        }
    }

    #[must_use]
    pub fn retention_at(self, epoch: u64) -> Option<TraceRetention> {
        (self.epoch == epoch).then_some(self.retention).flatten()
    }

    /// Birth exemption is physical liveness, not earlier strong/final reachability.
    #[must_use]
    pub fn marked(self, epoch: u64) -> bool {
        self.epoch == epoch && (self.born || self.retention.is_some())
    }

    #[must_use]
    pub fn marked_before(self, epoch: u64, include_final: bool) -> bool {
        self.retention_at(epoch).is_some_and(|strength| {
            strength
                <= if include_final {
                    TraceRetention::Final
                } else {
                    TraceRetention::Strong
                }
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TraceError {
    InvalidDomain,
    Conflict,
    AlreadyMarked,
    Exhausted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TraceState {
    pub generation: Generation,
    /// Stable birth identity, even after a later deletion retains the object.
    pub old_birth: u64,
    pub marks: [TraceMark; DOMAIN_COUNT],
}
impl std::ops::Deref for TraceState {
    type Target = [TraceMark; DOMAIN_COUNT];

    fn deref(&self) -> &Self::Target {
        &self.marks
    }
}
impl TraceState {
    #[must_use]
    pub fn unmarked(phase: Phase) -> Self {
        Self {
            generation: Generation::Young,
            old_birth: 0,
            marks: phase.domains.map(|d| TraceMark::unmarked(d.epoch)),
        }
    }

    /// Numeric publication only. The caller must separately protect initialized
    /// payloads/children and supply canonical root and reference protocols.
    pub fn publication(mut phase: Phase) -> Result<(Self, Phase), TraceError> {
        let mut state = Self::unmarked(phase);
        for index in 0..DOMAIN_COUNT {
            let domain = phase.domains[index];
            if domain.mode == DomainMode::Inactive {
                continue;
            }
            let mark = &mut state.marks[index];
            if domain.mode == DomainMode::Old
                || (index == OLD_DOMAIN && domain.floor == RETIREMENT_FLOOR)
            {
                mark.born = true;
                if domain.mode == DomainMode::Old {
                    state.old_birth = domain.epoch;
                }
            } else {
                mark.retention = Some(TraceRetention::Strong);
                if domain.floor == STRONG_FLOOR {
                    mark.ticket = phase.revision.checked_add(1).ok_or(TraceError::Exhausted)?;
                    phase.domains[index].pending =
                        domain.pending.checked_add(1).ok_or(TraceError::Exhausted)?;
                }
            }
        }
        Ok((state, phase))
    }

    #[must_use]
    pub fn logically_retired(self, phase: Phase) -> bool {
        phase.domains.iter().enumerate().any(|(index, domain)| {
            domain.mode.collects(self.generation)
                && domain.floor == RETIREMENT_FLOOR
                && !self.marks[index].marked(domain.epoch)
        })
    }

    #[must_use]
    pub fn reference_live(self, domain: Domain, index: usize, include_final: bool) -> bool {
        domain.mode != DomainMode::Inactive
            && self.marks.get(index).is_some_and(|mark| {
                !domain.mode.collects(self.generation)
                    || mark.marked_before(domain.epoch, include_final)
            })
    }

    pub fn shade(
        mut self,
        mut phase: Phase,
        index: usize,
        requested: TraceRetention,
    ) -> Result<(Self, Phase), TraceError> {
        let domain = *phase.domains.get(index).ok_or(TraceError::InvalidDomain)?;
        if domain.mode == DomainMode::Inactive
            || self.logically_retired(phase)
            || (domain.mode == DomainMode::Young && self.generation == Generation::Old)
        {
            return Err(TraceError::Conflict);
        }
        let retention =
            requested.max(TraceRetention::at_floor(domain.floor).ok_or(TraceError::Conflict)?);
        let mark = &mut self.marks[index];
        let old_young = domain.mode == DomainMode::Old && self.generation == Generation::Young;
        // Ordinary owning changes cannot turn a post-snapshot young birth into
        // an old snapshot root. Explicit callback-edge retention is separate.
        if old_young && mark.epoch == domain.epoch && mark.born {
            return Err(TraceError::Conflict);
        }
        if mark
            .retention_at(domain.epoch)
            .is_some_and(|old| old <= retention)
        {
            return Err(TraceError::AlreadyMarked);
        }
        if mark.ticket != NO_TICKET && mark.epoch != domain.epoch {
            return Err(TraceError::Conflict);
        }
        let ticket = if old_young {
            NO_TICKET
        } else {
            if mark.ticket == NO_TICKET {
                phase.domains[index].pending =
                    domain.pending.checked_add(1).ok_or(TraceError::Exhausted)?;
            }
            phase.revision.checked_add(1).ok_or(TraceError::Exhausted)?
        };
        *mark = TraceMark {
            epoch: domain.epoch,
            retention: Some(retention),
            ticket,
            born: false,
            phantom_decided: mark.epoch == domain.epoch && mark.phantom_decided,
        };
        Ok((self, phase))
    }

    pub fn acknowledge(
        mut self,
        mut phase: Phase,
        index: usize,
        ticket: u64,
    ) -> Result<(Self, Phase), TraceError> {
        let domain = *phase.domains.get(index).ok_or(TraceError::InvalidDomain)?;
        let mark = &mut self.marks[index];
        if domain.mode == DomainMode::Inactive
            || ticket == NO_TICKET
            || mark.ticket != ticket
            || mark.epoch != domain.epoch
        {
            return Err(TraceError::Conflict);
        }
        mark.ticket = NO_TICKET;
        phase.domains[index].pending = domain.pending.checked_sub(1).ok_or(TraceError::Conflict)?;
        Ok((self, phase))
    }

    pub fn can_retire(self, phase: Phase, index: usize) -> Result<(), TraceError> {
        let domain = *phase.domains.get(index).ok_or(TraceError::InvalidDomain)?;
        if !domain.mode.collects(self.generation)
            || domain.floor != RETIREMENT_FLOOR
            || self.marks[index].marked(domain.epoch)
            || self.marks.iter().any(|m| m.ticket != NO_TICKET)
        {
            return Err(TraceError::Conflict);
        }
        Ok(())
    }

    /// Generation publication and the old-core handoff are one exact cell effect.
    /// Existing young traversal tickets remain owned and must still be drained.
    pub fn promote(mut self, mut phase: Phase) -> Result<(Self, Phase), TraceError> {
        if self.generation != Generation::Young || self.logically_retired(phase) {
            return Err(TraceError::Conflict);
        }
        let old = phase.domains[OLD_DOMAIN];
        match old.mode {
            DomainMode::Old => {
                let ticket = if old.floor == STRONG_FLOOR {
                    if self.marks[OLD_DOMAIN].ticket == NO_TICKET {
                        phase.domains[OLD_DOMAIN].pending =
                            old.pending.checked_add(1).ok_or(TraceError::Exhausted)?;
                    }
                    phase.revision.checked_add(1).ok_or(TraceError::Exhausted)?
                } else {
                    NO_TICKET
                };
                self.marks[OLD_DOMAIN] = TraceMark {
                    epoch: old.epoch,
                    retention: Some(TraceRetention::Strong),
                    ticket,
                    born: false,
                    phantom_decided: self.marks[OLD_DOMAIN].epoch == old.epoch
                        && self.marks[OLD_DOMAIN].phantom_decided,
                };
            }
            DomainMode::Full if old.floor < RETIREMENT_FLOOR => {
                let retention = self.marks[YOUNG_DOMAIN]
                    .retention_at(phase.domains[YOUNG_DOMAIN].epoch)
                    .unwrap_or(TraceRetention::Phantom);
                match self.shade(phase, OLD_DOMAIN, retention) {
                    Ok((state, next)) => {
                        self = state;
                        phase = next;
                    }
                    Err(TraceError::AlreadyMarked) => {}
                    Err(error) => return Err(error),
                }
            }
            _ => {}
        }
        self.generation = Generation::Old;
        Ok((self, phase))
    }
}
