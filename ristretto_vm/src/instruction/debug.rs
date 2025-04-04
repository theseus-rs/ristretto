use crate::Result;
use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.2>
#[inline]
#[expect(clippy::unnecessary_wraps)]
pub(crate) fn breakpoint() -> Result<ExecutionResult> {
    // Breakpoint instruction is reserved for debugging and implementation dependent operations.
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.2>
#[inline]
#[expect(clippy::unnecessary_wraps)]
pub(crate) fn impdep1() -> Result<ExecutionResult> {
    // Impdep1 instruction is reserved for debugging and implementation dependent operations.
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.2>
#[inline]
#[expect(clippy::unnecessary_wraps)]
pub(crate) fn impdep2() -> Result<ExecutionResult> {
    // Impdep2 instruction is reserved for debugging and implementation dependent operations.
    Ok(Continue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint() -> Result<()> {
        assert_eq!(breakpoint()?, Continue);
        Ok(())
    }

    #[test]
    fn test_impdep1() -> Result<()> {
        assert_eq!(impdep1()?, Continue);
        Ok(())
    }

    #[test]
    fn test_impdep2() -> Result<()> {
        assert_eq!(impdep2()?, Continue);
        Ok(())
    }
}
