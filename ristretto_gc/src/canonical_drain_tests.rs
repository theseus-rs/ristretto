use super::*;

fn configuration() -> Configuration {
    Configuration {
        objects: 2,
        roots: 4,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    }
}

fn allocate(runtime: &Runtime<'_>, value: u64) -> Root<u64> {
    let mut worker = runtime.mutator().unwrap();
    let mut request = worker.allocate(value).unwrap();
    for _ in 0..10_000 {
        if let Poll::Ready(root) = request.poll().unwrap() {
            return root;
        }
    }
    panic!("allocation did not complete");
}

fn finish(runtime: &mut Runtime<'_>) -> usize {
    let mut drain = runtime.drain().unwrap();
    for polls in 1..=100_000 {
        if drain.poll().unwrap().1 {
            assert!(matches!(drain.poll(), Err(Error::Finished)));
            return polls;
        }
    }
    panic!("drain did not complete");
}

#[test]
fn drain_has_one_cell_observation_per_step_and_excludes_participants() {
    let collector = GarbageCollector::new();
    let config = configuration();
    let budget = Budget::new(config.charged_bytes().unwrap());
    let mut runtime = Runtime::new(&collector, config, &budget).unwrap();
    let mutator = runtime.mutator().unwrap();
    assert!(matches!(
        runtime.drain(),
        Err(Error::Request(RequestError::Busy))
    ));
    drop(mutator);
    let helper = runtime.helper().unwrap();
    assert!(matches!(
        runtime.drain(),
        Err(Error::Request(RequestError::Busy))
    ));
    drop(helper);
    let cells = config.capacities().unwrap().pools().2;
    assert_eq!(finish(&mut runtime), cells);
    assert_eq!(runtime.registered(), (0, 0));
    drop(runtime);
    assert_eq!(budget.used(), 0);
}

#[test]
fn drain_is_owner_local_and_cannot_certify_a_retained_root() {
    let collector = GarbageCollector::new();
    let ordinary = crate::Gc::new(&collector, 97_u64).into_rooted();
    let ordinary_bytes = collector.managed_bytes();
    let budget = Budget::new(usize::MAX);
    let mut first = Runtime::new(&collector, configuration(), &budget).unwrap();
    let mut second = Runtime::new(&collector, configuration(), &budget).unwrap();
    let first_root = allocate(&first, 11);
    let second_root = allocate(&second, 23);
    let mut retained = first.drain().unwrap();
    for _ in 0..512 {
        assert!(!retained.poll().unwrap().1);
    }
    drop(retained);
    assert_eq!(first.registered(), (0, 0));
    drop(first_root);
    finish(&mut first);
    assert_eq!(*second.borrow(&second_root).unwrap(), 23);
    assert_eq!(*ordinary, 97);
    assert_eq!(collector.managed_bytes(), ordinary_bytes + size_of::<u64>());
    drop(second_root);
    finish(&mut second);
    assert_eq!(collector.managed_bytes(), ordinary_bytes);
    drop((first, second));
    assert_eq!(budget.used(), 0);
}

#[test]
fn cancelled_request_and_drain_prefixes_refund_only_their_owner() {
    let collector = GarbageCollector::new();
    let ordinary = crate::Gc::new(&collector, 97_u64).into_rooted();
    let ordinary_bytes = collector.managed_bytes();
    for prefix in 0..32 {
        let budget = Budget::new(configuration().charged_bytes().unwrap());
        let mut runtime = Runtime::new(&collector, configuration(), &budget).unwrap();
        let mut worker = runtime.mutator().unwrap();
        let mut request = worker.allocate(11_u64).unwrap();
        let mut returned = None;
        for _ in 0..prefix {
            if let Poll::Ready(root) = request.poll().unwrap() {
                returned = Some(root);
                break;
            }
        }
        drop(request);
        drop((returned, worker));
        let mut drain = runtime.drain().unwrap();
        for _ in 0..prefix {
            if drain.poll().unwrap().1 {
                break;
            }
        }
        drop(drain);
        assert_eq!(runtime.registered(), (0, 0));
        finish(&mut runtime);
        drop(runtime);
        assert_eq!(collector.managed_bytes(), ordinary_bytes, "prefix {prefix}");
        assert_eq!(budget.used(), 0, "prefix {prefix}");
        assert_eq!(*ordinary, 97);
    }
}
