use super::*;

fn fixture() -> (Catalog<()>, RootToken<()>) {
    let catalog = Catalog::with_record_root_capacities_and_limit(1, 16, 16, 16, 1, 8, 16);
    let mut creation = catalog.begin_create_with(0, 8, ()).unwrap();
    for _ in 0..2 {
        assert!(creation.poll(&catalog).unwrap().is_none());
    }
    let cell = creation.poll(&catalog).unwrap().unwrap();
    let mut root = catalog.begin_create_root(0, cell).unwrap();
    assert!(root.poll(&catalog).unwrap().is_none());
    let token = root.poll(&catalog).unwrap().unwrap();
    (catalog, token)
}
fn attempt(catalog: &Catalog<()>, mut preparation: Preparation<()>) -> super::super::Attempt<()> {
    assert!(preparation.poll(catalog).unwrap().is_none());
    preparation.poll(catalog).unwrap().unwrap()
}

#[test]
fn root_visibility_is_not_a_completed_publication_certificate() {
    let (catalog, token) = fixture();
    assert_eq!(catalog.root_is_stably_live_in(&token, &()), Some(false));
    let proposal = catalog
        .begin_publish_root(
            catalog.snapshot_in(&()).unwrap(),
            token.try_share().unwrap(),
        )
        .unwrap();
    let proposal = attempt(&catalog, proposal);
    catalog.effect_and_stop(&proposal);
    // This is the real helper prefix after membership and root effects, before
    // the completion store. A Live status alone must not authorize access.
    proposal.operation().root.as_ref().unwrap().apply();
    assert_eq!(token.status(), RootStatus::Live);
    assert_eq!(catalog.root_is_stably_live_in(&token, &()), None);
    assert_eq!(catalog.root_is_stably_live_in(&token, &()), Some(true));
    assert_eq!(
        catalog.commit_in(&proposal, &()),
        super::super::Outcome::Committed
    );
}

#[test]
fn cancellation_and_late_slot_release_preserve_unique_root_custody() {
    let (catalog, token) = fixture();
    let retained = token.try_share().unwrap();
    let proposal = catalog
        .begin_cancel_root(
            catalog.snapshot_in(&()).unwrap(),
            token.try_share().unwrap(),
        )
        .unwrap();
    let proposal = attempt(&catalog, proposal);
    assert_eq!(
        catalog.commit_in(&proposal, &()),
        super::super::Outcome::Committed
    );
    assert_eq!(token.status(), RootStatus::Retired);
    assert!(!catalog.release_retired_root(&retained));
    drop(token);
    assert!(catalog.release_retired_root(&retained));
    let cell = catalog.discover(0).unwrap().unwrap();
    let mut creation = catalog.begin_create_root(0, cell).unwrap();
    assert!(creation.poll(&catalog).unwrap().is_none());
    let replacement = creation.poll(&catalog).unwrap().unwrap();
    assert_ne!(replacement.identity, retained.identity);
    assert!(!catalog.release_retired_root(&retained));
    assert_eq!(
        catalog.discover_root(0).unwrap().unwrap().identity,
        replacement.identity
    );
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn competing_root_publication_and_cancellation_have_one_controller_winner() {
    let (catalog, token) = fixture();
    let publish = catalog
        .begin_publish_root(
            catalog.snapshot_in(&()).unwrap(),
            token.try_share().unwrap(),
        )
        .unwrap();
    let publish = attempt(&catalog, publish);
    let cancel = catalog
        .begin_cancel_root(
            catalog.snapshot_in(&()).unwrap(),
            token.try_share().unwrap(),
        )
        .unwrap();
    let cancel = attempt(&catalog, cancel);
    let catalog = Arc::new(catalog);
    let other = catalog.clone();
    let publish = Arc::new(publish);
    let other_publish = publish.clone();
    let worker = std::thread::spawn(move || other.commit_in(&other_publish, &()));
    let _ = catalog.commit_in(&cancel, &());
    let _ = worker.join().unwrap();
    let published = catalog.commit_in(&publish, &()) == super::super::Outcome::Committed;
    let cancelled = catalog.commit_in(&cancel, &()) == super::super::Outcome::Committed;
    assert_ne!(published, cancelled);
    assert_eq!(
        token.status(),
        if published {
            RootStatus::Live
        } else {
            RootStatus::Retired
        }
    );
    assert_eq!(
        catalog.snapshot_in(&()).unwrap().phase().domains[0].pending,
        usize::from(published)
    );
}
