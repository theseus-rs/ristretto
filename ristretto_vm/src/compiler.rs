//! Java compiler support powered by the Ristretto virtual machine.

mod bridge;
mod compiled_classes;
#[expect(clippy::module_inception)]
mod compiler;
mod compiler_error;
mod java_source;
mod runtime;

pub use compiled_classes::CompiledClasses;
pub use compiler::Compiler;
pub use compiler_error::CompilerError;
pub use java_source::JavaSource;
