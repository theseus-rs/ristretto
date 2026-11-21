//! Exercise only the public opt-in API, as a separate caller crate.
#![cfg(feature = "experimental-canonical-runtime")]
#![expect(
    clippy::panic_in_result_fn,
    reason = "assert external ownership and accounting invariants"
)]
use ristretto_gc::canonical::{
    Budget, Configuration, Error, Helper, Mutator, RegisterError, Root, Runtime,
};
use ristretto_gc::{GarbageCollector, Trace};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::Poll;

struct Value {
    id: u64,
    drops: Arc<AtomicUsize>,
}
#[expect(unsafe_code, reason = "fixture payload has no managed references")]
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
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}
fn config() -> Configuration {
    Configuration {
        objects: 4,
        roots: 8,
        mutators: 2,
        helpers: 1,
        edges_per_object: 2,
    }
}
fn value(id: u64, drops: &Arc<AtomicUsize>) -> Value {
    Value {
        id,
        drops: drops.clone(),
    }
}
fn allocate<T: Trace + Send + Sync + 'static>(
    worker: &mut Mutator<'_>,
    helper: &mut Helper<'_>,
    value: T,
) -> Result<Root<T>, Box<dyn std::error::Error>> {
    let mut request = worker.allocate(value)?;
    for _ in 0..50_000 {
        if let Poll::Ready(root) = request.poll()? {
            return Ok(root);
        }
        helper.poll()?;
    }
    Err("external allocation watchdog".into())
}
fn write<S, T>(
    worker: &mut Mutator<'_>,
    helper: &mut Helper<'_>,
    source: &Root<S>,
    target: Option<&Root<T>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut request = worker.write(source, 0, target)?;
    for _ in 0..50_000 {
        if request.poll()?.is_ready() {
            return Ok(());
        }
        helper.poll()?;
    }
    Err("external write watchdog".into())
}
fn child<S, T: 'static>(
    worker: &mut Mutator<'_>,
    helper: &mut Helper<'_>,
    source: &Root<S>,
) -> Result<Option<Root<T>>, Box<dyn std::error::Error>> {
    let mut request = worker.child(source, 0)?;
    for _ in 0..50_000 {
        if let Poll::Ready(value) = request.poll()? {
            return Ok(value);
        }
        helper.poll()?;
    }
    Err("external read watchdog".into())
}
fn collect(
    helper: &mut Helper<'_>,
    drops: &AtomicUsize,
    expected: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..100_000 {
        helper.poll()?;
        if drops.load(Ordering::Relaxed) == expected {
            return Ok(());
        }
    }
    Err("external destruction watchdog".into())
}

#[test]
fn external_cycle_and_independent_root_identity() -> Result<(), Box<dyn std::error::Error>> {
    let gc = GarbageCollector::new();
    let charge = config().charged_bytes()?;
    let budget = Budget::new(charge);
    let runtime = Runtime::new(&gc, config(), &budget)?;
    let mut worker = runtime.mutator()?;
    let mut helper = runtime.helper()?;
    let drops = Arc::new(AtomicUsize::new(0));
    let a = allocate(&mut worker, &mut helper, value(11, &drops))?;
    let b = allocate(&mut worker, &mut helper, value(22, &drops))?;
    let alias = a.clone();
    assert!(alias.same_object(&a));
    assert!(!a.same_object(&b));
    write(&mut worker, &mut helper, &a, Some(&b))?;
    write(&mut worker, &mut helper, &b, Some(&a))?;
    let independent: Root<Value> = child(&mut worker, &mut helper, &a)?.ok_or("missing child")?;
    assert!(independent.same_object(&b));
    assert_eq!(runtime.borrow(&independent)?.id, 22);
    drop((a, b, alias));
    for _ in 0..2_000 {
        helper.poll()?;
    }
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    let parent: Root<Value> =
        child(&mut worker, &mut helper, &independent)?.ok_or("missing cycle parent")?;
    assert_eq!(runtime.borrow(&parent)?.id, 11);
    drop((parent, independent));
    collect(&mut helper, &drops, 2)?;
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(budget.used(), charge);
    drop((worker, helper, runtime));
    assert_eq!(budget.used(), 0);
    Ok(())
}

