use crate::Error::InternalError;
use crate::function::activation::RequestKind;
use crate::instruction::{
    RequestContinuation, ThrowContext, emit_bci, emit_pending_exception_check, single_inst_result,
};
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use cranelift::codegen::ir::Value;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{InstBuilder, StackSlotData, StackSlotKind, types};

/// # References
///
/// - [JVMS §6.5.aastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.aastore)
pub(crate) fn aastore(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
    continuation: Option<RequestContinuation<'_>>,
) -> crate::Result<()> {
    let value = stack.pop_object(function_builder)?;
    let index = stack.pop_int(function_builder)?;
    let array_ref = stack.pop_object(function_builder)?;
    let bci = emit_bci(function_builder, throw_context);
    let synchronous = |builder: &mut FunctionBuilder| {
        builder.ins().call(
            helpers.aastore,
            &[context_pointer, bci, array_ref, index, value],
        );
        Ok(builder.ins().iconst(types::I64, 0))
    };
    if let Some(continuation) = continuation {
        let index = function_builder.ins().sextend(types::I64, index);
        continuation.emit_bits(
            function_builder,
            stack,
            (RequestKind::ReferenceArrayStore, 0),
            &[array_ref, index, value],
            synchronous,
        )?;
    } else {
        synchronous(function_builder)?;
    }
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
    continuation: Option<RequestContinuation<'_>>,
) -> crate::Result<()> {
    let bci = emit_bci(function_builder, throw_context);
    let class_index = function_builder
        .ins()
        .iconst(types::I32, i64::from(cp_class_index));
    let synchronous = |builder: &mut FunctionBuilder| {
        let call = builder
            .ins()
            .call(helpers.new_object, &[context_pointer, bci, class_index]);
        single_inst_result(builder, call)
    };
    let result = if let Some(continuation) = continuation {
        continuation.emit(
            function_builder,
            stack,
            (RequestKind::NewObject, cp_class_index),
            &[],
            synchronous,
        )?
    } else {
        synchronous(function_builder)?
    };
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
    continuation: Option<RequestContinuation<'_>>,
) -> crate::Result<()> {
    let count = stack.pop_int(function_builder)?;
    let bci = emit_bci(function_builder, throw_context);
    let class_index = function_builder
        .ins()
        .iconst(types::I32, i64::from(cp_class_index));
    let synchronous = |builder: &mut FunctionBuilder| {
        let call = builder.ins().call(
            helpers.anewarray,
            &[context_pointer, bci, class_index, count],
        );
        single_inst_result(builder, call)
    };
    let result = if let Some(continuation) = continuation {
        continuation.emit(
            function_builder,
            stack,
            (RequestKind::NewReferenceArray, cp_class_index),
            &[count],
            synchronous,
        )?
    } else {
        synchronous(function_builder)?
    };
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
#[expect(
    clippy::too_many_arguments,
    reason = "allocation instruction and suspension context"
)]
pub(crate) fn multianewarray(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
    cp_class_index: u16,
    dimensions: u8,
    continuation: Option<RequestContinuation<'_>>,
) -> crate::Result<()> {
    const DIMENSION_ALIGN_SHIFT: u8 = 2; // Each array length is a JVM int.
    if dimensions == 0 {
        return Err(InternalError(
            "multianewarray requires at least one dimension".to_string(),
        ));
    }
    let slot_size = u32::from(dimensions) * types::I32.bytes();
    let slot = function_builder.create_sized_stack_slot(StackSlotData::new(
        StackSlotKind::ExplicitSlot,
        slot_size,
        DIMENSION_ALIGN_SHIFT,
    ));
    let mut popped = Vec::with_capacity(usize::from(dimensions));
    for _ in 0..dimensions {
        popped.push(stack.pop_int(function_builder)?);
    }
    popped.reverse();
    let ptr_type = function_builder.func.dfg.value_type(context_pointer);
    // The last popped value is the outermost dimension. Store outermost first (index 0).
    for (i, value) in popped.iter().enumerate() {
        let offset = i32::try_from(i * size_of::<i32>())
            .map_err(|error| InternalError(format!("{error:?}")))?;
        function_builder
            .ins()
            .stack_store(ptr_type, *value, slot, offset);
    }
    let dims_ptr = function_builder.ins().stack_addr(ptr_type, slot, 0);
    let bci = emit_bci(function_builder, throw_context);
    let class_index = function_builder
        .ins()
        .iconst(types::I32, i64::from(cp_class_index));
    let dims_len = function_builder
        .ins()
        .iconst(types::I32, i64::from(dimensions));
    let synchronous = |builder: &mut FunctionBuilder| {
        let call = builder.ins().call(
            helpers.multianewarray,
            &[context_pointer, bci, class_index, dims_ptr, dims_len],
        );
        single_inst_result(builder, call)
    };
    let result = if let Some(continuation) = continuation {
        continuation.emit(
            function_builder,
            stack,
            (RequestKind::NewMultiArray, cp_class_index),
            &popped,
            synchronous,
        )?
    } else {
        synchronous(function_builder)?
    };
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
    continuation: Option<RequestContinuation<'_>>,
) -> crate::Result<()> {
    let top = stack.pop_object(function_builder)?;
    let bci = emit_bci(function_builder, throw_context);
    let class_index = function_builder
        .ins()
        .iconst(types::I32, i64::from(cp_class_index));
    let synchronous = |builder: &mut FunctionBuilder| {
        builder
            .ins()
            .call(helpers.checkcast, &[context_pointer, bci, top, class_index]);
        Ok(top)
    };
    let result = if let Some(continuation) = continuation {
        continuation.emit_bits(
            function_builder,
            stack,
            (RequestKind::CheckCast, cp_class_index),
            &[top],
            synchronous,
        )?
    } else {
        synchronous(function_builder)?
    };
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
/// - [JVMS §6.5.instanceof](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.instanceof)
pub(crate) fn instanceof(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
    cp_class_index: u16,
    continuation: Option<RequestContinuation<'_>>,
) -> crate::Result<()> {
    let object = stack.pop_object(function_builder)?;
    let bci = emit_bci(function_builder, throw_context);
    let class_index = function_builder
        .ins()
        .iconst(types::I32, i64::from(cp_class_index));
    let synchronous = |builder: &mut FunctionBuilder| {
        let call = builder.ins().call(
            helpers.instanceof,
            &[context_pointer, bci, object, class_index],
        );
        let result = single_inst_result(builder, call)?;
        Ok(builder.ins().uextend(types::I64, result))
    };
    let bits = if let Some(continuation) = continuation {
        continuation.emit_bits(
            function_builder,
            stack,
            (RequestKind::InstanceOf, cp_class_index),
            &[object],
            synchronous,
        )?
    } else {
        synchronous(function_builder)?
    };
    let result = function_builder.ins().ireduce(types::I32, bits);
    stack.push_int(function_builder, result)?;
    emit_pending_exception_check(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
    )
}
