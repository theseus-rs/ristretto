use super::*;

fn worker() -> (CatalogHandle, Participant<(), GuardedCommand>) {
    let catalog = BoundedCatalog::new_in(
        Capacities::derive(1, 2, 1).unwrap(),
        &Budget::new(usize::MAX),
    )
    .unwrap();
    let worker = catalog.register_record_mutator().unwrap();
    (catalog, worker)
}

fn finish(worker: &mut Participant<(), GuardedCommand>, command: GuardedCommand) -> Progress {
    worker.submit(command).unwrap();
    for _ in 0..32 {
        let step = worker.poll_in(&());
        if step != Progress::Pending {
            return step;
        }
    }
    panic!("finite command did not finish")
}

#[test]
fn queued_finish_keeps_swept_epoch_after_another_helper_restarts() {
    let (catalog, mut owner) = worker();
    let mut helper = catalog.register_record_helper().unwrap();
    let epoch = owner.phase_in(&()).unwrap().domains[0].epoch;
    for _ in 0..super::super::RETIREMENT_FLOOR {
        assert_eq!(
            finish(&mut owner, GuardedCommand::Advance { domain: 0 }),
            Progress::Committed
        );
    }
    owner
        .submit(GuardedCommand::FinishEpoch { domain: 0, epoch })
        .unwrap();
    assert_eq!(owner.poll_in(&()), Progress::Pending);
    assert_eq!(
        finish(
            &mut helper,
            GuardedCommand::FinishEpoch { domain: 0, epoch }
        ),
        Progress::Committed
    );
    assert_eq!(
        finish(
            &mut helper,
            GuardedCommand::Start {
                domain: 0,
                mode: super::super::DomainMode::Full
            }
        ),
        Progress::Committed
    );
    for _ in 0..super::super::RETIREMENT_FLOOR {
        assert_eq!(
            finish(&mut helper, GuardedCommand::Advance { domain: 0 }),
            Progress::Committed
        );
    }
    let outcome = (0..32)
        .map(|_| owner.poll_in(&()))
        .find(|p| *p != Progress::Pending);
    assert_eq!(outcome, Some(Progress::Rejected(RequestError::Conflict)));
    assert_eq!(owner.phase_in(&()).unwrap().domains[0].epoch, epoch + 1);
    assert_eq!(
        owner.phase_in(&()).unwrap().domains[0].mode,
        super::super::DomainMode::Full
    );
}

#[test]
fn private_phase_cancellation_has_no_visible_effect() {
    for prefix in 0..5 {
        let (_, mut owner) = worker();
        let before = owner.phase_in(&()).unwrap();
        owner.submit(GuardedCommand::Advance { domain: 0 }).unwrap();
        for _ in 0..prefix {
            assert_eq!(owner.poll_in(&()), Progress::Pending);
        }
        assert!(owner.cancel());
        assert_eq!(owner.phase_in(&()).unwrap(), before);
    }
}

#[test]
fn guarded_dispatch_rejects_foreign_and_invalid_domain_commands() {
    let (_, mut owner) = worker();
    assert_eq!(
        finish(&mut owner, GuardedCommand::Advance { domain: 2 }),
        Progress::Rejected(RequestError::InvalidDomain)
    );
    assert_eq!(
        finish(
            &mut owner,
            GuardedCommand::Start {
                domain: 0,
                mode: super::super::DomainMode::Inactive
            }
        ),
        Progress::Rejected(RequestError::Conflict)
    );
    assert_eq!(
        owner.submit(GuardedCommand::Publish(RecordKey {
            catalog: 0,
            identity: 1,
            slot: 0
        })),
        Err(RequestError::Foreign)
    );
}
