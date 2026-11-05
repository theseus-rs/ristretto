use crate::verifiers::error;
use crate::{JavaStr, JavaString};

/// A symbolic member access together with its verified receiver type.
/// The context can resolve declaring classes, access flags, packages, and loader identities.
#[derive(Debug)]
pub struct MemberAccess<'a> {
    /// Class whose bytecode is being verified.
    pub current_class: &'a JavaStr,
    /// Symbolic owner in the constant pool.
    pub owner: &'a JavaStr,
    /// Member name.
    pub name: &'a JavaStr,
    /// Member descriptor.
    pub descriptor: &'a JavaStr,
    /// Receiver descriptor, if this is an instance access with a non-null receiver.
    pub receiver: Option<&'a JavaStr>,
    /// Instruction performing the access.
    pub instruction: &'a crate::attributes::Instruction,
}

/// Names and descriptors retain their exact modified UTF-8 identity, including lone surrogates.
///
/// A trait that allows the verifier to resolve type relationships without knowing about the VM.
pub trait VerificationContext {
    /// Validate member access using resolved flags and loader identities.
    /// Override this hook to enforce contextual constraints such as protected receiver
    /// access; the default leaves member resolution and access checking to the VM.
    ///
    /// # Errors
    /// Returns an error when the member is inaccessible or resolution fails.
    fn verify_member_access(&self, _access: &MemberAccess<'_>) -> error::Result<()> {
        Ok(())
    }

    /// Identity of the class-loading environment and its current hierarchy revision.
    /// Return `None` (the default) to disable reuse of verification results. A token
    /// must change whenever any resolution or access-check result can change.
    fn cache_token(&self) -> Option<u64> {
        None
    }

    /// Checks if a class is a subclass of another.
    ///
    /// # Errors
    /// Returns `VerifyError` if the check cannot be performed.
    fn is_subclass(&self, subclass: &JavaStr, superclass: &JavaStr) -> error::Result<bool>;

    /// Checks if a type is assignable to another.
    ///
    /// # Errors
    /// Returns `VerifyError` if the check cannot be performed.
    fn is_assignable(&self, target: &JavaStr, source: &JavaStr) -> error::Result<bool>;

    /// Finds the common superclass of two classes.
    ///
    /// # Errors
    /// Returns `VerifyError` if the common superclass cannot be found.
    fn common_superclass(&self, class1: &JavaStr, class2: &JavaStr) -> error::Result<JavaString>;
}
