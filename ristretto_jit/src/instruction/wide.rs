use crate::Error::InternalError;
use crate::Result;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn wide() -> Result<()> {
    // The wide instruction is not directly used by this implementation.  The wide
    // versions of instructions are specifically enumerated in the instruction set.
    Err(InternalError("Wide instruction attempted".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wide() {
        let result = wide();
        assert!(matches!(result, Err(InternalError(_))));
    }
}
