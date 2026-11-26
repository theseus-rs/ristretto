use super::super::{Budget, Configuration, GarbageCollector, Helper, Report, Runtime, complete};
use super::*;

#[tokio::test]
async fn identity_transactional_publication_and_stale_foreign_tokens() -> Result<(), Error> {
    let collector = GarbageCollector::new();
    let config = Configuration {
        objects: 2,
        roots: 8,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    };
    let budget = Budget::new(config.charged_bytes()?);
    let runtime = Runtime::new(&collector, config, &budget)?;
    let mut mutator = runtime.mutator()?;
    let mut helper = runtime.helper()?;
    let mut report = Report::default();
    let mut roots = Vec::new();
    for class in 0..2 {
        let mut request = mutator.allocate(Payload::object(class))?;
        roots.push(complete(&mut helper, &mut report, 100_000, || request.poll()).await?);
    }
    let a = roots
        .first()
        .ok_or_else(|| Error::Invalid("missing a".to_owned()))?;
    let b = roots
        .get(1)
        .ok_or_else(|| Error::Invalid("missing b".to_owned()))?;
    {
        let mut request = mutator.write(a, 0, Some(b))?;
        complete(&mut helper, &mut report, 100_000, || request.poll()).await?;
    }
    let independent = {
        let mut request = mutator.child::<Payload, Payload>(a, 0)?;
        complete(&mut helper, &mut report, 100_000, || request.poll())
            .await?
            .ok_or_else(|| Error::Invalid("missing child".to_owned()))?
    };
    let mut table = Table::new(2, 2, 4)?;
    let mut first = table.enter()?;
    let mut second = table.enter()?;
    assert!(table.enter().is_err());
    table.install_lease(lease(8).await?)?;
    let token_b = ready(table.intern(&mut first, b)?)?;
    assert_eq!(token_b, ready(table.intern(&mut second, &independent)?)?);
    let token_a = ready(table.intern(&mut second, a)?)?;
    let mut foreign = Table::new(1, 1, 1)?;
    let mut foreign_frame = foreign.enter()?;
    foreign.install_lease(lease(2).await?)?;
    let foreign_token = ready(foreign.intern(&mut foreign_frame, a)?)?;
    let mut object_limited = Table::new(1, 1, 2)?;
    let mut limited_frame = object_limited.enter()?;
    object_limited.install_lease(lease(2).await?)?;
    ready(object_limited.intern(&mut limited_frame, a)?)?;
    assert!(matches!(
        object_limited.intern(&mut limited_frame, b),
        Err(Error::Limit("frame-table objects"))
    ));
    assert!(matches!(
        foreign.intern(&mut foreign_frame, b),
        Err(Error::Limit("frame roots"))
    ));
    assert!(table.publish(&mut foreign_frame, &[]).is_err());
    assert_ne!(foreign_token, token_a);
    assert!(
        table
            .publish(&mut first, &[token_a, foreign_token])
            .is_err()
    );
    assert_eq!(first.roots.len(), 1);
    assert_eq!(first.roots.first().map(|value| value.0), Some(token_b));
    assert!(first.replacement.is_empty());
    assert!(table.publish(&mut first, &[token_a; 5]).is_err());
    let backing = [first.roots.as_ptr(), first.replacement.as_ptr()];
    table.publish(&mut first, &[token_b, token_a, token_b, 0])?;
    assert!(backing.contains(&first.roots.as_ptr()));
    assert!(backing.contains(&first.replacement.as_ptr()));
    table.publish(&mut first, &[token_b])?;
    drop(second);
    assert!(table.resolve(token_a).is_err());
    let mut second = table.enter()?;
    let new_token_a = ready(table.intern(&mut second, a)?)?;
    assert_ne!(new_token_a, token_a);
    assert!(table.publish(&mut first, &[new_token_a, token_a]).is_err());
    assert_eq!(first.roots.first().map(|value| value.0), Some(token_b));
    drop(first);
    assert!(table.resolve(token_b).is_err());
    assert!(table.resolve(-1).is_err());
    assert!(table.resolve(i64::MAX).is_err());
    assert_eq!(table.live_objects(), 1);
    drop((second, foreign_frame, limited_frame, independent, roots));
    assert_eq!(table.live_objects(), 0);
    for _ in 0..100_000 {
        helper.poll()?;
        if collector.managed_bytes() == 0 {
            break;
        }
    }
    assert_eq!(collector.managed_bytes(), 0);
    drop((table, foreign, object_limited, mutator, helper, runtime));
    assert_eq!(budget.used(), 0);
    Ok(())
}

