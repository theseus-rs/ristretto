/// Ristretto VM result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Errors that can occur when loading classes
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The arguments stack underflow
    #[error("Arguments stack underflow")]
    ArgumentsUnderflow,
    /// An array index is out of bounds
    #[error("An array index is out of bounds: {0}")]
    ArrayIndexOutOfBounds(usize),
    /// An exception occurred casting a value
    #[error("An exception occurred casting to: {0}")]
    ClassCastError(String),
    /// An error occurred while loading a class file
    #[error(transparent)]
    ClassFileError(#[from] ristretto_classfile::Error),
    /// An error occurred while loading a class
    #[error(transparent)]
    ClassLoaderError(#[from] ristretto_classloader::Error),
    /// Invalid constant
    #[error("Invalid constant; expected {expected}, found {actual}")]
    InvalidConstant { expected: String, actual: String },
    /// Invalid constant index
    #[error("Invalid instant index: {0}")]
    InvalidConstantIndex(u16),
    /// Invalid local variable
    #[error("Invalid local variable; expected {expected}, found {actual}")]
    InvalidLocalVariable { expected: String, actual: String },
    /// Invalid local variable index
    #[error("Invalid local variable index: {0}")]
    InvalidLocalVariableIndex(usize),
    /// Invalid operand for the operation
    #[error("Invalid operand; expected {expected}, found {actual}")]
    InvalidOperand { expected: String, actual: String },
    /// Invalid program counter
    #[error("Invalid program counter: {0}")]
    InvalidProgramCounter(usize),
    /// Invalid stack value
    #[error("Invalid stack value; expected {expected}, found {actual}")]
    InvalidStackValue { expected: String, actual: String },
    /// Null pointer
    #[error("Null pointer")]
    NullPointer,
    /// The operand stack overflowed
    #[error("Operand stack overflow")]
    OperandStackOverflow,
    /// The operand stack underflow
    #[error("Operand stack underflow")]
    OperandStackUnderflow,
    /// An error occurred while attempting to parse an integer
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    /// Poisoned lock
    #[error("Poisoned lock: {0}")]
    PoisonedLock(String),
    /// Runtime error
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    /// An error occurred while converting from an integer
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),
    /// Unsupported class file version
    #[error("Unsupported class file version: {0}")]
    UnsupportedClassFileVersion(u16),
}
