//! Typed exception lookup must leave generated code before awaiting resolution.
use super::*;
use crate::jit::static_call_tests::install_experimental;
use crate::jit::static_call_tests::{class_from_methods, method, register, vm_with_workers};
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::{Attribute, ExceptionTableEntry, Instruction};
use ristretto_jit::ExecutionStep;
use std::future::Future;
use std::num::NonZeroU32;
use std::task::{Context, Waker};
use std::time::Duration;

async fn fixture(thread: &Thread) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let catch_type = pool.add_class("java/lang/Throwable")?;
    let mut run = method(
        &mut pool,
        "run",
        "(Ljava/lang/Throwable;)I",
        vec![
            Instruction::Aload_0,
            Instruction::Athrow,
            Instruction::Pop,
            Instruction::Iconst_1,
            Instruction::Ireturn,
        ],
    )?;
    for attribute in &mut run.attributes {
        if let Attribute::Code {
            exception_table, ..
        } = attribute
        {
            *exception_table = vec![ExceptionTableEntry {
                range_pc: 0..2,
                handler_pc: 2,
                catch_type,
            }];
        }
    }
    register(
        thread,
        class_from_methods("ExceptionMatchRequest", "java/lang/Object", pool, vec![run])?,
    )
    .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn typed_exception_matching_returns_an_owning_request() -> Result<()> {
    let (vm, thread) = vm_with_workers(false, 0).await?;
    let class = fixture(&thread).await?;
    let method = class.try_get_method("run", "(Ljava/lang/Throwable;)I")?;
    let function = ristretto_jit::Compiler::new()?.compile(
        class.class_file(),
        method.definition(),
        &symbols(),
    )?;
    let value = thread
        .object("java/lang/NullPointerException", "", &[] as &[Value])
        .await?;
    let Value::Object(Some(owner)) = value else {
        return Err(InternalError("missing throwable".into()));
    };
    let context = RuntimeContext::new(vm.garbage_collector(), &vm, &thread, &class)?;
    let mut activation =
        function.activation(&[ristretto_jit::Value::Ptr(owner.as_handle_i64())])?;
    let step = activation.resume_with_context(NonZeroU32::new(16).unwrap(), context.as_ptr())?;
    let ExecutionStep::ExceptionMatch(request) = step else {
        return Err(InternalError(format!(
            "typed lookup ran inside generated code: {step:?}"
        )));
    };
    assert_eq!(request.bci, 1);
    assert_eq!(activation.suspended_bci(), Some(1));
    assert_eq!(activation.call_arguments()?, &[] as &[i64]);
    let result = context
        .invoke_exception_match(request, activation.call_arguments()?)
        .await?;
    assert_eq!(result, 1);
    activation.complete_call(result)?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::new(16).unwrap(), context.as_ptr())?,
        ExecutionStep::Returned(Some(ristretto_jit::Value::I32(1)))
    );
    Ok(())
}

const DESCRIPTOR: &str = "(Ljava/lang/Throwable;Ljava/lang/Object;)Ljava/lang/Object;";

fn owner(value: &Value) -> Result<&Gc<RwLock<Reference>>> {
    match value {
        Value::Object(Some(owner)) => Ok(owner),
        _ => Err(InternalError("missing fixture owner".to_owned())),
    }
}

fn collect(gc: &GarbageCollector) -> Result<()> {
    gc.collect_full()?;
    gc.collect_full()?;
    Ok(())
}

async fn configured_vm(
    concurrent: bool,
    compact: bool,
    workers: usize,
) -> Result<(Arc<VM>, Arc<Thread>)> {
    let gc = GarbageCollector::with_config(
        ristretto_gc::Configuration::builder()
            .threads(workers)
            .concurrent_publication(concurrent)
            .compact_heap(compact)
            .root_storage(ristretto_gc::RootStorage::Contexts)
            .max_heap_bytes(32 * 1024 * 1024)
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
    let thread = Thread::new(&Arc::downgrade(&vm), 570);
    Ok((vm, thread))
}

async fn ordered_fixture(thread: &Thread, name: &str, first_name: &str) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let first = pool.add_class(first_name)?;
    let second = pool.add_class("java/lang/NullPointerException")?;
    let mut run = method(
        &mut pool,
        "run",
        DESCRIPTOR,
        vec![
            Instruction::Aload_0,
            Instruction::Aconst_null,
            Instruction::Astore_0,
            Instruction::Athrow, // 3: only the pending slot and published frame own the throwable
            Instruction::Pop,
            Instruction::Aload_1,
            Instruction::Areturn, // 4: matching handler
            Instruction::Pop,
            Instruction::Aconst_null,
            Instruction::Areturn, // 7: other handlers
        ],
    )?;
    for attribute in &mut run.attributes {
        if let Attribute::Code {
            exception_table,
            max_stack,
            ..
        } = attribute
        {
            *max_stack = 2;
            *exception_table = [first, first, second, 0]
                .into_iter()
                .map(|catch_type| ExceptionTableEntry {
                    range_pc: 0..4,
                    handler_pc: if catch_type == second { 4 } else { 7 },
                    catch_type,
                })
                .collect();
        }
    }
    register(
        thread,
        class_from_methods(name, "java/lang/Object", pool, vec![run])?,
    )
    .await
}

