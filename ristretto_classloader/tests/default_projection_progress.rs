//! Default VM array projections must not make unrelated reclamation wait for a host guard.
#![expect(clippy::panic_in_result_fn)]
use ristretto_classloader::{Reference, Result, Value};
use ristretto_gc::sync::RwLock;
use ristretto_gc::{Configuration, GarbageCollector, Gc};

fn configuration(bytes: usize) -> Configuration {
    Configuration {
        max_heap_bytes: bytes,
        allocation_threshold: usize::MAX,
        incremental_step_size: 1,
        max_pause_time_us: 0,
        ..Configuration::default()
    }
}

fn bytes(value: i8) -> Reference {
    Reference::ByteArray(vec![value; 128].into())
}

fn finish(gc: &GarbageCollector) -> bool {
    (0..1024).any(|_| gc.step())
}

#[test]
fn default_primitive_projection_preserves_reclaimable_headroom() -> Result<()> {
    let size = Gc::allocation_size(&RwLock::new(bytes(7)))?;
    let gc = GarbageCollector::with_config(configuration(size * 2));
    let owner = Value::try_new_object(&gc, bytes(7))?;
    let garbage = Value::try_new_object(&gc, bytes(9))?;
    assert_eq!(gc.managed_bytes(), size * 2);
    drop(garbage);
    let guard = owner.as_byte_vec_mut()?;
    gc.collect();
    let completed = finish(&gc);
    let reclaimed = gc.managed_bytes() == size;
    assert_eq!(guard.first(), Some(&7));
    drop(guard);
    // Release the protected payload before any assertion can unwind cleanup.
    assert!(completed, "default typed projection blocked mark closure");
    assert!(reclaimed, "unrelated unreachable array was not reclaimed");
    let replacement = Value::try_new_object(&gc, bytes(11))?;
    assert_eq!(replacement.as_byte_vec_ref()?.first(), Some(&11));
    drop((replacement, owner));
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn default_native_mutator_allocates_after_reclamation_while_array_guard_is_parked() -> Result<()> {
    use std::future::Future;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc;
    use std::task::{Context, Poll, Waker};
    use std::time::Duration;

    let size = Gc::allocation_size(&RwLock::new(bytes(7)))?;
    let gc = GarbageCollector::with_config(configuration(size * 2));
    let garbage = Value::try_new_object(&gc, bytes(9))?;
    let (ready_tx, ready_rx) = mpsc::channel();
    let (resume_tx, resume_rx) = mpsc::channel::<()>();
    let released = Arc::new(AtomicBool::new(false));
    let owner_released = released.clone();
    let owner_gc = gc.clone();
    let owner = std::thread::spawn(move || -> Result<Value> {
        let owner = Value::try_new_object(&owner_gc, bytes(7))?;
        let guard = owner.as_byte_vec_mut()?;
        ready_tx.send(()).expect("observer remains live");
        let _ = resume_rx.recv_timeout(Duration::from_secs(if cfg!(miri) { 120 } else { 20 }));
        owner_released.store(true, Ordering::Release);
        assert_eq!(guard.first(), Some(&7));
        drop(guard);
        Ok(owner)
    });
    ready_rx
        .recv_timeout(Duration::from_secs(if cfg!(miri) { 120 } else { 20 }))
        .map_err(|error| {
            ristretto_classloader::Error::InternalError(format!("owner guard: {error}"))
        })?;
    assert_eq!(gc.managed_bytes(), size * 2);
    drop(garbage);
    let mut request = Box::pin(Value::try_new_object_async(&gc, bytes(11)));
    let mut context = Context::from_waker(Waker::noop());
    let mut completed = None;
    for _ in 0..1024 {
        if let Poll::Ready(result) = request.as_mut().poll(&mut context) {
            completed = Some(result);
            break;
        }
    }
    let reclaimed_before_release = gc.statistics()?.managed_bytes_freed >= size;
    let still_parked = !released.load(Ordering::Acquire);
    // Always unblock and join the host guard before inspecting the expected result.
    drop(request);
    drop(resume_tx);
    let owner = owner.join().map_err(|_| {
        ristretto_classloader::Error::InternalError("owner thread panicked".into())
    })??;
    let allocated = completed.expect("unrelated mutator waited for the parked array guard")?;
    assert!(still_parked, "the parked guard's failure deadline expired");
    assert!(
        reclaimed_before_release,
        "allocation did not force reclamation"
    );
    assert_eq!(allocated.as_byte_vec_ref()?.first(), Some(&11));
    drop((allocated, owner));
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

fn object_array(gc: &std::sync::Arc<GarbageCollector>, child: Value) -> Result<Value> {
    use ristretto_classfile::{ClassFile, ConstantPool};
    use ristretto_classloader::{Class, ObjectArray};
    let mut pool = ConstantPool::default();
    let this_class = pool.add_class("[Ljava/lang/Object;")?;
    let class = Class::from(
        None,
        ClassFile {
            constant_pool: pool,
            this_class,
            ..ClassFile::default()
        },
    )?;
    Ok(Value::new_object(
        gc,
        Reference::Array(ObjectArray {
            class,
            elements: vec![child].into(),
        }),
    ))
}

fn weak(value: &Value) -> Result<ristretto_gc::GcWeak<RwLock<Reference>>> {
    let Value::Object(Some(handle)) = value else {
        return Err(ristretto_classloader::Error::InternalError(
            "expected object value".into(),
        ));
    };
    Ok(ristretto_gc::GcWeak::new(
        handle,
        ristretto_gc::ReferenceKind::Weak,
        0,
    ))
}

#[test]
fn default_object_array_projection_retains_current_children_during_mutation() -> Result<()> {
    let gc = GarbageCollector::with_config(configuration(usize::MAX));
    let child = Value::try_new_object(&gc, bytes(7))?;
    let original = weak(&child)?;
    let owner = object_array(&gc, child)?;
    let (_, mut guard) = owner.as_class_vec_mut()?;
    let replacement = Value::try_new_object(&gc, bytes(11))?;
    let current = weak(&replacement)?;
    let escaped = std::mem::replace(
        guard.first_mut().ok_or_else(|| {
            ristretto_classloader::Error::InternalError("missing array element".into())
        })?,
        replacement,
    );
    gc.collect();
    let first = finish(&gc);
    let both_live = original.upgrade().is_some() && current.upgrade().is_some();
    drop(escaped);
    let mut subsequent = true;
    for _ in 0..3 {
        gc.collect();
        subsequent &= finish(&gc);
    }
    let obsolete_reclaimed = original.upgrade().is_none();
    let current_live = current.upgrade().is_some();
    assert_eq!(
        guard
            .first()
            .ok_or_else(|| ristretto_classloader::Error::InternalError(
                "missing array element".into()
            ))?
            .as_byte_vec_ref()?
            .first(),
        Some(&11)
    );
    *guard.first_mut().ok_or_else(|| {
        ristretto_classloader::Error::InternalError("missing array element".into())
    })? = Value::Object(None);
    for _ in 0..3 {
        gc.collect();
        subsequent &= finish(&gc);
    }
    let cleared_reclaimed = current.upgrade().is_none();
    drop(guard);
    assert!(
        first && subsequent,
        "covered object array blocked collection"
    );
    assert!(
        both_live && current_live,
        "current independently owned child was reclaimed"
    );
    assert!(
        obsolete_reclaimed && cleared_reclaimed,
        "obsolete array roots survived mutation"
    );
    drop(owner);
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn cancelling_a_default_object_array_projection_revokes_before_a_later_writer() -> Result<()> {
    use ristretto_gc::{Trace, TraceProgress};
    use std::future::Future;
    use std::task::{Context, Waker};
    let gc = GarbageCollector::with_config(configuration(usize::MAX));
    let child = Value::try_new_object(&gc, bytes(7))?;
    let child_weak = weak(&child)?;
    let owner = object_array(&gc, child)?;
    let mut future = Box::pin(async {
        let (_, guard) = owner.as_class_vec_mut()?;
        std::future::pending::<()>().await;
        drop(guard);
        Ok::<(), ristretto_classloader::Error>(())
    });
    let mut context = Context::from_waker(Waker::noop());
    assert!(future.as_mut().poll(&mut context).is_pending());
    gc.collect();
    let completed = finish(&gc);
    drop(future);
    assert!(completed, "held projection did not cover its rooted array");
    let ordinary = owner.as_reference_mut()?;
    let Value::Object(Some(handle)) = &owner else {
        panic!("object value")
    };
    assert_eq!((**handle).trace_step(&gc, 0, 1), TraceProgress::Busy);
    drop(ordinary);
    gc.collect_full()?;
    assert!(child_weak.upgrade().is_some());
    drop(owner);
    gc.stop()?;
    assert!(child_weak.upgrade().is_none());
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}
