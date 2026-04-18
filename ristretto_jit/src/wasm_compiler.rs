use crate::{Function, Result};
use ristretto_classfile::{ClassFile, Method};

/// Java Virtual Machine (JVM) bytecode to wasm code compiler.
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
    /// with the non-wasm compiler implementation.
    pub fn new() -> Result<Self> {
        Ok(Compiler {})
    }

    /// Returns whether the method can potentially be JIT compiled.
    ///
    /// Always returns `false` on wasm since native JIT is not supported.
    #[must_use]
    pub fn can_compile(_method: &Method) -> bool {
        false
    }

    /// Compiles the given bytecode into WASM code.
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_new() {
        let result = Compiler::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_can_compile() {
        let method = Method::default();
        assert!(!Compiler::can_compile(&method));
    }
}
