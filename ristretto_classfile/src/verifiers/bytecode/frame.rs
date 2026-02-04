//! # Stack Frame for Bytecode Verification
//!
//! This module provides an optimized `Frame` implementation for dataflow analysis
//! during bytecode verification. The frame represents the state of local variables
//! and the operand stack at a specific program point.
//!
//! # Performance Optimizations
//!
//! - Pre-allocated vectors with capacity based on `max_stack`/`max_locals`
//! - Efficient cloning with shallow copy when possible
//! - Direct indexing for local variable access
//!
//! # References
//!
//! - [JVMS §4.10.1 - Verification by Type Checking](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1)

use crate::verifiers::bytecode::type_system::VerificationType;
use crate::verifiers::context::VerificationContext;
use crate::verifiers::error::{Result, VerifyError};

/// Represents a stack frame used during bytecode verification.
///
/// Each frame contains:
/// - Local variables array with types at each index
/// - Operand stack with pushed types
/// - Maximum stack capacity
///
/// # JVMS Reference
///
/// The frame tracks types as described in [JVMS §4.10.1.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1.2), including proper
/// handling of category 2 types (long, double) which occupy two slots.
///
/// # Example
///
/// ```ignore
/// use ristretto_classfile::verifiers::bytecode::frame::Frame;
/// use ristretto_classfile::verifiers::bytecode::type_system::VerificationType;
///
/// let mut frame = Frame::new(10, 5); // 10 locals, max 5 stack
/// frame.push(VerificationType::Integer)?;
/// let value = frame.pop()?;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    /// Local variable types. Index corresponds to local variable index.
    /// Category 2 types occupy two consecutive slots (value + Top).
    pub locals: Vec<VerificationType>,

    /// Operand stack types. Last element is the top of stack.
    /// Category 2 types occupy two consecutive slots (value + Top).
    pub stack: Vec<VerificationType>,

    /// Maximum stack size (for overflow checking).
    max_stack: usize,
}

impl Frame {
    /// Creates a new frame with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `max_locals` - Maximum number of local variable slots
    /// * `max_stack` - Maximum operand stack depth
    ///
    /// # Returns
    ///
    /// A new frame with all locals initialized to `Top` and an empty stack.
    #[must_use]
    pub fn new(max_locals: usize, max_stack: usize) -> Self {
        Self {
            locals: vec![VerificationType::Top; max_locals],
            stack: Vec::with_capacity(max_stack),
            max_stack,
        }
    }

    /// Creates a frame with the given locals and an empty stack.
    ///
    /// # Arguments
    ///
    /// * `locals` - Initial local variable types
    /// * `max_stack` - Maximum operand stack depth
    #[must_use]
    pub fn with_locals(locals: Vec<VerificationType>, max_stack: usize) -> Self {
        Self {
            locals,
            stack: Vec::with_capacity(max_stack),
            max_stack,
        }
    }

    /// Returns the current stack depth.
    #[inline]
    #[must_use]
    pub fn stack_depth(&self) -> usize {
        self.stack.len()
    }

    /// Returns the number of local variable slots.
    #[inline]
    #[must_use]
    pub fn locals_count(&self) -> usize {
        self.locals.len()
    }

    /// Returns the maximum stack depth.
    #[inline]
    #[must_use]
    pub fn max_stack(&self) -> usize {
        self.max_stack
    }

    /// Checks if the operand stack is empty.
    #[inline]
    #[must_use]
    pub fn is_stack_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Pushes a verification type onto the operand stack.
    ///
    /// # Arguments
    ///
    /// * `ty` - The type to push
    ///
    /// # Errors
    ///
    /// Returns `VerifyError::StackOverflow` if the stack would exceed `max_stack`.
    ///
    /// # JVMS Reference
    ///
    /// Implements stack push semantics from [JVMS §4.10.1](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1).
    pub fn push(&mut self, ty: VerificationType) -> Result<()> {
        if self.stack.len() >= self.max_stack {
            return Err(VerifyError::VerifyError(
                "Operand stack overflow".to_string(),
            ));
        }
        self.stack.push(ty);
        Ok(())
    }

