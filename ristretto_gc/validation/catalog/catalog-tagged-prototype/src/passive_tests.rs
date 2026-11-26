use super::*;

#[test]
fn passive_observation_stable_and_single_acquisition_failure() {
    state::observation_tests::passive_observation_stable_and_single_acquisition_failure();
}

#[test]
fn passive_observation_phase_decisions_are_copied_without_help() {
    state::observation_tests::passive_observation_phase_decisions_are_copied_without_help();
}

#[test]
fn passive_observation_busy_never_certifies_effect_completion_or_enters_record() {
    state::observation_tests::passive_observation_busy_never_certifies_effect_completion_or_enters_record();
}

#[test]
fn passive_observation_parked_busy_owner_releases_transitive_record_on_return() {
    state::observation_tests::passive_observation_parked_busy_owner_releases_transitive_record_on_return();
}

#[test]
fn passive_observation_idle_gate_precedes_head_acquisition() {
    use crate::bounded::{BoundedCatalog, Capacities, Command, RequestError};
    let catalog = BoundedCatalog::new(Capacities::derive(1, 1, 1).unwrap()).unwrap();
    let mut participant = catalog.register_mutator().unwrap();
    participant.submit(Command::Advance { domain: 0 }).unwrap();
    FAIL_LOAD.with(|fail| fail.set(Some(0)));
    assert_eq!(
        participant.possible_phase_outcomes(),
        Err(RequestError::Busy)
    );
    assert_eq!(FAIL_LOAD.with(|fail| fail.get()), Some(0));
    assert!(participant.cancel());
    assert_eq!(
        participant.possible_phase_outcomes(),
        Err(RequestError::Contended)
    );
    assert_eq!(FAIL_LOAD.with(|fail| fail.get()), None);
    assert!(participant.possible_phase_outcomes().is_ok());
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn passive_observation_parked_participant_does_not_prevent_phase_progress() {
    use super::state::observation_tests::AFTER_ACQUIRE;
    use crate::bounded::{BoundedCatalog, Capacities, Command, Progress};
    use std::sync::mpsc::channel;
    use std::time::Duration;
    let catalog = BoundedCatalog::new(Capacities::derive(1, 1, 1).unwrap()).unwrap();
    let mut participant = catalog.register_mutator().unwrap();
    let mut helper = catalog.register_helper().unwrap();
    let (acquired_tx, acquired_rx) = channel();
    let (finished_tx, finished_rx) = channel();
    std::thread::scope(|scope| {
        let observer = scope.spawn(move || {
            AFTER_ACQUIRE.with(|slot| {
                *slot.borrow_mut() = Some(Box::new(move || {
                    acquired_tx.send(()).unwrap();
                    finished_rx.recv_timeout(Duration::from_secs(30)).unwrap();
                }))
            });
            participant.possible_phase_outcomes().unwrap()
        });
        acquired_rx.recv_timeout(Duration::from_secs(30)).unwrap();
        for _ in 0..24 {
            let phase = helper.phase_in(&()).unwrap();
            let domain = phase.domains[0];
            let command = if domain.mode == DomainMode::Inactive {
                Command::Start {
                    domain: 0,
                    mode: DomainMode::Full,
                }
            } else if domain.floor == RETIREMENT_FLOOR {
                Command::Finish { domain: 0 }
            } else {
                Command::Advance { domain: 0 }
            };
            helper.submit(command).unwrap();
            assert!((0..256).any(|_| match helper.poll() {
                Progress::Pending => false,
                Progress::Committed => true,
                other => panic!("phase progress failed: {other:?}"),
            }));
        }
        let latest = helper.possible_phase_outcomes().unwrap();
        assert!(latest[0].domains[0].epoch > 1);
        finished_tx.send(()).unwrap();
        let original = observer.join().unwrap();
        assert_eq!(original[0].domains[0].epoch, 1);
        assert_eq!(original[0], original[1]);
    });
    assert_eq!(catalog.registered(), (0, 1));
}
