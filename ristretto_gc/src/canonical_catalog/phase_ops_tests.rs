use super::super::{Outcome, share};
use super::*;

fn commit(catalog: &Catalog, mut preparation: Preparation) {
    for _ in 0..4 {
        if let Some(attempt) = preparation.poll(catalog).unwrap() {
            assert_eq!(catalog.commit_in(&attempt, &()), Outcome::Committed);
            return;
        }
    }
    panic!("finite preparation did not finish");
}
fn snapshot(catalog: &Catalog) -> Snapshot {
    catalog.snapshot_in(&()).unwrap()
}
fn create(catalog: &Catalog) -> Pooled<Cell> {
    let mut creation = catalog.begin_create_with(0, 8, ()).unwrap();
    for _ in 0..4 {
        if let Some(cell) = creation.poll(catalog).unwrap() {
            return cell;
        }
    }
    panic!("finite creation did not finish");
}
#[test]
fn phase_finish_cannot_overtake_unscanned_roots_or_pending_work() {
    let catalog = Catalog::with_record_root_capacities_and_limit(1, 32, 32, 32, 1, 32, 32);
    assert!(catalog.begin_advance(snapshot(&catalog), 0).is_none());
    assert!(catalog.begin_finish(snapshot(&catalog), 0).is_none());
    let cell = create(&catalog);
    commit(
        &catalog,
        catalog
            .begin_publish(snapshot(&catalog), share(&cell).unwrap())
            .unwrap(),
    );
    assert!(catalog.begin_advance(snapshot(&catalog), 0).is_none());
    assert_eq!(snapshot(&catalog).phase().domains[0].pending, 1);
}
#[test]
fn stale_finish_epoch_cannot_finish_a_new_collection() {
    let catalog = Catalog::<()>::with_record_capacities_and_limit(1, 32, 32, 32, 32);
    let old_epoch = snapshot(&catalog).phase().domains[0].epoch;
    for _ in 0..RETIREMENT_FLOOR {
        commit(
            &catalog,
            catalog.begin_advance(snapshot(&catalog), 0).unwrap(),
        );
    }
    commit(
        &catalog,
        catalog
            .begin_finish_epoch(snapshot(&catalog), 0, old_epoch)
            .unwrap(),
    );
    commit(
        &catalog,
        catalog
            .begin_start(snapshot(&catalog), 0, DomainMode::Full)
            .unwrap(),
    );
    for _ in 0..RETIREMENT_FLOOR {
        commit(
            &catalog,
            catalog.begin_advance(snapshot(&catalog), 0).unwrap(),
        );
    }
    assert!(
        catalog
            .begin_finish_epoch(snapshot(&catalog), 0, old_epoch)
            .is_none()
    );
    let fresh = snapshot(&catalog).phase().domains[0].epoch;
    assert_eq!(fresh, old_epoch + 1);
    commit(
        &catalog,
        catalog
            .begin_finish_epoch(snapshot(&catalog), 0, fresh)
            .unwrap(),
    );
    assert_eq!(
        snapshot(&catalog).phase().domains[0].mode,
        DomainMode::Inactive
    );
}
#[test]
fn acknowledgement_rejects_stale_ticket_and_settles_exact_work_once() {
    let catalog = Catalog::with_record_capacities_and_limit(1, 32, 32, 32, 32);
    let cell = create(&catalog);
    commit(
        &catalog,
        catalog
            .begin_publish(snapshot(&catalog), share(&cell).unwrap())
            .unwrap(),
    );
    let Membership::Tracing(marks) = cell.membership().unwrap() else {
        panic!("tracing")
    };
    assert!(
        catalog
            .begin_trace_acknowledge(
                snapshot(&catalog),
                share(&cell).unwrap(),
                0,
                marks[0].ticket + 1
            )
            .is_none()
    );
    commit(
        &catalog,
        catalog
            .begin_trace_acknowledge(
                snapshot(&catalog),
                share(&cell).unwrap(),
                0,
                marks[0].ticket,
            )
            .unwrap(),
    );
    assert_eq!(snapshot(&catalog).phase().domains[0].pending, 0);
    assert!(
        catalog
            .begin_trace_acknowledge(
                snapshot(&catalog),
                share(&cell).unwrap(),
                0,
                marks[0].ticket
            )
            .is_none()
    );
}
#[test]
fn invalid_phase_domain_and_mode_have_no_effect() {
    let catalog = Catalog::<()>::with_record_capacities_and_limit(1, 32, 32, 32, 32);
    let before = snapshot(&catalog).phase();
    assert!(catalog.begin_advance(snapshot(&catalog), 2).is_none());
    assert!(
        catalog
            .begin_finish_epoch(snapshot(&catalog), 2, 1)
            .is_none()
    );
    assert!(
        catalog
            .begin_start(snapshot(&catalog), 0, DomainMode::Inactive)
            .is_none()
    );
    assert_eq!(snapshot(&catalog).phase(), before);
}
