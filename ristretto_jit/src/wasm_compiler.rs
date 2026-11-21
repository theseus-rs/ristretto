use crate::{Function, Result};
use ristretto_classfile::{ClassFile, Method};

/// Java Virtual Machine (JVM) fallback compiler.
///
/// This is a no-op compiler that does not actually compile any code.
#[derive(Clone, Debug)]
pub struct Compiler {}

impl Compiler {
    /// Creates a new instance of the compiler for the host machine.
    ///
    /// # Errors
    ///
    /// This function currently never returns an error, but the signature is kept consistent
    /// with the native compiler implementation.
    pub fn new() -> Result<Self> {
        Ok(Compiler {})
    }

    /// Returns whether the method can potentially be JIT compiled.
    ///
    /// Always returns `false` since native JIT is not supported.
    #[must_use]
    pub fn can_compile(_method: &Method) -> bool {
        false
    }

    /// Request-only native preparation is unavailable on this target.
    #[cfg(feature = "experimental-reference-dispatch")]
    #[must_use]
    pub fn request_rejection_reason(_class: &ClassFile, _method: &Method) -> Option<&'static str> {
        Some("unsupported_target")
    }

    /// Compiles the given bytecode into native code.
    ///
    /// # Errors
    ///
    /// Always returns an error as this is a no-op compiler.
    pub fn compile(
        &self,
        _class_file: &ClassFile,
        _method: &Method,
        _symbols: &[(&str, *const u8)],
    ) -> Result<Function> {
        Err(crate::Error::InternalError("Not implemented".to_string()))
    }
    /// Request-only native code is unavailable on this target.
    /// # Errors
    /// Always returns an unsupported-target error.
    #[cfg(feature = "experimental-reference-dispatch")]
    pub fn compile_requests(
        &self,
        _class_file: &ClassFile,
        _method: &Method,
        _symbols: &[(&str, *const u8)],
    ) -> Result<Function> {
        Err(crate::Error::UnsupportedMethod(
            "unsupported_target".to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_new() {
        let result = Compiler::new();
        assert!(result.is_ok());
    }

    #[cfg(feature = "experimental-reference-dispatch")]
    #[test]
    fn request_compilation_fails_closed() -> Result<()> {
        let class = ClassFile::default();
        let method = Method::default();
        assert_eq!(
            Compiler::request_rejection_reason(&class, &method),
            Some("unsupported_target")
        );
        assert!(
            matches!(Compiler::new()?.compile_requests(&class, &method, &[]), Err(crate::Error::UnsupportedMethod(reason)) if reason == "unsupported_target")
        );
        Ok(())
    }

    #[test]
    fn test_can_compile() {
        let method = Method::default();
        assert!(!Compiler::can_compile(&method));
    }
}
