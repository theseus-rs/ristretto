//! Type resolution must leave generated frames before it can wait.
use super::*;
use crate::jit::static_call_tests::{class_from_methods, install_experimental, method, register};
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::Instruction;

async fn fixture(thread: &Thread) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let target = pool.add_class("[I")?;
    let mut methods = Vec::new();
    for (name, descriptor, instruction, ret) in [
        (
            "cast",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            Instruction::Checkcast(target),
            Instruction::Areturn,
        ),
        (
            "test",
            "(Ljava/lang/Object;)I",
            Instruction::Instanceof(target),
            Instruction::Ireturn,
        ),
    ] {
        methods.push(method(
            &mut pool,
            name,
            descriptor,
            vec![Instruction::Aload_0, instruction, ret],
        )?);
    }
    register(
        thread,
        class_from_methods("TypeCheckContinuations", "java/lang/Object", pool, methods)?,
    )
    .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn type_checks_return_owning_requests() -> Result<()> {
    let (vm, thread) = crate::jit::static_call_tests::vm_with_workers(false, 0).await?;
    let class = fixture(&thread).await?;
    for (name, descriptor) in [
        ("cast", "(Ljava/lang/Object;)Ljava/lang/Object;"),
        ("test", "(Ljava/lang/Object;)I"),
    ] {
        let function = install_experimental(&vm, &class, name, descriptor)?;
        assert!(
            function.supports_cooperation(),
            "{name} cannot leave its native frame before type resolution"
        );
    }
    Ok(())
}

use ristretto_jit::{ExecutionStep, TypeCheckKind};
use std::future::Future;
use std::num::NonZeroU32;
use std::task::{Context, Waker};
use std::time::Duration;

fn owner(value: &Value) -> Result<&Gc<RwLock<Reference>>> {
    match value {
        Value::Object(Some(owner)) => Ok(owner),
        _ => Err(InternalError("missing type-check reference".to_owned())),
    }
}

fn collect(gc: &GarbageCollector) -> Result<()> {
    gc.collect_full()?;
    gc.collect_full()?;
    Ok(())
}

async fn vm(concurrent: bool, compact: bool, workers: usize) -> Result<(Arc<VM>, Arc<Thread>)> {
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
    let thread = Thread::new(&Arc::downgrade(&vm), 559);
    Ok((vm, thread))
}

