use crate::Error::{InvalidConstant, InvalidConstantIndex};
use crate::Result;
use crate::control_flow_graph::type_stack::TypeStack;
use cranelift::prelude::{Type, types};
use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::{BaseType, Constant, ConstantPool, FieldType};

/// Simulates the effect of an instruction on the stack.
#[expect(clippy::too_many_lines)]
pub(crate) fn simulate(
    stack: &mut TypeStack,
    constant_pool: &ConstantPool,
    instruction: &Instruction,
) -> Result<()> {
    match instruction {
        Instruction::Nop
        | Instruction::Iinc(..)
        | Instruction::Iinc_w(..)
        | Instruction::Goto(..)
        | Instruction::Goto_w(..)
        | Instruction::Ret(..)
        | Instruction::Ret_w(..)
        | Instruction::Return
        | Instruction::Wide
        | Instruction::Breakpoint
        | Instruction::Impdep1
        | Instruction::Impdep2 => {}
        Instruction::Aconst_null => stack.push_object()?,
        Instruction::Iconst_m1
        | Instruction::Iconst_0
        | Instruction::Iconst_1
        | Instruction::Iconst_2
        | Instruction::Iconst_3
        | Instruction::Iconst_4
        | Instruction::Iconst_5 => stack.push_int()?,
        Instruction::Lconst_0 | Instruction::Lconst_1 => stack.push_long()?,
        Instruction::Fconst_0 | Instruction::Fconst_1 | Instruction::Fconst_2 => {
            stack.push_float()?;
        }
        Instruction::Dconst_0 | Instruction::Dconst_1 => stack.push_double()?,
        Instruction::Bipush(..) | Instruction::Sipush(..) => stack.push_int()?,
        Instruction::Ldc(index) => ldc(stack, constant_pool, u16::from(*index))?,
        Instruction::Ldc_w(index) => ldc(stack, constant_pool, *index)?,
        Instruction::Ldc2_w(index) => {
            let constant = constant_pool
                .get(*index)
                .ok_or_else(|| InvalidConstantIndex(*index))?;

            match constant {
                Constant::Long(_) => stack.push_long()?,
                Constant::Double(_) => stack.push_double()?,
                constant => {
                    return Err(InvalidConstant {
                        expected: "long|double".to_string(),
                        actual: format!("{constant:?}"),
                    });
                }
            }
        }
        Instruction::Iload(..) | Instruction::Iload_w(..) => stack.push_int()?,
        Instruction::Lload(..) | Instruction::Lload_w(..) => stack.push_long()?,
        Instruction::Fload(..) | Instruction::Fload_w(..) => stack.push_float()?,
        Instruction::Dload(..) | Instruction::Dload_w(..) => stack.push_double()?,
        Instruction::Aload(..) | Instruction::Aload_w(..) => stack.push_object()?,
        Instruction::Iload_0
        | Instruction::Iload_1
        | Instruction::Iload_2
        | Instruction::Iload_3 => stack.push_int()?,
        Instruction::Lload_0
        | Instruction::Lload_1
        | Instruction::Lload_2
        | Instruction::Lload_3 => stack.push_long()?,
        Instruction::Fload_0
        | Instruction::Fload_1
        | Instruction::Fload_2
        | Instruction::Fload_3 => stack.push_float()?,
        Instruction::Dload_0
        | Instruction::Dload_1
        | Instruction::Dload_2
        | Instruction::Dload_3 => stack.push_double()?,
        Instruction::Aload_0
        | Instruction::Aload_1
        | Instruction::Aload_2
        | Instruction::Aload_3 => stack.push_object()?,
        Instruction::Iaload => {
            let _ = stack.pop_int()?;
            let _ = stack.pop_object()?;
            stack.push_int()?;
        }
        Instruction::Laload => {
            let _ = stack.pop_int()?;
            let _ = stack.pop_object()?;
            stack.push_long()?;
        }
        Instruction::Faload => {
            let _ = stack.pop_int()?;
            let _ = stack.pop_object()?;
            stack.push_float()?;
        }
        Instruction::Daload => {
            let _ = stack.pop_int()?;
            let _ = stack.pop_object()?;
            stack.push_double()?;
        }
        Instruction::Aaload => {
            let _ = stack.pop_int()?;
            let _ = stack.pop_object()?;
            stack.push_object()?;
        }
        Instruction::Baload | Instruction::Caload | Instruction::Saload => {
            let _ = stack.pop_int()?;
            let _ = stack.pop_object()?;
            stack.push_int()?;
        }
        Instruction::Istore(..) | Instruction::Istore_w(..) => {
            let _ = stack.pop_int()?;
        }
        Instruction::Lstore(..) | Instruction::Lstore_w(..) => {
            let _ = stack.pop_long()?;
        }
        Instruction::Fstore(..) | Instruction::Fstore_w(..) => {
            let _ = stack.pop_float()?;
        }
        Instruction::Dstore(..) | Instruction::Dstore_w(..) => {
            let _ = stack.pop_double()?;
        }
        Instruction::Astore(..) | Instruction::Astore_w(..) => {
            let _ = stack.pop_object()?;
        }
        Instruction::Istore_0
        | Instruction::Istore_1
        | Instruction::Istore_2
        | Instruction::Istore_3 => {
            let _ = stack.pop_int()?;
        }
        Instruction::Lstore_0
        | Instruction::Lstore_1
        | Instruction::Lstore_2
        | Instruction::Lstore_3 => {
            let _ = stack.pop_long()?;
        }
        Instruction::Fstore_0
        | Instruction::Fstore_1
        | Instruction::Fstore_2
        | Instruction::Fstore_3 => {
            let _ = stack.pop_float()?;
        }
        Instruction::Dstore_0
        | Instruction::Dstore_1
        | Instruction::Dstore_2
        | Instruction::Dstore_3 => {
            let _ = stack.pop_double()?;
        }
        Instruction::Astore_0
        | Instruction::Astore_1
        | Instruction::Astore_2
        | Instruction::Astore_3 => {
            let _ = stack.pop_object()?;
        }
        Instruction::Iastore => {
            let _ = stack.pop_int()?;
            let _ = stack.pop_int()?;
            let _ = stack.pop_object()?;
        }
        Instruction::Lastore => {
            let _ = stack.pop_long()?;
            let _ = stack.pop_int()?;
            let _ = stack.pop_object()?;
        }
        Instruction::Fastore => {
            let _ = stack.pop_float()?;
            let _ = stack.pop_int()?;
            let _ = stack.pop_object()?;
        }
        Instruction::Dastore => {
            let _ = stack.pop_double()?;
            let _ = stack.pop_int()?;
            let _ = stack.pop_object()?;
        }
        Instruction::Aastore => {
            let _ = stack.pop_object()?;
            let _ = stack.pop_int()?;
            let _ = stack.pop_object()?;
        }
        Instruction::Bastore | Instruction::Castore | Instruction::Sastore => {
            let _ = stack.pop_int()?;
            let _ = stack.pop_int()?;
            let _ = stack.pop_object()?;
        }
        Instruction::Pop => {
            let _ = stack.pop()?;
        }
        Instruction::Pop2 => {
            let value_type = stack.pop()?;
            if is_category_1(value_type) {
                let _ = stack.pop()?;
            }
        }
        Instruction::Dup => {
            let value = stack.pop()?;
            stack.push(value)?;
            stack.push(value)?;
        }
        Instruction::Dup_x1 => {
            let value1 = stack.pop()?;
            let value2 = stack.pop()?;
            stack.push(value1)?;
            stack.push(value2)?;
            stack.push(value1)?;
        }
        Instruction::Dup_x2 => {
            let value1 = stack.pop()?;
            let value2 = stack.pop()?;
            if is_category_1(value2) {
                let value3 = stack.pop()?;
                stack.push(value1)?;
                stack.push(value3)?;
                stack.push(value2)?;
                stack.push(value1)?;
            } else {
                stack.push(value1)?;
                stack.push(value2)?;
                stack.push(value1)?;
            }
        }
        Instruction::Dup2 => {
            let value1 = stack.pop()?;
            if is_category_1(value1) {
                let value2 = stack.pop()?;
                stack.push(value2)?;
                stack.push(value1)?;
                stack.push(value2)?;
                stack.push(value1)?;
            } else {
                stack.push(value1)?;
                stack.push(value1)?;
            }
        }
        Instruction::Dup2_x1 => {
            let value1 = stack.pop()?;
            let value2 = stack.pop()?;
            if is_category_1(value1) {
                let value3 = stack.pop()?;
                stack.push(value2)?;
                stack.push(value1)?;
                stack.push(value3)?;
                stack.push(value2)?;
                stack.push(value1)?;
            } else {
                stack.push(value1)?;
                stack.push(value2)?;
                stack.push(value1)?;
            }
        }
        Instruction::Dup2_x2 => {
            let value1 = stack.pop()?;
            let value2 = stack.pop()?;
            if is_category_1(value1) {
                let value3 = stack.pop()?;
                if is_category_1(value3) {
                    let value4 = stack.pop()?;
                    stack.push(value2)?;
                    stack.push(value1)?;
                    stack.push(value4)?;
                } else {
                    stack.push(value1)?;
                }
                stack.push(value3)?;
                stack.push(value2)?;
                stack.push(value1)?;
            } else {
                if is_category_1(value2) {
                    let value3 = stack.pop()?;
                    stack.push(value1)?;
                    stack.push(value3)?;
                } else {
                    stack.push(value1)?;
                }
                stack.push(value2)?;
                stack.push(value1)?;
            }
        }
        Instruction::Swap => {
            let value1 = stack.pop()?;
            let value2 = stack.pop()?;
            stack.push(value1)?;
            stack.push(value2)?;
        }
        Instruction::Iadd
        | Instruction::Isub
        | Instruction::Imul
        | Instruction::Idiv
        | Instruction::Irem => {
            let _ = stack.pop_int()?;
            let _ = stack.pop_int()?;
            stack.push_int()?;
        }
        Instruction::Ladd
        | Instruction::Lsub
        | Instruction::Lmul
        | Instruction::Ldiv
        | Instruction::Lrem => {
            let _ = stack.pop_long()?;
            let _ = stack.pop_long()?;
            stack.push_long()?;
        }
        Instruction::Fadd
        | Instruction::Fsub
        | Instruction::Fmul
        | Instruction::Fdiv
        | Instruction::Frem => {
            let _ = stack.pop_float()?;
            let _ = stack.pop_float()?;
            stack.push_float()?;
        }
        Instruction::Dadd
        | Instruction::Dsub
        | Instruction::Dmul
        | Instruction::Ddiv
        | Instruction::Drem => {
            let _ = stack.pop_double()?;
            let _ = stack.pop_double()?;
            stack.push_double()?;
        }
        Instruction::Ineg => {
            let _ = stack.pop_int()?;
            stack.push_int()?;
        }
        Instruction::Lneg => {
            let _ = stack.pop_long()?;
            stack.push_long()?;
        }
        Instruction::Fneg => {
            let _ = stack.pop_float()?;
            stack.push_float()?;
        }
        Instruction::Dneg => {
            let _ = stack.pop_double()?;
            stack.push_double()?;
        }
        Instruction::Ishl
        | Instruction::Ishr
        | Instruction::Iushr
        | Instruction::Iand
        | Instruction::Ior
        | Instruction::Ixor => {
            let _ = stack.pop_int()?;
            let _ = stack.pop_int()?;
            stack.push_int()?;
        }
        Instruction::Lshl | Instruction::Lshr | Instruction::Lushr => {
            let _ = stack.pop_int()?;
            let _ = stack.pop_long()?;
            stack.push_long()?;
        }
        Instruction::Land | Instruction::Lor | Instruction::Lxor => {
            let _ = stack.pop_long()?;
            let _ = stack.pop_long()?;
            stack.push_long()?;
        }
        Instruction::I2l => {
            let _ = stack.pop_int()?;
            stack.push_long()?;
        }
        Instruction::I2f => {
            let _ = stack.pop_int()?;
            stack.push_float()?;
        }
        Instruction::I2d => {
            let _ = stack.pop_int()?;
            stack.push_double()?;
        }
        Instruction::L2i => {
            let _ = stack.pop_long()?;
            stack.push_int()?;
        }
        Instruction::L2f => {
            let _ = stack.pop_long()?;
            stack.push_float()?;
        }
        Instruction::L2d => {
            let _ = stack.pop_long()?;
            stack.push_double()?;
        }
        Instruction::F2i => {
            let _ = stack.pop_float()?;
            stack.push_int()?;
        }
        Instruction::F2l => {
            let _ = stack.pop_float()?;
            stack.push_long()?;
        }
        Instruction::F2d => {
            let _ = stack.pop_float()?;
            stack.push_double()?;
        }
        Instruction::D2i => {
            let _ = stack.pop_double()?;
            stack.push_int()?;
        }
        Instruction::D2l => {
            let _ = stack.pop_double()?;
            stack.push_long()?;
        }
        Instruction::D2f => {
            let _ = stack.pop_double()?;
            stack.push_float()?;
        }
        Instruction::I2b | Instruction::I2c | Instruction::I2s => {
            let _ = stack.pop_int()?;
            stack.push_int()?;
        }
        Instruction::Lcmp => {
            let _ = stack.pop_long()?;
            let _ = stack.pop_long()?;
            stack.push_int()?;
        }
        Instruction::Fcmpl | Instruction::Fcmpg => {
            let _ = stack.pop_float()?;
            let _ = stack.pop_float()?;
            stack.push_int()?;
        }
        Instruction::Dcmpl | Instruction::Dcmpg => {
            let _ = stack.pop_double()?;
            let _ = stack.pop_double()?;
            stack.push_int()?;
        }
        Instruction::Ifeq(..)
        | Instruction::Ifne(..)
        | Instruction::Iflt(..)
        | Instruction::Ifge(..)
        | Instruction::Ifgt(..)
        | Instruction::Ifle(..) => {
            let _ = stack.pop_int()?;
        }
        Instruction::If_icmpeq(..)
        | Instruction::If_icmpne(..)
        | Instruction::If_icmplt(..)
        | Instruction::If_icmpge(..)
        | Instruction::If_icmpgt(..)
        | Instruction::If_icmple(..) => {
            let _ = stack.pop_int()?;
            let _ = stack.pop_int()?;
        }
        Instruction::If_acmpeq(..) | Instruction::If_acmpne(..) => {
            let _ = stack.pop_object()?;
            let _ = stack.pop_object()?;
        }
        Instruction::Jsr(..) | Instruction::Jsr_w(..) => stack.push_int()?,
        Instruction::Tableswitch { .. } | Instruction::Lookupswitch { .. } => {
            let _ = stack.pop_int()?;
        }
        Instruction::Ireturn => {
            let _ = stack.pop_int()?;
        }
        Instruction::Lreturn => {
            let _ = stack.pop_long()?;
        }
        Instruction::Freturn => {
            let _ = stack.pop_float()?;
        }
        Instruction::Dreturn => {
            let _ = stack.pop_double()?;
        }
        Instruction::Areturn => {
            let _ = stack.pop_object()?;
        }
        Instruction::Getstatic(..) => stack.push_object()?,
        Instruction::Putstatic(..) => {
            let _ = stack.pop_object()?;
        }
        Instruction::Getfield(..) => {
            let _ = stack.pop_object()?;
            stack.push_object()?;
        }
        Instruction::Putfield(..) => {
            let _ = stack.pop_object()?;
            let _ = stack.pop_object()?;
        }
        Instruction::Invokevirtual(method_index)
        | Instruction::Invokespecial(method_index)
        | Instruction::Invokestatic(method_index)
        | Instruction::Invokeinterface(method_index, ..)
        | Instruction::Invokedynamic(method_index) => {
            let (_class_index, name_and_type_index) =
                constant_pool.try_get_method_ref(*method_index)?;
            let (_name_index, descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;

            if !matches!(instruction, Instruction::Invokespecial(..))
                && !matches!(instruction, Instruction::Invokestatic(..))
            {
                let _ = stack.pop_object()?;
            }

            let (parameters, return_type) = FieldType::parse_method_descriptor(method_descriptor)?;
            for parameter in parameters {
                match parameter {
                    FieldType::Base(
                        BaseType::Boolean
                        | BaseType::Byte
                        | BaseType::Char
                        | BaseType::Int
                        | BaseType::Short,
                    ) => {
                        let _ = stack.pop_int()?;
                    }
                    FieldType::Base(BaseType::Long) => {
                        let _ = stack.pop_long()?;
                    }
                    FieldType::Base(BaseType::Float) => {
                        let _ = stack.pop_float()?;
                    }
                    FieldType::Base(BaseType::Double) => {
                        let _ = stack.pop_double()?;
                    }
                    FieldType::Object(_) | FieldType::Array(_) => {
                        let _ = stack.pop_object()?;
                    }
                }
            }

            if let Some(return_type) = return_type {
                match return_type {
                    FieldType::Base(
                        BaseType::Boolean
                        | BaseType::Byte
                        | BaseType::Char
                        | BaseType::Int
                        | BaseType::Short,
                    ) => {
                        stack.push_int()?;
                    }
                    FieldType::Base(BaseType::Long) => {
                        stack.push_long()?;
                    }
                    FieldType::Base(BaseType::Float) => {
                        stack.push_float()?;
                    }
                    FieldType::Base(BaseType::Double) => {
                        stack.push_double()?;
                    }
                    FieldType::Object(_) | FieldType::Array(_) => {
                        stack.push_object()?;
                    }
                }
            }
        }
        Instruction::New(..) => {
            stack.push_object()?;
        }
        Instruction::Newarray(..) | Instruction::Anewarray(..) => {
            let _ = stack.pop_int()?;
            stack.push_object()?;
        }
        Instruction::Arraylength => {
            let _ = stack.pop_object()?;
            stack.push_int()?;
        }
        Instruction::Athrow => {
            let _ = stack.pop_object()?;
        }
        Instruction::Checkcast(..) => {
            let _ = stack.pop_object()?;
            stack.push_object()?;
        }
        Instruction::Instanceof(..) => {
            let _ = stack.pop_object()?;
            stack.push_int()?;
        }
        Instruction::Monitorenter | Instruction::Monitorexit => {
            let _ = stack.pop_object()?;
        }
        Instruction::Multianewarray(dimensions, ..) => {
            let _ = stack.pop_int()?;
            for _ in 1..*dimensions {
                let _ = stack.pop_int()?;
            }
            stack.push_object()?;
        }
        Instruction::Ifnull(..) | Instruction::Ifnonnull(..) => {
            let _ = stack.pop_object()?;
        }
    }
    Ok(())
}

/// Loads a constant from the constant pool and pushes the type onto the stack.
fn ldc(stack: &mut TypeStack, constant_pool: &ConstantPool, index: u16) -> Result<()> {
    let constant = constant_pool
        .get(index)
        .ok_or_else(|| InvalidConstantIndex(index))?;

    match constant {
        Constant::Integer(_) => stack.push_int(),
        Constant::Float(_) => stack.push_float(),
        Constant::String(_) | Constant::Class(_) => stack.push_object(),
        constant => Err(InvalidConstant {
            expected: "integer|float|string|class".to_string(),
            actual: format!("{constant:?}"),
        }),
    }
}

/// Returns true if the type is a category 1 type.
fn is_category_1(value_type: Type) -> bool {
    is_category_2(value_type)
}

/// Returns true if the type is a category 2 type.
fn is_category_2(value_type: Type) -> bool {
    value_type == types::I64 || value_type == types::F64
}
