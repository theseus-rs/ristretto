//! Actual monitor ownership, blocking and cancellation across generated/interpreted code.
use super::*;
use crate::jit::static_call_tests::{
    class_from_methods, install_experimental, method, register, vm_with_workers,
};
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::{ExceptionTableEntry, Instruction, MaxStack};
use ristretto_classloader::Reference;
use std::future::Future;
use std::task::{Context, Poll, Waker};

fn handlers(
    pool: &ConstantPool,
    method: &mut ristretto_classfile::Method,
    entries: &[ExceptionTableEntry],
) -> Result<()> {
    for attribute in &mut method.attributes {
        if let Attribute::Code {
            code,
            exception_table,
            max_stack,
            ..
        } = attribute
        {
            entries.clone_into(exception_table);
            *max_stack = code.max_stack_with_exception_table(pool, exception_table)?;
        }
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "exact bytecode and handler ranges define the shared oracle fixture"
)]
async fn fixture(thread: &Thread) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let java_thread = pool.add_class("java/lang/Thread")?;
    let holds = pool.add_method_ref(java_thread, "holdsLock", "(Ljava/lang/Object;)Z")?;
    let npe = pool.add_class("java/lang/NullPointerException")?;
    let illegal = pool.add_class("java/lang/IllegalMonitorStateException")?;
    let balanced = method(
        &mut pool,
        "balanced",
        "(Ljava/lang/Object;)I",
        vec![
            Instruction::Aload_0,
            Instruction::Monitorenter,
            Instruction::Aload_0,
            Instruction::Monitorenter,
            Instruction::Aload_0,
            Instruction::Invokestatic(holds),
            Instruction::Istore_1,
            Instruction::Aload_0,
            Instruction::Monitorexit,
            Instruction::Aload_0,
            Instruction::Invokestatic(holds),
            Instruction::Iload_1,
            Instruction::Iadd,
            Instruction::Istore_1,
            Instruction::Aload_0,
            Instruction::Monitorexit,
            Instruction::Aload_0,
            Instruction::Invokestatic(holds),
            Instruction::Iload_1,
            Instruction::Iadd,
            Instruction::Ireturn,
        ],
    )?;
    let retain = method(
        &mut pool,
        "retain",
        "(Ljava/lang/Object;[I)[I",
        vec![
            Instruction::Aload_1,
            Instruction::Aload_0,
            Instruction::Monitorenter,
            Instruction::Aload_0,
            Instruction::Monitorexit,
            Instruction::Areturn,
        ],
    )?;
    let hold = method(
        &mut pool,
        "hold",
        "(Ljava/lang/Object;)V",
        vec![
            Instruction::Aload_0,
            Instruction::Monitorenter,
            Instruction::Aconst_null,
            Instruction::Astore_0,
            Instruction::Goto(0),
        ],
    )?;
    let mut methods = vec![balanced, retain, hold];
    for (name, descriptor, operand, opcode, catch_type, value) in [
        (
            "nullEnter",
            "()I",
            Instruction::Aconst_null,
            Instruction::Monitorenter,
            npe,
            7,
        ),
        (
            "nullExit",
            "()I",
            Instruction::Aconst_null,
            Instruction::Monitorexit,
            npe,
            8,
        ),
        (
            "illegalExit",
            "(Ljava/lang/Object;)I",
            Instruction::Aload_0,
            Instruction::Monitorexit,
            illegal,
            9,
        ),
    ] {
        let mut body = method(
            &mut pool,
            name,
            descriptor,
            vec![
                operand,
                opcode,
                Instruction::Iconst_0,
                Instruction::Ireturn,
                Instruction::Pop,
                Instruction::Bipush(value),
                Instruction::Ireturn,
            ],
        )?;
        handlers(
            &pool,
            &mut body,
            &[ExceptionTableEntry {
                range_pc: 0..2,
                handler_pc: 4,
                catch_type,
            }],
        )?;
        methods.push(body);
    }
    let mut spin = method(
        &mut pool,
        "spin",
        "([I)V",
        vec![
            Instruction::Aload_0,
            Instruction::Monitorenter, // 0..2
            Instruction::Aload_0,
            Instruction::Monitorenter, // 2..4
            Instruction::Aload_0,
            Instruction::Iconst_0,
            Instruction::Iaload,
            Instruction::Ifeq(-3), // 7 -> 4
            Instruction::Aload_0,
            Instruction::Monitorexit, // 10..12
            Instruction::Aload_0,
            Instruction::Monitorexit, // 12..14
            Instruction::Return,
            Instruction::Astore_1,
            Instruction::Aload_0,
            Instruction::Monitorexit,
            Instruction::Aload_0,
            Instruction::Monitorexit,
            Instruction::Aload_1,
            Instruction::Athrow,
        ],
    )?;
    handlers(
        &pool,
        &mut spin,
        &[ExceptionTableEntry {
            range_pc: 4..10,
            handler_pc: 15,
            catch_type: 0,
        }],
    )?;
    methods.push(spin);
    let class = register(
        thread,
        class_from_methods("MonitorContinuations", "java/lang/Object", pool, methods)?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    Ok(class)
}

