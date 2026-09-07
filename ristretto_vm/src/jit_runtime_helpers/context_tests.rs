use super::*;
use crate::JavaError::NoClassDefFoundError;
use ristretto_classfile::{ClassFile, ConstantPool, FieldAccessFlags, FieldType, JavaStr};
use std::time::Duration;

fn caller(target: &str) -> Result<(Arc<Class>, u16, u16)> {
    let mut constant_pool = ConstantPool::default();
    let this_class = constant_pool.add_class("JitContextCaller")?;
    let class_index = constant_pool.add_class(target)?;
    let field_index = constant_pool.add_field_ref(class_index, "value", "I")?;
    let class = Class::from(
        None,
        ClassFile {
            constant_pool,
            this_class,
            ..Default::default()
        },
    )?;
    Ok((class, class_index, field_index))
}

async fn target(thread: &Thread, name: &str) -> Result<Arc<Class>> {
    let mut constant_pool = ConstantPool::default();
    let this_class = constant_pool.add_class(name)?;
    let fields = vec![ristretto_classfile::Field {
        access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
        name_index: constant_pool.add_utf8("value")?,
        descriptor_index: constant_pool.add_utf8("I")?,
        field_type: FieldType::parse("I")?,
        attributes: vec![],
    }];
    let class = Class::from(
        None,
        ClassFile {
            constant_pool,
            this_class,
            fields,
            ..Default::default()
        },
    )?;
    thread.register_class(class).await?;
    let class = thread
        .load_and_link_class(JavaStr::try_from_str(name)?)
        .await?;
    assert!(!class.is_initialized()?);
    Ok(class)
}

