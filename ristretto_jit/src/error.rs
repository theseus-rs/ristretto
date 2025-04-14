use ristretto_classfile::attributes::Instruction;

/// Ristretto JIT result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Errors that can occur when loading classes
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occurred while loading a class file
    #[error(transparent)]
    ClassFileError(#[from] ristretto_classfile::Error),
    /// A compilation error occurred generating the code
    #[error(transparent)]
    CodegenError(#[from] cranelift::codegen::CodegenError),
    /// Internal error
    #[error("Internal error: {0}")]
    InternalError(String),
    /// Invalid constant
    #[error("Invalid constant; expected {expected}, found {actual}")]
    InvalidConstant { expected: String, actual: String },
    /// Invalid constant index
    #[error("Invalid instant index: {0}")]
    InvalidConstantIndex(u16),
    /// Invalid local variable index
    #[error("Invalid local variable index: {0}")]
    InvalidLocalVariableIndex(usize),
    /// Invalid constant
    #[error("Invalid value; expected {expected}, found {actual}")]
    InvalidValue { expected: i8, actual: i8 },
    /// A module error occurred
    #[error(transparent)]
    ModuleError(#[from] cranelift::module::ModuleError),
    /// The operand stack underflow
    #[error("Operand stack underflow")]
    OperandStackUnderflow,
    /// The instruction is not supported by the JIT compiler
    #[error("Unsupported instruction: {0}")]
    UnsupportedInstruction(Instruction),
    /// The JIT compiler does not support the given method
    #[error("Unsupported method: {0}")]
    UnsupportedMethod(String),
    /// The target Instruction Set Architecture (ISA) is not supported
    #[error("Unsupported target ISA: {0}")]
    UnsupportedTargetISA(&'static str),
    /// JIT compilation is not supported for the type
    #[error("Unsupported type: {0}")]
    UnsupportedType(String),
}
