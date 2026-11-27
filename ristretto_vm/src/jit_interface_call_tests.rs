//! Interface calls through real verified classes, mixed frames and owning values.
use super::*;

fn declaration(
    pool: &mut ConstantPool,
    name: &str,
    descriptor: &str,
) -> Result<ristretto_classfile::Method> {
    Ok(ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
        name_index: pool.add_utf8(name)?,
        descriptor_index: pool.add_utf8(descriptor)?,
        attributes: vec![],
    })
}

async fn define(
    thread: &Thread,
    name: &str,
    parent: &str,
    interfaces: &[&str],
    mut pool: ConstantPool<'static>,
    mut methods: Vec<ristretto_classfile::Method>,
    interface: bool,
) -> Result<Arc<Class>> {
    let this_class = pool.add_class(name)?;
    let super_class = pool.add_class(parent)?;
    let interfaces = interfaces
        .iter()
        .map(|name| pool.add_class(name))
        .collect::<std::result::Result<Vec<_>, _>>()?;
    if !interface {
        let init = pool.add_method_ref(super_class, "<init>", "()V")?;
        methods.push(method_with_flags(
            &mut pool,
            "<init>",
            "()V",
            vec![
                Instruction::Aload_0,
                Instruction::Invokespecial(init),
                Instruction::Return,
            ],
            MethodAccessFlags::PUBLIC,
        )?);
    }
    register(
        thread,
        Class::from(
            None,
            ClassFile {
                version: if interface {
                    ristretto_classfile::JAVA_8
                } else {
                    ristretto_classfile::JAVA_5
                },
                constant_pool: pool,
                this_class,
                super_class,
                interfaces,
                methods,
                access_flags: ClassAccessFlags::PUBLIC
                    | if interface {
                        ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT
                    } else {
                        ClassAccessFlags::SUPER
                    },
                ..Default::default()
            },
        )?,
    )
    .await
}

async fn receiver(thread: &Thread, class: &Arc<Class>) -> Result<Value> {
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();
    let object = Object::new_in_async(gc, class.clone()).await?;
    Ok(Value::try_new_object_async(gc, Reference::Object(object)).await?)
}

