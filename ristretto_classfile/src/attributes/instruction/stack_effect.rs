use super::Instruction;
use crate::{Constant, ConstantPool, Error, FieldType, JavaStr, Result};

impl Instruction {
    /// Calculates the operand stack slots consumed and produced by this instruction.
    ///
    /// # Returns
    ///
    /// Returns `(consumed, produced)`, measured in JVM slots. `long` and `double`
    /// occupy two slots; all other values, including object and array references,
    /// occupy one. For example, `Iadd` returns `(2, 1)` and `Dup` returns `(1, 2)`.
    /// For an instruction that completes normally, the resulting stack height is
    /// `height - consumed + produced`, provided `height >= consumed`.
    ///
    /// Field and invocation effects use descriptors from `constant_pool`.
    /// Invocations consume argument slots plus one receiver slot when applicable
    /// and produce the return type's slot count, or zero for `void`.
    ///
    /// Return instructions and `Athrow` report their required operand slots and
    /// produce zero slots. The caller must terminate the current execution path,
    /// discard its remaining stack, and handle exception-handler stacks separately.
    /// This method does not check stack underflow, operand types or categories,
    /// or control flow.
    #[expect(clippy::too_many_lines)]
    pub(crate) fn stack_effect(&self, constant_pool: &ConstantPool<'_>) -> Result<(u16, u16)> {
        // Exhaustive match: new opcodes must explicitly define their stack effect.
        let effect = match self {
            Instruction::Nop
            | Instruction::Iinc(..)
            | Instruction::Iinc_w(..)
            | Instruction::Goto(..)
            | Instruction::Goto_w(..)
            | Instruction::Ret(..)
            | Instruction::Ret_w(..)
            | Instruction::Return => (0, 0),
            Instruction::Aconst_null
            | Instruction::Iconst_m1
            | Instruction::Iconst_0
            | Instruction::Iconst_1
            | Instruction::Iconst_2
            | Instruction::Iconst_3
            | Instruction::Iconst_4
            | Instruction::Iconst_5
            | Instruction::Fconst_0
            | Instruction::Fconst_1
            | Instruction::Fconst_2
            | Instruction::Bipush(..)
            | Instruction::Sipush(..)
            | Instruction::Ldc(..)
            | Instruction::Ldc_w(..)
            | Instruction::Iload(..)
            | Instruction::Fload(..)
            | Instruction::Aload(..)
            | Instruction::Iload_0
            | Instruction::Iload_1
            | Instruction::Iload_2
            | Instruction::Iload_3
            | Instruction::Fload_0
            | Instruction::Fload_1
            | Instruction::Fload_2
            | Instruction::Fload_3
            | Instruction::Aload_0
            | Instruction::Aload_1
            | Instruction::Aload_2
            | Instruction::Aload_3
            | Instruction::Iload_w(..)
            | Instruction::Fload_w(..)
            | Instruction::Aload_w(..)
            | Instruction::New(..)
            | Instruction::Jsr(..)
            | Instruction::Jsr_w(..) => (0, 1),
            Instruction::Lconst_0
            | Instruction::Lconst_1
            | Instruction::Dconst_0
            | Instruction::Dconst_1
            | Instruction::Ldc2_w(..)
            | Instruction::Lload(..)
            | Instruction::Dload(..)
            | Instruction::Lload_0
            | Instruction::Lload_1
            | Instruction::Lload_2
            | Instruction::Lload_3
            | Instruction::Dload_0
            | Instruction::Dload_1
            | Instruction::Dload_2
            | Instruction::Dload_3
            | Instruction::Lload_w(..)
            | Instruction::Dload_w(..) => (0, 2),
            Instruction::Istore(..)
            | Instruction::Fstore(..)
            | Instruction::Astore(..)
            | Instruction::Istore_0
            | Instruction::Istore_1
            | Instruction::Istore_2
            | Instruction::Istore_3
            | Instruction::Fstore_0
            | Instruction::Fstore_1
            | Instruction::Fstore_2
            | Instruction::Fstore_3
            | Instruction::Astore_0
            | Instruction::Astore_1
            | Instruction::Astore_2
            | Instruction::Astore_3
            | Instruction::Istore_w(..)
            | Instruction::Fstore_w(..)
            | Instruction::Astore_w(..)
            | Instruction::Pop
            | Instruction::Ifeq(..)
            | Instruction::Ifne(..)
            | Instruction::Iflt(..)
            | Instruction::Ifge(..)
            | Instruction::Ifgt(..)
            | Instruction::Ifle(..)
            | Instruction::Ifnull(..)
            | Instruction::Ifnonnull(..)
            | Instruction::Tableswitch(..)
            | Instruction::Lookupswitch(..)
            | Instruction::Ireturn
            | Instruction::Freturn
            | Instruction::Areturn
            | Instruction::Athrow
            | Instruction::Monitorenter
            | Instruction::Monitorexit => (1, 0),
            Instruction::Lstore(..)
            | Instruction::Dstore(..)
            | Instruction::Lstore_0
            | Instruction::Lstore_1
            | Instruction::Lstore_2
            | Instruction::Lstore_3
            | Instruction::Dstore_0
            | Instruction::Dstore_1
            | Instruction::Dstore_2
            | Instruction::Dstore_3
            | Instruction::Lstore_w(..)
            | Instruction::Dstore_w(..)
            | Instruction::Pop2
            | Instruction::Lreturn
            | Instruction::Dreturn
            | Instruction::If_icmpeq(..)
            | Instruction::If_icmpne(..)
            | Instruction::If_icmplt(..)
            | Instruction::If_icmpge(..)
            | Instruction::If_icmpgt(..)
            | Instruction::If_icmple(..)
            | Instruction::If_acmpeq(..)
            | Instruction::If_acmpne(..) => (2, 0),
            Instruction::Iaload
            | Instruction::Faload
            | Instruction::Aaload
            | Instruction::Baload
            | Instruction::Caload
            | Instruction::Saload
            | Instruction::Iadd
            | Instruction::Fadd
            | Instruction::Isub
            | Instruction::Fsub
            | Instruction::Imul
            | Instruction::Fmul
            | Instruction::Idiv
            | Instruction::Fdiv
            | Instruction::Irem
            | Instruction::Frem
            | Instruction::Ishl
            | Instruction::Ishr
            | Instruction::Iushr
            | Instruction::Iand
            | Instruction::Ior
            | Instruction::Ixor
            | Instruction::Fcmpl
            | Instruction::Fcmpg
            | Instruction::L2i
            | Instruction::L2f
            | Instruction::D2i
            | Instruction::D2f => (2, 1),
            Instruction::Laload
            | Instruction::Daload
            | Instruction::Lneg
            | Instruction::Dneg
            | Instruction::L2d
            | Instruction::D2l
            | Instruction::Swap => (2, 2),
            Instruction::Iastore
            | Instruction::Fastore
            | Instruction::Aastore
            | Instruction::Bastore
            | Instruction::Castore
            | Instruction::Sastore => (3, 0),
            Instruction::Lastore | Instruction::Dastore => (4, 0),
            Instruction::Ladd
            | Instruction::Dadd
            | Instruction::Lsub
            | Instruction::Dsub
            | Instruction::Lmul
            | Instruction::Dmul
            | Instruction::Ldiv
            | Instruction::Ddiv
            | Instruction::Lrem
            | Instruction::Drem
            | Instruction::Land
            | Instruction::Lor
            | Instruction::Lxor => (4, 2),
            Instruction::Lshl | Instruction::Lshr | Instruction::Lushr => (3, 2),
            Instruction::Lcmp | Instruction::Dcmpl | Instruction::Dcmpg => (4, 1),
            Instruction::Ineg
            | Instruction::Fneg
            | Instruction::I2f
            | Instruction::F2i
            | Instruction::I2b
            | Instruction::I2c
            | Instruction::I2s
            | Instruction::Newarray(..)
            | Instruction::Anewarray(..)
            | Instruction::Arraylength
            | Instruction::Checkcast(..)
            | Instruction::Instanceof(..) => (1, 1),
            Instruction::I2l
            | Instruction::I2d
            | Instruction::F2l
            | Instruction::F2d
            | Instruction::Dup => (1, 2),
            Instruction::Dup_x1 => (2, 3),
            Instruction::Dup_x2 => (3, 4),
            Instruction::Dup2 => (2, 4),
            Instruction::Dup2_x1 => (3, 5),
            Instruction::Dup2_x2 => (4, 6),
            Instruction::Getstatic(index)
            | Instruction::Putstatic(index)
            | Instruction::Getfield(index)
            | Instruction::Putfield(index) => {
                let (_, name_and_type) = constant_pool.try_get_field_ref(*index)?;
                let descriptor = descriptor(constant_pool, *name_and_type)?;
                let slots = u16::from(FieldType::parse_java_str(descriptor)?.slot_count());
                match self {
                    Instruction::Getstatic(..) => (0, slots),
                    Instruction::Putstatic(..) => (slots, 0),
                    Instruction::Getfield(..) => (1, slots),
                    _ => (1 + slots, 0),
                }
            }
            Instruction::Invokevirtual(index)
            | Instruction::Invokespecial(index)
            | Instruction::Invokestatic(index)
            | Instruction::Invokeinterface(index, _)
            | Instruction::Invokedynamic(index) => {
                let name_and_type = match (self, constant_pool.try_get(*index)?) {
                    (
                        Instruction::Invokevirtual(_)
                        | Instruction::Invokespecial(_)
                        | Instruction::Invokestatic(_),
                        Constant::MethodRef {
                            name_and_type_index,
                            ..
                        },
                    )
                    | (
                        Instruction::Invokespecial(_)
                        | Instruction::Invokestatic(_)
                        | Instruction::Invokeinterface(..),
                        Constant::InterfaceMethodRef {
                            name_and_type_index,
                            ..
                        },
                    )
                    | (
                        Instruction::Invokedynamic(_),
                        Constant::InvokeDynamic {
                            name_and_type_index,
                            ..
                        },
                    ) => *name_and_type_index,
                    _ => return Err(Error::InvalidConstantPoolIndexType(*index)),
                };
                let has_receiver = !matches!(
                    self,
                    Instruction::Invokestatic(_) | Instruction::Invokedynamic(_)
                );
                let effect =
                    invoke_effect(descriptor(constant_pool, name_and_type)?, has_receiver)?;
                if let Instruction::Invokeinterface(_, count) = self
                    && u16::from(*count) != effect.0
                {
                    return Err(Error::InvalidInstruction(self.code()));
                }
                effect
            }
            Instruction::Multianewarray(index, dimensions) => {
                let descriptor = constant_pool.try_get_class(*index)?;
                FieldType::parse_java_str(descriptor)?;
                let rank = descriptor
                    .as_bytes()
                    .iter()
                    .take_while(|&&b| b == b'[')
                    .count();
                if *dimensions == 0 || usize::from(*dimensions) > rank {
                    return Err(Error::InvalidInstruction(self.code()));
                }
                (u16::from(*dimensions), 1)
            }
            Instruction::Wide
            | Instruction::Breakpoint
            | Instruction::Impdep1
            | Instruction::Impdep2 => {
                return Err(Error::InvalidInstruction(self.code()));
            }
        };
        Ok(effect)
    }
}

fn descriptor<'a>(pool: &'a ConstantPool<'_>, name_and_type: u16) -> Result<&'a JavaStr> {
    let (_, index) = pool.try_get_name_and_type(name_and_type)?;
    pool.try_get_utf8(*index)
}

fn invoke_effect(descriptor: &JavaStr, has_receiver: bool) -> Result<(u16, u16)> {
    let (parameters, return_type) = FieldType::parse_method_descriptor(descriptor)?;
    // The parser limits parameter slots to 255, so this sum cannot overflow.
    let consumed = u16::from(has_receiver)
        + parameters
            .iter()
            .map(|parameter| u16::from(parameter.slot_count()))
            .sum::<u16>();
    if consumed > 255 {
        return Err(Error::InvalidMethodDescriptor(descriptor.to_string()));
    }
    let produced = return_type.map_or(0, |field_type| u16::from(field_type.slot_count()));
    Ok((consumed, produced))
}
