//! Constraints shared by both bytecode verification algorithms.
use super::frame::Frame;
use super::handlers::references::ConstantPoolResolver;
use super::type_system::VerificationType;
use crate::attributes::{Attribute, Instruction};
use crate::verifiers::error::{Result, VerifyError};
use crate::{ClassFile, Constant, FieldType, Method, MethodAccessFlags};

fn invalid(message: &str) -> VerifyError {
    VerifyError::VerifyError(message.to_string())
}

fn receiver<'a>(frame: &'a Frame, descriptor: &str) -> Result<&'a VerificationType> {
    let descriptor = crate::JavaStr::cow_from_str(descriptor);
    let (parameters, _) = FieldType::parse_method_descriptor(&descriptor)?;
    let slots = parameters
        .iter()
        .map(|p| {
            if VerificationType::from_field_type(p).is_category2() {
                2
            } else {
                1
            }
        })
        .sum();
    frame.peek_at(slots)
}

/// Validate operand constraints that apply even to unreachable instructions.
#[expect(
    clippy::too_many_lines,
    reason = "static operand rules for opcode families"
)]
pub(crate) fn verify_static(class: &ClassFile<'_>, method: &Method) -> Result<()> {
    let code_attributes: Vec<_> = method
        .attributes
        .iter()
        .filter(|a| matches!(a, Attribute::Code { .. }))
        .collect();
    let needs_code = class.constant_pool.try_get_utf8(method.name_index)? == "<clinit>"
        || !method
            .access_flags
            .intersects(MethodAccessFlags::ABSTRACT | MethodAccessFlags::NATIVE);
    if code_attributes.len() != usize::from(needs_code) {
        return Err(invalid(
            "Method must have exactly one Code attribute unless abstract or native",
        ));
    }
    let Some(Attribute::Code {
        code, max_locals, ..
    }) = code_attributes.first().copied()
    else {
        return Ok(());
    };
    let (_, bytes) = crate::attributes::offset_utils::instructions_to_bytes(code)?;
    if bytes.is_empty() || bytes.len() > 65535 {
        return Err(invalid("Invalid code length"));
    }
    let resolver = ConstantPoolResolver::new(class);
    for instruction in code {
        if instruction
            .max_locals_index()?
            .is_some_and(|index| index >= *max_locals)
        {
            return Err(invalid("Instruction local index exceeds max_locals"));
        }
        match instruction {
            Instruction::Ldc(i) => {
                super::handlers::misc::handle_ldc(&mut Frame::new(0, 2), class, u16::from(*i))?;
            }
            Instruction::Ldc_w(i) => {
                super::handlers::misc::handle_ldc(&mut Frame::new(0, 2), class, *i)?;
            }
            Instruction::Ldc2_w(i) => {
                super::handlers::misc::handle_ldc2_w(&mut Frame::new(0, 2), class, *i)?;
            }
            Instruction::Getfield(i)
            | Instruction::Getstatic(i)
            | Instruction::Putfield(i)
            | Instruction::Putstatic(i) => {
                resolver.resolve_field_ref(*i)?;
            }
            Instruction::Invokedynamic(i) => {
                resolver.resolve_invoke_dynamic(*i)?;
            }
            Instruction::Anewarray(i) => {
                if resolver
                    .resolve_class(*i)?
                    .bytes()
                    .take_while(|b| *b == b'[')
                    .count()
                    >= 255
                {
                    return Err(invalid("anewarray exceeds 255 dimensions"));
                }
            }
            Instruction::Breakpoint
            | Instruction::Impdep1
            | Instruction::Impdep2
            | Instruction::Wide => return Err(invalid("Reserved opcode in class file")),
            _ => {}
        }
        let invocation = match instruction {
            Instruction::Invokevirtual(i) => Some((*i, false, false)),
            Instruction::Invokeinterface(i, _) => Some((*i, true, false)),
            Instruction::Invokestatic(i) | Instruction::Invokespecial(i) => {
                Some((*i, false, class.version.major() >= 52))
            }
            _ => None,
        };
        if let Some((index, interface, either)) = invocation {
            let constant = class.constant_pool.try_get(index)?;
            if !match constant {
                Constant::MethodRef { .. } => !interface,
                Constant::InterfaceMethodRef { .. } => interface || either,
                _ => false,
            } {
                return Err(invalid(
                    "Invocation has an invalid constant pool reference kind",
                ));
            }
            let (_, name, descriptor) = resolver.resolve_method_ref(index)?;
            if name == "<clinit>"
                || (name == "<init>" && !matches!(instruction, Instruction::Invokespecial(_)))
            {
                return Err(invalid("Illegal invocation of a special method"));
            }
            if !matches!(instruction, Instruction::Invokestatic(_)) {
                let descriptor = crate::JavaStr::cow_from_str(&descriptor);
                let (parameters, _) = FieldType::parse_method_descriptor(&descriptor)?;
                let slots = 1 + parameters
                    .iter()
                    .map(|p| {
                        if VerificationType::from_field_type(p).is_category2() {
                            2
                        } else {
                            1
                        }
                    })
                    .sum::<usize>();
                if slots > 255 {
                    return Err(invalid(
                        "Invocation parameters including receiver exceed 255 slots",
                    ));
                }
                if let Instruction::Invokeinterface(_, count) = instruction
                    && usize::from(*count) != slots
                {
                    return Err(invalid("invokeinterface count does not match descriptor"));
                }
            }
        }
        match instruction {
            Instruction::Instanceof(i) | Instruction::Checkcast(i) => {
                resolver.resolve_class(*i)?;
            }
            Instruction::New(i) if resolver.resolve_class(*i)?.starts_with('[') => {
                return Err(invalid("new cannot allocate an array class"));
            }
            Instruction::Multianewarray(i, dimensions) => {
                let name = resolver.resolve_class(*i)?;
                let rank = name.bytes().take_while(|b| *b == b'[').count();
                if *dimensions == 0 || usize::from(*dimensions) > rank {
                    return Err(invalid("Invalid multianewarray dimensions"));
                }
            }
            Instruction::Jsr(_)
            | Instruction::Jsr_w(_)
            | Instruction::Ret(_)
            | Instruction::Ret_w(_)
                if class.version.major() >= 51 =>
            {
                return Err(invalid("jsr/ret require class file version below 51"));
            }
            _ => {}
        }
    }
    Ok(())
}