fn ready(value: Poll<i64>) -> Result<i64, Error> {
    match value {
        Poll::Ready(token) => Ok(token),
        Poll::Pending => Err(Error::Boundary("test needs token lease")),
    }
}
async fn lease(amount: u64) -> Result<TokenRange, Error> {
    let amount = NonZeroU64::new(amount).ok_or(Error::Boundary("zero test lease"))?;
    for _ in 0..100_000 {
        match TokenRange::try_acquire(amount) {
            Poll::Ready(Ok(range)) => return Ok(range),
            Poll::Ready(Err(_)) => return Err(Error::Limit("test token identity")),
            Poll::Pending => tokio::task::yield_now().await,
        }
    }
    Err(Error::Limit("test lease attempts"))
}

fn reclaim(helper: &mut Helper<'_>, collector: &GarbageCollector) -> Result<(), Error> {
    for _ in 0..100_000 {
        helper.poll()?;
        if collector.managed_bytes() == 0 {
            return Ok(());
        }
    }
    Err(Error::Limit("test cleanup"))
}

#[tokio::test]
async fn refills_cross_old_identity_boundary_without_authorizing_dead_or_foreign_tokens()
-> Result<(), Error> {
    let collector = GarbageCollector::new();
    let config = Configuration {
        objects: 2,
        roots: 8,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    };
    let budget = Budget::new(config.charged_bytes()?);
    let runtime = Runtime::new(&collector, config, &budget)?;
    let mut worker = runtime.mutator()?;
    let mut helper = runtime.helper()?;
    let mut report = Report::default();
    let a = {
        let mut request = worker.allocate(Payload::object(0))?;
        complete(&mut helper, &mut report, 100_000, || request.poll()).await?
    };
    let b = {
        let mut request = worker.allocate(Payload::object(1))?;
        complete(&mut helper, &mut report, 100_000, || request.poll()).await?
    };
    let table = RefCell::new(Table::with_lease_size(2, 2, 2, NonZeroU64::MIN)?);
    let first = RefCell::new(table.borrow().enter()?);
    let second = RefCell::new(table.borrow().enter()?);
    let token_a = intern_reference(&table, &first, &a, 100_000).await?;
    // No refill is needed to deduplicate an existing identity, even with zero attempts.
    assert_eq!(token_a, intern_reference(&table, &second, &a, 0).await?);
    assert_eq!(table.borrow().leases_installed(), 1);
    // Constant-storage leap, not billions of allocations or an endurance run.
    let _ = lease(u64::from(u32::MAX)).await?;
    let token_b = intern_reference(&table, &second, &b, 100_000).await?;
    assert!(token_b - token_a > i64::from(u32::MAX));
    assert!(table.borrow().resolve(token_a)?.is_some());
    assert!(table.borrow().resolve(token_b)?.is_some());
    let foreign = RefCell::new(Table::with_lease_size(1, 1, 1, NonZeroU64::MIN)?);
    let foreign_frame = RefCell::new(foreign.borrow().enter()?);
    let foreign_token = intern_reference(&foreign, &foreign_frame, &a, 100_000).await?;
    assert!(table.borrow().resolve(foreign_token).is_err());
    assert!(foreign.borrow().resolve(token_a).is_err());
    assert!(
        table
            .borrow()
            .publish(&mut foreign_frame.borrow_mut(), &[])
            .is_err()
    );
    table.borrow().publish(&mut first.borrow_mut(), &[])?;
    table
        .borrow()
        .publish(&mut second.borrow_mut(), &[token_b])?;
    assert!(table.borrow().resolve(token_a).is_err());
    let replacement = intern_reference(&table, &first, &a, 100_000).await?;
    assert_ne!(replacement, token_a);
    assert!(
        table
            .borrow()
            .publish(&mut first.borrow_mut(), &[replacement, token_a])
            .is_err()
    );
    assert!(table.borrow().resolve(replacement)?.is_some());
    assert_eq!(table.borrow().entries.len(), 2);
    assert_eq!(table.borrow().live_objects(), 2);
    drop((first, second, foreign_frame, a, b));
    assert_eq!(table.borrow().live_objects(), 0);
    reclaim(&mut helper, &collector)?;
    drop((table, foreign, worker, helper, runtime));
    assert_eq!(budget.used(), 0);
    Ok(())
}

