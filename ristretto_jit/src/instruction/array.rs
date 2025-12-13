use crate::Result;
use crate::operand_stack::OperandStack;
use cranelift::codegen::ir::{FuncRef, MemFlags};
use cranelift::prelude::{FunctionBuilder, InstBuilder, types};
use ristretto_classfile::attributes::ArrayType;

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.newarray>
pub(crate) fn newarray(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    atype: &ArrayType,
    malloc: FuncRef,
) -> Result<()> {
    let count = stack.pop_int(function_builder)?;
    let element_size = match atype {
        ArrayType::Boolean | ArrayType::Byte => 1,
        ArrayType::Char | ArrayType::Short => 2,
        ArrayType::Float | ArrayType::Int => 4,
        ArrayType::Double | ArrayType::Long => 8,
    };

    let element_size_val = function_builder.ins().iconst(types::I64, element_size);
    // Extend count to I64 for calculation
    let count_i64 = function_builder.ins().uextend(types::I64, count);

    // Calculate total size: 8 (length header) + count * element_size
    let data_size = function_builder.ins().imul(count_i64, element_size_val);
    let header_size = function_builder.ins().iconst(types::I64, 8);
    let total_size = function_builder.ins().iadd(header_size, data_size);

    // Call malloc
    let call = function_builder.ins().call(malloc, &[total_size]);
    let array_ptr = function_builder.inst_results(call)[0];

    // Store length at the beginning
    function_builder
        .ins()
        .store(MemFlags::trusted(), count_i64, array_ptr, 0);

    // Push array pointer to stack (as I64/Long)
    stack.push_long(function_builder, array_ptr)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.arraylength>
pub(crate) fn arraylength(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let array_ref = stack.pop_long(function_builder)?;
    // Eventually will need to check for null (0); for now, assume non-null or let it segfault/trap.

    // Load length from offset 0
    // Length is stored as I64, but arraylength returns int (I32).
    let length_i64 = function_builder
        .ins()
        .load(types::I64, MemFlags::trusted(), array_ref, 0);
    let length = function_builder.ins().ireduce(types::I32, length_i64);

    stack.push_int(function_builder, length)?;
    Ok(())
}
