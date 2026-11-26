//! Experimental constructor and non-virtual instance calls through real frames.
use super::*;
use ristretto_classfile::{Field, FieldAccessFlags, FieldType};

fn constructor_class() -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let this_class = pool.add_class("SpecialConstructor")?;
    let super_class = pool.add_class("java/lang/Object")?;
    let parent_init = pool.add_method_ref(super_class, "<init>", "()V")?;
    let init = pool.add_method_ref(this_class, "<init>", "(I)V")?;
    let value = pool.add_field_ref(this_class, "value", "I")?;
    let field = Field {
        access_flags: FieldAccessFlags::PUBLIC,
        name_index: pool.add_utf8("value")?,
        descriptor_index: pool.add_utf8("I")?,
        field_type: FieldType::parse("I")?,
        attributes: vec![],
    };
    let constructor = method_with_flags(
        &mut pool,
        "<init>",
        "(I)V",
        vec![
            Instruction::Aload_0,
            Instruction::Invokespecial(parent_init),
            Instruction::Aload_0,
            Instruction::Iload_1,
            Instruction::Putfield(value),
            Instruction::Return,
        ],
        MethodAccessFlags::PUBLIC,
    )?;
    let caller = method(
        &mut pool,
        "caller",
        "(I)LSpecialConstructor;",
        vec![
            Instruction::New(this_class),
            Instruction::Dup,
            Instruction::Iload_0,
            Instruction::Invokespecial(init),
            Instruction::Areturn,
        ],
    )?;
    Ok(Class::from(
        None,
        ClassFile {
            version: ristretto_classfile::JAVA_5,
            constant_pool: pool,
            this_class,
            super_class,
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
            fields: vec![field],
            methods: vec![constructor, caller],
            ..Default::default()
        },
    )?)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn special_constructor_crosses_real_frames_and_returns_the_owned_receiver() -> Result<()> {
    for concurrent in [false, true] {
        for compiled in [false, true] {
            let (vm, thread) = vm(concurrent).await?;
            let gc = vm.garbage_collector();
            let class = register(&thread, constructor_class()?).await?;
            thread.initialize_class(&class).await?;
            let descriptor = "(I)LSpecialConstructor;";
            let function = install_caller(&vm, &class, descriptor)?;
            let constructor = if compiled {
                Some(install_experimental(&vm, &class, "<init>", "(I)V")?)
            } else {
                None
            };
            let value = thread
                .execute(
                    &class,
                    &class.try_get_method("caller", descriptor)?,
                    &[Value::Int(77)],
                )
                .await?
                .ok_or_else(|| InternalError("missing constructed receiver".to_owned()))?;
            let weak = ristretto_gc::GcWeak::new(
                reference_owner(&value)?,
                ristretto_gc::ReferenceKind::Weak,
                0,
            );
            gc.collect_full()?;
            gc.collect_full()?;
            assert_eq!(value.as_object_ref()?.value("value")?, Value::Int(77));
            drop(value);
            gc.collect_full()?;
            gc.collect_full()?;
            assert!(weak.upgrade().is_none());
            assert!(thread.current_frame().is_err());
            #[cfg(feature = "execution-profile")]
            {
                assert_eq!(function.execution_count(), 1);
                if let Some(constructor) = constructor {
                    assert_eq!(constructor.execution_count(), 1);
                }
            }
            #[cfg(not(feature = "execution-profile"))]
            let _ = (function, constructor);
        }
    }
    Ok(())
}

