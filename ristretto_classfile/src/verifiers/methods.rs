use crate::BaseType;
use crate::FieldType;
use crate::attributes::Attribute;
use crate::attributes::Instruction;
use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::method::Method;
use crate::verifiers::attributes;
use crate::verifiers::attributes::AttributeContext;
use crate::verifiers::bytecode::control_flow::CodeInfo;
use crate::verifiers::error::Result;
use crate::verifiers::error::VerifyError::VerificationError;
use ahash::AHashSet;

/// Artifacts from a completed structural method verification.
#[derive(Debug)]
pub(crate) struct VerifiedMethod {
    pub(crate) code_info: Option<CodeInfo>,
}

/// Verify the `ClassFile` methods.
///
/// # Errors
/// Returns `VerificationError` if the methods are invalid.
pub(crate) fn verify(
    class_file: &ClassFile<'_>,
    mut on_method: impl FnMut(VerifiedMethod),
) -> Result<()> {
    let mut methods = AHashSet::new();
    for method in &class_file.methods {
        super::method_access_flags::verify(class_file, method)?;
        let pool = &class_file.constant_pool;
        if !methods.insert((
            pool.try_get_utf8(method.name_index)?,
            pool.try_get_utf8(method.descriptor_index)?,
        )) {
            return Err(crate::verifiers::VerifyError::ClassFormatError(
                "Duplicate method name and descriptor".to_string(),
            ));
        }
        let info = method
            .attributes
            .iter()
            .find_map(code_attribute_instructions)
            .map(CodeInfo::from_code)
            .transpose()?;
        verify_return_instructions(class_file, method, info.as_ref())?;
        attributes::verify_with_code_info(
            class_file,
            &method.attributes,
            AttributeContext::Method(method),
            info.as_ref(),
        )?;
        super::format::verify_method_contents(class_file, method)?;
        on_method(VerifiedMethod { code_info: info });
    }
    Ok(())
}

