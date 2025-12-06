/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.nop>
pub(crate) fn nop() {
    // No operation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop() {
        nop();
    }
}
