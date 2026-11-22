use ristretto_classfile::attributes::{Attribute, Instruction};
use ristretto_jit::{
    Compiler, ExecutionStep, Function, Result, TypeCheckKind, TypeCheckRequest, Value,
};
use std::num::NonZeroU32;

extern "C" fn publish(_context: *const u8, _roots: *const i64, _count: i32) -> i32 {
    1
}

fn pointer() -> *const u8 {
    std::ptr::from_ref(&0u8)
}

fn compile(target: &str) -> Result<(Function, u16)> {
    let mut class = super::continuation::fixture(
        "(Ljava/lang/Object;)Ljava/lang/Object;",
        &[Instruction::Aload_0, Instruction::Areturn],
    )?;
    let index = class.constant_pool.add_class(target)?;
    if let Attribute::Code { code, .. } = &mut class.methods[0].attributes[0] {
        *code = vec![
            Instruction::Aload_0,
            Instruction::Checkcast(index),
            Instruction::Areturn,
        ];
    }
    Ok((
        Compiler::new()?.compile_requests(
            &class,
            &class.methods[0],
            &[("jit_publish_frame_roots", publish as *const u8)],
        )?,
        index,
    ))
}

#[test]
fn checked_cast_preserves_identity_null_and_exact_target_request() -> Result<()> {
    for target in ["Node", "java/lang/Object", "[I", "[Ljava/lang/Object;"] {
        let (function, index) = compile(target)?;
        for token in [0, 71] {
            let mut activation = function.activation(&[Value::Ptr(token)])?;
            assert_eq!(
                activation.resume_with_context(NonZeroU32::MIN, pointer())?,
                ExecutionStep::TypeCheck(TypeCheckRequest {
                    kind: TypeCheckKind::CheckCast,
                    index,
                    bci: 1
                })
            );
            assert_eq!(activation.call_arguments()?, [token]);
            activation.complete_call(token)?;
            assert_eq!(
                activation.resume_with_context(NonZeroU32::MIN, pointer())?,
                ExecutionStep::Returned(Some(Value::Ptr(token)))
            );
        }
    }
    assert!(compile("[[I").is_err());
    Ok(())
}

#[test]
fn failed_cast_cancels_before_following_reference_code() -> Result<()> {
    let (function, _) = compile("Node")?;
    let mut activation = function.activation(&[Value::Ptr(83)])?;
    assert!(matches!(
        activation.resume_with_context(NonZeroU32::MIN, pointer())?,
        ExecutionStep::TypeCheck(_)
    ));
    assert!(
        activation
            .resume_with_context(NonZeroU32::MIN, pointer())
            .is_err()
    );
    drop(activation); // a host type fault never publishes a placeholder result
    Ok(())
}
