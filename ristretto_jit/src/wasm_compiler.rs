use crate::{Function, Result};
use ristretto_classfile::{ClassFile, Method};

/// Java Virtual Machine (JVM) bytecode to wasm code compiler.
///
/// This is a no-op compiler that does not actually compile any code.
#[derive(Debug)]
pub struct Compiler {}

impl Compiler {
    /// Creates a new instance of the compiler for the host machine.
    pub fn new() -> Result<Self> {
        Ok(Compiler {})
    }

    /// Compiles the given bytecode into wasm code.
    ///
    /// # Errors
    ///
    /// always returns an error as this is a no-op compiler.
    pub fn compile(&self, _class_file: &ClassFile, _method: &Method) -> Result<Function> {
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
}
