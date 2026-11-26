use crate::reference::{ObjectKey, ReferenceKind as Kind, ReferenceState, ReferenceTarget};
use crate::{
    Domain, DomainMode as Mode, Generation, Membership, Phase, TraceRetention as Retention,
    TraceState,
};
const TARGET: ObjectKey = ObjectKey {
    identity: 7,
    slot: 0,
};
const OWNER: ObjectKey = ObjectKey {
    identity: 8,
    slot: 1,
};
fn phase(floor: u8) -> Phase {
    Phase {
        revision: 4,
        root_cursors: [0, 0],
        reference_cursors: [0; 2],
        domains: [
            Domain {
                mode: Mode::Full,
                epoch: 2,
                floor,
                pending: 0,
            },
            Domain {
                mode: Mode::Inactive,
                epoch: 3,
                floor: 0,
                pending: 0,
            },
        ],
    }
}
fn target(phase: Phase, retention: Option<Retention>, born: bool) -> ReferenceTarget {
    let mut marks = TraceState::unmarked(phase);
    marks.marks[0].retention = retention;
    marks.marks[0].born = born;
    ReferenceTarget {
        key: TARGET,
        membership: Membership::Tracing(marks),
    }
}
fn slot(kind: Kind) -> ReferenceState {
    ReferenceState::created(kind, TARGET, Some(OWNER), phase(0))
}

#[test]
fn conditional_upgrade_boundary_table_uses_earlier_reachability() {
    for kind in [
        Kind::Soft,
        Kind::Weak,
        Kind::Final,
        Kind::Phantom,
        Kind::PhantomRetained,
    ] {
        for floor in 0..=5 {
            for retention in [
                None,
                Some(Retention::Strong),
                Some(Retention::Final),
                Some(Retention::Phantom),
            ] {
                for born in [false, true] {
                    let phase = phase(floor);
                    let observed = target(phase, retention, born);
                    let physically_live = floor != 5 || retention.is_some() || born;
                    let earlier_live = match kind {
                        Kind::Soft | Kind::Weak => {
                            floor == 0 || retention == Some(Retention::Strong)
                        }
                        Kind::Phantom => {
                            floor < 3
                                || matches!(retention, Some(Retention::Strong | Retention::Final))
                        }
                        Kind::Final | Kind::PhantomRetained => true,
                    };
                    assert_eq!(
                        slot(kind).upgrade_eligible(phase, Some(observed)),
                        physically_live && earlier_live,
                        "{kind:?}, floor={floor}, retention={retention:?}, born={born}"
                    );
                }
            }
        }
    }
}

#[test]
fn cloning_does_not_turn_a_closed_weak_boundary_into_a_new_birth() {
    for kind in [Kind::Soft, Kind::Weak, Kind::Phantom] {
        let late = phase(if kind == Kind::Phantom { 3 } else { 1 });
        let observed = target(late, None, false);
        let source = slot(kind);
        assert!(!source.upgrade_eligible(late, Some(observed)));
        let clone = source.cloned_at(late, Some(observed));
        assert_eq!(clone.target, None);
        assert_eq!(clone.owner, None);
        assert_eq!(clone.born_after, [0, 0]);
        // A genuinely new slot has a different, publication-coupled birth rule.
        let newly_created = ReferenceState::created(kind, TARGET, None, late);
        assert!(newly_created.upgrade_eligible(late, Some(observed)));
        let birth_clone = newly_created.cloned_at(late, Some(observed));
        assert_eq!(birth_clone.target, Some(TARGET));
        assert_eq!(birth_clone.born_after, [2, 0]);
        let next = Phase {
            domains: [
                Domain {
                    epoch: 4,
                    ..late.domains[0]
                },
                late.domains[1],
            ],
            ..late
        };
        assert_eq!(
            birth_clone
                .cloned_at(next, Some(target(next, None, false)))
                .target,
            None
        );
    }
}

