use crate::Method;
use crate::attributes::{Attribute, ExceptionTableEntry, Instruction, StackFrame};
use crate::class_file::ClassFile;
use crate::field_type::FieldType;
use crate::method_access_flags::MethodAccessFlags;
use crate::verifiers::context::VerificationContext;
use crate::verifiers::error::{Result, VerifyError};
use crate::verifiers::frame::Frame;
use crate::verifiers::types::VerificationType;
use std::collections::{HashMap, VecDeque};
use std::io::Cursor;

/// Verifies a method's bytecode.
///
/// # Errors
/// Returns `VerifyError` if the method's bytecode is invalid.
pub fn verify<C: VerificationContext>(
    class_file: &ClassFile,
    method: &Method,
    context: &C,
) -> Result<()> {
    if method
        .access_flags
        .intersects(MethodAccessFlags::NATIVE | MethodAccessFlags::ABSTRACT)
    {
        // Check if Code attribute exists
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
struct BytecodeVerifier<'a, C: VerificationContext> {
    class_file: &'a ClassFile,
    method: &'a Method,
    context: &'a C,
    code: &'a Vec<Instruction>,
    code_attributes: &'a Vec<Attribute>,
    exception_table: &'a Vec<ExceptionTableEntry>,
    max_stack: u16,
    max_locals: u16,
    instruction_offsets: Vec<u16>,
    offset_to_index: HashMap<u16, usize>,
    code_length: u16,
}

impl<'a, C: VerificationContext> BytecodeVerifier<'a, C> {
    /// Creates a new `BytecodeVerifier` for the given method.
    ///
    /// # Errors
    ///
    /// Returns `VerifyError` if the method's code cannot be processed.
    fn new(class_file: &'a ClassFile, method: &'a Method, context: &'a C) -> Result<Self> {
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

        let mut instruction_offsets = Vec::with_capacity(code.len());
        let mut offset_to_index = HashMap::with_capacity(code.len());

        let mut cursor = Cursor::new(Vec::new());
        for (index, instruction) in code.iter().enumerate() {
            let offset = u16::try_from(cursor.position())?;
            instruction_offsets.push(offset);
            offset_to_index.insert(offset, index);

            // We use to_bytes to determine length.
            // Ideally ristretto_classfile would expose length directly.
            instruction
                .to_bytes(&mut cursor)
                .map_err(|error| VerifyError::ClassFormatError(error.to_string()))?;
        }
        let code_length = u16::try_from(cursor.position())?;

        Ok(Self {
            class_file,
            method,
            context,
            code,
            code_attributes,
            exception_table,
            max_stack,
            max_locals,
            instruction_offsets,
            offset_to_index,
            code_length,
        })
    }

    /// Verifies the method's bytecode using dataflow analysis.
    ///
    /// # Errors
    ///
    /// Returns `VerifyError` if the bytecode is invalid.
    fn verify(&self) -> Result<()> {
        // 1. Initialize Frame
        let initial_frame = self.create_initial_frame()?;

        // 2. Dataflow Analysis
        // Map offset -> Frame
        let mut frames: HashMap<u16, Frame> = HashMap::new();
        let mut worklist: VecDeque<u16> = VecDeque::new();

        frames.insert(0, initial_frame.clone());
        worklist.push_back(0);

        // Handle StackMapTable to pre-populate frames at jump targets/offsets
        if let Some(Attribute::StackMapTable {
            frames: stack_frames,
            ..
        }) = self
            .code_attributes
            .iter()
            .find(|a| matches!(a, Attribute::StackMapTable { .. }))
        {
            let mut current_frame = initial_frame;
            for stack_frame in stack_frames {
                let offset_delta = match stack_frame {
                    StackFrame::SameFrame { frame_type } => u16::from(*frame_type),
                    StackFrame::SameLocals1StackItemFrame { frame_type, .. } => {
                        u16::from(*frame_type - 64)
                    }
                    StackFrame::SameLocals1StackItemFrameExtended { offset_delta, .. }
                    | StackFrame::ChopFrame { offset_delta, .. }
                    | StackFrame::SameFrameExtended { offset_delta, .. }
                    | StackFrame::AppendFrame { offset_delta, .. }
                    | StackFrame::FullFrame { offset_delta, .. } => *offset_delta,
                };

                // offset_delta is the instruction index
                let instruction_index = offset_delta as usize;
                if instruction_index >= self.instruction_offsets.len() {
                    return Err(VerifyError::VerifyError(format!(
                        "Invalid StackMapTable frame index {instruction_index}"
                    )));
                }
                let offset = self.instruction_offsets[instruction_index];

                let next_frame = self.apply_stack_frame(&current_frame, stack_frame)?;
                frames.insert(offset, next_frame.clone());
                worklist.push_back(offset);
                current_frame = next_frame;
            }
        }

        while let Some(offset) = worklist.pop_front() {
            let frame = frames
                .get(&offset)
                .cloned()
                .ok_or_else(|| VerifyError::VerifyError(format!("No frame at offset {offset}")))?;
            let index = *self.offset_to_index.get(&offset).ok_or_else(|| {
                VerifyError::VerifyError(format!("Invalid instruction offset {offset}"))
            })?;
            let instruction = &self.code[index];

            // Execute instruction (simulate)
            let (next_frame, successors) = self.execute_instruction(offset, instruction, &frame)?;

            // Merge next_frame into successors
            for successor_offset in successors {
                if successor_offset >= self.code_length {
                    return Err(VerifyError::VerifyError(format!(
                        "Fell off end of code at offset {offset}"
                    )));
                }

                if let Some(existing_frame) = frames.get_mut(&successor_offset) {
                    if self.merge_frames(existing_frame, &next_frame)? {
                        worklist.push_back(successor_offset);
                    }
                } else {
                    frames.insert(successor_offset, next_frame.clone());
                    worklist.push_back(successor_offset);
                }
            }

            // Exception Handlers
            for handler in self.exception_table {
                if handler.range_pc.contains(&offset) {
                    let mut handler_frame = frame.clone();
                    handler_frame.stack.clear();

                    let exception_type = if handler.catch_type == 0 {
                        VerificationType::Object("java/lang/Throwable".to_string())
                    } else {
                        let class_name = self
                            .class_file
                            .constant_pool
                            .try_get_class(handler.catch_type)
                            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;
                        VerificationType::Object(class_name.to_string())
                    };
                    handler_frame.push(exception_type)?;

                    if let Some(existing_frame) = frames.get_mut(&handler.handler_pc) {
                        if self.merge_frames(existing_frame, &handler_frame)? {
                            worklist.push_back(handler.handler_pc);
                        }
                    } else {
                        frames.insert(handler.handler_pc, handler_frame);
                        worklist.push_back(handler.handler_pc);
                    }
                }
            }
        }

        Ok(())
    }

    /// Creates the initial frame for the method.
    ///
    /// # Errors
    ///
    /// Returns `VerifyError` if the initial frame cannot be created.
    fn create_initial_frame(&self) -> Result<Frame> {
        let max_locals = self.max_locals as usize;
        let max_stack = self.max_stack as usize;
        let mut frame = Frame::new(max_locals, max_stack);

        let mut local_index = 0;

        let class_name = self
            .class_file
            .constant_pool
            .try_get_class(self.class_file.this_class)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;
        let method_name = self
            .class_file
            .constant_pool
            .try_get_utf8(self.method.name_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;
        let descriptor = self
            .class_file
            .constant_pool
            .try_get_utf8(self.method.descriptor_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        // 'this' reference
        if !self.method.access_flags.contains(MethodAccessFlags::STATIC) {
            if method_name == "<init>" {
                frame.locals[local_index] = VerificationType::UninitializedThis;
            } else {
                frame.locals[local_index] = VerificationType::Object(class_name.to_string());
            }
            local_index += 1;
        }

        // Parameters
        let (parameters, _) = FieldType::parse_method_descriptor(descriptor)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        for param in parameters {
            if local_index >= max_locals {
                return Err(VerifyError::VerifyError(
                    "Arguments exceed max_locals".to_string(),
                ));
            }
            // Map FieldType to VerificationType
            let v_type = match &param {
                FieldType::Base(b) => match b {
                    crate::BaseType::Float => VerificationType::Float,
                    crate::BaseType::Long => VerificationType::Long,
                    crate::BaseType::Double => VerificationType::Double,
                    // Byte, Short, Char, Boolean, Int -> Integer
                    _ => VerificationType::Integer,
                },
                FieldType::Object(name) => VerificationType::Object(name.clone()),
                FieldType::Array(_component) => {
                    // Primitive arrays are Objects in verification
                    VerificationType::Object(format!("[{param}")) // Rough approximation
                }
            };

            frame.locals[local_index] = v_type.clone();
            local_index += 1;
            if matches!(v_type, VerificationType::Long | VerificationType::Double)
                && local_index < max_locals
            {
                frame.locals[local_index] = VerificationType::Top;
                local_index += 1;
            }
        }

        Ok(frame)
    }

    /// Executes an instruction, returning the next frame and successor offsets.
    ///
    /// # Errors
    ///
    /// Returns `VerifyError` if the instruction cannot be executed.
    #[expect(clippy::too_many_lines)]
    fn execute_instruction(
        &self,
        offset: u16,
        instruction: &Instruction,
        frame: &Frame,
    ) -> Result<(Frame, Vec<u16>)> {
        let mut next_frame = frame.clone();
        let mut successors = Vec::new();

        // Calculate next offset (fallthrough)
        let mut cursor = Cursor::new(Vec::new());
        instruction
            .to_bytes(&mut cursor)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;
        let len = u16::try_from(cursor.position())?;
        let fallthrough = offset + len;

        let mut does_fallthrough = true;

        match instruction {
            // Constants
            Instruction::Aconst_null => next_frame.push(VerificationType::Null)?,
            Instruction::Iconst_m1
            | Instruction::Iconst_0
            | Instruction::Iconst_1
            | Instruction::Iconst_2
            | Instruction::Iconst_3
            | Instruction::Iconst_4
            | Instruction::Iconst_5 => next_frame.push(VerificationType::Integer)?,
            Instruction::Lconst_0 | Instruction::Lconst_1 => {
                next_frame.push(VerificationType::Long)?;
                next_frame.push(VerificationType::Top)?;
            }
            Instruction::Fconst_0 | Instruction::Fconst_1 | Instruction::Fconst_2 => {
                next_frame.push(VerificationType::Float)?;
            }
            Instruction::Dconst_0 | Instruction::Dconst_1 => {
                next_frame.push(VerificationType::Double)?;
                next_frame.push(VerificationType::Top)?;
            }
            Instruction::Bipush(_) | Instruction::Sipush(_) => {
                next_frame.push(VerificationType::Integer)?;
            }
            Instruction::Ldc(index) => self.push_constant(&mut next_frame, u16::from(*index))?,
            Instruction::Ldc_w(index) | Instruction::Ldc2_w(index) => {
                self.push_constant(&mut next_frame, *index)?;
            }

            // Loads
            Instruction::Iload(index) => {
                self.load_local(
                    &mut next_frame,
                    u16::from(*index),
                    &VerificationType::Integer,
                )?;
            }
            Instruction::Iload_w(index) => {
                self.load_local(&mut next_frame, *index, &VerificationType::Integer)?;
            }
            Instruction::Lload(index) => {
                self.load_local(&mut next_frame, u16::from(*index), &VerificationType::Long)?;
            }
            Instruction::Lload_w(index) => {
                self.load_local(&mut next_frame, *index, &VerificationType::Long)?;
            }
            Instruction::Fload(index) => {
                self.load_local(&mut next_frame, u16::from(*index), &VerificationType::Float)?;
            }
            Instruction::Fload_w(index) => {
                self.load_local(&mut next_frame, *index, &VerificationType::Float)?;
            }
            Instruction::Dload(index) => {
                self.load_local(
                    &mut next_frame,
                    u16::from(*index),
                    &VerificationType::Double,
                )?;
            }
            Instruction::Dload_w(index) => {
                self.load_local(&mut next_frame, *index, &VerificationType::Double)?;
            }
            Instruction::Aload(index) => {
                Self::load_reference_local(&mut next_frame, u16::from(*index))?;
            }
            Instruction::Aload_w(index) => {
                Self::load_reference_local(&mut next_frame, *index)?;
            }
            Instruction::Iload_0 => {
                self.load_local(&mut next_frame, 0, &VerificationType::Integer)?;
            }
            Instruction::Iload_1 => {
                self.load_local(&mut next_frame, 1, &VerificationType::Integer)?;
            }
            Instruction::Iload_2 => {
                self.load_local(&mut next_frame, 2, &VerificationType::Integer)?;
            }
            Instruction::Iload_3 => {
                self.load_local(&mut next_frame, 3, &VerificationType::Integer)?;
            }
            Instruction::Lload_0 => {
                self.load_local(&mut next_frame, 0, &VerificationType::Long)?;
            }
            Instruction::Lload_1 => {
                self.load_local(&mut next_frame, 1, &VerificationType::Long)?;
            }
            Instruction::Lload_2 => {
                self.load_local(&mut next_frame, 2, &VerificationType::Long)?;
            }
            Instruction::Lload_3 => {
                self.load_local(&mut next_frame, 3, &VerificationType::Long)?;
            }
            Instruction::Fload_0 => {
                self.load_local(&mut next_frame, 0, &VerificationType::Float)?;
            }
            Instruction::Fload_1 => {
                self.load_local(&mut next_frame, 1, &VerificationType::Float)?;
            }
            Instruction::Fload_2 => {
                self.load_local(&mut next_frame, 2, &VerificationType::Float)?;
            }
            Instruction::Fload_3 => {
                self.load_local(&mut next_frame, 3, &VerificationType::Float)?;
            }
            Instruction::Dload_0 => {
                self.load_local(&mut next_frame, 0, &VerificationType::Double)?;
            }
            Instruction::Dload_1 => {
                self.load_local(&mut next_frame, 1, &VerificationType::Double)?;
            }
            Instruction::Dload_2 => {
                self.load_local(&mut next_frame, 2, &VerificationType::Double)?;
            }
            Instruction::Dload_3 => {
                self.load_local(&mut next_frame, 3, &VerificationType::Double)?;
            }
            Instruction::Aload_0 => Self::load_reference_local(&mut next_frame, 0)?,
            Instruction::Aload_1 => Self::load_reference_local(&mut next_frame, 1)?,
            Instruction::Aload_2 => Self::load_reference_local(&mut next_frame, 2)?,
            Instruction::Aload_3 => Self::load_reference_local(&mut next_frame, 3)?,

            // Stores
            Instruction::Istore(index) => {
                self.store_local(
                    &mut next_frame,
                    u16::from(*index),
                    &VerificationType::Integer,
                )?;
            }
            Instruction::Istore_w(index) => {
                self.store_local(&mut next_frame, *index, &VerificationType::Integer)?;
            }
            Instruction::Lstore(index) => {
                self.store_local(&mut next_frame, u16::from(*index), &VerificationType::Long)?;
            }
            Instruction::Lstore_w(index) => {
                self.store_local(&mut next_frame, *index, &VerificationType::Long)?;
            }
            Instruction::Fstore(index) => {
                self.store_local(&mut next_frame, u16::from(*index), &VerificationType::Float)?;
            }
            Instruction::Fstore_w(index) => {
                self.store_local(&mut next_frame, *index, &VerificationType::Float)?;
            }
            Instruction::Dstore(index) => {
                self.store_local(
                    &mut next_frame,
                    u16::from(*index),
                    &VerificationType::Double,
                )?;
            }
            Instruction::Dstore_w(index) => {
                self.store_local(&mut next_frame, *index, &VerificationType::Double)?;
            }
            Instruction::Astore(index) => {
                self.store_reference_local(&mut next_frame, u16::from(*index))?;
            }
            Instruction::Astore_w(index) => {
                self.store_reference_local(&mut next_frame, *index)?;
            }
            Instruction::Istore_0 => {
                self.store_local(&mut next_frame, 0, &VerificationType::Integer)?;
            }
            Instruction::Istore_1 => {
                self.store_local(&mut next_frame, 1, &VerificationType::Integer)?;
            }
            Instruction::Istore_2 => {
                self.store_local(&mut next_frame, 2, &VerificationType::Integer)?;
            }
            Instruction::Istore_3 => {
                self.store_local(&mut next_frame, 3, &VerificationType::Integer)?;
            }
            Instruction::Lstore_0 => {
                self.store_local(&mut next_frame, 0, &VerificationType::Long)?;
            }
            Instruction::Lstore_1 => {
                self.store_local(&mut next_frame, 1, &VerificationType::Long)?;
            }
            Instruction::Lstore_2 => {
                self.store_local(&mut next_frame, 2, &VerificationType::Long)?;
            }
            Instruction::Lstore_3 => {
                self.store_local(&mut next_frame, 3, &VerificationType::Long)?;
            }
            Instruction::Fstore_0 => {
                self.store_local(&mut next_frame, 0, &VerificationType::Float)?;
            }
            Instruction::Fstore_1 => {
                self.store_local(&mut next_frame, 1, &VerificationType::Float)?;
            }
            Instruction::Fstore_2 => {
                self.store_local(&mut next_frame, 2, &VerificationType::Float)?;
            }
            Instruction::Fstore_3 => {
                self.store_local(&mut next_frame, 3, &VerificationType::Float)?;
            }
            Instruction::Dstore_0 => {
                self.store_local(&mut next_frame, 0, &VerificationType::Double)?;
            }
            Instruction::Dstore_1 => {
                self.store_local(&mut next_frame, 1, &VerificationType::Double)?;
            }
            Instruction::Dstore_2 => {
                self.store_local(&mut next_frame, 2, &VerificationType::Double)?;
            }
            Instruction::Dstore_3 => {
                self.store_local(&mut next_frame, 3, &VerificationType::Double)?;
            }
            Instruction::Astore_0 => self.store_reference_local(&mut next_frame, 0)?,
            Instruction::Astore_1 => self.store_reference_local(&mut next_frame, 1)?,
            Instruction::Astore_2 => self.store_reference_local(&mut next_frame, 2)?,
            Instruction::Astore_3 => self.store_reference_local(&mut next_frame, 3)?,

            // Stack Operations
            Instruction::Pop => {
                next_frame.pop()?;
            }
            Instruction::Pop2 => {
                let v = next_frame.pop()?;
                if matches!(v, VerificationType::Double | VerificationType::Long) {
                    // Category 2 takes 1 slot in VerificationType but represents 2 slots.
                } else {
                    next_frame.pop()?;
                }
            }
            Instruction::Dup => {
                let v = next_frame.pop()?;
                if matches!(
                    v,
                    VerificationType::Double | VerificationType::Long | VerificationType::Top
                ) {
                    return Err(VerifyError::VerifyError(
                        "dup on category 2 type".to_string(),
                    ));
                }
                next_frame.push(v.clone())?;
                next_frame.push(v)?;
            }
            Instruction::Swap => {
                let v1 = next_frame.pop()?;
                let v2 = next_frame.pop()?;
                if matches!(
                    v1,
                    VerificationType::Double | VerificationType::Long | VerificationType::Top
                ) || matches!(
                    v2,
                    VerificationType::Double | VerificationType::Long | VerificationType::Top
                ) {
                    return Err(VerifyError::VerifyError(
                        "swap on category 2 type".to_string(),
                    ));
                }
                next_frame.push(v1)?;
                next_frame.push(v2)?;
            }

            // Arithmetic
            Instruction::Iadd
            | Instruction::Isub
            | Instruction::Imul
            | Instruction::Idiv
            | Instruction::Irem
            | Instruction::Ishl
            | Instruction::Ishr
            | Instruction::Iushr
            | Instruction::Iand
            | Instruction::Ior
            | Instruction::Ixor => {
                self.pop_expect(&mut next_frame, &VerificationType::Integer)?;
                self.pop_expect(&mut next_frame, &VerificationType::Integer)?;
                next_frame.push(VerificationType::Integer)?;
            }
            Instruction::Ladd
            | Instruction::Lsub
            | Instruction::Lmul
            | Instruction::Ldiv
            | Instruction::Lrem
            | Instruction::Land
            | Instruction::Lor
            | Instruction::Lxor => {
                self.pop_expect(&mut next_frame, &VerificationType::Long)?;
                self.pop_expect(&mut next_frame, &VerificationType::Long)?;
                next_frame.push(VerificationType::Long)?;
                next_frame.push(VerificationType::Top)?;
            }
            // Shift operations for Long take int as shift amount
            Instruction::Lshl | Instruction::Lshr | Instruction::Lushr => {
                self.pop_expect(&mut next_frame, &VerificationType::Integer)?; // shift amount
                self.pop_expect(&mut next_frame, &VerificationType::Long)?; // value
                next_frame.push(VerificationType::Long)?;
                next_frame.push(VerificationType::Top)?;
            }

            // Return
            Instruction::Return => {
                does_fallthrough = false;
            } // void
            Instruction::Ireturn => {
                self.pop_expect(&mut next_frame, &VerificationType::Integer)?;
                does_fallthrough = false;
            }
            Instruction::Lreturn => {
                self.pop_expect(&mut next_frame, &VerificationType::Long)?;
                does_fallthrough = false;
            }
            Instruction::Freturn => {
                self.pop_expect(&mut next_frame, &VerificationType::Float)?;
                does_fallthrough = false;
            }
            Instruction::Dreturn => {
                self.pop_expect(&mut next_frame, &VerificationType::Double)?;
                does_fallthrough = false;
            }
            Instruction::Areturn => {
                let ret = next_frame.pop()?;
                if !matches!(ret, VerificationType::Object(_) | VerificationType::Null) {
                    // TODO: Array check, method signature check
                }
                does_fallthrough = false;
            }

            // Control Flow
            Instruction::Goto(offset_target) => {
                successors.push(Self::resolve_offset(*offset_target));
                does_fallthrough = false;
            }
            Instruction::Goto_w(offset_target) => {
                let target = u16::try_from(*offset_target)
                    .map_err(|_| VerifyError::VerifyError("Goto_w target too large".to_string()))?;
                successors.push(target);
                does_fallthrough = false;
            }

            Instruction::Ifeq(target)
            | Instruction::Ifne(target)
            | Instruction::Iflt(target)
            | Instruction::Ifge(target)
            | Instruction::Ifgt(target)
            | Instruction::Ifle(target) => {
                self.pop_expect(&mut next_frame, &VerificationType::Integer)?;
                successors.push(*target);
            }

            Instruction::If_icmpeq(target)
            | Instruction::If_icmpne(target)
            | Instruction::If_icmplt(target)
            | Instruction::If_icmpge(target)
            | Instruction::If_icmpgt(target)
            | Instruction::If_icmple(target) => {
                self.pop_expect(&mut next_frame, &VerificationType::Integer)?;
                self.pop_expect(&mut next_frame, &VerificationType::Integer)?;
                successors.push(*target);
            }

            Instruction::If_acmpeq(target) | Instruction::If_acmpne(target) => {
                let v1 = next_frame.pop()?;
                let v2 = next_frame.pop()?;
                if !matches!(
                    v1,
                    VerificationType::Object(_)
                        | VerificationType::Null
                        | VerificationType::Uninitialized(_)
                        | VerificationType::UninitializedThis
                ) {
                    return Err(VerifyError::VerifyError(
                        "If_acmp expected reference".to_string(),
                    ));
                }
                if !matches!(
                    v2,
                    VerificationType::Object(_)
                        | VerificationType::Null
                        | VerificationType::Uninitialized(_)
                        | VerificationType::UninitializedThis
                ) {
                    return Err(VerifyError::VerifyError(
                        "If_acmp expected reference".to_string(),
                    ));
                }
                successors.push(*target);
            }

            Instruction::Ifnull(target) | Instruction::Ifnonnull(target) => {
                let v = next_frame.pop()?;
                if !matches!(
                    v,
                    VerificationType::Object(_)
                        | VerificationType::Null
                        | VerificationType::Uninitialized(_)
                        | VerificationType::UninitializedThis
                ) {
                    return Err(VerifyError::VerifyError(
                        "Ifnull expected reference".to_string(),
                    ));
                }
                successors.push(*target);
            }

            // Object Creation
            Instruction::New(_) => {
                next_frame.push(VerificationType::Uninitialized(offset))?;
            }

            // Method Invocation
            Instruction::Invokespecial(index) => {
                let (class_name, method_name, descriptor) = self.resolve_method_ref(*index)?;

                let (params, _) = FieldType::parse_method_descriptor(&descriptor)
                    .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

                for param in params.iter().rev() {
                    let expected = Self::field_type_to_verification_type(param);
                    self.pop_expect(&mut next_frame, &expected)?;
                }

                let objectref = next_frame.pop()?;

                if method_name == "<init>" {
                    match objectref {
                        VerificationType::Uninitialized(new_offset) => {
                            let initialized_type = VerificationType::Object(class_name);
                            Self::initialize_object(
                                &mut next_frame,
                                &VerificationType::Uninitialized(new_offset),
                                &initialized_type,
                            );
                        }
                        VerificationType::UninitializedThis => {
                            let initialized_type = VerificationType::Object(class_name);
                            Self::initialize_object(
                                &mut next_frame,
                                &VerificationType::UninitializedThis,
                                &initialized_type,
                            );
                        }
                        _ => {
                            return Err(VerifyError::VerifyError(
                                "invokespecial <init> on initialized object".to_string(),
                            ));
                        }
                    }
                } else {
                    self.verify_assignable(&objectref, &VerificationType::Object(class_name))?;
                }
            }

            Instruction::Invokevirtual(index)
            | Instruction::Invokestatic(index)
            | Instruction::Invokeinterface(index, _) => {
                let (class_name, _method_name, descriptor) = self.resolve_method_ref(*index)?;

                let (params, return_type) = FieldType::parse_method_descriptor(&descriptor)
                    .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

                for param in params.iter().rev() {
                    let expected = Self::field_type_to_verification_type(param);
                    self.pop_expect(&mut next_frame, &expected)?;
                }

                if !matches!(instruction, Instruction::Invokestatic(_)) {
                    let objectref = next_frame.pop()?;
                    self.verify_assignable(&objectref, &VerificationType::Object(class_name))?;
                }

                if let Some(ret) = return_type {
                    let v_ret = Self::field_type_to_verification_type(&ret);
                    next_frame.push(v_ret)?;
                    if matches!(
                        ret,
                        FieldType::Base(crate::BaseType::Long | crate::BaseType::Double)
                    ) {
                        next_frame.push(VerificationType::Top)?;
                    }
                }
            }

            _ => {
                // Fallback for unimplemented
            }
        }

        if does_fallthrough {
            successors.push(fallthrough);
        }

        Ok((next_frame, successors))
    }

    const fn resolve_offset(offset: u16) -> u16 {
        offset
    }

    fn push_constant(&self, frame: &mut Frame, index: u16) -> Result<()> {
        let constant = self
            .class_file
            .constant_pool
            .get(index)
            .ok_or(VerifyError::VerifyError(
                "Invalid constant index".to_string(),
            ))?;
        match constant {
            crate::Constant::Integer(_) => frame.push(VerificationType::Integer)?,
            crate::Constant::Float(_) => frame.push(VerificationType::Float)?,
            crate::Constant::Long(_) => {
                frame.push(VerificationType::Long)?;
                frame.push(VerificationType::Top)?;
            }
            crate::Constant::Double(_) => {
                frame.push(VerificationType::Double)?;
                frame.push(VerificationType::Top)?;
            }
            crate::Constant::String(_) => {
                frame.push(VerificationType::Object("java/lang/String".to_string()))?;
            }
            crate::Constant::Class(_) => {
                frame.push(VerificationType::Object("java/lang/Class".to_string()))?;
            }
            _ => {
                return Err(VerifyError::VerifyError(
                    "Unsupported constant type".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn load_local(&self, frame: &mut Frame, index: u16, expected: &VerificationType) -> Result<()> {
        let ty = Self::get_local(frame, index)?;
        self.verify_assignable(&ty, expected)?;
        frame.push(ty)?;
        if matches!(expected, VerificationType::Long | VerificationType::Double) {
            frame.push(VerificationType::Top)?;
        }
        Ok(())
    }

    fn load_reference_local(frame: &mut Frame, index: u16) -> Result<()> {
        let ty = Self::get_local(frame, index)?;
        if !matches!(
            ty,
            VerificationType::Object(_)
                | VerificationType::Null
                | VerificationType::Uninitialized(_)
                | VerificationType::UninitializedThis
        ) {
            return Err(VerifyError::VerifyError(format!(
                "Register {index} contains {ty} but expected reference"
            )));
        }
        frame.push(ty)?;
        Ok(())
    }

    fn store_local(
        &self,
        frame: &mut Frame,
        index: u16,
        val_type: &VerificationType,
    ) -> Result<()> {
        if matches!(val_type, VerificationType::Long | VerificationType::Double) {
            let _top = frame.pop()?;
            let val = frame.pop()?;
            self.verify_assignable(&val, val_type)?;
            self.set_local(frame, index, val)?;
            self.set_local(frame, index + 1, VerificationType::Top)?;
        } else {
            let val = frame.pop()?;
            self.verify_assignable(&val, val_type)?;
            self.set_local(frame, index, val)?;
        }
        Ok(())
    }

    fn store_reference_local(&self, frame: &mut Frame, index: u16) -> Result<()> {
        let val = frame.pop()?;
        if !matches!(
            val,
            VerificationType::Object(_)
                | VerificationType::Null
                | VerificationType::Uninitialized(_)
                | VerificationType::UninitializedThis
        ) {
            return Err(VerifyError::VerifyError(format!(
                "Stack top {val} is not a reference"
            )));
        }
        self.set_local(frame, index, val)?;
        Ok(())
    }

    fn get_local(frame: &Frame, index: u16) -> Result<VerificationType> {
        if (index as usize) < frame.locals.len() {
            Ok(frame.locals[index as usize].clone())
        } else {
            Err(VerifyError::VerifyError(
                "Local index out of bounds".to_string(),
            ))
        }
    }

    fn set_local(&self, frame: &mut Frame, index: u16, val: VerificationType) -> Result<()> {
        if (index as usize) < frame.locals.len() {
            frame.locals[index as usize] = val;
            Ok(())
        } else if (index as usize) < self.max_locals as usize {
            frame
                .locals
                .resize((index + 1) as usize, VerificationType::Top);
            frame.locals[index as usize] = val;
            Ok(())
        } else {
            Err(VerifyError::VerifyError(
                "Local index out of max_locals".to_string(),
            ))
        }
    }

    fn pop_expect(&self, frame: &mut Frame, expected: &VerificationType) -> Result<()> {
        let val = frame.pop()?;
        if matches!(expected, VerificationType::Long | VerificationType::Double) {
            if val != VerificationType::Top {
                return Err(VerifyError::VerifyError(
                    "Expected Top for Cat2".to_string(),
                ));
            }
            let val2 = frame.pop()?;
            self.verify_assignable(&val2, expected)?;
        } else {
            self.verify_assignable(&val, expected)?;
        }
        Ok(())
    }

    fn verify_assignable(
        &self,
        source: &VerificationType,
        target: &VerificationType,
    ) -> Result<()> {
        if source == target {
            return Ok(());
        }
        match (source, target) {
            (VerificationType::Top, _)
            | (_, VerificationType::Top)
            | (VerificationType::Null, VerificationType::Object(_)) => Ok(()),
            (VerificationType::Object(s), VerificationType::Object(t)) => {
                if self.context.is_assignable(t, s)? {
                    Ok(())
                } else {
                    Err(VerifyError::VerifyError(format!(
                        "Type {source} is not assignable to {target}"
                    )))
                }
            }
            _ => Err(VerifyError::VerifyError(format!(
                "Type {source} is not assignable to {target}"
            ))),
        }
    }

    /// Replaces all occurrences of an uninitialized type with an initialized type.
    fn initialize_object(
        frame: &mut Frame,
        uninitialized: &VerificationType,
        initialized: &VerificationType,
    ) {
        for local in &mut frame.locals {
            if *local == *uninitialized {
                *local = initialized.clone();
            }
        }
        for stack_entry in &mut frame.stack {
            if *stack_entry == *uninitialized {
                *stack_entry = initialized.clone();
            }
        }
    }

    fn field_type_to_verification_type(ft: &FieldType) -> VerificationType {
        match ft {
            FieldType::Base(b) => match b {
                crate::BaseType::Boolean
                | crate::BaseType::Byte
                | crate::BaseType::Short
                | crate::BaseType::Char
                | crate::BaseType::Int => VerificationType::Integer,
                crate::BaseType::Float => VerificationType::Float,
                crate::BaseType::Long => VerificationType::Long,
                crate::BaseType::Double => VerificationType::Double,
            },
            FieldType::Object(s) => VerificationType::Object(s.clone()),
            FieldType::Array(_) => VerificationType::Object("java/lang/Object".to_string()), // Simplified
        }
    }

    fn resolve_method_ref(&self, index: u16) -> Result<(String, String, String)> {
        let (class_index, name_and_type_index) = self
            .class_file
            .constant_pool
            .try_get_method_ref(index)
            .or_else(|_| {
                self.class_file
                    .constant_pool
                    .try_get_interface_method_ref(index)
            })
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let class_name = self
            .class_file
            .constant_pool
            .try_get_class(*class_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let (name_index, descriptor_index) = self
            .class_file
            .constant_pool
            .try_get_name_and_type(*name_and_type_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let name = self
            .class_file
            .constant_pool
            .try_get_utf8(*name_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let descriptor = self
            .class_file
            .constant_pool
            .try_get_utf8(*descriptor_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        Ok((
            class_name.to_string(),
            name.to_string(),
            descriptor.to_string(),
        ))
    }

    /// Merges two `Frame`s, updating the target frame.
    ///
    /// # Errors
    ///
    /// Returns `VerifyError` if the frames cannot be merged.
    fn merge_frames(&self, target: &mut Frame, source: &Frame) -> Result<bool> {
        let mut changed = false;
        // If target has fewer locals, we ignore the extra locals in source (truncation).
        // If target has MORE locals, source is missing locals, which is an error.
        if target.locals.len() > source.locals.len() {
            return Err(VerifyError::VerifyError(
                "Frame merge mismatch: locals count".to_string(),
            ));
        }
        if target.stack.len() != source.stack.len() {
            return Err(VerifyError::VerifyError(
                "Frame merge mismatch: stack depth".to_string(),
            ));
        }

        for (t_local, s_local) in target.locals.iter_mut().zip(&source.locals) {
            if t_local != s_local && *t_local != VerificationType::Top {
                // Merge logic (LUB - Least Upper Bound)
                // Implement JVMS 4.10.1.2 merging rules
                let merged_type = self.merge_types(t_local, s_local)?;
                if *t_local != merged_type {
                    *t_local = merged_type;
                    changed = true;
                }
            }
        }

        for (t_stack, s_stack) in target.stack.iter_mut().zip(&source.stack) {
            if t_stack != s_stack && *t_stack != VerificationType::Top {
                let merged_type = self.merge_types(t_stack, s_stack)?;
                if *t_stack != merged_type {
                    *t_stack = merged_type;
                    changed = true;
                }
            }
        }

        Ok(changed)
    }

    fn merge_types(
        &self,
        t1: &VerificationType,
        t2: &VerificationType,
    ) -> Result<VerificationType> {
        if t1 == t2 {
            return Ok(t1.clone());
        }

        match (t1, t2) {
            (VerificationType::Top, _) | (_, VerificationType::Top) => Ok(VerificationType::Top),
            (VerificationType::Null, t) | (t, VerificationType::Null) => {
                if matches!(
                    t,
                    VerificationType::Object(_)
                        | VerificationType::Uninitialized(_)
                        | VerificationType::UninitializedThis
                ) {
                    Ok(t.clone()) // Null is assignable to any reference
                } else {
                    Ok(VerificationType::Top) // Incompatible
                }
            }
            (VerificationType::Object(c1), VerificationType::Object(c2)) => {
                // Find common superclass
                let common = self.context.common_superclass(c1, c2)?;
                Ok(VerificationType::Object(common))
            }
            // Handle array merging logic here if VerificationType::Object supports arrays in a way we can detect,
            // or if we add specific Array variant.
            // Currently generic Object(String) is used. The context.common_superclass should handle array strings like "[Ljava/lang/String;" vs "[Ljava/lang/Object;" -> "[Ljava/lang/Object;"
            // and "[I" vs "[F" -> "java/lang/Object" etc.
            (VerificationType::Uninitialized(_), VerificationType::Uninitialized(_)) => {
                // Must be exact match, checked at top. If different, merge to Top (verification failure really)
                Ok(VerificationType::Top)
            }
            (VerificationType::UninitializedThis, VerificationType::UninitializedThis) => {
                Ok(VerificationType::UninitializedThis)
            }
            _ => Ok(VerificationType::Top),
        }
    }

    /// Applies a `StackFrame` to the current `Frame`, producing the next `Frame`.
    ///
    /// # Errors
    ///
    /// Returns `VerifyError` if the stack frame cannot be applied.
    fn apply_stack_frame(&self, current_frame: &Frame, stack_frame: &StackFrame) -> Result<Frame> {
        let mut next_frame = current_frame.clone();
        next_frame.stack.clear();

        match stack_frame {
            StackFrame::SameFrame { .. } | StackFrame::SameFrameExtended { .. } => {
                // Locals same as previous, stack empty.
            }
            StackFrame::SameLocals1StackItemFrame { stack, .. }
            | StackFrame::SameLocals1StackItemFrameExtended { stack, .. } => {
                for s in stack {
                    let v_type = self.convert_verification_type(s);
                    next_frame.push(v_type)?;
                }
            }
            StackFrame::ChopFrame { frame_type, .. } => {
                let k = 251 - frame_type;
                let len = next_frame.locals.len();
                if len < k as usize {
                    return Err(VerifyError::VerifyError(
                        "ChopFrame: locals underflow".to_string(),
                    ));
                }
                next_frame.locals.truncate(len - k as usize);
            }
            StackFrame::AppendFrame { locals, .. } => {
                for local in locals {
                    let v_type = self.convert_verification_type(local);
                    next_frame.locals.push(v_type);
                }
            }
            StackFrame::FullFrame { locals, stack, .. } => {
                next_frame.locals.clear();
                for local in locals {
                    let v_type = self.convert_verification_type(local);
                    next_frame.locals.push(v_type);
                }
                for s in stack {
                    let v_type = self.convert_verification_type(s);
                    next_frame.push(v_type)?;
                }
            }
        }
        Ok(next_frame)
    }

    /// Converts a `crate::attributes::VerificationType` to `VerificationType`.
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
                    VerificationType::Object(name.to_string())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Method;
    use crate::Version;
    use crate::attributes::Attribute;
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
        // Index 1: Utf8 "TestClass"
        constant_pool
            .add(Constant::Utf8("TestClass".to_string()))
            .unwrap();
        // Index 2: Class "TestClass"
        let this_class_index = constant_pool.add(Constant::Class(1)).unwrap();
        // Index 3: Utf8 "testMethod"
        constant_pool
            .add(Constant::Utf8("testMethod".to_string()))
            .unwrap();
        // Index 4: Utf8 "()V"
        constant_pool
            .add(Constant::Utf8("()V".to_string()))
            .unwrap();
        // Index 5: Utf8 "Code"
        constant_pool
            .add(Constant::Utf8("Code".to_string()))
            .unwrap();

        ClassFile {
            version: Version::Java8 { minor: 0 },
            constant_pool,
            access_flags: crate::class_access_flags::ClassAccessFlags::PUBLIC,
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
    fn test_verify_stack_map_table() {
        let class_file = create_mock_class_file();
        // 0: goto 2 (3 bytes) -> target index 2 (offset 4)
        // 3: nop (1 byte) -> index 1
        // 4: return (1 byte) -> index 2

        let code = vec![Instruction::Goto(4), Instruction::Nop, Instruction::Return];

        // Frame at index 2: SameFrame (offset_delta=2)
        let stack_map = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![StackFrame::SameFrame { frame_type: 2 }],
        };

        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 1,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![stack_map],
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
    fn test_verify_stack_map_table_same_locals_1_stack_item() -> Result<()> {
        let class_file = create_mock_class_file();
        // 0: iconst_0 (1 byte) -> index 0. Stack: [int]
        // 1: goto 2 (3 bytes) -> target index 2 (offset 4). Stack: [int]
        // 4: iconst_1 (1 byte) -> index 2. Stack: [int] (from frame) -> [int, int]
        // 5: return (1 byte) -> index 3

        let code = vec![
            Instruction::Iconst_0,
            Instruction::Goto(4),
            Instruction::Iconst_1,
            Instruction::Return,
        ];

        // Frame at index 2: SameLocals1StackItemFrame (offset_delta=2)
        // Stack: [Integer]
        let stack_map = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![StackFrame::SameLocals1StackItemFrame {
                frame_type: 64 + 2,
                stack: vec![crate::attributes::VerificationType::Integer],
            }],
        };

        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 2,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![stack_map],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        verify(&class_file, &method, &MockContext)
    }

    #[test]
    fn test_verify_stack_map_table_full_frame() -> Result<()> {
        let class_file = create_mock_class_file();
        // 0: goto 2 (3 bytes) -> target index 2 (offset 4)
        // 3: nop (1 byte) -> index 1
        // 4: return (1 byte) -> index 2

        let code = vec![Instruction::Goto(4), Instruction::Nop, Instruction::Return];

        // Frame at index 2: FullFrame (offset_delta=2)
        // Locals: [Object("TestClass")]
        // Stack: []
        let stack_map = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![StackFrame::FullFrame {
                frame_type: 255,
                offset_delta: 2,
                locals: vec![crate::attributes::VerificationType::Object { cpool_index: 2 }],
                stack: vec![],
            }],
        };

        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 1,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![stack_map],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        verify(&class_file, &method, &MockContext)
    }

    #[test]
    fn test_verify_stack_map_table_same_frame_extended() -> Result<()> {
        let class_file = create_mock_class_file();
        let code = vec![Instruction::Goto(4), Instruction::Nop, Instruction::Return];

        let stack_map = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![StackFrame::SameFrameExtended {
                frame_type: 251,
                offset_delta: 2,
            }],
        };

        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 1,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![stack_map],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        verify(&class_file, &method, &MockContext)
    }

    #[test]
    fn test_verify_stack_map_table_same_locals_1_stack_item_extended() -> Result<()> {
        let class_file = create_mock_class_file();
        let code = vec![
            Instruction::Iconst_0,
            Instruction::Goto(4),
            Instruction::Iconst_1,
            Instruction::Return,
        ];

        let stack_map = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![StackFrame::SameLocals1StackItemFrameExtended {
                frame_type: 247,
                offset_delta: 2,
                stack: vec![crate::attributes::VerificationType::Integer],
            }],
        };

        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 2,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![stack_map],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        verify(&class_file, &method, &MockContext)
    }

    #[test]
    fn test_verify_stack_map_table_chop_frame() -> Result<()> {
        let class_file = create_mock_class_file();
        let code = vec![Instruction::Goto(4), Instruction::Nop, Instruction::Return];

        let stack_map = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![StackFrame::ChopFrame {
                frame_type: 249,
                offset_delta: 2,
            }],
        };

        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 1,
            max_locals: 3,
            code,
            exception_table: vec![],
            attributes: vec![stack_map],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        verify(&class_file, &method, &MockContext)
    }

    #[test]
    fn test_verify_stack_map_table_append_frame() -> Result<()> {
        let class_file = create_mock_class_file();
        // 0: Goto 2 (offset 4)
        // 3: Nop
        // 4: Iconst_0 (1 byte) -> index 2
        // 5: Istore_1 (1 byte) -> index 3
        // 6: Return (1 byte) -> index 4

        let code = vec![
            Instruction::Goto(4),
            Instruction::Nop,
            Instruction::Iconst_0,
            Instruction::Istore_1,
            Instruction::Return,
        ];

        // StackMap at 4: ChopFrame (chop 1).
        // StackMap at 6: AppendFrame (append int).

        let stack_map = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![
                StackFrame::ChopFrame {
                    frame_type: 250,
                    offset_delta: 4,
                },
                StackFrame::AppendFrame {
                    frame_type: 252,
                    offset_delta: 1,
                    locals: vec![crate::attributes::VerificationType::Integer],
                },
            ],
        };

        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 1,
            max_locals: 2,
            code,
            exception_table: vec![],
            attributes: vec![stack_map],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        verify(&class_file, &method, &MockContext)
    }

    #[test]
    fn test_verify_stack_map_table_chop_frame_underflow() {
        let class_file = create_mock_class_file();
        let code = vec![Instruction::Goto(4), Instruction::Nop, Instruction::Return];

        let stack_map = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![StackFrame::ChopFrame {
                frame_type: 249,
                offset_delta: 2,
            }],
        };

        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 1,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![stack_map],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        let result = verify(&class_file, &method, &MockContext);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "VerifyError: ChopFrame: locals underflow"
        );
    }

    #[test]
    fn test_verify_instructions() -> Result<()> {
        let class_file = create_mock_class_file();
        let code = vec![
            Instruction::Aconst_null,
            Instruction::Lconst_0,
            Instruction::Fconst_0,
            Instruction::Dconst_0,
            Instruction::Aload_0,
            Instruction::Return,
        ];

        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 7,
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

        verify(&class_file, &method, &MockContext)
    }

    #[test]
    fn test_verify_initial_frame_static() -> Result<()> {
        let class_file = create_mock_class_file();
        let code = vec![Instruction::Return];
        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 0,
            max_locals: 0,
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

        verify(&class_file, &method, &MockContext)
    }

    #[test]
    fn test_verify_initial_frame_parameters() -> Result<()> {
        let mut class_file = create_mock_class_file();
        class_file
            .constant_pool
            .add(Constant::Utf8("(IDJ)V".to_string()))
            .unwrap();

        let code = vec![Instruction::Return];
        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 0,
            max_locals: 6,
            code,
            exception_table: vec![],
            attributes: vec![],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 6,
            attributes: vec![code_attribute],
        };

        verify(&class_file, &method, &MockContext)
    }

    #[test]
    fn test_verify_native_method_with_code() {
        let class_file = create_mock_class_file();
        let code = vec![Instruction::Return];
        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 0,
            max_locals: 1,
            code,
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
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("is native or abstract but has Code attribute")
        );
    }

    #[test]
    fn test_verify_method_no_code() {
        let class_file = create_mock_class_file();
        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![],
        };

        let result = verify(&class_file, &method, &MockContext);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "ClassFormatError: Method has no Code attribute"
        );
    }

    #[test]
    fn test_verify_stack_map_invalid_index() {
        let class_file = create_mock_class_file();
        let code = vec![Instruction::Return];
        let stack_map = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![StackFrame::SameFrame { frame_type: 10 }],
        };
        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 0,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![stack_map],
        };
        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };
        let result = verify(&class_file, &method, &MockContext);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid StackMapTable frame index")
        );
    }

    #[test]
    fn test_verify_fall_off_end() {
        let class_file = create_mock_class_file();
        let code = vec![Instruction::Nop];
        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 0,
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
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Fell off end of code")
        );
    }

    #[test]
    fn test_verify_invalid_goto() {
        let class_file = create_mock_class_file();
        let code = vec![Instruction::Goto(10)];
        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 0,
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
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Fell off end of code")
        );
    }

    #[test]
    fn test_verify_merge_mismatch_stack_depth() {
        let class_file = create_mock_class_file();
        let code = vec![Instruction::Nop, Instruction::Return];

        let stack_map = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![StackFrame::SameLocals1StackItemFrame {
                frame_type: 64 + 1,
                stack: vec![crate::attributes::VerificationType::Integer],
            }],
        };

        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 1,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![stack_map],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        let result = verify(&class_file, &method, &MockContext);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Frame merge mismatch: stack depth")
        );
    }

    #[test]
    fn test_verify_aload_0_invalid() {
        let class_file = create_mock_class_file();
        let code = vec![Instruction::Aload_0, Instruction::Return];
        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 1,
            max_locals: 0,
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
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Local index out of bounds")
        );
    }
}
