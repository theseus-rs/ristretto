//! Casts transfer opaque tokens through requests; the embedding decides assignability.
use super::*;
use crate::TypeCheckKind;

fn method<'c>(class: &'c ClassFile<'_>) -> Result<&'c Method> {
    class
        .methods
        .first()
        .ok_or_else(|| InternalError("cast method".to_owned()))
}

fn code_mut<'c>(class: &'c mut ClassFile<'_>) -> Result<&'c mut Vec<Instruction>> {
    class
        .methods
        .first_mut()
        .and_then(|method| {
            method.attributes.iter_mut().find_map(|attribute| {
                if let Attribute::Code { code, .. } = attribute {
                    Some(code)
                } else {
                    None
                }
            })
        })
        .ok_or_else(|| InternalError("cast code".to_owned()))
}

fn fixture(target: &str, twice: bool) -> Result<(ClassFile<'static>, u16)> {
    let mut class = primitive_arrays::fixture(
        "(Ljava/lang/Object;)Ljava/lang/Object;",
        vec![Instruction::Aload_0, Instruction::Areturn],
    )?;
    let index = class.constant_pool.add_class(target)?;
    // Clear the parameter local: while suspended, only the request owns its token.
    let code = code_mut(&mut class)?;
    *code = vec![
        Instruction::Aload_0,
        Instruction::Aconst_null,
        Instruction::Astore_0,
        Instruction::Checkcast(index),
    ];
    if twice {
        code.push(Instruction::Checkcast(index));
    }
    code.push(Instruction::Areturn);
    // The two-slot prefix is the only larger stack shape than the initial fixture.
    for attribute in &mut class
        .methods
        .first_mut()
        .ok_or_else(|| InternalError("method".to_owned()))?
        .attributes
    {
        if let Attribute::Code { max_stack, .. } = attribute {
            *max_stack = 2;
        }
    }
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
fn casts_suspend_with_one_published_token_and_preserve_identity_without_legacy_helpers()
-> Result<()> {
    for target in [
        "java/lang/Object",
        "ClosedNode",
        "[Z",
        "[B",
        "[C",
        "[S",
        "[I",
        "[J",
        "[F",
        "[D",
        "[Ljava/lang/Object;",
        "[LClosedNode;",
    ] {
        let (class, index) = fixture(target, false)?;
        class.verify()?;
        assert!(!Compiler::can_compile(method(&class)?));
        assert!(Compiler::experimental_rejection_reason(&class, method(&class)?).is_some());
        assert_eq!(
            Compiler::request_rejection_reason(&class, method(&class)?),
            None
        );
        for token in [0, 91, i64::MAX] {
            let function = compile(&class, "run")?;
            let context = Context::new();
            let mut activation = function.activation(&[Value::Ptr(token)])?;
            drop(function);
            assert!(
                matches!(activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
                ExecutionStep::TypeCheck(request) if request.kind == TypeCheckKind::CheckCast
                    && request.index == index && request.bci == 3)
            );
            assert_eq!(activation.call_arguments()?, &[token]);
            if token != 0 {
                assert!(published(&context, token)?);
            }
            // A parked activation never retains the embedding context address.
            let context = Box::new(context);
            activation.complete_call(token)?;
            assert_eq!(
                activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
                ExecutionStep::Returned(Some(Value::Ptr(token)))
            );
            assert!(!context.failed.load(Ordering::Relaxed));
        }
    }
    Ok(())
}

#[test]
fn failed_cast_or_rejected_return_root_stops_before_the_next_request() -> Result<()> {
    let (class, _) = fixture("ClosedNode", true)?;
    for reject_return in [false, true] {
        let function = compile(&class, "run")?;
        let context = Context::new();
        let mut activation = function.activation(&[Value::Ptr(77)])?;
        assert!(matches!(
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
            ExecutionStep::TypeCheck(_)
        ));
        assert_eq!(activation.call_arguments()?, &[77]);
        assert!(published(&context, 77)?);
        if reject_return {
            context.rejected_token.store(77, Ordering::Relaxed);
        } else {
            context.failed.store(true, Ordering::Relaxed);
        }
        activation.complete_call(77)?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
            ExecutionStep::Returned(None)
        );
        assert!(context.failed.load(Ordering::Relaxed));
    }
    Ok(())
}

#[test]
fn invalid_cast_targets_and_instanceof_are_rejected_before_linking() -> Result<()> {
    for target in [
        "[[I",
        "[[LClosedNode;",
        "[Q",
        "[",
        "[L;",
        "Closed.Node",
        "java//lang/Object",
        "",
    ] {
        let (class, _) = fixture(target, false)?;
        assert_eq!(
            Compiler::request_rejection_reason(&class, method(&class)?),
            Some("experimental_unqualified_instruction")
        );
        assert!(matches!(
            Compiler::new()?.compile_requests(&class, method(&class)?, &[]),
            Err(crate::Error::UnsupportedMethod(_))
        ));
    }
    let (mut class, _) = fixture("ClosedNode", false)?;
    let wrong_kind = class.constant_pool.add_utf8("not a Class")?;
    for index in [0, wrong_kind, u16::MAX] {
        *code_mut(&mut class)?
            .get_mut(3)
            .ok_or_else(|| InternalError("cast index".to_owned()))? = Instruction::Checkcast(index);
        assert_eq!(
            Compiler::request_rejection_reason(&class, method(&class)?),
            Some("experimental_unqualified_instruction")
        );
        assert!(matches!(
            Compiler::new()?.compile_requests(&class, method(&class)?, &[]),
            Err(crate::Error::UnsupportedMethod(_))
        ));
    }
    let mut class = primitive_arrays::fixture(
        "(Ljava/lang/Object;)I",
        vec![Instruction::Iconst_0, Instruction::Ireturn],
    )?;
    let index = class.constant_pool.add_class("java/lang/Object")?;
    *code_mut(&mut class)? = vec![
        Instruction::Aload_0,
        Instruction::Instanceof(index),
        Instruction::Ireturn,
    ];
    class.verify()?;
    assert_eq!(
        Compiler::request_rejection_reason(&class, method(&class)?),
        Some("experimental_unqualified_instruction")
    );
    assert!(matches!(
        Compiler::new()?.compile_requests(&class, method(&class)?, &[]),
        Err(crate::Error::UnsupportedMethod(_))
    ));
    Ok(())
}
