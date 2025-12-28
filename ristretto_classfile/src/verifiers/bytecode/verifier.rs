use std::io::Cursor;
use std::sync::Arc;

use crate::FieldType;
use crate::attributes::{Attribute, Instruction, StackFrame};
use crate::class_file::ClassFile;
use crate::method::Method;
use crate::method_access_flags::MethodAccessFlags;
use crate::verifiers::context::VerificationContext;
use crate::verifiers::error::{Result, VerifyError};

use super::control_flow::{CodeInfo, Worklist, compute_successors, validate_exception_table};
use super::frame::Frame;
use super::handlers;
use super::handlers::references::ConstantPoolResolver;
use super::type_system::VerificationType;

/// Verifies a method's bytecode according to [JVMS §4.10.1](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1).
///
/// This is the main entry point for bytecode verification.
///
/// # Arguments
///
/// * `class_file` - The class file containing the method
/// * `method` - The method to verify
/// * `context` - The verification context for type hierarchy checks
///
/// # Errors
///
/// Returns a `VerifyError` if the bytecode is invalid.
///
/// # JVMS Reference
///
/// Implements verification by type checking as specified in [JVMS §4.10.1](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1).
pub fn verify<C: VerificationContext>(
    class_file: &ClassFile,
    method: &Method,
    context: &C,
) -> Result<()> {
    // Native and abstract methods should not have Code attribute
    if method
        .access_flags
        .intersects(MethodAccessFlags::NATIVE | MethodAccessFlags::ABSTRACT)
    {
        for attribute in &method.attributes {
            if matches!(attribute, Attribute::Code { .. }) {
                let name = class_file
                    .constant_pool
                    .try_get_utf8(method.name_index)
                    .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;
                return Err(VerifyError::ClassFormatError(format!(
                    "Method {name} is native or abstract but has Code attribute"
                )));
            }
        }
        return Ok(());
    }

    let verifier = BytecodeVerifier::new(class_file, method, context)?;
    verifier.verify()
}

/// Bytecode verifier for a single method.
///
/// This struct holds the state needed for verifying a method's bytecode.
struct BytecodeVerifier<'a, C: VerificationContext> {
    /// The class file containing the method.
    class_file: &'a ClassFile,
    /// The method being verified.
    method: &'a Method,
    /// The verification context for type hierarchy checks.
    context: &'a C,
    /// The bytecode instructions.
    code: &'a Vec<Instruction>,
    /// Attributes of the Code attribute.
    code_attributes: &'a Vec<Attribute>,
    /// Exception table entries.
    exception_table: &'a Vec<crate::attributes::ExceptionTableEntry>,
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
}

impl<'a, C: VerificationContext> BytecodeVerifier<'a, C> {
    /// Creates a new `BytecodeVerifier` for the given method.
    ///
    /// # Errors
    ///
    /// Returns `VerifyError` if the method's code cannot be processed.
    fn new(class_file: &'a ClassFile, method: &'a Method, context: &'a C) -> Result<Self> {
        // Extract Code attribute
        let (code, max_stack, max_locals, code_attributes, exception_table) = method
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

        Ok(Self {
            class_file,
            method,
            context,
            code,
            code_attributes,
            exception_table,
            max_stack,
            max_locals,
            code_info,
            return_type,
            major_version,
            current_class,
            method_name,
        })
    }

