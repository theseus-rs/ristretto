use crate::Result;
use crate::operand_stack::OperandStack;
use cranelift::codegen::ir::Value;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::types;

/// Determine if a value is a category 1 value.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.11.1-320>
fn is_category_1(function_builder: &mut FunctionBuilder, value: Value) -> bool {
    let value_type = function_builder.func.dfg.value_type(value);
    value_type != types::I64 && value_type != types::F64
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.pop>
pub(crate) fn pop(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let _ = stack.pop(function_builder)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.pop2>
pub(crate) fn pop2(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value = stack.pop(function_builder)?;
    if is_category_1(function_builder, value) {
        let _ = stack.pop(function_builder)?;
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dup>
pub(crate) fn dup(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value = stack.pop(function_builder)?;
    stack.push(function_builder, value)?;
    stack.push(function_builder, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dup_x1>
pub(crate) fn dup_x1(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let value1 = stack.pop(function_builder)?;
    let value2 = stack.pop(function_builder)?;
    stack.push(function_builder, value1)?;
    stack.push(function_builder, value2)?;
    stack.push(function_builder, value1)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dup_x2>
pub(crate) fn dup_x2(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let value1 = stack.pop(function_builder)?;
    let value2 = stack.pop(function_builder)?;
    if is_category_1(function_builder, value2) {
        let value3 = stack.pop(function_builder)?;
        stack.push(function_builder, value1)?;
        stack.push(function_builder, value3)?;
        stack.push(function_builder, value2)?;
        stack.push(function_builder, value1)?;
    } else {
        stack.push(function_builder, value1)?;
        stack.push(function_builder, value2)?;
        stack.push(function_builder, value1)?;
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dup2>
pub(crate) fn dup2(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value1 = stack.pop(function_builder)?;
    if is_category_1(function_builder, value1) {
        let value2 = stack.pop(function_builder)?;
        stack.push(function_builder, value2)?;
        stack.push(function_builder, value1)?;
        stack.push(function_builder, value2)?;
        stack.push(function_builder, value1)?;
    } else {
        stack.push(function_builder, value1)?;
        stack.push(function_builder, value1)?;
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dup2_x1>
pub(crate) fn dup2_x1(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let value1 = stack.pop(function_builder)?;
    let value2 = stack.pop(function_builder)?;
    if is_category_1(function_builder, value1) {
        let value3 = stack.pop(function_builder)?;
        stack.push(function_builder, value2)?;
        stack.push(function_builder, value1)?;
        stack.push(function_builder, value3)?;
        stack.push(function_builder, value2)?;
        stack.push(function_builder, value1)?;
    } else {
        stack.push(function_builder, value1)?;
        stack.push(function_builder, value2)?;
        stack.push(function_builder, value1)?;
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dup2_x2>
pub(crate) fn dup2_x2(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let value1 = stack.pop(function_builder)?;
    let value2 = stack.pop(function_builder)?;
    if is_category_1(function_builder, value1) {
        let value3 = stack.pop(function_builder)?;
        if is_category_1(function_builder, value3) {
            let value4 = stack.pop(function_builder)?;
            stack.push(function_builder, value2)?;
            stack.push(function_builder, value1)?;
            stack.push(function_builder, value4)?;
        } else {
            stack.push(function_builder, value1)?;
        }
        stack.push(function_builder, value3)?;
        stack.push(function_builder, value2)?;
        stack.push(function_builder, value1)?;
    } else {
        if is_category_1(function_builder, value2) {
            let value3 = stack.pop(function_builder)?;
            stack.push(function_builder, value1)?;
            stack.push(function_builder, value3)?;
        } else {
            stack.push(function_builder, value1)?;
        }
        stack.push(function_builder, value2)?;
        stack.push(function_builder, value1)?;
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.swap>
pub(crate) fn swap(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    // Swapping category 2 values (Double and Long) is not supported by the JVM specification and
    // there is no mention of what should happen in this case. We will just swap the values and
    // ignore the fact that category 2 values could be swapped here.
    let value1 = stack.pop(function_builder)?;
    let value2 = stack.pop(function_builder)?;
    stack.push(function_builder, value1)?;
    stack.push(function_builder, value2)?;
    Ok(())
}