#[tokio::test]
async fn pending_return_handoff_retains_roots_and_cancellation_needs_no_table_borrow()
-> Result<(), Error> {
    use std::future::Future;
    use std::task::{Context, Waker};
    for cancel in [false, true] {
        let collector = GarbageCollector::new();
        let config = Configuration {
            objects: 1,
            roots: 4,
            mutators: 1,
            helpers: 1,
            edges_per_object: 1,
        };
        let budget = Budget::new(config.charged_bytes()?);
        let runtime = Runtime::new(&collector, config, &budget)?;
        let mut worker = runtime.mutator()?;
        let mut helper = runtime.helper()?;
        let mut report = Report::default();
        let root = {
            let mut request = worker.allocate(Payload::object(0))?;
            complete(&mut helper, &mut report, 100_000, || request.poll()).await?
        };
        let table = RefCell::new(Table::with_lease_size(1, 2, 1, NonZeroU64::MIN)?);
        let returning_frame = RefCell::new(table.borrow().enter()?);
        let receiving_frame = RefCell::new(table.borrow().enter()?);
        let token = intern_reference(&table, &returning_frame, &root, 100_000).await?;
        let result = table
            .borrow()
            .resolve(token)?
            .ok_or(Error::Boundary("missing result root"))?;
        drop(root);
        drop(returning_frame);
        assert!(table.borrow().resolve(token).is_err());
        let calls = Cell::new(0);
        let (table_ref, frame_ref, calls_ref) = (&table, &receiving_frame, &calls);
        let mut future = Box::pin(async move {
            // This future is the sole owner of the returned root until publication succeeds.
            intern_with(table_ref, frame_ref, &result, 100_000, |size| {
                assert!(table_ref.try_borrow_mut().is_ok());
                assert!(frame_ref.try_borrow_mut().is_ok());
                let call = calls_ref.get();
                calls_ref.set(call + 1);
                if call == 0 {
                    Poll::Pending
                } else {
                    TokenRange::try_acquire(size)
                }
            })
            .await
        });
        let mut context = Context::from_waker(Waker::noop());
        assert!(future.as_mut().poll(&mut context).is_pending());
        assert_eq!(calls.get(), 1);
        assert_eq!(table.borrow().live_objects(), 0);
        for _ in 0..64 {
            helper.poll()?;
        }
        assert!(collector.managed_bytes() > 0);
        if cancel {
            let table_guard = table.borrow_mut();
            let frame_guard = receiving_frame.borrow_mut();
            drop(future);
            assert_eq!(table_guard.live_objects(), 0);
            drop((table_guard, frame_guard));
        } else {
            let returned = future.await?;
            assert_ne!(returned, token);
            assert_eq!(table.borrow().live_objects(), 1);
            assert!(table.borrow().resolve(returned)?.is_some());
            table
                .borrow()
                .publish(&mut receiving_frame.borrow_mut(), &[])?;
        }
        drop(receiving_frame);
        reclaim(&mut helper, &collector)?;
        drop((table, worker, helper, runtime));
        assert_eq!(budget.used(), 0);
    }
    Ok(())
}

#[tokio::test]
async fn exhaustion_and_contention_quota_fail_without_publishing_or_retaining_roots()
-> Result<(), Error> {
    let collector = GarbageCollector::new();
    let config = Configuration {
        objects: 1,
        roots: 2,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    };
    let budget = Budget::new(config.charged_bytes()?);
    let runtime = Runtime::new(&collector, config, &budget)?;
    let mut worker = runtime.mutator()?;
    let mut helper = runtime.helper()?;
    let mut report = Report::default();
    let root = {
        let mut request = worker.allocate(Payload::object(0))?;
        complete(&mut helper, &mut report, 100_000, || request.poll()).await?
    };
    let table = RefCell::new(Table::new(1, 1, 1)?);
    let frame = RefCell::new(table.borrow().enter()?);
    assert!(matches!(
        intern_with(&table, &frame, &root, 1, |_| Poll::Ready(Err(
            LeaseError::Exhausted
        )))
        .await,
        Err(Error::Limit("token identity"))
    ));
    let calls = Cell::new(0);
    assert!(matches!(
        intern_with(&table, &frame, &root, 2, |_| {
            calls.set(calls.get() + 1);
            Poll::Pending
        })
        .await,
        Err(Error::Limit("token lease attempts"))
    ));
    assert_eq!(calls.get(), 2);
    assert_eq!(table.borrow().live_objects(), 0);
    assert!(frame.borrow().roots.is_empty());
    let mut range = lease(2).await?;
    let discarded = range.issue().ok_or(Error::Boundary("empty test range"))?;
    table.borrow_mut().install_lease(range)?;
    assert!(table.borrow().resolve(discarded).is_err());
    let token = intern_reference(&table, &frame, &root, 0).await?;
    assert_ne!(token, discarded);
    drop((frame, root));
    reclaim(&mut helper, &collector)?;
    drop((table, worker, helper, runtime));
    assert_eq!(budget.used(), 0);
    Ok(())
}