    /// Pushes a category 2 type (Long or Double) onto the stack.
    ///
    /// This pushes the type followed by Top (for the second slot).
    ///
    /// # Arguments
    ///
    /// * `ty` - Must be `Long` or `Double`
    ///
    /// # Errors
    ///
    /// Returns an error if there's not enough stack space for two slots.
    pub fn push_category2(&mut self, ty: VerificationType) -> Result<()> {
        debug_assert!(ty.is_category2());
        if self.stack.len() + 2 > self.max_stack {
            return Err(VerifyError::VerifyError(
                "Operand stack overflow".to_string(),
            ));
        }
        self.stack.push(ty);
        self.stack.push(VerificationType::Top);
        Ok(())
    }

    /// Pops a verification type from the operand stack.
    ///
    /// # Returns
    ///
    /// The type at the top of the stack.
    ///
    /// # Errors
    ///
    /// Returns `VerifyError::StackUnderflow` if the stack is empty.
    pub fn pop(&mut self) -> Result<VerificationType> {
        self.stack
            .pop()
            .ok_or_else(|| VerifyError::VerifyError("Operand stack underflow".to_string()))
    }

    /// Pops a category 2 type (Long or Double) from the stack.
    ///
    /// This pops the Top marker first, then the actual type.
    ///
    /// # Returns
    ///
    /// The category 2 type (Long or Double).
    ///
    /// # Errors
    ///
    /// Returns an error if the stack doesn't contain a valid category 2 type.
    pub fn pop_category2(&mut self) -> Result<VerificationType> {
        let top = self.pop()?;
        if top != VerificationType::Top {
            return Err(VerifyError::VerifyError(format!(
                "Expected Top for category 2 second slot, got {top}"
            )));
        }
        let value = self.pop()?;
        if !value.is_category2() {
            return Err(VerifyError::VerifyError(format!(
                "Expected category 2 type, got {value}"
            )));
        }
        Ok(value)
    }

    /// Peeks at the type at the top of the stack without removing it.
    ///
    /// # Returns
    ///
    /// A reference to the type at the top of the stack.
    ///
    /// # Errors
    ///
    /// Returns an error if the stack is empty.
    pub fn peek(&self) -> Result<&VerificationType> {
        self.stack
            .last()
            .ok_or_else(|| VerifyError::VerifyError("Operand stack underflow".to_string()))
    }

    /// Peeks at a type at the specified depth from the top.
    ///
    /// Depth 0 is the top of stack.
    ///
    /// # Arguments
    ///
    /// * `depth` - Distance from top of stack
    ///
    /// # Returns
    ///
    /// A reference to the type at the specified depth.
    ///
    /// # Errors
    ///
    /// Returns an error if the depth exceeds the stack size.
    pub fn peek_at(&self, depth: usize) -> Result<&VerificationType> {
        let len = self.stack.len();
        if depth >= len {
            return Err(VerifyError::VerifyError(format!(
                "Stack depth {depth} exceeds stack size {len}"
            )));
        }
        Ok(&self.stack[len - 1 - depth])
    }

    /// Clears the operand stack.
    pub fn clear_stack(&mut self) {
        self.stack.clear();
    }

    /// Gets the type at a local variable index.
    ///
    /// # Arguments
    ///
    /// * `index` - The local variable index
    ///
    /// # Returns
    ///
    /// The type at the specified index.
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds.
    pub fn get_local(&self, index: u16) -> Result<&VerificationType> {
        let idx = index as usize;
        if idx >= self.locals.len() {
            return Err(VerifyError::VerifyError(format!(
                "Local variable index {index} out of bounds (max {})",
                self.locals.len()
            )));
        }
        Ok(&self.locals[idx])
    }

    /// Sets the type at a local variable index.
    ///
    /// # Arguments
    ///
    /// * `index` - The local variable index
    /// * `ty` - The type to set
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds.
    pub fn set_local(&mut self, index: u16, ty: VerificationType) -> Result<()> {
        let idx = index as usize;
        if idx >= self.locals.len() {
            return Err(VerifyError::VerifyError(format!(
                "Local variable index {index} out of bounds (max {})",
                self.locals.len()
            )));
        }
        self.locals[idx] = ty;
        Ok(())
    }

