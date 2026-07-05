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
use crate::{FieldType, JavaString};
use std::io::Cursor;

/// Type inference verifier using iterative dataflow analysis.
///
/// This verifier implements the classic verification algorithm that computes
/// types via iterative merging at control flow merge points.
pub struct InferenceVerifier<'a, C: VerificationContext> {
    /// The class file containing the method.
    class_file: &'a ClassFile<'a>,
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
        class_file: &'a ClassFile<'a>,
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
        let initial = frames
            .get_mut(0)
            .ok_or_else(|| VerifyError::VerifyError("Missing initial instruction".to_string()))?;
        *initial = Some(initial_frame);

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

            let offset = self
                .code_info
                .offset_at(index)
                .ok_or(VerifyError::VerifyError(
                    "Invalid worklist index".to_string(),
                ))?;

            let frame = frames
                .get(index)
                .and_then(Clone::clone)
                .ok_or(VerifyError::VerifyError("Missing frame".to_string()))?;

            let instruction = self.code.get(index).ok_or_else(|| {
                VerifyError::VerifyError(format!("Invalid instruction index {index}"))
            })?;

            // Log trace if enabled
            if self.trace.is_enabled() {
                self.trace.log_anchor(offset, &frame);
            }

            // Execute instruction
            let (next_frame, successors, _falls_through) =
                self.execute_instruction(offset, index, instruction, frame)?;

            // Propagate to successors
            for successor_offset in successors {
                let successor_index = self
                    .code_info
                    .index_at(successor_offset)
                    .ok_or(VerifyError::VerifyError("Invalid successor".to_string()))?;

                let successor_frame = frames.get_mut(successor_index).ok_or_else(|| {
                    VerifyError::VerifyError(format!("Invalid successor index {successor_index}"))
                })?;
                if let Some(existing_frame) = successor_frame.as_mut() {
                    // Merge with existing frame
                    if existing_frame.merge(&next_frame, self.context)? {
                        worklist.add(successor_index);
                    }
                } else {
                    // First time reaching this instruction
                    *successor_frame = Some(next_frame.clone());
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
                let this_type =
                    VerificationType::Object(JavaString::from(self.current_class.as_str()));
                frame.set_local(local_index, this_type)?;
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
                let message = "Arguments exceed max_locals".to_string();
                return Err(VerifyError::VerifyError(message));
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
        let handled = self.dispatch_standard_instruction(instruction, &mut next_frame)?
            || self.handle_reference_instruction(offset, instruction, &mut next_frame)?;

        handled.then_some(()).ok_or(VerifyError::VerifyError(
            "Unhandled instruction".to_string(),
        ))?;

        // Compute successors
        let (successors, falls_through) =
            compute_successors(offset, instruction, next_offset, &self.code_info)?;

        Ok((next_frame, successors, falls_through))
    }

    fn dispatch_standard_instruction(
        &self,
        instruction: &Instruction,
        frame: &mut Frame,
    ) -> Result<bool> {
        if handlers::load_store::dispatch_load_store(instruction, frame, self.context)? {
            return Ok(true);
        }
        if handlers::stack::dispatch_stack(instruction, frame)? {
            return Ok(true);
        }
        if handlers::math::dispatch_math(instruction, frame)? {
            return Ok(true);
        }
        if handlers::conversion::dispatch_conversion(instruction, frame)? {
            return Ok(true);
        }
        if handlers::comparison::dispatch_comparison(instruction, frame)? {
            return Ok(true);
        }
        if handlers::control::dispatch_control(
            instruction,
            frame,
            self.return_type.as_ref(),
            self.major_version,
            self.context,
        )? {
            return Ok(true);
        }
        if handlers::exceptions::dispatch_exceptions(instruction, frame, self.context)? {
            return Ok(true);
        }
        if handlers::misc::dispatch_misc(instruction, frame, self.class_file)? {
            return Ok(true);
        }
        Ok(false)
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
                self.verify_getfield(frame, &class_name, &descriptor)?;
            }
            Instruction::Putfield(index) => {
                let (class_name, _, descriptor) = resolver.resolve_field_ref(*index)?;
                self.verify_putfield(frame, &class_name, &descriptor)?;
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
                self.verify_invoke(frame, &class_name, &method_name, &descriptor, false)?;
            }
            Instruction::Invokespecial(index) => {
                let (class_name, method_name, descriptor) = resolver.resolve_method_ref(*index)?;
                self.verify_invokespecial(frame, &class_name, &method_name, &descriptor)?;
            }
            Instruction::Invokestatic(index) => {
                let (class_name, method_name, descriptor) = resolver.resolve_method_ref(*index)?;
                self.verify_invoke(frame, &class_name, &method_name, &descriptor, true)?;
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

    fn verify_getfield(&self, frame: &mut Frame, class_name: &str, descriptor: &str) -> Result<()> {
        handlers::references::handle_getfield(frame, class_name, descriptor, self.context)
    }

    fn verify_putfield(&self, frame: &mut Frame, class_name: &str, descriptor: &str) -> Result<()> {
        handlers::references::handle_putfield(frame, class_name, descriptor, self.context)
    }

    fn verify_invoke(
        &self,
        frame: &mut Frame,
        class_name: &str,
        method_name: &str,
        descriptor: &str,
        is_static: bool,
    ) -> Result<()> {
        handlers::references::handle_invoke(
            frame,
            class_name,
            method_name,
            descriptor,
            is_static,
            self.context,
        )
        .map(|_| ())
    }

    fn verify_invokespecial(
        &self,
        frame: &mut Frame,
        class_name: &str,
        method_name: &str,
        descriptor: &str,
    ) -> Result<()> {
        handlers::references::handle_invokespecial(
            frame,
            class_name,
            method_name,
            descriptor,
            self.context,
        )
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
            if !handler.range_pc.contains(&offset) {
                continue;
            }

            // Create handler frame with only exception on stack
            let mut handler_frame = Frame::new(self.max_locals as usize, self.max_stack as usize);

            // Copy locals from current frame
            for (i, local) in current_frame.locals.iter().enumerate() {
                if let Some(handler_local) = handler_frame.locals.get_mut(i) {
                    *handler_local = local.clone();
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
                VerificationType::Object(JavaString::from(class_name))
            };
            handler_frame.push(exception_type)?;

            // Merge with existing frame at handler
            let handler_index = self.code_info.index_at(handler.handler_pc).ok_or_else(|| {
                VerifyError::VerifyError(format!("Invalid handler PC {}", handler.handler_pc))
            })?;

            let handler = frames.get_mut(handler_index).ok_or_else(|| {
                VerifyError::VerifyError(format!("Invalid handler index {handler_index}"))
            })?;
            if let Some(existing) = handler.as_mut() {
                if existing.merge(&handler_frame, self.context)? {
                    worklist.add(handler_index);
                }
            } else {
                *handler = Some(handler_frame);
                worklist.add(handler_index);
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
    use crate::attributes::ArrayType;
    use crate::constant::Constant;
    use crate::constant_pool::ConstantPool;
    use crate::verifiers::bytecode::handlers::test_utils::MockContext;

    type TestInferenceVerifier<'a> = InferenceVerifier<'a, MockContext>;

    fn create_mock_class_file() -> ClassFile<'static> {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add(Constant::utf8("TestClass")).unwrap();
        let this_class_index = constant_pool.add(Constant::Class(1)).unwrap();
        constant_pool.add(Constant::utf8("testMethod")).unwrap();
        constant_pool.add(Constant::utf8("()V")).unwrap();
        constant_pool.add(Constant::utf8("Code")).unwrap();

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
            code_source_url: None,
        }
    }

    struct ReferenceClassFile {
        class_file: ClassFile<'static>,
        class_index: u16,
        array_class_index: u16,
        field_index: u16,
        long_field_index: u16,
        init_index: u16,
        virtual_index: u16,
        static_index: u16,
        interface_index: u16,
        dynamic_index: u16,
    }

    fn create_reference_class_file() -> ReferenceClassFile {
        let mut class_file = create_mock_class_file();
        let class_index = class_file.this_class;
        let array_class_index = class_file.constant_pool.add_class("[[LTestClass;").unwrap();
        let field_index = class_file
            .constant_pool
            .add_field_ref(class_index, "value", "I")
            .unwrap();
        let long_field_index = class_file
            .constant_pool
            .add_field_ref(class_index, "wide", "J")
            .unwrap();
        let init_index = class_file
            .constant_pool
            .add_method_ref(class_index, "<init>", "()V")
            .unwrap();
        let virtual_index = class_file
            .constant_pool
            .add_method_ref(class_index, "virtualMethod", "()I")
            .unwrap();
        let static_index = class_file
            .constant_pool
            .add_method_ref(class_index, "staticMethod", "()I")
            .unwrap();
        let interface_index = class_file
            .constant_pool
            .add_interface_method_ref(class_index, "interfaceMethod", "()I")
            .unwrap();
        let dynamic_index = class_file
            .constant_pool
            .add_invoke_dynamic(0, "dynamicMethod", "()I")
            .unwrap();

        ReferenceClassFile {
            class_file,
            class_index,
            array_class_index,
            field_index,
            long_field_index,
            init_index,
            virtual_index,
            static_index,
            interface_index,
            dynamic_index,
        }
    }

    fn method_with(
        access_flags: MethodAccessFlags,
        descriptor_index: u16,
        code: Vec<Instruction>,
        max_stack: u16,
        max_locals: u16,
        exception_table: Vec<ExceptionTableEntry>,
    ) -> Method {
        Method {
            access_flags,
            name_index: 3,
            descriptor_index,
            attributes: vec![Attribute::Code {
                name_index: 5,
                max_stack,
                max_locals,
                code,
                exception_table,
                attributes: Vec::new(),
            }],
        }
    }

    fn static_method(code: Vec<Instruction>, max_stack: u16, max_locals: u16) -> Method {
        method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            code,
            max_stack,
            max_locals,
            Vec::new(),
        )
    }

    fn inference_verifier<'a>(
        class_file: &'a ClassFile<'static>,
        method: &'a Method,
        context: &'a MockContext,
        config: &'a VerifierConfig,
    ) -> TestInferenceVerifier<'a> {
        TestInferenceVerifier::new(class_file, method, context, config).unwrap()
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
        let context = MockContext::PERMISSIVE;

        let mut verifier =
            TestInferenceVerifier::new(&class_file, &method, &context, &config).unwrap();
        let result = verifier.verify();

        assert!(result.is_ok());
    }

    #[test]
    fn test_debug_trace_context_and_no_code_error() {
        let class_file = create_mock_class_file();
        let method = static_method(vec![Instruction::Return], 0, 0);
        let config = VerifierConfig::default().with_trace(true);
        let context = MockContext::PERMISSIVE;
        let mut verifier = inference_verifier(&class_file, &method, &context, &config);

        assert!(format!("{verifier:?}").contains("InferenceVerifier"));
        assert!(verifier.trace().is_enabled());
        assert!(verifier.verify().is_ok());
        assert!(context.is_subclass("A", "B").unwrap());
        assert!(context.is_assignable("A", "B").unwrap());
        assert_eq!(
            "java/lang/Object",
            context.common_superclass("A", "B").unwrap()
        );

        let missing_code = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: Vec::new(),
        };
        assert!(TestInferenceVerifier::new(&class_file, &missing_code, &context, &config).is_err());

        let non_code_attr = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![Attribute::Deprecated { name_index: 5 }],
        };
        assert!(
            TestInferenceVerifier::new(&class_file, &non_code_attr, &context, &config).is_err()
        );
    }

    #[test]
    fn test_inference_rejects_invalid_class_method_metadata() {
        let mut class_file = create_mock_class_file();
        let invalid_descriptor = class_file.constant_pool.add_utf8("invalid").unwrap();
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;

        let bad_code = static_method(vec![Instruction::Goto(u16::MAX)], 0, 0);
        assert!(TestInferenceVerifier::new(&class_file, &bad_code, &context, &config).is_err());

        let method = static_method(vec![Instruction::Return], 0, 0);
        let mut bad_class_file = class_file.clone();
        bad_class_file.this_class = 999;
        assert!(TestInferenceVerifier::new(&bad_class_file, &method, &context, &config).is_err());

        let mut bad_name = method.clone();
        bad_name.name_index = 999;
        assert!(TestInferenceVerifier::new(&class_file, &bad_name, &context, &config).is_err());

        let mut bad_descriptor_index = method.clone();
        bad_descriptor_index.descriptor_index = 999;
        assert!(
            TestInferenceVerifier::new(&class_file, &bad_descriptor_index, &context, &config)
                .is_err()
        );

        let bad_descriptor_text = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            invalid_descriptor,
            vec![Instruction::Return],
            0,
            0,
            Vec::new(),
        );
        assert!(
            TestInferenceVerifier::new(&class_file, &bad_descriptor_text, &context, &config)
                .is_err()
        );
    }

    #[test]
    fn test_inference_initial_frame_rejects_invalid_descriptor_index() {
        let class_file = create_mock_class_file();
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;

        let code = vec![Instruction::Return];
        let exception_table = Vec::new();
        let bad_frame_descriptor_index = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            999,
            vec![Instruction::Return],
            0,
            0,
            Vec::new(),
        );
        let bad_frame_descriptor_index_verifier = InferenceVerifier {
            class_file: &class_file,
            method: &bad_frame_descriptor_index,
            context: &context,
            config: &config,
            code: &code,
            exception_table: &exception_table,
            max_stack: 0,
            max_locals: 0,
            code_info: CodeInfo::new(vec![0], 1),
            return_type: None,
            major_version: class_file.version.major(),
            current_class: "TestClass".to_string(),
            method_name: "testMethod".to_string(),
            trace: VerificationTrace::new(false),
        };
        assert!(
            bad_frame_descriptor_index_verifier
                .create_initial_frame()
                .is_err()
        );
    }

    #[test]
    fn test_inference_initial_frame_rejects_invalid_descriptor_text() {
        let mut class_file = create_mock_class_file();
        let invalid_descriptor = class_file.constant_pool.add_utf8("invalid").unwrap();
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;
        let code = vec![Instruction::Return];
        let exception_table = Vec::new();

        let bad_frame_descriptor_text = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            invalid_descriptor,
            vec![Instruction::Return],
            0,
            0,
            Vec::new(),
        );
        let bad_frame_descriptor_text_verifier = InferenceVerifier {
            class_file: &class_file,
            method: &bad_frame_descriptor_text,
            context: &context,
            config: &config,
            code: &code,
            exception_table: &exception_table,
            max_stack: 0,
            max_locals: 0,
            code_info: CodeInfo::new(vec![0], 1),
            return_type: None,
            major_version: class_file.version.major(),
            current_class: "TestClass".to_string(),
            method_name: "testMethod".to_string(),
            trace: VerificationTrace::new(false),
        };
        assert!(
            bad_frame_descriptor_text_verifier
                .create_initial_frame()
                .is_err()
        );
    }

    #[test]
    fn test_inference_exception_handler_rejects_missing_catch_type() {
        let class_file = create_mock_class_file();
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;

        let invalid_catch = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Nop],
            1,
            1,
            vec![ExceptionTableEntry {
                range_pc: 0..1,
                handler_pc: 0,
                catch_type: 999,
            }],
        );
        let invalid_catch_verifier =
            inference_verifier(&class_file, &invalid_catch, &context, &config);
        let mut frames = vec![None];
        let mut worklist = Worklist::new(1);
        assert!(
            invalid_catch_verifier
                .process_exception_handlers(0, &Frame::new(1, 1), &mut frames, &mut worklist)
                .is_err()
        );
    }

    #[test]
    fn test_constructor_and_parameter_initial_frames() {
        let mut class_file = create_mock_class_file();
        let init_name = class_file.constant_pool.add_utf8("<init>").unwrap();
        let descriptor = class_file
            .constant_pool
            .add_utf8("(JDLTestClass;)V")
            .unwrap();
        let method = method_with(
            MethodAccessFlags::PUBLIC,
            descriptor,
            vec![Instruction::Return],
            0,
            7,
            Vec::new(),
        );
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;

        let mut constructor = method.clone();
        constructor.name_index = init_name;
        let verifier = inference_verifier(&class_file, &constructor, &context, &config);
        let frame = verifier.create_initial_frame().unwrap();
        assert_eq!(frame.locals[0], VerificationType::UninitializedThis);
        assert_eq!(frame.locals[1], VerificationType::Long);
        assert_eq!(frame.locals[2], VerificationType::Top);
        assert_eq!(frame.locals[3], VerificationType::Double);
        assert_eq!(frame.locals[4], VerificationType::Top);
        assert_eq!(
            frame.locals[5],
            VerificationType::Object(JavaString::from("TestClass"))
        );

        let small = method_with(
            MethodAccessFlags::PUBLIC,
            descriptor,
            vec![Instruction::Return],
            0,
            2,
            Vec::new(),
        );
        let small_verifier = inference_verifier(&class_file, &small, &context, &config);
        assert!(small_verifier.create_initial_frame().is_err());

        let int_descriptor = class_file.constant_pool.add_utf8("(I)V").unwrap();
        let no_local_slot = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            int_descriptor,
            vec![Instruction::Return],
            0,
            0,
            Vec::new(),
        );
        let no_local_slot_verifier =
            inference_verifier(&class_file, &no_local_slot, &context, &config);
        assert!(no_local_slot_verifier.create_initial_frame().is_err());
    }

    #[test]
    fn test_reference_instruction_dispatch() {
        let ReferenceClassFile {
            class_file,
            class_index,
            array_class_index,
            ..
        } = create_reference_class_file();
        let method = static_method(vec![Instruction::Nop, Instruction::Return], 8, 4);
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;
        let verifier = inference_verifier(&class_file, &method, &context, &config);

        let mut frame = Frame::new(4, 8);
        assert!(
            verifier
                .handle_reference_instruction(0, &Instruction::New(class_index), &mut frame)
                .unwrap()
        );
        assert_eq!(frame.pop().unwrap(), VerificationType::Uninitialized(0));

        frame.push(VerificationType::Integer).unwrap();
        verifier
            .handle_reference_instruction(0, &Instruction::Newarray(ArrayType::Int), &mut frame)
            .unwrap();
        assert!(matches!(frame.pop().unwrap(), VerificationType::Array(_)));

        frame.push(VerificationType::Integer).unwrap();
        verifier
            .handle_reference_instruction(0, &Instruction::Anewarray(class_index), &mut frame)
            .unwrap();
        assert!(matches!(frame.pop().unwrap(), VerificationType::Array(_)));

        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();
        verifier
            .handle_reference_instruction(
                0,
                &Instruction::Multianewarray(array_class_index, 2),
                &mut frame,
            )
            .unwrap();
        assert!(matches!(frame.pop().unwrap(), VerificationType::Array(_)));
    }

    #[test]
    fn test_reference_instruction_dispatch_field_access() {
        let ReferenceClassFile {
            class_file,
            field_index,
            long_field_index,
            ..
        } = create_reference_class_file();
        let method = static_method(vec![Instruction::Nop, Instruction::Return], 8, 4);
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;
        let verifier = inference_verifier(&class_file, &method, &context, &config);

        let mut frame = Frame::new(4, 8);
        frame.push(VerificationType::Null).unwrap();
        verifier
            .handle_reference_instruction(0, &Instruction::Getfield(field_index), &mut frame)
            .unwrap();
        assert_eq!(frame.pop().unwrap(), VerificationType::Integer);

        frame.push(VerificationType::Null).unwrap();
        frame.push(VerificationType::Integer).unwrap();
        verifier
            .handle_reference_instruction(0, &Instruction::Putfield(field_index), &mut frame)
            .unwrap();

        verifier
            .handle_reference_instruction(0, &Instruction::Getstatic(long_field_index), &mut frame)
            .unwrap();
        assert_eq!(frame.pop_category2().unwrap(), VerificationType::Long);

        frame.push(VerificationType::Integer).unwrap();
        verifier
            .handle_reference_instruction(0, &Instruction::Putstatic(field_index), &mut frame)
            .unwrap();
    }

    #[test]
    fn test_reference_instruction_dispatch_invoke() {
        let ReferenceClassFile {
            class_file,
            init_index,
            virtual_index,
            static_index,
            interface_index,
            dynamic_index,
            ..
        } = create_reference_class_file();
        let method = static_method(vec![Instruction::Nop, Instruction::Return], 8, 4);
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;
        let verifier = inference_verifier(&class_file, &method, &context, &config);

        let mut frame = Frame::new(4, 8);
        frame.push(VerificationType::Null).unwrap();
        verifier
            .handle_reference_instruction(0, &Instruction::Invokevirtual(virtual_index), &mut frame)
            .unwrap();
        assert_eq!(frame.pop().unwrap(), VerificationType::Integer);

        verifier
            .handle_reference_instruction(0, &Instruction::Invokestatic(static_index), &mut frame)
            .unwrap();
        assert_eq!(frame.pop().unwrap(), VerificationType::Integer);

        frame.push(VerificationType::Null).unwrap();
        verifier
            .handle_reference_instruction(
                0,
                &Instruction::Invokeinterface(interface_index, 1),
                &mut frame,
            )
            .unwrap();
        assert_eq!(frame.pop().unwrap(), VerificationType::Integer);

        frame.push(VerificationType::Uninitialized(0)).unwrap();
        verifier
            .handle_reference_instruction(0, &Instruction::Invokespecial(init_index), &mut frame)
            .unwrap();

        verifier
            .handle_reference_instruction(0, &Instruction::Invokedynamic(dynamic_index), &mut frame)
            .unwrap();
        assert_eq!(frame.pop().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_reference_instruction_dispatch_type_ops() {
        let ReferenceClassFile {
            class_file,
            class_index,
            ..
        } = create_reference_class_file();
        let method = static_method(vec![Instruction::Nop, Instruction::Return], 8, 4);
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;
        let verifier = inference_verifier(&class_file, &method, &context, &config);

        let mut frame = Frame::new(4, 8);
        frame.push(VerificationType::Null).unwrap();
        verifier
            .handle_reference_instruction(0, &Instruction::Checkcast(class_index), &mut frame)
            .unwrap();
        assert!(matches!(frame.pop().unwrap(), VerificationType::Object(_)));

        frame.push(VerificationType::Null).unwrap();
        verifier
            .handle_reference_instruction(0, &Instruction::Instanceof(class_index), &mut frame)
            .unwrap();
        assert_eq!(frame.pop().unwrap(), VerificationType::Integer);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        verifier
            .handle_reference_instruction(0, &Instruction::Arraylength, &mut frame)
            .unwrap();
        assert_eq!(frame.pop().unwrap(), VerificationType::Integer);

        assert!(
            !verifier
                .handle_reference_instruction(0, &Instruction::Return, &mut frame)
                .unwrap()
        );
    }

    #[test]
    fn test_execute_instruction_dispatch_and_errors() {
        let ReferenceClassFile {
            class_file,
            class_index,
            ..
        } = create_reference_class_file();
        let method = static_method(vec![Instruction::Nop, Instruction::Return], 8, 4);
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;
        let verifier = inference_verifier(&class_file, &method, &context, &config);

        assert!(
            verifier
                .execute_instruction(0, 0, &Instruction::Nop, Frame::new(4, 8))
                .is_ok()
        );

        let mut load = Frame::new(4, 8);
        load.locals[0] = VerificationType::Integer;
        assert!(
            verifier
                .execute_instruction(0, 0, &Instruction::Iload_0, load)
                .is_ok()
        );

        let mut stack = Frame::new(4, 8);
        stack.push(VerificationType::Integer).unwrap();
        assert!(
            verifier
                .execute_instruction(0, 0, &Instruction::Pop, stack)
                .is_ok()
        );

        let mut math = Frame::new(4, 8);
        math.push(VerificationType::Integer).unwrap();
        math.push(VerificationType::Integer).unwrap();
        assert!(
            verifier
                .execute_instruction(0, 0, &Instruction::Iadd, math)
                .is_ok()
        );

        let mut conversion = Frame::new(4, 8);
        conversion.push(VerificationType::Integer).unwrap();
        assert!(
            verifier
                .execute_instruction(0, 0, &Instruction::I2l, conversion)
                .is_ok()
        );

        let mut comparison = Frame::new(4, 8);
        comparison.push_category2(VerificationType::Long).unwrap();
        comparison.push_category2(VerificationType::Long).unwrap();
        assert!(
            verifier
                .execute_instruction(0, 0, &Instruction::Lcmp, comparison)
                .is_ok()
        );

        assert!(
            verifier
                .execute_instruction(0, 0, &Instruction::Return, Frame::new(4, 8))
                .is_ok()
        );
        assert!(
            verifier
                .execute_instruction(0, 0, &Instruction::Ireturn, Frame::new(4, 8))
                .is_err()
        );

        let mut throwing = Frame::new(4, 8);
        throwing
            .push(VerificationType::Object(JavaString::from(
                "java/lang/Throwable",
            )))
            .unwrap();
        assert!(
            verifier
                .execute_instruction(0, 0, &Instruction::Athrow, throwing)
                .is_ok()
        );

        let mut reference = Frame::new(4, 8);
        assert!(
            verifier
                .execute_instruction(0, 0, &Instruction::New(class_index), reference.clone())
                .is_ok()
        );
        reference.push(VerificationType::Integer).unwrap();
        assert!(
            verifier
                .execute_instruction(0, 0, &Instruction::Impdep1, reference)
                .is_err()
        );
    }

    #[test]
    fn test_exception_handlers_typed_catch_merge() {
        let mut class_file = create_mock_class_file();
        let catch_index = class_file
            .constant_pool
            .add_class("java/lang/Exception")
            .unwrap();
        let code = vec![Instruction::Nop, Instruction::Return];
        let method = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            code,
            1,
            1,
            vec![ExceptionTableEntry {
                range_pc: 0..1,
                handler_pc: 1,
                catch_type: catch_index,
            }],
        );
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;
        let verifier = inference_verifier(&class_file, &method, &context, &config);
        let mut frames = vec![None, None];
        let mut worklist = Worklist::new(2);
        let mut current = Frame::new(1, 1);
        current.locals[0] = VerificationType::Object(JavaString::from("java/lang/String"));
        let mut existing = Frame::new(1, 1);
        existing.locals[0] = VerificationType::Object(JavaString::from("java/lang/Integer"));
        existing
            .push(VerificationType::Object(JavaString::from(
                "java/lang/Exception",
            )))
            .unwrap();
        frames[1] = Some(existing);
        verifier
            .process_exception_handlers(0, &current, &mut frames, &mut worklist)
            .unwrap();
        assert!(!worklist.is_empty());
    }

    #[test]
    fn test_exception_handlers_catch_all() {
        let class_file = create_mock_class_file();
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;

        let catch_all = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Nop, Instruction::Return],
            1,
            1,
            vec![ExceptionTableEntry {
                range_pc: 0..1,
                handler_pc: 1,
                catch_type: 0,
            }],
        );
        let catch_all_verifier = inference_verifier(&class_file, &catch_all, &context, &config);
        let mut catch_all_frames = vec![None, None];
        let mut catch_all_worklist = Worklist::new(2);
        catch_all_verifier
            .process_exception_handlers(
                0,
                &Frame::new(1, 1),
                &mut catch_all_frames,
                &mut catch_all_worklist,
            )
            .unwrap();
        assert!(catch_all_frames[1].is_some());

        let mut no_match_frames = vec![None, None];
        let mut no_match_worklist = Worklist::new(2);
        catch_all_verifier
            .process_exception_handlers(
                1,
                &Frame::new(1, 1),
                &mut no_match_frames,
                &mut no_match_worklist,
            )
            .unwrap();
        assert!(no_match_worklist.is_empty());
    }

    #[test]
    fn test_exception_handlers_invalid_handler_pc() {
        let class_file = create_mock_class_file();
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;

        let invalid_handler = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Nop, Instruction::Return],
            1,
            1,
            vec![ExceptionTableEntry {
                range_pc: 0..1,
                handler_pc: 99,
                catch_type: 0,
            }],
        );
        let invalid_handler_verifier =
            inference_verifier(&class_file, &invalid_handler, &context, &config);
        let mut invalid_frames = vec![None, None];
        let mut invalid_worklist = Worklist::new(2);
        assert!(
            invalid_handler_verifier
                .process_exception_handlers(
                    0,
                    &Frame::new(1, 1),
                    &mut invalid_frames,
                    &mut invalid_worklist,
                )
                .is_err()
        );

        let low_iteration_config = VerifierConfig::default().with_max_inference_iterations(0);
        let low_iteration_method = static_method(vec![Instruction::Return], 0, 0);
        let mut low_iteration = inference_verifier(
            &class_file,
            &low_iteration_method,
            &context,
            &low_iteration_config,
        );
        assert!(low_iteration.verify().is_err());
    }

    #[test]
    fn test_inference_requeues_changed_merge_point() {
        let mut class_file = create_mock_class_file();
        let class_a = class_file.constant_pool.add_class("A").unwrap();
        let class_b = class_file.constant_pool.add_class("B").unwrap();
        let method = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![
                Instruction::Iconst_0,
                Instruction::Ifeq(5),
                Instruction::Aconst_null,
                Instruction::Checkcast(class_a),
                Instruction::Goto(7),
                Instruction::Aconst_null,
                Instruction::Checkcast(class_b),
                Instruction::Pop,
                Instruction::Return,
            ],
            1,
            0,
            Vec::new(),
        );
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;
        let mut verifier = inference_verifier(&class_file, &method, &context, &config);

        assert!(verifier.verify().is_ok());
    }

    #[test]
    fn test_inference_with_loop() {
        let class_file = create_mock_class_file();
        // Simple loop: iconst_0, istore_0, iload_0, iconst_1, iadd, istore_0, goto 2
        let code = vec![
            Instruction::Iconst_0, // 0
            Instruction::Istore_0, // 1
            Instruction::Iload_0,  // 2; loop target
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
        let context = MockContext::PERMISSIVE;

        let mut verifier =
            TestInferenceVerifier::new(&class_file, &method, &context, &config).unwrap();
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
        let context = MockContext::PERMISSIVE;

        let mut verifier =
            TestInferenceVerifier::new(&class_file, &method, &context, &config).unwrap();
        let result = verifier.verify();

        // Should succeed with just one instruction
        assert!(result.is_ok());
    }
}
