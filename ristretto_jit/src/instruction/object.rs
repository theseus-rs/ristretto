use crate::Error::InternalError;
use crate::instruction::{ThrowContext, emit_bci, emit_null_check, emit_pending_exception_check};
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use cranelift::codegen::ir::FuncRef;
use cranelift::codegen::ir::Value;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{InstBuilder, StackSlotData, StackSlotKind, types};

/// Emits a call to a runtime helper for loading an element from a GC-managed array.
/// The helper is called with (`ctx`: ptr, `bci`: i32, `array_ptr`: i64, `index`: i32) and
/// returns the element value.
///
/// A null check is emitted before the call; a null receiver branches to the exception
/// dispatch with a `NullPointerException` in the pending exception slot. After the helper
/// returns, a pending-exception check is emitted so a runtime-detected error (e.g. internal
/// type mismatch surfaced as `InternalError`) is routed to the dispatch block instead of
/// aborting the host process.
///
/// The returned Cranelift `Value` has the type determined by the helper's return signature
/// and is dominated by the call site, so callers may push it onto the operand stack after
/// this function returns (i.e. on the post-check continue block).
pub(crate) fn array_load(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
    helper: FuncRef,
) -> crate::Result<cranelift::codegen::ir::Value> {
    let index = stack.pop_int(function_builder)?;
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
        .call(helper, &[context_pointer, bci, array_ref, index]);
    let value = function_builder.inst_results(call)[0];
    emit_pending_exception_check(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
    )?;
    Ok(value)
}

/// Emits a call to a runtime helper for storing an element into a GC managed array.
/// The helper is called with (`ctx`: ptr, `bci`: i32, `array_ptr`: i64, `index`: i32, `value`: T).
///
/// A null check is emitted before the call; a null receiver branches to the exception
/// dispatch with a `NullPointerException` in the pending exception slot. After the helper
/// returns, a pending-exception check is emitted so runtime-detected errors are routed to
/// the dispatch block instead of aborting the host process.
pub(crate) fn array_store(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
    helper: FuncRef,
    value: cranelift::codegen::ir::Value,
) -> crate::Result<()> {
    let index = stack.pop_int(function_builder)?;
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
    function_builder
        .ins()
        .call(helper, &[context_pointer, bci, array_ref, index, value]);
    emit_pending_exception_check(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
    )
}

/// Helper for loading and pushing an int-typed array element onto the stack.
pub(crate) fn array_load_int(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
    helper: FuncRef,
) -> crate::Result<()> {
    let value = array_load(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
        helper,
    )?;
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
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
) -> crate::Result<()> {
    let value = array_load(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
        helpers.aaload,
    )?;
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
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
) -> crate::Result<()> {
    let value = stack.pop_object(function_builder)?;
    let index = stack.pop_int(function_builder)?;
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
    function_builder.ins().call(
        helpers.aastore,
        &[context_pointer, bci, array_ref, index, value],
    );
    emit_pending_exception_check(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
    )
}

/// Emits a call to `jit_new` to allocate a new uninitialized object instance of the class
/// referenced by `cp_class_index`. The freshly allocated reference is pushed onto the stack.
///
/// # References
///
/// - [JVMS §6.5.new](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.new)
pub(crate) fn new_object(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
    cp_class_index: u16,
) -> crate::Result<()> {
    let bci = emit_bci(function_builder, throw_context);
    let class_index = function_builder
        .ins()
        .iconst(types::I32, i64::from(cp_class_index));
    let call = function_builder
        .ins()
        .call(helpers.new_object, &[context_pointer, bci, class_index]);
    let result = function_builder.inst_results(call)[0];
    stack.push_object(function_builder, result)?;
    emit_pending_exception_check(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
    )
}

/// # References
///
/// - [JVMS §6.5.anewarray](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.anewarray)
pub(crate) fn anewarray(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
    cp_class_index: u16,
) -> crate::Result<()> {
    let count = stack.pop_int(function_builder)?;
    let bci = emit_bci(function_builder, throw_context);
    let class_index = function_builder
        .ins()
        .iconst(types::I32, i64::from(cp_class_index));
    let call = function_builder.ins().call(
        helpers.anewarray,
        &[context_pointer, bci, class_index, count],
    );
    let result = function_builder.inst_results(call)[0];
    stack.push_object(function_builder, result)?;
    emit_pending_exception_check(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
    )
}

