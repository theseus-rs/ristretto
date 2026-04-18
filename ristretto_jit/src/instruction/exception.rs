use crate::Error::{InternalError, InvalidBlockAddress};
use crate::Result;
use crate::instruction::{ThrowContext, emit_bci};
use crate::jit_value;
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use cranelift::codegen::ir::{Block, BlockArg, Value};
use cranelift::prelude::{FunctionBuilder, InstBuilder, MemFlags, types};
use ristretto_classfile::attributes::ExceptionTableEntry;

/// Prepares a throw target for `throw_context.program_counter`.
///
/// Returns `(block, is_new)`. If `is_new` is `true`, the caller is responsible for calling
/// [`populate_dispatch_block`] after terminating its source block with a branch to `block`.
/// If `is_new` is `false`, `block` is the shared exception return block and no further work
/// is required.
fn prepare_throw_target(
    function_builder: &mut FunctionBuilder,
    throw_context: &ThrowContext<'_>,
) -> Result<(Block, bool)> {
    let pc_u16 = u16::try_from(throw_context.program_counter).map_err(|_| {
        InternalError(format!(
            "program_counter {} overflows u16",
            throw_context.program_counter
        ))
    })?;
    let covers = throw_context
        .exception_table
        .iter()
        .any(|entry| entry.range_pc.contains(&pc_u16));
    if !covers {
        return Ok((throw_context.exception_block, false));
    }
    Ok((function_builder.create_block(), true))
}

/// Populates a previously created dispatch block for `throw_context.program_counter`. The
/// caller must have already terminated its source block with a branch to `dispatch_block`.
///
/// The dispatch block walks covering exception-table entries in order:
/// 1. For a catch-all (`catch_type == 0`) entry: takes the pending exception and jumps to the
///    handler with the exception reference as the single operand stack argument.
/// 2. For a typed entry: calls `jit_exception_matches` to test whether the pending exception
///    is assignable to the entry's `catch_type`; on match, takes the exception and jumps to
///    the handler; otherwise falls through to the next entry.
/// 3. If no entry matches, jumps to the shared exception return block (propagate).
///
/// After this function returns, the current block is one of the (terminated) blocks emitted
/// inside the dispatch body; the caller should switch to its intended continuation block.
fn populate_dispatch_block(
    function_builder: &mut FunctionBuilder,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
    dispatch_block: Block,
) -> Result<()> {
    let pc_u16 = u16::try_from(throw_context.program_counter).map_err(|_| {
        InternalError(format!(
            "program_counter {} overflows u16",
            throw_context.program_counter
        ))
    })?;
    let covering: Vec<&ExceptionTableEntry> = throw_context
        .exception_table
        .iter()
        .filter(|entry| entry.range_pc.contains(&pc_u16))
        .collect();

    function_builder.switch_to_block(dispatch_block);

    for entry in covering {
        let handler_pc = usize::from(entry.handler_pc);
        let handler_block = *throw_context
            .blocks
            .get(&handler_pc)
            .ok_or(InvalidBlockAddress(handler_pc))?;
        if entry.catch_type == 0 {
            let call = function_builder
                .ins()
                .call(helpers.take_pending_exception, &[context_pointer]);
            let Some(&exception_ref) = function_builder.inst_results(call).first() else {
                return Err(InternalError(
                    "take_pending_exception returned no value".to_string(),
                ));
            };
            function_builder
                .ins()
                .jump(handler_block, &[BlockArg::Value(exception_ref)]);
            return Ok(());
        }

        let catch_type = function_builder
            .ins()
            .iconst(types::I32, i64::from(entry.catch_type));
        let call = function_builder
            .ins()
            .call(helpers.exception_matches, &[context_pointer, catch_type]);
        let Some(&matches) = function_builder.inst_results(call).first() else {
            return Err(InternalError(
                "exception_matches returned no value".to_string(),
            ));
        };

        let take_block = function_builder.create_block();
        let next_block = function_builder.create_block();
        function_builder
            .ins()
            .brif(matches, take_block, &[], next_block, &[]);

        function_builder.switch_to_block(take_block);
        let call = function_builder
            .ins()
            .call(helpers.take_pending_exception, &[context_pointer]);
        let Some(&exception_ref) = function_builder.inst_results(call).first() else {
            return Err(InternalError(
                "take_pending_exception returned no value".to_string(),
            ));
        };
        function_builder
            .ins()
            .jump(handler_block, &[BlockArg::Value(exception_ref)]);

        function_builder.switch_to_block(next_block);
    }

    function_builder
        .ins()
        .jump(throw_context.exception_block, &[]);
    Ok(())
}

