//! Allocation requests must be cancellable, own their roots, and preserve Java ordering.
use super::*;
use crate::jit::static_call_tests::{class_from_methods, install_experimental, method, register};
use ristretto_classfile::attributes::{
    ArrayType, Attribute, ExceptionTableEntry, Instruction, MaxStack,
};
use ristretto_classfile::{ConstantPool, FieldAccessFlags, FieldType};
use ristretto_jit::{AllocationKind, ExecutionStep};
use std::future::Future;
use std::num::NonZeroU32;
use std::task::{Context, Poll, Waker};
use std::time::Duration;

const LIMIT: usize = 32 * 1024 * 1024;

async fn vm(concurrent: bool, compact: bool, workers: usize) -> Result<(Arc<VM>, Arc<Thread>)> {
    let gc = GarbageCollector::with_config(
        ristretto_gc::Configuration::builder()
            .threads(workers)
            .compact_heap(compact)
            .diagnostics(true)
            .concurrent_publication(concurrent)
            .root_storage(ristretto_gc::RootStorage::Contexts)
            .max_heap_bytes(LIMIT)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let vm = VM::new(
        crate::ConfigurationBuilder::new()
            .garbage_collector(gc)
            .batch_compilation(false)
            .build()?,
    )
    .await?;
    if workers == 0 {
        vm.garbage_collector().stop()?;
    }
    assert_eq!(vm.garbage_collector().registered_workers(), workers);
    let thread = Thread::new(&Arc::downgrade(&vm), 3);
    Ok((vm, thread))
}

fn reference(value: &Value) -> Result<&Gc<RwLock<Reference>>> {
    match value {
        Value::Object(Some(owner)) => Ok(owner),
        _ => Err(InternalError("missing reference".to_owned())),
    }
}

fn collect(gc: &GarbageCollector) -> Result<()> {
    gc.collect_full()?;
    gc.collect_full()?;
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "keep exact shared bytecodes and handler ranges together"
)]
async fn fixture(thread: &Thread) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let own = pool.add_class("AllocationContinuations")?;
    let object = pool.add_class("java/lang/Object")?;
    let constructor = pool.add_method_ref(object, "<init>", "()V")?;
    let multi = pool.add_class("[[I")?;
    let attempts = pool.add_field_ref(own, "attempts", "I")?;
    let field = ristretto_classfile::Field {
        access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
        name_index: pool.add_utf8("attempts")?,
        descriptor_index: pool.add_utf8("I")?,
        field_type: FieldType::parse("I")?,
        attributes: vec![],
    };
    let mut methods = Vec::new();
    for (name, kind) in [
        ("booleans", ArrayType::Boolean),
        ("bytes", ArrayType::Byte),
        ("chars", ArrayType::Char),
        ("shorts", ArrayType::Short),
        ("ints", ArrayType::Int),
        ("longs", ArrayType::Long),
        ("floats", ArrayType::Float),
        ("doubles", ArrayType::Double),
    ] {
        methods.push(method(
            &mut pool,
            name,
            "(I)Ljava/lang/Object;",
            vec![
                Instruction::Iload_0,
                Instruction::Newarray(kind),
                Instruction::Areturn,
            ],
        )?);
    }
    for (name, descriptor, code) in [
        (
            "object",
            "()Ljava/lang/Object;",
            vec![
                Instruction::New(object),
                Instruction::Dup,
                Instruction::Invokespecial(constructor),
                Instruction::Areturn,
            ],
        ),
        (
            "references",
            "(I)[Ljava/lang/Object;",
            vec![
                Instruction::Iload_0,
                Instruction::Anewarray(object),
                Instruction::Areturn,
            ],
        ),
        (
            "multi",
            "(II)[[I",
            vec![
                Instruction::Iload_0,
                Instruction::Iload_1,
                Instruction::Multianewarray(multi, 2),
                Instruction::Areturn,
            ],
        ),
        (
            "partial",
            "(I)[[I",
            vec![
                Instruction::Iload_0,
                Instruction::Multianewarray(multi, 1),
                Instruction::Areturn,
            ],
        ),
        (
            "retain",
            "([II)[I",
            vec![
                Instruction::Getstatic(attempts),
                Instruction::Iconst_1,
                Instruction::Iadd,
                Instruction::Putstatic(attempts),
                Instruction::Aload_0,
                Instruction::Iload_1,
                Instruction::Newarray(ArrayType::Byte),
                Instruction::Pop,
                Instruction::Areturn,
            ],
        ),
        (
            "allocLoop",
            "()V",
            vec![
                Instruction::Iconst_0,
                Instruction::Newarray(ArrayType::Byte),
                Instruction::Pop,
                Instruction::Goto(-4),
            ],
        ),
        (
            "monitorLoop",
            "(Ljava/lang/Object;)V",
            vec![
                Instruction::Aload_0,
                Instruction::Monitorenter,
                Instruction::Aload_0,
                Instruction::Monitorexit,
                Instruction::Goto(-4),
            ],
        ),
    ] {
        methods.push(method(&mut pool, name, descriptor, code)?);
    }
    let negative = pool.add_class("java/lang/NegativeArraySizeException")?;
    let mut caught = method(
        &mut pool,
        "caught",
        "(I)I",
        vec![
            Instruction::Iload_0,
            Instruction::Newarray(ArrayType::Int),
            Instruction::Pop,
            Instruction::Iconst_0,
            Instruction::Ireturn,
            Instruction::Pop,
            Instruction::Iconst_1,
            Instruction::Ireturn,
        ],
    )?;
    for attribute in &mut caught.attributes {
        if let Attribute::Code {
            exception_table,
            max_stack,
            code,
            ..
        } = attribute
        {
            exception_table.push(ExceptionTableEntry {
                range_pc: 0..3,
                handler_pc: 6,
                catch_type: negative,
            });
            *max_stack = code.max_stack_with_exception_table(&pool, exception_table)?;
        }
    }
    methods.push(caught);
    let mut class =
        class_from_methods("AllocationContinuations", "java/lang/Object", pool, methods)?
            .class_file()
            .clone();
    class.fields.push(field);
    let class = register(thread, Class::from(None, class)?).await?;
    thread.initialize_class(&class).await?;
    Ok(class)
}

