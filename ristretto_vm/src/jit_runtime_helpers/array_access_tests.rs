//! Generated array requests preserve bits, checks and ownership across runtime waits.
use super::*;
use crate::jit::static_call_tests::{class_from_methods, install_experimental, method, register};
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::{Attribute, ExceptionTableEntry, Instruction as I};
use ristretto_jit::{ArrayAccessKind as A, ExecutionStep};
use std::future::Future;
use std::num::NonZeroU32;
use std::task::{Context, Waker};
use std::time::Duration;

fn types() -> [(char, I, I, I, I); 9] {
    [
        ('Z', I::Baload, I::Bastore, I::Iload_2, I::Ireturn),
        ('B', I::Baload, I::Bastore, I::Iload_2, I::Ireturn),
        ('C', I::Caload, I::Castore, I::Iload_2, I::Ireturn),
        ('S', I::Saload, I::Sastore, I::Iload_2, I::Ireturn),
        ('I', I::Iaload, I::Iastore, I::Iload_2, I::Ireturn),
        ('J', I::Laload, I::Lastore, I::Lload_2, I::Lreturn),
        ('F', I::Faload, I::Fastore, I::Fload_2, I::Freturn),
        ('D', I::Daload, I::Dastore, I::Dload_2, I::Dreturn),
        ('A', I::Aaload, I::Aastore, I::Aload_2, I::Areturn),
    ]
}

fn component(kind: char) -> &'static str {
    match kind {
        'Z' => "Z",
        'B' => "B",
        'C' => "C",
        'S' => "S",
        'I' => "I",
        'J' => "J",
        'F' => "F",
        'D' => "D",
        _ => "Ljava/lang/Object;",
    }
}
fn value_type(kind: char) -> &'static str {
    match kind {
        'Z' | 'B' | 'C' | 'S' => "I",
        _ => component(kind),
    }
}
fn descriptor(kind: char, operation: &str) -> String {
    let c = component(kind);
    let v = value_type(kind);
    match operation {
        "length" => format!("([{c})I"),
        "load" => format!("([{c}I){v}"),
        "store" => format!("([{c}I{v})V"),
        _ => format!("([{c}I{v}){v}"),
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "one exported class covers all array element types"
)]
async fn fixture(thread: &Thread) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let mut methods = Vec::new();
    for (kind, load, store, argument, ret) in types() {
        methods.push(method(
            &mut pool,
            &format!("length_{kind}"),
            &descriptor(kind, "length"),
            vec![I::Aload_0, I::Arraylength, I::Ireturn],
        )?);
        methods.push(method(
            &mut pool,
            &format!("load_{kind}"),
            &descriptor(kind, "load"),
            vec![I::Aload_0, I::Iload_1, load.clone(), ret.clone()],
        )?);
        if kind != 'A' {
            methods.push(method(
                &mut pool,
                &format!("store_{kind}"),
                &descriptor(kind, "store"),
                vec![
                    I::Aload_0,
                    I::Iload_1,
                    argument.clone(),
                    store.clone(),
                    I::Return,
                ],
            )?);
            // A residual value and the same local survive both requests. Returning
            // the residual after discarding the read proves float/wide state transfer.
            let pop = if matches!(kind, 'J' | 'D') {
                I::Pop2
            } else {
                I::Pop
            };
            methods.push(method(
                &mut pool,
                &format!("wide_{kind}"),
                &descriptor(kind, "wide"),
                vec![
                    argument.clone(),
                    I::Aload_0,
                    I::Iload_1,
                    argument,
                    store,
                    I::Aload_0,
                    I::Iload_1,
                    load,
                    pop,
                    ret,
                ],
            )?);
        }
    }
    let mut caught = method(
        &mut pool,
        "caught",
        "([II)I",
        vec![
            I::Aload_0,
            I::Iload_1,
            I::Iaload,
            I::Ireturn,
            I::Pop,
            I::Iconst_m1,
            I::Ireturn,
        ],
    )?;
    let catch_type = pool.add_class("java/lang/ArrayIndexOutOfBoundsException")?;
    for attribute in &mut caught.attributes {
        if let Attribute::Code {
            exception_table, ..
        } = attribute
        {
            exception_table.push(ExceptionTableEntry {
                range_pc: 0..4,
                handler_pc: 4,
                catch_type,
            });
        }
    }
    methods.push(caught);
    methods.push(method(
        &mut pool,
        "loop",
        "([I)V",
        vec![
            I::Aload_0,
            I::Iconst_0,
            I::Iconst_1,
            I::Iastore,
            I::Goto(-4),
        ],
    )?);
    let class = register(
        thread,
        class_from_methods(
            "ArrayAccessContinuations",
            "java/lang/Object",
            pool,
            methods,
        )?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    Ok(class)
}

