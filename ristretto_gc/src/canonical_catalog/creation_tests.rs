use super::*;

#[test]
fn private_and_announced_records_release_the_exact_owner_once() {
    struct Counted(Arc<AtomicUsize>);
    impl Record for Counted {
        type Context = ();
        type Guard<'a> = ();
        fn enter<'a>(&'a self, (): &'a ()) -> Result<Entered<'a, ()>, RecordError> {
            unreachable!("private creation never admits a payload");
        }
    }
    impl Drop for Counted {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    for stop in 0..=3 {
        let drops = Arc::new(AtomicUsize::new(0));
        let catalog = Catalog::<Counted>::with_record_capacities_and_limit(1, 8, 8, 8, 8);
        let mut creation = catalog
            .begin_create_with(0, 8, Counted(drops.clone()))
            .unwrap();
        let mut held = None;
        for _ in 0..stop {
            held = creation.poll(&catalog).unwrap();
        }
        assert_eq!(catalog.discover(0).unwrap().is_some(), stop == 3);
        drop(creation);
        assert_eq!(drops.load(Ordering::Relaxed), usize::from(stop < 3));
        drop(held);
        drop(catalog);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
    }
}

#[test]
fn competing_announcements_keep_the_installed_record() {
    let catalog = Catalog::<()>::with_record_capacities_and_limit(1, 8, 8, 8, 8);
    let mut first = catalog.begin_create_with(0, 8, ()).unwrap();
    let mut other = catalog.begin_create_with(0, 16, ()).unwrap();
    for _ in 0..2 {
        assert!(first.poll(&catalog).unwrap().is_none());
        assert!(other.poll(&catalog).unwrap().is_none());
    }
    let winner = first.poll(&catalog).unwrap().unwrap();
    assert!(matches!(other.poll(&catalog), Err(CreationError::Occupied)));
    let installed = catalog.discover(0).unwrap().unwrap();
    assert!(Owned::ptr_eq(&winner, &installed));
    assert_eq!(installed.address, 8);
}

#[test]
fn discovery_contention_is_not_an_empty_registry_slot() {
    let catalog = Catalog::<()>::with_record_capacities_and_limit(1, 8, 8, 8, 8);
    super::super::FAIL_LOAD.with(|remaining| remaining.set(Some(0)));
    assert!(catalog.discover(0).is_err());
    assert!(catalog.discover(0).unwrap().is_none());
}

#[test]
fn foreign_and_failed_acquisitions_preserve_private_creation() {
    let catalog = Catalog::<()>::with_record_capacities_and_limit(1, 8, 8, 8, 8);
    let foreign = Catalog::<()>::with_record_capacities_and_limit(1, 8, 8, 8, 8);
    let mut creation = catalog.begin_create_with(0, 8, ()).unwrap();
    assert!(matches!(
        creation.poll(&foreign),
        Err(CreationError::Foreign)
    ));
    for _ in 0..2 {
        assert!(creation.poll(&catalog).unwrap().is_none());
    }
    super::super::FAIL_SHARE.with(|remaining| remaining.set(Some(0)));
    assert!(creation.poll(&catalog).unwrap().is_none());
    assert!(catalog.discover(0).unwrap().is_none());
    let installed = creation.poll(&catalog).unwrap().unwrap();
    assert_eq!(installed.address, 8);
    assert!(matches!(
        creation.poll(&catalog),
        Err(CreationError::Finished)
    ));
}
