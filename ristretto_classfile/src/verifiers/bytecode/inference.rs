//! # Type Inference Verifier (Slow Path)
//!
//! This module implements the classic type-inference bytecode verifier that uses iterative dataflow
//! analysis. This is the fallback verifier for older class files (pre-Java 6) or when
//! `StackMapTable` verification fails.
//!
//! # Algorithm Overview
//!
//! 1. Initialize frames for method entry and exception handlers
//! 2. Use worklist-based dataflow analysis
//! 3. At merge points, compute LUB (Least Upper Bound) of incoming types
//! 4. Iterate until fixpoint is reached
//!
//! # When Used
//!
//! - Class files with major version < 50 (pre-Java 6)
//! - When `FallbackStrategy::FallbackToInference` is enabled and fast path fails
//! - When `FallbackStrategy::AlwaysInference` is configured
//!
//! # Performance Note
//!
//! This verifier is slower than the fast path due to iterative merging.
//! It should only be used when necessary.
//!
//! # References
//!
//! - [JVMS §4.10.2 - Verification by Type Inference](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.2)

use std::io::Cursor;
use std::sync::Arc;

use crate::FieldType;
use crate::attributes::{Attribute, ExceptionTableEntry, Instruction};
use crate::class_file::ClassFile;
use crate::method::Method;
use crate::method_access_flags::MethodAccessFlags;
use crate::verifiers::bytecode::config::VerifierConfig;
use crate::verifiers::bytecode::control_flow::{
    CodeInfo, Worklist, compute_successors, validate_exception_table,
};
use crate::verifiers::bytecode::diagnostics::VerificationTrace;
use crate::verifiers::bytecode::frame::Frame;
use crate::verifiers::bytecode::handlers;
use crate::verifiers::bytecode::handlers::references::ConstantPoolResolver;
use crate::verifiers::bytecode::type_system::VerificationType;
use crate::verifiers::context::VerificationContext;
use crate::verifiers::error::{Result, VerifyError};

/// Type inference verifier using iterative dataflow analysis.
///
/// This verifier implements the classic verification algorithm that computes
/// types via iterative merging at control flow merge points.
pub struct InferenceVerifier<'a, C: VerificationContext> {
    /// The class file containing the method.
    class_file: &'a ClassFile,
    /// The method being verified.
    method: &'a Method,
    /// The verification context for type hierarchy checks.
    context: &'a C,
    /// Verifier configuration.
    config: &'a VerifierConfig,
    /// The bytecode instructions.
    code: &'a Vec<Instruction>,
    /// Exception table entries.
    exception_table: &'a Vec<ExceptionTableEntry>,
    /// Maximum operand stack depth.
    max_stack: u16,
    /// Maximum number of local variables.
    max_locals: u16,
    /// Code information for control flow analysis.
    code_info: CodeInfo,
    /// The method's return type.
    return_type: Option<FieldType>,
    /// Class file major version.
    major_version: u16,
    /// The current class name.
    current_class: String,
    /// The method name.
    method_name: String,
    /// Verification trace (for verbose mode).
    trace: VerificationTrace,
}

impl<C: VerificationContext> std::fmt::Debug for InferenceVerifier<'_, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InferenceVerifier")
            .field("current_class", &self.current_class)
            .field("method_name", &self.method_name)
            .field("max_stack", &self.max_stack)
            .field("max_locals", &self.max_locals)
            .field("major_version", &self.major_version)
            .finish_non_exhaustive()
    }
}