async fn dispatch_classes(thread: &Thread) -> Result<(Arc<Class>, Vec<Arc<Class>>)> {
    let mut pool = ConstantPool::new();
    let value = declaration(&mut pool, "value", "()I")?;
    define(
        thread,
        "InterfaceValue",
        "java/lang/Object",
        &[],
        pool,
        vec![value],
        true,
    )
    .await?;
    let mut pool = ConstantPool::new();
    let target = pool.add_class("InterfaceValue")?;
    let target = pool.add_interface_method_ref(target, "value", "()I")?;
    let caller = method(
        &mut pool,
        "caller",
        "(LInterfaceValue;)I",
        vec![
            Instruction::Aload_0,
            Instruction::Invokeinterface(target, 1),
            Instruction::Ireturn,
        ],
    )?;
    let caller = define(
        thread,
        "InterfaceDispatch",
        "java/lang/Object",
        &[],
        pool,
        vec![caller],
        false,
    )
    .await?;
    thread.initialize_class(&caller).await?;
    let mut classes = Vec::new();
    for index in 0..7 {
        let mut pool = ConstantPool::new();
        let mut methods = Vec::new();
        if index != 6 {
            methods.push(method_with_flags(
                &mut pool,
                "value",
                "()I",
                vec![Instruction::Bipush(index), Instruction::Ireturn],
                MethodAccessFlags::PUBLIC,
            )?);
        }
        let parent = if index == 6 {
            "InterfaceTarget5"
        } else {
            "java/lang/Object"
        };
        let class = define(
            thread,
            &format!("InterfaceTarget{index}"),
            parent,
            &["InterfaceValue"],
            pool,
            methods,
            false,
        )
        .await?;
        thread.initialize_class(&class).await?;
        classes.push(class);
    }
    Ok((caller, classes))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn interface_dispatch_handles_inheritance_and_receiver_cache_overflow() -> Result<()> {
    for compiled in [false, true] {
        let (vm, thread) = vm(true).await?;
        let (caller, classes) = dispatch_classes(&thread).await?;
        let descriptor = "(LInterfaceValue;)I";
        let function = install_caller(&vm, &caller, descriptor)?;
        let mut functions = Vec::new();
        if compiled {
            for class in classes.iter().take(6) {
                functions.push(install_experimental(&vm, class, "value", "()I")?);
            }
        }
        for _ in 0..2 {
            for (index, class) in classes.iter().enumerate() {
                assert_eq!(
                    thread
                        .execute(
                            &caller,
                            &caller.try_get_method("caller", descriptor)?,
                            &[receiver(&thread, class).await?]
                        )
                        .await?,
                    Some(Value::Int(i32::try_from(index.min(5))?))
                );
            }
        }
        assert!(thread.current_frame().is_err());
        #[cfg(feature = "execution-profile")]
        {
            assert_eq!(function.execution_count(), 14);
            for (index, callee) in functions.iter().enumerate() {
                assert_eq!(callee.execution_count(), if index == 5 { 4 } else { 2 });
            }
        }
        #[cfg(not(feature = "execution-profile"))]
        let _ = function;
    }
    Ok(())
}

async fn default_interfaces(thread: &Thread) -> Result<()> {
    for (name, parents, number) in [
        ("InterfaceDefault", vec![], 40),
        ("InterfaceRefinedDefault", vec!["InterfaceDefault"], 41),
    ] {
        let mut pool = ConstantPool::new();
        let value = method_with_flags(
            &mut pool,
            "value",
            "()I",
            vec![Instruction::Bipush(number), Instruction::Ireturn],
            MethodAccessFlags::PUBLIC,
        )?;
        define(
            thread,
            name,
            "java/lang/Object",
            &parents,
            pool,
            vec![value],
            true,
        )
        .await?;
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn interface_defaults_select_the_most_specific_implementation() -> Result<()> {
    let (vm, thread) = vm(true).await?;
    default_interfaces(&thread).await?;
    let mut pool = ConstantPool::new();
    let target = pool.add_class("InterfaceDefault")?;
    let target = pool.add_interface_method_ref(target, "value", "()I")?;
    let descriptor = "(LInterfaceDefault;)I";
    let caller = method(
        &mut pool,
        "caller",
        descriptor,
        vec![
            Instruction::Aload_0,
            Instruction::Invokeinterface(target, 1),
            Instruction::Ireturn,
        ],
    )?;
    let caller = define(
        &thread,
        "InterfaceDefaultCaller",
        "java/lang/Object",
        &[],
        pool,
        vec![caller],
        false,
    )
    .await?;
    thread.initialize_class(&caller).await?;
    let function = install_caller(&vm, &caller, descriptor)?;
    for (name, parent, interfaces, value, expected) in [
        (
            "InterfaceDefaultBase",
            "java/lang/Object",
            vec!["InterfaceDefault"],
            None,
            40,
        ),
        (
            "InterfaceDefaultChild",
            "InterfaceDefaultBase",
            vec!["InterfaceRefinedDefault"],
            None,
            41,
        ),
        (
            "InterfaceDefaultOverride",
            "InterfaceDefaultChild",
            vec![],
            Some(42),
            42,
        ),
    ] {
        let mut pool = ConstantPool::new();
        let mut methods = Vec::new();
        if let Some(value) = value {
            methods.push(method_with_flags(
                &mut pool,
                "value",
                "()I",
                vec![Instruction::Bipush(value), Instruction::Ireturn],
                MethodAccessFlags::PUBLIC,
            )?);
        }
        let class = define(&thread, name, parent, &interfaces, pool, methods, false).await?;
        thread.initialize_class(&class).await?;
        assert_eq!(
            thread
                .execute(
                    &caller,
                    &caller.try_get_method("caller", descriptor)?,
                    &[receiver(&thread, &class).await?]
                )
                .await?,
            Some(Value::Int(expected))
        );
    }
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 3);
    #[cfg(not(feature = "execution-profile"))]
    let _ = function;
    Ok(())
}

async fn error_receiver(
    thread: &Thread,
    name: &str,
    interface_name: &str,
    interface: bool,
    method_kind: &str,
    implementation: &str,
) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let methods = match method_kind {
        "missing" => vec![],
        "abstract" => vec![declaration(&mut pool, "value", "()I")?],
        _ => vec![method_with_flags(
            &mut pool,
            "value",
            "()I",
            vec![Instruction::Iconst_1, Instruction::Ireturn],
            if method_kind == "static" {
                MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC
            } else {
                MethodAccessFlags::PUBLIC
            },
        )?],
    };
    define(
        thread,
        interface_name,
        "java/lang/Object",
        &[],
        pool,
        methods,
        interface,
    )
    .await?;
    let mut pool = ConstantPool::new();
    let methods = if implementation == "missing" {
        vec![]
    } else {
        vec![method_with_flags(
            &mut pool,
            "value",
            "()I",
            vec![Instruction::Iconst_2, Instruction::Ireturn],
            if implementation == "protected" {
                MethodAccessFlags::PROTECTED
            } else {
                MethodAccessFlags::PUBLIC
            },
        )?]
    };
    let interfaces = if interface && implementation != "nonmember" {
        vec![interface_name]
    } else {
        vec![]
    };
    let implementation = define(
        thread,
        &format!("InterfaceError{name}Impl"),
        if interface {
            "java/lang/Object"
        } else {
            interface_name
        },
        &interfaces,
        pool,
        methods,
        false,
    )
    .await?;
    thread.initialize_class(&implementation).await?;
    Ok(implementation)
}

const ERROR_CASES: [(&str, bool, &str, &str, &str); 6] = [
    (
        "Static",
        true,
        "static",
        "public",
        "IncompatibleClassChangeError",
    ),
    ("Missing", true, "missing", "public", "NoSuchMethodError"),
    (
        "Class",
        false,
        "public",
        "public",
        "IncompatibleClassChangeError",
    ),
    (
        "Abstract",
        true,
        "abstract",
        "missing",
        "AbstractMethodError",
    ),
    (
        "NonPublic",
        true,
        "abstract",
        "protected",
        "IllegalAccessError",
    ),
    (
        "NonMember",
        true,
        "abstract",
        "nonmember",
        "IncompatibleClassChangeError",
    ),
];

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn interface_errors_preserve_resolution_null_membership_and_selection_order() -> Result<()> {
    let (vm, thread) = vm(true).await?;
    for (name, interface, method_kind, implementation, expected) in ERROR_CASES {
        let interface_name = format!("InterfaceError{name}");
        let implementation = error_receiver(
            &thread,
            name,
            &interface_name,
            interface,
            method_kind,
            implementation,
        )
        .await?;
        let mut pool = ConstantPool::new();
        let target = pool.add_class(&interface_name)?;
        let target = pool.add_interface_method_ref(target, "value", "()I")?;
        let descriptor = if interface {
            "(Ljava/lang/Object;)I".to_owned()
        } else {
            format!("(L{interface_name};)I")
        };
        let descriptor = descriptor.as_str();
        let caller = method(
            &mut pool,
            "caller",
            descriptor,
            vec![
                Instruction::Aload_0,
                Instruction::Invokeinterface(target, 1),
                Instruction::Ireturn,
            ],
        )?;
        let caller = define(
            &thread,
            &format!("InterfaceError{name}Caller"),
            "java/lang/Object",
            &[],
            pool,
            vec![caller],
            false,
        )
        .await?;
        thread.initialize_class(&caller).await?;
        let function = install_caller(&vm, &caller, descriptor)?;
        for argument in [
            Value::Object(None),
            receiver(&thread, &implementation).await?,
        ] {
            let exception = if matches!(name, "Abstract" | "NonPublic" | "NonMember")
                && matches!(argument, Value::Object(None))
            {
                "NullPointerException"
            } else {
                expected
            };
            let result = thread
                .execute(
                    &caller,
                    &caller.try_get_method("caller", descriptor)?,
                    &[argument],
                )
                .await;
            let Err(crate::Error::Throwable(error)) = result else {
                return Err(InternalError(format!(
                    "missing {exception} for {name}: {result:?}"
                )));
            };
            assert_eq!(
                error.as_object_ref()?.class().name(),
                format!("java/lang/{exception}")
            );
            assert!(thread.current_frame().is_err());
        }
        #[cfg(feature = "execution-profile")]
        assert_eq!(function.execution_count(), 2);
        #[cfg(not(feature = "execution-profile"))]
        let _ = function;
    }
    Ok(())
}

async fn pressure_classes(thread: &Thread) -> Result<(Arc<Class>, Arc<Class>)> {
    let mut pool = ConstantPool::new();
    let allocate = declaration(&mut pool, "allocate", "(I)[I")?;
    define(
        thread,
        "InterfacePressure",
        "java/lang/Object",
        &[],
        pool,
        vec![allocate],
        true,
    )
    .await?;
    let mut pool = ConstantPool::new();
    let target = pool.add_class("InterfacePressure")?;
    let target = pool.add_interface_method_ref(target, "allocate", "(I)[I")?;
    let caller = method(
        &mut pool,
        "caller",
        "(LInterfacePressure;I)I",
        vec![
            Instruction::Iconst_1,
            Instruction::Newarray(ArrayType::Int),
            Instruction::Dup,
            Instruction::Iconst_0,
            Instruction::Bipush(77),
            Instruction::Iastore,
            Instruction::Aload_0,
            Instruction::Iload_1,
            Instruction::Invokeinterface(target, 2),
            Instruction::Iconst_0,
            Instruction::Iaload,
            Instruction::Swap,
            Instruction::Iconst_0,
            Instruction::Iaload,
            Instruction::Iadd,
            Instruction::Ireturn,
        ],
    )?;
    let caller = define(
        thread,
        "InterfacePressureCaller",
        "java/lang/Object",
        &[],
        pool,
        vec![caller],
        false,
    )
    .await?;
    thread.initialize_class(&caller).await?;
    let mut pool = ConstantPool::new();
    let allocate = method_with_flags(
        &mut pool,
        "allocate",
        "(I)[I",
        vec![
            Instruction::Iconst_1,
            Instruction::Newarray(ArrayType::Int),
            Instruction::Dup,
            Instruction::Iconst_0,
            Instruction::Bipush(88),
            Instruction::Iastore,
            Instruction::Astore_2,
            Instruction::Iload_1,
            Instruction::Ifeq(15),
            Instruction::Sipush(2048),
            Instruction::Newarray(ArrayType::Int),
            Instruction::Pop,
            Instruction::Iinc(1, -1),
            Instruction::Goto(-13),
            Instruction::Aload_2,
            Instruction::Areturn,
        ],
        MethodAccessFlags::PUBLIC,
    )?;
    let implementation = define(
        thread,
        "InterfacePressureImpl",
        "java/lang/Object",
        &["InterfacePressure"],
        pool,
        vec![allocate],
        false,
    )
    .await?;
    thread.initialize_class(&implementation).await?;
    Ok((caller, implementation))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn interface_allocation_retains_suspended_operands_and_returned_references() -> Result<()> {
    for concurrent in [false, true] {
        for workers in [0, 1] {
            for compiled in [false, true] {
                let (vm, thread) = vm_with_workers(concurrent, workers).await?;
                let gc = vm.garbage_collector();
                let (caller, implementation) = pressure_classes(&thread).await?;
                let descriptor = "(LInterfacePressure;I)I";
                let function = install_caller(&vm, &caller, descriptor)?;
                let callee = if compiled {
                    Some(install_experimental(
                        &vm,
                        &implementation,
                        "allocate",
                        "(I)[I",
                    )?)
                } else {
                    None
                };
                let parameters = [receiver(&thread, &implementation).await?, Value::Int(8000)];
                let before = gc.statistics()?;
                let diagnostics = gc.diagnostics();
                assert_eq!(
                    thread
                        .execute(
                            &caller,
                            &caller.try_get_method("caller", descriptor)?,
                            &parameters
                        )
                        .await?,
                    Some(Value::Int(165))
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

async fn value_classes(thread: &Thread) -> Result<(Arc<Class>, Arc<Class>)> {
    let mut interface_pool = ConstantPool::new();
    let mut interface_methods = Vec::new();
    let mut target_pool = ConstantPool::new();
    let mut target_methods = Vec::new();
    let mut caller_pool = ConstantPool::new();
    let target = caller_pool.add_class("InterfaceValues")?;
    let mut callers = Vec::new();
    for (index, (kind, load, ret, slots)) in [
        ("J", Instruction::Lload_1, Instruction::Lreturn, 3),
        ("F", Instruction::Fload_1, Instruction::Freturn, 2),
        ("D", Instruction::Dload_1, Instruction::Dreturn, 3),
        ("[I", Instruction::Aload_1, Instruction::Areturn, 2),
    ]
    .into_iter()
    .enumerate()
    {
        let descriptor = format!("({kind}){kind}");
        interface_methods.push(declaration(&mut interface_pool, "value", &descriptor)?);
        target_methods.push(method_with_flags(
            &mut target_pool,
            "value",
            &descriptor,
            vec![load.clone(), ret.clone()],
            MethodAccessFlags::PUBLIC,
        )?);
        let reference = caller_pool.add_interface_method_ref(target, "value", &descriptor)?;
        callers.push(method(
            &mut caller_pool,
            &format!("caller{index}"),
            &format!("(LInterfaceValues;{kind}){kind}"),
            vec![
                Instruction::Aload_0,
                load,
                Instruction::Invokeinterface(reference, slots),
                ret,
            ],
        )?);
    }
    define(
        thread,
        "InterfaceValues",
        "java/lang/Object",
        &[],
        interface_pool,
        interface_methods,
        true,
    )
    .await?;
    let target = define(
        thread,
        "InterfaceValuesImpl",
        "java/lang/Object",
        &["InterfaceValues"],
        target_pool,
        target_methods,
        false,
    )
    .await?;
    let caller = define(
        thread,
        "InterfaceValuesCaller",
        "java/lang/Object",
        &[],
        caller_pool,
        callers,
        false,
    )
    .await?;
    thread.initialize_class(&target).await?;
    thread.initialize_class(&caller).await?;
    Ok((caller, target))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn interface_native_calls_handle_primitive_array_receivers() -> Result<()> {
    let (vm, thread) = vm(true).await?;
    let gc = vm.garbage_collector();
    let mut pool = ConstantPool::new();
    let target = pool.add_class("java/lang/Cloneable")?;
    let target = pool.add_interface_method_ref(target, "getClass", "()Ljava/lang/Class;")?;
    let descriptor = "(Ljava/lang/Cloneable;)Ljava/lang/Class;";
    let caller = method(
        &mut pool,
        "caller",
        descriptor,
        vec![
            Instruction::Aload_0,
            Instruction::Invokeinterface(target, 1),
            Instruction::Areturn,
        ],
    )?;
    let caller = define(
        &thread,
        "InterfaceNative",
        "java/lang/Object",
        &[],
        pool,
        vec![caller],
        false,
    )
    .await?;
    thread.initialize_class(&caller).await?;
    let function = install_caller(&vm, &caller, descriptor)?;
    let object = thread.class("java/lang/Object").await?;
    let native = object.try_get_method("getClass", "()Ljava/lang/Class;")?;
    assert!(native.is_native());
    for array in [
        Value::try_new_object_async(gc, Reference::IntArray(vec![77].into())).await?,
        Value::try_new_object_async(gc, Reference::ByteArray(vec![11].into())).await?,
    ] {
        let expected = thread
            .execute(&object, &native, std::slice::from_ref(&array))
            .await?
            .ok_or_else(|| InternalError("missing expected class".to_owned()))?;
        let actual = thread
            .execute(
                &caller,
                &caller.try_get_method("caller", descriptor)?,
                &[array],
            )
            .await?
            .ok_or_else(|| InternalError("missing class result".to_owned()))?;
        gc.collect_full()?;
        assert_eq!(
            reference_owner(&expected)?.as_handle_i64(),
            reference_owner(&actual)?.as_handle_i64()
        );
    }
    assert!(thread.current_frame().is_err());
    #[cfg(feature = "execution-profile")]
    {
        assert_eq!(function.execution_count(), 2);
        assert!(
            vm.execution_profile()
                .methods
                .iter()
                .any(|entry| entry.class == "java/lang/Object"
                    && entry.name == "getClass"
                    && entry.intrinsic_entries >= 4)
        );
    }
    #[cfg(not(feature = "execution-profile"))]
    let _ = function;
    Ok(())
}

#[test]
fn interface_count_rejects_logical_value_counts_for_wide_arguments() -> Result<()> {
    for count in [0, 2, 255] {
        let mut pool = ConstantPool::new();
        let target = pool.add_class("InterfaceValues")?;
        let target = pool.add_interface_method_ref(target, "value", "(J)J")?;
        // Build valid frame bounds before deliberately corrupting the redundant
        // instruction count. The public compiler must reject unverified input too.
        let mut caller = method(
            &mut pool,
            "caller",
            "(LInterfaceValues;J)J",
            vec![
                Instruction::Aload_0,
                Instruction::Lload_1,
                Instruction::Invokeinterface(target, 3),
                Instruction::Lreturn,
            ],
        )?;
        for attribute in &mut caller.attributes {
            if let Attribute::Code { code, .. } = attribute {
                for instruction in code {
                    if let Instruction::Invokeinterface(_, slots) = instruction {
                        *slots = count;
                    }
                }
            }
        }
        let this_class = pool.add_class("InterfaceInvalidCount")?;
        let super_class = pool.add_class("java/lang/Object")?;
        let class = ClassFile {
            version: ristretto_classfile::JAVA_5,
            constant_pool: pool,
            this_class,
            super_class,
            ..Default::default()
        };
        let result = ristretto_jit::Compiler::new()?.compile(
            &class,
            &caller,
            &jit_runtime_helpers::symbols(),
        );
        assert!(
            matches!(result, Err(ristretto_jit::Error::UnsupportedMethod(message)) if message.contains("parameter-slot count"))
        );
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn interface_calls_preserve_primitive_bits_and_transfer_reference_results() -> Result<()> {
    for compiled in [false, true] {
        let (vm, thread) = vm(true).await?;
        let gc = vm.garbage_collector();
        let (base, child) = value_classes(&thread).await?;
        let receiver = receiver(&thread, &child).await?;
        for (index, (kind, parameters)) in [
            ("J", vec![Value::Long(i64::MIN), Value::Long(i64::MAX)]),
            (
                "F",
                vec![
                    Value::Float(-0.0),
                    Value::Float(f32::from_bits(0x7fc0_0042)),
                ],
            ),
            (
                "D",
                vec![
                    Value::Double(-0.0),
                    Value::Double(f64::from_bits(0x7ff8_0000_0000_0042)),
                ],
            ),
            (
                "[I",
                vec![
                    Value::Object(None),
                    Value::try_new_object_async(gc, Reference::IntArray(vec![77].into())).await?,
                ],
            ),
        ]
        .into_iter()
        .enumerate()
        {
            let name = format!("caller{index}");
            let descriptor = format!("(LInterfaceValues;{kind}){kind}");
            let function = install_experimental(&vm, &base, &name, &descriptor)?;
            let callee = if compiled {
                Some(install_experimental(
                    &vm,
                    &child,
                    "value",
                    &format!("({kind}){kind}"),
                )?)
            } else {
                None
            };
            for parameter in parameters {
                let result = thread
                    .execute(
                        &base,
                        &base.try_get_method(&name, &descriptor)?,
                        &[receiver.clone(), parameter.clone()],
                    )
                    .await?
                    .ok_or_else(|| InternalError("missing interface result".to_owned()))?;
                match (&parameter, &result) {
                    (Value::Float(expected), Value::Float(actual)) => {
                        assert_eq!(actual.to_bits(), expected.to_bits());
                    }
                    (Value::Double(expected), Value::Double(actual)) => {
                        assert_eq!(actual.to_bits(), expected.to_bits());
                    }
                    (Value::Object(Some(expected)), Value::Object(Some(actual))) => {
                        assert_eq!(actual.as_handle_i64(), expected.as_handle_i64());
                    }
                    _ => assert_eq!(result, parameter),
                }
                drop(parameter);
                if let Value::Object(Some(owner)) = &result {
                    let weak =
                        ristretto_gc::GcWeak::new(owner, ristretto_gc::ReferenceKind::Weak, 0);
                    gc.collect_full()?;
                    gc.collect_full()?;
                    assert_eq!(&[77], &*result.as_int_vec_ref()?);
                    drop(result);
                    gc.collect_full()?;
                    gc.collect_full()?;
                    assert!(weak.upgrade().is_none());
                }
            }
            #[cfg(feature = "execution-profile")]
            {
                assert_eq!(function.execution_count(), 2);
                if let Some(callee) = callee {
                    assert_eq!(callee.execution_count(), 2);
                }
            }
            #[cfg(not(feature = "execution-profile"))]
            let _ = (function, callee);
        }
        assert!(thread.current_frame().is_err());
    }
    Ok(())
}

async fn throw_classes(thread: &Thread, caught: bool) -> Result<(Arc<Class>, Arc<Class>)> {
    let name = format!("InterfaceThrow{caught}");
    let descriptor = "(Ljava/lang/Throwable;)Ljava/lang/Throwable;";
    let mut pool = ConstantPool::new();
    let value = declaration(&mut pool, "value", descriptor)?;
    define(
        thread,
        &name,
        "java/lang/Object",
        &[],
        pool,
        vec![value],
        true,
    )
    .await?;
    let mut pool = ConstantPool::new();
    let target = pool.add_class(&name)?;
    let target = pool.add_interface_method_ref(target, "value", descriptor)?;
    let mut caller = method(
        &mut pool,
        "caller",
        &format!("(L{name};Ljava/lang/Throwable;)Ljava/lang/Throwable;"),
        vec![
            Instruction::Aload_0,
            Instruction::Aload_1,
            Instruction::Invokeinterface(target, 2),
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
                    range_pc: 2..7,
                    handler_pc: 8,
                    catch_type: 0,
                });
            }
        }
    }
    let caller = define(
        thread,
        &format!("InterfaceThrowCaller{caught}"),
        "java/lang/Object",
        &[],
        pool,
        vec![caller],
        false,
    )
    .await?;
    thread.initialize_class(&caller).await?;
    let mut pool = ConstantPool::new();
    let value = method_with_flags(
        &mut pool,
        "value",
        descriptor,
        vec![Instruction::Aload_1, Instruction::Athrow],
        MethodAccessFlags::PUBLIC,
    )?;
    let implementation = define(
        thread,
        &format!("InterfaceThrowImpl{caught}"),
        "java/lang/Object",
        &[&name],
        pool,
        vec![value],
        false,
    )
    .await?;
    thread.initialize_class(&implementation).await?;
    Ok((caller, implementation))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn interface_exceptions_transfer_owners_and_release_receiver_roots() -> Result<()> {
    for concurrent in [false, true] {
        for compiled in [false, true] {
            for caught in [false, true] {
                let (vm, thread) = vm(concurrent).await?;
                let gc = vm.garbage_collector();
                let (base, child) = throw_classes(&thread, caught).await?;
                let descriptor =
                    format!("(LInterfaceThrow{caught};Ljava/lang/Throwable;)Ljava/lang/Throwable;");
                let function = install_caller(&vm, &base, &descriptor)?;
                let callee = if compiled {
                    Some(install_experimental(
                        &vm,
                        &child,
                        "value",
                        "(Ljava/lang/Throwable;)Ljava/lang/Throwable;",
                    )?)
                } else {
                    None
                };
                let parameters = [
                    receiver(&thread, &child).await?,
                    receiver(&thread, &thread.class("java/lang/RuntimeException").await?).await?,
                ];
                let receiver_weak = ristretto_gc::GcWeak::new(
                    reference_owner(&parameters[0])?,
                    ristretto_gc::ReferenceKind::Weak,
                    0,
                );
                let result = thread
                    .execute(
                        &base,
                        &base.try_get_method("caller", &descriptor)?,
                        &parameters,
                    )
                    .await;
                let value = match result {
                    Ok(Some(value)) if caught => value,
                    Err(crate::Error::Throwable(value)) if !caught => value,
                    other => {
                        return Err(InternalError(format!("wrong interface error: {other:?}")));
                    }
                };
                assert_eq!(
                    reference_owner(&value)?.as_handle_i64(),
                    reference_owner(&parameters[1])?.as_handle_i64()
                );
                drop(parameters);
                let weak = ristretto_gc::GcWeak::new(
                    reference_owner(&value)?,
                    ristretto_gc::ReferenceKind::Weak,
                    0,
                );
                gc.collect_full()?;
                gc.collect_full()?;
                assert_eq!(
                    value.as_object_ref()?.class().name(),
                    "java/lang/RuntimeException"
                );
                assert!(receiver_weak.upgrade().is_none());
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