#[tokio::test]
async fn unchanged_snapshot_keeps_owners_and_still_rejects_stale_foreign_and_excess_cells()
-> Result<(), Error> {
    let collector = GarbageCollector::new();
    let config = Configuration {
        objects: 2,
        roots: 8,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    };
    let budget = Budget::new(config.charged_bytes()?);
    let runtime = Runtime::new(&collector, config, &budget)?;
    let mut worker = runtime.mutator()?;
    let mut helper = runtime.helper()?;
    let mut report = Report::default();
    let a = {
        let mut request = worker.allocate(Payload::object(0))?;
        complete(&mut helper, &mut report, 100_000, || request.poll()).await?
    };
    let b = {
        let mut request = worker.allocate(Payload::object(1))?;
        complete(&mut helper, &mut report, 100_000, || request.poll()).await?
    };
    let mut table = Table::new(2, 1, 4)?;
    table.install_lease(lease(8).await?)?;
    let mut frame = table.enter()?;
    let token_a = ready(table.intern(&mut frame, &a)?)?;
    let token_b = ready(table.intern(&mut frame, &b)?)?;
    let original = frame.roots.as_ptr();
    let before = table.publication_census.get();
    table.publish(&mut frame, &[token_a, 0, token_b])?;
    assert_eq!(
        frame.roots.as_ptr(),
        original,
        "unchanged owners are not rebuilt"
    );
    assert_eq!(table.publication_census.get().rebound, before.rebound);
    table.publish(&mut frame, &[token_b, token_a])?;
    assert_ne!(
        frame.roots.as_ptr(),
        original,
        "changed order takes transactional path"
    );
    table.publish(&mut frame, &[token_b, token_b])?;
    let duplicated = frame.roots.as_ptr();
    assert_eq!(frame.roots.len(), 2);
    let owner = &frame
        .roots
        .first()
        .ok_or(Error::Boundary("missing test owner"))?
        .1;
    assert_eq!(Rc::strong_count(owner), 2);
    table.publish(&mut frame, &[0, token_b, token_b, 0])?;
    assert_eq!(frame.roots.as_ptr(), duplicated);
    assert!(table.publish(&mut frame, &[token_b, -1]).is_err());
    assert_eq!(frame.roots.as_ptr(), duplicated);
    assert_eq!(frame.roots.len(), 2);
    assert!(frame.replacement.is_empty());
    table.publish(&mut frame, &[])?;
    assert!(table.publish(&mut frame, &[0; 5]).is_err());
    assert!(table.resolve(token_a).is_err());
    // An external canonical root does not authorize a retired native token.
    let new_a = ready(table.intern(&mut frame, &a)?)?;
    assert_ne!(new_a, token_a);
    assert!(table.publish(&mut frame, &[token_a]).is_err());
    let mut foreign = Table::new(1, 1, 4)?;
    foreign.install_lease(lease(2).await?)?;
    let mut foreign_frame = foreign.enter()?;
    let foreign_a = ready(foreign.intern(&mut foreign_frame, &a)?)?;
    // This sequence exactly matches its snapshot, but belongs to another table.
    assert!(table.publish(&mut foreign_frame, &[foreign_a]).is_err());
    drop((a, b, frame, foreign_frame));
    assert_eq!(table.live_objects(), 0);
    assert_eq!(foreign.live_objects(), 0);
    reclaim(&mut helper, &collector)?;
    drop((table, foreign, worker, helper, runtime));
    assert_eq!(budget.used(), 0);
    Ok(())
}
