//! Exercise experimental calls through the real thread dispatcher. Only tests
//! install the caller in the cache; production reference/call eligibility stays closed.
#![expect(
    clippy::similar_names,
    reason = "caller and callee are distinct JVM roles"
)]
use super::*;
use ristretto_classfile::attributes::{
    ArrayType, Attribute, ExceptionTableEntry, Instruction, MaxLocals, MaxStack,
};
use ristretto_classfile::{ClassAccessFlags, ClassFile, ConstantPool, JavaStr, MethodAccessFlags};
use ristretto_classloader::{Object, Reference};
use ristretto_gc::sync::RwLock;

#[path = "jit_call_cooperation_tests.rs"]
mod cooperation;

#[path = "jit_special_call_tests.rs"]
mod special_call_tests;

#[path = "jit_virtual_call_tests.rs"]
mod virtual_call_tests;

#[path = "jit_interface_call_tests.rs"]
mod interface_call_tests;

fn reference_owner(value: &Value) -> Result<&Gc<RwLock<Reference>>> {
    match value {
        Value::Object(Some(owner)) => Ok(owner),
        _ => Err(InternalError("expected owned reference".to_owned())),
    }
}

pub(crate) fn method(
    pool: &mut ConstantPool,
    name: &str,
    descriptor: &str,
    code: Vec<Instruction>,
) -> Result<ristretto_classfile::Method> {
    method_with_flags(
        pool,
        name,
        descriptor,
        code,
        MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
    )
}

fn method_with_flags(
    pool: &mut ConstantPool,
    name: &str,
    descriptor: &str,
    code: Vec<Instruction>,
    flags: MethodAccessFlags,
) -> Result<ristretto_classfile::Method> {
    let mut method = ristretto_classfile::Method {
        access_flags: flags,
        name_index: pool.add_utf8(name)?,
        descriptor_index: pool.add_utf8(descriptor)?,
        attributes: vec![],
    };
    method.attributes.push(Attribute::Code {
        name_index: pool.add_utf8("Code")?,
        max_stack: code.max_stack(pool)?,
        max_locals: code.max_locals(pool, &method)?,
        code,
        exception_table: vec![],
        attributes: vec![],
    });
    Ok(method)
}

fn call_class(
    name: &str,
    descriptor: &str,
    loads: &[Instruction],
    ret: Instruction,
) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let this_class = pool.add_class(name)?;
    let super_class = pool.add_class("java/lang/Object")?;
    let callee_ref = pool.add_method_ref(this_class, "callee", descriptor)?;
    let mut caller_code = loads.to_vec();
    caller_code.extend([Instruction::Invokestatic(callee_ref), ret.clone()]);
    let caller = method(&mut pool, "caller", descriptor, caller_code)?;
    let mut callee_code = loads.to_vec();
    // The mixed-width reference fixture returns only its final reference.
    if descriptor == "(JFD[I)[I" {
        callee_code = vec![Instruction::Aload(5)];
    }
    callee_code.push(ret);
    let callee = method(&mut pool, "callee", descriptor, callee_code)?;
    Ok(Class::from(
        None,
        ClassFile {
            version: ristretto_classfile::JAVA_5,
            constant_pool: pool,
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
            this_class,
            super_class,
            methods: vec![caller, callee],
            ..Default::default()
        },
    )?)
}

async fn vm(concurrent: bool) -> Result<(Arc<VM>, Arc<Thread>)> {
    vm_with_workers(concurrent, 0).await
}