/// Verify that return instructions in the method match the method's return type.
/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ireturn>
fn verify_return_instructions(
    class_file: &ClassFile<'_>,
    method: &Method,
    info: Option<&CodeInfo>,
) -> Result<()> {
    // Get the method descriptor
    let Some(Constant::Utf8(descriptor)) = class_file.constant_pool.get(method.descriptor_index)
    else {
        return Ok(()); // Invalid descriptor handled elsewhere
    };

    // Parse the return type from the descriptor
    let Ok((_, return_type)) = FieldType::parse_method_descriptor(descriptor) else {
        return Ok(()); // Invalid descriptor handled elsewhere
    };

    let code = method
        .attributes
        .iter()
        .find_map(code_attribute_instructions);

    // If there's no Code attribute, nothing to verify (abstract/native methods)
    let Some(code) = code else {
        return Ok(());
    };

    // Most methods contain only compatible returns. Reachability matters only when
    // an incompatible return could be dead code; avoid constructing a CFG otherwise.
    if returns_are_compatible(code, return_type.as_ref()) {
        return Ok(());
    }

    // Helper to format return type for error messages
    let return_type_str = match &return_type {
        Some(t) => format!("{t:?}"),
        None => "void".to_string(),
    };

    // Return compatibility is a type-checking constraint on reachable instructions.
    let Some(info) = info else {
        return Ok(());
    };
    let reachable = reachable_instructions(method, code, info)?;
    for (index, instruction) in Instruction::with_offsets(code) {
        if !reachable.contains(&u16::try_from(index)?) {
            continue;
        }
        match instruction {
            Instruction::Return
                // void return; method must return void
                if return_type.is_some() => {
                    return Err(VerificationError {
                        context: "Method return type".to_string(),
                        message: ["Method returns ", &return_type_str, " but contains 'return' instruction (void return)"].concat(),
                    });
                }
            Instruction::Ireturn => {
                // int return; method must return int, byte, char, short, or boolean
                match &return_type {
                    Some(FieldType::Base(
                        BaseType::Int
                        | BaseType::Byte
                        | BaseType::Char
                        | BaseType::Short
                        | BaseType::Boolean,
                    )) => {}
                    _ => {
                        return Err(VerificationError {
                            context: "Method return type".to_string(),
                            message: ["Method returns ", &return_type_str, " but contains 'ireturn' instruction (int return)"].concat(),
                        });
                    }
                }
            }
            Instruction::Lreturn
                // long return
                if !matches!(&return_type, Some(FieldType::Base(BaseType::Long))) => {
                    return Err(VerificationError {
                        context: "Method return type".to_string(),
                        message: ["Method returns ", &return_type_str, " but contains 'lreturn' instruction (long return)"].concat(),
                    });
                }
            Instruction::Freturn
                // float return
                if !matches!(&return_type, Some(FieldType::Base(BaseType::Float))) => {
                    return Err(VerificationError {
                        context: "Method return type".to_string(),
                        message: ["Method returns ", &return_type_str, " but contains 'freturn' instruction (float return)"].concat(),
                    });
                }
            Instruction::Dreturn
                // double return
                if !matches!(&return_type, Some(FieldType::Base(BaseType::Double))) => {
                    return Err(VerificationError {
                        context: "Method return type".to_string(),
                        message: ["Method returns ", &return_type_str, " but contains 'dreturn' instruction (double return)"].concat(),
                    });
                }
            Instruction::Areturn => {
                // reference return; method must return an object or array
                match &return_type {
                    Some(FieldType::Object(_) | FieldType::Array(_)) => {}
                    _ => {
                        return Err(VerificationError {
                            context: "Method return type".to_string(),
                            message: ["Method returns ", &return_type_str, " but contains 'areturn' instruction (reference return)"].concat(),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn returns_are_compatible(code: &[Instruction], return_type: Option<&FieldType>) -> bool {
    let expected = match return_type {
        None => Instruction::Return,
        Some(FieldType::Base(BaseType::Long)) => Instruction::Lreturn,
        Some(FieldType::Base(BaseType::Float)) => Instruction::Freturn,
        Some(FieldType::Base(BaseType::Double)) => Instruction::Dreturn,
        Some(FieldType::Base(_)) => Instruction::Ireturn,
        Some(FieldType::Object(_) | FieldType::Array(_)) => Instruction::Areturn,
    };
    !code.iter().any(|instruction| {
        matches!(
            instruction,
            Instruction::Return
                | Instruction::Ireturn
                | Instruction::Lreturn
                | Instruction::Freturn
                | Instruction::Dreturn
                | Instruction::Areturn
        ) && instruction != &expected
    })
}

fn reachable_instructions(
    method: &Method,
    code: &[Instruction],
    info: &CodeInfo,
) -> Result<std::collections::HashSet<u16>> {
    let mut reachable = std::collections::HashSet::new();
    let mut pending = vec![0];
    for attribute in &method.attributes {
        if let Attribute::Code {
            exception_table, ..
        } = attribute
        {
            pending.extend(exception_table.iter().map(|h| h.handler_pc));
        }
    }
    while let Some(pc) = pending.pop() {
        if !reachable.insert(pc) {
            continue;
        }
        if let Some(index) = info.index_at(pc) {
            let Some(instruction) = code.get(index) else {
                continue;
            };
            let next = info.offset_at(index + 1).unwrap_or(info.code_length());
            match instruction {
                Instruction::Jsr(target) => {
                    pending.push(info.branch_target(pc, i32::from(*target))?);
                    pending.push(next);
                }
                Instruction::Jsr_w(target) => {
                    pending.push(info.branch_target(pc, *target)?);
                    pending.push(next);
                }
                _ => {
                    if let Ok((successors, _)) =
                        crate::verifiers::bytecode::control_flow::compute_successors(
                            pc,
                            instruction,
                            next,
                            info,
                        )
                    {
                        pending.extend(successors);
                    }
                }
            }
        }
    }
    Ok(reachable)
}

fn code_attribute_instructions(attribute: &Attribute) -> Option<&[Instruction]> {
    match attribute {
        Attribute::Code { code, .. } => Some(code),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::method_access_flags::MethodAccessFlags;

    fn verify(class_file: &ClassFile<'_>) -> Result<()> {
        super::verify(class_file, drop)
    }

    fn verify_return_instructions(class_file: &ClassFile<'_>, method: &Method) -> Result<()> {
        let info = method
            .attributes
            .iter()
            .find_map(code_attribute_instructions)
            .map(CodeInfo::from_code)
            .transpose()?;
        super::verify_return_instructions(class_file, method, info.as_ref())
    }

    fn get_test_class_file_and_method() -> (ClassFile<'static>, Method) {
        let mut class_file = ClassFile::default();
        let constant_pool = &mut class_file.constant_pool;
        constant_pool.push(Constant::utf8("foo"));
        constant_pool.push(Constant::utf8("()V"));
        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::NATIVE,
            name_index: 1,
            descriptor_index: 2,
            attributes: vec![],
        };
        (class_file, method)
    }

    fn create_method_with_code(
        descriptor: &str,
        code: Vec<Instruction>,
    ) -> (ClassFile<'static>, Method) {
        let mut class_file = ClassFile::default();
        let constant_pool = &mut class_file.constant_pool;
        constant_pool.push(Constant::utf8("test"));
        constant_pool.push(Constant::utf8(descriptor));
        constant_pool.push(Constant::utf8("Code"));

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 1,
            descriptor_index: 2,
            attributes: vec![Attribute::Code {
                name_index: 3,
                max_stack: 1,
                max_locals: 1,
                code,
                exception_table: vec![],
                attributes: vec![],
            }],
        };
        (class_file, method)
    }

    #[test]
    fn method_indices_must_reference_utf8() {
        let (mut class, method) = get_test_class_file_and_method();
        let integer = class.constant_pool.add_integer(42).unwrap();
        for invalid in [u16::MAX, integer] {
            let mut bad_name = method.clone();
            bad_name.name_index = invalid;
            class.methods = vec![bad_name];
            assert!(verify(&class).is_err());

            let mut bad_descriptor = method.clone();
            bad_descriptor.descriptor_index = invalid;
            class.methods = vec![bad_descriptor];
            assert!(verify(&class).is_err());
        }
    }

    #[test]
    fn test_success() {
        let (class_file, _method) = get_test_class_file_and_method();
        assert_eq!(Ok(()), verify(&class_file));
    }

    #[test]
    fn test_invalid_access_flags_visibility() {
        let (mut class_file, mut method) = get_test_class_file_and_method();
        method.access_flags = MethodAccessFlags::PUBLIC | MethodAccessFlags::PRIVATE;
        class_file.methods.push(method);
        assert!(verify(&class_file).is_err());
    }

    // Return type verification tests

    #[test]
    fn test_return_compatibility_distinguishes_dead_and_reachable_code() {
        let (class_file, method) = create_method_with_code(
            "()V",
            vec![
                Instruction::Goto(4),
                Instruction::Ireturn,
                Instruction::Return,
            ],
        );
        assert!(verify_return_instructions(&class_file, &method).is_ok());
        let (class_file, method) = create_method_with_code(
            "()V",
            vec![
                Instruction::Goto(3),
                Instruction::Ireturn,
                Instruction::Return,
            ],
        );
        assert!(verify_return_instructions(&class_file, &method).is_err());
    }

    #[test]
    fn test_void_return_valid() {
        let (class_file, method) = create_method_with_code("()V", vec![Instruction::Return]);
        assert!(verify_return_instructions(&class_file, &method).is_ok());
    }

    #[test]
    fn test_void_return_invalid_ireturn() {
        let (class_file, method) = create_method_with_code("()V", vec![Instruction::Ireturn]);
        let message = verify_return_instructions(&class_file, &method)
            .unwrap_err()
            .to_string();
        assert!(message.contains("ireturn"));
    }

    #[test]
    fn test_int_return_valid() {
        let (class_file, method) =
            create_method_with_code("()I", vec![Instruction::Iconst_0, Instruction::Ireturn]);
        assert!(verify_return_instructions(&class_file, &method).is_ok());
    }

    #[test]
    fn test_int_return_invalid_return() {
        let (class_file, method) = create_method_with_code("()I", vec![Instruction::Return]);
        let message = verify_return_instructions(&class_file, &method)
            .unwrap_err()
            .to_string();
        assert!(message.contains("void return"));
    }

    #[test]
    fn test_byte_return_valid() {
        let (class_file, method) =
            create_method_with_code("()B", vec![Instruction::Iconst_0, Instruction::Ireturn]);
        assert!(verify_return_instructions(&class_file, &method).is_ok());
    }

    #[test]
    fn test_boolean_return_valid() {
        let (class_file, method) =
            create_method_with_code("()Z", vec![Instruction::Iconst_0, Instruction::Ireturn]);
        assert!(verify_return_instructions(&class_file, &method).is_ok());
    }

    #[test]
    fn test_long_return_valid() {
        let (class_file, method) =
            create_method_with_code("()J", vec![Instruction::Lconst_0, Instruction::Lreturn]);
        assert!(verify_return_instructions(&class_file, &method).is_ok());
    }

    #[test]
    fn test_long_return_invalid_ireturn() {
        let (class_file, method) = create_method_with_code("()J", vec![Instruction::Ireturn]);
        let result = verify_return_instructions(&class_file, &method);
        assert!(result.is_err());
    }

    #[test]
    fn test_return_instruction_skips_invalid_descriptor_inputs() {
        let (class_file, mut method) = create_method_with_code("()V", vec![Instruction::Return]);
        method.descriptor_index = u16::MAX;
        assert_eq!(Ok(()), verify_return_instructions(&class_file, &method));

        let (mut class_file, method) =
            create_method_with_code("not a descriptor", vec![Instruction::Return]);
        class_file
            .constant_pool
            .set(method.descriptor_index, Constant::utf8("not a descriptor"))
            .unwrap();
        assert_eq!(Ok(()), verify_return_instructions(&class_file, &method));
    }

    #[test]
    fn test_return_instruction_skips_methods_without_code_attribute() {
        let (class_file, mut method) = create_method_with_code("()V", vec![Instruction::Return]);
        method.attributes = vec![Attribute::Deprecated { name_index: 0 }];
        assert_eq!(Ok(()), verify_return_instructions(&class_file, &method));

        method.attributes.clear();
        assert_eq!(Ok(()), verify_return_instructions(&class_file, &method));
    }

    #[test]
    fn test_category2_and_float_return_mismatches() {
        for (descriptor, instruction, needle) in [
            ("()I", Instruction::Lreturn, "lreturn"),
            ("()I", Instruction::Freturn, "freturn"),
            ("()I", Instruction::Dreturn, "dreturn"),
        ] {
            let (class_file, method) = create_method_with_code(descriptor, vec![instruction]);
            let message = verify_return_instructions(&class_file, &method)
                .unwrap_err()
                .to_string();
            assert!(message.contains(needle));
        }
    }

    #[test]
    fn test_float_return_valid() {
        let (class_file, method) =
            create_method_with_code("()F", vec![Instruction::Fconst_0, Instruction::Freturn]);
        assert!(verify_return_instructions(&class_file, &method).is_ok());
    }

    #[test]
    fn test_double_return_valid() {
        let (class_file, method) =
            create_method_with_code("()D", vec![Instruction::Dconst_0, Instruction::Dreturn]);
        assert!(verify_return_instructions(&class_file, &method).is_ok());
    }

    #[test]
    fn test_object_return_valid() {
        let (class_file, method) = create_method_with_code(
            "()Ljava/lang/Object;",
            vec![Instruction::Aconst_null, Instruction::Areturn],
        );
        assert!(verify_return_instructions(&class_file, &method).is_ok());
    }

    #[test]
    fn test_array_return_valid() {
        let (class_file, method) =
            create_method_with_code("()[I", vec![Instruction::Aconst_null, Instruction::Areturn]);
        assert!(verify_return_instructions(&class_file, &method).is_ok());
    }

    #[test]
    fn test_object_return_invalid_ireturn() {
        let (class_file, method) =
            create_method_with_code("()Ljava/lang/Object;", vec![Instruction::Ireturn]);
        let result = verify_return_instructions(&class_file, &method);
        assert!(result.is_err());
    }

    #[test]
    fn test_int_return_invalid_areturn() {
        let (class_file, method) = create_method_with_code("()I", vec![Instruction::Areturn]);
        let result = verify_return_instructions(&class_file, &method);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_success() {
        let (mut class_file, method) = create_method_with_code("()V", vec![Instruction::Return]);
        class_file.methods.push(method);
        assert_eq!(Ok(()), verify(&class_file));
    }
}
