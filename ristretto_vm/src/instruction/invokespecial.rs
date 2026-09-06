use crate::frame::{ExecutionResult, Frame, MethodCall};
use crate::instruction::resolve_method_ref;
use crate::method_ref_cache::InvokeKind;
use crate::operand_stack::OperandStack;
use crate::{Result, instruction};

/// Invokespecial instruction implementation.
///
/// # References
///
/// - [JVMS §6.5.invokespecial](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokespecial)
#[inline]
pub(crate) async fn invokespecial(
    frame: &Frame,
    stack: &mut OperandStack,
    method_index: u16,
) -> Result<ExecutionResult> {
    // Resolve the method with JPMS checks and caching
    let resolution = resolve_method_ref(frame, method_index, InvokeKind::Special).await?;

    instruction::receiver_class(stack.peek_at(resolution.param_count)?)?;

    // +1 for the receiver (this)
    let parameters = stack.drain_last(resolution.param_count + 1);
    Ok(ExecutionResult::Call(MethodCall {
        class: resolution.declaring_class.clone(),
        method: resolution.method.clone(),
        parameters,
        has_return_type: resolution.has_return_type,
    }))
}
