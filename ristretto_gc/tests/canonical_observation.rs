//! Finite component correctness checks; no timing or measurement boundary binding.
#![cfg(feature = "experimental-canonical-runtime")]
#![expect(
    clippy::panic_in_result_fn,
    reason = "assert observation and ownership invariants"
)]
use ristretto_gc::canonical::{Budget, Configuration, PrimaryCycles, Runtime};
use ristretto_gc::{GarbageCollector, Trace};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::Poll;

struct Value(Arc<AtomicUsize>);
#[expect(unsafe_code, reason = "the fixture contains no managed references")]
impl Trace for Value {
    fn canonical_payload() -> Option<ristretto_gc::CanonicalPayload<Self>> {
        // SAFETY: only primitive data and a host counter; no managed edges.
        Some(unsafe { ristretto_gc::CanonicalPayload::new() })
    }

    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Value {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}
fn config() -> Configuration {
    Configuration {
        objects: 1,
        roots: 2,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    }
}

#[test]
fn passive_observation_is_not_collection_help_or_reclamation_count()
-> Result<(), Box<dyn std::error::Error>> {
    let gc = GarbageCollector::new();
    let budget = Budget::new(config().charged_bytes()?);
    let runtime = Runtime::new(&gc, config(), &budget)?;
    let mut worker = runtime.mutator()?;
    let initial = PrimaryCycles { lower: 0, upper: 0 };
    for _ in 0..8 {
        assert_eq!(worker.primary_cycles()?, initial);
    }
    let mut helper = runtime.helper()?;
    let drops = Arc::new(AtomicUsize::new(0));
    let mut request = worker.allocate(Value(drops.clone()))?;
    let mut root = None;
    for _ in 0..5000 {
        if let Poll::Ready(value) = request.poll()? {
            root = Some(value);
            break;
        }
        helper.poll()?;
    }
    drop(request);
    let root = root.ok_or("allocation watchdog")?;
    let start = worker.primary_cycles()?;
    let mut completed = false;
    for _ in 0..10000 {
        helper.poll()?;
        let observation = worker.primary_cycles()?;
        assert!(observation.lower <= observation.upper);
        assert!(observation.upper >= start.lower);
        if observation.lower >= start.upper + 2 {
            completed = true;
            break;
        }
    }
    assert!(completed, "full-cycle watchdog");
    assert_eq!(
        drops.load(Ordering::Acquire),
        0,
        "completed cycles do not imply root reclamation"
    );
    assert!(gc.managed_bytes() > 0);
    drop(root);
    for _ in 0..10000 {
        helper.poll()?;
        if gc.managed_bytes() == 0 {
            break;
        }
    }
    assert_eq!(drops.load(Ordering::Acquire), 1);
    assert_eq!(gc.managed_bytes(), 0);
    let numbers = worker.primary_cycles()?;
    drop((worker, helper, runtime));
    assert_eq!(
        budget.used(),
        0,
        "copied bounds must retain no catalog ownership"
    );
    assert!(numbers.lower >= 2);
    Ok(())
}

#[test]
fn passive_observation_after_request_cancellation_preserves_credit_teardown()
-> Result<(), Box<dyn std::error::Error>> {
    let gc = GarbageCollector::new();
    let budget = Budget::new(config().charged_bytes()?);
    let runtime = Runtime::new(&gc, config(), &budget)?;
    let mut worker = runtime.mutator()?;
    let drops = Arc::new(AtomicUsize::new(0));
    let mut request = worker.allocate(Value(drops.clone()))?;
    assert!(request.poll()?.is_pending());
    drop(request);
    let bounds = worker.primary_cycles()?;
    assert_eq!(bounds, PrimaryCycles { lower: 0, upper: 0 });
    drop((worker, runtime));
    assert_eq!(drops.load(Ordering::Acquire), 1);
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(budget.used(), 0);
    Ok(())
}
