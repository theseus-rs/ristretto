//! Primitive typed views certify their whole Reference; general writers remain opaque.
use super::*;
use ristretto_classfile::{ClassFile, ConstantPool};
use ristretto_gc::{Configuration, GcWeak, ReferenceKind, RootStorage, Trace, TraceProgress};
use std::fmt::Debug;

fn finish(gc: &GarbageCollector) -> Result<()> {
    for _ in 0..20_000 {
        if gc.step() {
            return Ok(());
        }
    }
    Err(crate::Error::InternalError(
        "typed primitive borrow blocked collection".into(),
    ))
}
fn collect_while_pinned<T: Copy + Debug + PartialEq>(
    gc: &Arc<GarbageCollector>,
    source: &Value,
    guard: MappedRwLockWriteGuard<'_, [T]>,
) -> Result<()> {
    let pointer = guard.as_ptr();
    let expected = guard[0];
    let bytes = gc.managed_bytes();
    if let Value::Object(Some(owner)) = source {
        (**owner).trace(gc);
    }
    let guard = MappedRwLockWriteGuard::try_map(guard, |_| None::<&mut T>)
        .err()
        .ok_or_else(|| {
            crate::Error::InternalError("nested projection unexpectedly succeeded".into())
        })?;
    let mut guard = MappedRwLockWriteGuard::map(guard, |values| &mut values[0]);
    gc.collect_old();
    finish(gc)?;
    gc.collect_young();
    finish(gc)?;
    for _ in 0..3 {
        drop(Gc::try_new(gc, 42_u64)?);
        gc.collect();
        finish(gc)?;
        assert_eq!(gc.statistics()?.live_objects, 1);
        assert_eq!(gc.managed_bytes(), bytes);
        assert_eq!(std::ptr::from_ref(&*guard), pointer);
        assert_eq!(*guard, expected);
    }
    *guard = expected;
    drop(guard);
    Ok(())
}

#[test]
fn every_primitive_view_permits_collection_while_borrowed() -> Result<()> {
    for concurrent in [false, true] {
        for compact in [false, true] {
            for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(concurrent)
                        .compact_heap(compact)
                        .root_storage(storage)
                        .overlapping_generations(true)
                        .old_collection_interval(usize::MAX)
                        .allocation_threshold(usize::MAX)
                        .full_collection_interval(usize::MAX)
                        .incremental_step_size(8)
                        .max_pause_time_us(0)
                        .diagnostics(true)
                        .build(),
                );
                macro_rules! check {
                    ($variant:ident, $element:expr, $method:ident) => {{
                        let source =
                            Value::new_object(&gc, Reference::$variant(vec![$element; 64].into()));
                        collect_while_pinned(&gc, &source, source.$method()?)?;
                        collect_while_pinned(&gc, &source, source.as_bytes_mut()?)?;
                        drop(source);
                        gc.collect_full()?;
                        assert_eq!(gc.managed_bytes(), 0);
                    }};
                }
                let source = Value::new_object(&gc, Reference::BooleanArray(vec![1_i8; 64].into()));
                collect_while_pinned(&gc, &source, source.as_bytes_mut()?)?;
                drop(source);
                gc.collect_full()?;
                assert_eq!(gc.managed_bytes(), 0);
                check!(ByteArray, 7_i8, as_byte_vec_mut);
                check!(CharArray, 7_u16, as_char_vec_mut);
                check!(ShortArray, 7_i16, as_short_vec_mut);
                check!(IntArray, 7_i32, as_int_vec_mut);
                check!(LongArray, 7_i64, as_long_vec_mut);
                check!(FloatArray, 7_f32, as_float_vec_mut);
                check!(DoubleArray, 7_f64, as_double_vec_mut);
                gc.stop()?;
                assert_eq!(gc.diagnostics().active_payload_guards, 0);
            }
        }
    }
    Ok(())
}

#[test]
fn primitive_certificate_cannot_outlive_variant_replacement() -> Result<()> {
    let gc = GarbageCollector::new();
    let source = Value::new_object(&gc, Reference::IntArray(vec![7_i32; 64].into()));
    drop(source.as_int_vec_mut()?);
    let child = Value::new_object(&gc, Reference::ByteArray(vec![42_i8].into()));
    let weak = if let Value::Object(Some(owner)) = &child {
        GcWeak::new(owner, ReferenceKind::Weak, 0)
    } else {
        unreachable!()
    };
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
    *source.as_reference_mut()? = Reference::try_from((class, vec![child]))?;
    assert!(source.as_bytes_mut().is_err());
    assert!(source.as_int_vec_mut().is_err());
    let guard = source.as_reference_mut()?;
    if let Value::Object(Some(owner)) = &source {
        assert!(matches!(
            (**owner).trace_step(&gc, 0, 8),
            TraceProgress::Busy
        ));
    }
    drop(guard);
    for _ in 0..3 {
        gc.collect_full()?;
    }
    assert!(weak.upgrade().is_some());
    let stored = source.as_class_vec_ref()?.1[0].clone();
    assert_eq!(&*stored.as_byte_vec_ref()?, &[42]);
    drop(stored);
    drop(source);
    gc.stop()?;
    assert!(weak.upgrade().is_none());
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}