fn pressure_constructor_class() -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let this_class = pool.add_class("SpecialPressureConstructor")?;
    let super_class = pool.add_class("java/lang/Object")?;
    let parent_init = pool.add_method_ref(super_class, "<init>", "()V")?;
    let init = pool.add_method_ref(this_class, "<init>", "(I)V")?;
    let array = pool.add_field_ref(this_class, "array", "[I")?;
    let field = Field {
        access_flags: FieldAccessFlags::PUBLIC,
        name_index: pool.add_utf8("array")?,
        descriptor_index: pool.add_utf8("[I")?,
        field_type: FieldType::parse("[I")?,
        attributes: vec![],
    };
    let constructor = method_with_flags(
        &mut pool,
        "<init>",
        "(I)V",
        vec![
            Instruction::Aload_0,
            Instruction::Invokespecial(parent_init),
            Instruction::Aload_0,
            Instruction::Iconst_1,
            Instruction::Newarray(ArrayType::Int),
            Instruction::Dup,
            Instruction::Iconst_0,
            Instruction::Bipush(77),
            Instruction::Iastore,
            Instruction::Putfield(array),
            Instruction::Iload_1,
            Instruction::Ifeq(15),
            Instruction::Sipush(2048),
            Instruction::Newarray(ArrayType::Int),
            Instruction::Pop,
            Instruction::Iinc(1, -1),
            Instruction::Goto(-13),
            Instruction::Return,
        ],
        MethodAccessFlags::PUBLIC,
    )?;
    let caller = method(
        &mut pool,
        "caller",
        "(I)LSpecialPressureConstructor;",
        vec![
            Instruction::New(this_class),
            Instruction::Dup,
            Instruction::Iload_0,
            Instruction::Invokespecial(init),
            Instruction::Areturn,
        ],
    )?;
    Ok(Class::from(
        None,
        ClassFile {
            version: ristretto_classfile::JAVA_5,
            constant_pool: pool,
            this_class,
            super_class,
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
            fields: vec![field],
            methods: vec![constructor, caller],
            ..Default::default()
        },
    )?)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn special_constructor_retains_receiver_edges_during_reclamation() -> Result<()> {
    for concurrent in [false, true] {
        for workers in [0, 1] {
            for compiled in [false, true] {
                pressure_constructor(concurrent, workers, compiled).await?;
            }
        }
    }
    Ok(())
}

async fn pressure_constructor(concurrent: bool, workers: usize, compiled: bool) -> Result<()> {
    let (vm, thread) = vm_with_workers(concurrent, workers).await?;
    let gc = vm.garbage_collector();
    let class = register(&thread, pressure_constructor_class()?).await?;
    thread.initialize_class(&class).await?;
    let descriptor = "(I)LSpecialPressureConstructor;";
    let function = install_caller(&vm, &class, descriptor)?;
    let constructor = if compiled {
        Some(install_experimental(&vm, &class, "<init>", "(I)V")?)
    } else {
        None
    };
    let before = gc.statistics()?;
    let diagnostics = gc.diagnostics();
    let value = thread
        .execute(
            &class,
            &class.try_get_method("caller", descriptor)?,
            &[Value::Int(8000)],
        )
        .await?
        .ok_or_else(|| InternalError("missing constructed receiver".to_owned()))?;
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
    let weak = ristretto_gc::GcWeak::new(
        reference_owner(&value)?,
        ristretto_gc::ReferenceKind::Weak,
        0,
    );
    gc.collect_full()?;
    gc.collect_full()?;
    let array = value.as_object_ref()?.value("array")?;
    assert_eq!(&[77], &*array.as_int_vec_ref()?);
    let array_weak = ristretto_gc::GcWeak::new(
        reference_owner(&array)?,
        ristretto_gc::ReferenceKind::Weak,
        0,
    );
    drop((array, value));
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(weak.upgrade().is_none());
    assert!(array_weak.upgrade().is_none());
    assert!(thread.current_frame().is_err());
    #[cfg(feature = "execution-profile")]
    {
        assert_eq!(function.execution_count(), 1);
        if let Some(constructor) = constructor {
            assert_eq!(constructor.execution_count(), 1);
        }
    }
    #[cfg(not(feature = "execution-profile"))]
    let _ = (function, constructor);
    Ok(())
}

