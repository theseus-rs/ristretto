//! Test utilities for bytecode verification handlers.
//!
//! This module provides mock implementations of [`VerificationContext`] for testing
//! bytecode verification handlers.

use crate::verifiers::context::VerificationContext;
use crate::verifiers::error::{Result, VerifyError};
use crate::{JavaStr, JavaString};

/// A mock verification context for subclass and assignability checks.
///
/// This is useful for testing success paths where type compatibility is assumed.
#[derive(Clone, Copy, Debug)]
pub struct MockContext {
    checks_pass: bool,
}

impl MockContext {
    /// Context that returns `true` for subclass and assignability checks.
    pub const PERMISSIVE: Self = Self { checks_pass: true };

    /// Context that returns `false` for subclass and assignability checks.
    pub const STRICT: Self = Self { checks_pass: false };
}

impl VerificationContext for MockContext {
    fn cache_token(&self) -> Option<u64> {
        Some(u64::from(self.checks_pass))
    }

    fn is_subclass(&self, _: &JavaStr, _: &JavaStr) -> Result<bool> {
        Ok(self.checks_pass)
    }

    fn is_assignable(&self, _: &JavaStr, _: &JavaStr) -> Result<bool> {
        Ok(self.checks_pass)
    }

    fn common_superclass(&self, class1: &JavaStr, class2: &JavaStr) -> Result<JavaString> {
        if class1 == "error" || class2 == "error" {
            return Err(VerifyError::VerifyError(
                "common superclass failure".to_string(),
            ));
        }
        Ok("java/lang/Object".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_context_is_subclass() {
        let context = MockContext::PERMISSIVE;
        assert!(
            context
                .is_subclass(&JavaStr::cow_from_str("A"), &JavaStr::cow_from_str("B"))
                .unwrap()
        );
    }

    #[test]
    fn test_mock_context_is_assignable() {
        let context = MockContext::PERMISSIVE;
        assert!(
            context
                .is_assignable(&JavaStr::cow_from_str("A"), &JavaStr::cow_from_str("B"))
                .unwrap()
        );
    }

    #[test]
    fn test_mock_context_common_superclass() {
        let context = MockContext::PERMISSIVE;
        assert_eq!(
            context
                .common_superclass(&JavaStr::cow_from_str("A"), &JavaStr::cow_from_str("B"))
                .unwrap(),
            "java/lang/Object"
        );
    }

    #[test]
    fn test_strict_mock_context_is_subclass() {
        let context = MockContext::STRICT;
        assert!(
            !context
                .is_subclass(&JavaStr::cow_from_str("A"), &JavaStr::cow_from_str("B"))
                .unwrap()
        );
    }

    #[test]
    fn test_strict_mock_context_is_assignable() {
        let context = MockContext::STRICT;
        assert!(
            !context
                .is_assignable(&JavaStr::cow_from_str("A"), &JavaStr::cow_from_str("B"))
                .unwrap()
        );
    }

    #[test]
    fn test_strict_mock_context_common_superclass() {
        let context = MockContext::STRICT;
        assert_eq!(
            context
                .common_superclass(&JavaStr::cow_from_str("A"), &JavaStr::cow_from_str("B"))
                .unwrap(),
            "java/lang/Object"
        );
    }

    #[test]
    fn test_mock_context_common_superclass_error() {
        let context = MockContext::PERMISSIVE;
        assert!(
            context
                .common_superclass(&JavaStr::cow_from_str("error"), &JavaStr::cow_from_str("B"))
                .is_err()
        );
    }
}