    /// Verifies the method's bytecode using dataflow analysis.
    ///
    /// # Errors
    ///
    /// Returns `VerifyError` if the bytecode is invalid.
    fn verify(&self) -> Result<()> {
        // 1. Validate exception table
        validate_exception_table(self.exception_table, &self.code_info)?;

        // 2. Create initial frame
        let initial_frame = self.create_initial_frame()?;

        // 3. Initialize frames array (dense vector for performance)
        let mut frames: Vec<Option<Frame>> = vec![None; self.code_info.instruction_count()];
        frames[0] = Some(initial_frame.clone());

        // 4. Process StackMapTable to pre-populate frames
        self.process_stack_map_table(&mut frames, &initial_frame)?;

        // 5. Worklist-based dataflow analysis
        let mut worklist = Worklist::new(self.code_info.instruction_count());
        worklist.add(0);

        // Add all StackMapTable targets to worklist
        for (i, frame) in frames.iter().enumerate() {
            if frame.is_some() {
                worklist.add(i);
            }
        }

        while let Some(index) = worklist.pop() {
            let offset = self.code_info.offset_at(index).ok_or_else(|| {
                VerifyError::VerifyError(format!("Invalid instruction index {index}"))
            })?;

            let frame = frames[index]
                .clone()
                .ok_or_else(|| VerifyError::VerifyError(format!("No frame at offset {offset}")))?;

            let instruction = &self.code[index];

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
                    if existing_frame.merge(&next_frame, self.context)? {
                        worklist.add(successor_index);
                    }
                } else {
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

    /// Processes the `StackMapTable` to pre-populate frames.
    fn process_stack_map_table(
        &self,
        frames: &mut [Option<Frame>],
        initial_frame: &Frame,
    ) -> Result<()> {
        // Find StackMapTable attribute
        let stack_frames = self.code_attributes.iter().find_map(|attr| {
            if let Attribute::StackMapTable { frames, .. } = attr {
                Some(frames)
            } else {
                None
            }
        });

        let Some(stack_frames) = stack_frames else {
            return Ok(());
        };

        let mut current_frame = initial_frame.clone();
        let mut prev_offset: Option<u16> = None;

        for stack_frame in stack_frames {
            let offset_delta = get_offset_delta(stack_frame);

            // Calculate actual offset
            let offset = if let Some(prev) = prev_offset {
                prev + offset_delta + 1
            } else {
                offset_delta
            };

            // Validate offset
            let index = self.code_info.index_at(offset).ok_or_else(|| {
                VerifyError::VerifyError(format!(
                    "StackMapTable frame offset {offset} is not a valid instruction boundary"
                ))
            })?;

            // Apply stack frame to get next frame
            let next_frame = self.apply_stack_frame(&current_frame, stack_frame)?;
            frames[index] = Some(next_frame.clone());

            current_frame = next_frame;
            prev_offset = Some(offset);
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

    /// Handles reference-related instructions (field/method access, object creation).
    #[expect(clippy::too_many_lines)]
    #[expect(clippy::match_same_arms)]
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
            Instruction::Invokevirtual(index) => {
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
            Instruction::Invokeinterface(index, _) => {
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
        _frame: &Frame,
        frames: &mut [Option<Frame>],
        worklist: &mut Worklist,
    ) -> Result<()> {
        for handler in self.exception_table {
            if handler.range_pc.contains(&offset) {
                // Create handler frame with only exception on stack
                let mut handler_frame =
                    Frame::new(self.max_locals as usize, self.max_stack as usize);

                // Copy locals from current frame
                // (In full implementation, would copy from the frame at this point)

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

    /// Applies a `StackFrame` to produce a new Frame.
    fn apply_stack_frame(&self, current: &Frame, stack_frame: &StackFrame) -> Result<Frame> {
        let mut next = current.clone();
        next.clear_stack();

        match stack_frame {
            StackFrame::SameFrame { .. } | StackFrame::SameFrameExtended { .. } => {
                // Same locals, empty stack
            }
            StackFrame::SameLocals1StackItemFrame { stack, .. }
            | StackFrame::SameLocals1StackItemFrameExtended { stack, .. } => {
                for s in stack {
                    let v_type = self.convert_verification_type(s);
                    next.push(v_type)?;
                }
            }
            StackFrame::ChopFrame { frame_type, .. } => {
                let k = (251 - frame_type) as usize;
                let len = next.locals.len();
                if len < k {
                    return Err(VerifyError::VerifyError(
                        "ChopFrame: locals underflow".to_string(),
                    ));
                }
                next.locals.truncate(len - k);
            }
            StackFrame::AppendFrame { locals, .. } => {
                for local in locals {
                    let v_type = self.convert_verification_type(local);
                    next.locals.push(v_type);
                }
            }
            StackFrame::FullFrame { locals, stack, .. } => {
                next.locals.clear();
                for local in locals {
                    let v_type = self.convert_verification_type(local);
                    next.locals.push(v_type);
                }
                for s in stack {
                    let v_type = self.convert_verification_type(s);
                    next.push(v_type)?;
                }
            }
        }

        Ok(next)
    }

    /// Converts a classfile `VerificationType` to our internal type.
    fn convert_verification_type(
        &self,
        v_type: &crate::attributes::VerificationType,
    ) -> VerificationType {
        match v_type {
            crate::attributes::VerificationType::Top => VerificationType::Top,
            crate::attributes::VerificationType::Integer => VerificationType::Integer,
            crate::attributes::VerificationType::Float => VerificationType::Float,
            crate::attributes::VerificationType::Long => VerificationType::Long,
            crate::attributes::VerificationType::Double => VerificationType::Double,
            crate::attributes::VerificationType::Null => VerificationType::Null,
            crate::attributes::VerificationType::UninitializedThis => {
                VerificationType::UninitializedThis
            }
            crate::attributes::VerificationType::Object { cpool_index } => {
                if let Ok(name) = self.class_file.constant_pool.try_get_class(*cpool_index) {
                    VerificationType::Object(Arc::from(name))
                } else {
                    VerificationType::Top
                }
            }
            crate::attributes::VerificationType::Uninitialized { offset } => {
                VerificationType::Uninitialized(*offset)
            }
        }
    }
}

/// Extracts the offset delta from a stack frame.
fn get_offset_delta(frame: &StackFrame) -> u16 {
    match frame {
        StackFrame::SameFrame { frame_type } => u16::from(*frame_type),
        StackFrame::SameLocals1StackItemFrame { frame_type, .. } => u16::from(*frame_type) - 64,
        StackFrame::SameLocals1StackItemFrameExtended { offset_delta, .. }
        | StackFrame::ChopFrame { offset_delta, .. }
        | StackFrame::SameFrameExtended { offset_delta, .. }
        | StackFrame::AppendFrame { offset_delta, .. }
        | StackFrame::FullFrame { offset_delta, .. } => *offset_delta,
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
    fn test_verify_simple_method() {
        let class_file = create_mock_class_file();
        let code = vec![Instruction::Iconst_0, Instruction::Return];
        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 1,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        let result = verify(&class_file, &method, &MockContext);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_native_method() {
        let class_file = create_mock_class_file();
        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::NATIVE,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![],
        };

        let result = verify(&class_file, &method, &MockContext);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_native_method_with_code_fails() {
        let class_file = create_mock_class_file();
        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 1,
            max_locals: 1,
            code: vec![Instruction::Return],
            exception_table: vec![],
            attributes: vec![],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::NATIVE,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        let result = verify(&class_file, &method, &MockContext);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_method_without_code_fails() {
        let class_file = create_mock_class_file();
        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![],
        };

        let result = verify(&class_file, &method, &MockContext);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_arithmetic() {
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
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        let result = verify(&class_file, &method, &MockContext);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_local_variables() {
        let class_file = create_mock_class_file();
        let code = vec![
            Instruction::Iconst_5,
            Instruction::Istore_0,
            Instruction::Iload_0,
            Instruction::Pop,
            Instruction::Return,
        ];
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

        let result = verify(&class_file, &method, &MockContext);
        assert!(result.is_ok());
    }
}
