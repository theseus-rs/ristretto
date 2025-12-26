use crate::verifiers::error::{Result, VerifyError};
use crate::verifiers::types::VerificationType;

/// Represents a stack frame used during bytecode verification. Each frame contains local variables
/// and an operand stack. The locals and stack are represented as vectors of `VerificationType`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Frame {
    pub locals: Vec<VerificationType>,
    pub stack: Vec<VerificationType>,
    pub max_stack: usize,
}

impl Frame {
    #[must_use]
    pub(crate) fn new(max_locals: usize, max_stack: usize) -> Self {
        Self {
            locals: vec![VerificationType::Top; max_locals],
            stack: Vec::with_capacity(max_stack),
            max_stack,
        }
    }

    /// Push a verification type onto the operand stack.
    ///
    /// # Errors
    /// Returns `VerifyError::VerifyError` if the operand stack overflows.
    pub(crate) fn push(&mut self, ty: VerificationType) -> Result<()> {
        if self.stack.len() >= self.max_stack {
            return Err(VerifyError::VerifyError(
                "Operand stack overflow".to_string(),
            ));
        }
        self.stack.push(ty);
        Ok(())
    }

    /// Pop a verification type from the operand stack.
    ///
    /// # Errors
    /// Returns `VerifyError::VerifyError` if the operand stack is empty.
    pub(crate) fn pop(&mut self) -> Result<VerificationType> {
        self.stack
            .pop()
            .ok_or_else(|| VerifyError::VerifyError("Operand stack underflow".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let frame = Frame::new(2, 5);
        assert_eq!(frame.locals.len(), 2);
        assert_eq!(frame.locals[0], VerificationType::Top);
        assert_eq!(frame.locals[1], VerificationType::Top);
        assert!(frame.stack.is_empty());
        assert_eq!(frame.max_stack, 5);
    }

    #[test]
    fn test_push_pop_success() -> Result<()> {
        let mut frame = Frame::new(0, 2);
        frame.push(VerificationType::Integer)?;
        assert_eq!(frame.stack.len(), 1);
        assert_eq!(frame.stack[0], VerificationType::Integer);

        frame.push(VerificationType::Float)?;
        assert_eq!(frame.stack.len(), 2);

        let val = frame.pop()?;
        assert_eq!(val, VerificationType::Float);
        assert_eq!(frame.stack.len(), 1);

        let val = frame.pop()?;
        assert_eq!(val, VerificationType::Integer);
        assert!(frame.stack.is_empty());

        Ok(())
    }

    #[test]
    fn test_push_overflow() {
        let mut frame = Frame::new(10, 1);
        assert!(frame.push(VerificationType::Integer).is_ok());
        let result = frame.push(VerificationType::Integer);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            VerifyError::VerifyError("Operand stack overflow".to_string())
        );
    }

    #[test]
    fn test_pop_underflow() {
        let mut frame = Frame::new(10, 1);
        let result = frame.pop();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            VerifyError::VerifyError("Operand stack underflow".to_string())
        );
    }
}