async fn array(vm: &VM, thread: &Thread, kind: char) -> Result<Value> {
    let reference = match kind {
        'Z' => Reference::BooleanArray(vec![0].into()),
        'B' => Reference::ByteArray(vec![0].into()),
        'C' => Reference::CharArray(vec![0].into()),
        'S' => Reference::ShortArray(vec![0].into()),
        'I' => Reference::IntArray(vec![11].into()),
        'J' => Reference::LongArray(vec![0].into()),
        'F' => Reference::FloatArray(vec![0.0].into()),
        'D' => Reference::DoubleArray(vec![0.0].into()),
        _ => Reference::try_from((
            thread.class("[Ljava/lang/Object;").await?,
            vec![Value::Object(None)],
        ))?,
    };
    Ok(Value::try_new_object_async(vm.garbage_collector(), reference).await?)
}
fn owner(value: &Value) -> Result<&Gc<RwLock<Reference>>> {
    match value {
        Value::Object(Some(owner)) => Ok(owner),
        _ => Err(InternalError("missing array owner".to_owned())),
    }
}
fn collect(gc: &GarbageCollector) -> Result<()> {
    for _ in 0..2 {
        gc.collect();
        let mut completed = false;
        for _ in 0..100_000 {
            if gc.step() {
                completed = true;
                break;
            }
        }
        if !completed {
            return Err(InternalError(
                "full collection did not complete within the fixture work bound".to_owned(),
            ));
        }
    }
    Ok(())
}
fn weak(value: &Value) -> Result<ristretto_gc::GcWeak<RwLock<Reference>>> {
    Ok(ristretto_gc::GcWeak::new(
        owner(value)?,
        ristretto_gc::ReferenceKind::Weak,
        0,
    ))
}
fn same(actual: Option<Value>, expected: &Value) {
    match (actual, expected) {
        (Some(Value::Float(a)), Value::Float(b)) => assert_eq!(a.to_bits(), b.to_bits()),
        (Some(Value::Double(a)), Value::Double(b)) => assert_eq!(a.to_bits(), b.to_bits()),
        (a, b) => assert_eq!(a, Some(b.clone())),
    }
}
fn failure(result: Result<Option<Value>>, class: &str) -> Result<()> {
    let Err(crate::Error::Throwable(value)) = result else {
        return Err(InternalError(format!("missing {class}: {result:?}")));
    };
    assert_eq!(value.as_object_ref()?.class().name(), class);
    Ok(())
}

