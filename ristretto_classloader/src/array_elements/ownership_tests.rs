//! Extraction must transfer ownership before a managed array can lose its edge.
#![expect(clippy::unwrap_in_result, clippy::panic_in_result_fn)]
use super::*;
use crate::{Class, Object, ObjectArray, Reference, ReferenceWriteGuardExt, Result};
use ristretto_classfile::{ClassFile, ConstantPool, Field, FieldAccessFlags, FieldType};
use ristretto_gc::sync::{MappedRwLockWriteGuard, RwLock, RwLockWriteGuard};
use ristretto_gc::{Configuration, GcWeak, ReferenceKind, RootStorage};
use std::sync::Arc;

fn finish(gc: &GarbageCollector) {
    gc.collect();
    for _ in 0..20_000 {
        if gc.step() {
            return;
        }
    }
    panic!("collection failed to complete after releasing array guards");
}

fn weak(value: &Value) -> GcWeak<RwLock<Reference>> {
    match value {
        Value::Object(Some(reference)) => GcWeak::new(reference, ReferenceKind::Weak, 0),
        _ => unreachable!(),
    }
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

fn instance(gc: &Arc<GarbageCollector>, child: Value) -> Result<Value> {
    let mut pool = ConstantPool::default();
    let this_class = pool.add_class("MovedReference")?;
    let name_index = pool.add_utf8("child")?;
    let descriptor_index = pool.add_utf8("Ljava/lang/Object;")?;
    let class = Class::from(
        None,
        ClassFile {
            constant_pool: pool,
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
    object.set_value_at_slot(0, child)?;
    Ok(Value::new_object(gc, Reference::Object(object)))
}

#[derive(Clone, Copy, Debug)]
enum WholeAccess {
    Direct,
    Map,
    TryMap,
    FailedMap,
}

fn extract_whole(source: &Value, access: WholeAccess) -> Result<Reference> {
    let guard = source.as_reference_mut()?;
    let replacement = Reference::from(Vec::<i8>::new());
    Ok(match access {
        WholeAccess::Direct => {
            let mut guard = guard;
            std::mem::replace(&mut *guard, replacement)
        }
        WholeAccess::Map => {
            let mut guard = RwLockWriteGuard::map(guard, |value| value);
            std::mem::replace(&mut *guard, replacement)
        }
        WholeAccess::TryMap => {
            let mut guard = RwLockWriteGuard::try_map(guard, |value| Some(value)).unwrap();
            std::mem::replace(&mut *guard, replacement)
        }
        WholeAccess::FailedMap => {
            let mut extracted = None;
            let result = RwLockWriteGuard::try_map(guard, |value| {
                extracted = Some(std::mem::replace(value, replacement));
                None::<&mut Reference>
            });
            assert!(result.is_err());
            drop(result);
            extracted.expect("mapping callback moved the value")
        }
    })
}

#[test]
fn moving_a_whole_reference_preserves_its_transitive_graph() -> Result<()> {
    for concurrent in [false, true] {
        for compact in [false, true] {
            for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
                for (instance_value, access, buffer) in [false, true].into_iter().flat_map(|kind| {
                    [
                        WholeAccess::Direct,
                        WholeAccess::Map,
                        WholeAccess::TryMap,
                        WholeAccess::FailedMap,
                    ]
                    .into_iter()
                    .flat_map(move |access| [0, 1].map(|buffer| (kind, access, buffer)))
                }) {
                    let gc = GarbageCollector::with_config(
                        Configuration::builder()
                            .concurrent_publication(concurrent)
                            .compact_heap(compact)
                            .root_storage(storage)
                            .barrier_buffer_size(buffer)
                            .allocation_threshold(usize::MAX)
                            .full_collection_interval(usize::MAX)
                            .incremental_step_size(1)
                            .max_pause_time_us(0)
                            .build(),
                    );
                    let context = gc.register_context();
                    let (extracted, leaf, source_weak) = context.with(|| -> Result<_> {
                        let leaf = Value::new_object(&gc, Reference::from(vec![42_i8; 16]));
                        let leaf_weak = weak(&leaf);
                        let branch = array(&gc, vec![leaf])?;
                        let source = if instance_value {
                            instance(&gc, branch)?
                        } else {
                            array(&gc, vec![branch])?
                        };
                        let source_weak = weak(&source);
                        let extracted = extract_whole(&source, access)?;
                        drop(source);
                        Ok((extracted, leaf_weak, source_weak))
                    })?;
                    drop(context);
                    for _ in 0..3 {
                        finish(&gc);
                        assert!(
                            leaf.upgrade().is_some(),
                            "whole-value extraction lost its graph"
                        );
                    }
                    assert!(source_weak.upgrade().is_none());
                    let branch = match &extracted {
                        Reference::Object(object) => object.value_at_slot(0)?,
                        _ => extracted.as_class_vec_ref()?.1[0].clone(),
                    };
                    let child = branch.as_class_vec_ref()?.1[0].clone();
                    assert_eq!(&*child.as_byte_vec_ref()?, &[42; 16]);
                    drop((child, branch, extracted));
                    for _ in 0..3 {
                        finish(&gc);
                    }
                    assert!(leaf.upgrade().is_none());
                    gc.stop()?;
                    assert_eq!(gc.managed_bytes(), 0);
                    assert_eq!(gc.root_statistics().slot_roots, 0);
                    assert_eq!(gc.root_statistics().compatibility_roots, 0);
                }
            }
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Access {
    Point,
    Get,
    Index,
    Slice,
    AsMut,
    IterMut,
    Mapped,
    IntoIter,
}

fn extract(source: &Value, access: Access) -> Result<Value> {
    let mut guard = source.as_reference_mut()?;
    if let Access::Point = access {
        let value = guard.array_element_mut(1).expect("element");
        return Ok(std::mem::replace(value, Value::Object(None)));
    }
    if let Access::Mapped = access {
        let guard = RwLockWriteGuard::map(guard, |reference| {
            let Reference::Array(array) = reference else {
                unreachable!()
            };
            array.elements.get_mut(1).expect("element")
        });
        let mut nested = MappedRwLockWriteGuard::map(guard, |value| value);
        return Ok(std::mem::replace(&mut *nested, Value::Object(None)));
    }
    let Reference::Array(array) = &mut *guard else {
        unreachable!()
    };
    let element = match access {
        Access::Get => array.elements.get_mut(1).expect("element"),
        Access::Index => &mut array.elements[1],
        Access::Slice => &mut array.elements.deref_mut()[1],
        Access::AsMut => &mut array.elements.as_mut()[1],
        Access::IterMut => (&mut array.elements).into_iter().nth(1).expect("element"),
        Access::IntoIter => {
            let elements = std::mem::replace(&mut array.elements, Vec::new().into());
            return Ok(elements.into_iter().nth(1).expect("element"));
        }
        Access::Mapped | Access::Point => unreachable!(),
    };
    Ok(std::mem::replace(element, Value::Object(None)))
}

#[test]
fn every_mutable_extraction_keeps_transitive_edges_after_context_detach() -> Result<()> {
    for concurrent in [false, true] {
        for compact in [false, true] {
            for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
                for buffer in [0, 1] {
                    for access in [
                        Access::Point,
                        Access::Get,
                        Access::Index,
                        Access::Slice,
                        Access::AsMut,
                        Access::IterMut,
                        Access::Mapped,
                        Access::IntoIter,
                    ] {
                        let gc = GarbageCollector::with_config(
                            Configuration::builder()
                                .concurrent_publication(concurrent)
                                .compact_heap(compact)
                                .root_storage(storage)
                                .barrier_buffer_size(buffer)
                                .allocation_threshold(usize::MAX)
                                .full_collection_interval(usize::MAX)
                                .incremental_step_size(1)
                                .max_pause_time_us(0)
                                .diagnostics(true)
                                .build(),
                        );
                        let context = gc.register_context();
                        let (extracted, leaf, source_weak) = context.with(|| -> Result<_> {
                            let leaf = Value::new_object(&gc, Reference::from(vec![42_i8; 16]));
                            let leaf_weak = weak(&leaf);
                            let branch = array(&gc, vec![leaf])?;
                            let source = array(&gc, vec![Value::Int(7), branch, Value::Int(9)])?;
                            let source_weak = weak(&source);
                            let extracted = extract(&source, access)?;
                            drop(source);
                            Ok((extracted, leaf_weak, source_weak))
                        })?;
                        drop(context);
                        for _ in 0..3 {
                            finish(&gc);
                            assert!(leaf.upgrade().is_some(), "lost graph through {access:?}");
                        }
                        assert!(source_weak.upgrade().is_none());
                        let child = extracted.as_class_vec_ref()?.1[0].clone();
                        assert_eq!(&*child.as_byte_vec_ref()?, &[42; 16]);
                        drop(child);
                        drop(extracted);
                        for _ in 0..3 {
                            finish(&gc);
                        }
                        assert!(leaf.upgrade().is_none());
                        gc.stop()?;
                        assert_eq!(gc.managed_bytes(), 0);
                        assert_eq!(gc.statistics()?.live_objects, 0);
                        assert_eq!(gc.root_statistics().slot_roots, 0);
                        assert_eq!(gc.root_statistics().compatibility_roots, 0);
                        assert_eq!(gc.diagnostics().active_payload_guards, 0);
                    }
                }
            }
        }
    }
    Ok(())
}

#[test]
fn owned_iterator_roots_unconsumed_values_until_it_is_dropped() -> Result<()> {
    let gc = GarbageCollector::new();
    let first = Value::new_object(&gc, Reference::from(vec![1_i8]));
    let second = Value::new_object(&gc, Reference::from(vec![2_i8]));
    let first_weak = weak(&first);
    let second_weak = weak(&second);
    let source = array(&gc, vec![first, second])?;
    let mut iter = {
        let mut guard = source.as_reference_mut()?;
        let Reference::Array(array) = &mut *guard else {
            unreachable!()
        };
        std::mem::replace(&mut array.elements, Vec::new().into()).into_iter()
    };
    drop(source);
    let first = iter.next().expect("first");
    for _ in 0..3 {
        finish(&gc);
    }
    assert!(first_weak.upgrade().is_some());
    assert!(second_weak.upgrade().is_some());
    drop(iter);
    for _ in 0..3 {
        finish(&gc);
    }
    assert!(first_weak.upgrade().is_some());
    assert!(second_weak.upgrade().is_none());
    drop(first);
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn narrow_stores_publish_young_edges_without_rooting_unrelated_fields() -> Result<()> {
    for concurrent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            for object in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(concurrent)
                        .root_storage(storage)
                        .barrier_buffer_size(1)
                        .promotion_age(1)
                        .allocation_threshold(usize::MAX)
                        .full_collection_interval(usize::MAX)
                        .diagnostics(true)
                        .build(),
                );
                let context = gc.register_context();
                let source = context.with(|| {
                    let old = Value::new_object(&gc, Reference::from(vec![1_i8]));
                    if object {
                        instance(&gc, old)
                    } else {
                        array(&gc, vec![old; 64])
                    }
                })?;
                gc.collect_full()?;
                let replacement = Value::new_object(&gc, Reference::from(vec![42_i8]));
                let replacement_weak = weak(&replacement);
                context.with(|| -> Result<()> {
                    let before = gc.diagnostics().root_registrations;
                    let mut guard = source.as_reference_mut()?;
                    if object {
                        guard.set_value_at_slot(0, replacement)?;
                    } else {
                        let rejected = guard
                            .set_array_element(64, replacement)
                            .expect_err("invalid index must retain the input");
                        assert!(guard.array_element_mut(64).is_none());
                        assert_eq!(gc.diagnostics().root_registrations, before);
                        guard.set_array_element(63, rejected).expect("valid store");
                    }
                    assert_eq!(gc.diagnostics().root_registrations, before);
                    Ok(())
                })?;
                drop(context);
                gc.collect_young();
                let mut complete = false;
                for _ in 0..20_000 {
                    if gc.step() {
                        complete = true;
                        break;
                    }
                }
                assert!(complete, "young collection failed to finish");
                assert!(replacement_weak.upgrade().is_some());
                let stored = if object {
                    source.as_object_ref()?.value_at_slot(0)?
                } else {
                    source.as_class_vec_ref()?.1[63].clone()
                };
                assert_eq!(&*stored.as_byte_vec_ref()?, &[42]);
                drop(stored);
                drop(source);
                gc.stop()?;
                assert!(replacement_weak.upgrade().is_none());
                assert_eq!(gc.managed_bytes(), 0);
                assert_eq!(gc.root_statistics().slot_roots, 0);
                assert_eq!(gc.root_statistics().compatibility_roots, 0);
                assert_eq!(gc.diagnostics().active_payload_guards, 0);
            }
        }
    }
    Ok(())
}

#[test]
fn narrow_element_borrow_roots_only_selected_edge_and_whole_borrow_still_prepares() -> Result<()> {
    let gc = GarbageCollector::with_config(Configuration::builder().diagnostics(true).build());
    let leaf = Value::new_object(&gc, Reference::from(vec![42_i8]));
    let leaf_weak = weak(&leaf);
    let source = array(&gc, vec![leaf; 64])?;
    let mut guard = source.as_reference_mut()?;
    let before = gc.diagnostics().root_registrations;
    let moved = std::mem::replace(
        guard.array_element_mut(63).expect("element"),
        Value::Object(None),
    );
    assert_eq!(gc.diagnostics().root_registrations, before + 1);
    let whole = std::mem::replace(&mut *guard, Reference::from(Vec::<i8>::new()));
    assert_eq!(gc.diagnostics().root_registrations, before + 64);
    drop(guard);
    drop(source);
    drop(moved);
    for _ in 0..3 {
        finish(&gc);
    }
    assert!(
        leaf_weak.upgrade().is_some(),
        "later whole borrow must acquire its own roots"
    );
    drop(whole);
    gc.stop()?;
    assert!(leaf_weak.upgrade().is_none());
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn replacement_store_adds_no_roots_after_whole_value_borrow_preparation() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .diagnostics(true)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let old = Value::new_object(&gc, Reference::from(vec![1_i8]));
        let old_weak = weak(&old);
        let replacement = Value::new_object(&gc, Reference::from(vec![2_i8]));
        let replacement_weak = weak(&replacement);
        let source = array(&gc, vec![old])?;
        let before = gc.diagnostics().root_registrations;
        {
            let mut guard = source.as_reference_mut()?;
            let Reference::Array(array) = &mut *guard else {
                unreachable!()
            };
            // Exposing &mut Reference now has to protect a possible whole-value
            // move. Measure set itself separately from that required acquisition.
            assert_eq!(gc.diagnostics().root_registrations, before + 1);
            let prepared = gc.diagnostics().root_registrations;
            let rejected = array
                .elements
                .set(1, replacement)
                .expect_err("invalid index");
            assert_eq!(array.elements[0].as_byte_vec_ref()?[0], 1);
            array.elements.set(0, rejected).expect("valid index");
            assert_eq!(gc.diagnostics().root_registrations, prepared);
        }
        assert_eq!(gc.diagnostics().root_registrations, before + 1);
        for _ in 0..3 {
            finish(&gc);
        }
        assert!(old_weak.upgrade().is_none());
        assert!(replacement_weak.upgrade().is_some());
        drop(source);
        gc.stop()?;
        assert!(replacement_weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
#[cfg(not(target_family = "wasm"))]
fn mapped_borrow_unwind_demotes_values_left_in_the_container() -> Result<()> {
    let gc = GarbageCollector::new();
    let value = Value::new_object(&gc, Reference::from(vec![42_i8]));
    let leaf = weak(&value);
    let source = array(&gc, vec![value])?;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let guard = source.as_reference_mut().expect("array");
        let _mapped = RwLockWriteGuard::map(guard, |reference| {
            let Reference::Array(array) = reference else {
                unreachable!()
            };
            array.elements.as_mut()
        });
        panic!("cancel native mutable borrow");
    }));
    assert!(result.is_err());
    finish(&gc);
    assert!(leaf.upgrade().is_some());
    drop(source);
    gc.stop()?;
    assert!(leaf.upgrade().is_none());
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn mapped_borrow_cancellation_demotes_values_left_in_the_container() -> Result<()> {
    use std::future::Future;
    use std::task::{Context, Waker};
    let gc = GarbageCollector::new();
    let value = Value::new_object(&gc, Reference::from(vec![42_i8]));
    let leaf = weak(&value);
    let source = array(&gc, vec![value])?;
    let mut future = Box::pin(async {
        let guard = source.as_reference_mut().expect("array");
        let _mapped = RwLockWriteGuard::map(guard, |reference| {
            let Reference::Array(array) = reference else {
                unreachable!()
            };
            array.elements.as_mut()
        });
        std::future::pending::<()>().await;
    });
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    drop(future);
    finish(&gc);
    assert!(leaf.upgrade().is_some());
    drop(source);
    gc.stop()?;
    assert!(leaf.upgrade().is_none());
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

fn conditional_instance(gc: &Arc<GarbageCollector>, child: Value) -> Result<Value> {
    let mut pool = ConstantPool::default();
    let this_class = pool.add_class("java/lang/ref/WeakReference")?;
    let name_index = pool.add_utf8("referent")?;
    let descriptor_index = pool.add_utf8("Ljava/lang/Object;")?;
    let class = Class::from(
        None,
        ClassFile {
            constant_pool: pool,
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
    object.set_value_at_slot(0, child)?;
    Ok(Value::new_object(gc, Reference::Object(object)))
}

#[test]
fn moved_conditional_objects_enqueue_their_new_owner_without_strengthening_the_referent()
-> Result<()> {
    for concurrent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            for access in [
                WholeAccess::Direct,
                WholeAccess::Map,
                WholeAccess::TryMap,
                WholeAccess::FailedMap,
            ] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(concurrent)
                        .root_storage(storage)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let target = Value::new_object(&gc, Reference::ByteArray(vec![42].into()));
                let target_weak = weak(&target);
                let source = conditional_instance(&gc, target.clone())?;
                let source_weak = weak(&source);
                let destination = Value::new_object(&gc, Reference::IntArray(vec![7].into()));
                let moved = extract_whole(&source, access)?;
                *destination.as_reference_mut()? = moved;
                // Ordinary whole-value mutation must preserve this binding as well.
                {
                    let _ = &mut *destination.as_reference_mut()?;
                }
                let expected = match &destination {
                    Value::Object(Some(handle)) => handle.identity(),
                    _ => unreachable!(),
                };
                drop((source, target));
                for _ in 0..3 {
                    finish(&gc);
                }
                assert!(
                    source_weak.upgrade().is_none(),
                    "old reference owner was retained"
                );
                assert!(
                    target_weak.upgrade().is_none(),
                    "weak referent became a strong edge"
                );
                let pending = gc
                    .take_pending_reference::<RwLock<Reference>>()?
                    .expect("new owner must be enqueued");
                assert_eq!(pending.identity(), expected);
                assert!(!gc.has_pending_references(), "owner was enqueued twice");
                assert!(matches!(
                    pending.read().as_object_ref()?.value_at_slot(0)?,
                    Value::Object(None)
                ));
                drop((pending, destination));
                gc.stop()?;
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[test]
fn foreign_instance_and_array_fields_keep_their_source_heap_alive() -> Result<()> {
    for concurrent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            for is_array in [false, true] {
                let source = GarbageCollector::with_config(
                    Configuration::builder()
                        .root_storage(storage)
                        .concurrent_publication(concurrent)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let destination = GarbageCollector::with_config(
                    Configuration::builder()
                        .root_storage(storage)
                        .concurrent_publication(concurrent)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let child = Value::new_object(&source, Reference::ByteArray(vec![42].into()));
                let child_weak = weak(&child);
                let parent = if is_array {
                    array(&destination, vec![child])?
                } else {
                    instance(&destination, child)?
                };
                // Force the broad root/unroot path in addition to initial attachment.
                {
                    let _ = &mut *parent.as_reference_mut()?;
                }
                source.collect_full()?;
                assert!(child_weak.upgrade().is_some());
                let source_heap = Arc::downgrade(&source);
                drop(source);
                assert!(source_heap.upgrade().is_some());
                drop(parent);
                destination.stop()?;
                assert!(source_heap.upgrade().is_none());
                assert!(child_weak.upgrade().is_none());
                assert_eq!(destination.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[test]
fn a_java_array_point_store_retains_its_foreign_root_without_private_debt() -> Result<()> {
    let mut pending = Vec::new();
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .threads(0)
                .diagnostics(true)
                .concurrent_publication(counted)
                .barrier_buffer_size(4)
                .incremental_step_size(1)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let foreign = GarbageCollector::new();
        let existing = Value::new_object(&gc, Reference::ByteArray(vec![17].into()));
        let original = weak(&existing);
        let holder = array(&gc, vec![existing, Value::Object(None)])?;
        let incoming = Value::new_object(&foreign, Reference::ByteArray(vec![41].into()));
        let witness = weak(&incoming);
        let Value::Object(Some(owner)) = &holder else {
            unreachable!()
        };
        let context = gc.register_context();
        gc.collect();
        assert!(!gc.step());
        assert_eq!(gc.diagnostics().pending_barrier_records, 0);
        context
            .with(|| owner.write().set_array_element(1, incoming))
            .unwrap();
        pending.push((counted, gc.diagnostics().pending_barrier_records));
        gc.collect_full()?;
        foreign.collect_full()?;
        let guard = owner.read();
        let Reference::Array(array) = &*guard else {
            unreachable!()
        };
        let Value::Object(Some(existing)) = &array.elements[0] else {
            unreachable!()
        };
        let Value::Object(Some(incoming)) = &array.elements[1] else {
            unreachable!()
        };
        assert!(original.refers_to(existing) && witness.refers_to(incoming));
        drop(guard);
        owner
            .write()
            .set_array_element(1, Value::Object(None))
            .unwrap();
        foreign.collect_full()?;
        assert!(witness.upgrade().is_none());
        drop(holder);
        gc.collect_full()?;
        assert!(original.upgrade().is_none());
        gc.stop()?;
        foreign.stop()?;
        assert_eq!((gc.managed_bytes(), foreign.managed_bytes()), (0, 0));
    }
    assert_eq!(pending, [(false, 0), (true, 0)]);
    Ok(())
}

#[test]
fn batched_java_writes_preserve_unrelated_traced_elements() -> Result<()> {
    for counted in [false, true] {
        for (last, gap, expected) in [(1, false, 0), (2, true, 1), (32, false, 1)] {
            let heap = GarbageCollector::with_config(
                Configuration::builder()
                    .threads(0)
                    .diagnostics(true)
                    .concurrent_publication(counted)
                    .barrier_buffer_size(4)
                    .incremental_step_size(1)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let foreign = GarbageCollector::new();
            let local = Value::new_object(&heap, Reference::IntArray(vec![73].into()));
            let local_witness = weak(&local);
            let known = if gap { 1 } else { 33 };
            let mut values = vec![Value::Object(None); 34];
            values[known] = local;
            let holder = array(&heap, values)?;
            let Value::Object(Some(parent)) = &holder else {
                unreachable!()
            };
            let incoming = Value::new_object(&foreign, Reference::IntArray(vec![51].into()));
            let witness = weak(&incoming);
            let context = heap.register_context();
            heap.collect();
            assert!(!heap.step());
            context.with(|| {
                let mut writer = parent.write();
                writer.set_array_element(0, incoming.clone()).unwrap();
                writer.set_array_element(last, incoming).unwrap();
            });
            assert_eq!(heap.diagnostics().pending_barrier_records, expected);
            heap.collect_full()?;
            foreign.collect_full()?;
            let read = parent.read();
            let Reference::Array(array) = &*read else {
                unreachable!()
            };
            for (i, w) in [(0, &witness), (last, &witness), (known, &local_witness)] {
                let Value::Object(Some(target)) = &array.elements[i] else {
                    unreachable!()
                };
                assert!(w.refers_to(target));
            }
            drop(read);
            drop(holder);
            heap.collect_full()?;
            foreign.collect_full()?;
            assert!(local_witness.upgrade().is_none() && witness.upgrade().is_none());
            heap.stop()?;
            foreign.stop()?;
        }
    }
    Ok(())
}

#[cfg(panic = "unwind")]
#[test]
fn java_point_unwind_and_whole_array_replacement_preserve_ownership() -> Result<()> {
    for replace in [false, true] {
        let heap = GarbageCollector::with_config(
            Configuration::builder()
                .threads(0)
                .diagnostics(true)
                .barrier_buffer_size(4)
                .incremental_step_size(1)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let foreign = GarbageCollector::new();
        let holder = array(&heap, vec![Value::Object(None)])?;
        let Value::Object(Some(parent)) = &holder else {
            unreachable!()
        };
        let target = Value::new_object(&foreign, Reference::IntArray(vec![11].into()));
        let witness = weak(&target);
        let context = heap.register_context();
        heap.collect();
        assert!(!heap.step());
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            context.with(|| {
                let mut writer = parent.write();
                if replace {
                    let Reference::Array(old) = &*writer else {
                        unreachable!()
                    };
                    let class = old.class.clone();
                    *writer = Reference::Array(ObjectArray {
                        class,
                        elements: vec![target].into(),
                    });
                } else {
                    writer.set_array_element(0, target).unwrap();
                }
                panic!("unwind through the managed write guard");
            });
        }));
        assert!(result.is_err());
        assert_eq!(
            heap.diagnostics().pending_barrier_records,
            usize::from(replace)
        );
        heap.collect_full()?;
        foreign.collect_full()?;
        {
            let guard = parent.read();
            let Reference::Array(array) = &*guard else {
                unreachable!()
            };
            let Value::Object(Some(target)) = &array.elements[0] else {
                unreachable!()
            };
            assert!(witness.refers_to(target));
        }
        drop(holder);
        heap.collect_full()?;
        foreign.collect_full()?;
        assert!(witness.upgrade().is_none());
        heap.stop()?;
        foreign.stop()?;
    }
    Ok(())
}