fn owner(value: &Value) -> Result<&ristretto_gc::Gc<ristretto_gc::sync::RwLock<Reference>>> {
    match value {
        Value::Object(Some(owner)) => Ok(owner),
        _ => Err(InternalError("missing object".to_owned())),
    }
}

async fn semantics(concurrent: bool, workers: usize, compiled: bool) -> Result<()> {
    let (vm, thread) = vm_with_workers(concurrent, workers).await?;
    let class = fixture(&thread).await?;
    let gc = vm.garbage_collector();
    let array = Value::try_new_object_async(gc, Reference::IntArray(vec![0].into())).await?;
    let monitor = vm.monitor_registry().monitor(
        get_monitor_id(owner(&array)?)
            .ok_or_else(|| InternalError("missing monitor".to_owned()))?,
    );
    let mut functions = Vec::new();
    for (name, descriptor, parameters, expected) in [
        ("balanced", "(Ljava/lang/Object;)I", vec![array.clone()], 2),
        ("nullEnter", "()I", vec![], 7),
        ("nullExit", "()I", vec![], 8),
        (
            "illegalExit",
            "(Ljava/lang/Object;)I",
            vec![array.clone()],
            9,
        ),
    ] {
        if compiled {
            functions.push(install_experimental(&vm, &class, name, descriptor)?);
        }
        let body = class.try_get_method(name, descriptor)?;
        assert_eq!(
            thread.execute(&class, &body, &parameters).await?,
            Some(Value::Int(expected)),
            "{name}"
        );
        assert!(!monitor.is_owned_by(thread.id()));
        assert!(thread.frames()?.is_empty());
    }
    // Another owner is never released by a failed exit.
    monitor.acquire(999).await?;
    assert_eq!(
        thread
            .execute(
                &class,
                &class.try_get_method("illegalExit", "(Ljava/lang/Object;)I")?,
                std::slice::from_ref(&array)
            )
            .await?,
        Some(Value::Int(9))
    );
    assert!(monitor.is_owned_by(999));
    assert!(monitor.release(999)?);
    #[cfg(feature = "execution-profile")]
    for function in functions {
        assert!(function.execution_count() > 0);
    }
    Ok(())
}

async fn install_status_object(thread: &Thread) -> Result<()> {
    let mut pool = ConstantPool::new();
    let field = ristretto_classfile::Field {
        access_flags: FieldAccessFlags::PUBLIC,
        name_index: pool.add_utf8("threadStatus")?,
        descriptor_index: pool.add_utf8("I")?,
        field_type: FieldType::parse("I")?,
        attributes: vec![],
    };
    let mut definition = class_from_methods("MonitorStatus", "java/lang/Object", pool, vec![])?
        .class_file()
        .clone();
    definition.fields.push(field);
    let class = register(thread, Class::from(None, definition)?).await?;
    let vm = thread.vm()?;
    let object = Object::new_in_async(vm.garbage_collector(), class).await?;
    thread
        .set_java_object(Value::try_from_object_async(vm.garbage_collector(), object).await?)
        .await;
    Ok(())
}

