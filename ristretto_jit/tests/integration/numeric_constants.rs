use ristretto_classfile::Constant;
use ristretto_classfile::attributes::{Attribute, Instruction};
use ristretto_jit::{Compiler, ExecutionStep, Result, Value};
use std::num::NonZeroU32;

#[test]
fn numeric_ldc_and_ldc_w_preserve_integer_and_float_bits() -> Result<()> {
    for wide in [false, true] {
        for constant in [
            Constant::Integer(-1_000_000),
            Constant::Float(-0.0),
            Constant::Float(f32::from_bits(0x7fc0_1234)),
        ] {
            let (descriptor, initial, ret) = match &constant {
                Constant::Integer(_) => ("()I", Instruction::Iconst_0, Instruction::Ireturn),
                _ => ("()F", Instruction::Fconst_0, Instruction::Freturn),
            };
            let mut class = super::continuation::fixture(descriptor, &[initial, ret.clone()])?;
            if wide {
                for index in 0..260 {
                    class.constant_pool.add_utf8(format!("padding{index}"))?;
                }
            }
            let index = class.constant_pool.add(constant.clone())?;
            let load = if wide {
                assert!(index > 255);
                Instruction::Ldc_w(index)
            } else {
                Instruction::Ldc(u8::try_from(index)?)
            };
            if let Attribute::Code { code, .. } = &mut class.methods[0].attributes[0] {
                *code = vec![load, ret];
            }
            class.verify()?;
            assert!(!Compiler::can_compile(&class.methods[0]));
            let function = Compiler::new()?.compile_requests(&class, &class.methods[0], &[])?;
            let result = function.activation(&[])?.resume(NonZeroU32::MIN)?;
            match (result, constant) {
                (
                    ExecutionStep::Returned(Some(Value::I32(actual))),
                    Constant::Integer(expected),
                ) => assert_eq!(actual, expected),
                (ExecutionStep::Returned(Some(Value::F32(actual))), Constant::Float(expected)) => {
                    assert_eq!(actual.to_bits(), expected.to_bits());
                }
                _ => panic!("constant result kind"),
            }
        }
    }
    Ok(())
}

#[test]
fn class_and_string_constants_remain_rejected() -> Result<()> {
    for string in [false, true] {
        let mut class = super::continuation::fixture(
            "()Ljava/lang/Object;",
            &[Instruction::Aconst_null, Instruction::Areturn],
        )?;
        let index = if string {
            class.constant_pool.add_string("value")?
        } else {
            class.constant_pool.add_class("java/lang/Object")?
        };
        if let Attribute::Code { code, .. } = &mut class.methods[0].attributes[0] {
            *code = vec![Instruction::Ldc_w(index), Instruction::Areturn];
        }
        assert_eq!(
            Compiler::request_rejection_reason(&class, &class.methods[0]),
            Some("experimental_unqualified_instruction")
        );
    }
    Ok(())
}
