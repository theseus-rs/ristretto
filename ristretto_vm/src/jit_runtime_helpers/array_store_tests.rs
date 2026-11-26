//! Real generated reference stores, shared semantics and pending-type ownership.
use super::*;
use crate::jit::static_call_tests::{class_from_methods, install_experimental, method, register};
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::{Attribute, ExceptionTableEntry, Instruction};
use ristretto_jit::ExecutionStep;
use std::future::Future;
use std::num::NonZeroU32;
use std::task::{Context, Waker};
use std::time::Duration;

const STORE: &str = "([Ljava/lang/Object;ILjava/lang/Object;)V";
const WIDE: &str = "([Ljava/lang/Object;ILjava/lang/Object;J)J";

async fn fixture(thread: &Thread) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let store = vec![
        Instruction::Aload_0,
        Instruction::Iload_1,
        Instruction::Aload_2,
        Instruction::Aastore,
    ];
    let mut simple = store.clone();
    simple.push(Instruction::Return);
    let mut wide = vec![Instruction::Lload_3];
    wide.extend(store.clone());
    wide.extend([
        Instruction::Lload_3,
        Instruction::Ladd,
        Instruction::Lreturn,
    ]);
    let mut caught = store;
    caught.extend([
        Instruction::Iconst_1,
        Instruction::Ireturn,
        Instruction::Pop,
        Instruction::Iconst_0,
        Instruction::Ireturn,
    ]);
    let catch_type = pool.add_class("java/lang/ArrayStoreException")?;
    let mut caught = method(
        &mut pool,
        "caught",
        "([Ljava/lang/Object;ILjava/lang/Object;)I",
        caught,
    )?;
    for attribute in &mut caught.attributes {
        if let Attribute::Code {
            exception_table, ..
        } = attribute
        {
            exception_table.push(ExceptionTableEntry {
                range_pc: 0..6,
                handler_pc: 6,
                catch_type,
            });
        }
    }
    let methods = vec![
        method(&mut pool, "store", STORE, simple)?,
        method(&mut pool, "wide", WIDE, wide)?,
        method(
            &mut pool,
            "loop",
            "([Ljava/lang/Object;Ljava/lang/Object;)V",
            vec![
                Instruction::Aload_0,
                Instruction::Iconst_0,
                Instruction::Aload_1,
                Instruction::Aastore,
                Instruction::Goto(-4),
            ],
        )?,
        caught,
    ];
    let class = register(
        thread,
        class_from_methods(
            "ReferenceStoreContinuations",
            "java/lang/Object",
            pool,
            methods,
        )?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    Ok(class)
}

fn owner(value: &Value) -> Result<&Gc<RwLock<Reference>>> {
    match value {
        Value::Object(Some(owner)) => Ok(owner),
        _ => Err(InternalError("missing store owner".to_owned())),
    }
}

async fn array(vm: &VM, thread: &Thread, name: &str) -> Result<Value> {
    let class = thread.class(name).await?;
    Ok(Value::try_new_object_async(
        vm.garbage_collector(),
        Reference::try_from((class, vec![Value::Object(None)]))?,
    )
    .await?)
}

fn collect(gc: &GarbageCollector) -> Result<()> {
    gc.collect_full()?;
    gc.collect_full()?;
    Ok(())
}