#[expect(
    clippy::too_many_lines,
    reason = "keep forced waiting, migration, cancellation and root checks in one schedule"
)]
async fn suspended_match(vm: &Arc<VM>, thread: &Arc<Thread>, cancel: bool) -> Result<()> {
    let gc = vm.garbage_collector();
    let class = ordered_fixture(
        thread,
        &format!("SuspendedExceptionMatch{cancel}"),
        "java/lang/ArithmeticException",
    )
    .await?;
    let function = install_experimental(vm, &class, "run", DESCRIPTOR)?;
    let context = RuntimeContext::new(gc, vm, thread, &class)?;
    let throwable = thread
        .object("java/lang/NullPointerException", "", &[] as &[Value])
        .await?;
    let local = Value::try_new_object_async(gc, Reference::IntArray(vec![77].into())).await?;
    let garbage = Value::try_new_object_async(gc, Reference::IntArray(vec![13].into())).await?;
    let weak_throwable =
        ristretto_gc::GcWeak::new(owner(&throwable)?, ristretto_gc::ReferenceKind::Weak, 0);
    let weak_local =
        ristretto_gc::GcWeak::new(owner(&local)?, ristretto_gc::ReferenceKind::Weak, 0);
    let weak_garbage =
        ristretto_gc::GcWeak::new(owner(&garbage)?, ristretto_gc::ReferenceKind::Weak, 0);
    let local_handle = owner(&local)?.as_handle_i64();
    let mut activation = function.activation(&[
        ristretto_jit::Value::Ptr(owner(&throwable)?.as_handle_i64()),
        ristretto_jit::Value::Ptr(local_handle),
    ])?;
    let ExecutionStep::ExceptionMatch(request) =
        activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?
    else {
        return Err(InternalError("missing typed match request".to_owned()));
    };
    assert_eq!(request.bci, 3);
    assert_eq!(activation.suspended_bci(), Some(3));
    assert_eq!(activation.remaining_budget()?, None);
    assert_eq!(activation.call_arguments()?, &[] as &[i64]);
    assert!(
        activation
            .resume_with_context(NonZeroU32::MIN, context.as_ptr())
            .is_err()
    );
    drop(throwable);
    drop(local);
    drop(garbage);
    let loader = vm.class_loader();
    let gate = loader.write().await;
    let mut operation =
        Box::pin(context.invoke_exception_match(request, activation.call_arguments()?));
    assert!(
        operation
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    collect(gc)?;
    assert!(weak_throwable.upgrade().is_some() && weak_local.upgrade().is_some());
    assert!(weak_garbage.upgrade().is_none());
    let runtime = Handle::current();
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                let _runtime = runtime.enter();
                assert!(
                    operation
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            })
            .join()
    })
    .map_err(|_| InternalError("exception lookup migration panicked".to_owned()))?;
    if cancel {
        drop(operation);
        drop(activation);
        drop(context);
        collect(gc)?;
        assert!(weak_throwable.upgrade().is_none() && weak_local.upgrade().is_none());
        drop(gate);
    } else {
        drop(gate);
        assert_eq!(operation.await?, 0);
        activation.complete_call(0)?;
        assert!(activation.complete_call(0).is_err());
        // Duplicate catch types have different resume sites at the same bytecode.
        for matches in [0, 1] {
            let ExecutionStep::ExceptionMatch(next) =
                activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?
            else {
                return Err(InternalError(
                    "ordered matching skipped a typed entry".to_owned(),
                ));
            };
            if matches == 0 {
                assert_eq!(next.index, request.index);
            } else {
                assert_ne!(next.index, request.index);
            }
            assert_eq!(activation.suspended_bci(), Some(3));
            assert_eq!(
                context
                    .invoke_exception_match(next, activation.call_arguments()?)
                    .await?,
                matches
            );
            collect(gc)?;
            assert!(weak_throwable.upgrade().is_some() && weak_local.upgrade().is_some());
            activation.complete_call(matches)?;
        }
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?,
            ExecutionStep::Yielded
        );
        assert_eq!(activation.suspended_bci(), Some(4));
        let (activation, context, step) = std::thread::spawn(move || -> Result<_> {
            let _runtime = runtime.enter();
            let step = activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?;
            Ok((activation, context, step))
        })
        .join()
        .map_err(|_| InternalError("handler resumption migration panicked".to_owned()))??;
        assert_eq!(
            step,
            ExecutionStep::Returned(Some(ristretto_jit::Value::Ptr(local_handle)))
        );
        assert_eq!(context.pending_exception(), 0);
        let result = context.into_reference_result(local_handle)?;
        drop(activation);
        collect(gc)?;
        assert!(weak_throwable.upgrade().is_none());
        assert_eq!(&*result.as_int_vec_ref()?, &[77]);
        drop(result);
        collect(gc)?;
        assert!(weak_local.upgrade().is_none());
    }
    assert_eq!(thread.frames()?.len(), 0);
    Ok(())
}