async fn execute(
    vm: &Arc<VM>,
    thread: &Arc<Thread>,
    class: &Arc<Class>,
    name: &str,
    descriptor: &str,
    arguments: &[Value],
    mode: u8,
) -> Result<Option<Value>> {
    let body = class.try_get_method(name, descriptor)?;
    if mode == 0 {
        return thread.execute(class, &body, arguments).await;
    }
    let function = match vm
        .compiler()
        .and_then(|compiler| compiler.get_cached(Arc::as_ptr(&body) as usize))
        .flatten()
    {
        Some(function) => function,
        None => install_experimental(vm, class, name, descriptor)?,
    };
    assert!(
        function.supports_cooperation(),
        "{name} requires an owning request"
    );
    if mode == 1 {
        thread.execute(class, &body, arguments).await
    } else {
        crate::jit::execute(
            &function,
            arguments,
            vm.garbage_collector(),
            vm,
            thread,
            class,
        )
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "same fixtures across all element types and execution modes"
)]
async fn semantics(vm: &Arc<VM>, thread: &Arc<Thread>, class: &Arc<Class>) -> Result<()> {
    for (kind, _, _, _, _) in types() {
        let target = array(vm, thread, kind).await?;
        let values = match kind {
            'J' => vec![
                Value::Long(i64::MIN),
                Value::Long(-1),
                Value::Long(0),
                Value::Long(i64::MAX),
            ],
            'F' => [0, 0x8000_0000, 1, 0x7f80_0000, 0x7fc0_1234]
                .map(|bits| Value::Float(f32::from_bits(bits)))
                .to_vec(),
            'D' => [
                0,
                0x8000_0000_0000_0000,
                1,
                0x7ff0_0000_0000_0000,
                0x7ff8_1234_5678_9abc,
            ]
            .map(|bits| Value::Double(f64::from_bits(bits)))
            .to_vec(),
            'A' => vec![Value::Object(None)],
            _ => [i32::MIN, -65537, -1, 0, 2, 3, 128, 255, 65535, i32::MAX]
                .map(Value::Int)
                .to_vec(),
        };
        for mode in 0..3 {
            same(
                execute(
                    vm,
                    thread,
                    class,
                    &format!("length_{kind}"),
                    &descriptor(kind, "length"),
                    std::slice::from_ref(&target),
                    mode,
                )
                .await?,
                &Value::Int(1),
            );
            failure(
                execute(
                    vm,
                    thread,
                    class,
                    &format!("length_{kind}"),
                    &descriptor(kind, "length"),
                    &[Value::Object(None)],
                    mode,
                )
                .await,
                "java/lang/NullPointerException",
            )?;
            for value in &values {
                if kind != 'A' {
                    same(
                        execute(
                            vm,
                            thread,
                            class,
                            &format!("wide_{kind}"),
                            &descriptor(kind, "wide"),
                            &[target.clone(), Value::Int(0), value.clone()],
                            mode,
                        )
                        .await?,
                        value,
                    );
                }
                let expected = match value {
                    Value::Int(v) => Value::Int(match kind {
                        'Z' => v & 1,
                        'B' => i32::from(*v as i8),
                        'C' => i32::from(*v as u16),
                        'S' => i32::from(*v as i16),
                        _ => *v,
                    }),
                    value => value.clone(),
                };
                same(
                    execute(
                        vm,
                        thread,
                        class,
                        &format!("load_{kind}"),
                        &descriptor(kind, "load"),
                        &[target.clone(), Value::Int(0)],
                        mode,
                    )
                    .await?,
                    &expected,
                );
            }
            for (array, index, error) in [
                (
                    target.clone(),
                    -1,
                    "java/lang/ArrayIndexOutOfBoundsException",
                ),
                (
                    target.clone(),
                    1,
                    "java/lang/ArrayIndexOutOfBoundsException",
                ),
                (Value::Object(None), -1, "java/lang/NullPointerException"),
            ] {
                failure(
                    execute(
                        vm,
                        thread,
                        class,
                        &format!("load_{kind}"),
                        &descriptor(kind, "load"),
                        &[array.clone(), Value::Int(index)],
                        mode,
                    )
                    .await,
                    error,
                )?;
                if kind != 'A' {
                    let before = execute(
                        vm,
                        thread,
                        class,
                        &format!("load_{kind}"),
                        &descriptor(kind, "load"),
                        &[target.clone(), Value::Int(0)],
                        mode,
                    )
                    .await?
                    .ok_or_else(|| InternalError("missing element".to_owned()))?;
                    failure(
                        execute(
                            vm,
                            thread,
                            class,
                            &format!("store_{kind}"),
                            &descriptor(kind, "store"),
                            &[
                                array,
                                Value::Int(index),
                                values.first().cloned().ok_or_else(|| {
                                    InternalError("missing fixture value".to_owned())
                                })?,
                            ],
                            mode,
                        )
                        .await,
                        error,
                    )?;
                    same(
                        execute(
                            vm,
                            thread,
                            class,
                            &format!("load_{kind}"),
                            &descriptor(kind, "load"),
                            &[target.clone(), Value::Int(0)],
                            mode,
                        )
                        .await?,
                        &before,
                    );
                }
            }
        }
    }
    let target = array(vm, thread, 'I').await?;
    for mode in 0..3 {
        same(
            execute(
                vm,
                thread,
                class,
                "caught",
                "([II)I",
                &[target.clone(), Value::Int(1)],
                mode,
            )
            .await?,
            &Value::Int(-1),
        );
    }
    let function = install_experimental(vm, class, "loop", "([I)V")?;
    let body = class.try_get_method("loop", "([I)V")?;
    let arguments = [target];
    let mut operation = Box::pin(thread.execute(class, &body, &arguments));
    for _ in 0..3 {
        assert!(
            operation
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
    drop(operation);
    assert!(thread.frames()?.is_empty());
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "one forced schedule covers payload/error waiting, GC, migration, cancellation and completion"
)]
async fn suspended(
    vm: &Arc<VM>,
    thread: &Arc<Thread>,
    class: &Arc<Class>,
    kind: A,
    payload: bool,
    cancel: bool,
) -> Result<()> {
    println!("array-access schedule kind={kind:?} payload={payload} cancel={cancel}");
    let gc = vm.garbage_collector();
    let element = if kind == A::ReferenceLoad { 'A' } else { 'I' };
    let array = array(vm, thread, element).await?;
    let array_weak = weak(&array)?;
    let garbage = garbage(gc).await?;
    let garbage_weak = weak(&garbage)?;
    drop(garbage);
    let operation = match kind {
        A::Length => "length",
        A::IntStore => "store",
        _ => "load",
    };
    let function = install_experimental(
        vm,
        class,
        &format!("{operation}_{element}"),
        &descriptor(element, operation),
    )?;
    let context = RuntimeContext::new(gc, vm, thread, class)?;
    let raw = owner(&array)?.as_handle_i64();
    let mut arguments = vec![ristretto_jit::Value::Ptr(
        if !payload && kind == A::Length {
            0
        } else {
            raw
        },
    )];
    if kind != A::Length {
        arguments.push(ristretto_jit::Value::I32(if payload { 0 } else { -1 }));
    }
    if kind == A::IntStore {
        arguments.push(ristretto_jit::Value::I32(77));
    }
    let mut activation = function.activation(&arguments)?;
    let ExecutionStep::ArrayAccess(request) =
        activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?
    else {
        return Err(InternalError("missing array-access request".to_owned()));
    };
    assert_eq!(request.kind, kind);
    assert_eq!(request.bci, kind.argument_count());
    assert_eq!(activation.remaining_budget()?, None);
    let held_owner = if payload {
        Some(owner(&array)?.clone())
    } else {
        None
    };
    let guard = held_owner
        .as_ref()
        .filter(|_| kind != A::IntStore)
        .map(|owner| owner.write());
    let read_guard = held_owner
        .as_ref()
        .filter(|_| kind == A::IntStore)
        .map(|owner| owner.read());
    let loader = vm.class_loader();
    let gate = if payload {
        None
    } else {
        Some(loader.write().await)
    };
    drop(array);
    let mut future = Box::pin(context.invoke_array_access(request, activation.call_arguments()?));
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending(),
        "{kind:?} payload={payload}"
    );
    if !payload && kind != A::Length {
        let owner = array_weak
            .upgrade()
            .ok_or_else(|| InternalError("lost pending array".to_owned()))?;
        assert!(
            owner.try_write().is_some(),
            "throwable construction retained array guard"
        );
    }
    if guard.is_some() {
        // Reference is an enum that may contain edges. Its held writer still
        // excludes tracing, even for a primitive-array variant. Record bounded
        // collector probes; this is an open D/E limitation, not a passing GC gate.
        gc.collect();
        let completed = (0..20_000).any(|_| gc.step());
        println!("held Reference writer full-cycle completed={completed}");
    } else {
        collect(gc)?;
        assert!(garbage_weak.upgrade().is_none());
    }
    if kind != A::Length || payload {
        assert!(array_weak.upgrade().is_some());
    }
    let runtime = Handle::current();
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                let _entered = runtime.enter();
                assert!(
                    future
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            })
            .join()
    })
    .map_err(|_| InternalError("array request migration panicked".to_owned()))?;
    if cancel {
        drop(future);
        drop(activation);
        drop(context);
        // Cancellation itself completes while the contended resource remains held.
        if !payload {
            collect(gc)?;
            assert!(array_weak.upgrade().is_none());
        }
        drop(guard);
        drop(read_guard);
        drop(held_owner);
        drop(gate);
    } else {
        drop(guard);
        drop(read_guard);
        drop(gate);
        activation.complete_call(future.await?)?;
        if payload {
            assert_eq!(context.pending_exception(), 0);
        } else {
            assert_ne!(context.pending_exception(), 0);
        }
        let result = activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?;
        assert_eq!(
            result,
            ExecutionStep::Returned(if payload {
                match kind {
                    A::Length => Some(ristretto_jit::Value::I32(1)),
                    A::IntLoad => Some(ristretto_jit::Value::I32(11)),
                    _ => None,
                }
            } else {
                None
            })
        );
        if payload && kind == A::IntStore {
            let value = Value::Object(held_owner.clone());
            assert_eq!(&*value.as_int_vec_ref()?, &[77]);
        }
        drop(context);
        drop(activation);
        drop(held_owner);
    }
    collect(gc)?;
    assert!(array_weak.upgrade().is_none());
    assert!(garbage_weak.upgrade().is_none());
    Ok(())
}