/// Emits a check of the pending exception slot on `context`. If an exception is pending, control
/// branches to a per PC dispatch block that routes to a matching handler or propagates to the
/// shared exception return block; otherwise control continues in a fresh block with the same
/// operand stack arguments as before.
///
/// This routine must be called after every JIT emitted helper call that can throw. The current
/// operand stack is passed as arguments to the continuation block so the JIT's SSA form remains
/// consistent across the branch.
pub(crate) fn emit_pending_exception_check(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
) -> Result<()> {
    let call = function_builder
        .ins()
        .call(helpers.pending_exception, &[context_pointer]);
    let Some(&pending) = function_builder.inst_results(call).first() else {
        return Err(InternalError(
            "pending_exception helper returned no value".to_string(),
        ));
    };

    let (throw_target, is_new) = prepare_throw_target(function_builder, throw_context)?;

    let continue_block = function_builder.create_block();
    for ty in stack.to_type_vec(function_builder) {
        function_builder.append_block_param(continue_block, ty);
    }

    let stack_args = stack.as_block_arguments();
    function_builder
        .ins()
        .brif(pending, throw_target, &[], continue_block, &stack_args);

    if is_new {
        populate_dispatch_block(
            function_builder,
            helpers,
            context_pointer,
            throw_context,
            throw_target,
        )?;
    }

    function_builder.switch_to_block(continue_block);
    stack.reset(function_builder)?;
    Ok(())
}

/// Emits a null pointer check on `value_to_check`. If the value is zero, stores a
/// `NullPointerException` in the pending exception slot and branches to the PC appropriate
/// dispatch block (which routes to a matching catch handler or propagates). Otherwise,
/// control continues in a fresh block with the same operand-stack arguments as before.
///
/// This helper is used for array accesses (`aload`, `astore`, `arraylength`) where the
/// underlying runtime helper does not accept a context parameter and therefore cannot
/// signal an NPE itself.
pub(crate) fn emit_null_check(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
    value_to_check: Value,
) -> Result<()> {
    let null_block = function_builder.create_block();
    let continue_block = function_builder.create_block();
    for ty in stack.to_type_vec(function_builder) {
        function_builder.append_block_param(continue_block, ty);
    }
    let stack_args = stack.as_block_arguments();
    function_builder
        .ins()
        .brif(value_to_check, continue_block, &stack_args, null_block, &[]);

    function_builder.switch_to_block(null_block);
    let bci_value = function_builder.ins().iconst(
        types::I32,
        i64::try_from(throw_context.program_counter).unwrap_or(0),
    );
    function_builder
        .ins()
        .call(helpers.throw_npe, &[context_pointer, bci_value]);
    let (throw_target, is_new) = prepare_throw_target(function_builder, throw_context)?;
    function_builder.ins().jump(throw_target, &[]);
    if is_new {
        populate_dispatch_block(
            function_builder,
            helpers,
            context_pointer,
            throw_context,
            throw_target,
        )?;
    }

    function_builder.switch_to_block(continue_block);
    stack.reset(function_builder)?;
    Ok(())
}

/// Emits the body of the shared exception return block: write a `NONE` discriminant to the
/// return pointer and return from the function. Callers must have switched to `exception_block`
/// before invoking this.
pub(crate) fn emit_exception_return(function_builder: &mut FunctionBuilder, return_pointer: Value) {
    let none_discriminant = function_builder
        .ins()
        .iconst(types::I8, i64::from(jit_value::NONE));
    function_builder
        .ins()
        .store(MemFlags::new(), none_discriminant, return_pointer, 0);
    function_builder.ins().return_(&[]);
}

/// Emit the `athrow` instruction: store the popped reference as the pending exception and
/// branch to the PC appropriate dispatch block (handler or propagate).
///
/// # References
/// - [JVMS §6.5.athrow](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.athrow)
pub(crate) fn athrow(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
) -> Result<()> {
    let throwable = stack.pop_object(function_builder)?;
    let bci = emit_bci(function_builder, throw_context);
    function_builder
        .ins()
        .call(helpers.athrow, &[context_pointer, bci, throwable]);

    let (throw_target, is_new) = prepare_throw_target(function_builder, throw_context)?;
    function_builder.ins().jump(throw_target, &[]);
    if is_new {
        populate_dispatch_block(
            function_builder,
            helpers,
            context_pointer,
            throw_context,
            throw_target,
        )?;
    }
    Ok(())
}
