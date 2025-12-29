use crate::Result;
use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::instruction::resolve_method_ref;
use crate::method_ref_cache::InvokeKind;
use crate::operand_stack::OperandStack;

/// Invokestatic instruction implementation.
///
/// # References
///
/// - [JVMS §6.5.invokestatic](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokestatic)
#[inline]
pub(crate) async fn invokestatic(
    frame: &Frame,
    stack: &mut OperandStack,
    method_index: u16,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;

    // Resolve the method with JPMS checks and caching
    let resolution = resolve_method_ref(frame, method_index, InvokeKind::Static).await?;

    let parameters = stack.drain_last(resolution.method.parameters().len());
    let result =
        Box::pin(thread.execute(&resolution.declaring_class, &resolution.method, &parameters))
            .await?;
    if let Some(value) = result {
        stack.push(value)?;
    }

    Ok(Continue)
}