/// # References
///
/// - [JVMS §6.5.multianewarray](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.multianewarray)
pub(crate) fn multianewarray(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
    cp_class_index: u16,
    dimensions: u8,
) -> crate::Result<()> {
    if dimensions == 0 {
        return Err(InternalError(
            "multianewarray requires at least one dimension".to_string(),
        ));
    }
    let slot_size = u32::from(dimensions) * 4;
    let slot = function_builder.create_sized_stack_slot(StackSlotData::new(
        StackSlotKind::ExplicitSlot,
        slot_size,
        2,
    ));
    let mut popped = Vec::with_capacity(usize::from(dimensions));
    for _ in 0..dimensions {
        popped.push(stack.pop_int(function_builder)?);
    }
    // The last popped value is the outermost dimension. Store outermost first (index 0).
    for (i, value) in popped.iter().rev().enumerate() {
        let offset = i32::try_from(i * 4).map_err(|error| InternalError(format!("{error:?}")))?;
        function_builder.ins().stack_store(*value, slot, offset);
    }
    let ptr_type = function_builder.func.dfg.value_type(context_pointer);
    let dims_ptr = function_builder.ins().stack_addr(ptr_type, slot, 0);
    let bci = emit_bci(function_builder, throw_context);
    let class_index = function_builder
        .ins()
        .iconst(types::I32, i64::from(cp_class_index));
    let dims_len = function_builder
        .ins()
        .iconst(types::I32, i64::from(dimensions));
    let call = function_builder.ins().call(
        helpers.multianewarray,
        &[context_pointer, bci, class_index, dims_ptr, dims_len],
    );
    let result = function_builder.inst_results(call)[0];
    stack.push_object(function_builder, result)?;
    emit_pending_exception_check(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
    )
}

/// Emits the `checkcast` bytecode.
///
/// On success the operand stack is unchanged (the original reference remains on
/// the top); on failure the JIT runtime helper records a `ClassCastException` in
/// the pending-exception slot and we branch to the throw target.
///
/// Stack-shape invariant for helpers that may throw: any *non-exception* stack
/// effect (here, re-pushing `top`) is performed **before**
/// `emit_pending_exception_check`. The throw-target block does not consume any
/// extra arguments off the operand stack;it only reads the pending exception
/// slot;so the extra push on the throw path is a harmless no-op. Reordering
/// (i.e., checking the exception before the push) would leave the operand stack
/// inconsistent on the success path. Apply the same pattern to any future
/// helper that both produces a result and may throw.
///
/// # References
///
/// - [JVMS §6.5.checkcast](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.checkcast)
pub(crate) fn checkcast(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
    cp_class_index: u16,
) -> crate::Result<()> {
    let top = stack.pop_object(function_builder)?;
    let bci = emit_bci(function_builder, throw_context);
    let class_index = function_builder
        .ins()
        .iconst(types::I32, i64::from(cp_class_index));
    function_builder
        .ins()
        .call(helpers.checkcast, &[context_pointer, bci, top, class_index]);
    stack.push_object(function_builder, top)?;
    emit_pending_exception_check(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
    )
}

/// # References
///
/// - [JVMS §6.5.instanceof](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.instanceof)
pub(crate) fn instanceof(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
    cp_class_index: u16,
) -> crate::Result<()> {
    let object = stack.pop_object(function_builder)?;
    let bci = emit_bci(function_builder, throw_context);
    let class_index = function_builder
        .ins()
        .iconst(types::I32, i64::from(cp_class_index));
    let call = function_builder.ins().call(
        helpers.instanceof,
        &[context_pointer, bci, object, class_index],
    );
    let result = function_builder.inst_results(call)[0];
    stack.push_int(function_builder, result)?;
    emit_pending_exception_check(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
    )
}