/// Validate constructor receiver identity before an instruction changes the frame.
pub(super) fn verify_state<C: crate::verifiers::context::VerificationContext>(
    class: &ClassFile<'_>,
    code: &[Instruction],
    instruction: &Instruction,
    frame: &Frame,
    _context: &C,
) -> Result<()> {
    let resolver = ConstantPoolResolver::new(class);
    match instruction {
        Instruction::Invokespecial(index) => {
            let (owner, name, descriptor) = resolver.resolve_method_ref(*index)?;
            if name == "<init>" {
                match receiver(frame, &descriptor)? {
                    VerificationType::Uninitialized(offset) => {
                        let Some(Instruction::New(class_index)) = code.get(usize::from(*offset))
                        else {
                            return Err(invalid("Constructor receiver does not refer to new"));
                        };
                        if resolver.resolve_class(*class_index)? != owner {
                            return Err(invalid(
                                "Constructor owner does not match allocated class",
                            ));
                        }
                    }
                    VerificationType::UninitializedThis => {
                        let current = class.class_name()?.to_str_lossy();
                        let parent = if class.super_class == 0 {
                            String::new()
                        } else {
                            resolver.resolve_class(class.super_class)?
                        };
                        if owner != current && owner != parent {
                            return Err(invalid(
                                "Constructor must initialize this or its direct superclass",
                            ));
                        }
                    }
                    _ => return Err(invalid("Constructor requires an uninitialized receiver")),
                }
            }
        }
        Instruction::Putfield(index) => {
            let (owner, name, descriptor) = resolver.resolve_field_ref(*index)?;
            let ty = FieldType::parse_java_str(&crate::JavaStr::cow_from_str(&descriptor))?;
            let slots = if VerificationType::from_field_type(&ty).is_category2() {
                2
            } else {
                1
            };
            if *frame.peek_at(slots)? == VerificationType::UninitializedThis
                && (owner != class.class_name()?.to_str_lossy()
                    || !class.fields.iter().any(|field| {
                        class
                            .constant_pool
                            .try_get_utf8(field.name_index)
                            .is_ok_and(|n| n == name.as_str())
                            && class
                                .constant_pool
                                .try_get_utf8(field.descriptor_index)
                                .is_ok_and(|d| d == descriptor.as_str())
                    }))
            {
                return Err(invalid(
                    "putfield on uninitialized this must name a field declared by this class",
                ));
            }
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifiers::bytecode::handlers::test_utils::MockContext;
    use crate::{BaseType, Field, FieldAccessFlags};

    fn class() -> Result<ClassFile<'static>> {
        let mut class = ClassFile::default();
        class.this_class = class.constant_pool.add_class("Test")?;
        class.super_class = class.constant_pool.add_class("Parent")?;
        Ok(class)
    }

    #[test]
    fn constructor_requires_an_allocation_receiver() -> Result<()> {
        let mut class = class()?;
        let constructor = class
            .constant_pool
            .add_method_ref(class.this_class, "<init>", "()V")?;
        let code = [Instruction::Nop, Instruction::New(class.this_class)];
        for (receiver, valid) in [
            (VerificationType::Uninitialized(1), true),
            (VerificationType::Uninitialized(0), false),
            (VerificationType::Uninitialized(2), false),
            (VerificationType::Null, false),
            (VerificationType::java_lang_object(), false),
        ] {
            let mut frame = Frame::new(0, 1);
            frame.push(receiver)?;
            let result = verify_state(
                &class,
                &code,
                &Instruction::Invokespecial(constructor),
                &frame,
                &MockContext::PERMISSIVE,
            );
            assert_eq!(result.is_ok(), valid, "{result:?}");
        }
        Ok(())
    }

    #[test]
    fn constructor_receiver_is_below_category_two_arguments() -> Result<()> {
        let mut class = class()?;
        let constructor =
            class
                .constant_pool
                .add_method_ref(class.this_class, "<init>", "(JD)V")?;
        let mut frame = Frame::new(0, 5);
        frame.push(VerificationType::Uninitialized(0))?;
        frame.push_category2(VerificationType::Long)?;
        frame.push_category2(VerificationType::Double)?;
        verify_state(
            &class,
            &[Instruction::New(class.this_class)],
            &Instruction::Invokespecial(constructor),
            &frame,
            &MockContext::STRICT,
        )?;
        assert_eq!(frame.stack_depth(), 5);
        Ok(())
    }

    #[test]
    fn constructor_this_requires_own_class_or_direct_parent() -> Result<()> {
        for root in [false, true] {
            for (owner, valid) in [("Test", true), ("Parent", !root), ("Other", false)] {
                let mut class = class()?;
                if root {
                    class.super_class = 0;
                }
                let owner = class.constant_pool.add_class(owner)?;
                let constructor = class.constant_pool.add_method_ref(owner, "<init>", "()V")?;
                let mut frame = Frame::new(0, 1);
                frame.push(VerificationType::UninitializedThis)?;
                let result = verify_state(
                    &class,
                    &[],
                    &Instruction::Invokespecial(constructor),
                    &frame,
                    &MockContext::PERMISSIVE,
                );
                assert_eq!(result.is_ok(), valid, "{result:?}");
            }
        }
        Ok(())
    }

    #[test]
    fn uninitialized_this_putfield_requires_a_matching_declaration() -> Result<()> {
        for (descriptor, ty) in [("I", BaseType::Int), ("J", BaseType::Long)] {
            for (owner, name, declared_descriptor, valid) in [
                ("Test", "value", descriptor, true),
                ("Parent", "value", descriptor, false),
                ("Test", "other", descriptor, false),
                ("Test", "value", "F", false),
            ] {
                let mut class = class()?;
                class.fields.push(Field {
                    access_flags: FieldAccessFlags::PUBLIC,
                    name_index: class.constant_pool.add_utf8(name)?,
                    descriptor_index: class.constant_pool.add_utf8(declared_descriptor)?,
                    field_type: FieldType::parse(declared_descriptor)?,
                    attributes: vec![],
                });
                let owner = class.constant_pool.add_class(owner)?;
                let field = class
                    .constant_pool
                    .add_field_ref(owner, "value", descriptor)?;
                let mut frame = Frame::new(0, 3);
                frame.push(VerificationType::UninitializedThis)?;
                let value = VerificationType::from_field_type(&FieldType::Base(ty.clone()));
                if value.is_category2() {
                    frame.push_category2(value)?;
                } else {
                    frame.push(value)?;
                }
                let result = verify_state(
                    &class,
                    &[],
                    &Instruction::Putfield(field),
                    &frame,
                    &MockContext::PERMISSIVE,
                );
                assert_eq!(result.is_ok(), valid, "{result:?}");
            }
        }
        Ok(())
    }
}
