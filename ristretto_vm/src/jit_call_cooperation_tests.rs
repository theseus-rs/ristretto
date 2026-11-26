//! Call requests must release native callers and preserve Java ownership.
use super::*;
use crate::java_object::JavaObject;
use ristretto_classfile::{FieldAccessFlags, FieldType};
use std::future::Future;
use std::task::{Context, Poll, Waker};

fn field(
    pool: &mut ConstantPool,
    name: &str,
    descriptor: &str,
) -> Result<ristretto_classfile::Field> {
    Ok(ristretto_classfile::Field {
        access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
        name_index: pool.add_utf8(name)?,
        descriptor_index: pool.add_utf8(descriptor)?,
        field_type: FieldType::parse(descriptor)?,
        attributes: vec![],
    })
}

async fn with_field(
    thread: &Thread,
    class: Arc<Class>,
    field: ristretto_classfile::Field,
) -> Result<Arc<Class>> {
    let mut definition = class.class_file().clone();
    definition.fields.push(field);
    register(thread, Class::from(None, definition)?).await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn calls_in_a_loop_preserve_locals_operands_and_execute_once() -> Result<()> {
    for concurrent in [false, true] {
        let (vm, thread) = vm_with_workers(concurrent, 0).await?;
        let mut pool = ConstantPool::new();
        let owner = pool.add_class("ResumableCallLoop")?;
        let field = field(&mut pool, "count", "I")?;
        let count = pool.add_field_ref(owner, "count", "I")?;
        let step_ref = pool.add_method_ref(owner, "step", "()I")?;
        let step = method(
            &mut pool,
            "step",
            "()I",
            vec![
                Instruction::Getstatic(count),
                Instruction::Iconst_1,
                Instruction::Iadd,
                Instruction::Dup,
                Instruction::Putstatic(count),
                Instruction::Ireturn,
            ],
        )?;
        let run = method(
            &mut pool,
            "run",
            "(I)I",
            vec![
                Instruction::Iconst_0,
                Instruction::Iload_0,
                Instruction::Ifle(13),
                Instruction::Invokestatic(step_ref),
                Instruction::Iadd,
                Instruction::Iinc(0, -1),
                Instruction::Goto(-11),
                Instruction::Ireturn,
            ],
        )?;
        let class = with_field(
            &thread,
            class_from_methods(
                "ResumableCallLoop",
                "java/lang/Object",
                pool,
                vec![run, step],
            )?,
            field,
        )
        .await?;
        let function = install_experimental(&vm, &class, "run", "(I)I")?;
        install_experimental(&vm, &class, "step", "()I")?;
        let method = class.try_get_method("run", "(I)I")?;
        let parameters = [Value::Int(3)];
        let mut execution = Box::pin(thread.execute(&class, &method, &parameters));
        for expected in 0..3 {
            assert!(
                execution
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
            assert_eq!(class.static_value("count")?, Value::Int(expected));
            assert_eq!(thread.frames()?.len(), 1);
            assert_eq!(thread.current_frame()?.program_counter(), 5);
            vm.garbage_collector().collect_full()?;
        }
        let Poll::Ready(result) = execution
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
        else {
            return Err(InternalError("call loop did not finish".to_owned()));
        };
        assert_eq!(result?, Some(Value::Int(6)));
        assert_eq!(class.static_value("count")?, Value::Int(3));
        drop(execution);
        assert!(thread.frames()?.is_empty());
        #[cfg(feature = "execution-profile")]
        {
            assert_eq!(function.execution_count(), 1);
            assert_eq!(function.resume_count(), 3);
            assert_eq!(function.backedge_count(), 3);
        }
        #[cfg(not(feature = "execution-profile"))]
        let _ = function;
        // Match the maintained HotSpot oracle, including many reused call cells.
        for iterations in [0, 1, 10_000] {
            class.set_static_value("count", Value::Int(0))?;
            assert_eq!(
                thread
                    .execute(&class, &method, &[Value::Int(iterations)])
                    .await?,
                Some(Value::Int(iterations * (iterations + 1) / 2))
            );
            assert_eq!(class.static_value("count")?, Value::Int(iterations));
        }
    }
    Ok(())
}

async fn infinite_class(thread: &Thread) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let owner = pool.add_class("SuspendedCallOwner")?;
    let field = field(&mut pool, "observed", "[I")?;
    let observed = pool.add_field_ref(owner, "observed", "[I")?;
    let target = pool.add_method_ref(owner, "callee", "(Ljava/lang/Object;)V")?;
    let mut caller = method(
        &mut pool,
        "caller",
        "()V",
        vec![
            Instruction::Iconst_1,
            Instruction::Newarray(ArrayType::Int),
            Instruction::Dup,
            Instruction::Putstatic(observed),
            Instruction::Dup,
            Instruction::Invokestatic(target),
            Instruction::Pop,
            Instruction::Return,
        ],
    )?;
    caller.access_flags.insert(MethodAccessFlags::SYNCHRONIZED);
    let mut callee = method(
        &mut pool,
        "callee",
        "(Ljava/lang/Object;)V",
        vec![Instruction::Goto(0)],
    )?;
    callee.access_flags.insert(MethodAccessFlags::SYNCHRONIZED);
    let done = method(
        &mut pool,
        "done",
        "()I",
        vec![Instruction::Bipush(42), Instruction::Ireturn],
    )?;
    with_field(
        thread,
        class_from_methods(
            "SuspendedCallOwner",
            "java/lang/Object",
            pool,
            vec![caller, callee, done],
        )?,
        field,
    )
    .await
}

#[expect(
    clippy::too_many_lines,
    reason = "ordered pending-call, nested-frame and cancellation ownership assertions"
)]
async fn cancel_call(
    concurrent: bool,
    workers: usize,
    compiled: bool,
    entered: bool,
) -> Result<()> {
    let (vm, thread) = vm_with_workers(concurrent, workers).await?;
    let gc = vm.garbage_collector();
    let class = infinite_class(&thread).await?;
    let function = install_experimental(&vm, &class, "caller", "()V")?;
    let callee = class.try_get_method("callee", "(Ljava/lang/Object;)V")?;
    let compiler = vm
        .compiler()
        .ok_or_else(|| InternalError("missing compiler".to_owned()))?;
    if compiled {
        assert!(compiler.compile(&class, &callee).await?.is_some());
    } else {
        compiler.insert_cached(cache_key(&callee), None);
    }
    let mirror = class.to_object(&thread).await?;
    let mirror = reference_owner(&mirror)?;
    let monitor = vm.monitor_registry().monitor(
        ristretto_intrinsics::get_monitor_id(mirror)
            .ok_or_else(|| InternalError("missing monitor".to_owned()))?,
    );
    let method = class.try_get_method("caller", "()V")?;
    let call_thread = thread.clone();
    let call_class = class.clone();
    let mut execution = Box::pin(async move {
        call_thread
            .execute(&call_class, &method, &[] as &[Value])
            .await
    });
    suspend_at(execution.as_mut(), &thread, &class, "caller", 1, 8).await?;
    let observed = class.static_value("observed")?;
    let weak = ristretto_gc::GcWeak::new(
        reference_owner(&observed)?,
        ristretto_gc::ReferenceKind::Weak,
        0,
    );
    class.set_static_value("observed", Value::Object(None))?;
    drop(observed);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(
        weak.upgrade().is_some(),
        "pending call lost its operand/argument"
    );
    if entered {
        assert!(
            execution
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        assert_eq!(thread.frames()?.len(), 2);
        let runtime = tokio::runtime::Handle::current();
        execution = std::thread::spawn(move || {
            let _runtime = runtime.enter();
            assert!(
                execution
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
            execution
        })
        .join()
        .map_err(|_| InternalError("pending call migration panicked".to_owned()))?;
        gc.collect_full()?;
        gc.collect_full()?;
        assert!(weak.upgrade().is_some());
    }
    let unrelated = Value::try_new_object_async(gc, Reference::IntArray(vec![99].into())).await?;
    let unrelated_weak = ristretto_gc::GcWeak::new(
        reference_owner(&unrelated)?,
        ristretto_gc::ReferenceKind::Weak,
        0,
    );
    drop(unrelated);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(unrelated_weak.upgrade().is_none());
    let mut acquire = Box::pin(monitor.acquire(thread.id() + 1));
    assert!(
        acquire
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    drop(execution);
    assert!(thread.frames()?.is_empty());
    assert!(
        acquire
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_ready()
    );
    monitor.release(thread.id() + 1)?;
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(
        weak.upgrade().is_none(),
        "cancelled caller retained its roots"
    );
    assert_eq!(
        thread
            .execute(
                &class,
                &class.try_get_method("done", "()I")?,
                &[] as &[Value]
            )
            .await?,
        Some(Value::Int(42))
    );
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 1);
    #[cfg(not(feature = "execution-profile"))]
    let _ = function;
    Ok(())
}

#[test]
fn calls_cancel_before_or_during_interpreted_and_compiled_callees() -> Result<()> {
    const CHILD: &str = "RISTRETTO_CALL_CONTINUATION_CHILD";
    if std::env::var_os(CHILD).is_some() {
        return tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()?
            .block_on(async {
                for concurrent in [false, true] {
                    for workers in [0, 1] {
                        for compiled in [false, true] {
                            for entered in [false, true] {
                                cancel_call(concurrent, workers, compiled, entered).await?;
                            }
                        }
                    }
                }
                Ok(())
            });
    }
    let mut command = std::process::Command::new(std::env::current_exe()?);
    command.args(["--exact", "jit::static_call_tests::cooperation::calls_cancel_before_or_during_interpreted_and_compiled_callees", "--nocapture"]).env(CHILD, "1");
    ristretto_test_util::process::run(&mut command, Duration::from_secs(90))?.require_success()?;
    Ok(())
}

async fn initialization_classes(thread: &Thread) -> Result<(Arc<Class>, Arc<Class>)> {
    let mut pool = ConstantPool::new();
    let owner = pool.add_class("SuspendedInitialization")?;
    let count_field = field(&mut pool, "count", "I")?;
    let count = pool.add_field_ref(owner, "count", "I")?;
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
    let value = method(
        &mut pool,
        "value",
        "()I",
        vec![Instruction::Bipush(7), Instruction::Ireturn],
    )?;
    let target = with_field(
        thread,
        class_from_methods(
            "SuspendedInitialization",
            "java/lang/Object",
            pool,
            vec![initializer, value],
        )?,
        count_field,
    )
    .await?;
    let mut pool = ConstantPool::new();
    let target_ref = pool.add_class("SuspendedInitialization")?;
    let value = pool.add_method_ref(target_ref, "value", "()I")?;
    let run = method(
        &mut pool,
        "run",
        "()I",
        vec![Instruction::Invokestatic(value), Instruction::Ireturn],
    )?;
    let caller = register(
        thread,
        class_from_methods("InitializationCaller", "java/lang/Object", pool, vec![run])?,
    )
    .await?;
    Ok((target, caller))
}

async fn cancel_initialization(concurrent: bool, compiled: bool) -> Result<()> {
    use ristretto_classloader::InitializationState;
    let (vm, thread) = vm_with_workers(concurrent, 0).await?;
    let (target, caller) = initialization_classes(&thread).await?;
    assert_eq!(
        target.initialization_state()?,
        InitializationState::NotInitialized
    );
    if compiled {
        install_experimental(&vm, &caller, "run", "()I")?;
    }
    let method = caller.try_get_method("run", "()I")?;
    let parameters: [Value; 0] = [];
    let mut execution = Box::pin(thread.execute(&caller, &method, &parameters));
    let mut cx = Context::from_waker(Waker::noop());
    assert!(execution.as_mut().poll(&mut cx).is_pending());
    if compiled {
        // The first compiled slice yields before entering the call or initializer.
        assert_eq!(
            target.initialization_state()?,
            InitializationState::NotInitialized
        );
        assert_eq!(thread.frames()?.len(), 1);
        assert!(execution.as_mut().poll(&mut cx).is_pending());
    }
    assert_eq!(
        target.initialization_state()?,
        InitializationState::BeingInitialized {
            thread_id: thread.id()
        }
    );
    assert_eq!(target.static_value("count")?, Value::Int(1));
    assert_eq!(thread.frames()?.len(), 2);
    // Same-thread recursion and cancellation of a waiting thread do not own the claim.
    thread.initialize_class(&target).await?;
    let other = Thread::new(&Arc::downgrade(&vm), thread.id() + 1);
    let mut abandoned_waiter = Box::pin(other.initialize_class(&target));
    assert!(abandoned_waiter.as_mut().poll(&mut cx).is_pending());
    drop(abandoned_waiter);
    assert_eq!(
        target.initialization_state()?,
        InitializationState::BeingInitialized {
            thread_id: thread.id()
        }
    );
    let mut waiter = Box::pin(other.initialize_class(&target));
    assert!(waiter.as_mut().poll(&mut cx).is_pending());
    drop(execution);
    assert!(thread.frames()?.is_empty());
    assert!(
        matches!(target.initialization_state()?, InitializationState::Failed { error } if error == "Class initialization cancelled")
    );
    assert!(matches!(
        waiter.as_mut().poll(&mut cx),
        Poll::Ready(Err(crate::Error::JavaError(
            crate::JavaError::NoClassDefFoundError(_)
        )))
    ));
    assert!(matches!(
        thread.initialize_class(&target).await,
        Err(crate::Error::JavaError(
            crate::JavaError::NoClassDefFoundError(_)
        ))
    ));
    assert_eq!(
        target.static_value("count")?,
        Value::Int(1),
        "cancelled initializer replayed side effects"
    );
    Ok(())
}

#[test]
fn cancelling_initialization_releases_waiters_without_replaying_side_effects() -> Result<()> {
    const CHILD: &str = "RISTRETTO_CALL_INITIALIZATION_CHILD";
    if std::env::var_os(CHILD).is_some() {
        return tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()?
            .block_on(async {
                for concurrent in [false, true] {
                    for compiled in [false, true] {
                        cancel_initialization(concurrent, compiled).await?;
                    }
                }
                Ok(())
            });
    }
    let mut command = std::process::Command::new(std::env::current_exe()?);
    command.args(["--exact", "jit::static_call_tests::cooperation::cancelling_initialization_releases_waiters_without_replaying_side_effects", "--nocapture"]).env(CHILD, "1");
    ristretto_test_util::process::run(&mut command, Duration::from_secs(60))?.require_success()?;
    Ok(())
}
