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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;

    #[test]
    fn test_arc_frame_delegates_to_inner_frame() -> crate::Result<()> {
        let class = test_utils::class("FrameClass", &[])?;
        let method = test_utils::method("frameMethod", "()V");
        let frame = Arc::new(test_utils::MockFrame::new(
            class.clone(),
            method.clone(),
            42,
        ));

        let frame_ref: &dyn Frame = &frame;
        assert_eq!(frame_ref.class().name(), "FrameClass");
        assert!(Arc::ptr_eq(frame_ref.class(), &class));
        assert!(Arc::ptr_eq(frame_ref.method(), &method));
        assert_eq!(frame_ref.program_counter(), 42);
        Ok(())
    }
}
