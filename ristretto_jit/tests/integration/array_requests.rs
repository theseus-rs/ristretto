//! Raw request cells; the canonical consumer owns array validation and storage semantics.
use ristretto_classfile::attributes::{ArrayType, Instruction};
use ristretto_jit::{
    AllocationKind, AllocationRequest, ArrayAccessKind, ArrayAccessRequest, Compiler,
    ExecutionStep, Function, Result, Value,
};
use std::num::NonZeroU32;

extern "C" fn publish(_context: *const u8, _roots: *const i64, _count: i32) -> i32 {
    1
}

fn pointer() -> *const u8 {
    std::ptr::from_ref(&0u8)
}

fn compile(descriptor: &str, code: &[Instruction]) -> Result<Function> {
    let class = super::continuation::fixture(descriptor, code)?;
    Compiler::new()?.compile_requests(
        &class,
        &class.methods[0],
        &[("jit_publish_frame_roots", publish as *const u8)],
    )
}

#[test]
fn all_primitive_allocations_keep_signed_length_and_owned_result_protocol() -> Result<()> {
    for (kind, descriptor) in [
        (ArrayType::Boolean, "[Z"),
        (ArrayType::Byte, "[B"),
        (ArrayType::Char, "[C"),
        (ArrayType::Short, "[S"),
        (ArrayType::Int, "[I"),
        (ArrayType::Long, "[J"),
        (ArrayType::Float, "[F"),
        (ArrayType::Double, "[D"),
    ] {
        let function = compile(
            &format!("(I){descriptor}"),
            &[
                Instruction::Iload_0,
                Instruction::Newarray(kind.clone()),
                Instruction::Areturn,
            ],
        )?;
        for length in [-1, 0, 1] {
            let mut activation = function.activation(&[Value::I32(length)])?;
            assert_eq!(
                activation.resume_with_context(NonZeroU32::MIN, pointer())?,
                ExecutionStep::Allocation(AllocationRequest {
                    kind: AllocationKind::PrimitiveArray(kind.clone()),
                    bci: 1
                })
            );
            assert_eq!(activation.call_arguments()?, [i64::from(length)]);
            if length < 0 {
                drop(activation);
                continue;
            } // host rejects negative size
            activation.complete_call(37)?;
            assert_eq!(
                activation.resume_with_context(NonZeroU32::MIN, pointer())?,
                ExecutionStep::Returned(Some(Value::Ptr(37)))
            );
        }
    }
    Ok(())
}

#[test]
#[expect(clippy::too_many_lines, reason = "explicit primitive request matrix")]
fn primitive_store_load_cells_preserve_types_bits_and_signed_index() -> Result<()> {
    for (descriptor, value, local, store, load, ret, store_kind, load_kind, bits) in [
        (
            "I",
            Value::I32(-1),
            Instruction::Iload_2,
            Instruction::Iastore,
            Instruction::Iaload,
            Instruction::Ireturn,
            ArrayAccessKind::IntStore,
            ArrayAccessKind::IntLoad,
            -1,
        ),
        (
            "J",
            Value::I64(i64::MIN),
            Instruction::Lload_2,
            Instruction::Lastore,
            Instruction::Laload,
            Instruction::Lreturn,
            ArrayAccessKind::LongStore,
            ArrayAccessKind::LongLoad,
            i64::MIN,
        ),
        (
            "F",
            Value::F32(f32::from_bits(0xffc0_1234)),
            Instruction::Fload_2,
            Instruction::Fastore,
            Instruction::Faload,
            Instruction::Freturn,
            ArrayAccessKind::FloatStore,
            ArrayAccessKind::FloatLoad,
            0xffc0_1234,
        ),
        (
            "D",
            Value::F64(-0.0),
            Instruction::Dload_2,
            Instruction::Dastore,
            Instruction::Daload,
            Instruction::Dreturn,
            ArrayAccessKind::DoubleStore,
            ArrayAccessKind::DoubleLoad,
            i64::MIN,
        ),
        (
            "I",
            Value::I32(257),
            Instruction::Iload_2,
            Instruction::Bastore,
            Instruction::Baload,
            Instruction::Ireturn,
            ArrayAccessKind::ByteStore,
            ArrayAccessKind::ByteLoad,
            257,
        ),
        (
            "I",
            Value::I32(-1),
            Instruction::Iload_2,
            Instruction::Castore,
            Instruction::Caload,
            Instruction::Ireturn,
            ArrayAccessKind::CharStore,
            ArrayAccessKind::CharLoad,
            -1,
        ),
        (
            "I",
            Value::I32(65535),
            Instruction::Iload_2,
            Instruction::Sastore,
            Instruction::Saload,
            Instruction::Ireturn,
            ArrayAccessKind::ShortStore,
            ArrayAccessKind::ShortLoad,
            65535,
        ),
    ] {
        let function = compile(
            &format!("(Ljava/lang/Object;I{descriptor}){descriptor}"),
            &[
                Instruction::Aload_0,
                Instruction::Iload_1,
                local,
                store,
                Instruction::Aload_0,
                Instruction::Iload_1,
                load,
                ret,
            ],
        )?;
        let mut activation =
            function.activation(&[Value::Ptr(43), Value::I32(-7), value.clone()])?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, pointer())?,
            ExecutionStep::ArrayAccess(ArrayAccessRequest {
                kind: store_kind,
                bci: 3
            })
        );
        assert_eq!(activation.call_arguments()?, [43, -7, bits]);
        activation.complete_call(0)?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, pointer())?,
            ExecutionStep::ArrayAccess(ArrayAccessRequest {
                kind: load_kind,
                bci: 6
            })
        );
        assert_eq!(activation.call_arguments()?, [43, -7]);
        activation.complete_call(bits)?;
        let ExecutionStep::Returned(Some(result)) =
            activation.resume_with_context(NonZeroU32::MIN, pointer())?
        else {
            panic!("load result");
        };
        match result {
            Value::F32(v) => assert_eq!(i64::from(v.to_bits()), bits),
            Value::F64(v) => assert_eq!(i64::from_ne_bytes(v.to_bits().to_ne_bytes()), bits),
            result => assert_eq!(result, value),
        }
    }
    Ok(())
}

#[test]
fn array_length_null_and_cancel_are_explicit_host_boundaries() -> Result<()> {
    let function = compile(
        "(Ljava/lang/Object;)I",
        &[
            Instruction::Aload_0,
            Instruction::Arraylength,
            Instruction::Ireturn,
        ],
    )?;
    for token in [0, 17] {
        let mut activation = function.activation(&[Value::Ptr(token)])?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, pointer())?,
            ExecutionStep::ArrayAccess(ArrayAccessRequest {
                kind: ArrayAccessKind::Length,
                bci: 1
            })
        );
        assert_eq!(activation.call_arguments()?, [token]);
        assert!(
            activation
                .resume_with_context(NonZeroU32::MIN, pointer())
                .is_err()
        );
        if token == 0 {
            drop(activation);
            continue;
        }
        activation.complete_call(i64::from(i32::MAX))?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, pointer())?,
            ExecutionStep::Returned(Some(Value::I32(i32::MAX)))
        );
    }
    Ok(())
}