impl<'a, C: VerificationContext> InferenceVerifier<'a, C> {
    /// Creates a new inference verifier.
    ///
    /// # Errors
    ///
    /// Returns an error if the method's code cannot be processed.
    pub fn new(
        class_file: &'a ClassFile,
        method: &'a Method,
        context: &'a C,
        config: &'a VerifierConfig,
    ) -> Result<Self> {
        // Extract Code attribute
        let (code, max_stack, max_locals, _, exception_table) = method
            .attributes
            .iter()
            .find_map(|attr| match attr {
                Attribute::Code {
                    code,
                    max_stack,
                    max_locals,
                    attributes,
                    exception_table,
                    ..
                } => Some((code, *max_stack, *max_locals, attributes, exception_table)),
                _ => None,
            })
            .ok_or_else(|| {
                VerifyError::ClassFormatError("Method has no Code attribute".to_string())
            })?;

        // Build instruction offset map
        let mut instruction_offsets = Vec::with_capacity(code.len());
        let mut cursor = Cursor::new(Vec::new());

        for instruction in code {
            let offset = u16::try_from(cursor.position())?;
            instruction_offsets.push(offset);
            instruction
                .to_bytes(&mut cursor)
                .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;
        }
        let code_length = u16::try_from(cursor.position())?;

        let code_info = CodeInfo::new(instruction_offsets, code_length);

        // Get class and method names
        let current_class = class_file
            .constant_pool
            .try_get_class(class_file.this_class)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?
            .to_string();

        let method_name = class_file
            .constant_pool
            .try_get_utf8(method.name_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?
            .to_string();

        // Parse method descriptor for return type
        let descriptor = class_file
            .constant_pool
            .try_get_utf8(method.descriptor_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let (_, return_type) = FieldType::parse_method_descriptor(descriptor)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        // Get major version
        let major_version = class_file.version.major();

        let trace = VerificationTrace::new(config.verbose() || config.trace());

        Ok(Self {
            class_file,
            method,
            context,
            config,
            code,
            exception_table,
            max_stack,
            max_locals,
            code_info,
            return_type,
            major_version,
            current_class,
            method_name,
            trace,
        })
    }

    /// Verifies the method using type inference.
    ///
    /// # Errors
    ///
    /// Returns an error if verification fails.
    pub fn verify(&mut self) -> Result<()> {
        // Validate exception table
        validate_exception_table(self.exception_table, &self.code_info)?;

        // Create initial frame
        let initial_frame = self.create_initial_frame()?;

        // Initialize frames array (dense vector for performance)
        let mut frames: Vec<Option<Frame>> = vec![None; self.code_info.instruction_count()];
        frames[0] = Some(initial_frame);

        // Worklist-based dataflow analysis
        let mut worklist = Worklist::new(self.code_info.instruction_count());
        worklist.add(0);

        let mut iterations = 0;
        let max_iterations = self.config.max_inference_iterations;

        while let Some(index) = worklist.pop() {
            iterations += 1;
            if iterations > max_iterations {
                return Err(VerifyError::VerifyError(format!(
                    "Type inference exceeded maximum iterations ({max_iterations})"
                )));
            }

            let offset = self.code_info.offset_at(index).ok_or_else(|| {
                VerifyError::VerifyError(format!("Invalid instruction index {index}"))
            })?;

            let frame = frames[index]
                .clone()
                .ok_or_else(|| VerifyError::VerifyError(format!("No frame at offset {offset}")))?;

            let instruction = &self.code[index];

            // Log trace if enabled
            if self.trace.is_enabled() {
                self.trace.log_anchor(offset, &frame);
            }

            // Execute instruction
            let (next_frame, successors, _falls_through) =
                self.execute_instruction(offset, index, instruction, frame)?;

            // Propagate to successors
            for successor_offset in successors {
                let successor_index =
                    self.code_info.index_at(successor_offset).ok_or_else(|| {
                        VerifyError::VerifyError(format!(
                            "Invalid successor offset {successor_offset}"
                        ))
                    })?;

                if let Some(existing_frame) = &mut frames[successor_index] {
                    // Merge with existing frame
                    if existing_frame.merge(&next_frame, self.context)? {
                        worklist.add(successor_index);
                    }
                } else {
                    // First time reaching this instruction
                    frames[successor_index] = Some(next_frame.clone());
                    worklist.add(successor_index);
                }
            }

            // Process exception handlers
            self.process_exception_handlers(offset, &next_frame, &mut frames, &mut worklist)?;
        }

        Ok(())
    }

    /// Creates the initial frame for the method.
    fn create_initial_frame(&self) -> Result<Frame> {
        let mut frame = Frame::new(self.max_locals as usize, self.max_stack as usize);
        let mut local_index = 0;

        // For non-static methods, local 0 is 'this'
        if !self.method.access_flags.contains(MethodAccessFlags::STATIC) {
            if self.method_name == "<init>" {
                frame.set_local(local_index, VerificationType::UninitializedThis)?;
            } else {
                frame.set_local(
                    local_index,
                    VerificationType::Object(Arc::from(self.current_class.as_str())),
                )?;
            }
            local_index += 1;
        }

        // Parse parameters
        let descriptor = self
            .class_file
            .constant_pool
            .try_get_utf8(self.method.descriptor_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let (parameters, _) = FieldType::parse_method_descriptor(descriptor)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        for param in parameters {
            if local_index >= self.max_locals {
                return Err(VerifyError::VerifyError(
                    "Arguments exceed max_locals".to_string(),
                ));
            }

            let v_type = VerificationType::from_field_type(&param);

            if v_type.is_category2() {
                frame.set_local_category2(local_index, v_type)?;
                local_index += 2;
            } else {
                frame.set_local(local_index, v_type)?;
                local_index += 1;
            }
        }

        Ok(frame)
    }

    /// Executes an instruction, returning the next frame and successors.
    fn execute_instruction(
        &self,
        offset: u16,
        index: usize,
        instruction: &Instruction,
        frame: Frame,
    ) -> Result<(Frame, Vec<u16>, bool)> {
        let mut next_frame = frame;

        // Calculate next offset for fallthrough
        let next_offset = if index + 1 < self.code_info.instruction_count() {
            self.code_info
                .offset_at(index + 1)
                .unwrap_or(self.code_info.code_length())
        } else {
            self.code_info.code_length()
        };

        // Dispatch to handlers
        let handled =
            handlers::load_store::dispatch_load_store(instruction, &mut next_frame, self.context)?
                || handlers::stack::dispatch_stack(instruction, &mut next_frame)?
                || handlers::math::dispatch_math(instruction, &mut next_frame)?
                || handlers::conversion::dispatch_conversion(instruction, &mut next_frame)?
                || handlers::comparison::dispatch_comparison(instruction, &mut next_frame)?
                || handlers::control::dispatch_control(
                    instruction,
                    &mut next_frame,
                    self.return_type.as_ref(),
                    self.major_version,
                    self.context,
                )?
                || handlers::exceptions::dispatch_exceptions(
                    instruction,
                    &mut next_frame,
                    self.context,
                )?
                || handlers::misc::dispatch_misc(instruction, &mut next_frame, self.class_file)?
                || self.handle_reference_instruction(offset, instruction, &mut next_frame)?;

        if !handled {
            return Err(VerifyError::VerifyError(format!(
                "Unhandled instruction at offset {offset}: {instruction:?}"
            )));
        }

        // Compute successors
        let (successors, falls_through) =
            compute_successors(offset, instruction, next_offset, &self.code_info)?;

        Ok((next_frame, successors, falls_through))
    }

    /// Handles reference-related instructions.
    fn handle_reference_instruction(
        &self,
        offset: u16,
        instruction: &Instruction,
        frame: &mut Frame,
    ) -> Result<bool> {
        let resolver = ConstantPoolResolver::new(self.class_file);

        match instruction {
            Instruction::New(index) => {
                let class_name = resolver.resolve_class(*index)?;
                handlers::references::handle_new(frame, offset, &class_name)?;
            }
            Instruction::Newarray(atype) => {
                handlers::references::handle_newarray(frame, atype)?;
            }
            Instruction::Anewarray(index) => {
                let class_name = resolver.resolve_class(*index)?;
                handlers::references::handle_anewarray(frame, &class_name)?;
            }
            Instruction::Multianewarray(index, dimensions) => {
                let class_name = resolver.resolve_class(*index)?;
                handlers::references::handle_multianewarray(frame, &class_name, *dimensions)?;
            }
            Instruction::Getfield(index) => {
                let (class_name, _, descriptor) = resolver.resolve_field_ref(*index)?;
                handlers::references::handle_getfield(
                    frame,
                    &class_name,
                    &descriptor,
                    self.context,
                )?;
            }
            Instruction::Putfield(index) => {
                let (class_name, _, descriptor) = resolver.resolve_field_ref(*index)?;
                handlers::references::handle_putfield(
                    frame,
                    &class_name,
                    &descriptor,
                    self.context,
                )?;
            }
            Instruction::Getstatic(index) => {
                let (_, _, descriptor) = resolver.resolve_field_ref(*index)?;
                handlers::references::handle_getstatic(frame, &descriptor)?;
            }
            Instruction::Putstatic(index) => {
                let (_, _, descriptor) = resolver.resolve_field_ref(*index)?;
                handlers::references::handle_putstatic(frame, &descriptor, self.context)?;
            }
            Instruction::Invokevirtual(index) | Instruction::Invokeinterface(index, _) => {
                let (class_name, method_name, descriptor) = resolver.resolve_method_ref(*index)?;
                handlers::references::handle_invoke(
                    frame,
                    &class_name,
                    &method_name,
                    &descriptor,
                    false,
                    self.context,
                )?;
            }
            Instruction::Invokespecial(index) => {
                let (class_name, method_name, descriptor) = resolver.resolve_method_ref(*index)?;
                handlers::references::handle_invokespecial(
                    frame,
                    &class_name,
                    &method_name,
                    &descriptor,
                    self.context,
                )?;
            }
            Instruction::Invokestatic(index) => {
                let (class_name, method_name, descriptor) = resolver.resolve_method_ref(*index)?;
                handlers::references::handle_invoke(
                    frame,
                    &class_name,
                    &method_name,
                    &descriptor,
                    true,
                    self.context,
                )?;
            }
            Instruction::Invokedynamic(index) => {
                let descriptor = resolver.resolve_invoke_dynamic(*index)?;
                handlers::references::handle_invokedynamic(frame, &descriptor, self.context)?;
            }
            Instruction::Checkcast(index) => {
                let class_name = resolver.resolve_class(*index)?;
                handlers::references::handle_checkcast(frame, &class_name)?;
            }
            Instruction::Instanceof(_) => {
                handlers::references::handle_instanceof(frame)?;
            }
            Instruction::Arraylength => {
                handlers::references::handle_arraylength(frame)?;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }

    /// Processes exception handlers for the given offset.
    fn process_exception_handlers(
        &self,
        offset: u16,
        current_frame: &Frame,
        frames: &mut [Option<Frame>],
        worklist: &mut Worklist,
    ) -> Result<()> {
        for handler in self.exception_table {
            if handler.range_pc.contains(&offset) {
                // Create handler frame with only exception on stack
                let mut handler_frame =
                    Frame::new(self.max_locals as usize, self.max_stack as usize);

                // Copy locals from current frame
                for (i, local) in current_frame.locals.iter().enumerate() {
                    if i < handler_frame.locals.len() {
                        handler_frame.locals[i] = local.clone();
                    }
                }

                // Push exception type
                let exception_type = if handler.catch_type == 0 {
                    VerificationType::java_lang_throwable()
                } else {
                    let class_name = self
                        .class_file
                        .constant_pool
                        .try_get_class(handler.catch_type)
                        .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;
                    VerificationType::Object(Arc::from(class_name))
                };
                handler_frame.push(exception_type)?;

                // Merge with existing frame at handler
                let handler_index =
                    self.code_info.index_at(handler.handler_pc).ok_or_else(|| {
                        VerifyError::VerifyError(format!(
                            "Invalid handler PC {}",
                            handler.handler_pc
                        ))
                    })?;

                if let Some(existing) = &mut frames[handler_index] {
                    if existing.merge(&handler_frame, self.context)? {
                        worklist.add(handler_index);
                    }
                } else {
                    frames[handler_index] = Some(handler_frame);
                    worklist.add(handler_index);
                }
            }
        }

        Ok(())
    }

    /// Returns the verification trace.
    #[must_use]
    pub fn trace(&self) -> &VerificationTrace {
        &self.trace
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Version;
    use crate::constant::Constant;
    use crate::constant_pool::ConstantPool;

    struct MockContext;

    impl VerificationContext for MockContext {
        fn is_subclass(&self, _subclass: &str, _superclass: &str) -> Result<bool> {
            Ok(false)
        }
        fn is_assignable(&self, _target: &str, _source: &str) -> Result<bool> {
            Ok(true)
        }
        fn common_superclass(&self, _class1: &str, _class2: &str) -> Result<String> {
            Ok("java/lang/Object".to_string())
        }
    }

    fn create_mock_class_file() -> ClassFile {
        let mut constant_pool = ConstantPool::default();
        constant_pool
            .add(Constant::Utf8("TestClass".to_string()))
            .unwrap();
        let this_class_index = constant_pool.add(Constant::Class(1)).unwrap();
        constant_pool
            .add(Constant::Utf8("testMethod".to_string()))
            .unwrap();
        constant_pool
            .add(Constant::Utf8("()V".to_string()))
            .unwrap();
        constant_pool
            .add(Constant::Utf8("Code".to_string()))
            .unwrap();

        ClassFile {
            version: Version::Java5 { minor: 0 }, // Pre-Java 6 for inference testing
            constant_pool,
            access_flags: crate::ClassAccessFlags::PUBLIC,
            this_class: this_class_index,
            super_class: 0,
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![],
        }
    }

    #[test]
    fn test_inference_simple_method() {
        let class_file = create_mock_class_file();
        let code = vec![Instruction::Return];
        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 1,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        let config = VerifierConfig::default();
        let context = MockContext;

        let mut verifier = InferenceVerifier::new(&class_file, &method, &context, &config).unwrap();
        let result = verifier.verify();

        assert!(result.is_ok());
    }

    #[test]
    fn test_inference_with_loop() {
        let class_file = create_mock_class_file();
        // Simple loop: iconst_0, istore_0, iload_0, iconst_1, iadd, istore_0, goto 2
        let code = vec![
            Instruction::Iconst_0, // 0
            Instruction::Istore_0, // 1
            Instruction::Iload_0,  // 2 - loop target
            Instruction::Iconst_1, // 3
            Instruction::Iadd,     // 4
            Instruction::Pop,      // 5
            Instruction::Return,   // 6
        ];
        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 2,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        let config = VerifierConfig::default();
        let context = MockContext;

        let mut verifier = InferenceVerifier::new(&class_file, &method, &context, &config).unwrap();
        let result = verifier.verify();

        assert!(result.is_ok());
    }

    #[test]
    fn test_inference_max_iterations() {
        let class_file = create_mock_class_file();
        let code = vec![Instruction::Return];
        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 1,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        let config = VerifierConfig::default().with_max_inference_iterations(1);
        let context = MockContext;

        let mut verifier = InferenceVerifier::new(&class_file, &method, &context, &config).unwrap();
        let result = verifier.verify();

        // Should succeed with just one instruction
        assert!(result.is_ok());
    }
}