async fn assert_cached_failure(
    context: &RuntimeContext,
    body: &ristretto_classloader::Method,
    vm: &VM,
) -> Result<()> {
    // A repeated invocation-local failure must retain its Java error kind and
    // finish without revisiting a busy loader.
    let index = body
        .exception_table()
        .first()
        .ok_or_else(|| InternalError("missing typed fixture entry".to_owned()))?
        .catch_type;
    let loader = vm.class_loader();
    let gate = loader.write().await;
    let mut retry = Box::pin(resolve_class_ref_no_init_async(context, index));
    assert!(matches!(
        retry.as_mut().poll(&mut Context::from_waker(Waker::noop())),
        std::task::Poll::Ready(Err(JavaError(crate::JavaError::NoClassDefFoundError(_))))
    ));
    drop(retry);
    drop(gate);
    Ok(())
}

async fn semantics(vm: &Arc<VM>, thread: &Arc<Thread>) -> Result<()> {
    let gc = vm.garbage_collector();
    for missing in [false, true] {
        let class = ordered_fixture(
            thread,
            &format!("OrderedExceptionMatch{missing}"),
            if missing {
                "MissingExceptionMatchTarget"
            } else {
                "java/lang/ArithmeticException"
            },
        )
        .await?;
        let body = class.try_get_method("run", DESCRIPTOR)?;
        let local = Value::try_new_object_async(gc, Reference::IntArray(vec![99].into())).await?;
        for compiled in [false, true] {
            let function = compiled
                .then(|| install_experimental(vm, &class, "run", DESCRIPTOR))
                .transpose()?;
            for exception in [
                "java/lang/NullPointerException",
                "java/lang/ArithmeticException",
                "java/lang/Error",
            ] {
                let throwable = thread.object(exception, "", &[] as &[Value]).await?;
                let result = thread
                    .execute(&class, &body, &[throwable.clone(), local.clone()])
                    .await;
                if missing {
                    let Err(crate::Error::Throwable(error)) = result else {
                        return Err(InternalError(format!(
                            "missing catch type was caught: {result:?}"
                        )));
                    };
                    assert_eq!(
                        error.as_object_ref()?.class().name(),
                        "java/lang/NoClassDefFoundError"
                    );
                } else {
                    let result = result?
                        .ok_or_else(|| InternalError("missing handler result".to_owned()))?;
                    if exception == "java/lang/NullPointerException" {
                        assert_eq!(owner(&result)?.identity(), owner(&local)?.identity());
                    } else {
                        assert_eq!(result, Value::Object(None));
                    }
                }
                if let Some(function) = &function {
                    #[cfg(feature = "execution-profile")]
                    assert!(
                        function.execution_count() > 0,
                        "VM bypassed the installed compiled method"
                    );
                    let context = RuntimeContext::new(gc, vm, thread, &class)?;
                    let result = function.execute(
                        &[
                            ristretto_jit::Value::Ptr(owner(&throwable)?.as_handle_i64()),
                            ristretto_jit::Value::Ptr(owner(&local)?.as_handle_i64()),
                        ],
                        context.as_ptr(),
                    )?;
                    if missing {
                        assert_eq!(result, None);
                        let error = context.take_pending_exception_owner().ok_or_else(|| {
                            InternalError("missing synchronous resolution error".to_owned())
                        })?;
                        assert_eq!(
                            Value::Object(Some(error)).as_object_ref()?.class().name(),
                            "java/lang/NoClassDefFoundError"
                        );
                        assert_cached_failure(&context, &body, vm).await?;
                    } else {
                        assert_eq!(context.pending_exception(), 0);
                        assert_eq!(
                            result,
                            Some(ristretto_jit::Value::Ptr(
                                if exception == "java/lang/NullPointerException" {
                                    owner(&local)?.as_handle_i64()
                                } else {
                                    0
                                }
                            ))
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

async fn payload_wait(vm: &Arc<VM>, thread: &Arc<Thread>, cancel: bool) -> Result<()> {
    let gc = vm.garbage_collector();
    let class = fixture(thread).await?;
    let function = install_experimental(vm, &class, "run", "(Ljava/lang/Throwable;)I")?;
    let context = RuntimeContext::new(gc, vm, thread, &class)?;
    let throwable = thread
        .object("java/lang/NullPointerException", "", &[] as &[Value])
        .await?;
    let weak = ristretto_gc::GcWeak::new(owner(&throwable)?, ristretto_gc::ReferenceKind::Weak, 0);
    let mut activation = function.activation(&[ristretto_jit::Value::Ptr(
        owner(&throwable)?.as_handle_i64(),
    )])?;
    let ExecutionStep::ExceptionMatch(request) =
        activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?
    else {
        return Err(InternalError("missing payload match request".to_owned()));
    };
    resolve_class_ref_no_init_async(&context, request.index).await?;
    let guard = owner(&throwable)?.write();
    let mut operation =
        Box::pin(context.invoke_exception_match(request, activation.call_arguments()?));
    assert!(
        operation
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    // This verifies local request yielding only. Full collection through a held
    // Reference writer still needs the collector's separate stage-E protocol.
    if cancel {
        drop(operation);
        drop(activation);
        drop(context);
        drop(guard);
    } else {
        drop(guard);
        let bits = operation.await?;
        assert_eq!(bits, 1);
        activation.complete_call(bits)?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?,
            ExecutionStep::Yielded
        );
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?,
            ExecutionStep::Returned(Some(ristretto_jit::Value::I32(1)))
        );
        assert_eq!(context.pending_exception(), 0);
        drop(activation);
        drop(context);
    }
    drop(throwable);
    collect(gc)?;
    assert!(weak.upgrade().is_none());
    Ok(())
}

async fn catch_resolution_does_not_initialize(vm: &Arc<VM>, thread: &Arc<Thread>) -> Result<()> {
    let mut pool = ConstantPool::new();
    let initializer = method(&mut pool, "<clinit>", "()V", vec![Instruction::Goto(0)])?;
    let target = register(
        thread,
        class_from_methods(
            "DormantExceptionTarget",
            "java/lang/RuntimeException",
            pool,
            vec![initializer],
        )?,
    )
    .await?;
    let class = ordered_fixture(thread, "NoInitExceptionMatch", "DormantExceptionTarget").await?;
    let body = class.try_get_method("run", DESCRIPTOR)?;
    let throwable = thread
        .object("java/lang/NullPointerException", "", &[] as &[Value])
        .await?;
    let local =
        Value::try_new_object_async(vm.garbage_collector(), Reference::IntArray(vec![55].into()))
            .await?;
    for compiled in [false, true] {
        if compiled {
            install_experimental(vm, &class, "run", DESCRIPTOR)?;
        }
        let value = thread
            .execute(&class, &body, &[throwable.clone(), local.clone()])
            .await?
            .ok_or_else(|| InternalError("missing no-init result".to_owned()))?;
        assert_eq!(owner(&value)?.identity(), owner(&local)?.identity());
        assert!(!target.is_initialized()?, "catch resolution ran <clinit>");
    }
    Ok(())
}

async fn matrix() -> Result<()> {
    for concurrent in [false, true] {
        for compact in [false, true] {
            for workers in [0, 1] {
                let (vm, thread) = configured_vm(concurrent, compact, workers).await?;
                semantics(&vm, &thread).await?;
                catch_resolution_does_not_initialize(&vm, &thread).await?;
                for cancel in [false, true] {
                    suspended_match(&vm, &thread, cancel).await?;
                    payload_wait(&vm, &thread, cancel).await?;
                }
                println!(
                    "exception matrix passed concurrent={concurrent} compact={compact} workers={workers}"
                );
            }
        }
    }
    Ok(())
}

#[test]
fn exception_matching_preserves_order_and_suspended_roots() -> Result<()> {
    const CHILD: &str = "RISTRETTO_EXCEPTION_MATCH_CHILD";
    if std::env::var_os(CHILD).is_some() {
        return tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()?
            .block_on(matrix());
    }
    let mut command = std::process::Command::new(std::env::current_exe()?);
    command.args(["--exact", "jit_runtime_helpers::exception_continuation_tests::exception_matching_preserves_order_and_suspended_roots", "--nocapture"]).env(CHILD, "1");
    let output = ristretto_test_util::process::run(&mut command, Duration::from_secs(180))?;
    output.require_success()?;
    println!("{}", output.stdout);
    Ok(())
}