pub(crate) async fn vm_with_workers(
    concurrent: bool,
    workers: usize,
) -> Result<(Arc<VM>, Arc<Thread>)> {
    let gc = GarbageCollector::with_config(
        ristretto_gc::Configuration::builder()
            .threads(workers)
            .diagnostics(true)
            .concurrent_publication(concurrent)
            .root_storage(ristretto_gc::RootStorage::Contexts)
            .max_heap_bytes(32 * 1024 * 1024)
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

pub(crate) async fn register(thread: &Thread, class: Arc<Class>) -> Result<Arc<Class>> {
    // Opt-in export lets HotSpot verify and execute exactly the same class files.
    if let Some(directory) = std::env::var_os("RISTRETTO_JIT_CALL_FIXTURES") {
        let directory = std::path::PathBuf::from(directory);
        std::fs::create_dir_all(&directory)?;
        let mut bytes = Vec::new();
        class.class_file().to_bytes(&mut bytes)?;
        std::fs::write(directory.join(format!("{}.class", class.name())), bytes)?;
    }
    let name = class.name().to_owned();
    thread.register_class(class).await?;
    thread
        .load_and_link_class(JavaStr::try_from_str(&name)?)
        .await
}

pub(crate) fn class_from_methods(
    name: &str,
    parent: &str,
    pool: ConstantPool<'static>,
    methods: Vec<ristretto_classfile::Method>,
) -> Result<Arc<Class>> {
    class_from_methods_at_version(name, parent, pool, methods, ristretto_classfile::JAVA_5)
}

fn class_from_methods_at_version(
    name: &str,
    parent: &str,
    mut pool: ConstantPool<'static>,
    methods: Vec<ristretto_classfile::Method>,
    version: ristretto_classfile::Version,
) -> Result<Arc<Class>> {
    let this_class = pool.add_class(name)?;
    let super_class = pool.add_class(parent)?;
    Ok(Class::from(
        None,
        ClassFile {
            version,
            constant_pool: pool,
            this_class,
            super_class,
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
            methods,
            ..Default::default()
        },
    )?)
}

fn install_caller(vm: &VM, class: &Arc<Class>, descriptor: &str) -> Result<Arc<Function>> {
    install_experimental(vm, class, "caller", descriptor)
}

pub(crate) fn install_experimental(
    vm: &VM,
    class: &Arc<Class>,
    name: &str,
    descriptor: &str,
) -> Result<Arc<Function>> {
    let caller = class.try_get_method(name, descriptor)?;
    assert!(!ristretto_jit::Compiler::can_compile(caller.definition()));
    let function = Arc::new(ristretto_jit::Compiler::new()?.compile(
        class.class_file(),
        caller.definition(),
        &jit_runtime_helpers::symbols(),
    )?);
    let compiler = vm
        .compiler()
        .ok_or_else(|| InternalError("missing test compiler".to_owned()))?;
    compiler
        .function_cache
        .insert(cache_key(&caller), Some(function.clone()));
    Ok(function)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn static_calls_cross_real_frames_and_transfer_reference_results() -> Result<()> {
    for concurrent in [false, true] {
        let (vm, thread) = vm(concurrent).await?;
        let gc = vm.garbage_collector();
        let class = call_class(
            "StaticReferenceCall",
            "(JFD[I)[I",
            &[
                Instruction::Lload_0,
                Instruction::Fload_2,
                Instruction::Dload_3,
                Instruction::Aload(5),
            ],
            Instruction::Areturn,
        )?;
        let class = register(&thread, class).await?;
        thread.initialize_class(&class).await?;
        let function = install_caller(&vm, &class, "(JFD[I)[I")?;
        let parameters = vec![
            Value::Long(i64::MIN),
            Value::Float(-0.0),
            Value::Double(f64::INFINITY),
            Value::try_new_object_async(gc, Reference::IntArray(vec![77].into())).await?,
        ];
        let value = thread
            .execute(
                &class,
                &class.try_get_method("caller", "(JFD[I)[I")?,
                &parameters,
            )
            .await?
            .ok_or_else(|| InternalError("missing return".to_owned()))?;
        drop(parameters);
        let Value::Object(Some(owner)) = &value else {
            return Err(InternalError("missing returned owner".to_owned()));
        };
        let weak = ristretto_gc::GcWeak::new(owner, ristretto_gc::ReferenceKind::Weak, 0);
        gc.collect_full()?;
        gc.collect_full()?;
        assert_eq!(&[77], &*value.as_int_vec_ref()?);
        drop(value);
        gc.collect_full()?;
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
        assert!(
            thread.current_frame().is_err(),
            "nested call leaked a frame"
        );
        assert_eq!(
            thread
                .execute(
                    &class,
                    &class.try_get_method("caller", "(JFD[I)[I")?,
                    &[
                        Value::Long(0),
                        Value::Float(0.0),
                        Value::Double(0.0),
                        Value::Object(None)
                    ]
                )
                .await?,
            Some(Value::Object(None)),
        );
        #[cfg(feature = "execution-profile")]
        assert_eq!(function.execution_count(), 2);
        #[cfg(not(feature = "execution-profile"))]
        let _ = function;
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn static_calls_preserve_primitive_bits_through_compiled_callees() -> Result<()> {
    let (vm, thread) = vm(true).await?;
    for (index, (descriptor, load, ret, values)) in [
        (
            "(I)I",
            Instruction::Iload_0,
            Instruction::Ireturn,
            vec![Value::Int(i32::MIN), Value::Int(i32::MAX)],
        ),
        (
            "(J)J",
            Instruction::Lload_0,
            Instruction::Lreturn,
            vec![Value::Long(i64::MIN), Value::Long(i64::MAX)],
        ),
        (
            "(F)F",
            Instruction::Fload_0,
            Instruction::Freturn,
            vec![
                Value::Float(-0.0),
                Value::Float(f32::from_bits(0x7fc0_0042)),
            ],
        ),
        (
            "(D)D",
            Instruction::Dload_0,
            Instruction::Dreturn,
            vec![
                Value::Double(-0.0),
                Value::Double(f64::from_bits(0x7ff8_0000_0000_0042)),
            ],
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let class = register(
            &thread,
            call_class(&format!("StaticPrimitive{index}"), descriptor, &[load], ret)?,
        )
        .await?;
        thread.initialize_class(&class).await?;
        let function = install_caller(&vm, &class, descriptor)?;
        let caller = class.try_get_method("caller", descriptor)?;
        let callee = class.try_get_method("callee", descriptor)?;
        assert!(ristretto_jit::Compiler::can_compile(callee.definition()));
        for parameter in values {
            let result = thread
                .execute(&class, &caller, std::slice::from_ref(&parameter))
                .await?;
            match (parameter, result) {
                (Value::Float(expected), Some(Value::Float(actual))) => {
                    assert_eq!(actual.to_bits(), expected.to_bits());
                }
                (Value::Double(expected), Some(Value::Double(actual))) => {
                    assert_eq!(actual.to_bits(), expected.to_bits());
                }
                (expected, actual) => assert_eq!(actual, Some(expected)),
            }
        }
        let compiled_callee = vm
            .compiler()
            .and_then(|compiler| compiler.get_cached(cache_key(&callee)))
            .flatten()
            .ok_or_else(|| InternalError("callee did not compile".to_owned()))?;
        #[cfg(feature = "execution-profile")]
        {
            assert_eq!(function.execution_count(), 2);
            assert_eq!(compiled_callee.execution_count(), 2);
        }
        #[cfg(not(feature = "execution-profile"))]
        let _ = (function, compiled_callee);
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn static_calls_initialize_the_declaring_class_not_the_symbolic_subclass() -> Result<()> {
    let (vm, thread) = vm(true).await?;
    let mut pool = ConstantPool::new();
    let value = method(
        &mut pool,
        "value",
        "()I",
        vec![Instruction::Bipush(42), Instruction::Ireturn],
    )?;
    let parent = register(
        &thread,
        class_from_methods("StaticInitParent", "java/lang/Object", pool, vec![value])?,
    )
    .await?;
    let mut pool = ConstantPool::new();
    let mut initializer = method(
        &mut pool,
        "<clinit>",
        "()V",
        vec![Instruction::Aconst_null, Instruction::Athrow],
    )?;
    initializer.access_flags = MethodAccessFlags::STATIC;
    let child = register(
        &thread,
        class_from_methods("StaticInitChild", parent.name(), pool, vec![initializer])?,
    )
    .await?;
    let mut pool = ConstantPool::new();
    let target = pool.add_class(child.name())?;
    let target = pool.add_method_ref(target, "value", "()I")?;
    let caller = method(
        &mut pool,
        "caller",
        "()I",
        vec![Instruction::Invokestatic(target), Instruction::Ireturn],
    )?;
    let class = register(
        &thread,
        class_from_methods("StaticInitCaller", "java/lang/Object", pool, vec![caller])?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    assert!(!parent.is_initialized()?);
    assert!(!child.is_initialized()?);
    let function = install_caller(&vm, &class, "()I")?;
    let result = thread
        .execute(
            &class,
            &class.try_get_method("caller", "()I")?,
            &[] as &[Value],
        )
        .await?;
    assert_eq!(result, Some(Value::Int(42)));
    assert!(parent.is_initialized()?);
    assert!(!child.is_initialized()?);
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 1);
    #[cfg(not(feature = "execution-profile"))]
    let _ = function;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "explicit caught/uncaught mixed-frame fixture matrix"
)]
async fn static_call_exceptions_keep_owners_across_interpreted_and_compiled_callees() -> Result<()>
{
    for concurrent in [false, true] {
        let (vm, thread) = vm(concurrent).await?;
        let gc = vm.garbage_collector();
        for compiled in [false, true] {
            for caught in [false, true] {
                let name = format!("StaticThrow{compiled}{caught}");
                let descriptor = "(Ljava/lang/Throwable;)Ljava/lang/Throwable;";
                let mut pool = ConstantPool::new();
                let target = pool.add_class(&name)?;
                let target = pool.add_method_ref(target, "callee", descriptor)?;
                let mut caller = method(
                    &mut pool,
                    "caller",
                    descriptor,
                    vec![
                        Instruction::Aload_0,
                        Instruction::Invokestatic(target),
                        Instruction::Areturn,
                    ],
                )?;
                if caught {
                    for attribute in &mut caller.attributes {
                        if let Attribute::Code {
                            code,
                            exception_table,
                            ..
                        } = attribute
                        {
                            code.push(Instruction::Areturn);
                            exception_table.push(ExceptionTableEntry {
                                range_pc: 1..4,
                                handler_pc: 5,
                                catch_type: 0,
                            });
                        }
                    }
                }
                let callee = method(
                    &mut pool,
                    "callee",
                    descriptor,
                    vec![Instruction::Aload_0, Instruction::Athrow],
                )?;
                let class = register(
                    &thread,
                    class_from_methods(&name, "java/lang/Object", pool, vec![caller, callee])?,
                )
                .await?;
                thread.initialize_class(&class).await?;
                let function = install_caller(&vm, &class, descriptor)?;
                let callee = if compiled {
                    Some(install_experimental(&vm, &class, "callee", descriptor)?)
                } else {
                    None
                };
                let object =
                    Object::new_in_async(gc, thread.class("java/lang/RuntimeException").await?)
                        .await?;
                let parameters =
                    [Value::try_new_object_async(gc, Reference::Object(object)).await?];
                let result = thread
                    .execute(
                        &class,
                        &class.try_get_method("caller", descriptor)?,
                        &parameters,
                    )
                    .await;
                let value = match result {
                    Ok(Some(value)) if caught => value,
                    Err(crate::Error::Throwable(value)) if !caught => value,
                    other => {
                        return Err(InternalError(format!(
                            "wrong nested exception result: {other:?}"
                        )));
                    }
                };
                assert_eq!(
                    reference_owner(&value)?.as_handle_i64(),
                    reference_owner(&parameters[0])?.as_handle_i64()
                );
                drop(parameters);
                let owner = reference_owner(&value)?;
                let weak = ristretto_gc::GcWeak::new(owner, ristretto_gc::ReferenceKind::Weak, 0);
                gc.collect_full()?;
                gc.collect_full()?;
                assert_eq!(
                    value.as_object_ref()?.class().name(),
                    "java/lang/RuntimeException"
                );
                drop(value);
                gc.collect_full()?;
                gc.collect_full()?;
                assert!(weak.upgrade().is_none());
                assert!(thread.current_frame().is_err());
                #[cfg(feature = "execution-profile")]
                {
                    assert_eq!(function.execution_count(), 1);
                    if let Some(callee) = callee {
                        assert_eq!(callee.execution_count(), 1);
                    }
                }
                #[cfg(not(feature = "execution-profile"))]
                let _ = (function, callee);
            }
        }
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn static_call_preserves_a_suspended_callers_operand_during_reclamation() -> Result<()> {
    for concurrent in [false, true] {
        for workers in [0, 1] {
            for compiled in [false, true] {
                pressure_call(concurrent, workers, compiled).await?;
            }
        }
    }
    Ok(())
}

async fn pressure_call(concurrent: bool, workers: usize, compiled: bool) -> Result<()> {
    let (vm, thread) = vm_with_workers(concurrent, workers).await?;
    let gc = vm.garbage_collector();
    let mut pool = ConstantPool::new();
    let target = pool.add_class("StaticPressureCall")?;
    let target = pool.add_method_ref(target, "callee", "(I)V")?;
    let caller = method(
        &mut pool,
        "caller",
        "(I)[I",
        vec![
            Instruction::Iconst_1,
            Instruction::Newarray(ArrayType::Int),
            Instruction::Dup,
            Instruction::Iconst_0,
            Instruction::Bipush(77),
            Instruction::Iastore,
            Instruction::Iload_0,
            Instruction::Invokestatic(target),
            Instruction::Areturn,
        ],
    )?;
    let callee = method(
        &mut pool,
        "callee",
        "(I)V",
        vec![
            Instruction::Iload_0,
            Instruction::Ifeq(15),
            Instruction::Sipush(2048),
            Instruction::Newarray(ArrayType::Int),
            Instruction::Pop,
            Instruction::Iinc(0, -1),
            Instruction::Goto(-13),
            Instruction::Return,
        ],
    )?;
    let class = register(
        &thread,
        class_from_methods(
            "StaticPressureCall",
            "java/lang/Object",
            pool,
            vec![caller, callee],
        )?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    let function = install_caller(&vm, &class, "(I)[I")?;
    let callee = if compiled {
        Some(install_experimental(&vm, &class, "callee", "(I)V")?)
    } else {
        None
    };
    let before = gc.statistics()?;
    let diagnostics = gc.diagnostics();
    let value = thread
        .execute(
            &class,
            &class.try_get_method("caller", "(I)[I")?,
            &[Value::Int(8000)],
        )
        .await?
        .ok_or_else(|| InternalError("missing suspended caller result".to_owned()))?;
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
    let owner = reference_owner(&value)?;
    let weak = ristretto_gc::GcWeak::new(owner, ristretto_gc::ReferenceKind::Weak, 0);
    gc.collect_full()?;
    gc.collect_full()?;
    assert_eq!(&[77], &*value.as_int_vec_ref()?);
    drop(value);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(weak.upgrade().is_none());
    assert!(thread.current_frame().is_err());
    #[cfg(feature = "execution-profile")]
    {
        assert_eq!(function.execution_count(), 1);
        if let Some(callee) = callee {
            assert_eq!(callee.execution_count(), 1);
        }
    }
    #[cfg(not(feature = "execution-profile"))]
    let _ = (function, callee);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn static_calls_dispatch_native_intrinsics_with_owned_arguments() -> Result<()> {
    let (vm, thread) = vm(true).await?;
    let gc = vm.garbage_collector();
    let mut pool = ConstantPool::new();
    let target = pool.add_class("java/lang/System")?;
    let target = pool.add_method_ref(target, "identityHashCode", "(Ljava/lang/Object;)I")?;
    let caller = method(
        &mut pool,
        "caller",
        "([I)I",
        vec![
            Instruction::Aload_0,
            Instruction::Invokestatic(target),
            Instruction::Ireturn,
        ],
    )?;
    let class = register(
        &thread,
        class_from_methods("StaticNativeCaller", "java/lang/Object", pool, vec![caller])?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    let function = install_caller(&vm, &class, "([I)I")?;
    let parameters = [Value::try_new_object_async(gc, Reference::IntArray(vec![77].into())).await?];
    let weak = ristretto_gc::GcWeak::new(
        reference_owner(&parameters[0])?,
        ristretto_gc::ReferenceKind::Weak,
        0,
    );
    let system = thread.class("java/lang/System").await?;
    let native = system.try_get_method("identityHashCode", "(Ljava/lang/Object;)I")?;
    assert!(native.is_native());
    let expected = thread.execute(&system, &native, &parameters).await?;
    let actual = thread
        .execute(
            &class,
            &class.try_get_method("caller", "([I)I")?,
            &parameters,
        )
        .await?;
    assert_eq!(actual, expected);
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 1);
    #[cfg(not(feature = "execution-profile"))]
    let _ = function;
    drop(parameters);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(weak.upgrade().is_none());
    #[cfg(feature = "execution-profile")]
    assert!(
        vm.execution_profile()
            .methods
            .iter()
            .any(|record| record.class == "java/lang/System"
                && record.name == "identityHashCode"
                && record.intrinsic_entries >= 2)
    );
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn static_calls_reuse_the_full_argument_buffer_in_order() -> Result<()> {
    let (vm, thread) = vm(true).await?;
    let count = ristretto_jit::CALL_ARGUMENT_CAPACITY;
    let descriptor = format!("({})I", "I".repeat(count));
    let mut pool = ConstantPool::new();
    let target = pool.add_class("StaticMaximumArguments")?;
    let target = pool.add_method_ref(target, "callee", &descriptor)?;
    let mut loads = Vec::new();
    let mut sum = Vec::new();
    let mut parameters = Vec::new();
    let mut expected = 0;
    for index in 0..count {
        loads.push(Instruction::Iload(u8::try_from(index)?));
        if index > 0 {
            sum.extend([Instruction::Bipush(31), Instruction::Imul]);
        }
        sum.push(Instruction::Iload(u8::try_from(index)?));
        if index > 0 {
            sum.push(Instruction::Iadd);
        }
        let value = i32::try_from(index)? * 7 - 11;
        parameters.push(Value::Int(value));
        expected = i32::wrapping_add(i32::wrapping_mul(expected, 31), value);
    }
    let mut code = loads.clone();
    code.push(Instruction::Invokestatic(target));
    code.extend(loads);
    code.extend([
        Instruction::Invokestatic(target),
        Instruction::Iadd,
        Instruction::Ireturn,
    ]);
    sum.push(Instruction::Ireturn);
    let caller = method(&mut pool, "caller", &descriptor, code)?;
    let callee = method(&mut pool, "callee", &descriptor, sum)?;
    let class = register(
        &thread,
        class_from_methods(
            "StaticMaximumArguments",
            "java/lang/Object",
            pool,
            vec![caller, callee],
        )?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    let function = install_caller(&vm, &class, &descriptor)?;
    assert_eq!(
        thread
            .execute(
                &class,
                &class.try_get_method("caller", &descriptor)?,
                &parameters
            )
            .await?,
        Some(Value::Int(expected.wrapping_mul(2))),
    );
    assert!(thread.current_frame().is_err());
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 1);
    #[cfg(not(feature = "execution-profile"))]
    let _ = function;
    Ok(())
}

#[test]
fn static_call_lowering_rejects_more_than_255_java_parameter_slots() -> Result<()> {
    let mut pool = ConstantPool::new();
    let target = pool.add_class("InvalidWideCallTarget")?;
    let descriptor = format!("({})V", "J".repeat(128));
    let target = pool.add_method_ref(target, "callee", &descriptor)?;
    let mut code = vec![Instruction::Lconst_0; 128];
    code.extend([Instruction::Invokestatic(target), Instruction::Return]);
    // Bypass the fixture's max-stack analyzer, which already rejects this
    // malformed descriptor, to check the compiler's own validation path.
    let caller = ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: pool.add_utf8("caller")?,
        descriptor_index: pool.add_utf8("()V")?,
        attributes: vec![Attribute::Code {
            name_index: pool.add_utf8("Code")?,
            max_stack: 256,
            max_locals: 0,
            code,
            exception_table: vec![],
            attributes: vec![],
        }],
    };
    let class = class_from_methods("InvalidWideCall", "java/lang/Object", pool, vec![caller])?;
    let error = ristretto_jit::Compiler::new()?.compile(
        class.class_file(),
        class.try_get_method("caller", "()V")?.definition(),
        &jit_runtime_helpers::symbols(),
    );
    assert!(matches!(
        error,
        Err(ristretto_jit::Error::ClassFileError(
            ristretto_classfile::Error::InvalidMethodDescriptor(_)
        ))
    ));
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "paired valid-interface and invalid-instance initialization fixtures"
)]
async fn static_interface_calls_and_instance_target_errors_obey_initialization_order() -> Result<()>
{
    let (vm, thread) = vm(true).await?;
    for interface in [false, true] {
        let name = if interface {
            "StaticInterfaceTarget"
        } else {
            "InstanceTarget"
        };
        let mut pool = ConstantPool::new();
        let mut value = method(
            &mut pool,
            "value",
            "()I",
            vec![Instruction::Bipush(42), Instruction::Ireturn],
        )?;
        let mut methods = vec![];
        if !interface {
            value.access_flags = MethodAccessFlags::PUBLIC;
            // The instance receiver occupies slot zero even when unused.
            for attribute in &mut value.attributes {
                if let Attribute::Code { max_locals, .. } = attribute {
                    *max_locals = 1;
                }
            }
            let mut initializer = method(
                &mut pool,
                "<clinit>",
                "()V",
                vec![Instruction::Aconst_null, Instruction::Athrow],
            )?;
            initializer.access_flags = MethodAccessFlags::STATIC;
            methods.push(initializer);
        }
        methods.push(value);
        let this_class = pool.add_class(name)?;
        let super_class = pool.add_class("java/lang/Object")?;
        let target = register(
            &thread,
            Class::from(
                None,
                ClassFile {
                    version: ristretto_classfile::JAVA_8,
                    constant_pool: pool,
                    this_class,
                    super_class,
                    methods,
                    access_flags: if interface {
                        ClassAccessFlags::PUBLIC
                            | ClassAccessFlags::INTERFACE
                            | ClassAccessFlags::ABSTRACT
                    } else {
                        ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER
                    },
                    ..Default::default()
                },
            )?,
        )
        .await?;
        let mut pool = ConstantPool::new();
        let index = pool.add_class(name)?;
        let index = if interface {
            pool.add_interface_method_ref(index, "value", "()I")?
        } else {
            pool.add_method_ref(index, "value", "()I")?
        };
        let caller = method(
            &mut pool,
            "caller",
            "()I",
            vec![Instruction::Invokestatic(index), Instruction::Ireturn],
        )?;
        let class = register(
            &thread,
            class_from_methods_at_version(
                &format!("StaticKindCaller{interface}"),
                "java/lang/Object",
                pool,
                vec![caller],
                ristretto_classfile::JAVA_8,
            )?,
        )
        .await?;
        thread.initialize_class(&class).await?;
        let function = install_caller(&vm, &class, "()I")?;
        let result = thread
            .execute(
                &class,
                &class.try_get_method("caller", "()I")?,
                &[] as &[Value],
            )
            .await;
        #[cfg(feature = "execution-profile")]
        assert_eq!(function.execution_count(), 1);
        #[cfg(not(feature = "execution-profile"))]
        let _ = function;
        if interface {
            assert_eq!(result?, Some(Value::Int(42)));
            assert!(target.is_initialized()?);
        } else {
            let Err(crate::Error::Throwable(value)) = result else {
                return Err(InternalError("instance target did not throw".to_owned()));
            };
            assert_eq!(
                value.as_object_ref()?.class().name(),
                "java/lang/IncompatibleClassChangeError"
            );
            assert!(!target.is_initialized()?);
        }
    }
    Ok(())
}
