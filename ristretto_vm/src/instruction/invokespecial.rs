use crate::Result;
use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::instruction::resolve_method_ref;
use crate::method_ref_cache::InvokeKind;
use crate::operand_stack::OperandStack;

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
    let thread = frame.thread()?;

    // Resolve the method with JPMS checks and caching
    let resolution = resolve_method_ref(frame, method_index, InvokeKind::Special).await?;

    let parameters = stack.drain_last(resolution.method.parameters().len() + 1);
    let result =
        Box::pin(thread.execute(&resolution.declaring_class, &resolution.method, &parameters))
            .await?;
    if let Some(result) = result {
        stack.push(result)?;
    }
    Ok(Continue)
}
