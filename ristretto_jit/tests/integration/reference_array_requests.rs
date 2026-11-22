//! Reference-array request cells; ownership/type validation stays with the embedding.
use ristretto_classfile::attributes::{Attribute, Instruction, MaxStack};
use ristretto_jit::{
    AllocationKind, AllocationRequest, ArrayAccessKind, ArrayAccessRequest, ArrayStoreRequest,
    Compiler, ExecutionStep, Function, Result, Value,
};
use std::num::NonZeroU32;

extern "C" fn publish(_context: *const u8, _roots: *const i64, _count: i32) -> i32 {
    1
}

fn pointer() -> *const u8 {
    std::ptr::from_ref(&0u8)
}

fn allocation(component: &str) -> Result<(Function, u16)> {
    let mut class = super::continuation::fixture(
        "(I)[Ljava/lang/Object;",
        &[Instruction::Aconst_null, Instruction::Areturn],
    )?;
    let component = class.constant_pool.add_class(component)?;
    let code = vec![
        Instruction::Iload_0,
        Instruction::Anewarray(component),
        Instruction::Areturn,
    ];
    let stack = code.max_stack(&class.constant_pool)?;
    if let Attribute::Code {
        code: body,
        max_stack,
        ..
    } = &mut class.methods[0].attributes[0]
    {
        *body = code;
        *max_stack = stack;
    }
    Ok((
        Compiler::new()?.compile_requests(
            &class,
            &class.methods[0],
            &[("jit_publish_frame_roots", publish as *const u8)],
        )?,
        component,
    ))
}

#[test]
fn reference_allocation_carries_component_length_and_result_token() -> Result<()> {
    let (function, index) = allocation("java/lang/Object")?;
    for length in [-1, 0, 2048] {
        let mut activation = function.activation(&[Value::I32(length)])?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, pointer())?,
            ExecutionStep::Allocation(AllocationRequest {
                kind: AllocationKind::ReferenceArray(index),
                bci: 1
            })
        );
        assert_eq!(activation.call_arguments()?, [i64::from(length)]);
        if length < 0 {
            drop(activation);
            continue;
        }
        activation.complete_call(51)?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, pointer())?,
            ExecutionStep::Returned(Some(Value::Ptr(51)))
        );
    }
    assert!(allocation("[Ljava/lang/Object;").is_err());
    Ok(())
}

#[test]
fn reference_store_and_load_keep_array_index_and_value_tokens() -> Result<()> {
    let class = super::continuation::fixture(
        "([Ljava/lang/Object;ILjava/lang/Object;)Ljava/lang/Object;",
        &[
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
    let function = Compiler::new()?.compile_requests(
        &class,
        &class.methods[0],
        &[("jit_publish_frame_roots", publish as *const u8)],
    )?;
    for (array, index, value) in [(71, 1, 83), (71, -1, 0), (0, 0, 83)] {
        let mut activation =
            function.activation(&[Value::Ptr(array), Value::I32(index), Value::Ptr(value)])?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, pointer())?,
            ExecutionStep::ArrayStore(ArrayStoreRequest { bci: 3 })
        );
        assert_eq!(
            activation.call_arguments()?,
            [array, i64::from(index), value]
        );
        if array == 0 || index < 0 {
            drop(activation);
            continue;
        }
        activation.complete_call(0)?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, pointer())?,
            ExecutionStep::ArrayAccess(ArrayAccessRequest {
                kind: ArrayAccessKind::ReferenceLoad,
                bci: 6
            })
        );
        assert_eq!(activation.call_arguments()?, [array, i64::from(index)]);
        activation.complete_call(value)?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, pointer())?,
            ExecutionStep::Returned(Some(Value::Ptr(value)))
        );
    }
    Ok(())
}