    /// Sets a category 2 type at a local variable index.
    ///
    /// This sets the type at `index` and `Top` at `index + 1`.
    ///
    /// # Arguments
    ///
    /// * `index` - The local variable index
    /// * `ty` - The category 2 type (Long or Double)
    ///
    /// # Errors
    ///
    /// Returns an error if there's not enough space for two slots.
    pub fn set_local_category2(&mut self, index: u16, ty: VerificationType) -> Result<()> {
        debug_assert!(ty.is_category2());
        let idx = index as usize;
        if idx + 1 >= self.locals.len() {
            return Err(VerifyError::VerifyError(format!(
                "Local variable index {index} + 1 out of bounds for category 2 type"
            )));
        }
        self.locals[idx] = ty;
        self.locals[idx + 1] = VerificationType::Top;
        Ok(())
    }

    /// Gets a category 2 type from local variables.
    ///
    /// Validates that the slot at `index + 1` is `Top`.
    ///
    /// # Arguments
    ///
    /// * `index` - The local variable index
    ///
    /// # Returns
    ///
    /// The category 2 type.
    ///
    /// # Errors
    ///
    /// Returns an error if the local doesn't contain a valid category 2 type.
    pub fn get_local_category2(&self, index: u16) -> Result<&VerificationType> {
        let idx = index as usize;
        if idx + 1 >= self.locals.len() {
            return Err(VerifyError::VerifyError(format!(
                "Local variable index {index} + 1 out of bounds for category 2 type"
            )));
        }
        let ty = &self.locals[idx];
        if !ty.is_category2() {
            return Err(VerifyError::VerifyError(format!(
                "Expected category 2 type at local {index}, got {ty}"
            )));
        }
        let second = &self.locals[idx + 1];
        if *second != VerificationType::Top {
            return Err(VerifyError::VerifyError(format!(
                "Expected Top at local {} for category 2, got {second}",
                index + 1
            )));
        }
        Ok(ty)
    }

    /// Merges another frame into this frame.
    ///
    /// This implements the type merging rules from [JVMS §4.10.1.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1.2) for
    /// control flow merge points.
    ///
    /// # Arguments
    ///
    /// * `other` - The frame to merge with
    /// * `context` - The verification context for type hierarchy checks
    ///
    /// # Returns
    ///
    /// `true` if this frame was changed by the merge.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Stack depths don't match
    /// - Locals counts are incompatible
    /// - Type hierarchy checks fail
    pub fn merge<C: VerificationContext>(&mut self, other: &Frame, context: &C) -> Result<bool> {
        // Stack depths must match at merge points
        if self.stack.len() != other.stack.len() {
            return Err(VerifyError::VerifyError(format!(
                "Stack depth mismatch at merge point: {} vs {}",
                self.stack.len(),
                other.stack.len()
            )));
        }

        // Locals must be compatible
        if self.locals.len() < other.locals.len() {
            return Err(VerifyError::VerifyError(format!(
                "Locals count mismatch: {} vs {}",
                self.locals.len(),
                other.locals.len()
            )));
        }

        let mut changed = false;

        // Merge locals
        for (target, source) in self.locals.iter_mut().zip(&other.locals) {
            if target != source {
                let merged = target.merge(source, context)?;
                if *target != merged {
                    *target = merged;
                    changed = true;
                }
            }
        }

        // Merge stack
        for (target, source) in self.stack.iter_mut().zip(&other.stack) {
            if target != source {
                let merged = target.merge(source, context)?;
                if *target != merged {
                    *target = merged;
                    changed = true;
                }
            }
        }

        Ok(changed)
    }

