use ristretto_classloader::{Class, Method};
use std::sync::Arc;

/// Trait representing a stack frame in the call stack.
pub trait Frame: Send + Sync {
    /// Get the class that owns this frame.
    fn class(&self) -> &Arc<Class>;

    /// Get the method in this frame.
    fn method(&self) -> &Arc<Method>;

    /// Get the current program counter in this frame.
    fn program_counter(&self) -> usize;
}

/// Blanket implementation of Frame for `Arc<F>` where `F: Frame`.
impl<F: Frame> Frame for Arc<F> {
    fn class(&self) -> &Arc<Class> {
        (**self).class()
    }

    fn method(&self) -> &Arc<Method> {
        (**self).method()
    }

    fn program_counter(&self) -> usize {
        (**self).program_counter()
    }
}
