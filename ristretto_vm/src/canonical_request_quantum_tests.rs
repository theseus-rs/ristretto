use super::*;
use std::collections::HashMap;
type Result<T> = std::result::Result<T, Error>;
use std::cell::Cell;
use std::future::Future;
use std::task::{Context, Waker};

/// Check each individual request's resume bound, including completion on either
/// side of a suspension and exhaustion before/at/after a quantum boundary.
#[test]
fn pending_requests_have_bounded_resumes_and_unchanged_total_limits() -> Result<()> {
    let configuration = Configuration {
        objects: 2,
        roots: 4,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    };
    for (limit, ready_at) in [
        (0, None),
        (1, None),
        (16, None),
        (17, None),
        (18, None),
        (65, None),
        (1, Some(1)),
        (65, Some(2)),
        (65, Some(17)),
        (65, Some(18)),
        (65, Some(65)),
    ] {
        let budget = Budget::new(configuration.charged_bytes()?);
        let collector = GarbageCollector::new();
        let runtime = Runtime::new(&collector, configuration, &budget)?;
        let mut helper = runtime.helper()?;
        let mut report = Report::default();
        let calls = Cell::new(0);
        let mut future = Box::pin(complete(&mut helper, &mut report, limit, || {
            let count = calls.get() + 1;
            calls.set(count);
            Ok(if Some(count) == ready_at {
                Poll::Ready(count)
            } else {
                Poll::Pending
            })
        }));
        let mut context = Context::from_waker(Waker::noop());
        let mut resumes = 0;
        let outcome = loop {
            let before = calls.get();
            let result = future.as_mut().poll(&mut context);
            let work = calls.get() - before;
            assert!(work <= 16, "more than 16 request steps in one resume");
            resumes += 1;
            assert!(resumes <= limit + 2, "request failed to terminate");
            match result {
                Poll::Ready(result) => break result,
                Poll::Pending if resumes == 1 => assert_eq!(calls.get(), 1),
                Poll::Pending => assert_eq!(work, 16),
            }
        };
        drop(future);
        if let Some(expected) = ready_at {
            assert_eq!(outcome?, expected);
            assert_eq!(calls.get(), expected);
            assert_eq!(report.helper_polls, expected - 1);
        } else {
            assert!(matches!(outcome, Err(Error::Limit("request poll"))));
            assert_eq!(calls.get(), limit);
            assert_eq!(report.helper_polls, limit);
        }
        drop(helper);
        drop(runtime);
        assert_eq!(collector.managed_bytes(), 0);
        assert_eq!(budget.used(), 0);
    }
    Ok(())
}

#[test]
fn request_errors_return_immediately_without_an_extra_helper_step() -> Result<()> {
    let configuration = Configuration {
        objects: 2,
        roots: 4,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    };
    for fail_at in [1, 17, 18] {
        let budget = Budget::new(configuration.charged_bytes()?);
        let collector = GarbageCollector::new();
        let runtime = Runtime::new(&collector, configuration, &budget)?;
        let mut helper = runtime.helper()?;
        let mut report = Report::default();
        let calls = Cell::new(0);
        let mut future = Box::pin(complete::<()>(&mut helper, &mut report, 65, || {
            let count = calls.get() + 1;
            calls.set(count);
            if count == fail_at {
                Err(ristretto_gc::canonical::Error::Cancelled)
            } else {
                Ok(Poll::Pending)
            }
        }));
        let mut context = Context::from_waker(Waker::noop());
        let outcome = loop {
            let before = calls.get();
            let result = future.as_mut().poll(&mut context);
            assert!(calls.get() - before <= 16);
            assert!(calls.get() <= fail_at);
            if let Poll::Ready(result) = result {
                break result;
            }
            assert!(calls.get() < fail_at, "error must not suspend");
        };
        drop(future);
        assert!(matches!(
            outcome,
            Err(Error::Collector(ristretto_gc::canonical::Error::Cancelled))
        ));
        assert_eq!(calls.get(), fail_at);
        assert_eq!(report.helper_polls, fail_at - 1);
        drop(helper);
        drop(runtime);
        assert_eq!(collector.managed_bytes(), 0);
        assert_eq!(budget.used(), 0);
    }
    Ok(())
}

/// Cancel actual publications after the initial handoff and after later batched
/// work. The helper must reclaim payload backing and release catalog credits.
#[tokio::test]
async fn cancelling_batched_publication_releases_payload_and_catalog() -> Result<()> {
    let program = Program {
        classes: Vec::new(),
        names: HashMap::new(),
    };
    let configuration = Configuration {
        objects: 4,
        roots: 8,
        mutators: 1,
        helpers: 1,
        edges_per_object: 8,
    };
    for resumes in [1, 2, 4] {
        let budget = Budget::new(configuration.charged_bytes()?);
        let collector = GarbageCollector::new();
        let runtime = Runtime::new(&collector, configuration, &budget)?;
        let mut execution = Execution {
            program: &program,
            runtime: &runtime,
            mutator: runtime.mutator()?,
            helper: runtime.helper()?,
            limits: Limits::default(),
            report: Report::default(),
            recording: Recording::default(),
            statics: States::default(),
        };
        let mut future = Box::pin(execution.allocate_reference_array(Component::Object, 8));
        let mut context = Context::from_waker(Waker::noop());
        for _ in 0..resumes {
            assert!(future.as_mut().poll(&mut context).is_pending());
        }
        assert!(collector.managed_bytes() > 0);
        drop(future);
        execution.cleanup(&collector).await?;
        assert_eq!(collector.managed_bytes(), 0);
        drop(execution);
        drop(runtime);
        assert_eq!(budget.used(), 0);
    }
    Ok(())
}
