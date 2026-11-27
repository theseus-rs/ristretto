//! Resolution and access ordering exercised by actual field request code.
use super::*;

#[expect(
    clippy::too_many_lines,
    reason = "one exported class hierarchy and its invalid bytecode access sites"
)]
async fn fixture(thread: &Thread) -> Result<(Arc<Class>, Arc<Class>, Arc<Class>)> {
    let mut pool = ConstantPool::new();
    let own = pool.add_class("FieldResolutionBase")?;
    let mut fields = Vec::new();
    for (name, flags) in [
        ("value", FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC),
        ("instance", FieldAccessFlags::PUBLIC),
        ("hidden", FieldAccessFlags::PRIVATE),
        (
            "immutable",
            FieldAccessFlags::PUBLIC | FieldAccessFlags::FINAL | FieldAccessFlags::STATIC,
        ),
    ] {
        fields.push(field(&mut pool, name, "I", flags)?);
    }
    let value = pool.add_field_ref(own, "value", "I")?;
    let init = method(
        &mut pool,
        "<clinit>",
        "()V",
        vec![
            Instruction::Bipush(7),
            Instruction::Putstatic(value),
            Instruction::Return,
        ],
    )?;
    let base = register_fields(
        thread,
        "FieldResolutionBase",
        "java/lang/Object",
        pool,
        vec![init],
        fields,
    )
    .await?;
    let mut pool = ConstantPool::new();
    // Accessing an inherited static must never enter this initializer.
    let init = method(&mut pool, "<clinit>", "()V", vec![Instruction::Goto(0)])?;
    let child = register_fields(
        thread,
        "FieldResolutionChild",
        "FieldResolutionBase",
        pool,
        vec![init],
        vec![],
    )
    .await?;
    let mut pool = ConstantPool::new();
    let child_ref = pool.add_class("FieldResolutionChild")?;
    let base_ref = pool.add_class("FieldResolutionBase")?;
    let value = pool.add_field_ref(child_ref, "value", "I")?;
    let instance = pool.add_field_ref(base_ref, "instance", "I")?;
    let hidden = pool.add_field_ref(base_ref, "hidden", "I")?;
    let mismatch = pool.add_field_ref(base_ref, "instance", "J")?;
    let immutable = pool.add_field_ref(base_ref, "immutable", "I")?;
    let mut methods = Vec::new();
    for (name, code) in [
        (
            "readInherited",
            vec![Instruction::Getstatic(value), Instruction::Ireturn],
        ),
        (
            "writeInherited",
            vec![
                Instruction::Bipush(9),
                Instruction::Putstatic(value),
                Instruction::Getstatic(value),
                Instruction::Ireturn,
            ],
        ),
        (
            "wrongStatic",
            vec![Instruction::Getstatic(instance), Instruction::Ireturn],
        ),
        (
            "wrongInstance",
            vec![
                Instruction::Aconst_null,
                Instruction::Getfield(value),
                Instruction::Ireturn,
            ],
        ),
        (
            "wrongDescriptor",
            vec![
                Instruction::Aconst_null,
                Instruction::Getfield(mismatch),
                Instruction::Pop2,
                Instruction::Iconst_0,
                Instruction::Ireturn,
            ],
        ),
        (
            "privateNull",
            vec![
                Instruction::Aconst_null,
                Instruction::Getfield(hidden),
                Instruction::Ireturn,
            ],
        ),
        (
            "validNull",
            vec![
                Instruction::Aconst_null,
                Instruction::Getfield(instance),
                Instruction::Ireturn,
            ],
        ),
        (
            "writeFinal",
            vec![
                Instruction::Iconst_1,
                Instruction::Putstatic(immutable),
                Instruction::Iconst_0,
                Instruction::Ireturn,
            ],
        ),
    ] {
        methods.push(method(&mut pool, name, "()I", code)?);
    }
    let caller = register_fields(
        thread,
        "FieldResolutionCaller",
        "java/lang/Object",
        pool,
        methods,
        vec![],
    )
    .await?;
    thread.initialize_class(&caller).await?;
    Ok((base, child, caller))
}

pub(super) async fn check(vm: &Arc<VM>, thread: &Arc<Thread>) -> Result<()> {
    let (base, child, caller) = fixture(thread).await?;
    for (name, expected, error) in [
        ("readInherited", 7, None),
        ("writeInherited", 9, None),
        (
            "wrongStatic",
            0,
            Some("java/lang/IncompatibleClassChangeError"),
        ),
        (
            "wrongInstance",
            0,
            Some("java/lang/IncompatibleClassChangeError"),
        ),
        ("wrongDescriptor", 0, Some("java/lang/NoSuchFieldError")),
        ("privateNull", 0, Some("java/lang/IllegalAccessError")),
        ("validNull", 0, Some("java/lang/NullPointerException")),
        ("writeFinal", 0, Some("java/lang/IllegalAccessError")),
    ] {
        let body = caller.try_get_method(name, "()I")?;
        for compiled in [false, true] {
            let function = if compiled {
                Some(install_experimental(vm, &caller, name, "()I")?)
            } else {
                None
            };
            for _ in 0..2 {
                if base.is_initialized()? {
                    base.set_static_value("value", Value::Int(7))?;
                }
                let result = thread.execute(&caller, &body, &[] as &[Value]).await;
                if let Some(expected) = error {
                    let Err(crate::Error::Throwable(error)) = result else {
                        return Err(InternalError(format!(
                            "{name} compiled={compiled}: {result:?}"
                        )));
                    };
                    assert_eq!(error.as_object_ref()?.class().name(), expected, "{name}");
                } else {
                    assert_eq!(result?, Some(Value::Int(expected)), "{name}");
                    if let Some(function) = &function {
                        base.set_static_value("value", Value::Int(7))?;
                        assert_eq!(
                            crate::jit::execute(
                                function,
                                &[],
                                vm.garbage_collector(),
                                vm,
                                thread,
                                &caller
                            )?,
                            Some(Value::Int(expected))
                        );
                    }
                }
                assert!(
                    !child.is_initialized()?,
                    "inherited access initialized the symbolic class"
                );
                assert!(thread.frames()?.is_empty());
            }
        }
    }
    assert_eq!(base.static_value("immutable")?, Value::Int(0));
    repeated_resolution_errors(vm, thread, &caller).await?;
    Ok(())
}

async fn repeated_resolution_errors(
    vm: &Arc<VM>,
    thread: &Arc<Thread>,
    caller: &Arc<Class>,
) -> Result<()> {
    let context = RuntimeContext::new(vm.garbage_collector(), vm, thread, caller)?;
    for name in ["wrongDescriptor", "privateNull"] {
        let method = caller.try_get_method(name, "()I")?;
        let index = method
            .code()
            .iter()
            .find_map(|instruction| match instruction {
                Instruction::Getfield(index) => Some(*index),
                _ => None,
            })
            .ok_or_else(|| InternalError("missing fixture field access".to_owned()))?;
        for _ in 0..2 {
            let result = fields::resolve(&context, index).await;
            match name {
                "wrongDescriptor" => assert!(matches!(
                    result,
                    Err(JavaError(crate::JavaError::NoSuchFieldError(_)))
                )),
                _ => assert!(matches!(
                    result,
                    Err(JavaError(crate::JavaError::IllegalAccessError(_)))
                )),
            }
        }
    }
    Ok(())
}
