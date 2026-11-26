use super::*;

#[test]
fn participant_roles_have_independent_reusable_quotas() {
    let capacities = Capacities::derive(2, 2, 1).unwrap();
    let budget = Budget::new(usize::MAX);
    let catalog = BoundedCatalog::new_in(capacities, &budget).unwrap();
    let first = catalog.register_mutator().unwrap();
    let second = catalog.register_mutator().unwrap();
    let helper = catalog.register_helper().unwrap();
    assert_eq!(catalog.registered(), (2, 1));
    assert!(matches!(
        catalog.register_mutator(),
        Err(RegisterError::Limit)
    ));
    assert!(matches!(
        catalog.register_helper(),
        Err(RegisterError::Limit)
    ));
    drop(first);
    let replacement = catalog.register_mutator().unwrap();
    drop(helper);
    let helper = catalog.register_helper().unwrap();
    drop((second, replacement, helper));
    assert_eq!(catalog.registered(), (0, 0));
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn rejected_budget_admission_leaves_no_charge_and_last_participant_refunds() {
    let capacities = Capacities::derive(2, 1, 1).unwrap();
    let bytes = Footprint::for_capacities(capacities).unwrap().total();
    let budget = Budget::new(bytes);
    assert!(matches!(
        BoundedCatalog::<()>::with_records_and_storage_in(capacities, &budget, 1),
        Err(ConfigError::BudgetExceeded)
    ));
    assert_eq!(budget.used(), 0);
    let catalog = BoundedCatalog::new_in(capacities, &budget).unwrap();
    assert_eq!(budget.used(), bytes);
    assert!(matches!(
        BoundedCatalog::new_in(capacities, &budget),
        Err(ConfigError::BudgetExceeded)
    ));
    let helper = catalog.register_helper().unwrap();
    drop(catalog);
    assert_eq!(budget.used(), bytes);
    drop(helper);
    assert_eq!(budget.used(), 0);
    drop(BoundedCatalog::new_in(capacities, &budget).unwrap());
    assert_eq!(budget.used(), 0);
}

#[test]
fn invalid_capacities_and_supplement_overflow_do_not_reserve() {
    assert_eq!(Capacities::derive(0, 1, 1), Err(ConfigError::EmptyRegistry));
    assert_eq!(Capacities::derive(1, 1, 0), Err(ConfigError::NoHelpers));
    assert_eq!(
        Capacities::derive(1, usize::MAX, 1),
        Err(ConfigError::CapacityOverflow)
    );
    let capacities = Capacities::derive(1, 1, 1).unwrap();
    let budget = Budget::new(usize::MAX);
    assert!(matches!(
        BoundedCatalog::<()>::with_records_and_storage_in(capacities, &budget, usize::MAX),
        Err(ConfigError::CapacityOverflow)
    ));
    assert_eq!(budget.used(), 0);
}
