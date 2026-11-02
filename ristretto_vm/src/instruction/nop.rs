use crate::Result;

/// # References
///
/// - [JVMS §6.5.nop](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.nop)
#[inline]
#[expect(clippy::unnecessary_wraps)]
pub(crate) fn nop() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop() -> Result<()> {
        nop()?;
        Ok(())
    }
}
