use super::*;

fn domain_phase(modes: [DomainMode; 2], floors: [u8; 2]) -> Phase {
    Phase {
        root_cursors: [0; 2],
        reference_cursors: [0; 2],
        revision: 11,
        domains: std::array::from_fn(|i| Domain {
            mode: modes[i],
            epoch: 7 + i as u64,
            floor: floors[i],
            pending: 0,
        }),
    }
}

#[test]
fn publication_floor_table_distinguishes_births_from_reachability_and_work() {
    for young_mode in [DomainMode::Inactive, DomainMode::Young, DomainMode::Full] {
        for old_mode in [DomainMode::Inactive, DomainMode::Full, DomainMode::Old] {
            for young_floor in 0..=RETIREMENT_FLOOR {
                for old_floor in 0..=RETIREMENT_FLOOR {
                    let modes = [young_mode, old_mode];
                    let floors = [young_floor, old_floor];
                    if (0..2).any(|i| modes[i] == DomainMode::Inactive && floors[i] != 0) {
                        continue;
                    }
                    let before = domain_phase(modes, floors);
                    let (resident, after) = TraceState::publication(before).unwrap();
                    assert_eq!(resident.generation, Generation::Young);
                    assert!(!resident.logically_retired(after));
                    for i in 0..2 {
                        let domain = after.domains[i];
                        let inactive = modes[i] == DomainMode::Inactive;
                        let birth = !inactive
                            && (modes[i] == DomainMode::Old
                                || (i == 1 && floors[i] == RETIREMENT_FLOOR));
                        let needs_trace = !inactive && !birth && floors[i] == 0;
                        assert_eq!(domain.pending, usize::from(needs_trace), "{before:?}");
                        assert_eq!(resident[i].ticket, if needs_trace { 12 } else { 0 });
                        assert_eq!(resident[i].born, birth);
                        assert_eq!(resident[i].marked(domain.epoch), !inactive);
                        assert_eq!(
                            resident[i].marked_before(domain.epoch, true),
                            !inactive && !birth
                        );
                        assert!(!resident[i].marked_before(domain.epoch + 1, true));
                    }
                    assert_eq!(
                        resident.old_birth,
                        if old_mode == DomainMode::Old { 8 } else { 0 }
                    );
                }
            }
        }
    }
}

#[test]
fn generation_exclusions_and_other_domain_retirement_prevent_resurrection() {
    let phase = domain_phase([DomainMode::Young, DomainMode::Old], [RETIREMENT_FLOOR; 2]);
    let mut state = TraceState::unmarked(phase);
    assert_eq!(state.can_retire(phase, 1), Err(TraceError::Conflict));
    assert_eq!(state.can_retire(phase, 0), Ok(()));
    assert!(state.reference_live(phase.domains[1], 1, false));
    state.generation = Generation::Old;
    assert_eq!(state.can_retire(phase, 0), Err(TraceError::Conflict));
    assert_eq!(state.can_retire(phase, 1), Ok(()));
    assert!(state.reference_live(phase.domains[0], 0, false));
    for retiring in 0..2 {
        let mut phase = domain_phase([DomainMode::Full; 2], [0; 2]);
        phase.domains[retiring].floor = RETIREMENT_FLOOR;
        let state = TraceState::unmarked(phase);
        assert_eq!(
            state.shade(phase, 1 - retiring, TraceRetention::Strong),
            Err(TraceError::Conflict)
        );
        assert_eq!(state.promote(phase), Err(TraceError::Conflict));
    }
}

#[test]
fn idle_and_birth_states_cannot_authorize_ordinary_retirement_or_reference_liveness() {
    let idle = domain_phase([DomainMode::Inactive; 2], [0; 2]);
    let (state, _) = TraceState::publication(idle).unwrap();
    for i in 0..2 {
        assert_eq!(
            state.shade(idle, i, TraceRetention::Strong),
            Err(TraceError::Conflict)
        );
        assert_eq!(state.can_retire(idle, i), Err(TraceError::Conflict));
        assert!(!state.reference_live(idle.domains[i], i, true));
    }
    let retiring = domain_phase(
        [DomainMode::Inactive, DomainMode::Full],
        [0, RETIREMENT_FLOOR],
    );
    let (state, phase) = TraceState::publication(retiring).unwrap();
    assert!(state[1].marked(8));
    assert!(!state.reference_live(phase.domains[1], 1, true));
    assert_eq!(state.can_retire(phase, 1), Err(TraceError::Conflict));
}

#[test]
fn initial_young_handoff_is_retained_without_old_queue_and_full_promotion_preserves_strength() {
    let phase = domain_phase([DomainMode::Young, DomainMode::Old], [0; 2]);
    let state = TraceState::unmarked(phase);
    let (retained, after) = state.shade(phase, 1, TraceRetention::Strong).unwrap();
    assert!(retained[1].marked_before(8, false));
    assert_eq!(retained[1].ticket, 0);
    assert_eq!(after.domains.map(|d| d.pending), [0, 0]);
    let phase = domain_phase([DomainMode::Young, DomainMode::Full], [0; 2]);
    let (state, phase) = TraceState::publication(phase).unwrap();
    let (promoted, after) = state.promote(phase).unwrap();
    assert_eq!(promoted.marks, state.marks);
    assert_eq!(after, phase);
    assert_eq!(promoted.generation, Generation::Old);
}

#[test]
fn publication_and_promotion_refuse_count_or_ticket_wrap() {
    let mut phase = domain_phase([DomainMode::Young, DomainMode::Old], [0; 2]);
    phase.domains[0].pending = usize::MAX;
    assert_eq!(TraceState::publication(phase), Err(TraceError::Exhausted));
    phase.domains[0].pending = 0;
    let (state, mut phase) = TraceState::publication(phase).unwrap();
    phase.domains[1].pending = usize::MAX;
    assert_eq!(state.promote(phase), Err(TraceError::Exhausted));
    phase.domains[1].pending = 0;
    phase.revision = u64::MAX;
    assert_eq!(state.promote(phase), Err(TraceError::Exhausted));
    assert_eq!(TraceState::publication(phase), Err(TraceError::Exhausted));
}