async fn garbage(gc: &Arc<GarbageCollector>) -> Result<Value> {
    Ok(Value::try_new_object_async(gc, Reference::IntArray(vec![13].into())).await?)
}

async fn reference_result(vm: &Arc<VM>, thread: &Arc<Thread>, class: &Arc<Class>) -> Result<()> {
    let gc = vm.garbage_collector();
    let child = garbage(gc).await?;
    let child_weak = weak(&child)?;
    let array = Value::try_new_object_async(
        gc,
        Reference::try_from((
            thread.class("[Ljava/lang/Object;").await?,
            vec![child.clone()],
        ))?,
    )
    .await?;
    let array_weak = weak(&array)?;
    let function = install_experimental(vm, class, "load_A", &descriptor('A', "load"))?;
    let context = RuntimeContext::new(gc, vm, thread, class)?;
    let mut activation = function.activation(&[
        ristretto_jit::Value::Ptr(owner(&array)?.as_handle_i64()),
        ristretto_jit::Value::I32(0),
    ])?;
    let ExecutionStep::ArrayAccess(request) =
        activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?
    else {
        return Err(InternalError("missing reference load".to_owned()));
    };
    let bits = context
        .invoke_array_access(request, activation.call_arguments()?)
        .await?;
    assert_eq!(bits, owner(&child)?.as_handle_i64());
    drop(child);
    drop(array);
    collect(gc)?;
    assert!(child_weak.upgrade().is_some());
    activation.complete_call(bits)?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?,
        ExecutionStep::Returned(Some(ristretto_jit::Value::Ptr(bits)))
    );
    let returned = context.into_reference_result(bits)?;
    drop(activation);
    collect(gc)?;
    assert!(array_weak.upgrade().is_none());
    assert_eq!(&*returned.as_int_vec_ref()?, &[13]);
    drop(returned);
    collect(gc)?;
    assert!(child_weak.upgrade().is_none());
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
                let thread = Thread::new(&Arc::downgrade(&vm), 562);
                let class = fixture(&thread).await?;
                semantics(&vm, &thread, &class).await?;
                reference_result(&vm, &thread, &class).await?;
                for cancel in [false, true] {
                    for kind in [A::Length, A::IntLoad, A::IntStore, A::ReferenceLoad] {
                        suspended(&vm, &thread, &class, kind, false, cancel).await?;
                    }
                    for kind in [A::Length, A::IntLoad, A::IntStore] {
                        suspended(&vm, &thread, &class, kind, true, cancel).await?;
                    }
                }
                println!(
                    "array-access matrix passed concurrent={concurrent} compact={compact} workers={workers}"
                );
            }
        }
    }
    Ok(())
}

#[test]
fn array_accesses_suspend_and_preserve_bits_checks_and_roots() -> Result<()> {
    const CHILD: &str = "RISTRETTO_ARRAY_ACCESS_CHILD";
    if std::env::var_os(CHILD).is_some() {
        return tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()?
            .block_on(matrix());
    }
    let mut command = std::process::Command::new(std::env::current_exe()?);
    command.args(["--exact","jit_runtime_helpers::array_access_tests::array_accesses_suspend_and_preserve_bits_checks_and_roots","--nocapture"]).env(CHILD,"1");
    let output = ristretto_test_util::process::run(&mut command, Duration::from_secs(300))?;
    output.require_success()?;
    println!("{}", output.stdout);
    Ok(())
}