fn empty_constructor(pool: &mut ConstantPool, parent: &str) -> Result<ristretto_classfile::Method> {
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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn special_superclass_selection_ignores_receiver_overrides_and_checks_null() -> Result<()> {
    let (vm, thread) = vm(true).await?;
    let mut leaf = None;
    for (name, parent, number) in [
        ("SpecialGrandparent", "java/lang/Object", 1),
        ("SpecialParent", "SpecialGrandparent", 2),
        ("SpecialLeaf", "SpecialParent", 3),
    ] {
        let mut pool = ConstantPool::new();
        let mut methods = vec![
            empty_constructor(&mut pool, parent)?,
            method_with_flags(
                &mut pool,
                "value",
                "()I",
                vec![Instruction::Bipush(number), Instruction::Ireturn],
                MethodAccessFlags::PUBLIC,
            )?,
        ];
        if number == 3 {
            let target = pool.add_class("SpecialGrandparent")?;
            let target = pool.add_method_ref(target, "value", "()I")?;
            methods.push(method(
                &mut pool,
                "caller",
                "(LSpecialLeaf;)I",
                vec![
                    Instruction::Aload_0,
                    Instruction::Invokespecial(target),
                    Instruction::Ireturn,
                ],
            )?);
        }
        let class = register(&thread, class_from_methods(name, parent, pool, methods)?).await?;
        thread.initialize_class(&class).await?;
        leaf = Some(class);
    }
    let class = leaf.ok_or_else(|| InternalError("missing leaf".to_owned()))?;
    let function = install_caller(&vm, &class, "(LSpecialLeaf;)I")?;
    let object = Object::new_in_async(vm.garbage_collector(), class.clone()).await?;
    let value =
        Value::try_new_object_async(vm.garbage_collector(), Reference::Object(object)).await?;
    for _ in 0..2 {
        assert_eq!(
            thread
                .execute(
                    &class,
                    &class.try_get_method("caller", "(LSpecialLeaf;)I")?,
                    std::slice::from_ref(&value)
                )
                .await?,
            Some(Value::Int(2))
        );
    }
    let result = thread
        .execute(
            &class,
            &class.try_get_method("caller", "(LSpecialLeaf;)I")?,
            &[Value::Object(None)],
        )
        .await;
    let Err(crate::Error::Throwable(error)) = result else {
        return Err(InternalError("null receiver did not throw".to_owned()));
    };
    assert_eq!(
        error.as_object_ref()?.class().name(),
        "java/lang/NullPointerException"
    );
    assert!(thread.current_frame().is_err());
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 3);
    #[cfg(not(feature = "execution-profile"))]
    let _ = function;
    Ok(())
}

