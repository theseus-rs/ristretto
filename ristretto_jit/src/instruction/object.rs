use crate::operand_stack::OperandStack;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{InstBuilder, MemFlags, types};

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.aload>
pub(crate) fn aload(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    element_type: types::Type,
    element_size: i32,
    sign_extend: bool,
    zero_extend: bool,
) -> crate::Result<()> {
    let index = stack.pop_int(function_builder)?;
    let array_ref = stack.pop_long(function_builder)?;

    // Calculate address: array_ref + 8 + index * element_size
    let header_size = function_builder.ins().iconst(types::I64, 8);
    let index_i64 = function_builder.ins().uextend(types::I64, index);
    let element_size_val = function_builder
        .ins()
        .iconst(types::I64, i64::from(element_size));
    let offset = function_builder.ins().imul(index_i64, element_size_val);
    let data_offset = function_builder.ins().iadd(header_size, offset);
    let address = function_builder.ins().iadd(array_ref, data_offset);

    let mut value = function_builder
        .ins()
        .load(element_type, MemFlags::trusted(), address, 0);

    match element_type {
        types::I32 => stack.push_int(function_builder, value)?,
        types::I64 => stack.push_long(function_builder, value)?,
        types::F32 => stack.push_float(function_builder, value)?,
        types::F64 => stack.push_double(function_builder, value)?,
        types::I8 | types::I16 => {
            if sign_extend {
                value = function_builder.ins().sextend(types::I32, value);
            } else if zero_extend {
                value = function_builder.ins().uextend(types::I32, value);
            }
            stack.push_int(function_builder, value)?;
        }
        _ => {
            return Err(crate::Error::InternalError(format!(
                "Unsupported element type {element_type}"
            )));
        }
    }

    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.astore>
pub(crate) fn astore(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    element_type: types::Type,
    element_size: i32,
) -> crate::Result<()> {
    let value = match element_type {
        types::F32 => stack.pop_float(function_builder)?,
        types::F64 => stack.pop_double(function_builder)?,
        types::I64 => stack.pop_long(function_builder)?,
        types::I8 | types::I16 => {
            let val = stack.pop_int(function_builder)?;
            function_builder.ins().ireduce(element_type, val)
        }
        _ => stack.pop_int(function_builder)?,
    };

    let index = stack.pop_int(function_builder)?;
    let array_ref = stack.pop_long(function_builder)?;

    let header_size = function_builder.ins().iconst(types::I64, 8);
    let index_i64 = function_builder.ins().uextend(types::I64, index);
    let element_size_val = function_builder
        .ins()
        .iconst(types::I64, i64::from(element_size));
    let offset = function_builder.ins().imul(index_i64, element_size_val);
    let data_offset = function_builder.ins().iadd(header_size, offset);
    let address = function_builder.ins().iadd(array_ref, data_offset);

    function_builder
        .ins()
        .store(MemFlags::trusted(), value, address, 0);
    Ok(())
}