/// The sole owner at a blocked resolution must be the published compiled frame.
#[expect(
    clippy::too_many_lines,
    reason = "ordered request, wait, migration, completion and cancellation proof"
)]
async fn suspended_resolution(
    vm: &Arc<VM>,
    thread: &Arc<Thread>,
    cast: bool,
    cancel: bool,
) -> Result<()> {
    // A distinct class identity keeps each resolution initially uncached.
    let name = format!("TypeWait{cast}{cancel}");
    let mut pool = ConstantPool::new();
    let target = pool.add_class("[I")?;
    let descriptor = if cast {
        "(Ljava/lang/Object;)Ljava/lang/Object;"
    } else {
        "(Ljava/lang/Object;)I"
    };
    let body = method(
        &mut pool,
        "run",
        descriptor,
        vec![
            Instruction::Aload_0,
            if cast {
                Instruction::Checkcast(target)
            } else {
                Instruction::Instanceof(target)
            },
            if cast {
                Instruction::Areturn
            } else {
                Instruction::Ireturn
            },
        ],
    )?;
    let class = register(
        thread,
        class_from_methods(&name, "java/lang/Object", pool, vec![body])?,
    )
    .await?;
    assert!(class.class_loader()?.is_none());
    let function = install_experimental(vm, &class, "run", descriptor)?;
    let gc = vm.garbage_collector();
    let context = RuntimeContext::new(gc, vm, thread, &class)?;
    let value = Value::try_new_object_async(gc, Reference::IntArray(vec![77].into())).await?;
    let handle = owner(&value)?.as_handle_i64();
    let weak = ristretto_gc::GcWeak::new(owner(&value)?, ristretto_gc::ReferenceKind::Weak, 0);
    let garbage = Value::try_new_object_async(gc, Reference::IntArray(vec![13].into())).await?;
    let garbage_weak =
        ristretto_gc::GcWeak::new(owner(&garbage)?, ristretto_gc::ReferenceKind::Weak, 0);
    drop(garbage);
    let mut activation = function.activation(&[ristretto_jit::Value::Ptr(handle)])?;
    let ExecutionStep::TypeCheck(request) =
        activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?
    else {
        return Err(InternalError("missing type-check request".to_owned()));
    };
    assert_eq!(
        request.kind,
        if cast {
            TypeCheckKind::CheckCast
        } else {
            TypeCheckKind::InstanceOf
        }
    );
    assert_eq!(request.index, target);
    assert_eq!(request.bci, 1);
    assert_eq!(activation.suspended_bci(), Some(1));
    assert_eq!(activation.remaining_budget()?, None);
    assert_eq!(activation.call_arguments()?, &[handle]);
    assert!(
        activation
            .resume_with_context(NonZeroU32::MIN, context.as_ptr())
            .is_err()
    );
    drop(value);
    // This is a real async loader wait, with no test hook inside the runtime request.
    let loader = vm.class_loader();
    let gate = loader.write().await;
    let mut operation = Box::pin(context.invoke_type_check(request, activation.call_arguments()?));
    let mut cx = Context::from_waker(Waker::noop());
    assert!(operation.as_mut().poll(&mut cx).is_pending());
    collect(gc)?;
    assert!(weak.upgrade().is_some());
    assert!(garbage_weak.upgrade().is_none());
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
    .map_err(|_| InternalError("type check migration panicked".to_owned()))?;
    if cancel {
        drop(operation);
        drop(activation);
        drop(context);
        collect(gc)?;
        assert!(
            weak.upgrade().is_none(),
            "cancelled resolution retained its operand"
        );
        drop(gate);
    } else {
        drop(gate);
        let bits = operation.await?;
        assert_eq!(context.pending_exception(), 0);
        assert_eq!(bits, if cast { handle } else { 1 });
        activation.complete_call(bits)?;
        assert!(activation.complete_call(bits).is_err());
        let expected = if cast {
            ristretto_jit::Value::Ptr(handle)
        } else {
            ristretto_jit::Value::I32(1)
        };
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?,
            ExecutionStep::Returned(Some(expected))
        );
        if cast {
            let result = context.into_reference_result(handle)?;
            collect(gc)?;
            assert_eq!(&*result.as_int_vec_ref()?, &[77]);
            drop(result);
        } else {
            drop(context);
        }
        drop(activation);
        collect(gc)?;
        assert!(weak.upgrade().is_none());
    }
    assert!(thread.frames()?.is_empty());
    Ok(())
}

const CAST: &str = "(Ljava/lang/Object;)Ljava/lang/Object;";
const TEST: &str = "(Ljava/lang/Object;)I";

