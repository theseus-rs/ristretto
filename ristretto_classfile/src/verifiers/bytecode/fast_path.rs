//! # `StackMapTable`-Driven Fast Path Verifier
//!
//! This module implements a fast "type-checking" verification strategy
//! that uses `StackMapTable` frames as trusted anchors, avoiding iterative dataflow.
//!
//! # Algorithm Overview
//!
//! 1. Pre-decode `StackMapTable` into canonical frames at specific bytecode offsets
//! 2. Verify control-flow shape with worklist, using `StackMapTable` frames as anchors
//! 3. At merge points, validate computed frame against `StackMapTable` frame
//! 4. Use a single mutable "current frame" for straight-line code
//!
//! # Key Optimization
//!
//! The fast path avoids iterative merges by enforcing that all merge points
//! have `StackMapTable` entries. Any time a merge would be needed without a
//! `StackMapTable` entry, verification fails (strict mode) or falls back to
//! type inference (compat mode).
//!
//! # References
//!
//! - [JVMS §4.10.1 - Verification by Type Checking](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1)

use crate::FieldType;
use crate::JavaStr;
use crate::JavaString;
use crate::attributes::{Attribute, ExceptionTableEntry, Instruction};
use crate::class_file::ClassFile;
use crate::method::Method;
use crate::method_access_flags::MethodAccessFlags;
use crate::verifiers::bytecode::config::VerifierConfig;
use crate::verifiers::bytecode::control_flow::{
    CodeInfo, compute_successors, validate_exception_table,
};
use crate::verifiers::bytecode::diagnostics::{VerificationDiagnostic, VerificationTrace};
use crate::verifiers::bytecode::frame::Frame;
use crate::verifiers::bytecode::handlers;
use crate::verifiers::bytecode::handlers::references::ConstantPoolResolver;
use crate::verifiers::bytecode::stackmap::DecodedStackMapTable;
use crate::verifiers::bytecode::type_system::VerificationType;
use crate::verifiers::context::VerificationContext;
use crate::verifiers::error::{Result, VerifyError};
use ahash::AHashSet;
use std::io::Cursor;

/// Extracted components from a Code attribute.
type CodeAttributeParts<'a> = (
    &'a Vec<Instruction>,
    u16,
    u16,
    &'a Vec<Attribute>,
    &'a Vec<ExceptionTableEntry>,
);

/// Result of fast-path verification attempt.
#[derive(Debug)]
pub enum FastPathResult {
    /// Verification succeeded.
    Success,
    /// Verification failed with the given error.
    Failed(VerifyError),
    /// Fast path cannot handle this method, need to fall back to inference.
    NeedsFallback(String),
}

/// Fast-path verifier using `StackMapTable`-driven type checking.
///
/// This verifier implements a fast verification algorithm that uses `StackMapTable` frames as
/// trusted anchors, performing a single pass through the bytecode without iterative dataflow.
pub struct FastPathVerifier<'a, C: VerificationContext> {
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
    /// The method descriptor.
    method_descriptor: String,
    /// Decoded `StackMapTable`.
    stack_map_table: DecodedStackMapTable,
    /// Initial frame at method entry.
    initial_frame: Frame,
    /// Set of jump targets (for merge point detection).
    jump_targets: AHashSet<u16>,
    /// Set of exception handler entry points.
    handler_entries: AHashSet<u16>,
    /// Verification trace (for verbose mode).
    trace: VerificationTrace,
}

impl<C: VerificationContext> std::fmt::Debug for FastPathVerifier<'_, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FastPathVerifier")
            .field("current_class", &self.current_class)
            .field("method_name", &self.method_name)
            .field("method_descriptor", &self.method_descriptor)
            .field("max_stack", &self.max_stack)
            .field("max_locals", &self.max_locals)
            .field("major_version", &self.major_version)
            .finish_non_exhaustive()
    }
}

impl<'a, C: VerificationContext> FastPathVerifier<'a, C> {
    /// Creates a new fast-path verifier.
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
        let (code, max_stack, max_locals, code_attributes, exception_table) =
            Self::extract_code_attribute(method)?;

        let code_info = Self::build_code_info(code)?;
        let (current_class, method_name, method_descriptor) =
            Self::extract_method_info(class_file, method)?;
        let method_descriptor_js = JavaStr::cow_from_str(&method_descriptor);
        let (_, return_type) = FieldType::parse_method_descriptor(&method_descriptor_js)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;
        let major_version = class_file.version.major();

        let initial_frame = Self::create_initial_frame_static(
            method,
            class_file,
            &current_class,
            &method_name,
            max_locals,
            max_stack,
        )?;
        let stack_map_table =
            Self::decode_stack_map_table(code_attributes, &initial_frame, class_file, max_stack)?;
        stack_map_table.validate_offsets(|offset| code_info.is_valid_offset(offset))?;