    /// Replaces all occurrences of an uninitialized type with an initialized type.
    ///
    /// This is called after `invokespecial <init>` to convert uninitialized
    /// references to their initialized form.
    ///
    /// # Arguments
    ///
    /// * `uninitialized` - The uninitialized type to replace
    /// * `initialized` - The initialized type to replace with
    pub fn initialize_object(
        &mut self,
        uninitialized: &VerificationType,
        initialized: &VerificationType,
    ) {
        for local in &mut self.locals {
            if local == uninitialized {
                *local = initialized.clone();
            }
        }
        for stack_entry in &mut self.stack {
            if stack_entry == uninitialized {
                *stack_entry = initialized.clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct MockContext;

    impl VerificationContext for MockContext {
        fn is_subclass(&self, subclass: &str, superclass: &str) -> Result<bool> {
            Ok(subclass == superclass)
        }

        fn is_assignable(&self, target: &str, source: &str) -> Result<bool> {
            Ok(target == source || target == "java/lang/Object")
        }

        fn common_superclass(&self, _class1: &str, _class2: &str) -> Result<String> {
            Ok("java/lang/Object".to_string())
        }
    }

    #[test]
    fn test_new_frame() {
        let frame = Frame::new(5, 10);
        assert_eq!(frame.locals_count(), 5);
        assert_eq!(frame.max_stack(), 10);
        assert!(frame.is_stack_empty());
        assert_eq!(frame.stack_depth(), 0);
    }

    #[test]
    fn test_push_pop() {
        let mut frame = Frame::new(5, 10);

        frame.push(VerificationType::Integer).unwrap();
        assert_eq!(frame.stack_depth(), 1);

        let ty = frame.pop().unwrap();
        assert_eq!(ty, VerificationType::Integer);
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_stack_overflow() {
        let mut frame = Frame::new(5, 2);

        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = frame.push(VerificationType::Integer);
        assert!(result.is_err());
    }

    #[test]
    fn test_stack_underflow() {
        let mut frame = Frame::new(5, 10);

        let result = frame.pop();
        assert!(result.is_err());
    }

    #[test]
    fn test_category2_operations() {
        let mut frame = Frame::new(5, 10);

        frame.push_category2(VerificationType::Long).unwrap();
        assert_eq!(frame.stack_depth(), 2);

        let ty = frame.pop_category2().unwrap();
        assert_eq!(ty, VerificationType::Long);
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_local_operations() {
        let mut frame = Frame::new(5, 10);

        frame.set_local(2, VerificationType::Integer).unwrap();
        let ty = frame.get_local(2).unwrap();
        assert_eq!(*ty, VerificationType::Integer);
    }

    #[test]
    fn test_local_category2() {
        let mut frame = Frame::new(5, 10);

        frame
            .set_local_category2(1, VerificationType::Double)
            .unwrap();

        let ty = frame.get_local_category2(1).unwrap();
        assert_eq!(*ty, VerificationType::Double);

        assert_eq!(*frame.get_local(2).unwrap(), VerificationType::Top);
    }

    #[test]
    fn test_local_out_of_bounds() {
        let frame = Frame::new(5, 10);

        let result = frame.get_local(10);
        assert!(result.is_err());
    }

    #[test]
    fn test_merge_same_frames() {
        let ctx = MockContext;
        let mut frame1 = Frame::new(5, 10);
        frame1.set_local(0, VerificationType::Integer).unwrap();
        frame1.push(VerificationType::Float).unwrap();

        let frame2 = frame1.clone();

        let changed = frame1.merge(&frame2, &ctx).unwrap();
        assert!(!changed);
    }

    #[test]
    fn test_merge_different_types() {
        let ctx = MockContext;
        let mut frame1 = Frame::new(5, 10);
        frame1
            .set_local(0, VerificationType::Object(Arc::from("java/lang/String")))
            .unwrap();

        let mut frame2 = Frame::new(5, 10);
        frame2
            .set_local(0, VerificationType::Object(Arc::from("java/lang/Integer")))
            .unwrap();

        let changed = frame1.merge(&frame2, &ctx).unwrap();
        assert!(changed);
        assert_eq!(
            *frame1.get_local(0).unwrap(),
            VerificationType::Object(Arc::from("java/lang/Object"))
        );
    }

    #[test]
    fn test_merge_stack_depth_mismatch() {
        let ctx = MockContext;
        let mut frame1 = Frame::new(5, 10);
        frame1.push(VerificationType::Integer).unwrap();

        let frame2 = Frame::new(5, 10);

        let result = frame1.merge(&frame2, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_object() {
        let mut frame = Frame::new(5, 10);
        let uninit = VerificationType::Uninitialized(0);
        let init = VerificationType::Object(Arc::from("java/lang/Object"));

        frame.set_local(0, uninit.clone()).unwrap();
        frame.push(uninit.clone()).unwrap();

        frame.initialize_object(&uninit, &init);

        assert_eq!(*frame.get_local(0).unwrap(), init);
        assert_eq!(*frame.peek().unwrap(), init);
    }

    #[test]
    fn test_peek_at() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Long).unwrap();

        assert_eq!(*frame.peek_at(0).unwrap(), VerificationType::Long);
        assert_eq!(*frame.peek_at(1).unwrap(), VerificationType::Float);
        assert_eq!(*frame.peek_at(2).unwrap(), VerificationType::Integer);

        assert!(frame.peek_at(3).is_err());
    }
}
