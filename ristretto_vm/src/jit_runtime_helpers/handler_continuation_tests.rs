//! Exception-edge loops and handler calls must suspend with enumerable roots.
use super::*;
use crate::jit::static_call_tests::install_experimental;
use ristretto_classfile::attributes::{Attribute, ExceptionTableEntry, MaxStack};
use ristretto_classfile::{FieldAccessFlags, FieldType, MethodAccessFlags};
use std::future::Future;
use std::task::{Context, Poll, Waker};
use std::time::Duration;

fn handlers(
    pool: &ConstantPool,
    method: &mut ristretto_classfile::Method,
    entries: &[ExceptionTableEntry],
) -> Result<()> {
    for attribute in &mut method.attributes {
        if let Attribute::Code {
            exception_table,
            max_stack,
            code,
            ..
        } = attribute
        {
            *exception_table = entries.to_vec();
            *max_stack = code.max_stack_with_exception_table(pool, exception_table)?;
        }
    }
    Ok(())
}

async fn edge_class(thread: &Thread) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let arithmetic = pool.add_class("java/lang/ArithmeticException")?;
    let npe = pool.add_class("java/lang/NullPointerException")?;
    let code = vec![
        Instruction::Iconst_1,
        Instruction::Newarray(ArrayType::Int),
        Instruction::Astore_1,
        Instruction::Aload_1,
        Instruction::Iconst_0,
        Instruction::Bipush(77),
        Instruction::Iastore,
        Instruction::Goto(11), // 9 -> 20: initial throw
        Instruction::Pop,      // 12: handler; its operand is the pending throwable
        Instruction::Iinc(0, -1),
        Instruction::Iload_0,
        Instruction::Ifle(5),
        Instruction::Aconst_null,
        Instruction::Athrow, // 20..22 -> handler 12
        Instruction::Aload_1,
        Instruction::Areturn, // 22: only the handler reaches this
        Instruction::Pop,
        Instruction::Aconst_null,
        Instruction::Areturn, // 24: wrong type
    ];
    assert!(
        code.iter()
            .all(|instruction| instruction.branch_offsets().all(|offset| offset > 0))
    );
    let mut run = method(&mut pool, "run", "(I)[I", code)?;
    handlers(
        &pool,
        &mut run,
        &[
            ExceptionTableEntry {
                range_pc: 20..22,
                handler_pc: 24,
                catch_type: arithmetic,
            },
            ExceptionTableEntry {
                range_pc: 20..22,
                handler_pc: 12,
                catch_type: npe,
            },
        ],
    )?;
    register(
        thread,
        class_from_methods("HandlerEdgeLoop", "java/lang/Object", pool, vec![run])?,
    )
    .await
}

fn handler_roots(context: &RuntimeContext) -> Result<(WeakReference, WeakReference)> {
    let roots = context.reference_roots.lock();
    assert_eq!(
        roots.len(),
        2,
        "only the handler local and exception operand remain live"
    );
    let mut array = None;
    let mut exception = None;
    for owner in roots.values() {
        let weak = ristretto_gc::GcWeak::new(owner, ristretto_gc::ReferenceKind::Weak, 0);
        match &*owner.read() {
            Reference::IntArray(_) => array = Some(weak),
            Reference::Object(object) => {
                assert_eq!(object.class().name(), "java/lang/NullPointerException");
                exception = Some(weak);
            }
            other => return Err(InternalError(format!("unexpected handler root {other:?}"))),
        }
    }
    Ok((
        array.ok_or_else(|| InternalError("missing handler local".to_owned()))?,
        exception.ok_or_else(|| InternalError("missing handler exception".to_owned()))?,
    ))
}

