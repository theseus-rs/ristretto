use super::*;

#[test]
fn owning_link_failures_preserve_the_installed_generation() {
    let pool = Pool::new(4);
    let first = pool.try_allocate(7).unwrap();
    let expected = first.try_clone().unwrap();
    let link = PooledLink::new(pool.clone(), first);
    FAIL_LOAD.with(|remaining| remaining.set(Some(0)));
    assert!(link.load().is_none());
    assert!(link.points_to(&expected));
    let replacement = pool.try_allocate(9).unwrap();
    assert!(link.compare(&expected, replacement));
    let stale = pool.try_allocate(11).unwrap();
    assert!(!link.compare(&expected, stale));
    assert_eq!(*link.load().unwrap(), 9);
}

#[test]
fn failed_optional_acquisition_is_not_an_empty_link() {
    let pool = Pool::<u64>::new(1);
    let link = OptionalLink::empty(pool.clone());
    assert!(link.load().unwrap().is_none());
    assert!(link.compare(None, Some(pool.try_allocate(7).unwrap())));
    FAIL_LOAD.with(|remaining| remaining.set(Some(0)));
    assert!(link.load().is_err());
    assert!(link.is_occupied());
    assert_eq!(*link.load().unwrap().unwrap(), 7);
}

#[test]
fn stopped_vacancy_preference_expires_without_owning_a_slot() {
    let preference = vacancy::Preference::default();
    assert!(preference.nominate(1));
    preference.offer(3);
    assert_eq!(preference.slot_for(1), Some(3));
    for _ in 1..vacancy::DEFERRALS {
        assert!(preference.defer(3, 2));
    }
    assert!(!preference.defer(3, 2));
    assert_eq!(preference.slot_for(1), None);
    assert!(preference.nominate(2));
    preference.finish(2);
    assert!(preference.nominate(3));
}