#[test]
fn full_root_quota_read_observes_later_null() -> Result<(), Box<dyn std::error::Error>> {
    let gc = GarbageCollector::new();
    let configuration = Configuration {
        roots: 2,
        ..config()
    };
    let budget = Budget::new(configuration.charged_bytes()?);
    let runtime = Runtime::new(&gc, configuration, &budget)?;
    let mut reader = runtime.mutator()?;
    let mut writer = runtime.mutator()?;
    let mut helper = runtime.helper()?;
    let drops = Arc::new(AtomicUsize::new(0));
    let a = allocate(&mut reader, &mut helper, value(1, &drops))?;
    let b = allocate(&mut reader, &mut helper, value(2, &drops))?;
    write(&mut writer, &mut helper, &a, Some(&b))?;
    let mut request = reader.child::<Value, Value>(&a, 0)?;
    for _ in 0..300 {
        assert!(request.poll()?.is_pending());
        helper.poll()?;
    }
    write::<Value, Value>(&mut writer, &mut helper, &a, None)?;
    let mut finished = false;
    for _ in 0..50_000 {
        if let Poll::Ready(child) = request.poll()? {
            assert!(child.is_none());
            finished = true;
            break;
        }
        helper.poll()?;
    }
    assert!(finished, "full quota null read did not finish");
    assert!(matches!(request.poll(), Err(Error::Finished)));
    drop(request);
    drop((a, b));
    collect(&mut helper, &drops, 2)?;
    drop((reader, writer, helper, runtime));
    assert_eq!(budget.used(), 0);
    Ok(())
}

