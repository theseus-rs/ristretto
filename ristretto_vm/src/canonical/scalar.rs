//! Primitive JVM values used by the closed canonical execution tier.
use super::value::PrimitiveFrameValue;
use super::{Error, ScalarValue};
use ristretto_classfile::{BaseType, Constant, ConstantPool, FieldType, attributes::Instruction};

/// JVM field stores and integer returns narrow the stack value to its declared type.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "JVM narrowing for byte, char, short and boolean descriptors"
)]
pub(super) fn narrow_integer(value: i32, kind: BaseType) -> i32 {
    match kind {
        BaseType::Byte => i32::from(value as i8),
        BaseType::Char => i32::from(value as u16),
        BaseType::Short => i32::from(value as i16),
        BaseType::Boolean => value & 1,
        _ => value,
    }
}

pub(super) fn normalize_return(value: ScalarValue, field: &FieldType) -> ScalarValue {
    match (value, field) {
        (ScalarValue::Int(value), FieldType::Base(kind)) => {
            ScalarValue::Int(narrow_integer(value, *kind))
        }
        (value, _) => value,
    }
}

pub(super) fn constant(
    pool: &ConstantPool<'_>,
    index: u16,
    wide: bool,
) -> Result<ScalarValue, Error> {
    match (pool.try_get(index)?, wide) {
        (Constant::Integer(value), false) => Ok(ScalarValue::Int(*value)),
        (Constant::Float(value), false) => Ok(ScalarValue::Float(*value)),
        (Constant::Long(value), true) => Ok(ScalarValue::Long(*value)),
        (Constant::Double(value), true) => Ok(ScalarValue::Double(*value)),
        _ => Err(Error::Unsupported(
            "canonical primitive constant".to_owned(),
        )),
    }
}

pub(super) fn supports(instruction: &Instruction) -> bool {
    matches!(
        instruction,
        Instruction::Lconst_0
            | Instruction::Lconst_1
            | Instruction::Fconst_0
            | Instruction::Fconst_1
            | Instruction::Fconst_2
            | Instruction::Dconst_0
            | Instruction::Dconst_1
            | Instruction::Lload(_)
            | Instruction::Fload(_)
            | Instruction::Dload(_)
            | Instruction::Lload_w(_)
            | Instruction::Fload_w(_)
            | Instruction::Dload_w(_)
            | Instruction::Lload_0
            | Instruction::Lload_1
            | Instruction::Lload_2
            | Instruction::Lload_3
            | Instruction::Fload_0
            | Instruction::Fload_1
            | Instruction::Fload_2
            | Instruction::Fload_3
            | Instruction::Dload_0
            | Instruction::Dload_1
            | Instruction::Dload_2
            | Instruction::Dload_3
            | Instruction::Lstore(_)
            | Instruction::Fstore(_)
            | Instruction::Dstore(_)
            | Instruction::Lstore_w(_)
            | Instruction::Fstore_w(_)
            | Instruction::Dstore_w(_)
            | Instruction::Lstore_0
            | Instruction::Lstore_1
            | Instruction::Lstore_2
            | Instruction::Lstore_3
            | Instruction::Fstore_0
            | Instruction::Fstore_1
            | Instruction::Fstore_2
            | Instruction::Fstore_3
            | Instruction::Dstore_0
            | Instruction::Dstore_1
            | Instruction::Dstore_2
            | Instruction::Dstore_3
            | Instruction::Ladd
            | Instruction::Lsub
            | Instruction::Lmul
            | Instruction::Lneg
            | Instruction::Fadd
            | Instruction::Fsub
            | Instruction::Fmul
            | Instruction::Fneg
            | Instruction::Dadd
            | Instruction::Dsub
            | Instruction::Dmul
            | Instruction::Dneg
            | Instruction::Lcmp
            | Instruction::Fcmpl
            | Instruction::Fcmpg
            | Instruction::Dcmpl
            | Instruction::Dcmpg
            | Instruction::I2b
            | Instruction::I2c
            | Instruction::I2s
            | Instruction::I2l
            | Instruction::I2f
            | Instruction::I2d
            | Instruction::L2i
            | Instruction::L2f
            | Instruction::L2d
            | Instruction::F2i
            | Instruction::F2l
            | Instruction::F2d
            | Instruction::D2i
            | Instruction::D2l
            | Instruction::D2f
    )
}

