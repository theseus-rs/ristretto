use crate::Result;
use crate::frame::{ExecutionResult, Frame, MethodCall};
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
    // Resolve the method with JPMS checks and caching
    let resolution = resolve_method_ref(frame, method_index, InvokeKind::Static).await?;

    if !resolution.declaring_class.is_initialized()? {
        frame
            .thread()?
            .initialize_class(&resolution.declaring_class)
            .await?;
    }

    let parameters = stack.drain_last(resolution.param_count);
    Ok(ExecutionResult::Call(MethodCall {
        class: resolution.declaring_class.clone(),
        method: resolution.method.clone(),
        parameters,
        has_return_type: resolution.has_return_type,
    }))
}