async fn assert_status(thread: &Thread, expected: i32) -> Result<()> {
    assert_eq!(
        thread
            .java_object()
            .await
            .as_object_ref()?
            .value("threadStatus")?,
        Value::Int(expected)
    );
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "verify ownership throughout cancellation, not just the final free state"
)]
async fn cancellation(concurrent: bool, workers: usize, compiled: bool) -> Result<()> {
    let (vm, thread) = vm_with_workers(concurrent, workers).await?;
    let class = fixture(&thread).await?;
    install_status_object(&thread).await?;
    let function = if compiled {
        Some(install_experimental(&vm, &class, "spin", "([I)V")?)
    } else {
        None
    };
    let gc = vm.garbage_collector();
    let array = Value::try_new_object_async(gc, Reference::IntArray(vec![0].into())).await?;
    let weak = ristretto_gc::GcWeak::new(owner(&array)?, ristretto_gc::ReferenceKind::Weak, 0);
    let monitor = vm.monitor_registry().monitor(
        get_monitor_id(owner(&array)?)
            .ok_or_else(|| InternalError("missing monitor".to_owned()))?,
    );
    let body = class.try_get_method("spin", "([I)V")?;
    let parameters = [array];
    let mut execution = Box::pin(thread.execute(&class, &body, &parameters));
    assert!(
        execution
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    assert!(
        monitor.is_owned_by(thread.id()),
        "loop must really hold its object monitor"
    );
    // Migrate the suspended invocation and verify ownership follows the Java thread.
    let runtime = tokio::runtime::Handle::current();
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
    .map_err(|_| InternalError("monitor migration panicked".to_owned()))?;
    assert!(monitor.is_owned_by(thread.id()));
    assert_status(&thread, 0x04).await?;
    // An unrelated owner cannot enter while either reentrant entry remains.
    let mut contender = Box::pin(monitor.acquire(999));
    assert!(
        contender
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    let garbage = Value::try_new_object_async(gc, Reference::IntArray(vec![13].into())).await?;
    let garbage_weak =
        ristretto_gc::GcWeak::new(owner(&garbage)?, ristretto_gc::ReferenceKind::Weak, 0);
    drop(garbage);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(weak.upgrade().is_some());
    assert!(
        garbage_weak.upgrade().is_none(),
        "held Java monitor must not retain unrelated garbage"
    );
    drop(execution);
    assert!(thread.frames()?.is_empty());
    assert_eq!(thread.stack.read().used_slots, 0);
    assert!(!monitor.is_owned_by(thread.id()));
    assert!(matches!(
        contender
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop())),
        Poll::Ready(Ok(()))
    ));
    assert!(
        monitor.release(999)?,
        "cancellation must release both reentrant entries"
    );
    drop(contender);
    drop(parameters);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(
        weak.upgrade().is_none(),
        "cancelled monitor owner leaked its object"
    );
    #[cfg(feature = "execution-profile")]
    if let Some(function) = function {
        assert!(function.execution_count() > 0);
    }
    #[cfg(not(feature = "execution-profile"))]
    let _ = function;

    // Cancellation while queued must neither steal nor release another owner's lock.
    let array = Value::try_new_object_async(gc, Reference::IntArray(vec![1].into())).await?;
    let monitor = vm.monitor_registry().monitor(
        get_monitor_id(owner(&array)?)
            .ok_or_else(|| InternalError("missing monitor".to_owned()))?,
    );
    monitor.acquire(999).await?;
    let parameters = [array];
    let mut waiting = Box::pin(thread.execute(&class, &body, &parameters));
    assert!(
        waiting
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    assert_status(&thread, 0x400).await?;
    drop(waiting);
    assert_status(&thread, 0x04).await?;
    assert!(monitor.is_owned_by(999));
    assert!(thread.frames()?.is_empty());
    assert!(monitor.release(999)?);
    // The catch-all path must release both entries when array access throws.
    let empty = Value::try_new_object_async(gc, Reference::IntArray(vec![].into())).await?;
    let empty_monitor = vm.monitor_registry().monitor(
        get_monitor_id(owner(&empty)?)
            .ok_or_else(|| InternalError("missing monitor".to_owned()))?,
    );
    let result = thread.execute(&class, &body, &[empty]).await;
    assert!(
        matches!(result, Err(crate::Error::Throwable(_))),
        "{result:?}"
    );
    assert!(!empty_monitor.is_owned_by(thread.id()));
    empty_monitor.acquire(999).await?;
    assert!(empty_monitor.release(999)?);
    // Reuse the same logical thread and monitor after cancellation.
    assert_eq!(thread.execute(&class, &body, &parameters).await?, None);
    assert!(!monitor.is_owned_by(thread.id()));
    Ok(())
}

#[test]
fn explicit_monitors_own_contend_and_cancel() -> Result<()> {
    let mut command = std::process::Command::new(std::env::current_exe()?);
    command.args([
        "--exact",
        "thread::jit_monitor_tests::monitor_child",
        "--ignored",
        "--nocapture",
    ]);
    let output = ristretto_test_util::process::run(&mut command, Duration::from_secs(120))?;
    output.require_success()?;
    assert!(
        output
            .stdout
            .contains("monitor semantics and cancellation passed: 8 configurations")
    );
    println!("{}", output.stdout);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[ignore = "bounded subprocess fixture for compiled monitor waits and infinite loops"]
async fn monitor_child() -> Result<()> {
    for concurrent in [false, true] {
        for workers in [0, 1] {
            for compiled in [false, true] {
                semantics(concurrent, workers, compiled).await?;
                cancellation(concurrent, workers, compiled).await?;
                if compiled {
                    direct_roots(concurrent, workers).await?;
                }
                println!(
                    "monitor concurrent={concurrent} workers={workers} compiled={compiled} passed"
                );
            }
        }
    }
    println!("monitor semantics and cancellation passed: 8 configurations");
    Ok(())
}

/// Drop all Java argument owners and publish an empty generated snapshot while
/// the monitor is held. The monitor ledger alone must keep the object alive.
#[expect(
    clippy::too_many_lines,
    reason = "follow reference ownership from pending request through cancellation and return"
)]
async fn direct_roots(concurrent: bool, workers: usize) -> Result<()> {
    use crate::jit_runtime_helpers::RuntimeContext;
    use ristretto_jit::{ExecutionStep, MonitorKind};
    use std::num::NonZeroU32;
    let (vm, thread) = vm_with_workers(concurrent, workers).await?;
    let class = fixture(&thread).await?;
    let gc = vm.garbage_collector();
    let function = install_experimental(&vm, &class, "hold", "(Ljava/lang/Object;)V")?;
    let context = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let object = Value::try_new_object_async(gc, Reference::IntArray(vec![11].into())).await?;
    let weak = ristretto_gc::GcWeak::new(owner(&object)?, ristretto_gc::ReferenceKind::Weak, 0);
    let monitor = vm.monitor_registry().monitor(
        get_monitor_id(owner(&object)?)
            .ok_or_else(|| InternalError("missing monitor".to_owned()))?,
    );
    let mut activation =
        function.activation(&[ristretto_jit::Value::Ptr(owner(&object)?.as_handle_i64())])?;
    let ExecutionStep::Monitor(request) =
        activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?
    else {
        return Err(InternalError("missing monitor request".to_owned()));
    };
    assert_eq!(request.kind, MonitorKind::Enter);
    assert_eq!(request.bci, 1);
    assert!(!monitor.is_owned_by(thread.id()));
    drop(object);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(
        weak.upgrade().is_some(),
        "pending request lost its argument"
    );
    let boundary = ExecutionBoundary {
        thread: &thread,
        depth: 0,
        monitor_checkpoint: thread.explicit_monitors.checkpoint(),
        completed: false,
    };
    let bits = context
        .invoke_monitor(request, activation.call_arguments()?)
        .await?;
    activation.complete_call(bits)?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?,
        ExecutionStep::Yielded
    );
    assert!(monitor.is_owned_by(thread.id()));
    // No argument owner, local, operand or invocation context remains.
    drop(activation);
    drop(context);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(
        weak.upgrade().is_some(),
        "held monitor lost its object root"
    );
    // Cancelling an inner host scope releases its own reentrant entry only.
    {
        let inner = ExecutionBoundary {
            thread: &thread,
            depth: 0,
            monitor_checkpoint: thread.explicit_monitors.checkpoint(),
            completed: false,
        };
        thread
            .enter_monitor(
                &weak
                    .upgrade()
                    .ok_or_else(|| InternalError("missing held object".to_owned()))?,
            )
            .await?;
        drop(inner);
    }
    assert!(monitor.is_owned_by(thread.id()));
    let mut contender = Box::pin(monitor.acquire(999));
    assert!(
        contender
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    drop(boundary);
    assert!(matches!(
        contender
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop())),
        Poll::Ready(Ok(()))
    ));
    assert!(monitor.release(999)?);
    drop(contender);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(weak.upgrade().is_none());

    // Remaining reference operands are restored after each monitor request.
    let function = install_experimental(&vm, &class, "retain", "(Ljava/lang/Object;[I)[I")?;
    let lock = Value::try_new_object_async(gc, Reference::IntArray(vec![0].into())).await?;
    let result = Value::try_new_object_async(gc, Reference::IntArray(vec![42].into())).await?;
    let expected = ristretto_jit::Value::Ptr(owner(&result)?.as_handle_i64());
    let context = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let mut activation = function.activation(&[
        ristretto_jit::Value::Ptr(owner(&lock)?.as_handle_i64()),
        ristretto_jit::Value::Ptr(owner(&result)?.as_handle_i64()),
    ])?;
    let result_weak =
        ristretto_gc::GcWeak::new(owner(&result)?, ristretto_gc::ReferenceKind::Weak, 0);
    let mut inputs = Some((lock, result));
    for kind in [MonitorKind::Enter, MonitorKind::Exit] {
        let ExecutionStep::Monitor(request) =
            activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?
        else {
            return Err(InternalError("missing residual monitor request".to_owned()));
        };
        assert_eq!(request.kind, kind);
        drop(inputs.take());
        gc.collect_full()?;
        gc.collect_full()?;
        assert!(
            result_weak.upgrade().is_some(),
            "suspended residual reference was lost"
        );
        let bits = context
            .invoke_monitor(request, activation.call_arguments()?)
            .await?;
        activation.complete_call(bits)?;
        gc.collect_full()?;
    }
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?,
        ExecutionStep::Returned(Some(expected))
    );
    drop(activation);
    drop(context);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(
        result_weak.upgrade().is_none(),
        "completed residual operand remained rooted"
    );
    Ok(())
}
