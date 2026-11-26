//! Reference-array request ABI tests. The embedding owns roots and Java type checks.
use super::*;
use crate::{AllocationKind, ArrayAccessKind};
use ristretto_classfile::attributes::{MaxLocals, MaxStack};

fn method<'c>(class: &'c ClassFile<'_>) -> Result<&'c Method> {
    class
        .methods
        .first()
        .ok_or_else(|| InternalError("fixture method".to_owned()))
}

fn allocation_fixture(component: &str) -> Result<(ClassFile<'static>, u16)> {
    let mut class = primitive_arrays::fixture(
        "(ILjava/lang/Object;)Ljava/lang/Object;",
        vec![Instruction::Aconst_null, Instruction::Areturn],
    )?;
    let index = class.constant_pool.add_class(component)?;
    let code = vec![
        Instruction::Iload_0,
        Instruction::Anewarray(index),
        Instruction::Astore_2,
        Instruction::Aload_2,
        Instruction::Iconst_0,
        Instruction::Aload_1,
        Instruction::Aastore,
        Instruction::Aload_2,
        Instruction::Iconst_0,
        Instruction::Aaload,
        Instruction::Areturn,
    ];
    let method = class
        .methods
        .first_mut()
        .ok_or_else(|| InternalError("fixture method".to_owned()))?;
    let max_stack = code.max_stack(&class.constant_pool)?;
    let max_locals = code.max_locals(&class.constant_pool, method)?;
    method.attributes = vec![Attribute::Code {
        name_index: class.constant_pool.add_utf8("Code")?,
        max_stack,
        max_locals,
        code,
        exception_table: vec![],
        attributes: vec![],
    }];
    class.verify()?;
    Ok((class, index))
}

fn published(context: &Context, token: i64) -> Result<bool> {
    Ok(context
        .roots
        .lock()
        .map_err(|error| InternalError(error.to_string()))?
        .contains(&token))
}

#[test]
fn reference_array_roundtrip_uses_only_requests_and_preserves_opaque_tokens() -> Result<()> {
    for component in ["java/lang/Object", "ClosedNode"] {
        for value in [0, 91, i64::MAX] {
            let (class, index) = allocation_fixture(component)?;
            let method = method(&class)?;
            assert!(!Compiler::can_compile(method));
            assert!(Compiler::experimental_rejection_reason(&class, method).is_some());
            assert_eq!(Compiler::request_rejection_reason(&class, method), None);
            let function = compile(&class, "run")?;
            let context = Context::new();
            let mut activation = function.activation(&[Value::I32(1), Value::Ptr(value)])?;
            // Activation owns generated code; no function or context address is retained.
            drop(function);
            assert!(matches!(
                activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
                ExecutionStep::Allocation(request)
                    if request.kind == AllocationKind::ReferenceArray(index) && request.bci == 1
            ));
            assert_eq!(activation.call_arguments()?, &[1]);
            if value != 0 {
                assert!(published(&context, value)?);
            }
            activation.complete_call(77)?;
            let moved_context = Box::new(context);
            assert!(matches!(
                activation.resume_with_context(NonZeroU32::MIN, moved_context.pointer())?,
                ExecutionStep::ArrayStore(request) if request.bci == 8
            ));
            assert_eq!(activation.call_arguments()?, &[77, 0, value]);
            assert!(published(&moved_context, 77)?);
            if value != 0 {
                assert!(published(&moved_context, value)?);
            }
            activation.complete_call(0)?;
            assert!(matches!(
                activation.resume_with_context(NonZeroU32::MIN, moved_context.pointer())?,
                ExecutionStep::ArrayAccess(request)
                    if request.kind == ArrayAccessKind::ReferenceLoad && request.bci == 11
            ));
            assert_eq!(activation.call_arguments()?, &[77, 0]);
            activation.complete_call(value)?;
            assert_eq!(
                activation.resume_with_context(NonZeroU32::MIN, moved_context.pointer())?,
                ExecutionStep::Returned(Some(Value::Ptr(value)))
            );
        }
    }
    Ok(())
}