#[test]
fn cancelled_public_requests_release_real_values_and_allow_reuse()
-> Result<(), Box<dyn std::error::Error>> {
    let gc = GarbageCollector::new();
    let budget = Budget::new(config().charged_bytes()?);
    let runtime = Runtime::new(&gc, config(), &budget)?;
    let mut worker = runtime.mutator()?;
    let mut helper = runtime.helper()?;
    let drops = Arc::new(AtomicUsize::new(0));
    for (iteration, polls) in [0, 1, 7, 23, 47, 95].into_iter().enumerate() {
        let mut request = worker.allocate(value(3, &drops))?;
        for _ in 0..polls {
            if let Poll::Ready(root) = request.poll()? {
                drop(root);
                break;
            }
            helper.poll()?;
        }
        drop(request);
        collect(&mut helper, &drops, iteration * 2 + 1)?;
        let root = allocate(&mut worker, &mut helper, value(4, &drops))?;
        assert_eq!(runtime.borrow(&root)?.id, 4);
        drop(root);
        collect(&mut helper, &drops, iteration * 2 + 2)?;
    }
    drop((worker, helper, runtime));
    assert_eq!(budget.used(), 0);
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn public_admission_type_index_and_foreign_runtime_errors() -> Result<(), Box<dyn std::error::Error>>
{
    struct Opaque;
    impl Trace for Opaque {
        fn trace(&self, _: &GarbageCollector) {}
    }
    let gc = GarbageCollector::new();
    let budget = Budget::new(config().charged_bytes()? * 2);
    let runtime = Runtime::new(&gc, config(), &budget)?;
    let foreign = Runtime::new(&gc, config(), &budget)?;
    let mut worker = runtime.mutator()?;
    let mut other = foreign.mutator()?;
    let mut helper = runtime.helper()?;
    let mut other_helper = foreign.helper()?;
    let drops = Arc::new(AtomicUsize::new(0));
    assert!(matches!(
        worker.allocate(Opaque),
        Err(Error::UnsupportedPayload)
    ));
    let a = allocate(&mut worker, &mut helper, value(5, &drops))?;
    let b = allocate(&mut worker, &mut helper, 77_u64)?;
    let foreign_root = allocate(&mut other, &mut other_helper, 77_u64)?;
    assert!(!b.same_object(&foreign_root));
    assert!(matches!(foreign.borrow(&a), Err(Error::ForeignRuntime)));
    assert!(matches!(
        worker.write(&a, 0, Some(&foreign_root)),
        Err(Error::ForeignRuntime)
    ));
    assert!(matches!(
        worker.child::<Value, Value>(&a, 2),
        Err(Error::Index)
    ));
    assert!(matches!(worker.write(&a, 2, Some(&a)), Err(Error::Index)));
    write(&mut worker, &mut helper, &a, Some(&b))?;
    let mut wrong = worker.child::<Value, Value>(&a, 0)?;
    let mut rejected = false;
    for _ in 0..50_000 {
        match wrong.poll() {
            Err(Error::WrongType) => {
                rejected = true;
                break;
            }
            Ok(Poll::Pending) => {
                helper.poll()?;
            }
            other => return Err(format!("wrong-type read unexpectedly returned {other:?}").into()),
        }
    }
    assert!(rejected, "wrong-type read did not reject");
    drop(wrong);
    let right: Root<u64> = child(&mut worker, &mut helper, &a)?.ok_or("missing typed child")?;
    assert!(right.same_object(&b));
    assert_eq!(*runtime.borrow(&right)?, 77);
    drop((right, a, b, foreign_root));
    drop((worker, helper, runtime, other, other_helper, foreign));
    assert_eq!(budget.used(), 0);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn public_configuration_quota_and_final_alias_credit() -> Result<(), Box<dyn std::error::Error>> {
    let gc = GarbageCollector::new();
    for invalid in [
        Configuration {
            objects: 0,
            ..config()
        },
        Configuration {
            helpers: 0,
            ..config()
        },
        Configuration {
            roots: 0,
            ..config()
        },
        Configuration {
            edges_per_object: 0,
            ..config()
        },
        Configuration {
            edges_per_object: 9,
            ..config()
        },
        Configuration {
            objects: usize::MAX,
            ..config()
        },
    ] {
        assert!(invalid.charged_bytes().is_err());
        let budget = Budget::new(usize::MAX);
        assert!(Runtime::new(&gc, invalid, &budget).is_err());
        assert_eq!(budget.used(), 0);
    }
    let charge = config().charged_bytes()?;
    let insufficient = Budget::new(charge - 1);
    assert!(matches!(
        Runtime::new(&gc, config(), &insufficient),
        Err(Error::Configuration(_))
    ));
    assert_eq!(insufficient.used(), 0);
    let budget = Budget::new(charge);
    let runtime = Runtime::new(&gc, config(), &budget)?;
    let mut worker = runtime.mutator()?;
    let second = runtime.mutator()?;
    let mut helper = runtime.helper()?;
    assert_eq!(runtime.registered(), (2, 1));
    assert!(matches!(
        runtime.mutator(),
        Err(Error::Registration(RegisterError::Limit))
    ));
    assert!(matches!(
        runtime.helper(),
        Err(Error::Registration(RegisterError::Limit))
    ));
    let drops = Arc::new(AtomicUsize::new(0));
    let root = allocate(&mut worker, &mut helper, value(9, &drops))?;
    let alias = root.clone();
    drop((runtime, worker, second, helper, root));
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    assert_eq!(budget.used(), charge);
    drop(alias);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(budget.used(), 0);
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn public_aliases_cross_threads_while_helper_polls() -> Result<(), Box<dyn std::error::Error>> {
    let gc = GarbageCollector::new();
    let budget = Budget::new(config().charged_bytes()?);
    let runtime = Runtime::new(&gc, config(), &budget)?;
    let mut worker = runtime.mutator()?;
    let mut helper = runtime.helper()?;
    let drops = Arc::new(AtomicUsize::new(0));
    let root = allocate(&mut worker, &mut helper, value(10, &drops))?;
    std::thread::scope(|scope| -> Result<(), Box<dyn std::error::Error>> {
        for _ in 0..3 {
            let root = &root;
            let runtime = &runtime;
            scope.spawn(move || {
                for _ in 0..100 {
                    let alias = root.clone();
                    assert!(alias.same_object(root));
                    assert!(runtime.borrow(&alias).is_ok_and(|view| view.id == 10));
                }
            });
        }
        for _ in 0..1_000 {
            helper.poll()?;
        }
        Ok(())
    })?;
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(root);
    collect(&mut helper, &drops, 1)?;
    drop((worker, helper, runtime));
    assert_eq!(budget.used(), 0);
    Ok(())
}
