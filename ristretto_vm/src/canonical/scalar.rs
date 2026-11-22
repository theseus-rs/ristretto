//! Primitive JVM values used by the closed canonical execution tier.
use super::{Error, Result, Value, load, pop, store};
use ristretto_classfile::{BaseType, Constant, ConstantPool, FieldType, attributes::Instruction};

/// JVMS ireturn narrows before handing the result to the caller, even when the
/// verified method's bytecode did not explicitly perform a narrowing conversion.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "JVM ireturn narrowing for byte, char, short and boolean descriptors"
)]
pub(super) fn normalize_return(value: Value, field: &FieldType) -> Value {
    match (value, field) {
        (Value::Int(value), FieldType::Base(kind)) => Value::Int(match kind {
            BaseType::Byte => i32::from(value as i8),
            BaseType::Char => i32::from(value as u16),
            BaseType::Short => i32::from(value as i16),
            BaseType::Boolean => value & 1,
            _ => value,
        }),
        (value, _) => value,
    }
}

pub(super) fn constant(pool: &ConstantPool<'_>, index: u16, wide: bool) -> Result<Value> {
    match (pool.try_get(index)?, wide) {
        (Constant::Integer(value), false) => Ok(Value::Int(*value)),
        (Constant::Float(value), false) => Ok(Value::Float(*value)),
        (Constant::Long(value), true) => Ok(Value::Long(*value)),
        (Constant::Double(value), true) => Ok(Value::Double(*value)),
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
pub(super) fn execute(
    instruction: &Instruction,
    locals: &mut [Option<Value>],
    stack: &mut Vec<Value>,
) -> Result<()> {
    let value = match instruction {
        Instruction::Lconst_0 => Value::Long(0),
        Instruction::Lconst_1 => Value::Long(1),
        Instruction::Fconst_0 => Value::Float(0.0),
        Instruction::Fconst_1 => Value::Float(1.0),
        Instruction::Fconst_2 => Value::Float(2.0),
        Instruction::Dconst_0 => Value::Double(0.0),
        Instruction::Dconst_1 => Value::Double(1.0),
        Instruction::Lload(index) | Instruction::Fload(index) | Instruction::Dload(index) => {
            return load(locals, stack, usize::from(*index));
        }
        Instruction::Lload_w(index) | Instruction::Fload_w(index) | Instruction::Dload_w(index) => {
            return load(locals, stack, usize::from(*index));
        }
        Instruction::Lload_0 | Instruction::Fload_0 | Instruction::Dload_0 => {
            return load(locals, stack, 0);
        }
        Instruction::Lload_1 | Instruction::Fload_1 | Instruction::Dload_1 => {
            return load(locals, stack, 1);
        }
        Instruction::Lload_2 | Instruction::Fload_2 | Instruction::Dload_2 => {
            return load(locals, stack, 2);
        }
        Instruction::Lload_3 | Instruction::Fload_3 | Instruction::Dload_3 => {
            return load(locals, stack, 3);
        }
        Instruction::Lstore(index) | Instruction::Fstore(index) | Instruction::Dstore(index) => {
            return store(locals, stack, usize::from(*index));
        }
        Instruction::Lstore_w(index)
        | Instruction::Fstore_w(index)
        | Instruction::Dstore_w(index) => {
            return store(locals, stack, usize::from(*index));
        }
        Instruction::Lstore_0 | Instruction::Fstore_0 | Instruction::Dstore_0 => {
            return store(locals, stack, 0);
        }
        Instruction::Lstore_1 | Instruction::Fstore_1 | Instruction::Dstore_1 => {
            return store(locals, stack, 1);
        }
        Instruction::Lstore_2 | Instruction::Fstore_2 | Instruction::Dstore_2 => {
            return store(locals, stack, 2);
        }
        Instruction::Lstore_3 | Instruction::Fstore_3 | Instruction::Dstore_3 => {
            return store(locals, stack, 3);
        }
        Instruction::Ladd | Instruction::Lsub | Instruction::Lmul => {
            let right = pop(stack)?.long()?;
            let left = pop(stack)?.long()?;
            Value::Long(match instruction {
                Instruction::Ladd => left.wrapping_add(right),
                Instruction::Lsub => left.wrapping_sub(right),
                _ => left.wrapping_mul(right),
            })
        }
        Instruction::Fadd | Instruction::Fsub | Instruction::Fmul => {
            let right = pop(stack)?.float()?;
            let left = pop(stack)?.float()?;
            Value::Float(match instruction {
                Instruction::Fadd => left + right,
                Instruction::Fsub => left - right,
                _ => left * right,
            })
        }
        Instruction::Dadd | Instruction::Dsub | Instruction::Dmul => {
            let right = pop(stack)?.double()?;
            let left = pop(stack)?.double()?;
            Value::Double(match instruction {
                Instruction::Dadd => left + right,
                Instruction::Dsub => left - right,
                _ => left * right,
            })
        }
        Instruction::Lneg => Value::Long(pop(stack)?.long()?.wrapping_neg()),
        Instruction::Fneg => Value::Float(-pop(stack)?.float()?),
        Instruction::Dneg => Value::Double(-pop(stack)?.double()?),
        Instruction::Lcmp => {
            let right = pop(stack)?.long()?;
            let left = pop(stack)?.long()?;
            Value::Int(ordering(left.cmp(&right)))
        }
        Instruction::Fcmpl | Instruction::Fcmpg => {
            let right = pop(stack)?.float()?;
            let left = pop(stack)?.float()?;
            Value::Int(left.partial_cmp(&right).map_or(
                if matches!(instruction, Instruction::Fcmpl) {
                    -1
                } else {
                    1
                },
                ordering,
            ))
        }
        Instruction::Dcmpl | Instruction::Dcmpg => {
            let right = pop(stack)?.double()?;
            let left = pop(stack)?.double()?;
            Value::Int(left.partial_cmp(&right).map_or(
                if matches!(instruction, Instruction::Dcmpl) {
                    -1
                } else {
                    1
                },
                ordering,
            ))
        }
        Instruction::I2b => Value::Int(i32::from(pop(stack)?.int()? as i8)),
        Instruction::I2c => Value::Int(i32::from(pop(stack)?.int()? as u16)),
        Instruction::I2s => Value::Int(i32::from(pop(stack)?.int()? as i16)),
        Instruction::I2l => Value::Long(i64::from(pop(stack)?.int()?)),
        Instruction::I2f => Value::Float(pop(stack)?.int()? as f32),
        Instruction::I2d => Value::Double(f64::from(pop(stack)?.int()?)),
        Instruction::L2i => Value::Int(pop(stack)?.long()? as i32),
        Instruction::L2f => Value::Float(pop(stack)?.long()? as f32),
        Instruction::L2d => Value::Double(pop(stack)?.long()? as f64),
        Instruction::F2i => Value::Int(pop(stack)?.float()? as i32),
        Instruction::F2l => Value::Long(pop(stack)?.float()? as i64),
        Instruction::F2d => Value::Double(f64::from(pop(stack)?.float()?)),
        Instruction::D2i => Value::Int(pop(stack)?.double()? as i32),
        Instruction::D2l => Value::Long(pop(stack)?.double()? as i64),
        Instruction::D2f => Value::Float(pop(stack)?.double()? as f32),
        _ => return Err(Error::Invalid("unsupported scalar opcode".into())),
    };
    stack.push(value);
    Ok(())
}
fn ordering(value: std::cmp::Ordering) -> i32 {
    match value {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}
