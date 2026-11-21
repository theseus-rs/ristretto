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
    use Instruction as I;
    matches!(
        instruction,
        I::Lconst_0
            | I::Lconst_1
            | I::Fconst_0
            | I::Fconst_1
            | I::Fconst_2
            | I::Dconst_0
            | I::Dconst_1
            | I::Lload(_)
            | I::Fload(_)
            | I::Dload(_)
            | I::Lload_w(_)
            | I::Fload_w(_)
            | I::Dload_w(_)
            | I::Lload_0
            | I::Lload_1
            | I::Lload_2
            | I::Lload_3
            | I::Fload_0
            | I::Fload_1
            | I::Fload_2
            | I::Fload_3
            | I::Dload_0
            | I::Dload_1
            | I::Dload_2
            | I::Dload_3
            | I::Lstore(_)
            | I::Fstore(_)
            | I::Dstore(_)
            | I::Lstore_w(_)
            | I::Fstore_w(_)
            | I::Dstore_w(_)
            | I::Lstore_0
            | I::Lstore_1
            | I::Lstore_2
            | I::Lstore_3
            | I::Fstore_0
            | I::Fstore_1
            | I::Fstore_2
            | I::Fstore_3
            | I::Dstore_0
            | I::Dstore_1
            | I::Dstore_2
            | I::Dstore_3
            | I::Ladd
            | I::Lsub
            | I::Lmul
            | I::Lneg
            | I::Fadd
            | I::Fsub
            | I::Fmul
            | I::Fneg
            | I::Dadd
            | I::Dsub
            | I::Dmul
            | I::Dneg
            | I::Lcmp
            | I::Fcmpl
            | I::Fcmpg
            | I::Dcmpl
            | I::Dcmpg
            | I::I2b
            | I::I2c
            | I::I2s
            | I::I2l
            | I::I2f
            | I::I2d
            | I::L2i
            | I::L2f
            | I::L2d
            | I::F2i
            | I::F2l
            | I::F2d
            | I::D2i
            | I::D2l
            | I::D2f
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
    use Instruction as I;
    let value = match instruction {
        I::Lconst_0 => Value::Long(0),
        I::Lconst_1 => Value::Long(1),
        I::Fconst_0 => Value::Float(0.0),
        I::Fconst_1 => Value::Float(1.0),
        I::Fconst_2 => Value::Float(2.0),
        I::Dconst_0 => Value::Double(0.0),
        I::Dconst_1 => Value::Double(1.0),
        I::Lload(index) | I::Fload(index) | I::Dload(index) => {
            return load(locals, stack, usize::from(*index));
        }
        I::Lload_w(index) | I::Fload_w(index) | I::Dload_w(index) => {
            return load(locals, stack, usize::from(*index));
        }
        I::Lload_0 | I::Fload_0 | I::Dload_0 => return load(locals, stack, 0),
        I::Lload_1 | I::Fload_1 | I::Dload_1 => return load(locals, stack, 1),
        I::Lload_2 | I::Fload_2 | I::Dload_2 => return load(locals, stack, 2),
        I::Lload_3 | I::Fload_3 | I::Dload_3 => return load(locals, stack, 3),
        I::Lstore(index) | I::Fstore(index) | I::Dstore(index) => {
            return store(locals, stack, usize::from(*index));
        }
        I::Lstore_w(index) | I::Fstore_w(index) | I::Dstore_w(index) => {
            return store(locals, stack, usize::from(*index));
        }
        I::Lstore_0 | I::Fstore_0 | I::Dstore_0 => return store(locals, stack, 0),
        I::Lstore_1 | I::Fstore_1 | I::Dstore_1 => return store(locals, stack, 1),
        I::Lstore_2 | I::Fstore_2 | I::Dstore_2 => return store(locals, stack, 2),
        I::Lstore_3 | I::Fstore_3 | I::Dstore_3 => return store(locals, stack, 3),
        I::Ladd | I::Lsub | I::Lmul => {
            let right = pop(stack)?.long()?;
            let left = pop(stack)?.long()?;
            Value::Long(match instruction {
                I::Ladd => left.wrapping_add(right),
                I::Lsub => left.wrapping_sub(right),
                _ => left.wrapping_mul(right),
            })
        }
        I::Fadd | I::Fsub | I::Fmul => {
            let right = pop(stack)?.float()?;
            let left = pop(stack)?.float()?;
            Value::Float(match instruction {
                I::Fadd => left + right,
                I::Fsub => left - right,
                _ => left * right,
            })
        }
        I::Dadd | I::Dsub | I::Dmul => {
            let right = pop(stack)?.double()?;
            let left = pop(stack)?.double()?;
            Value::Double(match instruction {
                I::Dadd => left + right,
                I::Dsub => left - right,
                _ => left * right,
            })
        }
        I::Lneg => Value::Long(pop(stack)?.long()?.wrapping_neg()),
        I::Fneg => Value::Float(-pop(stack)?.float()?),
        I::Dneg => Value::Double(-pop(stack)?.double()?),
        I::Lcmp => {
            let right = pop(stack)?.long()?;
            let left = pop(stack)?.long()?;
            Value::Int(ordering(left.cmp(&right)))
        }
        I::Fcmpl | I::Fcmpg => {
            let right = pop(stack)?.float()?;
            let left = pop(stack)?.float()?;
            Value::Int(left.partial_cmp(&right).map_or(
                if matches!(instruction, I::Fcmpl) {
                    -1
                } else {
                    1
                },
                ordering,
            ))
        }
        I::Dcmpl | I::Dcmpg => {
            let right = pop(stack)?.double()?;
            let left = pop(stack)?.double()?;
            Value::Int(left.partial_cmp(&right).map_or(
                if matches!(instruction, I::Dcmpl) {
                    -1
                } else {
                    1
                },
                ordering,
            ))
        }
        I::I2b => Value::Int(i32::from(pop(stack)?.int()? as i8)),
        I::I2c => Value::Int(i32::from(pop(stack)?.int()? as u16)),
        I::I2s => Value::Int(i32::from(pop(stack)?.int()? as i16)),
        I::I2l => Value::Long(i64::from(pop(stack)?.int()?)),
        I::I2f => Value::Float(pop(stack)?.int()? as f32),
        I::I2d => Value::Double(f64::from(pop(stack)?.int()?)),
        I::L2i => Value::Int(pop(stack)?.long()? as i32),
        I::L2f => Value::Float(pop(stack)?.long()? as f32),
        I::L2d => Value::Double(pop(stack)?.long()? as f64),
        I::F2i => Value::Int(pop(stack)?.float()? as i32),
        I::F2l => Value::Long(pop(stack)?.float()? as i64),
        I::F2d => Value::Double(f64::from(pop(stack)?.float()?)),
        I::D2i => Value::Int(pop(stack)?.double()? as i32),
        I::D2l => Value::Long(pop(stack)?.double()? as i64),
        I::D2f => Value::Float(pop(stack)?.double()? as f32),
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
