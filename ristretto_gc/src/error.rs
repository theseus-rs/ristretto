//! Error handling for the Ristretto Garbage Collector.
//!
//! This module provides a comprehensive error type system for the Ristretto Garbage Collector,
//! covering errors that might occur during garbage collection and memory management.

use std::sync::PoisonError;

/// Ristretto Garbage Collector result type
///
/// This is a type alias for the standard library's [`Result`](core::result::Result) type with the
/// error type defaulting to [`Error`].
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Errors that can occur during garbage collection and memory management.
///
/// This enum represents all possible error conditions that might arise during
/// garbage collection operations, thread synchronization, and memory management.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Collection phase error
    #[error("Garbage collection phase error: {0}")]
    CollectionPhaseError(String),
    /// Lock acquisition error
    #[error("Failed to acquire lock: {0}")]
    LockError(String),
    /// Root management error
    #[error("Failed to manage GC roots: {0}")]
    RootError(String),
    /// Statistics access error
    #[error("Failed to access garbage collection statistics: {0}")]
    StatsError(String),
    /// Thread synchronization error
    #[error("Thread synchronization error: {0}")]
    SyncError(String),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(error: PoisonError<T>) -> Self {
        Error::LockError(format!("Poisoned lock: {error}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_poison_error() {
        let poison_error: PoisonError<()> = PoisonError::new(());
        let error: Error = poison_error.into();
        assert!(matches!(error, Error::LockError(_)));
    }
}
