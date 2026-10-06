use crate::verifiers::error;

/// A symbolic member access together with its verified receiver type.
/// The context can resolve declaring classes, access flags, packages, and loader identities.
#[derive(Debug)]
pub struct MemberAccess<'a> {
    /// Class whose bytecode is being verified.
    pub current_class: &'a str,
    /// Symbolic owner in the constant pool.
    pub owner: &'a str,
    /// Member name.
    pub name: &'a str,
    /// Member descriptor.
    pub descriptor: &'a str,
    /// Receiver descriptor, if this is an instance access with a non-null receiver.
    pub receiver: Option<&'a str>,
    /// Instruction performing the access.
    pub instruction: &'a crate::attributes::Instruction,
}

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