fn throwing_constructor(caught: bool) -> Result<Arc<Class>> {
    let name = format!("SpecialThrow{caught}");
    let mut pool = ConstantPool::new();
    let target = pool.add_class(&name)?;
    let init = pool.add_method_ref(target, "<init>", "(Ljava/lang/Throwable;)V")?;
    let parent = pool.add_class("java/lang/Object")?;
    let parent_init = pool.add_method_ref(parent, "<init>", "()V")?;
    let constructor = method_with_flags(
        &mut pool,
        "<init>",
        "(Ljava/lang/Throwable;)V",
        vec![
            Instruction::Aload_0,
            Instruction::Invokespecial(parent_init),
            Instruction::Aload_1,
            Instruction::Athrow,
        ],
        MethodAccessFlags::PUBLIC,
    )?;
    let mut caller = method(
        &mut pool,
        "caller",
        "(Ljava/lang/Throwable;)Ljava/lang/Throwable;",
        vec![
            Instruction::New(target),
            Instruction::Dup,
            Instruction::Aload_0,
            Instruction::Invokespecial(init),
            Instruction::Pop,
            Instruction::Aconst_null,
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
                    range_pc: 5..8,
                    handler_pc: 11,
                    catch_type: 0,
                });
            }
        }
    }
    class_from_methods(&name, "java/lang/Object", pool, vec![constructor, caller])
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn special_constructor_exceptions_transfer_owners_and_unwind_frames() -> Result<()> {
    for concurrent in [false, true] {
        for compiled in [false, true] {
            for caught in [false, true] {
                let (vm, thread) = vm(concurrent).await?;
                let gc = vm.garbage_collector();
                let class = register(&thread, throwing_constructor(caught)?).await?;
                thread.initialize_class(&class).await?;
                let descriptor = "(Ljava/lang/Throwable;)Ljava/lang/Throwable;";
                let function = install_caller(&vm, &class, descriptor)?;
                let constructor = if compiled {
                    Some(install_experimental(
                        &vm,
                        &class,
                        "<init>",
                        "(Ljava/lang/Throwable;)V",
                    )?)
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
                        return Err(InternalError(format!("wrong constructor error: {other:?}")));
                    }
                };
                assert_eq!(
                    reference_owner(&value)?.as_handle_i64(),
                    reference_owner(&parameters[0])?.as_handle_i64()
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
                drop(value);
                gc.collect_full()?;
                gc.collect_full()?;
                assert!(weak.upgrade().is_none());
                assert!(thread.current_frame().is_err());
                #[cfg(feature = "execution-profile")]
                {
                    assert_eq!(function.execution_count(), 1);
                    if let Some(constructor) = constructor {
                        assert_eq!(constructor.execution_count(), 1);
                    }
                }
                #[cfg(not(feature = "execution-profile"))]
                let _ = (function, constructor);
            }
        }
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn special_interface_default_uses_the_symbolic_target() -> Result<()> {
    let (vm, thread) = vm(true).await?;
    let mut pool = ConstantPool::new();
    let this_class = pool.add_class("SpecialDefault")?;
    let super_class = pool.add_class("java/lang/Object")?;
    let value = method_with_flags(
        &mut pool,
        "value",
        "()I",
        vec![Instruction::Bipush(42), Instruction::Ireturn],
        MethodAccessFlags::PUBLIC,
    )?;
    let interface = register(
        &thread,
        Class::from(
            None,
            ClassFile {
                version: ristretto_classfile::JAVA_8,
                constant_pool: pool,
                this_class,
                super_class,
                access_flags: ClassAccessFlags::PUBLIC
                    | ClassAccessFlags::INTERFACE
                    | ClassAccessFlags::ABSTRACT,
                methods: vec![value],
                ..Default::default()
            },
        )?,
    )
    .await?;
    let mut pool = ConstantPool::new();
    let this_class = pool.add_class("SpecialDefaultCaller")?;
    let super_class = pool.add_class("java/lang/Object")?;
    let target = pool.add_class(interface.name())?;
    let value = pool.add_interface_method_ref(target, "value", "()I")?;
    let constructor = empty_constructor(&mut pool, "java/lang/Object")?;
    let caller = method(
        &mut pool,
        "caller",
        "(LSpecialDefaultCaller;)I",
        vec![
            Instruction::Aload_0,
            Instruction::Invokespecial(value),
            Instruction::Ireturn,
        ],
    )?;
    let override_method = method_with_flags(
        &mut pool,
        "value",
        "()I",
        vec![Instruction::Bipush(99), Instruction::Ireturn],
        MethodAccessFlags::PUBLIC,
    )?;
    let class = register(
        &thread,
        Class::from(
            None,
            ClassFile {
                version: ristretto_classfile::JAVA_8,
                constant_pool: pool,
                this_class,
                super_class,
                access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
                interfaces: vec![target],
                methods: vec![constructor, caller, override_method],
                ..Default::default()
            },
        )?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    let function = install_caller(&vm, &class, "(LSpecialDefaultCaller;)I")?;
    let object = Object::new_in_async(vm.garbage_collector(), class.clone()).await?;
    let value =
        Value::try_new_object_async(vm.garbage_collector(), Reference::Object(object)).await?;
    assert_eq!(
        thread
            .execute(
                &class,
                &class.try_get_method("caller", "(LSpecialDefaultCaller;)I")?,
                &[value]
            )
            .await?,
        Some(Value::Int(42))
    );
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 1);
    #[cfg(not(feature = "execution-profile"))]
    let _ = function;
    assert!(thread.current_frame().is_err());
    Ok(())
}

#[test]
fn special_call_argument_limit_includes_the_receiver() -> Result<()> {
    let mut pool = ConstantPool::new();
    let class = pool.add_class("TooManySpecialArguments")?;
    let descriptor = format!("({})V", "I".repeat(255));
    let target = pool.add_method_ref(class, "callee", &descriptor)?;
    let mut code = vec![Instruction::Aconst_null];
    code.extend(vec![Instruction::Iconst_0; 255]);
    code.extend([Instruction::Invokespecial(target), Instruction::Return]);
    // The fixture analyzer also rejects 256 slots. Bypass it to exercise
    // the compiler's independent limit before it writes the outgoing buffer.
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
    let class = class_from_methods(
        "TooManySpecialArguments",
        "java/lang/Object",
        pool,
        vec![caller],
    )?;
    let result = ristretto_jit::Compiler::new()?.compile(
        class.class_file(),
        class.try_get_method("caller", "()V")?.definition(),
        &jit_runtime_helpers::symbols(),
    );
    assert!(
        matches!(result, Err(ristretto_jit::Error::UnsupportedMethod(message)) if message.contains("parameter-slot limit"))
    );
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn special_private_call_uses_all_255_cells_including_the_receiver() -> Result<()> {
    let (vm, thread) = vm(true).await?;
    let mut pool = ConstantPool::new();
    let target = pool.add_class("SpecialMaximum")?;
    let descriptor = format!("({})I", "I".repeat(254));
    let caller_descriptor = format!("(LSpecialMaximum;{})I", "I".repeat(254));
    let target = pool.add_method_ref(target, "value", &descriptor)?;
    let mut methods = vec![empty_constructor(&mut pool, "java/lang/Object")?];
    let mut code = vec![Instruction::Aload_0];
    let mut sum = Vec::new();
    let mut parameters = Vec::new();
    let mut expected = 0_i32;
    for index in 1..=254 {
        code.push(Instruction::Iload(index));
        if index > 1 {
            sum.extend([Instruction::Bipush(31), Instruction::Imul]);
        }
        sum.push(Instruction::Iload(index));
        if index > 1 {
            sum.push(Instruction::Iadd);
        }
        let value = i32::from(index) * 7 - 11;
        parameters.push(Value::Int(value));
        expected = expected.wrapping_mul(31).wrapping_add(value);
    }
    code.extend([Instruction::Invokespecial(target), Instruction::Ireturn]);
    sum.push(Instruction::Ireturn);
    methods.push(method_with_flags(
        &mut pool,
        "value",
        &descriptor,
        sum,
        MethodAccessFlags::PRIVATE,
    )?);
    methods.push(method(&mut pool, "caller", &caller_descriptor, code)?);
    let class = register(
        &thread,
        class_from_methods("SpecialMaximum", "java/lang/Object", pool, methods)?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    let function = install_caller(&vm, &class, &caller_descriptor)?;
    let callee = install_experimental(&vm, &class, "value", &descriptor)?;
    let object = Object::new_in_async(vm.garbage_collector(), class.clone()).await?;
    parameters.insert(
        0,
        Value::try_new_object_async(vm.garbage_collector(), Reference::Object(object)).await?,
    );
    assert_eq!(
        thread
            .execute(
                &class,
                &class.try_get_method("caller", &caller_descriptor)?,
                &parameters
            )
            .await?,
        Some(Value::Int(expected))
    );
    #[cfg(feature = "execution-profile")]
    {
        assert_eq!(function.execution_count(), 1);
        assert_eq!(callee.execution_count(), 1);
    }
    #[cfg(not(feature = "execution-profile"))]
    let _ = (function, callee);
    Ok(())
}

fn wide_private_class() -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let target = pool.add_class("SpecialWide")?;
    let mut methods = vec![empty_constructor(&mut pool, "java/lang/Object")?];
    for (index, (kind, load, ret)) in [
        ("J", Instruction::Lload_1, Instruction::Lreturn),
        ("F", Instruction::Fload_1, Instruction::Freturn),
        ("D", Instruction::Dload_1, Instruction::Dreturn),
        ("[I", Instruction::Aload_1, Instruction::Areturn),
    ]
    .into_iter()
    .enumerate()
    {
        let descriptor = format!("({kind}){kind}");
        let reference = pool.add_method_ref(target, "value", &descriptor)?;
        methods.push(method_with_flags(
            &mut pool,
            "value",
            &descriptor,
            vec![load.clone(), ret.clone()],
            MethodAccessFlags::PRIVATE,
        )?);
        methods.push(method(
            &mut pool,
            &format!("caller{index}"),
            &format!("(LSpecialWide;{kind}){kind}"),
            vec![
                Instruction::Aload_0,
                load,
                Instruction::Invokespecial(reference),
                ret,
            ],
        )?);
    }
    class_from_methods("SpecialWide", "java/lang/Object", pool, methods)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn special_private_calls_preserve_wide_bits_and_reference_results() -> Result<()> {
    for compiled in [false, true] {
        let (vm, thread) = vm(true).await?;
        let gc = vm.garbage_collector();
        let class = register(&thread, wide_private_class()?).await?;
        thread.initialize_class(&class).await?;
        let object = Object::new_in_async(gc, class.clone()).await?;
        let receiver = Value::try_new_object_async(gc, Reference::Object(object)).await?;
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
            let descriptor = format!("(LSpecialWide;{kind}){kind}");
            let function = install_experimental(&vm, &class, &name, &descriptor)?;
            let callee = if compiled {
                Some(install_experimental(
                    &vm,
                    &class,
                    "value",
                    &format!("({kind}){kind}"),
                )?)
            } else {
                None
            };
            for parameter in parameters {
                let result = thread
                    .execute(
                        &class,
                        &class.try_get_method(&name, &descriptor)?,
                        &[receiver.clone(), parameter.clone()],
                    )
                    .await?
                    .ok_or_else(|| InternalError("missing private result".to_owned()))?;
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