        let (jump_targets, handler_entries) =
            Self::collect_control_flow_targets(code, exception_table, &code_info);
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
            method_descriptor,
            stack_map_table,
            initial_frame,
            jump_targets,
            handler_entries,
            trace,
        })
    }

    /// Extracts the Code attribute from a method.
    fn extract_code_attribute(method: &'a Method) -> Result<CodeAttributeParts<'a>> {
        method
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
            })
    }

    /// Builds the code info structure from instructions.
    fn build_code_info(code: &[Instruction]) -> Result<CodeInfo> {
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
        Ok(CodeInfo::new(instruction_offsets, code_length))
    }

    /// Extracts method information from the constant pool.
    fn extract_method_info(
        class_file: &ClassFile<'_>,
        method: &Method,
    ) -> Result<(String, String, String)> {
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
        let method_descriptor = class_file
            .constant_pool
            .try_get_utf8(method.descriptor_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?
            .to_string();
        Ok((current_class, method_name, method_descriptor))
    }

    /// Decodes the `StackMapTable` attribute if present.
    fn decode_stack_map_table(
        code_attributes: &[Attribute],
        initial_frame: &Frame,
        class_file: &ClassFile<'_>,
        max_stack: u16,
    ) -> Result<DecodedStackMapTable> {
        let stack_frames = code_attributes.iter().find_map(|attr| {
            if let Attribute::StackMapTable { frames, .. } = attr {
                Some(frames)
            } else {
                None
            }
        });

        if let Some(frames) = stack_frames {
            DecodedStackMapTable::decode(frames, initial_frame, class_file, max_stack)
        } else {
            Ok(DecodedStackMapTable::empty())
        }
    }

    /// Collects jump targets and exception handler entries.
    fn collect_control_flow_targets(
        code: &[Instruction],
        exception_table: &[ExceptionTableEntry],
        code_info: &CodeInfo,
    ) -> (AHashSet<u16>, AHashSet<u16>) {
        let mut jump_targets = AHashSet::default();
        let mut handler_entries = AHashSet::default();

        for handler in exception_table {
            handler_entries.insert(handler.handler_pc);
        }

        let code_length = code_info.code_length();
        for (index, instruction) in code.iter().enumerate() {
            let offset = code_info.offset_at(index).unwrap_or(0);
            let next_offset = code_info.offset_at(index + 1).unwrap_or(code_length);

            if let Ok((successors, _)) =
                compute_successors(offset, instruction, next_offset, code_info)
            {
                for succ in successors {
                    if succ != next_offset {
                        jump_targets.insert(succ);
                    }
                }
            }
        }

        (jump_targets, handler_entries)
    }

    /// Creates the initial frame for the method (static helper).
    fn create_initial_frame_static(
        method: &Method,
        class_file: &ClassFile<'_>,
        current_class: &str,
        method_name: &str,
        max_locals: u16,
        max_stack: u16,
    ) -> Result<Frame> {
        let mut frame = Frame::new(max_locals as usize, max_stack as usize);
        let mut local_index = 0;

        // For non-static methods, local 0 is 'this'
        if !method.access_flags.contains(MethodAccessFlags::STATIC) {
            if method_name == "<init>" {
                frame.set_local(local_index, VerificationType::UninitializedThis)?;
            } else {
                let this_type = VerificationType::Object(JavaString::from(current_class));
                frame.set_local(local_index, this_type)?;
            }
            local_index += 1;
        }

        // Parse parameters
        let descriptor = class_file
            .constant_pool
            .try_get_utf8(method.descriptor_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let (parameters, _) = FieldType::parse_method_descriptor(descriptor)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        for param in parameters {
            if local_index >= max_locals {
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

    /// Creates the initial frame for the method.
    fn create_initial_frame(&self) -> Result<Frame> {
        Self::create_initial_frame_static(
            self.method,
            self.class_file,
            &self.current_class,
            &self.method_name,
            self.max_locals,
            self.max_stack,
        )
    }

    /// Verifies the method using the fast path algorithm.
    ///
    /// # Returns
    ///
    /// - `FastPathResult::Success` if verification passes
    /// - `FastPathResult::Failed(err)` if verification fails with an error
    /// - `FastPathResult::NeedsFallback(reason)` if fast path cannot handle this method
    pub fn verify(&mut self) -> FastPathResult {
        // Validate exception table
        if let Err(e) = validate_exception_table(self.exception_table, &self.code_info) {
            return FastPathResult::Failed(e);
        }

        // Check if we have StackMapTable for v50+ classes
        if self.config.requires_stackmap(self.major_version) && self.stack_map_table.is_empty() {
            // For classes that require StackMapTable but don't have one
            if !self.code.is_empty() && self.has_control_flow() {
                if self.config.allows_inference_fallback() {
                    return FastPathResult::NeedsFallback(
                        "StackMapTable required but not present".to_string(),
                    );
                }
                return FastPathResult::Failed(VerifyError::VerifyError(
                    "StackMapTable required for class file version 50+ with control flow"
                        .to_string(),
                ));
            }
        }

        // Perform the actual verification
        let initial_frame = self.initial_frame.clone();
        match self.verify_with_stackmaps(&initial_frame) {
            Ok(()) => FastPathResult::Success,
            Err(error) => {
                // Check if this is a fallback-eligible error
                if self.config.allows_inference_fallback() && Self::is_fallback_eligible(&error) {
                    FastPathResult::NeedsFallback(format!("Fast path failed: {error}"))
                } else {
                    FastPathResult::Failed(error)
                }
            }
        }
    }

    /// Checks if the method has any control flow beyond straight-line.
    ///
    /// Straight-line code (no jumps, no exception handlers) doesn't require
    /// `StackMapTable` even in Java 6+ class files.
    fn has_control_flow(&self) -> bool {
        !self.jump_targets.is_empty() || !self.handler_entries.is_empty()
    }

    /// Checks if an error is eligible for fallback to inference.
    fn is_fallback_eligible(error: &VerifyError) -> bool {
        match error {
            VerifyError::VerifyError(msg) => {
                let message = msg.to_ascii_lowercase();
                !message.contains("mismatch")
                    && (message.contains("merge point")
                        || message.contains("missing frame")
                        || message.contains("stackmaptable"))
            }
            _ => false,
        }
    }

    /// Core verification using StackMapTable-driven algorithm.
    fn verify_with_stackmaps(&mut self, initial_frame: &Frame) -> Result<()> {
        // State at each PC that is a merge point (jump target, handler entry, or stackmap frame)
        let mut anchor_states: Vec<Option<Frame>> = vec![None; self.code_info.instruction_count()];

        // Set initial frame at PC 0
        anchor_states[0] = Some(initial_frame.clone());

        // Pre-populate frames from StackMapTable
        for decoded in self.stack_map_table.frames() {
            if let Some(index) = self.code_info.index_at(decoded.offset) {
                let frame = self
                    .stack_map_table
                    .to_frame(decoded, self.max_locals, self.max_stack);
                anchor_states[index] = Some(frame);
            }
        }

        // Worklist of instruction indices to process
        let mut worklist: Vec<usize> = vec![0];
        let mut visited = vec![false; self.code_info.instruction_count()];

        while let Some(start_index) = worklist.pop() {
            if visited[start_index] {
                continue;
            }

            // Get the frame at this anchor
            let start_frame = anchor_states[start_index]
                .clone()
                .ok_or(VerifyError::VerifyError("Missing frame".to_string()))?;

            // Process straight-line code from this anchor
            let mut current_frame = start_frame;
            let mut index = start_index;

            while index < self.code.len() {
                let offset = self
                    .code_info
                    .offset_at(index)
                    .ok_or(VerifyError::VerifyError("Invalid index".to_string()))?;

                // If this is an anchor point (not the start), validate and use StackMapTable frame
                if index != start_index && self.is_anchor_point(offset) {
                    // Validate current frame against StackMapTable frame
                    if let Some(decoded) = self.stack_map_table.get(offset) {
                        let expected =
                            self.stack_map_table
                                .to_frame(decoded, self.max_locals, self.max_stack);
                        self.validate_frame_compatibility(&current_frame, &expected, offset)?;
                    }

                    // Add to worklist if not visited
                    if !visited[index] {
                        worklist.push(index);
                    }
                    break;
                }

                visited[index] = true;

                let instruction = &self.code[index];

                // Log trace if enabled
                if self.trace.is_enabled() {
                    let pre_frame = current_frame.clone();
                    self.trace.log_anchor(offset, &pre_frame);
                }

                // Execute instruction
                let next_offset = self
                    .code_info
                    .offset_at(index + 1)
                    .unwrap_or(self.code_info.code_length());

                let (next_frame, successors, falls_through) =
                    self.execute_instruction(offset, index, instruction, current_frame.clone())?;

                // Process exception handlers
                let handler_result = self.process_exception_handlers(
                    offset,
                    &next_frame,
                    &mut anchor_states,
                    &mut worklist,
                );
                handler_result?;

                // Handle successors
                for succ_offset in &successors {
                    if *succ_offset == next_offset && falls_through {
                        continue;
                    }
                    // This is a branch target
                    let succ_index = self
                        .code_info
                        .index_at(*succ_offset)
                        .ok_or(VerifyError::VerifyError("Invalid successor".to_string()))?;
                    if visited[succ_index] {
                        continue;
                    }

                    let successor_result = self.handle_successor(
                        succ_index,
                        *succ_offset,
                        &next_frame,
                        &mut anchor_states,
                    );
                    successor_result?;
                    worklist.push(succ_index);
                }

                // Continue straight-line execution if falls through
                if falls_through {
                    current_frame = next_frame;
                    index += 1;
                } else {
                    break;
                }
            }
        }

        Ok(())
    }

    /// Checks if an offset is an anchor point (`StackMapTable` frame, jump target, or handler).
    fn is_anchor_point(&self, offset: u16) -> bool {
        self.stack_map_table.has_frame_at(offset)
            || self.jump_targets.contains(&offset)
            || self.handler_entries.contains(&offset)
    }

    /// Checks if an offset is a merge point (multiple predecessors possible).
    fn is_merge_point(&self, offset: u16) -> bool {
        self.jump_targets.contains(&offset) || self.handler_entries.contains(&offset)
    }

    /// Handles a successor during verification.
    fn handle_successor(
        &self,
        succ_index: usize,
        succ_offset: u16,
        next_frame: &Frame,
        anchor_states: &mut [Option<Frame>],
    ) -> Result<()> {
        // Check if there's a StackMapTable frame
        if let Some(decoded) = self.stack_map_table.get(succ_offset) {
            let expected = self
                .stack_map_table
                .to_frame(decoded, self.max_locals, self.max_stack);
            self.validate_frame_compatibility(next_frame, &expected, succ_offset)?;
            anchor_states[succ_index] = Some(expected);
        } else if self.is_merge_point(succ_offset) {
            // Merge point without StackMapTable frame
            if self.config.fallback_strategy
                == crate::verifiers::bytecode::config::FallbackStrategy::Strict
            {
                return Err(VerifyError::VerifyError(format!(
                    "Merge point at offset {succ_offset} requires StackMapTable frame"
                )));
            }
            // Store computed frame for merge
            if let Some(existing) = &mut anchor_states[succ_index] {
                existing.merge(next_frame, self.context)?;
            } else {
                anchor_states[succ_index] = Some(next_frame.clone());
            }
        } else {
            anchor_states[succ_index] = Some(next_frame.clone());
        }
        Ok(())
    }

    /// Validates that the computed frame is compatible with the expected frame.
    fn validate_frame_compatibility(
        &self,
        computed: &Frame,
        expected: &Frame,
        offset: u16,
    ) -> Result<()> {
        // Stack depths must match
        if computed.stack.len() != expected.stack.len() {
            let computed_len = computed.stack.len();
            let expected_len = expected.stack.len();
            return Err(VerifyError::VerifyError(format!(
                "Stack depth mismatch at offset {offset}: computed {computed_len} vs expected {expected_len}"
            )));
        }

        // Validate stack types are assignable
        for (i, (comp_type, exp_type)) in computed.stack.iter().zip(&expected.stack).enumerate() {
            if comp_type.is_assignable_to(exp_type, self.context)? {
                continue;
            }
            let message = format!(
                "Stack type mismatch at offset {offset}, slot {i}: {comp_type} not assignable to {exp_type}"
            );
            return Err(VerifyError::VerifyError(message));
        }

        // Validate local types are assignable (for used locals)
        let min_locals = computed.locals.len().min(expected.locals.len());
        for i in 0..min_locals {
            let comp_type = &computed.locals[i];
            let exp_type = &expected.locals[i];

            // Top is compatible with anything
            if *comp_type == VerificationType::Top || *exp_type == VerificationType::Top {
                continue;
            }

            if comp_type.is_assignable_to(exp_type, self.context)? {
                continue;
            }
            let message = format!(
                "Local type mismatch at offset {offset}, local {i}: {comp_type} not assignable to {exp_type}"
            );
            return Err(VerifyError::VerifyError(message));
        }

        Ok(())
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
        anchor_states: &mut [Option<Frame>],
        worklist: &mut Vec<usize>,
    ) -> Result<()> {
        for handler in self.exception_table {
            if !handler.range_pc.contains(&offset) {
                continue;
            }
            let handler_index = self.code_info.index_at(handler.handler_pc).ok_or_else(|| {
                VerifyError::VerifyError(format!("Invalid handler PC {}", handler.handler_pc))
            })?;

            // Get exception type
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

            // Create handler frame: same locals, stack = [exception_type]
            let mut handler_frame = Frame::new(self.max_locals as usize, self.max_stack as usize);

            // Copy locals from current frame
            for (i, local) in current_frame.locals.iter().enumerate() {
                if i < handler_frame.locals.len() {
                    handler_frame.locals[i] = local.clone();
                }
            }

            // Push exception type
            handler_frame.push(exception_type)?;

            // Check if StackMapTable has a frame for this handler
            if let Some(decoded) = self.stack_map_table.get(handler.handler_pc) {
                let expected =
                    self.stack_map_table
                        .to_frame(decoded, self.max_locals, self.max_stack);
                // Validate handler frame compatibility
                // Note: We're lenient here; just check stack has exception type
                if expected.stack.len() != 1 {
                    let message = format!(
                        "Exception handler at {} should have exactly one stack item",
                        handler.handler_pc
                    );
                    return Err(VerifyError::VerifyError(message));
                }
                anchor_states[handler_index] = Some(expected);
            } else if anchor_states[handler_index].is_none() {
                anchor_states[handler_index] = Some(handler_frame);
            }

            worklist.push(handler_index);
        }

        Ok(())
    }

    /// Creates a diagnostic for a verification failure.
    #[must_use]
    pub fn create_diagnostic(&self, pc: u16, message: &str) -> VerificationDiagnostic {
        VerificationDiagnostic::new(
            &self.current_class,
            &self.method_name,
            &self.method_descriptor,
            pc,
            message,
        )
    }

    /// Returns the verification trace (for verbose mode).
    #[must_use]
    pub fn trace(&self) -> &VerificationTrace {
        &self.trace
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Version;
    use crate::attributes::{ArrayType, StackFrame, TableSwitch};
    use crate::constant::Constant;
    use crate::constant_pool::ConstantPool;
    use crate::verifiers::bytecode::handlers::test_utils::MockContext;

    type TestFastPathVerifier<'a> = FastPathVerifier<'a, MockContext>;

    fn create_mock_class_file() -> ClassFile<'static> {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add(Constant::utf8("TestClass")).unwrap();
        let this_class_index = constant_pool.add(Constant::Class(1)).unwrap();
        constant_pool.add(Constant::utf8("testMethod")).unwrap();
        constant_pool.add(Constant::utf8("()V")).unwrap();
        constant_pool.add(Constant::utf8("Code")).unwrap();

        ClassFile {
            version: Version::Java8 { minor: 0 },
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
        attributes: Vec<Attribute>,
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
                attributes,
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
            Vec::new(),
        )
    }

    fn fast_verifier<'a>(
        class_file: &'a ClassFile<'static>,
        method: &'a Method,
        context: &'a MockContext,
        config: &'a VerifierConfig,
    ) -> TestFastPathVerifier<'a> {
        TestFastPathVerifier::new(class_file, method, context, config).unwrap()
    }

    #[test]
    fn test_fast_path_simple_method() {
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
            TestFastPathVerifier::new(&class_file, &method, &context, &config).unwrap();
        let result = verifier.verify();

        assert!(matches!(result, FastPathResult::Success));
    }

    #[test]
    fn test_fast_path_arithmetic() {
        let class_file = create_mock_class_file();
        let code = vec![
            Instruction::Iconst_1,
            Instruction::Iconst_2,
            Instruction::Iadd,
            Instruction::Pop,
            Instruction::Return,
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
            TestFastPathVerifier::new(&class_file, &method, &context, &config).unwrap();
        let result = verifier.verify();

        assert!(matches!(result, FastPathResult::Success));
    }

    #[test]
    fn test_debug_trace_and_context_methods() {
        let class_file = create_mock_class_file();
        let method = static_method(vec![Instruction::Return], 0, 0);
        let config = VerifierConfig::default().with_trace(true);
        let context = MockContext::PERMISSIVE;
        let verifier = fast_verifier(&class_file, &method, &context, &config);

        let debug = format!("{verifier:?}");
        assert!(debug.contains("FastPathVerifier"));
        assert!(verifier.trace().is_enabled());
        assert!(context.is_subclass("A", "B").unwrap());
        assert!(context.is_assignable("A", "B").unwrap());
        assert_eq!(
            "java/lang/Object",
            context.common_superclass("A", "B").unwrap()
        );
        let diagnostic = verifier.create_diagnostic(7, "message");
        assert_eq!(7, diagnostic.pc);
    }

    #[test]
    fn test_fast_path_constructor_and_parameter_initial_frames() {
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
            Vec::new(),
        );
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;

        let mut constructor = method.clone();
        constructor.name_index = init_name;
        let frame = TestFastPathVerifier::create_initial_frame_static(
            &constructor,
            &class_file,
            "TestClass",
            "<init>",
            7,
            1,
        )
        .unwrap();
        assert_eq!(frame.locals[0], VerificationType::UninitializedThis);
        assert_eq!(frame.locals[1], VerificationType::Long);
        assert_eq!(frame.locals[2], VerificationType::Top);
        assert_eq!(frame.locals[3], VerificationType::Double);
        assert_eq!(frame.locals[4], VerificationType::Top);
        assert_eq!(
            frame.locals[5],
            VerificationType::Object(JavaString::from("TestClass"))
        );

        let small = TestFastPathVerifier::create_initial_frame_static(
            &method,
            &class_file,
            "TestClass",
            "testMethod",
            2,
            1,
        );
        assert!(small.is_err());

        let verifier = fast_verifier(&class_file, &constructor, &context, &config);
        assert_eq!(
            verifier.create_initial_frame().unwrap().locals[0],
            VerificationType::UninitializedThis
        );
    }

    #[test]
    fn test_fast_path_stackmap_and_control_flow_outcomes() {
        let class_file = create_mock_class_file();
        let context = MockContext::PERMISSIVE;
        let strict = VerifierConfig::default();
        let permissive = VerifierConfig::permissive();
        let code = vec![Instruction::Goto(2), Instruction::Nop, Instruction::Return];
        let method = static_method(code, 0, 0);

        let mut strict_verifier = fast_verifier(&class_file, &method, &context, &strict);
        assert!(matches!(
            strict_verifier.verify(),
            FastPathResult::Failed(_)
        ));

        let mut permissive_verifier = fast_verifier(&class_file, &method, &context, &permissive);
        assert!(matches!(
            permissive_verifier.verify(),
            FastPathResult::NeedsFallback(_)
        ));

        let stackmap_method = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Goto(1), Instruction::Return],
            0,
            0,
            vec![Attribute::StackMapTable {
                name_index: 5,
                frames: vec![StackFrame::SameFrame { frame_type: 3 }],
            }],
            Vec::new(),
        );
        let mut verifier = fast_verifier(&class_file, &stackmap_method, &context, &strict);
        assert!(matches!(verifier.verify(), FastPathResult::Success));

        let seeded_stackmap_method = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Nop, Instruction::Return],
            0,
            0,
            vec![Attribute::StackMapTable {
                name_index: 5,
                frames: vec![StackFrame::SameFrame { frame_type: 0 }],
            }],
            Vec::new(),
        );
        let mut missing_frame_verifier =
            fast_verifier(&class_file, &seeded_stackmap_method, &context, &permissive);
        missing_frame_verifier.jump_targets.insert(1);
        assert!(matches!(
            missing_frame_verifier.verify(),
            FastPathResult::NeedsFallback(reason) if reason.contains("Missing frame")
        ));

        assert!(TestFastPathVerifier::is_fallback_eligible(
            &VerifyError::VerifyError("merge point missing".to_string())
        ));
        assert!(TestFastPathVerifier::is_fallback_eligible(
            &VerifyError::VerifyError(
                "Merge point at offset 1 requires StackMapTable frame".to_string()
            )
        ));
        assert!(TestFastPathVerifier::is_fallback_eligible(
            &VerifyError::VerifyError("Missing frame".to_string())
        ));
        assert!(TestFastPathVerifier::is_fallback_eligible(
            &VerifyError::VerifyError("StackMapTable missing".to_string())
        ));
        assert!(!TestFastPathVerifier::is_fallback_eligible(
            &VerifyError::VerifyError("StackMapTable mismatch".to_string())
        ));
        assert!(!TestFastPathVerifier::is_fallback_eligible(
            &VerifyError::ClassFormatError("bad".to_string())
        ));
    }

    #[test]
    fn test_fast_path_frame_compatibility_and_successor_paths() {
        let class_file = create_mock_class_file();
        let method = static_method(vec![Instruction::Return], 0, 0);
        let config = VerifierConfig::permissive();
        let context = MockContext::PERMISSIVE;
        let verifier = fast_verifier(&class_file, &method, &context, &config);

        let mut computed = Frame::new(2, 2);
        computed.push(VerificationType::Integer).unwrap();
        let mut expected = Frame::new(2, 2);
        assert!(
            verifier
                .validate_frame_compatibility(&computed, &expected, 0)
                .is_err()
        );

        expected
            .push(VerificationType::Object(JavaString::from(
                "java/lang/Object",
            )))
            .unwrap();
        assert!(
            verifier
                .validate_frame_compatibility(&computed, &expected, 0)
                .is_err()
        );

        let mut local_computed = Frame::new(1, 1);
        local_computed.locals[0] = VerificationType::Object(JavaString::from("A"));
        let mut local_expected = Frame::new(1, 1);
        local_expected.locals[0] = VerificationType::Integer;
        assert!(
            verifier
                .validate_frame_compatibility(&local_computed, &local_expected, 0)
                .is_err()
        );

        local_expected.locals[0] = VerificationType::Top;
        assert!(
            verifier
                .validate_frame_compatibility(&local_computed, &local_expected, 0)
                .is_ok()
        );

        let mut anchors = vec![None, None];
        verifier
            .handle_successor(1, 1, &Frame::new(0, 0), &mut anchors)
            .unwrap();
        assert!(anchors[1].is_some());
    }

    #[test]
    fn test_fast_path_reference_instruction_dispatch_array_creation() {
        let ReferenceClassFile {
            class_file,
            class_index,
            array_class_index,
            ..
        } = create_reference_class_file();
        let method = static_method(vec![Instruction::Return], 8, 4);
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;
        let verifier = fast_verifier(&class_file, &method, &context, &config);

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
    fn test_fast_path_reference_instruction_dispatch_field_access() {
        let ReferenceClassFile {
            class_file,
            field_index,
            long_field_index,
            ..
        } = create_reference_class_file();
        let method = static_method(vec![Instruction::Return], 8, 4);
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;
        let verifier = fast_verifier(&class_file, &method, &context, &config);

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
    fn test_fast_path_reference_instruction_dispatch_invokes() {
        let ReferenceClassFile {
            class_file,
            init_index,
            virtual_index,
            static_index,
            interface_index,
            dynamic_index,
            ..
        } = create_reference_class_file();
        let method = static_method(vec![Instruction::Return], 8, 4);
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;
        let verifier = fast_verifier(&class_file, &method, &context, &config);

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
    fn test_fast_path_reference_instruction_dispatch_type_ops() {
        let ReferenceClassFile {
            class_file,
            class_index,
            ..
        } = create_reference_class_file();
        let method = static_method(vec![Instruction::Return], 8, 4);
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;
        let verifier = fast_verifier(&class_file, &method, &context, &config);

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
            .push(VerificationType::Object(JavaString::from("[I")))
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
    fn test_fast_path_exception_handlers_and_stackmap_decode_errors() {
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
            Vec::new(),
            vec![ExceptionTableEntry {
                range_pc: 0..1,
                handler_pc: 1,
                catch_type: catch_index,
            }],
        );
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;
        let verifier = fast_verifier(&class_file, &method, &context, &config);
        let mut anchors = vec![None, None];
        let mut worklist = Vec::new();
        let mut frame = Frame::new(1, 1);
        frame.locals[0] = VerificationType::Integer;

        verifier
            .process_exception_handlers(0, &frame, &mut anchors, &mut worklist)
            .unwrap();
        assert!(anchors[1].is_some());
        assert_eq!(worklist, vec![1]);

        let invalid = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Return],
            1,
            0,
            vec![Attribute::StackMapTable {
                name_index: 5,
                frames: vec![StackFrame::SameFrame { frame_type: 2 }],
            }],
            Vec::new(),
        );
        assert!(TestFastPathVerifier::new(&class_file, &invalid, &context, &config).is_err());

        let missing_code = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![],
        };
        assert!(TestFastPathVerifier::new(&class_file, &missing_code, &context, &config).is_err());
    }

    #[test]
    fn test_fast_path_class_format_error_edges() {
        let mut class_file = create_mock_class_file();
        let invalid_descriptor = class_file.constant_pool.add_utf8("invalid").unwrap();
        let context = MockContext::PERMISSIVE;
        let config = VerifierConfig::default();

        let bad_code = static_method(vec![Instruction::Goto(u16::MAX)], 0, 0);
        assert!(TestFastPathVerifier::new(&class_file, &bad_code, &context, &config).is_err());
        assert!(TestFastPathVerifier::build_code_info(&[Instruction::Goto(u16::MAX)]).is_err());

        let bad_descriptor_for_new = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            invalid_descriptor,
            vec![Instruction::Return],
            0,
            0,
            Vec::new(),
            Vec::new(),
        );
        assert!(
            TestFastPathVerifier::new(&class_file, &bad_descriptor_for_new, &context, &config)
                .is_err()
        );

        let method = static_method(vec![Instruction::Return], 0, 0);
        let mut bad_class_file = class_file.clone();
        bad_class_file.this_class = 999;
        assert!(TestFastPathVerifier::extract_method_info(&bad_class_file, &method).is_err());

        let mut bad_name = method.clone();
        bad_name.name_index = 999;
        assert!(TestFastPathVerifier::extract_method_info(&class_file, &bad_name).is_err());

        let mut bad_descriptor_index = method.clone();
        bad_descriptor_index.descriptor_index = 999;
        assert!(
            TestFastPathVerifier::extract_method_info(&class_file, &bad_descriptor_index).is_err()
        );
        assert!(
            TestFastPathVerifier::create_initial_frame_static(
                &bad_descriptor_index,
                &class_file,
                "TestClass",
                "testMethod",
                0,
                0,
            )
            .is_err()
        );

        let bad_descriptor_text = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            invalid_descriptor,
            vec![Instruction::Return],
            0,
            0,
            Vec::new(),
            Vec::new(),
        );
        assert!(
            TestFastPathVerifier::create_initial_frame_static(
                &bad_descriptor_text,
                &class_file,
                "TestClass",
                "testMethod",
                0,
                0,
            )
            .is_err()
        );

        let invalid_catch = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Nop],
            1,
            1,
            Vec::new(),
            vec![ExceptionTableEntry {
                range_pc: 0..1,
                handler_pc: 0,
                catch_type: 999,
            }],
        );
        let verifier = fast_verifier(&class_file, &invalid_catch, &context, &config);
        let mut anchors = vec![None];
        let mut worklist = Vec::new();
        assert!(
            verifier
                .process_exception_handlers(0, &Frame::new(1, 1), &mut anchors, &mut worklist)
                .is_err()
        );
    }

    #[test]
    fn test_fast_path_code_attribute_and_control_flow_edges() {
        let class_file = create_mock_class_file();

        let non_code_method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![Attribute::Deprecated { name_index: 5 }],
        };
        assert!(TestFastPathVerifier::extract_code_attribute(&non_code_method).is_err());

        let initial_frame = Frame::new(0, 0);
        assert!(
            TestFastPathVerifier::decode_stack_map_table(
                &[Attribute::Deprecated { name_index: 5 }],
                &initial_frame,
                &class_file,
                0,
            )
            .unwrap()
            .is_empty()
        );

        let code_info = CodeInfo::new(vec![0], 1);
        let (targets, handlers) = TestFastPathVerifier::collect_control_flow_targets(
            &[Instruction::Goto(99)],
            &[],
            &code_info,
        );
        assert!(targets.is_empty());
        assert!(handlers.is_empty());
    }

    #[test]
    fn test_fast_path_initial_frame_edge_cases() {
        let mut class_file = create_mock_class_file();
        let context = MockContext::PERMISSIVE;
        let strict = VerifierConfig::default();

        let instance_method = method_with(
            MethodAccessFlags::PUBLIC,
            4,
            vec![Instruction::Return],
            0,
            1,
            Vec::new(),
            Vec::new(),
        );
        let frame = TestFastPathVerifier::create_initial_frame_static(
            &instance_method,
            &class_file,
            "TestClass",
            "testMethod",
            1,
            0,
        )
        .unwrap();
        assert_eq!(
            frame.locals[0],
            VerificationType::Object(JavaString::from("TestClass"))
        );

        let wide_descriptor = class_file.constant_pool.add_utf8("(JD)V").unwrap();
        let wide_method = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            wide_descriptor,
            vec![Instruction::Return],
            0,
            3,
            Vec::new(),
            Vec::new(),
        );
        assert!(
            TestFastPathVerifier::create_initial_frame_static(
                &wide_method,
                &class_file,
                "TestClass",
                "testMethod",
                3,
                0,
            )
            .is_err()
        );

        let int_descriptor = class_file.constant_pool.add_utf8("(I)V").unwrap();
        let too_few_locals = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            int_descriptor,
            vec![Instruction::Return],
            0,
            0,
            Vec::new(),
            Vec::new(),
        );
        assert!(
            TestFastPathVerifier::new(&class_file, &too_few_locals, &context, &strict).is_err()
        );
    }

    #[test]
    fn test_fast_path_verify_termination_and_stackmap_targets() {
        let class_file = create_mock_class_file();
        let context = MockContext::PERMISSIVE;
        let strict = VerifierConfig::default();

        let fall_off_method = static_method(vec![Instruction::Nop], 0, 0);
        let mut fall_off_verifier = fast_verifier(&class_file, &fall_off_method, &context, &strict);
        assert!(matches!(
            fall_off_verifier.verify(),
            FastPathResult::Failed(_)
        ));

        let anchor_success = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Nop, Instruction::Return],
            0,
            0,
            vec![Attribute::StackMapTable {
                name_index: 5,
                frames: vec![StackFrame::SameFrame { frame_type: 1 }],
            }],
            Vec::new(),
        );
        let mut anchor_success_verifier =
            fast_verifier(&class_file, &anchor_success, &context, &strict);
        assert!(matches!(
            anchor_success_verifier.verify(),
            FastPathResult::Success
        ));

        let loop_target_method = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Goto(0)],
            0,
            0,
            vec![Attribute::StackMapTable {
                name_index: 5,
                frames: vec![StackFrame::SameFrame { frame_type: 0 }],
            }],
            Vec::new(),
        );
        let mut loop_target_verifier =
            fast_verifier(&class_file, &loop_target_method, &context, &strict);
        assert!(matches!(
            loop_target_verifier.verify(),
            FastPathResult::Success
        ));

        let relative_return = 1;
        let duplicate_target_method = static_method(
            vec![
                Instruction::Iconst_0,
                Instruction::Tableswitch(Box::new(TableSwitch {
                    default: relative_return,
                    low: 0,
                    high: 0,
                    offsets: vec![relative_return],
                })),
                Instruction::Return,
            ],
            1,
            0,
        );
        let mut duplicate_target_verifier =
            fast_verifier(&class_file, &duplicate_target_method, &context, &strict);
        assert!(matches!(
            duplicate_target_verifier.verify(),
            FastPathResult::Success
        ));
    }

    #[test]
    fn test_fast_path_verify_failure_and_trace_edges() {
        let class_file = create_mock_class_file();
        let context = MockContext::PERMISSIVE;
        let strict = VerifierConfig::default();

        let invalid_handler_method = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Nop, Instruction::Return],
            0,
            0,
            Vec::new(),
            vec![ExceptionTableEntry {
                range_pc: 1..1,
                handler_pc: 0,
                catch_type: 0,
            }],
        );
        let mut invalid_handler_verifier =
            fast_verifier(&class_file, &invalid_handler_method, &context, &strict);
        assert!(matches!(
            invalid_handler_verifier.verify(),
            FastPathResult::Failed(_)
        ));

        let trace_config = VerifierConfig::default().with_trace(true);
        let trace_method = static_method(vec![Instruction::Nop, Instruction::Return], 0, 0);
        let mut trace_verifier = fast_verifier(&class_file, &trace_method, &context, &trace_config);
        assert!(matches!(trace_verifier.verify(), FastPathResult::Success));

        let permissive = VerifierConfig::permissive();
        let fallback_mismatch = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Iconst_0, Instruction::Return],
            1,
            0,
            vec![Attribute::StackMapTable {
                name_index: 5,
                frames: vec![StackFrame::SameFrame { frame_type: 1 }],
            }],
            Vec::new(),
        );
        let mut fallback_verifier =
            fast_verifier(&class_file, &fallback_mismatch, &context, &permissive);
        assert!(matches!(
            fallback_verifier.verify(),
            FastPathResult::Failed(_)
        ));

        let unsupported_method = static_method(vec![Instruction::Impdep1], 0, 0);
        let mut unsupported_verifier =
            fast_verifier(&class_file, &unsupported_method, &context, &strict);
        assert!(matches!(
            unsupported_verifier.verify(),
            FastPathResult::Failed(_)
        ));
    }

    #[test]
    fn test_fast_path_execute_instruction_edges() {
        let class_file = create_mock_class_file();
        let context = MockContext::PERMISSIVE;
        let strict = VerifierConfig::default();
        let trace_method = static_method(vec![Instruction::Nop, Instruction::Return], 0, 0);

        let verifier = fast_verifier(&class_file, &trace_method, &context, &strict);
        let mut load = Frame::new(1, 1);
        load.locals[0] = VerificationType::Integer;
        assert!(
            verifier
                .dispatch_standard_instruction(&Instruction::Iload_0, &mut load)
                .unwrap()
        );
        assert!(
            verifier
                .execute_instruction(0, 0, &Instruction::Ret(0), Frame::new(1, 1))
                .is_err()
        );
        assert!(
            verifier
                .execute_instruction(0, 0, &Instruction::Return, Frame::new(0, 0))
                .is_ok()
        );
        let mut conversion = Frame::new(0, 2);
        conversion.push(VerificationType::Integer).unwrap();
        assert!(
            verifier
                .execute_instruction(0, 0, &Instruction::I2l, conversion)
                .is_ok()
        );
        let mut comparison = Frame::new(0, 4);
        comparison.push_category2(VerificationType::Long).unwrap();
        comparison.push_category2(VerificationType::Long).unwrap();
        assert!(
            verifier
                .execute_instruction(0, 0, &Instruction::Lcmp, comparison)
                .is_ok()
        );

        let throw_method = static_method(vec![Instruction::Athrow], 1, 0);
        let throw_verifier = fast_verifier(&class_file, &throw_method, &context, &strict);
        let mut throw_frame = Frame::new(0, 1);
        throw_frame
            .push(VerificationType::java_lang_throwable())
            .unwrap();
        assert!(
            throw_verifier
                .execute_instruction(0, 0, &Instruction::Athrow, throw_frame)
                .is_ok()
        );

        let ReferenceClassFile {
            class_file: reference_class_file,
            class_index,
            ..
        } = create_reference_class_file();
        let reference_method = static_method(
            vec![Instruction::New(class_index), Instruction::Return],
            1,
            0,
        );
        let reference_verifier =
            fast_verifier(&reference_class_file, &reference_method, &context, &strict);
        assert!(
            reference_verifier
                .execute_instruction(0, 0, &Instruction::New(class_index), Frame::new(0, 1))
                .is_ok()
        );
    }

    #[test]
    fn test_fast_path_successor_and_frame_compatibility_edges() {
        let class_file = create_mock_class_file();
        let context = MockContext::PERMISSIVE;
        let strict = VerifierConfig::default();
        let permissive = VerifierConfig::permissive();
        let trace_method = static_method(vec![Instruction::Nop, Instruction::Return], 0, 0);

        let mut strict_merge = fast_verifier(&class_file, &trace_method, &context, &strict);
        strict_merge.jump_targets.insert(1);
        let mut anchors = vec![None, None];
        assert!(
            strict_merge
                .handle_successor(1, 1, &Frame::new(0, 0), &mut anchors)
                .is_err()
        );

        let mut merge_verifier = fast_verifier(&class_file, &trace_method, &context, &permissive);
        merge_verifier.jump_targets.insert(1);
        let mut existing = Frame::new(1, 0);
        existing.locals[0] = VerificationType::Object(JavaString::from("A"));
        let mut next = Frame::new(1, 0);
        next.locals[0] = VerificationType::Object(JavaString::from("B"));
        let mut anchors = vec![None, Some(existing)];
        merge_verifier
            .handle_successor(1, 1, &next, &mut anchors)
            .unwrap();
        let mut anchors = vec![None, None];
        merge_verifier
            .handle_successor(1, 1, &next, &mut anchors)
            .unwrap();
        assert!(anchors[1].is_some());

        let mut compatible = Frame::new(1, 1);
        compatible.locals[0] = VerificationType::Integer;
        compatible.stack.push(VerificationType::Integer);
        let mut expected_compatible = Frame::new(1, 1);
        expected_compatible.locals[0] = VerificationType::Integer;
        expected_compatible.stack.push(VerificationType::Integer);
        merge_verifier
            .validate_frame_compatibility(&compatible, &expected_compatible, 1)
            .unwrap();

        let mut computed = Frame::new(1, 1);
        computed.stack.push(VerificationType::Integer);
        let mut expected = Frame::new(1, 1);
        expected.stack.push(VerificationType::Float);
        assert!(
            merge_verifier
                .validate_frame_compatibility(&computed, &expected, 1)
                .is_err()
        );

        let mut computed = Frame::new(1, 0);
        computed.locals[0] = VerificationType::Integer;
        let mut expected = Frame::new(1, 0);
        expected.locals[0] = VerificationType::Float;
        assert!(
            merge_verifier
                .validate_frame_compatibility(&computed, &expected, 1)
                .is_err()
        );
    }

    #[test]
    fn test_fast_path_stackmap_anchor_and_invalid_handler_edges() {
        let class_file = create_mock_class_file();
        let context = MockContext::PERMISSIVE;
        let strict = VerifierConfig::default();

        let no_decoded_anchor_method =
            static_method(vec![Instruction::Nop, Instruction::Return], 0, 0);
        let mut no_decoded_anchor_verifier =
            fast_verifier(&class_file, &no_decoded_anchor_method, &context, &strict);
        no_decoded_anchor_verifier.jump_targets.insert(1);
        let initial_frame = no_decoded_anchor_verifier.initial_frame.clone();
        assert!(
            no_decoded_anchor_verifier
                .verify_with_stackmaps(&initial_frame)
                .is_err()
        );

        let invalid_pc_method = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Nop],
            1,
            1,
            Vec::new(),
            vec![ExceptionTableEntry {
                range_pc: 0..1,
                handler_pc: 99,
                catch_type: 0,
            }],
        );
        let invalid_pc_verifier = fast_verifier(&class_file, &invalid_pc_method, &context, &strict);
        let mut anchors = vec![None];
        assert!(
            invalid_pc_verifier
                .process_exception_handlers(0, &Frame::new(1, 1), &mut anchors, &mut Vec::new())
                .is_err()
        );
        let mut invalid_pc_verify =
            fast_verifier(&class_file, &invalid_pc_method, &context, &strict);
        assert!(matches!(
            invalid_pc_verify.verify(),
            FastPathResult::Failed(_)
        ));
    }

    #[test]
    fn test_fast_path_exception_handler_catch_all_edges() {
        let class_file = create_mock_class_file();
        let context = MockContext::PERMISSIVE;
        let strict = VerifierConfig::default();

        let catch_all_method = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Nop],
            1,
            1,
            Vec::new(),
            vec![ExceptionTableEntry {
                range_pc: 0..1,
                handler_pc: 0,
                catch_type: 0,
            }],
        );
        let catch_all_verifier = fast_verifier(&class_file, &catch_all_method, &context, &strict);
        let mut anchors = vec![None];
        let mut worklist = Vec::new();
        catch_all_verifier
            .process_exception_handlers(0, &Frame::new(1, 1), &mut anchors, &mut worklist)
            .unwrap();
        assert!(anchors[0].is_some());
        assert_eq!(worklist, vec![0]);

        let no_match_method = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Nop, Instruction::Return],
            1,
            1,
            Vec::new(),
            vec![ExceptionTableEntry {
                range_pc: 1..2,
                handler_pc: 0,
                catch_type: 0,
            }],
        );
        let no_match_verifier = fast_verifier(&class_file, &no_match_method, &context, &strict);
        let mut anchors = vec![None, None];
        let mut worklist = Vec::new();
        no_match_verifier
            .process_exception_handlers(0, &Frame::new(1, 1), &mut anchors, &mut worklist)
            .unwrap();
        assert!(worklist.is_empty());
    }

    #[test]
    fn test_fast_path_exception_handler_stackmap_edges() {
        let class_file = create_mock_class_file();
        let context = MockContext::PERMISSIVE;
        let strict = VerifierConfig::default();

        let bad_handler_stack = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Nop],
            1,
            1,
            vec![Attribute::StackMapTable {
                name_index: 5,
                frames: vec![StackFrame::SameFrame { frame_type: 0 }],
            }],
            vec![ExceptionTableEntry {
                range_pc: 0..1,
                handler_pc: 0,
                catch_type: 0,
            }],
        );
        let bad_handler_verifier =
            fast_verifier(&class_file, &bad_handler_stack, &context, &strict);
        let mut bad_handler_anchors = [None];
        assert!(
            bad_handler_verifier
                .process_exception_handlers(
                    0,
                    &Frame::new(1, 1),
                    &mut bad_handler_anchors,
                    &mut Vec::new()
                )
                .is_err()
        );

        let good_handler_stack = method_with(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            4,
            vec![Instruction::Nop],
            1,
            1,
            vec![Attribute::StackMapTable {
                name_index: 5,
                frames: vec![StackFrame::FullFrame {
                    frame_type: 255,
                    offset_delta: 0,
                    locals: Vec::new(),
                    stack: vec![crate::attributes::VerificationType::Object {
                        cpool_index: class_file.this_class,
                    }],
                }],
            }],
            vec![ExceptionTableEntry {
                range_pc: 0..1,
                handler_pc: 0,
                catch_type: 0,
            }],
        );
        let good_handler_verifier =
            fast_verifier(&class_file, &good_handler_stack, &context, &strict);
        let mut anchors = vec![None];
        let mut worklist = Vec::new();
        good_handler_verifier
            .process_exception_handlers(0, &Frame::new(1, 1), &mut anchors, &mut worklist)
            .unwrap();
        assert!(anchors[0].is_some());
        assert_eq!(worklist, vec![0]);
    }
}
