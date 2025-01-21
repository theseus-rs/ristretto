use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.invokedynamic>
#[inline]
pub(crate) async fn invokedynamic(
    _frame: &Frame,
    _stack: &mut OperandStack,
    _method_index: u16,
) -> crate::Result<ExecutionResult> {
    todo!("invokedynamic")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "invokedynamic")]
    async fn test_invokedynamic() {
        let (_vm, _thread, frame) = crate::test::frame().await.expect("frame");
        let mut stack = OperandStack::with_max_size(1);
        let method_index = 0;
        let _ = invokedynamic(&frame, &mut stack, method_index).await;
    }
}
