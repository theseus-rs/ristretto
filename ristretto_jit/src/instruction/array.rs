use crate::Result;
use crate::instruction::{ThrowContext, emit_bci, emit_null_check, emit_pending_exception_check};
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use cranelift::codegen::ir::Value;
use cranelift::prelude::{FunctionBuilder, InstBuilder};
use ristretto_classfile::attributes::ArrayType;

/// # References
///
/// - [JVMS §6.5.newarray](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.newarray)
pub(crate) fn newarray(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    atype: &ArrayType,
    context_pointer: Value,
    helpers: &RuntimeHelpers,
) -> Result<()> {
    let count = stack.pop_int(function_builder)?;

    let helper = match atype {
        ArrayType::Boolean => helpers.new_bool_array,
        ArrayType::Byte => helpers.new_byte_array,
        ArrayType::Char => helpers.new_char_array,
        ArrayType::Short => helpers.new_short_array,
        ArrayType::Int => helpers.new_int_array,
        ArrayType::Long => helpers.new_long_array,
        ArrayType::Float => helpers.new_float_array,
        ArrayType::Double => helpers.new_double_array,
    };

    let call = function_builder
        .ins()
        .call(helper, &[context_pointer, count]);
    let array_ptr = function_builder.inst_results(call)[0];

    // Push as object reference (Gc pointer encoded as i64)
    stack.push_object(function_builder, array_ptr)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.arraylength](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.arraylength)
pub(crate) fn arraylength(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
) -> Result<()> {
    let array_ref = stack.pop_object(function_builder)?;
    emit_null_check(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
        array_ref,
    )?;
    let bci = emit_bci(function_builder, throw_context);
    let call = function_builder
        .ins()
        .call(helpers.arraylength, &[context_pointer, bci, array_ref]);
    let length = function_builder.inst_results(call)[0];
    emit_pending_exception_check(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
    )?;
    stack.push_int(function_builder, length)?;
    Ok(())
}