#[expect(
    clippy::too_many_lines,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    reason = "explicit JVM numeric opcode and narrowing semantics"
)]
pub(super) fn execute<V: PrimitiveFrameValue>(
    instruction: &Instruction,
    locals: &mut [V::Slot],
    stack: &mut Vec<V>,
) -> Result<(), Error> {
    let value = match instruction {
        Instruction::Lconst_0 => ScalarValue::Long(0),
        Instruction::Lconst_1 => ScalarValue::Long(1),
        Instruction::Fconst_0 => ScalarValue::Float(0.0),
        Instruction::Fconst_1 => ScalarValue::Float(1.0),
        Instruction::Fconst_2 => ScalarValue::Float(2.0),
        Instruction::Dconst_0 => ScalarValue::Double(0.0),
        Instruction::Dconst_1 => ScalarValue::Double(1.0),
        Instruction::Lload(index) | Instruction::Fload(index) | Instruction::Dload(index) => {
            return V::load(locals, stack, usize::from(*index));
        }
        Instruction::Lload_w(index) | Instruction::Fload_w(index) | Instruction::Dload_w(index) => {
            return V::load(locals, stack, usize::from(*index));
        }
        Instruction::Lload_0 | Instruction::Fload_0 | Instruction::Dload_0 => {
            return V::load(locals, stack, 0);
        }
        Instruction::Lload_1 | Instruction::Fload_1 | Instruction::Dload_1 => {
            return V::load(locals, stack, 1);
        }
        Instruction::Lload_2 | Instruction::Fload_2 | Instruction::Dload_2 => {
            return V::load(locals, stack, 2);
        }
        Instruction::Lload_3 | Instruction::Fload_3 | Instruction::Dload_3 => {
            return V::load(locals, stack, 3);
        }
        Instruction::Lstore(index) | Instruction::Fstore(index) | Instruction::Dstore(index) => {
            return V::store(locals, stack, usize::from(*index));
        }
        Instruction::Lstore_w(index)
        | Instruction::Fstore_w(index)
        | Instruction::Dstore_w(index) => {
            return V::store(locals, stack, usize::from(*index));
        }
        Instruction::Lstore_0 | Instruction::Fstore_0 | Instruction::Dstore_0 => {
            return V::store(locals, stack, 0);
        }
        Instruction::Lstore_1 | Instruction::Fstore_1 | Instruction::Dstore_1 => {
            return V::store(locals, stack, 1);
        }
        Instruction::Lstore_2 | Instruction::Fstore_2 | Instruction::Dstore_2 => {
            return V::store(locals, stack, 2);
        }
        Instruction::Lstore_3 | Instruction::Fstore_3 | Instruction::Dstore_3 => {
            return V::store(locals, stack, 3);
        }
        Instruction::Ladd | Instruction::Lsub | Instruction::Lmul => {
            let right = V::pop_scalar(stack)?.long()?;
            let left = V::pop_scalar(stack)?.long()?;
            ScalarValue::Long(match instruction {
                Instruction::Ladd => left.wrapping_add(right),
                Instruction::Lsub => left.wrapping_sub(right),
                _ => left.wrapping_mul(right),
            })
        }
        Instruction::Fadd | Instruction::Fsub | Instruction::Fmul => {
            let right = V::pop_scalar(stack)?.float()?;
            let left = V::pop_scalar(stack)?.float()?;
            ScalarValue::Float(match instruction {
                Instruction::Fadd => left + right,
                Instruction::Fsub => left - right,
                _ => left * right,
            })
        }
        Instruction::Dadd | Instruction::Dsub | Instruction::Dmul => {
            let right = V::pop_scalar(stack)?.double()?;
            let left = V::pop_scalar(stack)?.double()?;
            ScalarValue::Double(match instruction {
                Instruction::Dadd => left + right,
                Instruction::Dsub => left - right,
                _ => left * right,
            })
        }
        Instruction::Lneg => ScalarValue::Long(V::pop_scalar(stack)?.long()?.wrapping_neg()),
        Instruction::Fneg => ScalarValue::Float(-V::pop_scalar(stack)?.float()?),
        Instruction::Dneg => ScalarValue::Double(-V::pop_scalar(stack)?.double()?),
        Instruction::Lcmp => {
            let right = V::pop_scalar(stack)?.long()?;
            let left = V::pop_scalar(stack)?.long()?;
            ScalarValue::Int(ordering(left.cmp(&right)))
        }
        Instruction::Fcmpl | Instruction::Fcmpg => {
            let right = V::pop_scalar(stack)?.float()?;
            let left = V::pop_scalar(stack)?.float()?;
            ScalarValue::Int(left.partial_cmp(&right).map_or(
                if matches!(instruction, Instruction::Fcmpl) {
                    -1
                } else {
                    1
                },
                ordering,
            ))
        }
        Instruction::Dcmpl | Instruction::Dcmpg => {
            let right = V::pop_scalar(stack)?.double()?;
            let left = V::pop_scalar(stack)?.double()?;
            ScalarValue::Int(left.partial_cmp(&right).map_or(
                if matches!(instruction, Instruction::Dcmpl) {
                    -1
                } else {
                    1
                },
                ordering,
            ))
        }
        Instruction::I2b => ScalarValue::Int(i32::from(V::pop_scalar(stack)?.int()? as i8)),
        Instruction::I2c => ScalarValue::Int(i32::from(V::pop_scalar(stack)?.int()? as u16)),
        Instruction::I2s => ScalarValue::Int(i32::from(V::pop_scalar(stack)?.int()? as i16)),
        Instruction::I2l => ScalarValue::Long(i64::from(V::pop_scalar(stack)?.int()?)),
        Instruction::I2f => ScalarValue::Float(V::pop_scalar(stack)?.int()? as f32),
        Instruction::I2d => ScalarValue::Double(f64::from(V::pop_scalar(stack)?.int()?)),
        Instruction::L2i => ScalarValue::Int(V::pop_scalar(stack)?.long()? as i32),
        Instruction::L2f => ScalarValue::Float(V::pop_scalar(stack)?.long()? as f32),
        Instruction::L2d => ScalarValue::Double(V::pop_scalar(stack)?.long()? as f64),
        Instruction::F2i => ScalarValue::Int(V::pop_scalar(stack)?.float()? as i32),
        Instruction::F2l => ScalarValue::Long(V::pop_scalar(stack)?.float()? as i64),
        Instruction::F2d => ScalarValue::Double(f64::from(V::pop_scalar(stack)?.float()?)),
        Instruction::D2i => ScalarValue::Int(V::pop_scalar(stack)?.double()? as i32),
        Instruction::D2l => ScalarValue::Long(V::pop_scalar(stack)?.double()? as i64),
        Instruction::D2f => ScalarValue::Float(V::pop_scalar(stack)?.double()? as f32),
        _ => return Err(Error::Invalid("unsupported scalar opcode".into())),
    };
    stack.push(value.into());
    Ok(())
}
fn ordering(value: std::cmp::Ordering) -> i32 {
    match value {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}