async fn basic(vm: &Arc<VM>, thread: &Arc<Thread>, class: &Arc<Class>) -> Result<()> {
    let gc = vm.garbage_collector();
    for name in [
        "booleans", "bytes", "chars", "shorts", "ints", "longs", "floats", "doubles",
    ] {
        let descriptor = "(I)Ljava/lang/Object;";
        let function = install_experimental(vm, class, name, descriptor)?;
        assert!(function.supports_cooperation());
        let body = class.try_get_method(name, descriptor)?;
        let value = thread
            .execute(class, &body, &[Value::Int(3)])
            .await?
            .ok_or_else(|| InternalError("missing array".to_owned()))?;
        let expected = match name {
            "booleans" => "[Z",
            "bytes" => "[B",
            "chars" => "[C",
            "shorts" => "[S",
            "ints" => "[I",
            "longs" => "[J",
            "floats" => "[F",
            _ => "[D",
        };
        assert_eq!(reference(&value)?.read().class_name()?, expected);
        assert!(value.as_bytes()?.iter().all(|byte| *byte == 0));
        let weak =
            ristretto_gc::GcWeak::new(reference(&value)?, ristretto_gc::ReferenceKind::Weak, 0);
        collect(gc)?;
        assert!(weak.upgrade().is_some());
        drop(value);
        collect(gc)?;
        assert!(weak.upgrade().is_none());
        #[cfg(feature = "execution-profile")]
        {
            assert_eq!(function.execution_count(), 1);
            assert_eq!(function.resume_count(), 1);
        }
        let empty = thread
            .execute(class, &body, &[Value::Int(0)])
            .await?
            .ok_or_else(|| InternalError("missing empty array".to_owned()))?;
        assert!(empty.as_bytes()?.is_empty());
        let failure = thread.execute(class, &body, &[Value::Int(-1)]).await;
        assert!(
            matches!(failure, Err(crate::Error::Throwable(ref value)) if value.as_object_ref().is_ok_and(|object| object.class().name() == "java/lang/NegativeArraySizeException")),
            "{failure:?}"
        );
    }
    for (name, descriptor, arguments) in [
        ("references", "(I)[Ljava/lang/Object;", vec![Value::Int(-1)]),
        ("partial", "(I)[[I", vec![Value::Int(-1)]),
        ("multi", "(II)[[I", vec![Value::Int(0), Value::Int(-1)]),
    ] {
        install_experimental(vm, class, name, descriptor)?;
        let result = thread
            .execute(class, &class.try_get_method(name, descriptor)?, &arguments)
            .await;
        assert!(
            matches!(result, Err(crate::Error::Throwable(ref value)) if value.as_object_ref().is_ok_and(|object| object.class().name() == "java/lang/NegativeArraySizeException")),
            "{result:?}"
        );
    }
    for (name, descriptor, parameters) in [
        ("object", "()Ljava/lang/Object;", vec![]),
        ("references", "(I)[Ljava/lang/Object;", vec![Value::Int(3)]),
        ("multi", "(II)[[I", vec![Value::Int(3), Value::Int(2)]),
        ("partial", "(I)[[I", vec![Value::Int(3)]),
        ("caught", "(I)I", vec![Value::Int(-1)]),
    ] {
        install_experimental(vm, class, name, descriptor)?;
        let result = thread
            .execute(class, &class.try_get_method(name, descriptor)?, &parameters)
            .await?
            .ok_or_else(|| InternalError("missing result".to_owned()))?;
        match name {
            "caught" => assert_eq!(result, Value::Int(1)),
            "object" => assert_eq!(result.as_object_ref()?.class().name(), "java/lang/Object"),
            "multi" => {
                let (_, values) = result.as_class_vec_ref()?;
                assert_eq!(values.len(), 3);
                for child in values.iter() {
                    assert_eq!(&*child.as_int_vec_ref()?, &[0, 0]);
                }
            }
            "partial" | "references" => {
                let (_, values) = result.as_class_vec_ref()?;
                assert_eq!(values.len(), 3);
                assert!(values.iter().all(Value::is_null));
            }
            _ => return Err(InternalError("unexpected fixture".to_owned())),
        }
    }
    assert!(thread.frames()?.is_empty());
    Ok(())
}

