//! Request-only calls have no synchronous invoke-helper imports.
use ristretto_classfile::attributes::{Attribute, Instruction, MaxLocals, MaxStack};
use ristretto_classfile::{ClassFile, MethodAccessFlags};
use ristretto_jit::{CallKind, CallRequest, Compiler, ExecutionStep, Function, Result, Value};
use std::num::NonZeroU32;

extern "C" fn publish(_context: *const u8, _roots: *const i64, _count: i32) -> i32 {
    1
}
fn pointer() -> *const u8 {
    std::ptr::from_ref(&0u8)
}

fn fixture(
    descriptor: &str,
    target: &str,
    kind: CallKind,
    instance: bool,
    mut code: Vec<Instruction>,
) -> Result<(ClassFile<'static>, u16)> {
    let initial = match descriptor.split(')').next_back().unwrap_or("") {
        "V" => vec![Instruction::Return],
        "J" => vec![Instruction::Lconst_0, Instruction::Lreturn],
        "D" => vec![Instruction::Dconst_0, Instruction::Dreturn],
        "F" => vec![Instruction::Fconst_0, Instruction::Freturn],
        result if result.starts_with('L') || result.starts_with('[') => {
            vec![Instruction::Aconst_null, Instruction::Areturn]
        }
        _ => vec![Instruction::Iconst_0, Instruction::Ireturn],
    };
    let mut class = super::continuation::fixture(descriptor, &initial)?;
    if instance {
        class.methods[0]
            .access_flags
            .remove(MethodAccessFlags::STATIC);
    }
    let owner = class.constant_pool.add_class("Callee")?;
    let index = if kind == CallKind::Interface {
        class
            .constant_pool
            .add_interface_method_ref(owner, "call", target)?
    } else {
        class.constant_pool.add_method_ref(owner, "call", target)?
    };
    for instruction in &mut code {
        *instruction = match instruction {
            Instruction::Invokestatic(0) => Instruction::Invokestatic(index),
            Instruction::Invokespecial(0) => Instruction::Invokespecial(index),
            Instruction::Invokevirtual(0) => Instruction::Invokevirtual(index),
            Instruction::Invokeinterface(0, count) => Instruction::Invokeinterface(index, *count),
            _ => instruction.clone(),
        };
    }
    let stack = code.max_stack(&class.constant_pool)?;
    let locals = code.max_locals(&class.constant_pool, &class.methods[0])?;
    if let Attribute::Code {
        code: body,
        max_stack,
        max_locals,
        ..
    } = &mut class.methods[0].attributes[0]
    {
        *body = code;
        *max_stack = stack;
        *max_locals = locals;
    }
    class.verify()?;
    Ok((class, index))
}
fn compile(class: &ClassFile) -> Result<Function> {
    Compiler::new()?.compile_requests(
        class,
        &class.methods[0],
        &[("jit_publish_frame_roots", publish as *const u8)],
    )
}

#[test]
fn wide_static_arguments_exact_bci_and_completion_protocol() -> Result<()> {
    let (class, index) = fixture(
        "(JI)J",
        "(JI)J",
        CallKind::Static,
        false,
        vec![
            Instruction::Lload_0,
            Instruction::Iload_2,
            Instruction::Invokestatic(0),
            Instruction::Lreturn,
        ],
    )?;
    assert!(!Compiler::can_compile(&class.methods[0]));
    let function = compile(&class)?;
    let mut call = function.activation(&[Value::I64(i64::MIN), Value::I32(-7)])?;
    assert!(call.call_arguments().is_err());
    assert!(call.complete_call(0).is_err());
    assert_eq!(
        call.resume_with_context(NonZeroU32::new(3).expect("positive"), pointer())?,
        ExecutionStep::Call(CallRequest {
            kind: CallKind::Static,
            bci: 2,
            index
        })
    );
    assert_eq!(call.call_arguments()?, [i64::MIN, -7]);
    assert_eq!(call.suspended_bci(), Some(2));
    assert_eq!(call.remaining_budget()?.map(NonZeroU32::get), Some(2));
    assert!(
        call.resume_with_context(NonZeroU32::MIN, pointer())
            .is_err()
    );
    assert_eq!(call.call_arguments()?, [i64::MIN, -7]);
    call.complete_call(i64::MAX)?;
    assert!(call.complete_call(0).is_err());
    assert_eq!(
        call.resume_with_context(NonZeroU32::MIN, pointer())?,
        ExecutionStep::Returned(Some(Value::I64(i64::MAX)))
    );
    Ok(())
}

#[test]
fn receiver_forms_and_instance_entry_preserve_reference_tokens() -> Result<()> {
    for (kind, instruction) in [
        (CallKind::Special, Instruction::Invokespecial(0)),
        (CallKind::Virtual, Instruction::Invokevirtual(0)),
        (CallKind::Interface, Instruction::Invokeinterface(0, 1)),
    ] {
        let (class, index) = fixture(
            "()Ljava/lang/Object;",
            "()Ljava/lang/Object;",
            kind,
            true,
            vec![Instruction::Aload_0, instruction, Instruction::Areturn],
        )?;
        let function = compile(&class)?;
        let mut call = function.activation(&[Value::Ptr(23)])?;
        assert_eq!(
            call.resume_with_context(NonZeroU32::MIN, pointer())?,
            ExecutionStep::Call(CallRequest {
                kind,
                bci: 1,
                index
            })
        );
        assert_eq!(call.call_arguments()?, [23]);
        assert_eq!(call.remaining_budget()?, None);
        call.complete_call(27)?;
        assert_eq!(
            call.resume_with_context(NonZeroU32::MIN, pointer())?,
            ExecutionStep::Returned(Some(Value::Ptr(27)))
        );
    }
    Ok(())
}

