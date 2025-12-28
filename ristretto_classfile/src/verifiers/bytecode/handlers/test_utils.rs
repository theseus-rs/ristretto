//! Test utilities for bytecode verification handlers.
//!
//! This module provides mock implementations of [`VerificationContext`] for testing
//! bytecode verification handlers.

use crate::verifiers::context::VerificationContext;
use crate::verifiers::error::Result;

/// A mock verification context that returns `true` for all subclass and assignability checks.
///
/// This is useful for testing success paths where type compatibility is assumed.
#[derive(Debug)]
pub struct MockContext;

impl VerificationContext for MockContext {
    fn is_subclass(&self, _: &str, _: &str) -> Result<bool> {
        Ok(true)
    }
    fn is_assignable(&self, _: &str, _: &str) -> Result<bool> {
        Ok(true)
    }
    fn common_superclass(&self, _: &str, _: &str) -> Result<String> {
        Ok("java/lang/Object".to_string())
    }
}

/// A strict mock verification context that returns `false` for all subclass and assignability checks.
///
/// This is useful for testing failure paths where type compatibility must fail.
#[derive(Debug)]
pub struct StrictMockContext;

impl VerificationContext for StrictMockContext {
    fn is_subclass(&self, _: &str, _: &str) -> Result<bool> {
        Ok(false)
    }
    fn is_assignable(&self, _: &str, _: &str) -> Result<bool> {
        Ok(false)
    }
    fn common_superclass(&self, _: &str, _: &str) -> Result<String> {
        Ok("java/lang/Object".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_context_is_subclass() {
        let context = MockContext;
        assert!(context.is_subclass("A", "B").unwrap());
    }

    #[test]
    fn test_mock_context_is_assignable() {
        let context = MockContext;
        assert!(context.is_assignable("A", "B").unwrap());
    }

    #[test]
    fn test_mock_context_common_superclass() {
        let context = MockContext;
        assert_eq!(
            context.common_superclass("A", "B").unwrap(),
            "java/lang/Object"
        );
    }

    #[test]
    fn test_strict_mock_context_is_subclass() {
        let context = StrictMockContext;
        assert!(!context.is_subclass("A", "B").unwrap());
    }

    #[test]
    fn test_strict_mock_context_is_assignable() {
        let context = StrictMockContext;
        assert!(!context.is_assignable("A", "B").unwrap());
    }

    #[test]
    fn test_strict_mock_context_common_superclass() {
        let context = StrictMockContext;
        assert_eq!(
            context.common_superclass("A", "B").unwrap(),
            "java/lang/Object"
        );
    }
}
