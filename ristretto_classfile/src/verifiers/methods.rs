use crate::BaseType;
use crate::FieldType;
use crate::attributes::Attribute;
use crate::attributes::Instruction;
use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::method::Method;
use crate::verifiers::attributes::AttributeContext;
use crate::verifiers::error::Result;
use crate::verifiers::error::VerifyError::{
    InvalidConstantPoolIndex, InvalidConstantPoolIndexType, VerificationError,
};
use crate::verifiers::{attributes, method_access_flags};

/// Verify the `ClassFile` methods.
///
/// # Errors
/// Returns `VerificationError` if the methods are invalid.
pub(crate) fn verify(class_file: &ClassFile) -> Result<()> {
    for method in &class_file.methods {
        method_access_flags::verify(class_file, method)?;
        verify_name_index(class_file, method)?;
        verify_descriptor_index(class_file, method)?;
        verify_return_instructions(class_file, method)?;
        attributes::verify(
            class_file,
            &method.attributes,
            AttributeContext::Method(method),
        )?;
    }
    Ok(())
}

/// Verify that return instructions in the method match the method's return type.
/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ireturn>
fn verify_return_instructions(class_file: &ClassFile, method: &Method) -> Result<()> {
    // Get the method descriptor
    let Some(Constant::Utf8(descriptor)) = class_file.constant_pool.get(method.descriptor_index)
    else {
        return Ok(()); // Invalid descriptor handled elsewhere
    };

    // Parse the return type from the descriptor
    let Ok((_, return_type)) = FieldType::parse_method_descriptor(descriptor) else {
        return Ok(()); // Invalid descriptor handled elsewhere
    };

    // Find the Code attribute
    let code = method.attributes.iter().find_map(|attr| {
        if let Attribute::Code { code, .. } = attr {
            Some(code)
        } else {
            None
        }
    });

    // If there's no Code attribute, nothing to verify (abstract/native methods)
    let Some(code) = code else {
        return Ok(());
    };

    // Helper to format return type for error messages
    let return_type_str = match &return_type {
        Some(t) => format!("{t:?}"),
        None => "void".to_string(),
    };

    // Verify each return instruction matches the expected return type
    for instruction in code {
        match instruction {
            Instruction::Return => {
                // void return - method must return void
                if return_type.is_some() {
                    return Err(VerificationError {
                        context: "Method return type".to_string(),
                        message: format!(
                            "Method returns {return_type_str} but contains 'return' instruction (void return)"
                        ),
                    });
                }
            }
            Instruction::Ireturn => {
                // int return - method must return int, byte, char, short, or boolean
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
                            message: format!(
                                "Method returns {return_type_str} but contains 'ireturn' instruction (int return)"
                            ),
                        });
                    }
                }
            }
            Instruction::Lreturn => {
                // long return
                if !matches!(&return_type, Some(FieldType::Base(BaseType::Long))) {
                    return Err(VerificationError {
                        context: "Method return type".to_string(),
                        message: format!(
                            "Method returns {return_type_str} but contains 'lreturn' instruction (long return)"
                        ),
                    });
                }
            }
            Instruction::Freturn => {
                // float return
                if !matches!(&return_type, Some(FieldType::Base(BaseType::Float))) {
                    return Err(VerificationError {
                        context: "Method return type".to_string(),
                        message: format!(
                            "Method returns {return_type_str} but contains 'freturn' instruction (float return)"
                        ),
                    });
                }
            }
            Instruction::Dreturn => {
                // double return
                if !matches!(&return_type, Some(FieldType::Base(BaseType::Double))) {
                    return Err(VerificationError {
                        context: "Method return type".to_string(),
                        message: format!(
                            "Method returns {return_type_str} but contains 'dreturn' instruction (double return)"
                        ),
                    });
                }
            }
            Instruction::Areturn => {
                // reference return - method must return an object or array
                match &return_type {
                    Some(FieldType::Object(_) | FieldType::Array(_)) => {}
                    _ => {
                        return Err(VerificationError {
                            context: "Method return type".to_string(),
                            message: format!(
                                "Method returns {return_type_str} but contains 'areturn' instruction (reference return)"
                            ),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn verify_name_index(class_file: &ClassFile, method: &Method) -> Result<()> {
    let name_index = method.name_index;
    match class_file.constant_pool.get(name_index) {
        Some(Constant::Utf8 { .. }) => {} // valid constant
        None => return Err(InvalidConstantPoolIndex(name_index)),
        _ => return Err(InvalidConstantPoolIndexType(name_index)),
    }
    Ok(())
}

fn verify_descriptor_index(class_file: &ClassFile, method: &Method) -> Result<()> {
    let descriptor_index = method.descriptor_index;
    match class_file.constant_pool.get(descriptor_index) {
        Some(Constant::Utf8 { .. }) => {} // valid constant
        None => return Err(InvalidConstantPoolIndex(descriptor_index)),
        _ => return Err(InvalidConstantPoolIndexType(descriptor_index)),
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::method_access_flags::MethodAccessFlags;

    fn get_test_class_file_and_method() -> (ClassFile, Method) {
        let mut class_file = ClassFile::default();
        let constant_pool = &mut class_file.constant_pool;
        constant_pool.push(Constant::Utf8("foo".to_string()));
        constant_pool.push(Constant::Utf8("V".to_string()));
        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 1,
            descriptor_index: 2,
            attributes: vec![],
        };
        (class_file, method)
    }

    fn create_method_with_code(descriptor: &str, code: Vec<Instruction>) -> (ClassFile, Method) {
        let mut class_file = ClassFile::default();
        let constant_pool = &mut class_file.constant_pool;
        constant_pool.push(Constant::Utf8("test".to_string()));
        constant_pool.push(Constant::Utf8(descriptor.to_string()));
        constant_pool.push(Constant::Utf8("Code".to_string()));

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
    fn test_success() {
        let (class_file, _method) = get_test_class_file_and_method();
        assert_eq!(Ok(()), crate::verifiers::methods::verify(&class_file));
    }

    #[test]
    fn test_invalid_access_flags_visibility() {
        let (mut class_file, mut method) = get_test_class_file_and_method();
        method.access_flags = MethodAccessFlags::PUBLIC | MethodAccessFlags::PRIVATE;
        class_file.methods.push(method);
        assert!(verify(&class_file).is_err());
    }

    #[test]
    fn test_invalid_name_index() {
        let (class_file, mut method) = get_test_class_file_and_method();
        method.name_index = u16::MAX;
        assert_eq!(
            Err(InvalidConstantPoolIndex(u16::MAX)),
            verify_name_index(&class_file, &method)
        );
    }

    #[test]
    fn test_invalid_name_index_type() -> Result<()> {
        let (mut class_file, mut method) = get_test_class_file_and_method();
        let constant_pool = &mut class_file.constant_pool;
        constant_pool.push(Constant::Class(method.name_index));
        method.name_index = u16::try_from(constant_pool.len())?;
        assert_eq!(
            Err(InvalidConstantPoolIndexType(method.name_index)),
            verify_name_index(&class_file, &method)
        );
        Ok(())
    }

    #[test]
    fn test_invalid_descriptor_index() {
        let (class_file, mut method) = get_test_class_file_and_method();
        method.descriptor_index = u16::MAX;
        assert_eq!(
            Err(InvalidConstantPoolIndex(u16::MAX)),
            verify_descriptor_index(&class_file, &method)
        );
    }

    #[test]
    fn test_invalid_descriptor_index_type() -> Result<()> {
        let (mut class_file, mut method) = get_test_class_file_and_method();
        let constant_pool = &mut class_file.constant_pool;
        constant_pool.push(Constant::Class(method.descriptor_index));
        method.descriptor_index = u16::try_from(constant_pool.len())?;
        assert_eq!(
            Err(InvalidConstantPoolIndexType(method.descriptor_index)),
            verify_descriptor_index(&class_file, &method)
        );
        Ok(())
    }

    // Return type verification tests

    #[test]
    fn test_void_return_valid() {
        let (class_file, method) = create_method_with_code("()V", vec![Instruction::Return]);
        assert!(verify_return_instructions(&class_file, &method).is_ok());
    }

    #[test]
    fn test_void_return_invalid_ireturn() {
        let (class_file, method) = create_method_with_code("()V", vec![Instruction::Ireturn]);
        let result = verify_return_instructions(&class_file, &method);
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("ireturn"));
        }
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
        let result = verify_return_instructions(&class_file, &method);
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("void return"));
        }
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