#[test]
fn another_domain_closure_is_not_bypassed_by_a_local_birth() {
    let mut phase = phase(1);
    phase.domains[1] = Domain {
        mode: Mode::Full,
        epoch: 9,
        floor: 1,
        pending: 0,
    };
    let mut weak = slot(Kind::Weak);
    weak.born_after = [phase.domains[0].epoch, 0];
    let mut observed = target(phase, None, false);
    assert!(!weak.upgrade_eligible(phase, Some(observed)));
    let Membership::Tracing(ref mut marks) = observed.membership else {
        panic!();
    };
    marks.marks[1].retention = Some(Retention::Strong);
    assert!(weak.upgrade_eligible(phase, Some(observed)));
    weak.born_after[0] = 0;
    assert!(!weak.upgrade_eligible(phase, Some(observed)));
}

#[test]
fn exact_identity_retirement_and_explicit_clear_override_births_and_retention_kinds() {
    let phase = phase(5);
    for kind in [
        Kind::Soft,
        Kind::Weak,
        Kind::Final,
        Kind::Phantom,
        Kind::PhantomRetained,
    ] {
        let mut reference = ReferenceState::created(kind, TARGET, Some(OWNER), phase);
        let mut live = target(phase, Some(Retention::Strong), false);
        assert!(reference.upgrade_eligible(phase, Some(live)));
        assert!(!reference.upgrade_eligible(phase, None));
        live.key.identity += 1;
        assert!(!reference.upgrade_eligible(phase, Some(live)));
        live.key = TARGET;
        live.membership = Membership::Retired;
        assert!(!reference.upgrade_eligible(phase, Some(live)));
        live.membership = Membership::Prepared;
        assert!(!reference.upgrade_eligible(phase, Some(live)));
        assert!(!reference.upgrade_eligible(phase, Some(target(phase, None, false))));
        reference.activated = true;
        reference.pending = true;
        let cleared = reference.cleared();
        assert!(cleared.activated && cleared.pending);
        assert_eq!(cleared.owner, Some(OWNER));
        assert!(
            !cleared.upgrade_eligible(phase, Some(target(phase, Some(Retention::Strong), false)))
        );
        let clone = cleared.cloned_at(phase, Some(target(phase, Some(Retention::Strong), false)));
        assert_eq!(clone.target, None);
        assert!(!clone.activated && !clone.pending);
    }
}

#[test]
fn generational_exclusions_do_not_require_a_collected_generation_mark() {
    for (mode, generation) in [
        (Mode::Young, Generation::Old),
        (Mode::Old, Generation::Young),
    ] {
        let mut phase = phase(5);
        phase.domains[0].mode = mode;
        let mut observed = target(phase, None, false);
        let Membership::Tracing(ref mut marks) = observed.membership else {
            panic!();
        };
        marks.generation = generation;
        assert!(slot(Kind::Weak).upgrade_eligible(phase, Some(observed)));
        phase.domains[1] = Domain {
            mode: Mode::Full,
            epoch: 9,
            floor: 5,
            pending: 0,
        };
        assert!(!slot(Kind::Final).upgrade_eligible(phase, Some(observed)));
    }
}

#[test]
fn retained_phantom_changes_processing_stage_after_activation() {
    for (kind, before, after) in [
        (Kind::Soft, 1, 1),
        (Kind::Weak, 1, 1),
        (Kind::Final, 2, 2),
        (Kind::Phantom, 3, 3),
        (Kind::PhantomRetained, 3, 2),
    ] {
        let mut reference = slot(kind);
        for stage in 0..=5 {
            assert_eq!(reference.relevant_stage(stage), stage == before);
        }
        reference.activated = true;
        for stage in 0..=5 {
            assert_eq!(reference.relevant_stage(stage), stage == after);
        }
    }
}

#[test]
fn phantom_group_flag_survives_same_epoch_shading_and_expires_on_a_new_epoch() {
    let phase = phase(0);
    let mut state = TraceState::unmarked(phase);
    state.marks[0].retention = Some(Retention::Phantom);
    state.marks[0].phantom_decided = true;
    let (state, pending) = state.shade(phase, 0, Retention::Strong).unwrap();
    assert!(state[0].phantom_decided);
    let (state, drained) = state.acknowledge(pending, 0, state[0].ticket).unwrap();
    assert!(state[0].phantom_decided);
    let mut later = drained;
    later.domains[0].epoch += 1;
    let (state, _) = state.shade(later, 0, Retention::Strong).unwrap();
    assert!(!state[0].phantom_decided);
    assert_eq!(state[0].epoch, later.domains[0].epoch);
}