async fn edge_lifetime(concurrent: bool, workers: usize, cancel: bool) -> Result<()> {
    let (vm, thread) = vm_with_workers(concurrent, workers).await?;
    let gc = vm.garbage_collector();
    let class = edge_class(&thread).await?;
    let method = class.try_get_method("run", "(I)[I")?;
    assert!(!ristretto_jit::Compiler::can_compile(method.definition()));
    let function = ristretto_jit::Compiler::new()?.compile(
        class.class_file(),
        method.definition(),
        &symbols(),
    )?;
    assert!(
        function.supports_cooperation(),
        "exception-only cycles need local polls"
    );
    let context = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let mut activation = function.activation(&[ristretto_jit::Value::I32(3)])?;
    assert_eq!(
        resume_allocations(&mut activation, &context).await?,
        ExecutionStep::Yielded
    );
    assert_eq!(activation.suspended_bci(), Some(12));
    let (array, mut previous) = handler_roots(&context)?;
    collect_twice(gc)?;
    assert!(array.upgrade().is_some() && previous.upgrade().is_some());
    if cancel {
        drop(activation);
        drop(context);
        collect_twice(gc)?;
        assert!(array.upgrade().is_none() && previous.upgrade().is_none());
        return Ok(());
    }
    let (mut activation, context, step) = migrate(activation, context)?;
    assert_eq!(step, ExecutionStep::Yielded);
    collect_twice(gc)?;
    assert!(
        previous.upgrade().is_none(),
        "previous exception stayed rooted"
    );
    (_, previous) = handler_roots(&context)?;
    assert_eq!(
        resume_allocations(&mut activation, &context).await?,
        ExecutionStep::Yielded
    );
    collect_twice(gc)?;
    assert!(previous.upgrade().is_none());
    (_, previous) = handler_roots(&context)?;
    let ExecutionStep::Returned(Some(ristretto_jit::Value::Ptr(handle))) =
        resume_allocations(&mut activation, &context).await?
    else {
        return Err(InternalError("handler did not return its local".to_owned()));
    };
    let result = context.into_reference_result(handle)?;
    drop(activation);
    collect_twice(gc)?;
    assert!(previous.upgrade().is_none());
    assert_eq!(&*result.as_int_vec_ref()?, &[77]);
    drop(result);
    collect_twice(gc)?;
    assert!(array.upgrade().is_none());
    #[cfg(feature = "execution-profile")]
    {
        assert_eq!(function.execution_count(), 1);
        // Initial allocation/store requests and three pairs of typed-handler
        // matches precede the three handler-entry suspensions and final return.
        assert_eq!(function.resume_count(), 11);
        // This counter deliberately counts explicit branch backedges only.
        assert_eq!(function.backedge_count(), 0);
    }
    Ok(())
}

#[test]
fn exception_only_cycles_yield_preserve_handler_roots_and_release_old_throwables() -> Result<()> {
    const CHILD: &str = "RISTRETTO_HANDLER_EDGE_CHILD";
    if std::env::var_os(CHILD).is_none() {
        let mut command = std::process::Command::new(std::env::current_exe()?);
        command.args(["--exact", "jit_runtime_helpers::continuation_tests::handlers::exception_only_cycles_yield_preserve_handler_roots_and_release_old_throwables", "--nocapture"]).env(CHILD, "1");
        ristretto_test_util::process::run(&mut command, Duration::from_secs(90))?
            .require_success()?;
        return Ok(());
    }
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?
        .block_on(async {
            for concurrent in [false, true] {
                for workers in [0, 1] {
                    for cancel in [false, true] {
                        edge_lifetime(concurrent, workers, cancel).await?;
                    }
                }
            }
            Ok(())
        })
}

