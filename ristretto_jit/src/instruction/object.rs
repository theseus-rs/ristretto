use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use cranelift::codegen::ir::FuncRef;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::InstBuilder;

/// Emits a call to a runtime helper for loading an element from a GC-managed array.
/// The helper is called with (`array_ptr`: i64, `index`: i32) and returns the element value.
///
/// The returned Cranelift `Value` has the type determined by the helper's return signature.
pub(crate) fn array_load(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helper: FuncRef,
) -> crate::Result<cranelift::codegen::ir::Value> {
    let index = stack.pop_int(function_builder)?;
    let array_ref = stack.pop_object(function_builder)?;
    let call = function_builder.ins().call(helper, &[array_ref, index]);
    Ok(function_builder.inst_results(call)[0])
}

/// Emits a call to a runtime helper for storing an element into a GC managed array.
/// The helper is called with (`array_ptr`: i64, `index`: i32, `value`: T).
pub(crate) fn array_store(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helper: FuncRef,
    value: cranelift::codegen::ir::Value,
) -> crate::Result<()> {
    let index = stack.pop_int(function_builder)?;
    let array_ref = stack.pop_object(function_builder)?;
    function_builder
        .ins()
        .call(helper, &[array_ref, index, value]);
    Ok(())
}

/// Helper for loading and pushing an int-typed array element onto the stack.
pub(crate) fn array_load_int(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    helper: FuncRef,
) -> crate::Result<()> {
    let _ = helpers; // used for type context only
    let value = array_load(function_builder, stack, helper)?;
    stack.push_int(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.aaload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.aaload)
pub(crate) fn aaload(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
) -> crate::Result<()> {
    let value = array_load(function_builder, stack, helpers.aaload)?;
    stack.push_object(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.aastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.aastore)
pub(crate) fn aastore(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
) -> crate::Result<()> {
    let value = stack.pop_object(function_builder)?;
    array_store(function_builder, stack, helpers.aastore, value)
}