#[test]
fn reference_array_faults_stop_before_subsequent_requests() -> Result<()> {
    let class = primitive_arrays::fixture(
        "([Ljava/lang/Object;ILjava/lang/Object;)Ljava/lang/Object;",
        vec![
            Instruction::Aload_0,
            Instruction::Iload_1,
            Instruction::Aload_2,
            Instruction::Aastore,
            Instruction::Aload_0,
            Instruction::Iload_1,
            Instruction::Aaload,
            Instruction::Areturn,
        ],
    )?;
    // Java null/bounds/store precedence is owned by the VM helper. Native code
    // must deliver all operands unchanged and stop when that helper records an error.
    for (array, index, value) in [(0, -1, 99), (77, -1, 99), (77, i32::MAX, 99), (77, 0, 88)] {
        let function = compile(&class, "run")?;
        let context = Context::new();
        let mut activation =
            function.activation(&[Value::Ptr(array), Value::I32(index), Value::Ptr(value)])?;
        assert!(matches!(
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
            ExecutionStep::ArrayStore(_)
        ));
        assert_eq!(
            activation.call_arguments()?,
            &[array, i64::from(index), value]
        );
        context.failed.store(true, Ordering::Relaxed);
        activation.complete_call(0)?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
            ExecutionStep::Returned(None)
        );
    }
    Ok(())
}

#[test]
fn failed_allocation_and_rejected_load_result_do_not_escape() -> Result<()> {
    let (class, _) = allocation_fixture("java/lang/Object")?;
    let function = compile(&class, "run")?;
    let context = Context::new();
    let mut activation = function.activation(&[Value::I32(-1), Value::Ptr(0)])?;
    assert!(matches!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Allocation(_)
    ));
    assert_eq!(activation.call_arguments()?, &[-1]);
    context.failed.store(true, Ordering::Relaxed);
    activation.complete_call(0)?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Returned(None)
    );

    let class = primitive_arrays::fixture(
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        vec![
            Instruction::Aload_0,
            Instruction::Iconst_0,
            Instruction::Aaload,
            Instruction::Areturn,
        ],
    )?;
    let function = compile(&class, "run")?;
    let context = Context::new();
    let mut activation = function.activation(&[Value::Ptr(77)])?;
    assert!(matches!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::ArrayAccess(request) if request.kind == ArrayAccessKind::ReferenceLoad
    ));
    context.rejected_token.store(88, Ordering::Relaxed);
    activation.complete_call(88)?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Returned(None)
    );
    assert!(context.failed.load(Ordering::Relaxed));
    Ok(())
}

#[test]
fn array_component_and_invalid_constant_pool_indices_reject_before_codegen() -> Result<()> {
    for component in ["[Ljava/lang/Object;", "[I", "[[Ljava/lang/Object;"] {
        let (class, _) = allocation_fixture(component)?;
        assert_eq!(
            Compiler::request_rejection_reason(&class, method(&class)?),
            Some("experimental_unqualified_instruction")
        );
        assert!(compile(&class, "run").is_err());
    }
    let (mut class, _) = allocation_fixture("java/lang/Object")?;
    let wrong_kind = class.constant_pool.add_utf8("not a Class constant")?;
    for index in [0, wrong_kind] {
        let method = class
            .methods
            .first_mut()
            .ok_or_else(|| InternalError("fixture method".to_owned()))?;
        for attribute in &mut method.attributes {
            if let Attribute::Code { code, .. } = attribute {
                let instruction = code
                    .get_mut(1)
                    .ok_or_else(|| InternalError("fixture allocation".to_owned()))?;
                *instruction = Instruction::Anewarray(index);
            }
        }
        assert_eq!(
            Compiler::request_rejection_reason(&class, self::method(&class)?),
            Some("experimental_unqualified_instruction")
        );
        assert!(compile(&class, "run").is_err());
    }
    Ok(())
}

#[test]
fn reference_array_load_and_store_do_not_expand_ordinary_dispatch_policy() -> Result<()> {
    for (descriptor, code) in [
        (
            "([Ljava/lang/Object;)Ljava/lang/Object;",
            vec![
                Instruction::Aload_0,
                Instruction::Iconst_0,
                Instruction::Aaload,
                Instruction::Areturn,
            ],
        ),
        (
            "([Ljava/lang/Object;Ljava/lang/Object;)V",
            vec![
                Instruction::Aload_0,
                Instruction::Iconst_0,
                Instruction::Aload_1,
                Instruction::Aastore,
                Instruction::Return,
            ],
        ),
    ] {
        let class = primitive_arrays::fixture(descriptor, code)?;
        let method = method(&class)?;
        assert!(!Compiler::can_compile(method));
        assert_eq!(
            Compiler::experimental_rejection_reason(&class, method),
            Some("experimental_unqualified_instruction")
        );
        assert_eq!(Compiler::request_rejection_reason(&class, method), None);
        let _function = compile(&class, "run")?;
    }
    Ok(())
}