#[expect(
    clippy::too_many_lines,
    reason = "keep fixture bytecode and its exception-table offsets together"
)]
async fn call_class(thread: &Thread) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let owner = pool.add_class("HandlerCall")?;
    let fail = pool.add_method_ref(owner, "fail", "(Ljava/lang/Throwable;)V")?;
    let pause = pool.add_method_ref(
        owner,
        "pause",
        "(Ljava/lang/Throwable;[IZ)Ljava/lang/Throwable;",
    )?;
    let array = pool.add_field_ref(owner, "array", "[I")?;
    let error = pool.add_field_ref(owner, "error", "Ljava/lang/Throwable;")?;
    let arithmetic = pool.add_class("java/lang/ArithmeticException")?;
    let npe = pool.add_class("java/lang/NullPointerException")?;
    let mut run = method(
        &mut pool,
        "run",
        "(Z)Ljava/lang/Throwable;",
        vec![
            Instruction::Iconst_1,
            Instruction::Newarray(ArrayType::Int),
            Instruction::Dup,
            Instruction::Putstatic(array),
            Instruction::Astore_1,
            Instruction::Aconst_null,
            Instruction::Invokestatic(fail), // 8, 9
            Instruction::Aconst_null,
            Instruction::Areturn,
            Instruction::Dup,
            Instruction::Putstatic(error),
            Instruction::Astore_2, // 14
            Instruction::Aload_2,
            Instruction::Aload_1,
            Instruction::Iload_0,
            Instruction::Invokestatic(pause),
            Instruction::Areturn, // 22, 25
            Instruction::Pop,
            Instruction::Aconst_null,
            Instruction::Areturn,
        ],
    )?;
    handlers(
        &pool,
        &mut run,
        &[
            ExceptionTableEntry {
                range_pc: 8..12,
                handler_pc: 26,
                catch_type: arithmetic,
            },
            ExceptionTableEntry {
                range_pc: 8..12,
                handler_pc: 14,
                catch_type: npe,
            },
        ],
    )?;
    run.access_flags.insert(MethodAccessFlags::SYNCHRONIZED);
    let fail = method(
        &mut pool,
        "fail",
        "(Ljava/lang/Throwable;)V",
        vec![Instruction::Aload_0, Instruction::Athrow],
    )?;
    let mut pause = method(
        &mut pool,
        "pause",
        "(Ljava/lang/Throwable;[IZ)Ljava/lang/Throwable;",
        vec![
            Instruction::Iload_2,
            Instruction::Ifeq(6),
            Instruction::Goto(0),
            Instruction::Aload_0,
            Instruction::Areturn,
        ],
    )?;
    pause.access_flags.insert(MethodAccessFlags::SYNCHRONIZED);
    let done = method(
        &mut pool,
        "done",
        "()I",
        vec![Instruction::Bipush(42), Instruction::Ireturn],
    )?;
    let mut fields = Vec::new();
    for (name, descriptor) in [("array", "[I"), ("error", "Ljava/lang/Throwable;")] {
        fields.push(ristretto_classfile::Field {
            access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
            name_index: pool.add_utf8(name)?,
            descriptor_index: pool.add_utf8(descriptor)?,
            field_type: FieldType::parse(descriptor)?,
            attributes: vec![],
        });
    }
    let class = class_from_methods(
        "HandlerCall",
        "java/lang/Object",
        pool,
        vec![run, fail, pause, done],
    )?;
    let mut definition = class.class_file().clone();
    definition.fields = fields;
    register(thread, Class::from(None, definition)?).await
}

fn weak(value: &Value) -> Result<WeakReference> {
    let Value::Object(Some(owner)) = value else {
        return Err(InternalError("missing reference".to_owned()));
    };
    Ok(ristretto_gc::GcWeak::new(
        owner,
        ristretto_gc::ReferenceKind::Weak,
        0,
    ))
}