#[test]
fn double_and_float_argument_bits_are_not_numeric_conversions() -> Result<()> {
    let (class, _) = fixture(
        "(DF)D",
        "(DF)D",
        CallKind::Static,
        false,
        vec![
            Instruction::Dload_0,
            Instruction::Fload_2,
            Instruction::Invokestatic(0),
            Instruction::Dreturn,
        ],
    )?;
    let function = compile(&class)?;
    for double in [-0.0, f64::from_bits(0x7ff8_0123_4567_89ab)] {
        let float = f32::from_bits(0xffc0_1234);
        let mut call = function.activation(&[Value::F64(double), Value::F32(float)])?;
        assert!(matches!(
            call.resume_with_context(NonZeroU32::MIN, pointer())?,
            ExecutionStep::Call(_)
        ));
        assert_eq!(
            call.call_arguments()?,
            [
                i64::from_ne_bytes(double.to_bits().to_ne_bytes()),
                i64::from(float.to_bits())
            ]
        );
        call.complete_call(i64::from_ne_bytes(double.to_bits().to_ne_bytes()))?;
        let ExecutionStep::Returned(Some(Value::F64(result))) =
            call.resume_with_context(NonZeroU32::MIN, pointer())?
        else {
            panic!("double result");
        };
        assert_eq!(result.to_bits(), double.to_bits());
    }
    Ok(())
}

#[test]
fn live_operand_and_locals_resume_without_replaying_a_call() -> Result<()> {
    let (class, index) = fixture(
        "(I)I",
        "(I)I",
        CallKind::Static,
        false,
        vec![
            Instruction::Bipush(9),
            Instruction::Iload_0,
            Instruction::Invokestatic(0),
            Instruction::Iadd,
            Instruction::Iload_0,
            Instruction::Iadd,
            Instruction::Ireturn,
        ],
    )?;
    let function = compile(&class)?;
    let mut call = function.activation(&[Value::I32(4)])?;
    assert_eq!(
        call.resume_with_context(NonZeroU32::MIN, pointer())?,
        ExecutionStep::Call(CallRequest {
            kind: CallKind::Static,
            bci: 3,
            index
        })
    );
    call.complete_call(20)?;
    assert_eq!(
        call.resume_with_context(NonZeroU32::MIN, pointer())?,
        ExecutionStep::Returned(Some(Value::I32(33)))
    );
    Ok(())
}

#[test]
fn pending_calls_can_be_cancelled_and_interface_slot_counts_are_checked() -> Result<()> {
    let (class, _) = fixture(
        "(JI)J",
        "(JI)J",
        CallKind::Static,
        false,
        vec![
            Instruction::Lload_0,
            Instruction::Iload_2,
            Instruction::Invokestatic(0),
            Instruction::Lreturn,
        ],
    )?;
    let function = compile(&class)?;
    let mut call = function.activation(&[Value::I64(1), Value::I32(2)])?;
    assert!(matches!(
        call.resume_with_context(NonZeroU32::MIN, pointer())?,
        ExecutionStep::Call(_)
    ));
    drop(call); // a host error/cancellation never resumes placeholder results
    let (mut class, _) = fixture(
        "(J)V",
        "(J)V",
        CallKind::Interface,
        true,
        vec![
            Instruction::Aload_0,
            Instruction::Lload_1,
            Instruction::Invokeinterface(0, 3),
            Instruction::Return,
        ],
    )?;
    assert!(compile(&class).is_ok());
    if let Attribute::Code { code, .. } = &mut class.methods[0].attributes[0] {
        let Instruction::Invokeinterface(_, count) = &mut code[2] else {
            panic!("invokeinterface");
        };
        *count = 2;
    }
    assert!(compile(&class).is_err());
    Ok(())
}

#[test]
fn zero_argument_void_calls_share_fuel_without_replaying_prior_calls() -> Result<()> {
    let (class, index) = fixture(
        "()V",
        "()V",
        CallKind::Static,
        false,
        vec![
            Instruction::Invokestatic(0),
            Instruction::Invokestatic(0),
            Instruction::Return,
        ],
    )?;
    let function = compile(&class)?;
    let mut call = function.activation(&[])?;
    for (bci, budget) in [(0, 2), (3, 1)] {
        assert_eq!(
            call.resume_with_context(NonZeroU32::new(budget).expect("positive"), pointer())?,
            ExecutionStep::Call(CallRequest {
                kind: CallKind::Static,
                bci,
                index
            })
        );
        assert_eq!(call.call_arguments()?, []);
        assert_eq!(
            call.remaining_budget()?.map(NonZeroU32::get),
            NonZeroU32::new(budget - 1).map(NonZeroU32::get)
        );
        call.complete_call(0)?;
    }
    assert_eq!(
        call.resume_with_context(NonZeroU32::MIN, pointer())?,
        ExecutionStep::Returned(None)
    );
    Ok(())
}

#[test]
fn unreachable_call_restore_target_is_never_executed() -> Result<()> {
    let (class, _) = fixture(
        "()V",
        "()V",
        CallKind::Static,
        false,
        vec![
            Instruction::Goto(6),
            Instruction::Invokestatic(0),
            Instruction::Return,
        ],
    )?;
    let function = compile(&class)?;
    let mut call = function.activation(&[])?;
    assert_eq!(
        call.resume_with_context(NonZeroU32::MIN, pointer())?,
        ExecutionStep::Returned(None)
    );
    Ok(())
}
