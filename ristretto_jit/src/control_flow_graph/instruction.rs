use crate::Error::{InvalidConstant, InvalidConstantIndex};
use crate::Result;
use crate::control_flow_graph::type_stack::TypeStack;
use cranelift::prelude::{Type, types};
use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::{BaseType, Constant, ConstantPool, FieldType};
use tracing::trace;

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
        Instruction::Iload(..)
        | Instruction::Iload_w(..)
        | Instruction::Iload_0
        | Instruction::Iload_1
        | Instruction::Iload_2
        | Instruction::Iload_3 => stack.push_int()?,
        Instruction::Lload(..)
        | Instruction::Lload_w(..)
        | Instruction::Lload_0
        | Instruction::Lload_1
        | Instruction::Lload_2
        | Instruction::Lload_3 => stack.push_long()?,
        Instruction::Fload(..)
        | Instruction::Fload_w(..)
        | Instruction::Fload_0
        | Instruction::Fload_1
        | Instruction::Fload_2
        | Instruction::Fload_3 => stack.push_float()?,
        Instruction::Dload(..)
        | Instruction::Dload_w(..)
        | Instruction::Dload_0
        | Instruction::Dload_1
        | Instruction::Dload_2
        | Instruction::Dload_3 => stack.push_double()?,
        Instruction::Aload(..)
        | Instruction::Aload_w(..)
        | Instruction::Aload_0
        | Instruction::Aload_1
        | Instruction::Aload_2
        | Instruction::Aload_3 => stack.push_object()?,
        Instruction::Iaload | Instruction::Baload | Instruction::Caload | Instruction::Saload => {
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
        Instruction::Istore(..)
        | Instruction::Istore_w(..)
        | Instruction::Istore_0
        | Instruction::Istore_1
        | Instruction::Istore_2
        | Instruction::Istore_3 => {
            let _ = stack.pop_int()?;
        }
        Instruction::Lstore(..)
        | Instruction::Lstore_w(..)
        | Instruction::Lstore_0
        | Instruction::Lstore_1
        | Instruction::Lstore_2
        | Instruction::Lstore_3 => {
            let _ = stack.pop_long()?;
        }
        Instruction::Fstore(..)
        | Instruction::Fstore_w(..)
        | Instruction::Fstore_0
        | Instruction::Fstore_1
        | Instruction::Fstore_2
        | Instruction::Fstore_3 => {
            let _ = stack.pop_float()?;
        }
        Instruction::Dstore(..)
        | Instruction::Dstore_w(..)
        | Instruction::Dstore_0
        | Instruction::Dstore_1
        | Instruction::Dstore_2
        | Instruction::Dstore_3 => {
            let _ = stack.pop_double()?;
        }
        Instruction::Astore(..)
        | Instruction::Astore_w(..)
        | Instruction::Astore_0
        | Instruction::Astore_1
        | Instruction::Astore_2
        | Instruction::Astore_3 => {
            let _ = stack.pop_object()?;
        }
        Instruction::Iastore
        | Instruction::Bastore
        | Instruction::Castore
        | Instruction::Sastore => {
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
        Instruction::Getstatic(index) => {
            let (class_index, name_and_type_index) = constant_pool.try_get_field_ref(*index)?;
            let (name_index, descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let field_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;

            #[cfg(debug_assertions)]
            {
                let class_name = constant_pool.try_get_class(*class_index)?;
                let field_name = constant_pool.try_get_utf8(*name_index)?;
                trace!("Simulating {instruction} on {class_name}.{field_name}");
            }

            let field_type = FieldType::parse(field_descriptor)?;
            push_field_type(stack, &field_type)?;
        }
        Instruction::Putstatic(index) => {
            let (class_index, name_and_type_index) = constant_pool.try_get_field_ref(*index)?;
            let (name_index, descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let field_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;

            #[cfg(debug_assertions)]
            {
                let class_name = constant_pool.try_get_class(*class_index)?;
                let field_name = constant_pool.try_get_utf8(*name_index)?;
                trace!("Simulating {instruction} on {class_name}.{field_name}");
            }

            let field_type = FieldType::parse(field_descriptor)?;
            let _ = pop_field_type(stack, &field_type)?;
        }
        Instruction::Getfield(index) => {
            let (class_index, name_and_type_index) = constant_pool.try_get_field_ref(*index)?;
            let (name_index, descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let field_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;

            #[cfg(debug_assertions)]
            {
                let class_name = constant_pool.try_get_class(*class_index)?;
                let field_name = constant_pool.try_get_utf8(*name_index)?;
                trace!("Simulating {instruction} on {class_name}.{field_name}");
            }

            let _ = stack.pop_object()?;
            let field_type = FieldType::parse(field_descriptor)?;
            push_field_type(stack, &field_type)?;
        }
        Instruction::Putfield(index) => {
            let (class_index, name_and_type_index) = constant_pool.try_get_field_ref(*index)?;
            let (name_index, descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let field_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;

            #[cfg(debug_assertions)]
            {
                let class_name = constant_pool.try_get_class(*class_index)?;
                let field_name = constant_pool.try_get_utf8(*name_index)?;
                trace!("Simulating {instruction} on {class_name}.{field_name}");
            }

            let field_type = FieldType::parse(field_descriptor)?;
            let _ = pop_field_type(stack, &field_type)?;
            let _ = stack.pop_object()?;
        }
        Instruction::Invokevirtual(method_index)
        | Instruction::Invokespecial(method_index)
        | Instruction::Invokestatic(method_index)
        | Instruction::Invokedynamic(method_index) => {
            let (class_index, name_and_type_index) =
                constant_pool.try_get_method_ref(*method_index)?;
            let (name_index, descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;

            #[cfg(debug_assertions)]
            {
                let class_name = constant_pool.try_get_class(*class_index)?;
                let method_name = constant_pool.try_get_utf8(*name_index)?;
                trace!("Simulating {instruction} on {class_name}.{method_name}{method_descriptor}");
            }

            if !matches!(instruction, Instruction::Invokespecial(..))
                && !matches!(instruction, Instruction::Invokestatic(..))
            {
                let _ = stack.pop_object()?;
            }

            let (parameters, return_type) = FieldType::parse_method_descriptor(method_descriptor)?;
            for parameter in parameters {
                pop_field_type(stack, &parameter)?;
            }

            if let Some(return_type) = return_type {
                push_field_type(stack, &return_type)?;
            }
        }
        Instruction::Invokeinterface(method_index, ..) => {
            let (class_index, name_and_type_index) =
                constant_pool.try_get_interface_method_ref(*method_index)?;
            let (name_index, descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;

            #[cfg(debug_assertions)]
            {
                let class_name = constant_pool.try_get_class(*class_index)?;
                let method_name = constant_pool.try_get_utf8(*name_index)?;
                trace!("Simulating {instruction} on {class_name}.{method_name}{method_descriptor}");
            }

            let _ = stack.pop_object()?;
            let (parameters, return_type) = FieldType::parse_method_descriptor(method_descriptor)?;
            for parameter in parameters {
                pop_field_type(stack, &parameter)?;
            }

            if let Some(return_type) = return_type {
                push_field_type(stack, &return_type)?;
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

/// Returns true if the type is a category 1 type.
fn is_category_1(value_type: Type) -> bool {
    !is_category_2(value_type)
}

/// Returns true if the type is a category 2 type.
fn is_category_2(value_type: Type) -> bool {
    value_type == types::I64 || value_type == types::F64
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

/// Pushes a field type onto the stack.
fn push_field_type(stack: &mut TypeStack, field_type: &FieldType) -> Result<()> {
    match field_type {
        FieldType::Base(
            BaseType::Boolean | BaseType::Byte | BaseType::Char | BaseType::Int | BaseType::Short,
        ) => stack.push_int(),
        FieldType::Base(BaseType::Long) => stack.push_long(),
        FieldType::Base(BaseType::Float) => stack.push_float(),
        FieldType::Base(BaseType::Double) => stack.push_double(),
        FieldType::Object(_) | FieldType::Array(_) => stack.push_object(),
    }
}

/// Pops a field type from the stack.
fn pop_field_type(stack: &mut TypeStack, field_type: &FieldType) -> Result<Type> {
    match field_type {
        FieldType::Base(
            BaseType::Boolean | BaseType::Byte | BaseType::Char | BaseType::Int | BaseType::Short,
        ) => stack.pop_int(),
        FieldType::Base(BaseType::Long) => stack.pop_long(),
        FieldType::Base(BaseType::Float) => stack.pop_float(),
        FieldType::Base(BaseType::Double) => stack.pop_double(),
        FieldType::Object(_) | FieldType::Array(_) => stack.pop_object(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use ristretto_classfile::attributes::ArrayType;

    #[test]
    fn test_ldc_integer() -> Result<()> {
        let mut stack = TypeStack::new();
        let mut constant_pool = ConstantPool::new();
        let index = constant_pool.add_integer(42)?;
        ldc(&mut stack, &constant_pool, index)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_ldc_float() -> Result<()> {
        let mut stack = TypeStack::new();
        let mut constant_pool = ConstantPool::new();
        let index = constant_pool.add_float(42.0)?;
        ldc(&mut stack, &constant_pool, index)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_float().is_ok());
        Ok(())
    }

    #[test]
    fn test_ldc_string() -> Result<()> {
        let mut stack = TypeStack::new();
        let mut constant_pool = ConstantPool::new();
        let index = constant_pool.add_string("foo")?;
        ldc(&mut stack, &constant_pool, index)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_object().is_ok());
        Ok(())
    }

    #[test]
    fn test_ldc_class() -> Result<()> {
        let mut stack = TypeStack::new();
        let mut constant_pool = ConstantPool::new();
        let index = constant_pool.add_class("Foo")?;
        ldc(&mut stack, &constant_pool, index)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_object().is_ok());
        Ok(())
    }

    #[test]
    fn test_ldc_long_error() -> Result<()> {
        let mut stack = TypeStack::new();
        let mut constant_pool = ConstantPool::new();
        let index = constant_pool.add_long(42)?;
        assert!(ldc(&mut stack, &constant_pool, index).is_err());
        Ok(())
    }

    #[test]
    fn test_is_category_1() {
        assert!(is_category_1(types::I32));
        assert!(is_category_1(types::F32));
        assert!(!is_category_1(types::I64));
        assert!(!is_category_1(types::F64));
    }

    #[test]
    fn test_is_category_2() {
        assert!(is_category_2(types::I64));
        assert!(is_category_2(types::F64));
        assert!(!is_category_2(types::I32));
        assert!(!is_category_2(types::F32));
    }

    #[test]
    fn test_push_field_type_int() -> Result<()> {
        let field_types = vec![
            FieldType::Base(BaseType::Boolean),
            FieldType::Base(BaseType::Byte),
            FieldType::Base(BaseType::Char),
            FieldType::Base(BaseType::Int),
            FieldType::Base(BaseType::Short),
        ];
        for field_type in field_types {
            let mut stack = TypeStack::new();
            push_field_type(&mut stack, &field_type)?;
            assert!(stack.pop_int().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_push_field_type_long() -> Result<()> {
        let mut stack = TypeStack::new();
        let field_type = FieldType::Base(BaseType::Long);
        push_field_type(&mut stack, &field_type)?;
        assert!(stack.pop_long().is_ok());
        Ok(())
    }

    #[test]
    fn test_push_field_type_float() -> Result<()> {
        let mut stack = TypeStack::new();
        let field_type = FieldType::Base(BaseType::Float);
        push_field_type(&mut stack, &field_type)?;
        assert!(stack.pop_float().is_ok());
        Ok(())
    }

    #[test]
    fn test_push_field_type_double() -> Result<()> {
        let mut stack = TypeStack::new();
        let field_type = FieldType::Base(BaseType::Double);
        push_field_type(&mut stack, &field_type)?;
        assert!(stack.pop_double().is_ok());
        Ok(())
    }

    #[test]
    fn test_push_field_type_object() -> Result<()> {
        let field_types = vec![
            FieldType::Object("java/lang/String".to_string()),
            FieldType::Array(Box::new(FieldType::Base(BaseType::Int))),
        ];
        for field_type in field_types {
            let mut stack = TypeStack::new();
            push_field_type(&mut stack, &field_type)?;
            assert!(stack.pop_object().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_pop_field_type_int() -> Result<()> {
        let field_types = vec![
            FieldType::Base(BaseType::Boolean),
            FieldType::Base(BaseType::Byte),
            FieldType::Base(BaseType::Char),
            FieldType::Base(BaseType::Int),
            FieldType::Base(BaseType::Short),
        ];
        for field_type in field_types {
            let mut stack = TypeStack::new();
            stack.push_int()?;
            let value_type = pop_field_type(&mut stack, &field_type)?;
            assert_eq!(value_type, types::I32);
        }
        Ok(())
    }

    #[test]
    fn test_pop_field_type_long() -> Result<()> {
        let mut stack = TypeStack::new();
        stack.push_long()?;
        let field_type = FieldType::Base(BaseType::Long);
        let value_type = pop_field_type(&mut stack, &field_type)?;
        assert_eq!(value_type, types::I64);
        Ok(())
    }

    #[test]
    fn test_pop_field_type_float() -> Result<()> {
        let mut stack = TypeStack::new();
        stack.push_float()?;
        let field_type = FieldType::Base(BaseType::Float);
        let value_type = pop_field_type(&mut stack, &field_type)?;
        assert_eq!(value_type, types::F32);
        Ok(())
    }

    #[test]
    fn test_pop_field_type_double() -> Result<()> {
        let mut stack = TypeStack::new();
        stack.push_double()?;
        let field_type = FieldType::Base(BaseType::Double);
        let value_type = pop_field_type(&mut stack, &field_type)?;
        assert_eq!(value_type, types::F64);
        Ok(())
    }

    #[test]
    fn test_pop_field_type_object() -> Result<()> {
        let field_types = vec![
            FieldType::Object("java/lang/String".to_string()),
            FieldType::Array(Box::new(FieldType::Base(BaseType::Int))),
        ];
        for field_type in field_types {
            let mut stack = TypeStack::new();
            stack.push_object()?;
            let value_type = pop_field_type(&mut stack, &field_type)?;
            assert_eq!(value_type, types::I64);
        }
        Ok(())
    }

    #[test]
    fn test_simulate_nop_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Nop,
            Instruction::Iinc(1, 2),
            Instruction::Iinc_w(1, 2),
            Instruction::Goto(1),
            Instruction::Goto_w(1),
            Instruction::Ret(1),
            Instruction::Ret_w(1),
            Instruction::Return,
            Instruction::Wide,
            Instruction::Breakpoint,
            Instruction::Impdep1,
            Instruction::Impdep2,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 0);
        }
        Ok(())
    }

    #[test]
    fn test_simulate_aconst_null() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        let instruction = Instruction::Aconst_null;
        simulate(&mut stack, &constant_pool, &instruction)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_object().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_iconst_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Iconst_m1,
            Instruction::Iconst_0,
            Instruction::Iconst_1,
            Instruction::Iconst_2,
            Instruction::Iconst_3,
            Instruction::Iconst_4,
            Instruction::Iconst_5,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_int().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_lconst_instructions() -> Result<()> {
        let instructions = vec![Instruction::Lconst_0, Instruction::Lconst_1];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_long().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_fconst_instructions() -> Result<()> {
        let instructions = vec![Instruction::Fconst_0, Instruction::Fconst_1];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_float().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_dconst_instructions() -> Result<()> {
        let instructions = vec![Instruction::Dconst_0, Instruction::Dconst_1];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_double().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_bipush_and_sipush_instructions() -> Result<()> {
        let instructions = vec![Instruction::Bipush(1), Instruction::Sipush(1)];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_int().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_ldc_instruction() -> Result<()> {
        let mut stack = TypeStack::new();
        let mut constant_pool = ConstantPool::new();
        let index = constant_pool.add_integer(42)?;
        let index = u8::try_from(index)?;
        let instruction = Instruction::Ldc(index);
        simulate(&mut stack, &constant_pool, &instruction)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_ldc_w_instruction() -> Result<()> {
        let mut stack = TypeStack::new();
        let mut constant_pool = ConstantPool::new();
        let index = constant_pool.add_integer(42)?;
        let instruction = Instruction::Ldc_w(index);
        simulate(&mut stack, &constant_pool, &instruction)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_ldc_w_long_instruction() -> Result<()> {
        let mut stack = TypeStack::new();
        let mut constant_pool = ConstantPool::new();
        let index = constant_pool.add_long(42)?;
        let instruction = Instruction::Ldc2_w(index);
        simulate(&mut stack, &constant_pool, &instruction)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_long().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_ldc_w_double_instruction() -> Result<()> {
        let mut stack = TypeStack::new();
        let mut constant_pool = ConstantPool::new();
        let index = constant_pool.add_double(42.0)?;
        let instruction = Instruction::Ldc2_w(index);
        simulate(&mut stack, &constant_pool, &instruction)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_double().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_ldc_w_int_instruction_error() -> Result<()> {
        let mut stack = TypeStack::new();
        let mut constant_pool = ConstantPool::new();
        let index = constant_pool.add_integer(42)?;
        let instruction = Instruction::Ldc2_w(index);
        let result = simulate(&mut stack, &constant_pool, &instruction);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_simulate_iload_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Iload(0),
            Instruction::Iload_w(0),
            Instruction::Iload_0,
            Instruction::Iload_1,
            Instruction::Iload_2,
            Instruction::Iload_3,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_int().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_lload_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Lload(0),
            Instruction::Lload_w(0),
            Instruction::Lload_0,
            Instruction::Lload_1,
            Instruction::Lload_2,
            Instruction::Lload_3,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_long().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_fload_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Fload(0),
            Instruction::Fload_w(0),
            Instruction::Fload_0,
            Instruction::Fload_1,
            Instruction::Fload_2,
            Instruction::Fload_3,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_float().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_dload_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Dload(0),
            Instruction::Dload_w(0),
            Instruction::Dload_0,
            Instruction::Dload_1,
            Instruction::Dload_2,
            Instruction::Dload_3,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_double().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_aload_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Aload(0),
            Instruction::Aload_w(0),
            Instruction::Aload_0,
            Instruction::Aload_1,
            Instruction::Aload_2,
            Instruction::Aload_3,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_object().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_iaload() -> Result<()> {
        let instructions = vec![
            Instruction::Iaload,
            Instruction::Baload,
            Instruction::Caload,
            Instruction::Saload,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_object()?;
            stack.push_int()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_int().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_laload() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Laload)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_long().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_faload() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Faload)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_float().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_daload() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Daload)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_double().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_aaload() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Aaload)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_object().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_istore_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Istore(0),
            Instruction::Istore_w(0),
            Instruction::Istore_0,
            Instruction::Istore_1,
            Instruction::Istore_2,
            Instruction::Istore_3,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_int()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 0);
        }
        Ok(())
    }

    #[test]
    fn test_simulate_lstore_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Lstore(0),
            Instruction::Lstore_w(0),
            Instruction::Lstore_0,
            Instruction::Lstore_1,
            Instruction::Lstore_2,
            Instruction::Lstore_3,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_long()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 0);
        }
        Ok(())
    }

    #[test]
    fn test_simulate_fstore_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Fstore(0),
            Instruction::Fstore_w(0),
            Instruction::Fstore_0,
            Instruction::Fstore_1,
            Instruction::Fstore_2,
            Instruction::Fstore_3,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_float()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 0);
        }
        Ok(())
    }

    #[test]
    fn test_simulate_dstore_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Dstore(0),
            Instruction::Dstore_w(0),
            Instruction::Dstore_0,
            Instruction::Dstore_1,
            Instruction::Dstore_2,
            Instruction::Dstore_3,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_double()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 0);
        }
        Ok(())
    }

    #[test]
    fn test_simulate_astore_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Astore(0),
            Instruction::Astore_w(0),
            Instruction::Astore_0,
            Instruction::Astore_1,
            Instruction::Astore_2,
            Instruction::Astore_3,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_object()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 0);
        }
        Ok(())
    }

    #[test]
    fn test_simulate_iastore_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Iastore,
            Instruction::Bastore,
            Instruction::Castore,
            Instruction::Sastore,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_object()?;
            stack.push_int()?;
            stack.push_int()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 0);
        }
        Ok(())
    }

    #[test]
    fn test_simulate_lastore() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        stack.push_int()?;
        stack.push_long()?;
        simulate(&mut stack, &constant_pool, &Instruction::Lastore)?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_fastore() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        stack.push_int()?;
        stack.push_float()?;
        simulate(&mut stack, &constant_pool, &Instruction::Fastore)?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_dastore() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        stack.push_int()?;
        stack.push_double()?;
        simulate(&mut stack, &constant_pool, &Instruction::Dastore)?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_aastore() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        stack.push_int()?;
        stack.push_object()?;
        simulate(&mut stack, &constant_pool, &Instruction::Aastore)?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_pop() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Pop)?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_pop2_int() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Pop2)?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_pop2_long() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_long()?;
        simulate(&mut stack, &constant_pool, &Instruction::Pop2)?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_dup() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Dup)?;
        assert_eq!(stack.len(), 2);
        assert!(stack.pop_int().is_ok());
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_dup_x1() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_float()?;
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Dup_x1)?;
        assert_eq!(stack.len(), 3);
        assert!(stack.pop_int().is_ok());
        assert!(stack.pop_float().is_ok());
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_dup_x2_int() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_float()?;
        stack.push_int()?;
        stack.push_float()?;
        simulate(&mut stack, &constant_pool, &Instruction::Dup_x2)?;
        assert_eq!(stack.len(), 4);
        assert!(stack.pop_float().is_ok());
        assert!(stack.pop_int().is_ok());
        assert!(stack.pop_float().is_ok());
        assert!(stack.pop_float().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_dup_x2_long() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_long()?;
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Dup_x2)?;
        assert_eq!(stack.len(), 3);
        assert!(stack.pop_int().is_ok());
        assert!(stack.pop_long().is_ok());
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_dup2_int() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_float()?;
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Dup2)?;
        assert_eq!(stack.len(), 4);
        assert!(stack.pop_int().is_ok());
        assert!(stack.pop_float().is_ok());
        assert!(stack.pop_int().is_ok());
        assert!(stack.pop_float().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_dup2_long() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_long()?;
        simulate(&mut stack, &constant_pool, &Instruction::Dup2)?;
        assert_eq!(stack.len(), 2);
        assert!(stack.pop_long().is_ok());
        assert!(stack.pop_long().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_dup2_x1_int() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        stack.push_float()?;
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Dup2_x1)?;
        assert_eq!(stack.len(), 5);
        assert!(stack.pop_int().is_ok());
        assert!(stack.pop_float().is_ok());
        assert!(stack.pop_int().is_ok());
        assert!(stack.pop_int().is_ok());
        assert!(stack.pop_float().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_dup2_x1_long() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        stack.push_long()?;
        simulate(&mut stack, &constant_pool, &Instruction::Dup2_x1)?;
        assert_eq!(stack.len(), 3);
        assert!(stack.pop_long().is_ok());
        assert!(stack.pop_int().is_ok());
        assert!(stack.pop_long().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_dup2_x2_int_int() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_float()?;
        stack.push_int()?;
        stack.push_float()?;
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Dup2_x2)?;
        assert_eq!(stack.len(), 6);
        assert!(stack.pop_int().is_ok());
        assert!(stack.pop_float().is_ok());
        assert!(stack.pop_int().is_ok());
        assert!(stack.pop_float().is_ok());
        assert!(stack.pop_int().is_ok());
        assert!(stack.pop_float().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_dup2_x2_int_long() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_long()?;
        stack.push_float()?;
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Dup2_x2)?;
        assert_eq!(stack.len(), 4);
        assert!(stack.pop_int().is_ok());
        assert!(stack.pop_float().is_ok());
        assert!(stack.pop_long().is_ok());
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_dup2_x2_long_int() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_float()?;
        stack.push_int()?;
        stack.push_long()?;
        simulate(&mut stack, &constant_pool, &Instruction::Dup2_x2)?;
        assert_eq!(stack.len(), 4);
        assert!(stack.pop_long().is_ok());
        assert!(stack.pop_int().is_ok());
        assert!(stack.pop_float().is_ok());
        assert!(stack.pop_long().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_dup2_x2_long_long() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_long()?;
        stack.push_long()?;
        simulate(&mut stack, &constant_pool, &Instruction::Dup2_x2)?;
        assert_eq!(stack.len(), 3);
        assert!(stack.pop_long().is_ok());
        assert!(stack.pop_long().is_ok());
        assert!(stack.pop_long().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_swap() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_float()?;
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Swap)?;
        assert_eq!(stack.len(), 2);
        assert!(stack.pop_float().is_ok());
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_imath_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Iadd,
            Instruction::Isub,
            Instruction::Imul,
            Instruction::Idiv,
            Instruction::Irem,
            Instruction::Iand,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_int()?;
            stack.push_int()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_int().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_lmath_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Ladd,
            Instruction::Lsub,
            Instruction::Lmul,
            Instruction::Ldiv,
            Instruction::Lrem,
            Instruction::Land,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_long()?;
            stack.push_long()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_long().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_fmath_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Fadd,
            Instruction::Fsub,
            Instruction::Fmul,
            Instruction::Fdiv,
            Instruction::Frem,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_float()?;
            stack.push_float()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_float().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_dmath_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Dadd,
            Instruction::Dsub,
            Instruction::Dmul,
            Instruction::Ddiv,
            Instruction::Drem,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_double()?;
            stack.push_double()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_double().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_ineg() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Ineg)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_lneg() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_long()?;
        simulate(&mut stack, &constant_pool, &Instruction::Lneg)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_long().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_fneg() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_float()?;
        simulate(&mut stack, &constant_pool, &Instruction::Fneg)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_float().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_dneg() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_double()?;
        simulate(&mut stack, &constant_pool, &Instruction::Dneg)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_double().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_ibitwise_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Ishl,
            Instruction::Ishr,
            Instruction::Iushr,
            Instruction::Iand,
            Instruction::Ior,
            Instruction::Ixor,
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_int()?;
            stack.push_int()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_int().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_lbitshift_instructions() -> Result<()> {
        let instructions = vec![Instruction::Lshl, Instruction::Lshr, Instruction::Lushr];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_long()?;
            stack.push_int()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_long().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_lbitwise_instructions() -> Result<()> {
        let instructions = vec![Instruction::Land, Instruction::Lor, Instruction::Lxor];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_long()?;
            stack.push_long()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_long().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_i2l() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::I2l)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_long().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_i2f() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::I2f)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_float().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_i2d() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::I2d)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_double().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_l2i() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_long()?;
        simulate(&mut stack, &constant_pool, &Instruction::L2i)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_l2f() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_long()?;
        simulate(&mut stack, &constant_pool, &Instruction::L2f)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_float().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_l2d() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_long()?;
        simulate(&mut stack, &constant_pool, &Instruction::L2d)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_double().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_f2i() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_float()?;
        simulate(&mut stack, &constant_pool, &Instruction::F2i)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_f2l() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_float()?;
        simulate(&mut stack, &constant_pool, &Instruction::F2l)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_long().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_f2d() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_float()?;
        simulate(&mut stack, &constant_pool, &Instruction::F2d)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_double().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_d2i() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_double()?;
        simulate(&mut stack, &constant_pool, &Instruction::D2i)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_d2l() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_double()?;
        simulate(&mut stack, &constant_pool, &Instruction::D2l)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_long().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_d2f() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_double()?;
        simulate(&mut stack, &constant_pool, &Instruction::D2f)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_float().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_i2b() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::I2b)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_i2c() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::I2c)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_i2s() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::I2s)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_lcmp() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_long()?;
        stack.push_long()?;
        simulate(&mut stack, &constant_pool, &Instruction::Lcmp)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_fcmp_instructions() -> Result<()> {
        let instructions = vec![Instruction::Fcmpl, Instruction::Fcmpg];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_float()?;
            stack.push_float()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_int().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_dcmp_instructions() -> Result<()> {
        let instructions = vec![Instruction::Dcmpl, Instruction::Dcmpg];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_double()?;
            stack.push_double()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_int().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_if_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Ifeq(3),
            Instruction::Ifne(3),
            Instruction::Iflt(3),
            Instruction::Ifge(3),
            Instruction::Ifgt(3),
            Instruction::Ifle(3),
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_int()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 0);
        }
        Ok(())
    }

    #[test]
    fn test_simulate_if_icmp_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::If_icmpeq(3),
            Instruction::If_icmpne(3),
            Instruction::If_icmplt(3),
            Instruction::If_icmpge(3),
            Instruction::If_icmpgt(3),
            Instruction::If_icmple(3),
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_int()?;
            stack.push_int()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 0);
        }
        Ok(())
    }

    #[test]
    fn test_simulate_if_acmp_instructions() -> Result<()> {
        let instructions = vec![Instruction::If_acmpeq(3), Instruction::If_acmpne(3)];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_object()?;
            stack.push_object()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 0);
        }
        Ok(())
    }

    #[test]
    fn test_simulate_jsr_instructions() -> Result<()> {
        let instructions = vec![Instruction::Jsr(3), Instruction::Jsr_w(3)];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_int().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_switch_instructions() -> Result<()> {
        let instructions = vec![
            Instruction::Tableswitch {
                default: 3,
                low: 1,
                high: 2,
                offsets: vec![1; 2],
            },
            Instruction::Lookupswitch {
                default: 3,
                pairs: IndexMap::new(),
            },
        ];
        for instruction in instructions {
            let mut stack = TypeStack::new();
            let constant_pool = ConstantPool::new();
            stack.push_int()?;
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 0);
        }
        Ok(())
    }

    #[test]
    fn test_simulate_ireturn() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Ireturn)?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_lreturn() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_long()?;
        simulate(&mut stack, &constant_pool, &Instruction::Lreturn)?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_freturn() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_float()?;
        simulate(&mut stack, &constant_pool, &Instruction::Freturn)?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_dreturn() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_double()?;
        simulate(&mut stack, &constant_pool, &Instruction::Dreturn)?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_areturn() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        simulate(&mut stack, &constant_pool, &Instruction::Areturn)?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_getstatic() -> Result<()> {
        let mut stack = TypeStack::new();
        let mut constant_pool = ConstantPool::new();
        let class_index = constant_pool.add_class("java/lang/Object")?;
        let field_ref_index = constant_pool.add_field_ref(class_index, "hashCode", "I")?;
        simulate(
            &mut stack,
            &constant_pool,
            &Instruction::Getstatic(field_ref_index),
        )?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_putstatic() -> Result<()> {
        let mut stack = TypeStack::new();
        let mut constant_pool = ConstantPool::new();
        let class_index = constant_pool.add_class("java/lang/Object")?;
        let field_ref_index = constant_pool.add_field_ref(class_index, "hashCode", "I")?;
        stack.push_int()?;
        simulate(
            &mut stack,
            &constant_pool,
            &Instruction::Putstatic(field_ref_index),
        )?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_getfield() -> Result<()> {
        let mut stack = TypeStack::new();
        let mut constant_pool = ConstantPool::new();
        let class_index = constant_pool.add_class("java/lang/Object")?;
        let field_ref_index = constant_pool.add_field_ref(class_index, "hashCode", "I")?;
        stack.push_object()?;
        simulate(
            &mut stack,
            &constant_pool,
            &Instruction::Getfield(field_ref_index),
        )?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_putfield() -> Result<()> {
        let mut stack = TypeStack::new();
        let mut constant_pool = ConstantPool::new();
        let class_index = constant_pool.add_class("java/lang/Object")?;
        let field_ref_index = constant_pool.add_field_ref(class_index, "hashCode", "I")?;
        stack.push_object()?;
        stack.push_int()?;
        simulate(
            &mut stack,
            &constant_pool,
            &Instruction::Putfield(field_ref_index),
        )?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_invoke_instructions() -> Result<()> {
        let mut constant_pool = ConstantPool::new();
        let class_index = constant_pool.add_class("java/lang/Object")?;
        let method_index = constant_pool.add_method_ref(class_index, "foo", "(I)I")?;
        let instructions = vec![
            Instruction::Invokevirtual(method_index),
            Instruction::Invokespecial(method_index),
            Instruction::Invokestatic(method_index),
            Instruction::Invokedynamic(method_index),
        ];

        for instruction in instructions {
            let mut stack = TypeStack::new();
            stack.push_int()?;
            if !matches!(instruction, Instruction::Invokespecial(..))
                && !matches!(instruction, Instruction::Invokestatic(..))
            {
                stack.push_object()?;
            }
            simulate(&mut stack, &constant_pool, &instruction)?;
            assert_eq!(stack.len(), 1);
            assert!(stack.pop_int().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_simulate_invokeinterface() -> Result<()> {
        let mut stack = TypeStack::new();
        let mut constant_pool = ConstantPool::new();
        let class_index = constant_pool.add_class("java/lang/Object")?;
        let method_index = constant_pool.add_interface_method_ref(class_index, "foo", "(I)I")?;
        stack.push_int()?;
        stack.push_object()?;
        simulate(
            &mut stack,
            &constant_pool,
            &Instruction::Invokeinterface(method_index, 0),
        )?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_new() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        simulate(&mut stack, &constant_pool, &Instruction::New(3))?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_object().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_newarray() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        simulate(
            &mut stack,
            &constant_pool,
            &Instruction::Newarray(ArrayType::Int),
        )?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_object().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_anewarray() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        simulate(&mut stack, &constant_pool, &Instruction::Anewarray(3))?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_object().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_arraylength() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        simulate(&mut stack, &constant_pool, &Instruction::Arraylength)?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_athrow() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        simulate(&mut stack, &constant_pool, &Instruction::Athrow)?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_checkcast() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        simulate(&mut stack, &constant_pool, &Instruction::Checkcast(3))?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_object().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_instanceof() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        simulate(&mut stack, &constant_pool, &Instruction::Instanceof(3))?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_int().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_monitorenter() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        simulate(&mut stack, &constant_pool, &Instruction::Monitorenter)?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_monitorexit() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        simulate(&mut stack, &constant_pool, &Instruction::Monitorexit)?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_multianewarray() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_int()?;
        stack.push_int()?;
        simulate(
            &mut stack,
            &constant_pool,
            &Instruction::Multianewarray(2, 2),
        )?;
        assert_eq!(stack.len(), 1);
        assert!(stack.pop_object().is_ok());
        Ok(())
    }

    #[test]
    fn test_simulate_ifnull() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        simulate(&mut stack, &constant_pool, &Instruction::Ifnull(3))?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }

    #[test]
    fn test_simulate_ifnonnull() -> Result<()> {
        let mut stack = TypeStack::new();
        let constant_pool = ConstantPool::new();
        stack.push_object()?;
        simulate(&mut stack, &constant_pool, &Instruction::Ifnonnull(3))?;
        assert_eq!(stack.len(), 0);
        Ok(())
    }
}