#[expect(
    clippy::too_many_lines,
    reason = "one exported type-check and exception fixture"
)]
async fn semantic_fixture(thread: &Thread) -> Result<(Arc<Class>, Arc<Class>)> {
    let mut pool = ConstantPool::new();
    let initializer = method(&mut pool, "<clinit>", "()V", vec![Instruction::Goto(0)])?;
    let target = register(
        thread,
        class_from_methods(
            "UninitializedTypeTarget",
            "java/lang/Object",
            pool,
            vec![initializer],
        )?,
    )
    .await?;
    let mut pool = ConstantPool::new();
    let mut methods = Vec::new();
    for (label, name) in [
        ("Ints", "[I"),
        ("Objects", "[Ljava/lang/Object;"),
        ("Matrix", "[[I"),
        ("Object", "java/lang/Object"),
        ("Cloneable", "java/lang/Cloneable"),
        ("Serializable", "java/io/Serializable"),
        ("Uninitialized", "UninitializedTypeTarget"),
        ("Missing", "MissingTypeCheckTarget"),
    ] {
        let index = pool.add_class(name)?;
        for (prefix, descriptor, instruction, ret) in [
            (
                "cast",
                CAST,
                Instruction::Checkcast(index),
                Instruction::Areturn,
            ),
            (
                "test",
                TEST,
                Instruction::Instanceof(index),
                Instruction::Ireturn,
            ),
        ] {
            methods.push(method(
                &mut pool,
                &format!("{prefix}{label}"),
                descriptor,
                vec![Instruction::Aload_0, instruction, ret],
            )?);
        }
    }
    let object = pool.add_class("java/lang/Object")?;
    methods.push(method(
        &mut pool,
        "loop",
        "(Ljava/lang/Object;)V",
        vec![
            Instruction::Aload_0,
            Instruction::Instanceof(object),
            Instruction::Pop,
            Instruction::Goto(-5),
        ],
    )?);
    // Both a wide operand below the request and a live wide local must survive the join.
    methods.push(method(
        &mut pool,
        "wide",
        "(Ljava/lang/Object;J)J",
        vec![
            Instruction::Lload_1,
            Instruction::Aload_0,
            Instruction::Checkcast(object),
            Instruction::Pop,
            Instruction::Lload_1,
            Instruction::Ladd,
            Instruction::Lreturn,
        ],
    )?);
    let ints = pool.add_class("[I")?;
    let error = pool.add_class("java/lang/ClassCastException")?;
    let mut caught = method(
        &mut pool,
        "caught",
        TEST,
        vec![
            Instruction::Aload_0,
            Instruction::Checkcast(ints),
            Instruction::Pop,
            Instruction::Iconst_1,
            Instruction::Ireturn,
            Instruction::Pop,
            Instruction::Iconst_0,
            Instruction::Ireturn,
        ],
    )?;
    for attribute in &mut caught.attributes {
        if let ristretto_classfile::attributes::Attribute::Code {
            exception_table, ..
        } = attribute
        {
            exception_table.push(ristretto_classfile::attributes::ExceptionTableEntry {
                range_pc: 0..7,
                handler_pc: 7,
                catch_type: error,
            });
        }
    }
    methods.push(caught);
    let class = register(
        thread,
        class_from_methods("TypeCheckSemantics", "java/lang/Object", pool, methods)?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    Ok((class, target))
}

fn check_result(
    result: Result<Option<Value>>,
    input: &Value,
    cast: bool,
    matches: bool,
) -> Result<()> {
    if cast && !input.is_null() && !matches {
        let Err(crate::Error::Throwable(error)) = result else {
            return Err(InternalError(format!(
                "expected ClassCastException: {result:?}"
            )));
        };
        assert_eq!(
            error.as_object_ref()?.class().name(),
            "java/lang/ClassCastException"
        );
    } else {
        assert_eq!(
            result?,
            Some(if cast {
                input.clone()
            } else {
                Value::Int(i32::from(matches && !input.is_null()))
            })
        );
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "one interpreted, resumed and synchronous differential matrix"
)]
async fn semantics(vm: &Arc<VM>, thread: &Arc<Thread>) -> Result<()> {
    let (class, target) = semantic_fixture(thread).await?;
    let gc = vm.garbage_collector();
    let object = thread.class("java/lang/Object").await?;
    let mut values = vec![Value::Object(None)];
    for reference in [
        Reference::IntArray(vec![77].into()),
        Reference::LongArray(vec![78].into()),
        Reference::Object(Object::new_in_async(gc, object).await?),
        Reference::Object(Object::new_in_async(gc, target.clone()).await?),
    ] {
        values.push(Value::try_new_object_async(gc, reference).await?);
    }
    for name in ["[Ljava/lang/Object;", "[[I"] {
        let array = thread.class(name).await?;
        values.push(
            Value::try_new_object_async(
                gc,
                Reference::Array(ristretto_classloader::ObjectArray {
                    class: array,
                    elements: vec![Value::Object(None)].into(),
                }),
            )
            .await?,
        );
    }
    let ints = values
        .get(1)
        .ok_or_else(|| InternalError("missing int-array fixture".to_owned()))?;
    let longs = values
        .get(2)
        .ok_or_else(|| InternalError("missing long-array fixture".to_owned()))?;
    for (label, expected) in [
        ("Ints", [false, true, false, false, false, false, false]),
        ("Objects", [false, false, false, false, false, true, true]),
        ("Matrix", [false, false, false, false, false, false, true]),
        ("Object", [false, true, true, true, true, true, true]),
        ("Cloneable", [false, true, true, false, false, true, true]),
        (
            "Serializable",
            [false, true, true, false, false, true, true],
        ),
        (
            "Uninitialized",
            [false, false, false, false, true, false, false],
        ),
    ] {
        for cast in [false, true] {
            let name = format!("{}{label}", if cast { "cast" } else { "test" });
            let descriptor = if cast { CAST } else { TEST };
            let body = class.try_get_method(&name, descriptor)?;
            for compiled in [false, true] {
                let function = if compiled {
                    Some(install_experimental(vm, &class, &name, descriptor)?)
                } else {
                    None
                };
                for (input, matches) in values.iter().zip(expected) {
                    check_result(
                        thread
                            .execute(&class, &body, std::slice::from_ref(input))
                            .await,
                        input,
                        cast,
                        matches,
                    )?;
                    if let Some(function) = &function {
                        check_result(
                            crate::jit::execute(
                                function,
                                std::slice::from_ref(input),
                                gc,
                                vm,
                                thread,
                                &class,
                            ),
                            input,
                            cast,
                            matches,
                        )?;
                    }
                }
                assert!(
                    !target.is_initialized()?,
                    "type check initialized its target"
                );
            }
        }
    }
    for (name, descriptor) in [("castMissing", CAST), ("testMissing", TEST)] {
        let body = class.try_get_method(name, descriptor)?;
        for compiled in [false, true] {
            if compiled {
                install_experimental(vm, &class, name, descriptor)?;
            }
            check_result(
                thread.execute(&class, &body, &[Value::Object(None)]).await,
                &Value::Object(None),
                name.starts_with("cast"),
                false,
            )?;
        }
    }
    for (name, descriptor) in [("wide", "(Ljava/lang/Object;J)J"), ("caught", TEST)] {
        let body = class.try_get_method(name, descriptor)?;
        for compiled in [false, true] {
            if compiled {
                install_experimental(vm, &class, name, descriptor)?;
            }
            if name == "wide" {
                for bits in [i64::MIN, -1, 0, 42, i64::MAX] {
                    assert_eq!(
                        thread
                            .execute(&class, &body, &[ints.clone(), Value::Long(bits)])
                            .await?,
                        Some(Value::Long(bits.wrapping_mul(2)))
                    );
                }
            } else {
                for (input, expected) in [(ints.clone(), 1), (longs.clone(), 0)] {
                    assert_eq!(
                        thread.execute(&class, &body, &[input]).await?,
                        Some(Value::Int(expected))
                    );
                }
            }
        }
    }
    let function = install_experimental(vm, &class, "loop", "(Ljava/lang/Object;)V")?;
    let arguments = [ints.clone()];
    let body = class.try_get_method("loop", "(Ljava/lang/Object;)V")?;
    let mut execution = Box::pin(thread.execute(&class, &body, &arguments));
    let mut cx = Context::from_waker(Waker::noop());
    for _ in 0..3 {
        assert!(execution.as_mut().poll(&mut cx).is_pending());
    }
    #[cfg(feature = "execution-profile")]
    {
        assert_eq!(function.execution_count(), 1);
        assert!(function.resume_count() > 2);
    }
    drop(function);
    assert_eq!(thread.frames()?.len(), 1);
    drop(execution);
    assert!(thread.frames()?.is_empty());
    Ok(())
}

async fn matrix() -> Result<()> {
    for concurrent in [false, true] {
        for compact in [false, true] {
            for workers in [0, 1] {
                let (vm, thread) = vm(concurrent, compact, workers).await?;
                semantics(&vm, &thread).await?;
                for cast in [false, true] {
                    for cancel in [false, true] {
                        suspended_resolution(&vm, &thread, cast, cancel).await?;
                    }
                }
                println!(
                    "type-check matrix passed concurrent={concurrent} compact={compact} workers={workers}"
                );
            }
        }
    }
    Ok(())
}

#[test]
fn type_checks_preserve_semantics_and_suspended_roots() -> Result<()> {
    const CHILD: &str = "RISTRETTO_TYPE_CHECK_CONTINUATION_CHILD";
    if std::env::var_os(CHILD).is_some() {
        return tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()?
            .block_on(matrix());
    }
    let mut command = std::process::Command::new(std::env::current_exe()?);
    command.args(["--exact", "jit_runtime_helpers::type_check_continuation_tests::type_checks_preserve_semantics_and_suspended_roots", "--nocapture"]).env(CHILD,"1");
    let output = ristretto_test_util::process::run(&mut command, Duration::from_secs(180))?;
    output.require_success()?;
    println!("{}", output.stdout);
    Ok(())
}