fn outcome(result: Result<Option<Value>>, expected: Option<&str>) -> Result<()> {
    if let Some(expected) = expected {
        let Err(crate::Error::Throwable(value)) = result else {
            return Err(InternalError(format!("missing {expected}: {result:?}")));
        };
        assert_eq!(value.as_object_ref()?.class().name(), expected);
    } else {
        assert_eq!(result?, None);
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "same values and bytecodes across interpreter, resumed and synchronous execution"
)]
async fn semantics(vm: &Arc<VM>, thread: &Arc<Thread>, class: &Arc<Class>) -> Result<()> {
    let gc = vm.garbage_collector();
    let target = array(vm, thread, "[Ljava/lang/String;").await?;
    let object = Value::try_new_object_async(
        gc,
        Reference::Object(Object::new_in_async(gc, thread.class("java/lang/Object").await?).await?),
    )
    .await?;
    let string = Value::try_new_object_async(
        gc,
        Reference::Object(Object::new_in_async(gc, thread.class("java/lang/String").await?).await?),
    )
    .await?;
    let body = class.try_get_method("store", STORE)?;
    for compiled in [false, true] {
        let function = if compiled {
            Some(install_experimental(vm, class, "store", STORE)?)
        } else {
            None
        };
        for (array, index, value, expected) in [
            (
                Value::Object(None),
                -1,
                object.clone(),
                Some("java/lang/NullPointerException"),
            ),
            (
                target.clone(),
                -1,
                object.clone(),
                Some("java/lang/ArrayIndexOutOfBoundsException"),
            ),
            (
                target.clone(),
                1,
                object.clone(),
                Some("java/lang/ArrayIndexOutOfBoundsException"),
            ),
            (
                target.clone(),
                0,
                object.clone(),
                Some("java/lang/ArrayStoreException"),
            ),
            (target.clone(), 0, string.clone(), None),
            (target.clone(), 0, Value::Object(None), None),
        ] {
            let arguments = [array, Value::Int(index), value.clone()];
            for synchronous in [false, true] {
                if synchronous && function.is_none() {
                    continue;
                }
                let old = target.as_class_vec_ref()?.1.first().cloned();
                let result = if let Some(function) = &function
                    && synchronous
                {
                    crate::jit::execute(function, &arguments, gc, vm, thread, class)
                } else {
                    thread.execute(class, &body, &arguments).await
                };
                outcome(result, expected)?;
                let stored = target.as_class_vec_ref()?.1.first().cloned();
                if expected.is_none() {
                    assert_eq!(stored, Some(value.clone()));
                } else {
                    assert_eq!(stored, old, "failed store changed array contents");
                }
            }
        }
    }
    let target = array(vm, thread, "[Ljava/lang/Object;").await?;
    for (name, descriptor) in [
        ("wide", WIDE),
        ("caught", "([Ljava/lang/Object;ILjava/lang/Object;)I"),
    ] {
        let body = class.try_get_method(name, descriptor)?;
        for compiled in [false, true] {
            if compiled {
                install_experimental(vm, class, name, descriptor)?;
            }
            if name == "wide" {
                for bits in [i64::MIN, -1, 0, 42, i64::MAX] {
                    assert_eq!(
                        thread
                            .execute(
                                class,
                                &body,
                                &[
                                    target.clone(),
                                    Value::Int(0),
                                    object.clone(),
                                    Value::Long(bits)
                                ]
                            )
                            .await?,
                        Some(Value::Long(bits.wrapping_mul(2)))
                    );
                }
            } else {
                let strings = array(vm, thread, "[Ljava/lang/String;").await?;
                assert_eq!(
                    thread
                        .execute(class, &body, &[strings, Value::Int(0), object.clone()])
                        .await?,
                    Some(Value::Int(0))
                );
                assert_eq!(
                    thread
                        .execute(
                            class,
                            &body,
                            &[target.clone(), Value::Int(0), object.clone()]
                        )
                        .await?,
                    Some(Value::Int(1))
                );
            }
        }
    }
    let function = install_experimental(
        vm,
        class,
        "loop",
        "([Ljava/lang/Object;Ljava/lang/Object;)V",
    )?;
    let body = class.try_get_method("loop", "([Ljava/lang/Object;Ljava/lang/Object;)V")?;
    let arguments = [target, object];
    let mut execution = Box::pin(thread.execute(class, &body, &arguments));
    for _ in 0..3 {
        assert!(
            execution
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
    }
    #[cfg(feature = "execution-profile")]
    {
        assert_eq!(function.execution_count(), 1);
        assert!(function.resume_count() > 2);
    }
    drop(function);
    drop(execution);
    assert!(thread.frames()?.is_empty());
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "pending store, guards, GC, migration, cancellation and commit form one schedule"
)]
async fn suspended(
    vm: &Arc<VM>,
    thread: &Arc<Thread>,
    class: &Arc<Class>,
    cancel: bool,
) -> Result<()> {
    let gc = vm.garbage_collector();
    let array = array(vm, thread, "[Ljava/lang/Object;").await?;
    let value = Value::try_new_object_async(gc, Reference::IntArray(vec![77].into())).await?;
    let handles = [
        owner(&array)?.as_handle_i64(),
        owner(&value)?.as_handle_i64(),
    ];
    let [array_handle, value_handle] = handles;
    let array_weak =
        ristretto_gc::GcWeak::new(owner(&array)?, ristretto_gc::ReferenceKind::Weak, 0);
    let value_weak =
        ristretto_gc::GcWeak::new(owner(&value)?, ristretto_gc::ReferenceKind::Weak, 0);
    let garbage = Value::try_new_object_async(gc, Reference::IntArray(vec![13].into())).await?;
    let garbage_weak =
        ristretto_gc::GcWeak::new(owner(&garbage)?, ristretto_gc::ReferenceKind::Weak, 0);
    drop(garbage);
    let function = install_experimental(vm, class, "store", STORE)?;
    let context = RuntimeContext::new(gc, vm, thread, class)?;
    let mut activation = function.activation(&[
        ristretto_jit::Value::Ptr(array_handle),
        ristretto_jit::Value::I32(0),
        ristretto_jit::Value::Ptr(value_handle),
    ])?;
    let ExecutionStep::ArrayStore(request) =
        activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?
    else {
        return Err(InternalError("missing array store request".to_owned()));
    };
    assert_eq!(request.bci, 3);
    assert_eq!(
        activation.call_arguments()?,
        &[array_handle, 0, value_handle]
    );
    assert_eq!(activation.remaining_budget()?, None);
    drop(array);
    drop(value);
    let loader = vm.class_loader();
    let gate = loader.write().await;
    let mut operation = Box::pin(context.invoke_array_store(request, activation.call_arguments()?));
    assert!(
        operation
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    // A resolution wait must leave the destination writable and unmodified.
    {
        let array = array_weak
            .upgrade()
            .ok_or_else(|| InternalError("lost pending array".to_owned()))?;
        let guard = array
            .try_write()
            .ok_or_else(|| InternalError("store retained array guard across await".to_owned()))?;
        let Reference::Array(array) = &*guard else {
            return Err(InternalError("wrong array kind".to_owned()));
        };
        assert!(array.elements.first().is_some_and(Value::is_null));
    }
    collect(gc)?;
    assert!(array_weak.upgrade().is_some());
    assert!(value_weak.upgrade().is_some());
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
    .map_err(|_| InternalError("store migration panicked".to_owned()))?;
    if cancel {
        drop(operation);
        drop(activation);
        drop(context);
        collect(gc)?;
        assert!(array_weak.upgrade().is_none());
        assert!(value_weak.upgrade().is_none());
        drop(gate);
    } else {
        let array = array_weak
            .upgrade()
            .ok_or_else(|| InternalError("lost completed array".to_owned()))?;
        drop(gate);
        activation.complete_call(operation.await?)?;
        assert_eq!(context.pending_exception(), 0);
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?,
            ExecutionStep::Returned(None)
        );
        drop(context);
        drop(activation);
        collect(gc)?;
        let array = Value::Object(Some(array));
        let stored = array
            .as_class_vec_ref()?
            .1
            .first()
            .cloned()
            .ok_or_else(|| InternalError("missing stored value".to_owned()))?;
        assert_eq!(owner(&stored)?.as_handle_i64(), value_handle);
        assert_eq!(&*stored.as_int_vec_ref()?, &[77]);
        drop(stored);
        drop(array);
        collect(gc)?;
        assert!(array_weak.upgrade().is_none());
        assert!(value_weak.upgrade().is_none());
    }
    Ok(())
}

