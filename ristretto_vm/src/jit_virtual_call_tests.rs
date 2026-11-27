//! Experimental virtual dispatch through real VM frames and the normal selector.
use super::*;

fn default_constructor(
    pool: &mut ConstantPool,
    parent: &str,
) -> Result<ristretto_classfile::Method> {
    let parent = pool.add_class(parent)?;
    let init = pool.add_method_ref(parent, "<init>", "()V")?;
    method_with_flags(
        pool,
        "<init>",
        "()V",
        vec![
            Instruction::Aload_0,
            Instruction::Invokespecial(init),
            Instruction::Return,
        ],
        MethodAccessFlags::PUBLIC,
    )
}

async fn object(thread: &Thread, class: &Arc<Class>) -> Result<Value> {
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();
    let object = Object::new_in_async(gc, class.clone()).await?;
    Ok(Value::try_new_object_async(gc, Reference::Object(object)).await?)
}

async fn dispatch_classes(thread: &Thread) -> Result<Vec<Arc<Class>>> {
    let mut classes = Vec::new();
    for index in 0..7 {
        let name = format!("VirtualTarget{index}");
        let parent = match index {
            0 => "java/lang/Object",
            6 => "VirtualTarget5",
            _ => "VirtualTarget0",
        };
        let mut pool = ConstantPool::new();
        let mut methods = vec![default_constructor(&mut pool, parent)?];
        if index != 6 {
            methods.push(method_with_flags(
                &mut pool,
                "value",
                "()I",
                vec![Instruction::Bipush(index), Instruction::Ireturn],
                MethodAccessFlags::PUBLIC,
            )?);
        }
        if index == 0 {
            let target = pool.add_class(&name)?;
            let target = pool.add_method_ref(target, "value", "()I")?;
            methods.push(method(
                &mut pool,
                "caller",
                "(LVirtualTarget0;)I",
                vec![
                    Instruction::Aload_0,
                    Instruction::Invokevirtual(target),
                    Instruction::Ireturn,
                ],
            )?);
        }
        let class = register(thread, class_from_methods(&name, parent, pool, methods)?).await?;
        thread.initialize_class(&class).await?;
        classes.push(class);
    }
    Ok(classes)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn virtual_dispatch_selects_overrides_beyond_the_receiver_cache() -> Result<()> {
    for compiled in [false, true] {
        let (vm, thread) = vm(true).await?;
        let classes = dispatch_classes(&thread).await?;
        let base = classes
            .first()
            .ok_or_else(|| InternalError("missing base class".to_owned()))?;
        let function = install_caller(&vm, base, "(LVirtualTarget0;)I")?;
        let mut functions = Vec::new();
        if compiled {
            for class in classes.iter().take(6) {
                functions.push(install_experimental(&vm, class, "value", "()I")?);
            }
        }
        for _ in 0..2 {
            for (index, class) in classes.iter().enumerate() {
                let result = thread
                    .execute(
                        base,
                        &base.try_get_method("caller", "(LVirtualTarget0;)I")?,
                        &[object(&thread, class).await?],
                    )
                    .await?;
                assert_eq!(result, Some(Value::Int(i32::try_from(index.min(5))?)));
            }
        }
        let result = thread
            .execute(
                base,
                &base.try_get_method("caller", "(LVirtualTarget0;)I")?,
                &[Value::Object(None)],
            )
            .await;
        let Err(crate::Error::Throwable(value)) = result else {
            return Err(InternalError("null receiver did not throw".to_owned()));
        };
        assert_eq!(
            value.as_object_ref()?.class().name(),
            "java/lang/NullPointerException"
        );
        assert!(thread.current_frame().is_err());
        #[cfg(feature = "execution-profile")]
        {
            assert_eq!(function.execution_count(), 15);
            for (index, callee) in functions.iter().enumerate() {
                assert_eq!(callee.execution_count(), if index == 5 { 4 } else { 2 });
            }
        }
        #[cfg(not(feature = "execution-profile"))]
        let _ = function;
    }
    Ok(())
}

async fn pressure_classes(thread: &Thread) -> Result<(Arc<Class>, Arc<Class>)> {
    let mut pool = ConstantPool::new();
    let target = pool.add_class("VirtualPressureBase")?;
    let target = pool.add_method_ref(target, "allocate", "(I)[I")?;
    let constructor = default_constructor(&mut pool, "java/lang/Object")?;
    let allocate = method_with_flags(
        &mut pool,
        "allocate",
        "(I)[I",
        vec![Instruction::Aconst_null, Instruction::Areturn],
        MethodAccessFlags::PUBLIC,
    )?;
    let caller = method(
        &mut pool,
        "caller",
        "(LVirtualPressureBase;I)I",
        vec![
            Instruction::Iconst_1,
            Instruction::Newarray(ArrayType::Int),
            Instruction::Dup,
            Instruction::Iconst_0,
            Instruction::Bipush(77),
            Instruction::Iastore,
            Instruction::Aload_0,
            Instruction::Iload_1,
            Instruction::Invokevirtual(target),
            Instruction::Iconst_0,
            Instruction::Iaload,
            Instruction::Swap,
            Instruction::Iconst_0,
            Instruction::Iaload,
            Instruction::Iadd,
            Instruction::Ireturn,
        ],
    )?;
    let base = register(
        thread,
        class_from_methods(
            "VirtualPressureBase",
            "java/lang/Object",
            pool,
            vec![constructor, allocate, caller],
        )?,
    )
    .await?;
    let mut pool = ConstantPool::new();
    let constructor = default_constructor(&mut pool, base.name())?;
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
    let child = register(
        thread,
        class_from_methods(
            "VirtualPressureChild",
            base.name(),
            pool,
            vec![constructor, allocate],
        )?,
    )
    .await?;
    thread.initialize_class(&child).await?;
    Ok((base, child))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn virtual_allocation_retains_suspended_operands_and_returned_references() -> Result<()> {
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
    let (base, child) = pressure_classes(&thread).await?;
    let descriptor = "(LVirtualPressureBase;I)I";
    let function = install_caller(&vm, &base, descriptor)?;
    let callee = if compiled {
        Some(install_experimental(&vm, &child, "allocate", "(I)[I")?)
    } else {
        None
    };
    let parameters = [object(&thread, &child).await?, Value::Int(8000)];
    let before = gc.statistics()?;
    let diagnostics = gc.diagnostics();
    assert_eq!(
        thread
            .execute(
                &base,
                &base.try_get_method("caller", descriptor)?,
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
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn virtual_private_resolution_excludes_subclass_methods() -> Result<()> {
    let (vm, thread) = vm(true).await?;
    let mut classes = Vec::new();
    for (name, parent, value, flags) in [
        (
            "VirtualPrivateBase",
            "java/lang/Object",
            7,
            MethodAccessFlags::PRIVATE,
        ),
        (
            "VirtualPrivateChild",
            "VirtualPrivateBase",
            99,
            MethodAccessFlags::PUBLIC,
        ),
    ] {
        let mut pool = ConstantPool::new();
        let mut methods = vec![
            default_constructor(&mut pool, parent)?,
            method_with_flags(
                &mut pool,
                "value",
                "()I",
                vec![Instruction::Bipush(value), Instruction::Ireturn],
                flags,
            )?,
        ];
        if flags == MethodAccessFlags::PRIVATE {
            let target = pool.add_class(name)?;
            let target = pool.add_method_ref(target, "value", "()I")?;
            methods.push(method(
                &mut pool,
                "caller",
                "(LVirtualPrivateBase;)I",
                vec![
                    Instruction::Aload_0,
                    Instruction::Invokevirtual(target),
                    Instruction::Ireturn,
                ],
            )?);
        }
        let class = register(&thread, class_from_methods(name, parent, pool, methods)?).await?;
        thread.initialize_class(&class).await?;
        classes.push(class);
    }
    let base = classes
        .first()
        .ok_or_else(|| InternalError("missing private base".to_owned()))?;
    let function = install_caller(&vm, base, "(LVirtualPrivateBase;)I")?;
    let callee = install_experimental(&vm, base, "value", "()I")?;
    for class in &classes {
        assert_eq!(
            thread
                .execute(
                    base,
                    &base.try_get_method("caller", "(LVirtualPrivateBase;)I")?,
                    &[object(&thread, class).await?]
                )
                .await?,
            Some(Value::Int(7))
        );
    }
    #[cfg(feature = "execution-profile")]
    {
        assert_eq!(function.execution_count(), 2);
        assert_eq!(callee.execution_count(), 2);
    }
    #[cfg(not(feature = "execution-profile"))]
    let _ = (function, callee);
    Ok(())
}

async fn invalid_target(thread: &Thread, kind: &str) -> Result<(Arc<Class>, Arc<Class>)> {
    let name = format!("VirtualError{kind}");
    let mut pool = ConstantPool::new();
    let target = pool.add_class(&name)?;
    let target = pool.add_method_ref(target, "value", "()I")?;
    let mut methods = vec![
        default_constructor(&mut pool, "java/lang/Object")?,
        method(
            &mut pool,
            "caller",
            &format!("(L{name};)I"),
            vec![
                Instruction::Aload_0,
                Instruction::Invokevirtual(target),
                Instruction::Ireturn,
            ],
        )?,
    ];
    if kind == "Static" {
        methods.push(method(
            &mut pool,
            "value",
            "()I",
            vec![Instruction::Iconst_1, Instruction::Ireturn],
        )?);
    } else if kind == "Abstract" {
        methods.push(ristretto_classfile::Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
            name_index: pool.add_utf8("value")?,
            descriptor_index: pool.add_utf8("()I")?,
            attributes: vec![],
        });
    }
    let mut file = class_from_methods(&name, "java/lang/Object", pool, methods)?
        .class_file()
        .clone();
    if kind == "Abstract" {
        file.access_flags |= ClassAccessFlags::ABSTRACT;
    }
    let base = register(thread, Class::from(None, file)?).await?;
    let receiver_class = if kind == "Abstract" {
        let mut pool = ConstantPool::new();
        let constructor = default_constructor(&mut pool, base.name())?;
        register(
            thread,
            class_from_methods(
                "VirtualMissingImplementation",
                base.name(),
                pool,
                vec![constructor],
            )?,
        )
        .await?
    } else {
        base.clone()
    };
    thread.initialize_class(&receiver_class).await?;
    Ok((base, receiver_class))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn virtual_linkage_null_and_abstract_errors_keep_their_order() -> Result<()> {
    let (vm, thread) = vm(true).await?;
    for (kind, nonnull_error, null_error) in [
        (
            "Static",
            "java/lang/IncompatibleClassChangeError",
            "java/lang/IncompatibleClassChangeError",
        ),
        (
            "Missing",
            "java/lang/NoSuchMethodError",
            "java/lang/NoSuchMethodError",
        ),
        (
            "Abstract",
            "java/lang/AbstractMethodError",
            "java/lang/NullPointerException",
        ),
    ] {
        let (class, receiver_class) = invalid_target(&thread, kind).await?;
        let descriptor = format!("(L{};)I", class.name());
        let function = install_caller(&vm, &class, &descriptor)?;
        for (receiver, expected) in [
            (Value::Object(None), null_error),
            (object(&thread, &receiver_class).await?, nonnull_error),
        ] {
            let result = thread
                .execute(
                    &class,
                    &class.try_get_method("caller", &descriptor)?,
                    &[receiver],
                )
                .await;
            let Err(crate::Error::Throwable(error)) = result else {
                return Err(InternalError(
                    "invalid virtual target did not throw".to_owned(),
                ));
            };
            assert_eq!(error.as_object_ref()?.class().name(), expected);
            assert!(thread.current_frame().is_err());
        }
        #[cfg(feature = "execution-profile")]
        assert_eq!(function.execution_count(), 2);
        #[cfg(not(feature = "execution-profile"))]
        let _ = function;
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn virtual_native_call_resolves_primitive_array_classes_without_payload_guards() -> Result<()>
{
    let (vm, thread) = vm(true).await?;
    let gc = vm.garbage_collector();
    let mut pool = ConstantPool::new();
    let target = pool.add_class("java/lang/Object")?;
    let target = pool.add_method_ref(target, "getClass", "()Ljava/lang/Class;")?;
    let descriptor = "(Ljava/lang/Object;)Ljava/lang/Class;";
    let caller = method(
        &mut pool,
        "caller",
        descriptor,
        vec![
            Instruction::Aload_0,
            Instruction::Invokevirtual(target),
            Instruction::Areturn,
        ],
    )?;
    let class = register(
        &thread,
        class_from_methods("VirtualNative", "java/lang/Object", pool, vec![caller])?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    let function = install_caller(&vm, &class, descriptor)?;
    let object_class = thread.class("java/lang/Object").await?;
    let native = object_class.try_get_method("getClass", "()Ljava/lang/Class;")?;
    assert!(native.is_native());
    for receiver in [
        object(&thread, &object_class).await?,
        Value::try_new_object_async(gc, Reference::IntArray(vec![77].into())).await?,
        Value::try_new_object_async(gc, Reference::ByteArray(vec![11].into())).await?,
    ] {
        let expected = thread
            .execute(&object_class, &native, std::slice::from_ref(&receiver))
            .await?
            .ok_or_else(|| InternalError("missing class result".to_owned()))?;
        let actual = thread
            .execute(
                &class,
                &class.try_get_method("caller", descriptor)?,
                &[receiver],
            )
            .await?
            .ok_or_else(|| InternalError("missing class result".to_owned()))?;
        gc.collect_full()?;
        assert_eq!(
            reference_owner(&actual)?.as_handle_i64(),
            reference_owner(&expected)?.as_handle_i64()
        );
    }
    assert!(thread.current_frame().is_err());
    #[cfg(feature = "execution-profile")]
    {
        assert_eq!(function.execution_count(), 3);
        assert!(
            vm.execution_profile()
                .methods
                .iter()
                .any(|entry| entry.class == "java/lang/Object"
                    && entry.name == "getClass"
                    && entry.intrinsic_entries >= 6)
        );
    }
    #[cfg(not(feature = "execution-profile"))]
    let _ = function;
    Ok(())
}

async fn default_interface(
    thread: &Thread,
    name: &str,
    parent: Option<&str>,
    number: i8,
) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let this_class = pool.add_class(name)?;
    let super_class = pool.add_class("java/lang/Object")?;
    let interfaces = if let Some(parent) = parent {
        vec![pool.add_class(parent)?]
    } else {
        vec![]
    };
    let value = method_with_flags(
        &mut pool,
        "value",
        "()I",
        vec![Instruction::Bipush(number), Instruction::Ireturn],
        MethodAccessFlags::PUBLIC,
    )?;
    register(
        thread,
        Class::from(
            None,
            ClassFile {
                version: ristretto_classfile::JAVA_8,
                constant_pool: pool,
                this_class,
                super_class,
                interfaces,
                methods: vec![value],
                access_flags: ClassAccessFlags::PUBLIC
                    | ClassAccessFlags::INTERFACE
                    | ClassAccessFlags::ABSTRACT,
                ..Default::default()
            },
        )?,
    )
    .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn virtual_selection_uses_the_most_specific_interface_default() -> Result<()> {
    let (vm, thread) = vm(true).await?;
    default_interface(&thread, "VirtualDefault", None, 40).await?;
    default_interface(&thread, "VirtualRefinedDefault", Some("VirtualDefault"), 41).await?;
    let mut classes = Vec::new();
    for (name, parent, interface, own_value) in [
        (
            "VirtualDefaultBase",
            "java/lang/Object",
            Some("VirtualDefault"),
            None,
        ),
        (
            "VirtualDefaultChild",
            "VirtualDefaultBase",
            Some("VirtualRefinedDefault"),
            None,
        ),
        (
            "VirtualDefaultOverride",
            "VirtualDefaultChild",
            None,
            Some(42),
        ),
    ] {
        let mut pool = ConstantPool::new();
        let this_class = pool.add_class(name)?;
        let super_class = pool.add_class(parent)?;
        let interfaces = if let Some(interface) = interface {
            vec![pool.add_class(interface)?]
        } else {
            vec![]
        };
        let mut methods = vec![default_constructor(&mut pool, parent)?];
        if let Some(value) = own_value {
            methods.push(method_with_flags(
                &mut pool,
                "value",
                "()I",
                vec![Instruction::Bipush(value), Instruction::Ireturn],
                MethodAccessFlags::PUBLIC,
            )?);
        }
        if classes.is_empty() {
            let target = pool.add_method_ref(this_class, "value", "()I")?;
            methods.push(method(
                &mut pool,
                "caller",
                "(LVirtualDefaultBase;)I",
                vec![
                    Instruction::Aload_0,
                    Instruction::Invokevirtual(target),
                    Instruction::Ireturn,
                ],
            )?);
        }
        let class = register(
            &thread,
            Class::from(
                None,
                ClassFile {
                    version: ristretto_classfile::JAVA_8,
                    constant_pool: pool,
                    this_class,
                    super_class,
                    interfaces,
                    methods,
                    access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
                    ..Default::default()
                },
            )?,
        )
        .await?;
        thread.initialize_class(&class).await?;
        classes.push(class);
    }
    let base = classes
        .first()
        .ok_or_else(|| InternalError("missing default base".to_owned()))?;
    let function = install_caller(&vm, base, "(LVirtualDefaultBase;)I")?;
    for (index, class) in classes.iter().enumerate() {
        assert_eq!(
            thread
                .execute(
                    base,
                    &base.try_get_method("caller", "(LVirtualDefaultBase;)I")?,
                    &[object(&thread, class).await?]
                )
                .await?,
            Some(Value::Int(40 + i32::try_from(index)?))
        );
    }
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 3);
    #[cfg(not(feature = "execution-profile"))]
    let _ = function;
    Ok(())
}

async fn value_classes(thread: &Thread) -> Result<(Arc<Class>, Arc<Class>)> {
    let mut classes = Vec::new();
    for (name, parent, base) in [
        ("VirtualValuesBase", "java/lang/Object", true),
        ("VirtualValuesChild", "VirtualValuesBase", false),
    ] {
        let mut pool = ConstantPool::new();
        let target = pool.add_class(name)?;
        let mut methods = vec![default_constructor(&mut pool, parent)?];
        for (index, (kind, load, zero, ret)) in [
            (
                "J",
                Instruction::Lload_1,
                Instruction::Lconst_0,
                Instruction::Lreturn,
            ),
            (
                "F",
                Instruction::Fload_1,
                Instruction::Fconst_0,
                Instruction::Freturn,
            ),
            (
                "D",
                Instruction::Dload_1,
                Instruction::Dconst_0,
                Instruction::Dreturn,
            ),
            (
                "[I",
                Instruction::Aload_1,
                Instruction::Aconst_null,
                Instruction::Areturn,
            ),
        ]
        .into_iter()
        .enumerate()
        {
            let descriptor = format!("({kind}){kind}");
            methods.push(method_with_flags(
                &mut pool,
                "value",
                &descriptor,
                vec![if base { zero } else { load.clone() }, ret.clone()],
                MethodAccessFlags::PUBLIC,
            )?);
            if base {
                let reference = pool.add_method_ref(target, "value", &descriptor)?;
                methods.push(method(
                    &mut pool,
                    &format!("caller{index}"),
                    &format!("(LVirtualValuesBase;{kind}){kind}"),
                    vec![
                        Instruction::Aload_0,
                        load,
                        Instruction::Invokevirtual(reference),
                        ret,
                    ],
                )?);
            }
        }
        let class = register(thread, class_from_methods(name, parent, pool, methods)?).await?;
        thread.initialize_class(&class).await?;
        classes.push(class);
    }
    let [base, child] = classes.as_slice() else {
        return Err(InternalError("missing value hierarchy".to_owned()));
    };
    Ok((base.clone(), child.clone()))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn virtual_overrides_preserve_primitive_bits_and_transfer_reference_results() -> Result<()> {
    for compiled in [false, true] {
        let (vm, thread) = vm(true).await?;
        let gc = vm.garbage_collector();
        let (base, child) = value_classes(&thread).await?;
        let receiver = object(&thread, &child).await?;
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
            let descriptor = format!("(LVirtualValuesBase;{kind}){kind}");
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
                    .ok_or_else(|| InternalError("missing virtual result".to_owned()))?;
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
    let name = format!("VirtualThrowBase{caught}");
    let mut pool = ConstantPool::new();
    let target = pool.add_class(&name)?;
    let target = pool.add_method_ref(
        target,
        "value",
        "(Ljava/lang/Throwable;)Ljava/lang/Throwable;",
    )?;
    let constructor = default_constructor(&mut pool, "java/lang/Object")?;
    let value = method_with_flags(
        &mut pool,
        "value",
        "(Ljava/lang/Throwable;)Ljava/lang/Throwable;",
        vec![Instruction::Aconst_null, Instruction::Areturn],
        MethodAccessFlags::PUBLIC,
    )?;
    let descriptor = format!("(L{name};Ljava/lang/Throwable;)Ljava/lang/Throwable;");
    let mut caller = method(
        &mut pool,
        "caller",
        &descriptor,
        vec![
            Instruction::Aload_0,
            Instruction::Aload_1,
            Instruction::Invokevirtual(target),
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
                    range_pc: 2..5,
                    handler_pc: 6,
                    catch_type: 0,
                });
            }
        }
    }
    let base = register(
        thread,
        class_from_methods(
            &name,
            "java/lang/Object",
            pool,
            vec![constructor, value, caller],
        )?,
    )
    .await?;
    let mut pool = ConstantPool::new();
    let constructor = default_constructor(&mut pool, base.name())?;
    let value = method_with_flags(
        &mut pool,
        "value",
        "(Ljava/lang/Throwable;)Ljava/lang/Throwable;",
        vec![Instruction::Aload_1, Instruction::Athrow],
        MethodAccessFlags::PUBLIC,
    )?;
    let child = register(
        thread,
        class_from_methods(
            &format!("VirtualThrowChild{caught}"),
            base.name(),
            pool,
            vec![constructor, value],
        )?,
    )
    .await?;
    thread.initialize_class(&child).await?;
    Ok((base, child))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn virtual_exceptions_transfer_owners_and_release_receiver_roots() -> Result<()> {
    for concurrent in [false, true] {
        for compiled in [false, true] {
            for caught in [false, true] {
                let (vm, thread) = vm(concurrent).await?;
                let gc = vm.garbage_collector();
                let (base, child) = throw_classes(&thread, caught).await?;
                let descriptor = format!(
                    "(L{};Ljava/lang/Throwable;)Ljava/lang/Throwable;",
                    base.name()
                );
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
                    object(&thread, &child).await?,
                    object(&thread, &thread.class("java/lang/RuntimeException").await?).await?,
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
                    other => return Err(InternalError(format!("wrong virtual error: {other:?}"))),
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