/// A small ready allocation or monitor cannot reset the budget on every request.
async fn ready_requests_yield(
    vm: &Arc<VM>,
    thread: &Arc<Thread>,
    class: &Arc<Class>,
) -> Result<()> {
    let lock =
        Value::try_new_object_async(vm.garbage_collector(), Reference::IntArray(vec![0].into()))
            .await?;
    let monitor = vm.monitor_registry().monitor(
        ristretto_intrinsics::get_monitor_id(reference(&lock)?)
            .ok_or_else(|| InternalError("missing monitor".to_owned()))?,
    );
    for (name, descriptor, parameters) in [
        ("allocLoop", "()V", vec![]),
        ("monitorLoop", "(Ljava/lang/Object;)V", vec![lock.clone()]),
    ] {
        let function = install_experimental(vm, class, name, descriptor)?;
        let body = class.try_get_method(name, descriptor)?;
        let mut execution = Box::pin(thread.execute(class, &body, &parameters));
        assert!(
            execution
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        #[cfg(feature = "execution-profile")]
        {
            assert!(function.resume_count() > 0);
            assert!(function.resume_count() <= u64::from(function.cooperation_budget().get()));
        }
        #[cfg(not(feature = "execution-profile"))]
        let _ = function;
        drop(execution);
        assert!(thread.frames()?.is_empty());
        assert!(!monitor.is_owned_by(thread.id()));
    }
    Ok(())
}

/// Large initialization suspends before publication and drops its reserved ranges.
async fn initialization_cancel(
    vm: &Arc<VM>,
    thread: &Arc<Thread>,
    class: &Arc<Class>,
) -> Result<()> {
    let gc = vm.garbage_collector();
    for (name, descriptor, args) in [
        (
            "bytes",
            "(I)Ljava/lang/Object;",
            vec![Value::Int(1024 * 1024)],
        ),
        (
            "references",
            "(I)[Ljava/lang/Object;",
            vec![Value::Int(32 * 1024)],
        ),
        ("multi", "(II)[[I", vec![Value::Int(512), Value::Int(0)]),
    ] {
        let function = install_experimental(vm, class, name, descriptor)?;
        let body = class.try_get_method(name, descriptor)?;
        // Prime helper metadata and sentinel reuse outside the measured ownership interval.
        let small = if name == "multi" {
            vec![Value::Int(1), Value::Int(0)]
        } else {
            vec![Value::Int(1)]
        };
        drop(thread.execute(class, &body, &small).await?);
        collect(gc)?;
        let before = gc.managed_bytes();
        let occupied = gc
            .region_heap()
            .map(|heap| heap.statistics().occupied_bytes);
        let mut execution = Box::pin(thread.execute(class, &body, &args));
        assert!(
            execution
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending(),
            "{name} did not yield during initialization"
        );
        assert_eq!(
            thread.current_frame()?.program_counter(),
            if name == "multi" { 2 } else { 1 }
        );
        let runtime = Handle::current();
        std::thread::scope(|scope| {
            scope
                .spawn(|| {
                    let _runtime = runtime.enter();
                    assert!(
                        execution
                            .as_mut()
                            .poll(&mut Context::from_waker(Waker::noop()))
                            .is_pending()
                    );
                })
                .join()
        })
        .map_err(|_| InternalError("allocation migration panicked".to_owned()))?;
        drop(execution);
        assert!(thread.frames()?.is_empty());
        collect(gc)?;
        assert!(gc.managed_bytes() <= before, "{name} leaked logical memory");
        if let Some(before) = occupied {
            assert!(
                gc.region_heap()
                    .is_some_and(|heap| heap.statistics().occupied_bytes <= before),
                "{name} leaked payload ranges"
            );
        }
        #[cfg(feature = "execution-profile")]
        assert!(function.execution_count() >= 2);
        #[cfg(not(feature = "execution-profile"))]
        let _ = function;
    }
    Ok(())
}

/// A pending allocation owns residual references, yields during actual recovery,
/// cancels its waiter, and later completes exactly once after headroom returns.
#[expect(
    clippy::too_many_lines,
    reason = "keep cancellation and exactly-once recovery ownership transitions together"
)]
async fn recovery(vm: &Arc<VM>, thread: &Arc<Thread>, class: &Arc<Class>) -> Result<()> {
    struct LiveBytes(Vec<u8>);
    // SAFETY: this fixture contains no outgoing GC references.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl ristretto_gc::Trace for LiveBytes {
        fn trace(&self, _: &GarbageCollector) {}
        fn external_size(&self) -> usize {
            self.0.len()
        }
    }
    let gc = vm.garbage_collector();
    let function = install_experimental(vm, class, "retain", "([II)[I")?;
    let context = RuntimeContext::new(gc, vm, thread, class)?;
    let protected = Value::try_new_object_async(gc, Reference::IntArray(vec![77].into())).await?;
    let weak =
        ristretto_gc::GcWeak::new(reference(&protected)?, ristretto_gc::ReferenceKind::Weak, 0);
    let handle = reference(&protected)?.as_handle_i64();
    collect(gc)?;
    let overhead = Gc::allocation_size(&LiveBytes(Vec::new()))?;
    let length = LIMIT - gc.managed_bytes() - overhead;
    let filler = Gc::try_new_async(gc, LiveBytes(vec![7; length])).await?;
    let mut activation = function.activation(&[
        ristretto_jit::Value::Ptr(handle),
        ristretto_jit::Value::I32(1024),
    ])?;
    let ExecutionStep::Allocation(request) = resume_fields(&mut activation, &context).await? else {
        return Err(InternalError("missing allocation request".to_owned()));
    };
    assert!(matches!(
        request.kind,
        AllocationKind::PrimitiveArray(ArrayType::Byte)
    ));
    assert_eq!(activation.remaining_budget()?, None);
    drop(protected);
    let before = gc.diagnostics();
    let args = activation.call_arguments()?;
    let mut allocation = Box::pin(context.invoke_allocation(request.clone(), args));
    assert!(
        allocation
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    // Workerless recovery removes its waiter, performs one bounded assist, and
    // self-wakes. Native-worker waits keep a registered waiter instead.
    assert!(
        allocation
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    let pending = gc.diagnostics();
    assert_eq!(pending.async_assists, before.async_assists + 1);
    assert_eq!(pending.full_collection_calls, before.full_collection_calls);
    assert_eq!(pending.allocation_waiters, 0);
    assert!(weak.upgrade().is_some());
    drop(allocation);
    assert_eq!(gc.diagnostics().allocation_waiters, 0);
    // Cancellation does not authorize completing/replaying this activation.
    drop(activation);
    drop(context);
    collect(gc)?;
    assert!(weak.upgrade().is_none());
    drop(filler);
    collect(gc)?;

    let context = RuntimeContext::new(gc, vm, thread, class)?;
    let protected = Value::try_new_object_async(gc, Reference::IntArray(vec![88].into())).await?;
    let handle = reference(&protected)?.as_handle_i64();
    collect(gc)?;
    let length = LIMIT - gc.managed_bytes() - overhead;
    let filler = Gc::try_new_async(gc, LiveBytes(vec![8; length])).await?;
    let mut activation = function.activation(&[
        ristretto_jit::Value::Ptr(handle),
        ristretto_jit::Value::I32(1024),
    ])?;
    let ExecutionStep::Allocation(request) = resume_fields(&mut activation, &context).await? else {
        return Err(InternalError("missing second request".to_owned()));
    };
    let attempts = class.static_value("attempts")?;
    let resumed_before = gc.diagnostics();
    let mut allocation = Box::pin(context.invoke_allocation(request, activation.call_arguments()?));
    assert!(
        allocation
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    assert!(
        allocation
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    assert_eq!(
        gc.diagnostics().async_assists,
        resumed_before.async_assists + 1
    );
    drop(filler);
    let bits = allocation.await?;
    assert_eq!(
        gc.diagnostics().full_collection_calls,
        resumed_before.full_collection_calls
    );
    activation.complete_call(bits)?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?,
        ExecutionStep::Returned(Some(ristretto_jit::Value::Ptr(handle)))
    );
    assert_eq!(
        class.static_value("attempts")?,
        attempts,
        "resumption replayed allocation-side effects"
    );
    let result = context.into_reference_result(handle)?;
    assert_eq!(&*result.as_int_vec_ref()?, &[88]);
    let after = gc.diagnostics();
    assert_eq!(
        after.allocation_recovery.count,
        before.allocation_recovery.count
    );
    assert_eq!(after.allocation_waiters, 0);
    Ok(())
}

async fn initialization_fixture(thread: &Thread) -> Result<(Arc<Class>, Arc<Class>)> {
    let mut pool = ConstantPool::new();
    let own = pool.add_class("AllocationInitialization")?;
    let object = pool.add_class("java/lang/Object")?;
    let object_init = pool.add_method_ref(object, "<init>", "()V")?;
    let count = pool.add_field_ref(own, "count", "I")?;
    let field = ristretto_classfile::Field {
        access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
        name_index: pool.add_utf8("count")?,
        descriptor_index: pool.add_utf8("I")?,
        field_type: FieldType::parse("I")?,
        attributes: vec![],
    };
    let initializer = method(
        &mut pool,
        "<clinit>",
        "()V",
        vec![
            Instruction::Getstatic(count),
            Instruction::Iconst_1,
            Instruction::Iadd,
            Instruction::Putstatic(count),
            Instruction::Goto(0),
        ],
    )?;
    let mut constructor = method(
        &mut pool,
        "<init>",
        "()V",
        vec![
            Instruction::Aload_0,
            Instruction::Invokespecial(object_init),
            Instruction::Return,
        ],
    )?;
    constructor
        .access_flags
        .remove(ristretto_classfile::MethodAccessFlags::STATIC);
    let mut target = class_from_methods(
        "AllocationInitialization",
        "java/lang/Object",
        pool,
        vec![initializer, constructor],
    )?
    .class_file()
    .clone();
    target.fields.push(field);
    let target = register(thread, Class::from(None, target)?).await?;

    let mut pool = ConstantPool::new();
    let target_ref = pool.add_class("AllocationInitialization")?;
    let array_ref = pool.add_class("[[LAllocationInitialization;")?;
    let init = pool.add_method_ref(target_ref, "<init>", "()V")?;
    let mut methods = Vec::new();
    for (name, descriptor, code) in [
        (
            "make",
            "()LAllocationInitialization;",
            vec![
                Instruction::New(target_ref),
                Instruction::Dup,
                Instruction::Invokespecial(init),
                Instruction::Areturn,
            ],
        ),
        (
            "array",
            "(I)[LAllocationInitialization;",
            vec![
                Instruction::Iload_0,
                Instruction::Anewarray(target_ref),
                Instruction::Areturn,
            ],
        ),
        (
            "multi",
            "(II)[[LAllocationInitialization;",
            vec![
                Instruction::Iload_0,
                Instruction::Iload_1,
                Instruction::Multianewarray(array_ref, 2),
                Instruction::Areturn,
            ],
        ),
    ] {
        methods.push(method(&mut pool, name, descriptor, code)?);
    }
    let caller = register(
        thread,
        class_from_methods(
            "AllocationInitializationCaller",
            "java/lang/Object",
            pool,
            methods,
        )?,
    )
    .await?;
    Ok((target, caller))
}

/// Array resolution must not initialize its component; `new` must await it with
/// a cancellable claim and no generated frame borrowing the runtime context.
async fn class_initialization_cancel(vm: &Arc<VM>, thread: &Arc<Thread>) -> Result<()> {
    use ristretto_classloader::InitializationState;
    let (target, caller) = initialization_fixture(thread).await?;
    for (name, descriptor, arguments) in [
        (
            "array",
            "(I)[LAllocationInitialization;",
            vec![Value::Int(2)],
        ),
        (
            "multi",
            "(II)[[LAllocationInitialization;",
            vec![Value::Int(2), Value::Int(3)],
        ),
    ] {
        install_experimental(vm, &caller, name, descriptor)?;
        let value = thread
            .execute(
                &caller,
                &caller.try_get_method(name, descriptor)?,
                &arguments,
            )
            .await?
            .ok_or_else(|| InternalError("missing uninitialized-component array".to_owned()))?;
        assert_eq!(value.as_class_vec_ref()?.1.len(), 2);
        assert_eq!(
            target.initialization_state()?,
            InitializationState::NotInitialized
        );
        assert_eq!(target.static_value("count")?, Value::Int(0));
    }
    install_experimental(vm, &caller, "make", "()LAllocationInitialization;")?;
    let method = caller.try_get_method("make", "()LAllocationInitialization;")?;
    let parameters: [Value; 0] = [];
    let mut execution = Box::pin(thread.execute(&caller, &method, &parameters));
    let mut cx = Context::from_waker(Waker::noop());
    assert!(execution.as_mut().poll(&mut cx).is_pending());
    assert_eq!(
        target.initialization_state()?,
        InitializationState::BeingInitialized {
            thread_id: thread.id()
        }
    );
    assert_eq!(target.static_value("count")?, Value::Int(1));
    assert_eq!(thread.frames()?.len(), 2);
    let other = Thread::new(&Arc::downgrade(vm), thread.id() + 1);
    let mut waiter = Box::pin(other.initialize_class(&target));
    assert!(waiter.as_mut().poll(&mut cx).is_pending());
    drop(execution);
    assert!(thread.frames()?.is_empty());
    assert!(
        matches!(target.initialization_state()?, InitializationState::Failed { error } if error == "Class initialization cancelled")
    );
    assert!(matches!(
        waiter.as_mut().poll(&mut cx),
        Poll::Ready(Err(JavaError(crate::JavaError::NoClassDefFoundError(_))))
    ));
    let result = thread.execute(&caller, &method, &parameters).await;
    assert!(
        matches!(result, Err(crate::Error::Throwable(ref value)) if value.as_object_ref().is_ok_and(|object| object.class().name() == "java/lang/NoClassDefFoundError")),
        "{result:?}"
    );
    assert_eq!(
        target.static_value("count")?,
        Value::Int(1),
        "new replayed cancelled class initialization"
    );
    Ok(())
}

async fn matrix() -> Result<()> {
    for concurrent in [false, true] {
        for compact in [false, true] {
            for workers in [0, 1] {
                let (vm, thread) = vm(concurrent, compact, workers).await?;
                let class = fixture(&thread).await?;
                basic(&vm, &thread, &class).await?;
                ready_requests_yield(&vm, &thread, &class).await?;
                initialization_cancel(&vm, &thread, &class).await?;
                if workers == 0 {
                    recovery(&vm, &thread, &class).await?;
                }
                class_initialization_cancel(&vm, &thread).await?;
                println!(
                    "allocation continuations passed concurrent={concurrent} compact={compact} workers={workers}"
                );
            }
        }
    }
    Ok(())
}

async fn resume_fields(
    activation: &mut ristretto_jit::Activation,
    context: &RuntimeContext,
) -> Result<ExecutionStep> {
    loop {
        match activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())? {
            ExecutionStep::Field(request) => {
                let bits = context
                    .invoke_field(request, activation.call_arguments()?)
                    .await?;
                activation.complete_call(bits)?;
            }
            step => return Ok(step),
        }
    }
}

#[test]
fn allocations_suspend_preserve_roots_cancel_and_bound_ready_requests() -> Result<()> {
    const CHILD: &str = "RISTRETTO_ALLOCATION_CONTINUATION_CHILD";
    if std::env::var_os(CHILD).is_some() {
        return tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()?
            .block_on(matrix());
    }
    let mut command = std::process::Command::new(std::env::current_exe()?);
    command.args(["--exact","jit_runtime_helpers::allocation_continuation_tests::allocations_suspend_preserve_roots_cancel_and_bound_ready_requests","--nocapture"]).env(CHILD,"1");
    let output = ristretto_test_util::process::run(&mut command, Duration::from_secs(180))?;
    output.require_success()?;
    println!("{}", output.stdout);
    Ok(())
}
