//! A held object-array slice preserves roots while collection finishes independently.
#![expect(clippy::panic_in_result_fn)]
use ristretto_classfile::{ClassFile, ConstantPool, Field, FieldAccessFlags, FieldType};
use ristretto_classloader::{Class, Object, ObjectArray, Reference, Result, Value};
use ristretto_gc::sync::{MappedRwLockWriteGuard, RwLock};
use ristretto_gc::{
    Configuration, GarbageCollector, GcWeak, ReferenceKind, RootStorage, Trace, TraceProgress,
};
use std::sync::Arc;

fn collectors() -> impl Iterator<Item = Arc<GarbageCollector>> {
    std::iter::once(GarbageCollector::with_config(Configuration {
        allocation_threshold: usize::MAX,
        incremental_step_size: 8,
        max_pause_time_us: 0,
        ..Configuration::default()
    }))
    .chain([false, true].into_iter().flat_map(|concurrent| {
        [false, true].into_iter().flat_map(move |independent| {
            [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts]
                .into_iter()
                .map(move |storage| {
                    GarbageCollector::with_config(
                        Configuration::builder()
                            .concurrent_publication(concurrent)
                            .independent_phases(independent)
                            .independent_marking(independent)
                            .independent_relocation(independent)
                            .overlapping_generations(true)
                            .root_storage(storage)
                            .compact_heap(true)
                            .diagnostics(true)
                            .allocation_threshold(usize::MAX)
                            .old_collection_interval(usize::MAX)
                            .full_collection_interval(usize::MAX)
                            .incremental_step_size(8)
                            .max_pause_time_us(0)
                            .build(),
                    )
                })
        })
    }))
}

fn finish(gc: &GarbageCollector) -> Result<()> {
    for _ in 0..20_000 {
        if gc.step() {
            return Ok(());
        }
    }
    Err(ristretto_classloader::Error::InternalError(
        "held rooted array blocked collection".into(),
    ))
}

fn array(gc: &Arc<GarbageCollector>, values: Vec<Value>) -> Result<Value> {
    let mut constant_pool = ConstantPool::default();
    let this_class = constant_pool.add_class("[Ljava/lang/Object;")?;
    let class = Class::from(
        None,
        ClassFile {
            constant_pool,
            this_class,
            ..ClassFile::default()
        },
    )?;
    Ok(Value::new_object(
        gc,
        Reference::Array(ObjectArray {
            class,
            elements: values.into(),
        }),
    ))
}

fn weak(value: &Value) -> Result<GcWeak<RwLock<Reference>>> {
    match value {
        Value::Object(Some(handle)) => Ok(GcWeak::new(handle, ReferenceKind::Weak, 0)),
        _ => Err(ristretto_classloader::Error::InternalError(
            "expected object".into(),
        )),
    }
}

