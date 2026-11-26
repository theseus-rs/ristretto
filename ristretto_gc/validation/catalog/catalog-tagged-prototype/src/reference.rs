//! Copied conditional-reference decisions. These grant no root, payload access,
//! or publication certificate; an owning controller must validate them at commit.
use crate::{DomainMode, Membership, Phase, TraceState};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceKind {
    Soft,
    Weak,
    Final,
    Phantom,
    PhantomRetained,
}
impl ReferenceKind {
    pub fn retaining(self) -> bool {
        matches!(self, Self::Final | Self::PhantomRetained)
    }
    fn boundary(self) -> Option<u8> {
        match self {
            Self::Soft | Self::Weak => Some(1),
            Self::Phantom => Some(3),
            Self::Final | Self::PhantomRetained => None,
        }
    }
    pub fn includes_final(self) -> bool {
        matches!(self, Self::Phantom | Self::PhantomRetained)
    }
}

/// Exact identity within one catalog. The eventual operation also checks catalog
/// ownership. An equal slot number alone cannot authorize a replacement target.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObjectKey {
    pub identity: u64,
    pub slot: usize,
}
/// Exact queued-root identity; it is not an object key or an owning capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReferenceRoot {
    pub identity: u64,
    pub slot: usize,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReferenceTarget {
    pub key: ObjectKey,
    pub membership: Membership,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReferenceState {
    pub kind: ReferenceKind,
    pub target: Option<ObjectKey>,
    pub owner: Option<ObjectKey>,
    pub born_after: [u64; 2],
    pub activated: bool,
    pub pending: bool,
    pub queued_root: Option<ReferenceRoot>,
    pub construction_root: Option<ReferenceRoot>,
}
impl ReferenceState {
    /// Caller must validate target and source-root authority in the same owning
    /// publication operation. This function only computes the proposed state.
    pub fn created(
        kind: ReferenceKind,
        target: ObjectKey,
        owner: Option<ObjectKey>,
        phase: Phase,
    ) -> Self {
        Self {
            kind,
            target: Some(target),
            owner,
            born_after: phase.domains.map(|domain| {
                if domain.mode != DomainMode::Inactive && domain.floor != 0 {
                    domain.epoch
                } else {
                    0
                }
            }),
            activated: false,
            pending: false,
            queued_root: None,
            construction_root: None,
        }
    }
    fn exact_target(self, phase: Phase, observed: Option<ReferenceTarget>) -> Option<TraceState> {
        let observed = observed?;
        if self.target != Some(observed.key) {
            return None;
        }
        let Membership::Tracing(marks) = observed.membership else {
            return None;
        };
        (!marks.logically_retired(phase)).then_some(marks)
    }
    /// Retaining references still require a present, exact, non-retired target.
    /// Birth exemptions and reachability are evaluated independently per domain.
    pub fn upgrade_eligible(self, phase: Phase, observed: Option<ReferenceTarget>) -> bool {
        let Some(target) = self.exact_target(phase, observed) else {
            return false;
        };
        phase.domains.iter().enumerate().all(|(index, domain)| {
            domain.mode == DomainMode::Inactive
                || self.born_after[index] == domain.epoch
                || self
                    .kind
                    .boundary()
                    .is_none_or(|boundary| domain.floor < boundary)
                || target.reference_live(*domain, index, self.kind.includes_final())
        })
    }
    /// A clone receives independent slot state, not a fresh late-floor exemption.
    /// Cleared/ineligible sources remain cleared even if a new slot would be born.
    pub fn cloned_at(self, phase: Phase, target: Option<ReferenceTarget>) -> Self {
        Self {
            kind: self.kind,
            target: self
                .upgrade_eligible(phase, target)
                .then_some(self.target)
                .flatten(),
            owner: None,
            born_after: std::array::from_fn(|index| {
                let epoch = phase.domains[index].epoch;
                if self.born_after[index] == epoch {
                    epoch
                } else {
                    0
                }
            }),
            activated: false,
            pending: false,
            queued_root: None,
            construction_root: None,
        }
    }
    /// Explicit clearing never resets an earlier activation or pending notice.
    pub fn cleared(mut self) -> Self {
        self.target = None;
        self
    }
    pub fn relevant_stage(self, stage: u8) -> bool {
        stage
            == match self.kind {
                ReferenceKind::Soft | ReferenceKind::Weak => 1,
                ReferenceKind::Final => 2,
                ReferenceKind::Phantom => 3,
                ReferenceKind::PhantomRetained => {
                    if self.activated {
                        2
                    } else {
                        3
                    }
                }
            }
    }
}
