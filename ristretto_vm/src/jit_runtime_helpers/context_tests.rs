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
        assert!(ctx.reference_roots.lock().is_empty());
        assert!(ctx.references.get().is_none());
    }
    let after = gc.statistics()?;
    assert_eq!(
        before.bytes_managed + before.managed_bytes_freed,
        after.bytes_managed + after.managed_bytes_freed
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
    let array = alloc_reference(&inner, Reference::IntArray(vec![1].into()));
    assert!(!inner.reference_roots.lock().is_empty());
    assert!(outer.reference_roots.lock().is_empty());
    collect(gc).await?;
    assert!(matches!(
        &*Gc::<RwLock<Reference>>::from_handle_rooted(gc, array)?.read(),
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
    let value = Value::Object(ctx.take_pending_exception_owner());
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
async fn a_panicking_exception_consumer_releases_the_transferred_root() -> Result<()> {
    let (vm, thread, class) = crate::test::class().await?;
    let gc = vm.garbage_collector();
    let ctx = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let target = Gc::try_new(gc, RwLock::new(Reference::IntArray(vec![7].into())))?.into_rooted();
    let weak = ristretto_gc::GcWeak::new(&target, ristretto_gc::ReferenceKind::Weak, 0);
    let handle = target.as_handle_i64();
    ctx.set_pending_exception(handle);
    drop(target);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let owner = ctx.take_pending_exception_owner();
        assert_eq!(owner.as_ref().map(Gc::as_handle_i64), Some(handle));
        panic!("exception consumer unwind");
    }));
    assert!(result.is_err());
    assert_eq!(ctx.pending_exception(), 0);
    drop(ctx);
    collect(gc).await?;
    collect(gc).await?;
    assert!(
        weak.upgrade().is_none(),
        "unwind leaked the transferred exception root"
    );
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn repeated_helper_results_share_one_owner_and_release_it_on_return() -> Result<()> {
    let (vm, thread, class) = crate::test::class().await?;
    let gc = vm.garbage_collector();
    let ctx = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let target = Gc::try_new(gc, RwLock::new(Reference::IntArray(vec![7].into())))?.into_rooted();
    let weak = ristretto_gc::GcWeak::new(&target, ristretto_gc::ReferenceKind::Weak, 0);
    let mut edge = target.clone();
    // SAFETY: The target owns the edge throughout its uses and helper-result registration.
    #[expect(unsafe_code, reason = "explicit non-owning edge fixture")]
    unsafe {
        edge.unroot();
    };
    for _ in 0..1_000 {
        ctx.retain_helper_result(&edge)?;
        ctx.retain_helper_result(&target)?;
    }
    assert_eq!(ctx.reference_roots.lock().len(), 1);
    drop((target, edge));
    collect(gc).await?;
    collect(gc).await?;
    assert!(weak.upgrade().is_some(), "helper result lost its root");
    drop(ctx);
    collect(gc).await?;
    collect(gc).await?;
    assert!(
        weak.upgrade().is_none(),
        "helper result survived invocation cleanup"
    );
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn concurrent_helper_result_ownership_keeps_distinct_identities() -> Result<()> {
    let (vm, thread, class) = crate::test::class().await?;
    let gc = vm.garbage_collector();
    let ctx = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let targets = (0..3)
        .map(|value| {
            Gc::try_new(gc, RwLock::new(Reference::IntArray(vec![value].into())))
                .map(GcRootGuard::into_rooted)
        })
        .collect::<ristretto_gc::Result<Vec<_>>>()?;
    let weak = targets
        .iter()
        .map(|target| ristretto_gc::GcWeak::new(target, ristretto_gc::ReferenceKind::Weak, 0))
        .collect::<Vec<_>>();
    let start = std::sync::Barrier::new(4);
    std::thread::scope(|scope| {
        let workers = (0..4)
            .map(|_| {
                scope.spawn(|| -> Result<()> {
                    start.wait();
                    for _ in 0..250 {
                        for target in &targets {
                            ctx.retain_helper_result(target)?;
                        }
                    }
                    Ok(())
                })
            })
            .collect::<Vec<_>>();
        for worker in workers {
            worker.join().expect("helper root worker panicked")?;
        }
        Ok::<_, crate::Error>(())
    })?;
    assert_eq!(ctx.reference_roots.lock().len(), 3);
    drop(targets);
    collect(gc).await?;
    assert!(weak.iter().all(|target| target.upgrade().is_some()));
    drop(ctx);
    collect(gc).await?;
    collect(gc).await?;
    assert!(weak.iter().all(|target| target.upgrade().is_none()));
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn frame_root_replacement_is_transactional_and_independent_of_a_stalled_frame() -> Result<()>
{
    let (vm, thread, class) = crate::test::class().await?;
    let gc = vm.garbage_collector();
    let ctx = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let first = alloc_reference(&ctx, Reference::IntArray(vec![1].into()));
    let second = alloc_reference(&ctx, Reference::IntArray(vec![2].into()));
    let weak = |handle| -> Result<_> {
        let owner = Gc::<RwLock<Reference>>::from_handle_rooted(gc, handle)?;
        Ok(ristretto_gc::GcWeak::new(
            &owner,
            ristretto_gc::ReferenceKind::Weak,
            0,
        ))
    };
    let first_weak = weak(first)?;
    let second_weak = weak(second)?;
    let wrong_type = Gc::new(gc, 42_u64);
    let prepared = Gc::try_new(gc, RwLock::new(Reference::IntArray(vec![3].into())))?.into_rooted();
    let prepared_weak = ristretto_gc::GcWeak::new(&prepared, ristretto_gc::ReferenceKind::Weak, 0);
    // SAFETY: this test is the sole activation; a failed transfer must not release
    // either prior root. A new valid identity precedes the invalid one so failure
    // must also release the owner prepared through the cached-miss path.
    assert!(
        unsafe {
            ctx.publish_frame_roots(&[first, prepared.as_handle_i64(), wrong_type.as_handle_i64()])
        }
        .is_err()
    );
    assert_eq!(ctx.reference_roots.lock().len(), 2);
    drop(prepared);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(
        prepared_weak.upgrade().is_none(),
        "failed transfer leaked a prepared owner"
    );
    assert!(first_weak.upgrade().is_some());
    assert!(second_weak.upgrade().is_some());
    // SAFETY: only first is subsequently used by the simulated activation.
    unsafe { ctx.publish_frame_roots(&[first, 0, first]) }?;
    {
        let roots = ctx.reference_roots.lock();
        assert_eq!(roots.len(), 1);
        gc.collect_full()?;
        gc.collect_full()?;
    }
    assert!(first_weak.upgrade().is_some());
    assert!(
        second_weak.upgrade().is_none(),
        "discarded result retained by the stalled frame"
    );
    // SAFETY: the activation no longer uses either raw identity.
    unsafe { ctx.publish_frame_roots(&[]) }?;
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(first_weak.upgrade().is_none());
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn existing_frame_roots_reuse_storage_and_release_only_omitted_owners() -> Result<()> {
    let (vm, thread, class) = crate::test::class().await?;
    let gc = vm.garbage_collector();
    let ctx = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let first = alloc_reference(&ctx, Reference::IntArray(vec![1].into()));
    let second = alloc_reference(&ctx, Reference::IntArray(vec![2].into()));
    let third = alloc_reference(&ctx, Reference::IntArray(vec![3].into()));
    let weak = [first, second, third].map(|handle| -> Result<_> {
        let owner = Gc::<RwLock<Reference>>::from_handle_rooted(gc, handle)?;
        Ok(ristretto_gc::GcWeak::new(
            &owner,
            ristretto_gc::ReferenceKind::Weak,
            0,
        ))
    });
    let [first_weak, second_weak, third_weak] = weak;
    let (first_weak, second_weak, third_weak) = (first_weak?, second_weak?, third_weak?);
    let location = || -> Result<_> {
        let roots = ctx.reference_roots.lock();
        let owner = roots
            .get(&first)
            .ok_or_else(|| InternalError("missing retained owner".to_owned()))?;
        Ok((std::ptr::from_ref(owner), roots.capacity()))
    };
    let original = location()?;
    // SAFETY: this test is the sole executor and retains all identities it uses.
    unsafe { ctx.publish_frame_roots(&[0, third, first, second, third]) }?;
    assert_eq!(
        location()?,
        original,
        "unchanged set rebuilt its owning map"
    );
    // The number of entries in the input is unchanged, but duplicate identities
    // must not keep omitted roots alive. Ownership must be compared as a set.
    unsafe { ctx.publish_frame_roots(&[first, first, 0]) }?;
    assert_eq!(
        location()?,
        original,
        "subset publication replaced map storage"
    );
    assert_eq!(ctx.reference_roots.lock().len(), 1);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(first_weak.upgrade().is_some());
    assert!(second_weak.upgrade().is_none());
    assert!(third_weak.upgrade().is_none());
    // An unknown, wrong-typed identity must still validate transactionally.
    let wrong_type = Gc::new(gc, 42_u64);
    assert!(unsafe { ctx.publish_frame_roots(&[first, wrong_type.as_handle_i64()]) }.is_err());
    assert_eq!(location()?, original);
    unsafe { ctx.publish_frame_roots(&[0, 0]) }?;
    assert!(ctx.reference_roots.lock().is_empty());
    assert_eq!(ctx.reference_roots.lock().capacity(), original.1);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(first_weak.upgrade().is_none());
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn changing_small_frame_roots_reuse_storage_without_retaining_discarded_owners() -> Result<()>
{
    let (vm, thread, class) = crate::test::class().await?;
    let gc = vm.garbage_collector();
    let ctx = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let retained = alloc_reference(&ctx, Reference::IntArray(vec![1].into()));
    let discarded = alloc_reference(&ctx, Reference::IntArray(vec![2].into()));
    let discarded_owner = Gc::<RwLock<Reference>>::from_handle_rooted(gc, discarded)?;
    let discarded_weak =
        ristretto_gc::GcWeak::new(&discarded_owner, ristretto_gc::ReferenceKind::Weak, 0);
    drop(discarded_owner);
    let new_owners =
        [3, 4].map(|value| Gc::try_new(gc, RwLock::new(Reference::IntArray(vec![value].into()))));
    let [first, second] = new_owners;
    let (first, second) = (first?.into_rooted(), second?.into_rooted());
    let weak = [&first, &second]
        .map(|owner| ristretto_gc::GcWeak::new(owner, ristretto_gc::ReferenceKind::Weak, 0));
    ctx.reference_roots.lock().reserve(8);
    let location = || -> Result<_> {
        let roots = ctx.reference_roots.lock();
        let owner = roots
            .get(&retained)
            .ok_or_else(|| InternalError("missing retained owner".to_owned()))?;
        Ok((std::ptr::from_ref(owner), roots.capacity()))
    };
    let original = location()?;
    // SAFETY: the sole activation drops the discarded identity and publishes
    // every identity it will subsequently use, including duplicate/null inputs.
    unsafe {
        ctx.publish_frame_roots(&[
            retained,
            first.as_handle_i64(),
            0,
            first.as_handle_i64(),
            second.as_handle_i64(),
        ])
    }?;
    assert_eq!(
        location()?,
        original,
        "changing set rebuilt sufficient map storage"
    );
    assert_eq!(ctx.reference_roots.lock().len(), 3);
    drop((first, second));
    {
        let _stalled_frame = ctx.reference_roots.lock();
        gc.collect_full()?;
        gc.collect_full()?;
    }
    assert!(discarded_weak.upgrade().is_none());
    assert!(weak.iter().all(|owner| owner.upgrade().is_some()));
    unsafe { ctx.publish_frame_roots(&[retained]) }?;
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(weak.iter().all(|owner| owner.upgrade().is_none()));
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn changing_frame_roots_release_owners_across_the_inline_limit() -> Result<()> {
    let (vm, thread, class) = crate::test::class().await?;
    let gc = vm.garbage_collector();
    let ctx = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let wrong_type = Gc::new(gc, 42_u64);
    let mut prior = Vec::<ristretto_gc::GcWeak<RwLock<Reference>>>::new();
    for count in [1, 1, 8, 32, 31, 33, 32, 32, 1, 0] {
        let owners = (0..count)
            .map(|value| {
                Gc::try_new(gc, RwLock::new(Reference::IntArray(vec![value].into())))
                    .map(GcRootGuard::into_rooted)
            })
            .collect::<ristretto_gc::Result<Vec<_>>>()?;
        let handles = owners.iter().map(Gc::as_handle_i64).collect::<Vec<_>>();
        let weak = owners
            .iter()
            .map(|owner| ristretto_gc::GcWeak::new(owner, ristretto_gc::ReferenceKind::Weak, 0))
            .collect::<Vec<_>>();
        // SAFETY: only this activation publishes; the complete successor state
        // uses only these newly created references, still owned during handoff.
        unsafe { ctx.publish_frame_roots(&handles) }?;
        assert_eq!(ctx.reference_roots.lock().len(), owners.len());
        drop(owners);
        // A failed complete replacement must preserve the sole/batched/fallback
        // owner set even after all external argument owners have gone away.
        assert!(unsafe { ctx.publish_frame_roots(&[wrong_type.as_handle_i64()]) }.is_err());
        let temporary =
            Gc::try_new(gc, RwLock::new(Reference::IntArray(vec![99].into())))?.into_rooted();
        let temporary_weak =
            ristretto_gc::GcWeak::new(&temporary, ristretto_gc::ReferenceKind::Weak, 0);
        // Include an existing owner before a new valid owner and an invalid one.
        // The large fallback must release its lock before validation/rollback.
        assert!(
            unsafe {
                ctx.publish_frame_roots(&[
                    handles.first().copied().unwrap_or(0),
                    temporary.as_handle_i64(),
                    wrong_type.as_handle_i64(),
                ])
            }
            .is_err()
        );
        drop(temporary);
        assert_eq!(ctx.reference_roots.lock().len(), handles.len());
        gc.collect_full()?;
        gc.collect_full()?;
        assert!(weak.iter().all(|owner| owner.upgrade().is_some()));
        assert!(prior.iter().all(|owner| owner.upgrade().is_none()));
        assert!(temporary_weak.upgrade().is_none());
        prior = weak;
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn invalid_frame_root_buffers_set_an_exception_without_discarding_roots() -> Result<()> {
    let (vm, thread, class) = crate::test::class().await?;
    let ctx = RuntimeContext::new(vm.garbage_collector(), &vm, &thread, &class)?;
    let handle = alloc_reference(&ctx, Reference::IntArray(vec![1].into()));
    for count in [
        -1,
        1,
        i32::try_from(ristretto_jit::FRAME_ROOT_CAPACITY)? + 1,
    ] {
        assert_eq!(
            jit_publish_frame_roots(ctx.as_ptr(), std::ptr::null(), count),
            0
        );
        assert!(ctx.take_pending_exception_owner().is_some());
        assert!(ctx.reference_roots.lock().contains_key(&handle));
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn invalid_call_buffers_throw_before_reading_arguments() -> Result<()> {
    let (vm, thread, class) = crate::test::class().await?;
    let ctx = RuntimeContext::new(vm.garbage_collector(), &vm, &thread, &class)?;
    let handle = alloc_reference(&ctx, Reference::IntArray(vec![1].into()));
    let storage = [0_i64; 2];
    let helpers: [extern "C" fn(*const u8, i32, i32, *const i64, i32) -> i64; 4] = [
        calls::jit_invokestatic,
        calls::jit_invokespecial,
        calls::jit_invokevirtual,
        calls::jit_invokeinterface,
    ];
    for helper in helpers {
        for (pointer, count) in [
            (std::ptr::null(), -1),
            (std::ptr::null(), 1),
            (
                std::ptr::null(),
                i32::try_from(ristretto_jit::CALL_ARGUMENT_CAPACITY)? + 1,
            ),
            (storage.as_ptr().wrapping_byte_add(1), 1),
        ] {
            assert_eq!(helper(ctx.as_ptr(), 7, 0, pointer, count), 0);
            assert!(ctx.take_pending_exception_owner().is_some());
            assert!(ctx.reference_roots.lock().contains_key(&handle));
            assert_eq!(ctx.current_bci.load(Ordering::Relaxed), 7);
        }
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn concurrent_exception_replacement_and_transfer_keep_pointer_and_owner_paired() -> Result<()>
{
    let (vm, thread, class) = crate::test::class().await?;
    let gc = vm.garbage_collector();
    let ctx = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let first = Gc::try_new(gc, RwLock::new(Reference::IntArray(vec![1].into())))?.into_rooted();
    let second = Gc::try_new(gc, RwLock::new(Reference::IntArray(vec![2].into())))?.into_rooted();
    std::thread::scope(|scope| {
        for handle in [first.as_handle_i64(), second.as_handle_i64()] {
            let ctx = &ctx;
            scope.spawn(move || {
                for _ in 0..1_000 {
                    ctx.set_pending_exception(handle);
                    std::thread::yield_now();
                }
            });
        }
        for _ in 0..2_000 {
            let (value, owner) = ctx.take_exception_owner();
            assert_eq!(owner.as_ref().map_or(0, Gc::as_handle_i64), value);
            std::thread::yield_now();
        }
    });
    let (value, owner) = ctx.take_exception_owner();
    assert_eq!(owner.as_ref().map_or(0, Gc::as_handle_i64), value);
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
    assert!(first.field_reference(field_index)?.get().is_none());
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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn handler_entry_moves_the_pending_owner_without_a_second_registration() -> Result<()> {
    let (vm, thread, class) = crate::test::class().await?;
    let gc = vm.garbage_collector();
    let ctx = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let target = Gc::try_new(gc, RwLock::new(Reference::IntArray(vec![17].into())))?.into_rooted();
    let handle = target.as_handle_i64();
    let weak = ristretto_gc::GcWeak::new(&target, ristretto_gc::ReferenceKind::Weak, 0);
    ctx.set_pending_exception(handle);
    drop(target);
    // Hold a clone of the original registration so replacing it with a new one
    // cannot be hidden by immediate removal of the pending slot's old root.
    let original = ctx
        .pending_exception_owner
        .lock()
        .as_ref()
        .cloned()
        .ok_or_else(|| InternalError("missing pending owner".to_owned()))?;
    let roots = || {
        let stats = gc.root_statistics();
        stats.slot_roots + stats.compatibility_roots
    };
    let before = roots();
    assert_eq!(jit_take_pending_exception(ctx.as_ptr()), handle);
    assert_eq!(ctx.pending_exception(), 0);
    assert_eq!(
        roots(),
        before,
        "handler transfer registered an additional root"
    );
    drop(original);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(weak.upgrade().is_some());
    // SAFETY: this sole simulated activation has consumed the exception.
    unsafe { ctx.publish_frame_roots(&[]) }?;
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(weak.upgrade().is_none());
    assert_eq!(jit_take_pending_exception(ctx.as_ptr()), 0);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn completed_reference_context_validates_fallbacks_and_releases_unused_roots() -> Result<()> {
    let (vm, thread, class) = crate::test::class().await?;
    let gc = vm.garbage_collector();
    let wrong_type = Gc::new(gc, 42_u64);
    for case in 0..3 {
        let external =
            Gc::try_new(gc, RwLock::new(Reference::IntArray(vec![23].into())))?.into_rooted();
        let external_handle = external.as_handle_i64();
        let external_weak =
            ristretto_gc::GcWeak::new(&external, ristretto_gc::ReferenceKind::Weak, 0);
        let handle = match case {
            0 => 0,
            1 => external_handle,
            _ => wrong_type.as_handle_i64(),
        };
        let ctx = RuntimeContext::new(gc, &vm, &thread, &class)?;
        let garbage = alloc_reference(&ctx, Reference::IntArray(vec![1].into()));
        let temporary = Gc::<RwLock<Reference>>::from_handle_rooted(gc, garbage)?;
        let weak = ristretto_gc::GcWeak::new(&temporary, ristretto_gc::ReferenceKind::Weak, 0);
        drop(temporary);
        let result = ctx.into_reference_result(handle);
        drop(external);
        if handle == 0 {
            assert_eq!(result?, Value::Object(None));
        } else if handle == external_handle {
            let result = result?;
            gc.collect_full()?;
            gc.collect_full()?;
            assert_eq!(&[23], &*result.as_int_vec_ref()?);
            assert!(external_weak.upgrade().is_some());
        } else {
            assert!(result.is_err(), "wrong-typed result identity was accepted");
        }
        gc.collect_full()?;
        gc.collect_full()?;
        assert!(
            external_weak.upgrade().is_none(),
            "fallback result leaked its owner"
        );
        assert!(
            weak.upgrade().is_none(),
            "completed context retained an unused helper result"
        );
    }
    Ok(())
}
