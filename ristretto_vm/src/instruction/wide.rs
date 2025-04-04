use crate::Error::InvalidOperand;
use crate::Result;
use crate::frame::ExecutionResult;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn wide() -> Result<ExecutionResult> {
    // The wide instruction is not directly used by this implementation.  The wide
    // versions of instructions are specifically enumerated in the instruction set.
    Err(InvalidOperand {
        expected: "*_w instruction".to_string(),
        actual: "Wide".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wide() {
        let result = wide();
        assert!(matches!(
            result,
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "*_w instruction" && actual == "Wide"
        ));
    }
}
