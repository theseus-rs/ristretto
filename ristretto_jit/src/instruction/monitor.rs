use crate::Result;
use crate::compiler::continuation::CallContinuation;
use crate::function::activation::RequestKind;
use crate::instruction::{ThrowContext, emit_bci, emit_pending_exception_check};
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use cranelift::codegen::ir::{StackSlot, Value};
use cranelift::prelude::{FunctionBuilder, InstBuilder, types};

/// Return before awaiting a monitor, preserving the exact post-instruction state.
/// The synchronous embedding API uses the same runtime semantics through a helper.
#[expect(
    clippy::too_many_arguments,
    reason = "instruction and continuation emission context"
)]
pub(crate) fn monitor(
    builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context: Value,
    throw: &ThrowContext<'_>,
    arguments: StackSlot,
    enter: bool,
    continuation: Option<CallContinuation<'_>>,
) -> Result<()> {
    let object = stack.pop_object(builder)?;
    let pointer_type = builder.func.dfg.value_type(context);
    builder
        .ins()
        .stack_store(pointer_type, object, arguments, 0);
    let bci = emit_bci(builder, throw);
    let helper = if enter {
        helpers.monitorenter
    } else {
        helpers.monitorexit
    };
    let synchronous = |builder: &mut FunctionBuilder| {
        builder.ins().call(helper, &[context, bci, object]);
        Ok(builder.ins().iconst(types::I64, 0))
    };
    if let Some(continuation) = continuation {
        continuation.emit(
            builder,
            stack,
            arguments,
            1,
            throw.program_counter,
            (
                if enter {
                    RequestKind::MonitorEnter
                } else {
                    RequestKind::MonitorExit
                },
                0,
            ),
            synchronous,
        )?;
    } else {
        synchronous(builder)?;
    }
    emit_pending_exception_check(builder, stack, helpers, context, throw)
}
