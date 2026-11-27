//! Shared array-store semantics must also hold in the threadless interpreter.
use super::*;
use crate::Error::JavaError;
use ristretto_classfile::{ClassAccessFlags, ClassFile, ConstantPool};
use ristretto_classloader::{ClassLoader, ClassPath};

fn owner(value: &Value) -> Result<&Gc<RwLock<Reference>>> {
    match value {
        Value::Object(Some(owner)) => Ok(owner),
        _ => Err(InternalError("missing array-store owner".to_owned())),
    }
}

#[tokio::test]
async fn aastore_array_covariance_preserves_component_identity() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    for (array_name, value_name, compatible) in [
        ("[Ljava/lang/Object;", "[I", true),
        ("[[Ljava/lang/Object;", "[Ljava/lang/String;", true),
        ("[[Ljava/lang/Object;", "[I", false),
        ("[[Ljava/lang/Cloneable;", "[[I", true),
        ("[[Ljava/io/Serializable;", "[[I", true),
        ("[[[Ljava/lang/Object;", "[[[I", true),
        ("[[I", "[J", false),
    ] {
        let target = thread.class(array_name).await?;
        let array = Value::try_new_object_async(
            vm.garbage_collector(),
            Reference::try_from((target, vec![Value::Object(None)]))?,
        )
        .await?;
        let class = thread.class(value_name).await?;
        let reference = match value_name {
            "[I" => Reference::IntArray(vec![1].into()),
            "[J" => Reference::LongArray(vec![2].into()),
            _ => Reference::try_from((class, vec![Value::Object(None)]))?,
        };
        let value = Value::try_new_object_async(vm.garbage_collector(), reference).await?;
        let result =
            store_reference_array(&thread, owner(&array)?, 0, Some(owner(&value)?.clone())).await;
        if compatible {
            result?;
            assert_eq!(
                array.as_class_vec_ref()?.1.first(),
                Some(&value),
                "{array_name} <- {value_name}"
            );
        } else {
            assert!(
                matches!(
                    result,
                    Err(JavaError(crate::JavaError::ArrayStoreException(_)))
                ),
                "{array_name} <- {value_name}: {result:?}"
            );
            assert!(
                array
                    .as_class_vec_ref()?
                    .1
                    .first()
                    .is_some_and(Value::is_null)
            );
        }
    }
    Ok(())
}

#[tokio::test]
async fn aastore_resolved_sibling_components_do_not_initialize() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    let parent = thread.class("java/lang/Object").await?;
    let mut definitions = Vec::new();
    let mut loaders = Vec::new();
    for label in ["first", "second"] {
        let loader = ClassLoader::new(label, ClassPath::new(Vec::new()));
        let mut pool = ConstantPool::new();
        let this_class = pool.add_class("example/StoreElement")?;
        let super_class = pool.add_class("java/lang/Object")?;
        let class = Class::from(
            Some(Arc::downgrade(&loader)),
            ClassFile {
                version: ristretto_classfile::JAVA_5,
                constant_pool: pool,
                access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
                this_class,
                super_class,
                ..Default::default()
            },
        )?;
        class.set_parent(Some(parent.clone()))?;
        class.finalize_field_layout()?;
        // Deliberately absent from name lookup: array identity already owns the definition.
        assert!(loader.find_loaded(class.java_name()).await.is_none());
        definitions.push(class);
        loaders.push(loader);
    }
    let first = definitions
        .first()
        .ok_or_else(|| InternalError("missing first class".to_owned()))?;
    let second = definitions
        .last()
        .ok_or_else(|| InternalError("missing second class".to_owned()))?;
    let array_class = thread.array_class(first, 1).await?;
    let array = Value::try_new_object_async(
        vm.garbage_collector(),
        Reference::try_from((array_class, vec![Value::Object(None)]))?,
    )
    .await?;
    for (class, compatible) in [(first, true), (second, false)] {
        let value = Value::try_new_object_async(
            vm.garbage_collector(),
            Reference::Object(Object::new_in_async(vm.garbage_collector(), class.clone()).await?),
        )
        .await?;
        let result =
            store_reference_array(&thread, owner(&array)?, 0, Some(owner(&value)?.clone())).await;
        if compatible {
            result?;
        } else {
            assert!(matches!(
                result,
                Err(JavaError(crate::JavaError::ArrayStoreException(_)))
            ));
        }
        assert!(!class.is_initialized()?);
    }
    assert!(!first.is_initialized()?);
    drop(loaders);
    Ok(())
}

#[tokio::test]
async fn aastore_wait_owns_operands_without_holding_payload_guards() -> Result<()> {
    use std::future::Future;
    use std::task::{Context, Waker};
    for cancel in [false, true] {
        let (vm, thread, frame) = crate::test::frame().await?;
        let gc = vm.garbage_collector();
        gc.stop()?;
        let class = thread.class("[Ljava/lang/Object;").await?;
        let array = Value::try_new_object_async(
            gc,
            Reference::try_from((class, vec![Value::Object(None)]))?,
        )
        .await?;
        let value = Value::try_new_object_async(gc, Reference::IntArray(vec![77].into())).await?;
        let array_weak =
            ristretto_gc::GcWeak::new(owner(&array)?, ristretto_gc::ReferenceKind::Weak, 0);
        let value_weak =
            ristretto_gc::GcWeak::new(owner(&value)?, ristretto_gc::ReferenceKind::Weak, 0);
        let garbage = Value::try_new_object_async(gc, Reference::IntArray(vec![13].into())).await?;
        let garbage_weak =
            ristretto_gc::GcWeak::new(owner(&garbage)?, ristretto_gc::ReferenceKind::Weak, 0);
        drop(garbage);
        let mut stack = OperandStack::with_max_size(3);
        stack.push(array)?;
        stack.push_int(0)?;
        stack.push(value)?;
        let loader = vm.class_loader();
        let gate = loader.write().await;
        let mut operation = Box::pin(aastore(&frame, &mut stack));
        assert!(
            operation
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        {
            let array = array_weak
                .upgrade()
                .ok_or_else(|| InternalError("lost pending interpreter array".to_owned()))?;
            assert!(array.try_write().is_some(), "payload guard spans an await");
        }
        gc.collect_full()?;
        gc.collect_full()?;
        assert!(array_weak.upgrade().is_some());
        assert!(value_weak.upgrade().is_some());
        assert!(garbage_weak.upgrade().is_none());
        if cancel {
            drop(operation);
            gc.collect_full()?;
            gc.collect_full()?;
            assert!(array_weak.upgrade().is_none());
            assert!(value_weak.upgrade().is_none());
            drop(gate);
        } else {
            let array = array_weak
                .upgrade()
                .ok_or_else(|| InternalError("lost committed interpreter array".to_owned()))?;
            drop(gate);
            assert_eq!(operation.await?, Continue);
            let array = Value::Object(Some(array));
            let stored = array
                .as_class_vec_ref()?
                .1
                .first()
                .cloned()
                .ok_or_else(|| InternalError("missing committed value".to_owned()))?;
            assert_eq!(&*stored.as_int_vec_ref()?, &[77]);
            drop(stored);
            drop(array);
            gc.collect_full()?;
            gc.collect_full()?;
            assert!(array_weak.upgrade().is_none());
            assert!(value_weak.upgrade().is_none());
        }
        assert!(stack.is_empty());
    }
    Ok(())
}
