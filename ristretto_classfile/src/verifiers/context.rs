use crate::verifiers::error;

/// A trait that allows the verifier to resolve type relationships without knowing about the VM.
pub trait VerificationContext {
    /// Checks if a class is a subclass of another.
    ///
    /// # Errors
    /// Returns `VerifyError` if the check cannot be performed.
    fn is_subclass(&self, subclass: &str, superclass: &str) -> error::Result<bool>;

    /// Checks if a type is assignable to another.
    ///
    /// # Errors
    /// Returns `VerifyError` if the check cannot be performed.
    fn is_assignable(&self, target: &str, source: &str) -> error::Result<bool>;

    /// Finds the common superclass of two classes.
    ///
    /// # Errors
    /// Returns `VerifyError` if the common superclass cannot be found.
    fn common_superclass(&self, class1: &str, class2: &str) -> error::Result<String>;
}
