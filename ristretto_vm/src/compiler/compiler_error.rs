use crate::Error;

/// An error reported while running `javac`.
#[derive(Debug, thiserror::Error)]
pub enum CompilerError {
    /// Java source did not compile successfully.
    #[error("Java compilation failed")]
    CompilationFailed,
    /// The supplied compiler arguments were invalid.
    #[error("invalid javac arguments")]
    InvalidArguments,
    /// An unrecoverable compiler system error occurred.
    #[error("javac system error")]
    SystemError,
    /// The compiler terminated abnormally.
    #[error("javac terminated abnormally")]
    AbnormalTermination,
    /// The compiler returned an exit code not defined by `javac`.
    #[error("unknown javac exit code: {0}")]
    UnknownExitCode(i32),
    /// The compiler VM or supporting infrastructure failed.
    #[error(transparent)]
    Vm(#[from] Error),
}

impl CompilerError {
    /// Return the standard `javac` process exit code for this error.
    #[must_use]
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::CompilationFailed => 1,
            Self::InvalidArguments => 2,
            Self::SystemError | Self::Vm(_) => 3,
            Self::AbnormalTermination => 4,
            Self::UnknownExitCode(code) => *code,
        }
    }

    pub(super) fn from_exit_code(exit_code: i32) -> Result<(), Self> {
        match exit_code {
            0 => Ok(()),
            1 => Err(Self::CompilationFailed),
            2 => Err(Self::InvalidArguments),
            3 => Err(Self::SystemError),
            4 => Err(Self::AbnormalTermination),
            code => Err(Self::UnknownExitCode(code)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiler_error_exit_codes() {
        assert_eq!(1, CompilerError::CompilationFailed.exit_code());
        assert_eq!(2, CompilerError::InvalidArguments.exit_code());
        assert_eq!(3, CompilerError::SystemError.exit_code());
        assert_eq!(4, CompilerError::AbnormalTermination.exit_code());
        assert_eq!(17, CompilerError::UnknownExitCode(17).exit_code());
    }
}