async fn collect(gc: &GarbageCollector) -> Result<()> {
    let completed = gc.statistics()?.collections_completed;
    gc.collect();
    tokio::time::timeout(Duration::from_secs(5), async {
        while gc.statistics()?.collections_completed == completed {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        Ok::<_, crate::Error>(())
    })
    .await
    .map_err(|error| InternalError(error.to_string()))?
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn repeated_calls_reuse_metadata_and_rooted_sentinel_without_gc_allocations() -> Result<()> {
    let (vm, thread, class) = crate::test::class().await?;
    let gc = vm.garbage_collector();
    let first = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let exceptions = first.exception_classes.clone();
    let sentinel = first.sentinel_throwable;
    drop(first);
    collect(gc).await?;
    let before = gc.statistics()?;
    for _ in 0..1_000 {
        let ctx = RuntimeContext::new(gc, &vm, &thread, &class)?;
        assert!(Arc::ptr_eq(&ctx.exception_classes, &exceptions));
        assert_eq!(ctx.sentinel_throwable, sentinel);
        assert_eq!(ctx.pending_exception(), 0);
        assert_eq!(ctx.current_bci.load(Ordering::Relaxed), -1);
        assert!(ctx.transient_roots.lock().is_empty());
        assert!(ctx.references.get().is_none());
    }
    let after = gc.statistics()?;
    assert_eq!(
        before.bytes_allocated + before.bytes_freed,
        after.bytes_allocated + after.bytes_freed
    );
    assert_eq!(thread.jit_runtime().sentinels.lock().len(), 1);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn nested_contexts_and_java_threads_keep_invocation_state_separate() -> Result<()> {
    let (vm, thread, class) = crate::test::class().await?;
    let gc = vm.garbage_collector();
    let outer = RuntimeContext::new(gc, &vm, &thread, &class)?;
    outer.current_bci.store(42, Ordering::Relaxed);
    outer.set_pending_exception(outer.sentinel_throwable);
    let inner = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let other_thread = Thread::new(&Arc::downgrade(&vm), thread.id() + 1);
    let other = RuntimeContext::new(gc, &vm, &other_thread, &class)?;
    for ctx in [&inner, &other] {
        assert!(Arc::ptr_eq(
            &outer.exception_classes,
            &ctx.exception_classes
        ));
        assert_ne!(outer.sentinel_throwable, ctx.sentinel_throwable);
        assert_eq!(ctx.pending_exception(), 0);
        assert_eq!(ctx.current_bci.load(Ordering::Relaxed), -1);
    }
    assert_ne!(inner.sentinel_throwable, other.sentinel_throwable);
    let array = alloc_reference(&inner, Reference::IntArray(vec![1].into_boxed_slice()));
    assert!(!inner.transient_roots.lock().is_empty());
    assert!(outer.transient_roots.lock().is_empty());
    collect(gc).await?;
    assert!(matches!(
        &*Gc::<RwLock<Reference>>::from_raw_i64(array)?.read(),
        Reference::IntArray(_)
    ));
    drop(inner);
    assert_eq!(outer.pending_exception(), outer.sentinel_throwable);
    assert_eq!(outer.current_bci.load(Ordering::Relaxed), 42);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn escaped_sentinel_is_retained_and_never_reused() -> Result<()> {
    let (vm, thread, class) = crate::test::class().await?;
    let gc = vm.garbage_collector();
    let ctx = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let escaped = ctx.sentinel_throwable;
    ctx.set_pending_exception(escaped);
    let value = ctx.take_pending_exception_into(|ptr| {
        Gc::<RwLock<Reference>>::from_raw_i64(ptr).map(|gc| Value::Object(Some(gc)))
    })?;
    drop(ctx);
    assert!(thread.jit_runtime().sentinels.lock().is_empty());
    collect(gc).await?;
    assert_eq!(
        value.as_object_ref()?.class().name(),
        "java/lang/VirtualMachineError"
    );
    let next = RuntimeContext::new(gc, &vm, &thread, &class)?;
    assert_ne!(next.sentinel_throwable, escaped);
    assert_eq!(next.pending_exception(), 0);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn resolutions_survive_calls_and_distinguish_referring_class_identity() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    let target = target(&thread, "JitContextTarget").await?;
    let (caller, class_index, field_index) = caller(target.name())?;
    let first = RuntimeContext::new(vm.garbage_collector(), &vm, &thread, &caller)?;
    let resolved_class = resolve_class_ref_no_init(&first, class_index)?;
    let field = resolve_field_ref(&first, field_index)?;
    drop(first);
    let other_thread = Thread::new(&Arc::downgrade(&vm), thread.id() + 1);
    let second = RuntimeContext::new(vm.garbage_collector(), &vm, &other_thread, &caller)?;
    assert!(Arc::ptr_eq(
        &resolved_class,
        &resolve_class_ref_no_init(&second, class_index)?
    ));
    assert!(Arc::ptr_eq(
        &field,
        &resolve_field_ref(&second, field_index)?
    ));
    assert!(!target.is_initialized()?);
    let other_class = Class::from(None, caller.class_file().clone())?;
    let distinct = RuntimeContext::new(vm.garbage_collector(), &vm, &thread, &other_class)?;
    assert!(!Arc::ptr_eq(
        &field,
        &resolve_field_ref(&distinct, field_index)?
    ));
    target.set_static_value("value", Value::Int(17))?;
    assert_eq!(getstatic(&second, field_index)?, Value::Int(17));
    assert!(target.is_initialized()?);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn failed_resolutions_remain_local_when_successes_are_shared() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    let (caller, class_index, field_index) = caller("JitLateTarget")?;
    let first = RuntimeContext::new(vm.garbage_collector(), &vm, &thread, &caller)?;
    assert!(resolve_class_ref_no_init(&first, class_index).is_err());
    assert!(resolve_field_ref(&first, field_index).is_err());
    assert!(first.references(class_index)?.class.get().is_none());
    assert!(first.references(field_index)?.field.get().is_none());
    let target = target(&thread, "JitLateTarget").await?;
    let second = RuntimeContext::new(vm.garbage_collector(), &vm, &thread, &caller)?;
    assert!(Arc::ptr_eq(
        &target,
        &resolve_class_ref_no_init(&second, class_index)?
    ));
    resolve_field_ref(&second, field_index)?;
    // An invocation's prior failure remains sticky even if another call resolves it.
    assert!(resolve_class_ref_no_init(&first, class_index).is_err());
    assert!(resolve_field_ref(&first, field_index).is_err());
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn shared_resolution_does_not_publish_recursive_initialization_as_complete() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    let target = target(&thread, "JitInitializingTarget").await?;
    let (caller, class_index, field_index) = caller(target.name())?;
    let first = RuntimeContext::new(vm.garbage_collector(), &vm, &thread, &caller)?;
    target.begin_initialization(thread.id())?;
    resolve_class_ref(&first, class_index)?;
    resolve_field_ref_static(&first, field_index)?;
    assert!(!target.is_initialized()?);
    let other_thread = Thread::new(&Arc::downgrade(&vm), thread.id() + 1);
    let second = RuntimeContext::new(vm.garbage_collector(), &vm, &other_thread, &caller)?;
    let third = RuntimeContext::new(vm.garbage_collector(), &vm, &other_thread, &caller)?;
    let mut class_waiter =
        tokio::task::spawn_blocking(move || resolve_class_ref(&second, class_index));
    let mut field_waiter =
        tokio::task::spawn_blocking(move || resolve_field_ref_static(&third, field_index));
    let class_blocked = tokio::time::timeout(Duration::from_millis(30), &mut class_waiter)
        .await
        .is_err();
    let field_blocked = tokio::time::timeout(Duration::from_millis(30), &mut field_waiter)
        .await
        .is_err();
    // Release waiters before asserting, including when a regression made one return early.
    target.fail_initialization("test initialization failure".to_owned())?;
    assert!(matches!(
        class_waiter.await,
        Ok(Err(JavaError(NoClassDefFoundError(_))))
    ));
    assert!(matches!(
        field_waiter.await,
        Ok(Err(JavaError(NoClassDefFoundError(_))))
    ));
    assert!(class_blocked && field_blocked);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn shared_setup_does_not_keep_vm_or_thread_alive() -> Result<()> {
    let (vm, thread, class) = crate::test::class().await?;
    let weak_vm = Arc::downgrade(&vm);
    let weak_thread = Arc::downgrade(&thread);
    let ctx = RuntimeContext::new(vm.garbage_collector(), &vm, &thread, &class)?;
    drop(ctx);
    assert_eq!(thread.jit_runtime().sentinels.lock().len(), 1);
    // JDK initialization starts daemon threads that retain the VM until shutdown.
    vm.wait_for_non_daemon_threads().await?;
    drop(thread);
    drop(vm);
    assert!(weak_thread.upgrade().is_none());
    assert!(weak_vm.upgrade().is_none());
    Ok(())
}
