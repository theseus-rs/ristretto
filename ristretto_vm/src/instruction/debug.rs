use crate::Result;

/// # References
///
/// - [JVMS §6.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.2)
#[inline]
#[expect(clippy::unnecessary_wraps)]
pub(crate) fn breakpoint() -> Result<()> {
    // Breakpoint instruction is reserved for debugging and implementation dependent operations.
    Ok(())
}

/// # References
///
/// - [JVMS §6.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.2)
#[inline]
#[expect(clippy::unnecessary_wraps)]
pub(crate) fn impdep1() -> Result<()> {
    // Impdep1 instruction is reserved for debugging and implementation dependent operations.
    Ok(())
}

/// # References
///
/// - [JVMS §6.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.2)
#[inline]
#[expect(clippy::unnecessary_wraps)]
pub(crate) fn impdep2() -> Result<()> {
    // Impdep2 instruction is reserved for debugging and implementation dependent operations.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint() -> Result<()> {
        breakpoint()?;
        Ok(())
    }

    #[test]
    fn test_impdep1() -> Result<()> {
        impdep1()?;
        Ok(())
    }

    #[test]
    fn test_impdep2() -> Result<()> {
        impdep2()?;
        Ok(())
    }
}
