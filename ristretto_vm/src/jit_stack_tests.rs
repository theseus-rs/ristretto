//! Native compiled and mixed frames must share the bounded Java stack budget.
use super::*;
use crate::jit::static_call_tests::{class_from_methods, method, register, vm_with_workers};
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::Instruction;
use ristretto_classloader::Reference;

fn pad_locals(method: &mut ristretto_classfile::Method, slots: u16) {
    for attribute in &mut method.attributes {
        if let Attribute::Code { max_locals, .. } = attribute {
            *max_locals = (*max_locals).max(slots);
        }
    }
}

fn assert_empty(thread: &Thread) {
    let stack = thread.stack.read();
    assert!(stack.entries.is_empty());
    assert_eq!(stack.used_slots, 0);
    assert_eq!(stack.overflow_reserve_depth, 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn production_compiled_entry_enforces_the_java_stack_budget() -> Result<()> {
    let (vm, thread) = vm_with_workers(true, 0).await?;
    let mut pool = ConstantPool::new();
    let mut value = method(
        &mut pool,
        "value",
        "()I",
        vec![Instruction::Bipush(7), Instruction::Ireturn],
    )?;
    pad_locals(&mut value, 128);
    let class = register(
        &thread,
        class_from_methods("CompiledStackBounds", "java/lang/Object", pool, vec![value])?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    let value = class.try_get_method("value", "()I")?;
    assert!(ristretto_jit::Compiler::can_compile(value.definition()));
    let function = vm
        .compiler()
        .ok_or_else(|| InternalError("missing compiler".to_owned()))?
        .compile(&class, &value)
        .await?
        .ok_or_else(|| InternalError("production compilation failed".to_owned()))?;
    thread.stack.write().max_slots = 8;
    let result = thread.execute(&class, &value, &[] as &[Value]).await;
    assert!(
        matches!(result, Err(crate::Error::JavaError(StackOverflowError(_)))),
        "{result:?}"
    );
    assert_empty(&thread);
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 0);
    thread.stack.write().max_slots = Frame::stack_slots_for(&value)?;
    for _ in 0..2 {
        assert_eq!(
            thread.execute(&class, &value, &[] as &[Value]).await?,
            Some(Value::Int(7))
        );
        assert_empty(&thread);
    }
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 2);
    #[cfg(not(feature = "execution-profile"))]
    let _ = function;
    Ok(())
}

async fn recursive_class(thread: &Thread, synchronized: bool) -> Result<Arc<Class>> {
    let name = format!("CompiledStackRecursion{synchronized}");
    let mut pool = ConstantPool::new();
    let target = pool.add_class(&name)?;
    let descriptor = "(I[I)[I";
    let even = pool.add_method_ref(target, "even", descriptor)?;
    let odd = pool.add_method_ref(target, "odd", descriptor)?;
    let overflow = pool.add_class("java/lang/StackOverflowError")?;
    let mut methods = Vec::new();
    for (name, reference) in [("even", odd), ("odd", even)] {
        let mut value = method(
            &mut pool,
            name,
            descriptor,
            vec![
                Instruction::Iload_0,
                Instruction::Ifeq(11),
                Instruction::Iload_0,
                Instruction::Iconst_1,
                Instruction::Isub,
                Instruction::Aload_1,
                Instruction::Invokestatic(reference),
                Instruction::Areturn,
                Instruction::Aload_1,
                Instruction::Areturn,
            ],
        )?;
        if synchronized {
            value.access_flags.insert(MethodAccessFlags::SYNCHRONIZED);
        }
        methods.push(value);
    }
    let mut caught = method(
        &mut pool,
        "caught",
        "(I[I)Ljava/lang/Throwable;",
        vec![
            Instruction::Iload_0,
            Instruction::Aload_1,
            Instruction::Invokestatic(even),
            Instruction::Pop,
            Instruction::Aconst_null,
            Instruction::Areturn,
            Instruction::Areturn,
        ],
    )?;
    for attribute in &mut caught.attributes {
        if let Attribute::Code {
            exception_table, ..
        } = attribute
        {
            exception_table.push(ristretto_classfile::attributes::ExceptionTableEntry {
                range_pc: 2..5,
                handler_pc: 8,
                catch_type: overflow,
            });
        }
    }
    methods.push(caught);
    // The native-capacity fixture invokes these bodies through the synchronous
    // embedding API, independently of which opcodes support VM continuations.
    let caught_ref = pool.add_method_ref(target, "caught", "(I[I)Ljava/lang/Throwable;")?;
    for (name, descriptor, reference) in [
        ("nativeEven", "(I[I)[I", even),
        ("nativeCaught", "(I[I)Ljava/lang/Throwable;", caught_ref),
    ] {
        let bridge = native_bridge(&mut pool, name, descriptor, reference)?;
        methods.push(bridge);
    }
    let class = register(
        thread,
        class_from_methods(&name, "java/lang/Object", pool, methods)?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    Ok(class)
}

fn native_bridge(
    pool: &mut ConstantPool,
    name: &str,
    descriptor: &str,
    reference: u16,
) -> Result<ristretto_classfile::Method> {
    let java_thread = pool.add_class("java/lang/Thread")?;
    let holds_lock = pool.add_method_ref(java_thread, "holdsLock", "(Ljava/lang/Object;)Z")?;
    let mut bridge = method(
        pool,
        name,
        descriptor,
        vec![
            Instruction::Aload_1,
            Instruction::Dup,
            Instruction::Astore_2,
            Instruction::Monitorenter,
            Instruction::Aload_1, // prove the monitor was acquired, not merely free at exit
            Instruction::Invokestatic(holds_lock),
            Instruction::Ifne(5),
            Instruction::Aconst_null,
            Instruction::Athrow,
            Instruction::Iload_0,
            Instruction::Aload_1,
            Instruction::Invokestatic(reference),
            Instruction::Aload_2,
            Instruction::Monitorexit,
            Instruction::Areturn,
            Instruction::Astore_3,
            Instruction::Aload_2,
            Instruction::Monitorexit,
            Instruction::Aload_3,
            Instruction::Athrow,
        ],
    )?;
    for attribute in &mut bridge.attributes {
        if let Attribute::Code {
            exception_table, ..
        } = attribute
        {
            exception_table.push(ristretto_classfile::attributes::ExceptionTableEntry {
                range_pc: 4..20,
                handler_pc: 21,
                catch_type: 0,
            });
        }
    }
    Ok(bridge)
}

fn install_recursive(
    vm: &VM,
    class: &Arc<Class>,
    modes: [bool; 3],
) -> Result<Vec<Arc<ristretto_jit::Function>>> {
    let mut functions = Vec::new();
    for ((name, descriptor), compiled) in [
        ("even", "(I[I)[I"),
        ("odd", "(I[I)[I"),
        ("caught", "(I[I)Ljava/lang/Throwable;"),
    ]
    .into_iter()
    .zip(modes)
    {
        if compiled {
            functions.push(jit::static_call_tests::install_experimental(
                vm, class, name, descriptor,
            )?);
        }
    }
    Ok(functions)
}

fn reference(value: &Value) -> Result<&ristretto_gc::Gc<ristretto_gc::sync::RwLock<Reference>>> {
    match value {
        Value::Object(Some(owner)) => Ok(owner),
        _ => Err(InternalError("missing recursive reference".to_owned())),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mixed_recursive_frames_share_budget_and_release_roots_after_overflow() -> Result<()> {
    for concurrent in [false, true] {
        for even in [false, true] {
            for odd in [false, true] {
                for caught in [false, true] {
                    recursive_overflow(concurrent, [even, odd, caught], false).await?;
                }
            }
        }
    }
    Ok(())
}

async fn recursive_overflow(concurrent: bool, modes: [bool; 3], synchronized: bool) -> Result<()> {
    let (vm, thread) = vm_with_workers(concurrent, 0).await?;
    let gc = vm.garbage_collector();
    let class = recursive_class(&thread, synchronized).await?;
    let functions = install_recursive(&vm, &class, modes)?;
    let even = class.try_get_method("even", "(I[I)[I")?;
    let caught = class.try_get_method("caught", "(I[I)Ljava/lang/Throwable;")?;
    let acquired = thread.acquire_sync_monitor(&class, &even, &[]).await?;
    let monitor = acquired.0.as_ref().map(|guard| guard.monitor.clone());
    assert_eq!(
        monitor.is_some(),
        synchronized,
        "synchronized method needs a monitor"
    );
    drop(acquired);
    let array = Value::try_new_object_async(gc, Reference::IntArray(vec![77].into())).await?;
    let weak = ristretto_gc::GcWeak::new(reference(&array)?, ristretto_gc::ReferenceKind::Weak, 0);
    let normal = thread
        .try_execute(&class, &even, &[Value::Int(2), array.clone()])
        .await?;
    assert_eq!(
        reference(&normal)?.as_handle_i64(),
        reference(&array)?.as_handle_i64()
    );
    drop(normal);
    assert_empty(&thread);
    #[cfg(feature = "execution-profile")]
    let before: Vec<_> = functions
        .iter()
        .map(|function| function.execution_count())
        .collect();
    let recursive_slots = Frame::stack_slots_for(&even)?;
    let original_limit = thread.stack.read().max_slots;
    let mut errors = Vec::new();
    for catch in [true, false] {
        thread.stack.write().max_slots = recursive_slots * 2
            + if catch {
                Frame::stack_slots_for(&caught)?
            } else {
                0
            };
        let method = if catch { &caught } else { &even };
        let result = thread
            .execute(&class, method, &[Value::Int(8), array.clone()])
            .await;
        let error = match result {
            Ok(Some(value)) if catch => value,
            Err(crate::Error::Throwable(value)) if !catch => value,
            other => return Err(InternalError(format!("wrong recursion result: {other:?}"))),
        };
        assert_eq!(
            error.as_object_ref()?.class().name(),
            "java/lang/StackOverflowError"
        );
        assert_empty(&thread);
        if let Some(monitor) = &monitor {
            assert!(!monitor.is_owned_by(thread.id()));
            timeout_at(
                Instant::now() + Duration::from_secs(1),
                monitor.acquire(999),
            )
            .await
            .map_err(|_| InternalError("recursive monitor leaked".to_owned()))??;
            assert!(monitor.release(999)?);
        }
        check_backtrace(&error, catch)?;
        errors.push(error);
    }
    thread.stack.write().max_slots = original_limit;
    #[cfg(feature = "execution-profile")]
    for ((function, prior), expected) in functions.iter().zip(before).zip(
        modes
            .into_iter()
            .zip([2, 2, 1])
            .filter_map(|(compiled, count)| compiled.then_some(count)),
    ) {
        assert_eq!(function.execution_count() - prior, expected);
    }
    #[cfg(not(feature = "execution-profile"))]
    let _ = functions;
    drop(array);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(
        weak.upgrade().is_none(),
        "recursive argument root survived unwind"
    );
    check_error_owners(gc, errors)?;
    let array = Value::try_new_object_async(gc, Reference::IntArray(vec![88].into())).await?;
    assert_eq!(
        &[88],
        &*thread
            .try_execute(&class, &even, &[Value::Int(3), array])
            .await?
            .as_int_vec_ref()?
    );
    assert_empty(&thread);
    Ok(())
}

fn check_error_owners(gc: &Arc<ristretto_gc::GarbageCollector>, errors: Vec<Value>) -> Result<()> {
    let error_weak: Vec<_> = errors
        .iter()
        .map(|error| {
            Ok(ristretto_gc::GcWeak::new(
                reference(error)?,
                ristretto_gc::ReferenceKind::Weak,
                0,
            ))
        })
        .collect::<Result<_>>()?;
    for error in &errors {
        assert_eq!(
            error.as_object_ref()?.class().name(),
            "java/lang/StackOverflowError"
        );
    }
    drop(errors);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(error_weak.iter().all(|weak| weak.upgrade().is_none()));
    Ok(())
}

fn check_backtrace(error: &Value, caught: bool) -> Result<()> {
    let backtrace = error.as_object_ref()?.value("backtrace")?;
    let entries = backtrace.as_class_vec_ref()?.1.to_vec();
    let expected = if caught {
        vec![
            ("odd", "(I[I)[I", 8),
            ("even", "(I[I)[I", 8),
            ("caught", "(I[I)Ljava/lang/Throwable;", 2),
        ]
    } else {
        vec![("odd", "(I[I)[I", 8), ("even", "(I[I)[I", 8)]
    };
    assert_eq!(entries.len(), expected.len());
    for (entry, (name, descriptor, offset)) in entries.iter().zip(expected) {
        let fields = entry.as_class_vec_ref()?.1;
        let [_, actual_name, actual_descriptor, pc] = &*fields else {
            return Err(InternalError("invalid backtrace entry".to_owned()));
        };
        assert_eq!(actual_name.as_string()?, name);
        assert_eq!(actual_descriptor.as_string()?, descriptor);
        assert_eq!(pc.as_i32()?, offset);
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn synchronized_mixed_frames_release_every_monitor_entry_after_overflow() -> Result<()> {
    for modes in [
        [false, true, false],
        [true, false, true],
        [true, true, true],
    ] {
        recursive_overflow(true, modes, true).await?;
    }
    Ok(())
}

async fn pressure_class(thread: &Thread) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let target = pool.add_class("CompiledStackPressure")?;
    let even = pool.add_method_ref(target, "even", "(II)I")?;
    let odd = pool.add_method_ref(target, "odd", "(II)I")?;
    let mut methods = Vec::new();
    for (name, reference) in [("even", odd), ("odd", even)] {
        methods.push(method(
            &mut pool,
            name,
            "(II)I",
            vec![
                Instruction::Iconst_1,
                Instruction::Newarray(ristretto_classfile::attributes::ArrayType::Int),
                Instruction::Dup,
                Instruction::Iconst_0,
                Instruction::Iload_0,
                Instruction::Iconst_1,
                Instruction::Iadd,
                Instruction::Iastore,
                Instruction::Astore_2,
                Instruction::Iload_0,
                Instruction::Ifeq(15),
                Instruction::Iload_0,
                Instruction::Iconst_1,
                Instruction::Isub,
                Instruction::Iload_1,
                Instruction::Invokestatic(reference),
                Instruction::Aload_2,
                Instruction::Iconst_0,
                Instruction::Iaload,
                Instruction::Iadd,
                Instruction::Ireturn,
                Instruction::Iload_1,
                Instruction::Ifeq(15),
                Instruction::Sipush(2048),
                Instruction::Newarray(ristretto_classfile::attributes::ArrayType::Int),
                Instruction::Pop,
                Instruction::Iinc(1, -1),
                Instruction::Goto(-13),
                Instruction::Aload_2,
                Instruction::Iconst_0,
                Instruction::Iaload,
                Instruction::Ireturn,
            ],
        )?);
    }
    let class = register(
        thread,
        class_from_methods("CompiledStackPressure", "java/lang/Object", pool, methods)?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    Ok(class)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn recursive_frames_keep_distinct_arrays_live_during_leaf_reclamation() -> Result<()> {
    for concurrent in [false, true] {
        for workers in [0, 1] {
            for compiled in [[false, true], [true, false], [true, true]] {
                recursive_pressure(concurrent, workers, compiled).await?;
            }
        }
    }
    Ok(())
}

async fn recursive_pressure(concurrent: bool, workers: usize, compiled: [bool; 2]) -> Result<()> {
    let (vm, thread) = vm_with_workers(concurrent, workers).await?;
    let gc = vm.garbage_collector();
    let class = pressure_class(&thread).await?;
    let mut functions = Vec::new();
    for (name, enabled) in ["even", "odd"].into_iter().zip(compiled) {
        if enabled {
            functions.push(jit::static_call_tests::install_experimental(
                &vm, &class, name, "(II)I",
            )?);
        }
    }
    let even = class.try_get_method("even", "(II)I")?;
    thread.stack.write().max_slots = Frame::stack_slots_for(&even)? * 5;
    let before = gc.statistics()?;
    let diagnostics = gc.diagnostics();
    assert_eq!(
        thread
            .try_execute(&class, &even, &[Value::Int(4), Value::Int(8000)])
            .await?,
        Value::Int(15)
    );
    let after = gc.statistics()?;
    assert!(after.objects_swept > before.objects_swept);
    assert!(after.bytes_freed > before.bytes_freed);
    assert_eq!(gc.registered_workers(), workers);
    if workers == 0 {
        assert!(gc.diagnostics().async_assists > diagnostics.async_assists);
        assert_eq!(
            gc.diagnostics().worker_steps.count,
            diagnostics.worker_steps.count
        );
    }
    assert_empty(&thread);
    #[cfg(feature = "execution-profile")]
    for (function, expected) in functions.iter().zip(
        compiled
            .into_iter()
            .zip([3, 2])
            .filter_map(|(compiled, count)| compiled.then_some(count)),
    ) {
        assert_eq!(function.execution_count(), expected);
    }
    #[cfg(not(feature = "execution-profile"))]
    let _ = functions;
    Ok(())
}

#[cfg(target_os = "linux")]
use ristretto_test_util::process;

/// Linux is the host with native stack bounds exercised by this qualification.
#[cfg(target_os = "linux")]
#[test]
fn native_recursion_is_catchable_and_releases_roots_and_monitors() -> Result<()> {
    for concurrent in [false, true] {
        for stack_bytes in [1024 * 1024, 2 * 1024 * 1024] {
            for mode in ["111", "100", "011"] {
                let mut command = std::process::Command::new("sh");
                command
                    .args(["-c", "ulimit -c 0; exec \"$@\"", "native-stack-test"])
                    .arg(std::env::current_exe()?)
                    .args([
                        "--exact",
                        "thread::jit_stack_tests::native_stack_child",
                        "--ignored",
                        "--nocapture",
                    ])
                    .env("RISTRETTO_NATIVE_STACK_BYTES", stack_bytes.to_string())
                    .env("RISTRETTO_NATIVE_STACK_MODE", mode)
                    .env("RISTRETTO_NATIVE_STACK_CONCURRENT", concurrent.to_string());
                let output = process::run(&mut command, Duration::from_secs(90))?;
                output.require_success()?;
                assert!(output.stdout.contains("native stack recovery passed"));
                println!(
                    "stack={stack_bytes} concurrent={concurrent} mode={mode} {}",
                    output.stdout
                );
            }
        }
    }
    Ok(())
}

/// Run in a subprocess: a missing host-stack guard aborts instead of unwinding.
#[test]
#[ignore = "subprocess fixture for native stack exhaustion"]
fn native_stack_child() -> Result<()> {
    let stack_bytes = std::env::var("RISTRETTO_NATIVE_STACK_BYTES")
        .unwrap_or_else(|_| "1048576".to_owned())
        .parse::<usize>()
        .map_err(|error| InternalError(error.to_string()))?;
    let modes = match std::env::var("RISTRETTO_NATIVE_STACK_MODE").as_deref() {
        Ok("100") => [true, false, false],
        Ok("011") => [false, true, true],
        _ => [true; 3],
    };
    let concurrent = std::env::var("RISTRETTO_NATIVE_STACK_CONCURRENT").as_deref() != Ok("false");
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?;
    // Bootstrap on the test thread, so the selected stack size isolates Java
    // recursion rather than the debug build's class-loader bootstrap frames.
    let (vm, thread, class, functions) = runtime.block_on(async {
        let (vm, thread) = vm_with_workers(concurrent, 0).await?;
        let class = recursive_class(&thread, true).await?;
        let functions = install_recursive(&vm, &class, modes)?;
        for (name, descriptor) in [
            ("nativeEven", "(I[I)[I"),
            ("nativeCaught", "(I[I)Ljava/lang/Throwable;"),
        ] {
            let bridge =
                jit::static_call_tests::install_experimental(&vm, &class, name, descriptor)?;
            assert!(bridge.supports_cooperation());
        }
        Ok::<_, crate::Error>((vm, thread, class, functions))
    })?;
    std::thread::Builder::new()
        .name("compiled-stack-probe".to_owned())
        .stack_size(stack_bytes)
        .spawn(move || runtime.block_on(native_recursion(&vm, &thread, &class, &functions)))?
        .join()
        .map_err(|_| InternalError("native stack probe panicked".to_owned()))?
}

async fn native_recursion(
    vm: &Arc<VM>,
    thread: &Arc<Thread>,
    class: &Arc<Class>,
    functions: &[Arc<ristretto_jit::Function>],
) -> Result<()> {
    assert!(
        stacker::remaining_stack().is_some(),
        "native stack bounds unavailable"
    );
    let gc = vm.garbage_collector();
    let even = class.try_get_method("even", "(I[I)[I")?;
    let native_even = class.try_get_method("nativeEven", "(I[I)[I")?;
    let native_caught = class.try_get_method("nativeCaught", "(I[I)Ljava/lang/Throwable;")?;
    let acquired = thread.acquire_sync_monitor(class, &even, &[]).await?;
    let monitor = acquired
        .0
        .as_ref()
        .ok_or_else(|| InternalError("missing recursive monitor".to_owned()))?
        .monitor
        .clone();
    drop(acquired);
    let array = Value::try_new_object_async(gc, Reference::IntArray(vec![99].into())).await?;
    let weak = ristretto_gc::GcWeak::new(reference(&array)?, ristretto_gc::ReferenceKind::Weak, 0);
    let argument_monitor = vm.monitor_registry().monitor(
        get_monitor_id(reference(&array)?)
            .ok_or_else(|| InternalError("missing argument monitor".to_owned()))?,
    );
    let mut errors = Vec::new();
    for catch in [true, false] {
        let result = execute_synchronously(
            vm,
            thread,
            class,
            if catch { &native_caught } else { &native_even },
            &[Value::Int(100_000), array.clone()],
        )
        .await;
        let error = match result {
            Ok(Some(error)) if catch => error,
            Err(crate::Error::Throwable(error)) if !catch => error,
            other => {
                return Err(InternalError(format!(
                    "wrong native recursion result: {other:?}"
                )));
            }
        };
        check_native_overflow(thread, &error, &even)?;
        assert_empty(thread);
        for monitor in [&monitor, &argument_monitor] {
            assert!(!monitor.is_owned_by(thread.id()));
            timeout_at(
                Instant::now() + Duration::from_secs(1),
                monitor.acquire(999),
            )
            .await
            .map_err(|_| InternalError("native overflow leaked monitor".to_owned()))??;
            assert!(monitor.release(999)?);
        }
        errors.push(error);
    }
    let [first, second] = errors.as_slice() else {
        return Err(InternalError("missing native stack exceptions".to_owned()));
    };
    assert_ne!(
        reference(first)?.as_handle_i64(),
        reference(second)?.as_handle_i64()
    );
    #[cfg(feature = "execution-profile")]
    {
        assert!(
            functions
                .iter()
                .all(|function| function.execution_count() > 0)
        );
        println!(
            "native stack entries: {:?}",
            functions
                .iter()
                .map(|f| f.execution_count())
                .collect::<Vec<_>>()
        );
    }
    #[cfg(not(feature = "execution-profile"))]
    let _ = functions;
    drop(array);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(
        weak.upgrade().is_none(),
        "native overflow leaked argument root"
    );
    check_error_owners(gc, errors)?;
    let array = Value::try_new_object_async(gc, Reference::IntArray(vec![88].into())).await?;
    assert_eq!(
        &[88],
        &*thread
            .try_execute(class, &even, &[Value::Int(3), array])
            .await?
            .as_int_vec_ref()?
    );
    assert_empty(thread);
    println!("native stack recovery passed");
    Ok(())
}

fn check_native_overflow(thread: &Thread, error: &Value, method: &Method) -> Result<()> {
    let (message, backtrace) = {
        let object = error.as_object_ref()?;
        assert_eq!(object.class().name(), "java/lang/StackOverflowError");
        (object.value("detailMessage")?, object.value("backtrace")?)
    };
    assert_eq!(message.as_string()?, "Native stack exhausted");
    let depth = backtrace.as_class_vec_ref()?.1.len();
    assert!(depth > 2);
    // Native capacity, not the much larger Java slot budget, ended this recursion.
    assert!(depth * Frame::stack_slots_for(method)? < thread.stack.read().max_slots);
    println!("native stack depth: {depth}");
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn production_synchronized_entry_creates_and_obeys_its_class_monitor() -> Result<()> {
    let (vm, thread) = vm_with_workers(true, 0).await?;
    let mut pool = ConstantPool::new();
    let mut value = method(
        &mut pool,
        "value",
        "()I",
        vec![Instruction::Bipush(7), Instruction::Ireturn],
    )?;
    value.access_flags.insert(MethodAccessFlags::SYNCHRONIZED);
    let class = register(
        &thread,
        class_from_methods(
            "CompiledStackMonitor",
            "java/lang/Object",
            pool,
            vec![value],
        )?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    assert!(class.object()?.is_none());
    let value = class.try_get_method("value", "()I")?;
    assert!(ristretto_jit::Compiler::can_compile(value.definition()));
    let function = vm
        .compiler()
        .ok_or_else(|| InternalError("missing compiler".to_owned()))?
        .compile(&class, &value)
        .await?
        .ok_or_else(|| InternalError("production compilation failed".to_owned()))?;
    assert_eq!(
        thread.execute(&class, &value, &[] as &[Value]).await?,
        Some(Value::Int(7))
    );
    let mirror = class
        .object()?
        .ok_or_else(|| InternalError("missing synchronized mirror".to_owned()))?;
    let id = get_monitor_id(reference(&mirror)?)
        .ok_or_else(|| InternalError("missing monitor ID".to_owned()))?;
    let monitor = vm.monitor_registry().monitor(id);
    assert!(!monitor.is_owned_by(thread.id()));
    monitor.acquire(999).await?;
    // Polling the call must wait for the existing owner, before entering JIT code.
    assert!(
        timeout_at(
            Instant::now() + Duration::from_millis(25),
            thread.execute(&class, &value, &[] as &[Value])
        )
        .await
        .is_err()
    );
    assert_empty(&thread);
    assert!(monitor.release(999)?);
    assert_eq!(
        thread.execute(&class, &value, &[] as &[Value]).await?,
        Some(Value::Int(7))
    );
    assert!(!monitor.is_owned_by(thread.id()));
    assert_empty(&thread);
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 2);
    #[cfg(not(feature = "execution-profile"))]
    let _ = function;
    Ok(())
}

/// Model an embedding that deliberately retains a native caller, while preserving
/// the real Java frame, stack charge and method-monitor ownership.
async fn execute_synchronously(
    vm: &Arc<VM>,
    thread: &Arc<Thread>,
    class: &Arc<Class>,
    method: &Arc<Method>,
    parameters: &[Value],
) -> Result<Option<Value>> {
    let mut boundary = ExecutionBoundary {
        thread,
        depth: thread.stack.read().entries.len(),
        monitor_checkpoint: thread.explicit_monitors.checkpoint(),
        completed: false,
    };
    let function = vm
        .compiler()
        .ok_or_else(|| InternalError("missing compiler".to_owned()))?
        .compile(class, method)
        .await?
        .ok_or_else(|| InternalError("missing native fixture".to_owned()))?;
    let monitor = thread
        .acquire_sync_monitor(class, method, parameters)
        .await?;
    {
        let mut stack = thread.stack.write();
        stack.check_capacity(Frame::stack_slots_for(method)?, class, method)?;
        let frame = stack.take_frame(&Arc::downgrade(thread), class, method);
        stack.push_jit(frame, false, monitor)?;
    }
    let result = jit::execute(
        &function,
        parameters,
        vm.garbage_collector(),
        vm,
        thread,
        class,
    );
    thread.pop_frame()?;
    boundary.completed = true;
    result
}