#[test]
fn held_array_projection_allows_collection() -> Result<()> {
    for gc in collectors() {
        let child = Value::new_object(&gc, Reference::ByteArray(vec![42_i8].into()));
        let child_weak = weak(&child)?;
        let source = array(&gc, vec![child])?;
        let (_, guard) = source.as_class_vec_mut()?;
        for _ in 0..3 {
            drop(Value::new_object(
                &gc,
                Reference::ByteArray(vec![7_i8].into()),
            ));
            gc.collect();
            finish(&gc)?;
            assert!(
                child_weak.upgrade().is_some(),
                "certified slice lost its child"
            );
            assert_eq!(gc.statistics()?.live_objects, 2);
            assert_eq!(guard.len(), 1);
        }
        gc.collect_old();
        finish(&gc)?;
        gc.collect_young();
        finish(&gc)?;
        drop(guard);
        drop(source);
        gc.stop()?;
        assert!(child_weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(gc.diagnostics().active_payload_guards, 0);
    }
    Ok(())
}

#[test]
fn array_replacements_and_nested_escapes_release_obsolete_roots() -> Result<()> {
    for gc in collectors() {
        let child = Value::new_object(&gc, Reference::ByteArray(vec![42_i8].into()));
        let child_weak = weak(&child)?;
        let source = array(&gc, vec![child])?;
        let (_, guard) = source.as_class_vec_mut()?;
        let guard = MappedRwLockWriteGuard::try_map(guard, |_| None::<&mut Value>)
            .expect_err("failed nested map retains certification");
        let mut guard =
            MappedRwLockWriteGuard::map(guard, |values| values.first_mut().expect("element"));
        let pointer = std::ptr::from_ref(&*guard);
        let replacement = Value::new_object(&gc, Reference::IntArray(vec![17].into()));
        let replacement_weak = weak(&replacement)?;
        let escaped = std::mem::replace(&mut *guard, replacement);
        gc.collect();
        finish(&gc)?;
        assert!(child_weak.upgrade().is_some());
        assert!(replacement_weak.upgrade().is_some());
        assert_eq!(&*escaped.as_byte_vec_ref()?, &[42]);
        drop(escaped);
        for _ in 0..3 {
            gc.collect();
            finish(&gc)?;
        }
        assert!(
            child_weak.upgrade().is_none(),
            "replaced Value retained its old root"
        );
        assert!(replacement_weak.upgrade().is_some());
        assert_eq!(&*guard.as_int_vec_ref()?, &[17]);
        assert_eq!(std::ptr::from_ref(&*guard), pointer);
        assert_eq!(gc.statistics()?.live_objects, 2);
        *guard = Value::Object(None);
        for _ in 0..3 {
            gc.collect();
            finish(&gc)?;
        }
        assert!(replacement_weak.upgrade().is_none());
        assert_eq!(gc.statistics()?.live_objects, 1);
        drop(guard);
        // A later unrestricted writer is not covered by the old certificate.
        let ordinary = source.as_reference_mut()?;
        if let Value::Object(Some(owner)) = &source {
            assert!(matches!(
                (**owner).trace_step(&gc, 0, 8),
                TraceProgress::Busy
            ));
        }
        drop(ordinary);
        drop(source);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

fn conditional_instance(gc: &Arc<GarbageCollector>, target: Value) -> Result<Value> {
    let mut constant_pool = ConstantPool::default();
    let this_class = constant_pool.add_class("java/lang/ref/WeakReference")?;
    let name_index = constant_pool.add_utf8("referent")?;
    let descriptor_index = constant_pool.add_utf8("Ljava/lang/Object;")?;
    let class = Class::from(
        None,
        ClassFile {
            constant_pool,
            this_class,
            fields: vec![Field {
                access_flags: FieldAccessFlags::PUBLIC,
                name_index,
                descriptor_index,
                field_type: FieldType::Object("java/lang/Object".into()),
                attributes: vec![],
            }],
            ..ClassFile::default()
        },
    )?;
    let mut object = Object::new(class)?;
    object.set_value_at_slot(0, target)?;
    Ok(Value::new_object(gc, Reference::Object(object)))
}

#[test]
fn rooted_array_targets_still_process_conditional_references() -> Result<()> {
    for gc in collectors() {
        let target = Value::new_object(&gc, Reference::ByteArray(vec![42_i8].into()));
        let target_weak = weak(&target)?;
        let reference = conditional_instance(&gc, target)?;
        let reference_weak = weak(&reference)?;
        let expected = match &reference {
            Value::Object(Some(owner)) => owner.identity(),
            _ => {
                return Err(ristretto_classloader::Error::InternalError(
                    "expected reference".into(),
                ));
            }
        };
        let source = array(&gc, vec![reference])?;
        let (_, guard) = source.as_class_vec_mut()?;
        for _ in 0..3 {
            gc.collect();
            finish(&gc)?;
        }
        assert!(reference_weak.upgrade().is_some());
        assert!(
            target_weak.upgrade().is_none(),
            "conditional target was strengthened"
        );
        let pending = gc
            .take_pending_reference::<RwLock<Reference>>()?
            .expect("conditional owner enqueued");
        assert_eq!(pending.identity(), expected);
        assert!(
            !gc.has_pending_references(),
            "duplicate conditional delivery"
        );
        assert!(matches!(
            pending.read().as_object_ref()?.value_at_slot(0)?,
            Value::Object(None)
        ));
        drop(pending);
        drop(guard);
        drop(source);
        gc.stop()?;
        assert!(reference_weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn rooted_array_foreign_fields_keep_source_heap_protection() -> Result<()> {
    for gc in collectors() {
        let source_gc = GarbageCollector::new();
        let child = Value::new_object(&source_gc, Reference::ByteArray(vec![42_i8].into()));
        let child_weak = weak(&child)?;
        let owner = array(&gc, vec![child])?;
        let (_, mut guard) = owner.as_class_vec_mut()?;
        for _ in 0..3 {
            source_gc.collect_full()?;
            gc.collect();
            finish(&gc)?;
            assert!(child_weak.upgrade().is_some());
        }
        *guard.first_mut().expect("foreign child") = Value::Object(None);
        for _ in 0..3 {
            source_gc.collect_full()?;
            gc.collect();
            finish(&gc)?;
        }
        assert!(child_weak.upgrade().is_none());
        drop(guard);
        drop(owner);
        gc.stop()?;
        source_gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(source_gc.managed_bytes(), 0);
    }
    Ok(())
}