#[expect(
    clippy::too_many_lines,
    reason = "ordered live-frame cancellation and ownership assertions"
)]
async fn handler_call(
    concurrent: bool,
    workers: usize,
    compiled: bool,
    entered: bool,
) -> Result<()> {
    let (vm, thread) = vm_with_workers(concurrent, workers).await?;
    let gc = vm.garbage_collector();
    let class = call_class(&thread).await?;
    let function = install_experimental(&vm, &class, "run", "(Z)Ljava/lang/Throwable;")?;
    assert!(function.supports_cooperation());
    for (name, descriptor) in [
        ("fail", "(Ljava/lang/Throwable;)V"),
        ("pause", "(Ljava/lang/Throwable;[IZ)Ljava/lang/Throwable;"),
    ] {
        if compiled {
            install_experimental(&vm, &class, name, descriptor)?;
        }
    }
    let method = class.try_get_method("run", "(Z)Ljava/lang/Throwable;")?;
    let value = thread
        .try_execute(&class, &method, &[Value::Int(0)])
        .await?;
    assert_eq!(
        value.as_object_ref()?.class().name(),
        "java/lang/NullPointerException"
    );
    assert_eq!(value, class.static_value("error")?);
    assert_eq!(&*class.static_value("array")?.as_int_vec_ref()?, &[0]);
    drop(value);
    class.set_static_value("error", Value::Object(None))?;
    class.set_static_value("array", Value::Object(None))?;
    let call_thread = thread.clone();
    let call_class = class.clone();
    let mut execution = Box::pin(async move {
        call_thread
            .execute(&call_class, &method, &[Value::Int(1)])
            .await
    });
    let mut cx = Context::from_waker(Waker::noop());
    for _ in 0..64 {
        assert!(execution.as_mut().poll(&mut cx).is_pending());
        if thread.frames()?.len() == 1 && thread.current_frame()?.program_counter() == 22 {
            break;
        }
    }
    assert_eq!(thread.frames()?.len(), 1);
    assert_eq!(thread.current_frame()?.program_counter(), 22);
    let array = weak(&class.static_value("array")?)?;
    let exception = weak(&class.static_value("error")?)?;
    class.set_static_value("array", Value::Object(None))?;
    class.set_static_value("error", Value::Object(None))?;
    collect_twice(gc)?;
    assert!(array.upgrade().is_some() && exception.upgrade().is_some());
    if entered {
        assert!(execution.as_mut().poll(&mut cx).is_pending());
        assert_eq!(thread.frames()?.len(), 2);
        let runtime = Handle::current();
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
        .map_err(|_| InternalError("handler call migration panicked".to_owned()))?;
    }
    let unrelated = Value::try_new_object_async(gc, Reference::IntArray(vec![99].into())).await?;
    let unrelated_weak = weak(&unrelated)?;
    drop(unrelated);
    collect_twice(gc)?;
    assert!(unrelated_weak.upgrade().is_none());
    assert!(array.upgrade().is_some() && exception.upgrade().is_some());
    let mirror = class
        .object()?
        .ok_or_else(|| InternalError("missing class mirror".to_owned()))?;
    let Value::Object(Some(mirror)) = mirror else {
        return Err(InternalError("invalid class mirror".to_owned()));
    };
    let monitor = vm.monitor_registry().monitor(
        ristretto_intrinsics::get_monitor_id(&mirror)
            .ok_or_else(|| InternalError("missing monitor".to_owned()))?,
    );
    let mut acquire = Box::pin(monitor.acquire(thread.id() + 1));
    assert!(acquire.as_mut().poll(&mut cx).is_pending());
    drop(execution);
    assert!(thread.frames()?.is_empty());
    assert!(matches!(
        acquire.as_mut().poll(&mut cx),
        Poll::Ready(Ok(()))
    ));
    monitor.release(thread.id() + 1)?;
    collect_twice(gc)?;
    assert!(array.upgrade().is_none() && exception.upgrade().is_none());
    assert_eq!(
        thread
            .try_execute(
                &class,
                &class.try_get_method("done", "()I")?,
                &[] as &[Value]
            )
            .await?,
        Value::Int(42)
    );
    Ok(())
}

#[test]
fn handler_calls_preserve_locals_exceptions_and_cancel_nested_frames() -> Result<()> {
    const CHILD: &str = "RISTRETTO_HANDLER_CALL_CHILD";
    if std::env::var_os(CHILD).is_none() {
        let mut command = std::process::Command::new(std::env::current_exe()?);
        command.args(["--exact", "jit_runtime_helpers::continuation_tests::handlers::handler_calls_preserve_locals_exceptions_and_cancel_nested_frames", "--nocapture"]).env(CHILD, "1");
        ristretto_test_util::process::run(&mut command, Duration::from_secs(90))?
            .require_success()?;
        return Ok(());
    }
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()?
        .block_on(async {
            for concurrent in [false, true] {
                for workers in [0, 1] {
                    for compiled in [false, true] {
                        for entered in [false, true] {
                            handler_call(concurrent, workers, compiled, entered).await?;
                        }
                    }
                }
            }
            Ok(())
        })
}
