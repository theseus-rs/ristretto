use crate::Result;
use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;

/// # References
///
/// - [JVMS §6.5.nop](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.nop)
#[inline]
#[expect(clippy::unnecessary_wraps)]
pub(crate) fn nop() -> Result<ExecutionResult> {
    Ok(Continue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop() -> Result<()> {
        assert_eq!(nop()?, Continue);
        Ok(())
    }
}