async fn matrix() -> Result<()> {
    for concurrent in [false, true] {
        for compact in [false, true] {
            for workers in [0, 1] {
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
                let thread = Thread::new(&Arc::downgrade(&vm), 561);
                let class = fixture(&thread).await?;
                semantics(&vm, &thread, &class).await?;
                for cancel in [false, true] {
                    suspended(&vm, &thread, &class, cancel).await?;
                }
                println!(
                    "reference-store matrix passed concurrent={concurrent} compact={compact} workers={workers}"
                );
            }
        }
    }
    Ok(())
}

#[test]
fn reference_stores_suspend_and_preserve_java_order_and_roots() -> Result<()> {
    const CHILD: &str = "RISTRETTO_REFERENCE_STORE_CHILD";
    if std::env::var_os(CHILD).is_some() {
        return tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()?
            .block_on(matrix());
    }
    let mut command = std::process::Command::new(std::env::current_exe()?);
    command.args(["--exact", "jit_runtime_helpers::array_store_tests::reference_stores_suspend_and_preserve_java_order_and_roots", "--nocapture"]).env(CHILD,"1");
    let output = ristretto_test_util::process::run(&mut command, Duration::from_secs(180))?;
    output.require_success()?;
    println!("{}", output.stdout);
    Ok(())
}
