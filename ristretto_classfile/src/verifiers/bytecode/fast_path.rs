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

use ahash::AHashSet;
use std::io::Cursor;
use std::sync::Arc;

use crate::FieldType;
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
    /// The method descriptor.
    method_descriptor: String,
    /// Decoded `StackMapTable`.
    stack_map_table: DecodedStackMapTable,
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
        class_file: &'a ClassFile,
        method: &'a Method,
        context: &'a C,
        config: &'a VerifierConfig,
    ) -> Result<Self> {
        let (code, max_stack, max_locals, code_attributes, exception_table) =
            Self::extract_code_attribute(method)?;

        let code_info = Self::build_code_info(code)?;
        let (current_class, method_name, method_descriptor) =
            Self::extract_method_info(class_file, method)?;
        let (_, return_type) = FieldType::parse_method_descriptor(&method_descriptor)
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
        class_file: &ClassFile,
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
        class_file: &ClassFile,
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
        class_file: &ClassFile,
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
                frame.set_local(
                    local_index,
                    VerificationType::Object(Arc::from(current_class)),
                )?;
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

        // Create initial frame
        let initial_frame = match self.create_initial_frame() {
            Ok(f) => f,
            Err(e) => return FastPathResult::Failed(e),
        };

        // Perform the actual verification
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
        matches!(error,
            VerifyError::VerifyError(msg) if msg.contains("merge point") || msg.contains("StackMapTable")
        )
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
            let start_frame = anchor_states[start_index].clone().ok_or_else(|| {
                VerifyError::VerifyError(format!(
                    "No frame at anchor index {} (offset {:?})",
                    start_index,
                    self.code_info.offset_at(start_index)
                ))
            })?;

            // Process straight-line code from this anchor
            let mut current_frame = start_frame;
            let mut index = start_index;

            loop {
                if index >= self.code.len() {
                    break;
                }

                let offset = self.code_info.offset_at(index).ok_or_else(|| {
                    VerifyError::VerifyError(format!("Invalid instruction index {index}"))
                })?;

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
                self.process_exception_handlers(
                    offset,
                    &next_frame,
                    &mut anchor_states,
                    &mut worklist,
                )?;

                // Handle successors
                for succ_offset in &successors {
                    if *succ_offset == next_offset && falls_through {
                        continue;
                    }
                    // This is a branch target
                    let Some(succ_index) = self.code_info.index_at(*succ_offset) else {
                        continue;
                    };
                    if visited[succ_index] {
                        continue;
                    }

                    self.handle_successor(
                        succ_index,
                        *succ_offset,
                        &next_frame,
                        &mut anchor_states,
                    )?;
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
            if !comp_type.is_assignable_to(exp_type, self.context)? {
                return Err(VerifyError::VerifyError(format!(
                    "Stack type mismatch at offset {offset}, slot {i}: {comp_type} not assignable to {exp_type}"
                )));
            }
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

            if !comp_type.is_assignable_to(exp_type, self.context)? {
                return Err(VerifyError::VerifyError(format!(
                    "Local type mismatch at offset {offset}, local {i}: {comp_type} not assignable to {exp_type}"
                )));
            }
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
        anchor_states: &mut [Option<Frame>],
        worklist: &mut Vec<usize>,
    ) -> Result<()> {
        for handler in self.exception_table {
            if handler.range_pc.contains(&offset) {
                let handler_index =
                    self.code_info.index_at(handler.handler_pc).ok_or_else(|| {
                        VerifyError::VerifyError(format!(
                            "Invalid handler PC {}",
                            handler.handler_pc
                        ))
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
                    VerificationType::Object(Arc::from(class_name))
                };

                // Create handler frame: same locals, stack = [exception_type]
                let mut handler_frame =
                    Frame::new(self.max_locals as usize, self.max_stack as usize);

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
                    // Note: We're lenient here - just check stack has exception type
                    if expected.stack.len() != 1 {
                        return Err(VerifyError::VerifyError(format!(
                            "Exception handler at {} should have exactly one stack item",
                            handler.handler_pc
                        )));
                    }
                    anchor_states[handler_index] = Some(expected);
                } else if anchor_states[handler_index].is_none() {
                    anchor_states[handler_index] = Some(handler_frame);
                }

                worklist.push(handler_index);
            }
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
            version: Version::Java8 { minor: 0 },
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
        let context = MockContext;

        let mut verifier = FastPathVerifier::new(&class_file, &method, &context, &config).unwrap();
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
        let context = MockContext;

        let mut verifier = FastPathVerifier::new(&class_file, &method, &context, &config).unwrap();
        let result = verifier.verify();

        match &result {
            FastPathResult::Success => {}
            FastPathResult::Failed(error) => panic!("Fast path failed: {error}"),
            FastPathResult::NeedsFallback(reason) => panic!("Fast path needs fallback: {reason}"),
        }
    }
}
