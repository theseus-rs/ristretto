//! Record attribute verification according to [JVMS §4.7.30](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.30).
//!
//! This module validates the Record attribute which describes the components
//! of a record class. Record classes were introduced in Java 16.

use crate::attributes::Attribute;
use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::verifiers::error::Result;
use crate::verifiers::error::VerifyError::{
    InvalidConstantPoolIndex, InvalidConstantPoolIndexType, VerificationError,
};

/// Verify the `Record` attribute for a class.
///
/// According to [JVMS §4.7.30](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.30):
/// - The `Record` attribute may appear at most once.
/// - It must only appear in a class that has `ACC_FINAL` set (records are implicitly final).
/// - Each component's `name_index` must be a valid `CONSTANT_Utf8_info`.
/// - Each component's `descriptor_index` must be a valid `CONSTANT_Utf8_info`.
/// - Component names must be unique.
/// - Component descriptors must be valid field descriptors.
///
/// # Errors
/// Returns an error if the Record attribute is invalid.
pub(crate) fn verify(class_file: &ClassFile) -> Result<()> {
    let mut has_record = false;

    for attribute in &class_file.attributes {
        if let Attribute::Record { records, .. } = attribute {
            if has_record {
                return Err(VerificationError {
                    context: "Record".to_string(),
                    message: "Multiple Record attributes are not allowed".to_string(),
                });
            }
            has_record = true;

            // Verify each record component
            let mut component_names = std::collections::HashSet::new();

            for (i, record_component) in records.iter().enumerate() {
                // Verify name_index points to a valid CONSTANT_Utf8_info
                let component_name = match class_file.constant_pool.get(record_component.name_index)
                {
                    Some(Constant::Utf8(name)) => name.clone(),
                    Some(_) => {
                        return Err(InvalidConstantPoolIndexType(record_component.name_index));
                    }
                    None => return Err(InvalidConstantPoolIndex(record_component.name_index)),
                };

                // Verify descriptor_index points to a valid CONSTANT_Utf8_info
                let descriptor = match class_file
                    .constant_pool
                    .get(record_component.descriptor_index)
                {
                    Some(Constant::Utf8(desc)) => desc.clone(),
                    Some(_) => {
                        return Err(InvalidConstantPoolIndexType(
                            record_component.descriptor_index,
                        ));
                    }
                    None => {
                        return Err(InvalidConstantPoolIndex(record_component.descriptor_index));
                    }
                };

                // Validate the descriptor is a valid field descriptor
                if !is_valid_field_descriptor(&descriptor) {
                    return Err(VerificationError {
                        context: "Record".to_string(),
                        message: format!(
                            "Invalid field descriptor '{descriptor}' for record component '{component_name}' at index {i}"
                        ),
                    });
                }

                // Check for duplicate component names
                if !component_names.insert(component_name.clone()) {
                    return Err(VerificationError {
                        context: "Record".to_string(),
                        message: format!(
                            "Duplicate record component name '{component_name}' at index {i}"
                        ),
                    });
                }

                // Recursively verify component attributes
                verify_component_attributes(class_file, &record_component.attributes, i)?;
            }
        }
    }

    Ok(())
}

/// Verify attributes of a record component.
fn verify_component_attributes(
    class_file: &ClassFile,
    attributes: &[Attribute],
    component_index: usize,
) -> Result<()> {
    for attribute in attributes {
        match attribute {
            // Signature attribute is allowed on record components
            Attribute::Signature {
                signature_index, ..
            } => match class_file.constant_pool.get(*signature_index) {
                Some(Constant::Utf8(_)) => {}
                Some(_) => return Err(InvalidConstantPoolIndexType(*signature_index)),
                None => return Err(InvalidConstantPoolIndex(*signature_index)),
            },
            // RuntimeVisibleAnnotations, RuntimeInvisibleAnnotations,
            // RuntimeVisibleTypeAnnotations, RuntimeInvisibleTypeAnnotations,
            // and Unknown attributes are allowed on record components per JVMS
            Attribute::RuntimeVisibleAnnotations { .. }
            | Attribute::RuntimeInvisibleAnnotations { .. }
            | Attribute::RuntimeVisibleTypeAnnotations { .. }
            | Attribute::RuntimeInvisibleTypeAnnotations { .. }
            | Attribute::Unknown { .. } => {}
            // Other attributes are not valid on record components
            other => {
                return Err(VerificationError {
                    context: "Record".to_string(),
                    message: format!(
                        "Invalid attribute '{}' on record component at index {component_index}",
                        other.name()
                    ),
                });
            }
        }
    }
    Ok(())
}

/// Check if a string is a valid field descriptor according to [JVMS §4.3.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.3.2).
fn is_valid_field_descriptor(descriptor: &str) -> bool {
    if descriptor.is_empty() {
        return false;
    }

    let mut chars = descriptor.chars().peekable();
    parse_field_type(&mut chars) && chars.peek().is_none()
}

/// Parse a field type from a character iterator.
fn parse_field_type(chars: &mut std::iter::Peekable<std::str::Chars>) -> bool {
    match chars.next() {
        Some('B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z') => true,
        Some('L') => {
            // Object type: L<classname>;
            let mut found_semicolon = false;
            let mut class_name_len = 0;
            for c in chars.by_ref() {
                if c == ';' {
                    found_semicolon = true;
                    break;
                }
                // Valid class name characters
                if !c.is_alphanumeric() && c != '/' && c != '_' && c != '$' {
                    return false;
                }
                class_name_len += 1;
            }
            // Class name must not be empty
            found_semicolon && class_name_len > 0
        }
        Some('[') => {
            // Array type
            parse_field_type(chars)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClassAccessFlags;
    use crate::attributes::Record;

    fn create_record_component(class_file: &mut ClassFile, name: &str, descriptor: &str) -> Record {
        let name_index = class_file.constant_pool.add_utf8(name).unwrap();
        let descriptor_index = class_file.constant_pool.add_utf8(descriptor).unwrap();
        Record {
            name_index,
            descriptor_index,
            attributes: vec![],
        }
    }

    #[test]
    fn test_valid_record_single_component() {
        let mut class_file = ClassFile {
            access_flags: ClassAccessFlags::FINAL | ClassAccessFlags::SUPER,
            ..Default::default()
        };
        let component = create_record_component(&mut class_file, "name", "Ljava/lang/String;");
        class_file.attributes.push(Attribute::Record {
            name_index: 0,
            records: vec![component],
        });

        assert!(verify(&class_file).is_ok());
    }

    #[test]
    fn test_valid_record_multiple_components() {
        let mut class_file = ClassFile {
            access_flags: ClassAccessFlags::FINAL | ClassAccessFlags::SUPER,
            ..Default::default()
        };
        let component1 = create_record_component(&mut class_file, "x", "I");
        let component2 = create_record_component(&mut class_file, "y", "I");
        let component3 = create_record_component(&mut class_file, "name", "Ljava/lang/String;");
        class_file.attributes.push(Attribute::Record {
            name_index: 0,
            records: vec![component1, component2, component3],
        });

        assert!(verify(&class_file).is_ok());
    }

    #[test]
    fn test_record_empty_components() {
        let mut class_file = ClassFile {
            access_flags: ClassAccessFlags::FINAL | ClassAccessFlags::SUPER,
            ..Default::default()
        };
        class_file.attributes.push(Attribute::Record {
            name_index: 0,
            records: vec![],
        });

        // Empty record is allowed (though unusual)
        assert!(verify(&class_file).is_ok());
    }

    #[test]
    fn test_record_invalid_name_index() {
        let mut class_file = ClassFile::default();
        class_file.attributes.push(Attribute::Record {
            name_index: 0,
            records: vec![Record {
                name_index: 9999,
                descriptor_index: 1,
                attributes: vec![],
            }],
        });

        let result = verify(&class_file);
        assert!(matches!(result, Err(InvalidConstantPoolIndex(9999))));
    }

    #[test]
    fn test_record_wrong_name_index_type() {
        let mut class_file = ClassFile::default();
        let class_index = class_file.constant_pool.add_class("SomeClass").unwrap();
        let descriptor_index = class_file.constant_pool.add_utf8("I").unwrap();
        class_file.attributes.push(Attribute::Record {
            name_index: 0,
            records: vec![Record {
                name_index: class_index,
                descriptor_index,
                attributes: vec![],
            }],
        });

        let result = verify(&class_file);
        assert!(matches!(result, Err(InvalidConstantPoolIndexType(_))));
    }

    #[test]
    fn test_record_invalid_descriptor_index() {
        let mut class_file = ClassFile::default();
        let name_index = class_file.constant_pool.add_utf8("field").unwrap();
        class_file.attributes.push(Attribute::Record {
            name_index: 0,
            records: vec![Record {
                name_index,
                descriptor_index: 9999,
                attributes: vec![],
            }],
        });

        let result = verify(&class_file);
        assert!(matches!(result, Err(InvalidConstantPoolIndex(9999))));
    }

    #[test]
    fn test_record_wrong_descriptor_index_type() {
        let mut class_file = ClassFile::default();
        let name_index = class_file.constant_pool.add_utf8("field").unwrap();
        let class_index = class_file.constant_pool.add_class("SomeClass").unwrap();
        class_file.attributes.push(Attribute::Record {
            name_index: 0,
            records: vec![Record {
                name_index,
                descriptor_index: class_index,
                attributes: vec![],
            }],
        });

        let result = verify(&class_file);
        assert!(matches!(result, Err(InvalidConstantPoolIndexType(_))));
    }

    #[test]
    fn test_record_invalid_descriptor() {
        let mut class_file = ClassFile::default();
        let component = create_record_component(&mut class_file, "field", "InvalidDescriptor");
        class_file.attributes.push(Attribute::Record {
            name_index: 0,
            records: vec![component],
        });

        let result = verify(&class_file);
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("Invalid field descriptor"));
        }
    }

    #[test]
    fn test_record_duplicate_component_names() {
        let mut class_file = ClassFile::default();
        let component1 = create_record_component(&mut class_file, "name", "Ljava/lang/String;");
        let component2 = create_record_component(&mut class_file, "name", "I");
        class_file.attributes.push(Attribute::Record {
            name_index: 0,
            records: vec![component1, component2],
        });

        let result = verify(&class_file);
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("Duplicate record component name"));
        }
    }

    #[test]
    fn test_multiple_record_attributes() {
        let mut class_file = ClassFile::default();
        let component = create_record_component(&mut class_file, "field", "I");
        class_file.attributes.push(Attribute::Record {
            name_index: 0,
            records: vec![component.clone()],
        });
        class_file.attributes.push(Attribute::Record {
            name_index: 0,
            records: vec![component],
        });

        let result = verify(&class_file);
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("Multiple Record attributes"));
        }
    }

    #[test]
    fn test_no_record_attribute() {
        let class_file = ClassFile::default();
        assert!(verify(&class_file).is_ok());
    }

    #[test]
    fn test_record_with_signature_attribute() {
        let mut class_file = ClassFile::default();
        let name_index = class_file.constant_pool.add_utf8("items").unwrap();
        let descriptor_index = class_file
            .constant_pool
            .add_utf8("Ljava/util/List;")
            .unwrap();
        let signature_index = class_file
            .constant_pool
            .add_utf8("Ljava/util/List<TT;>;")
            .unwrap();
        let signature_name_index = class_file.constant_pool.add_utf8("Signature").unwrap();

        class_file.attributes.push(Attribute::Record {
            name_index: 0,
            records: vec![Record {
                name_index,
                descriptor_index,
                attributes: vec![Attribute::Signature {
                    name_index: signature_name_index,
                    signature_index,
                }],
            }],
        });

        assert!(verify(&class_file).is_ok());
    }

    #[test]
    fn test_record_with_invalid_component_attribute() {
        let mut class_file = ClassFile::default();
        let component = Record {
            name_index: class_file.constant_pool.add_utf8("field").unwrap(),
            descriptor_index: class_file.constant_pool.add_utf8("I").unwrap(),
            attributes: vec![Attribute::Code {
                name_index: 0,
                max_stack: 0,
                max_locals: 0,
                code: vec![],
                exception_table: vec![],
                attributes: vec![],
            }],
        };
        class_file.attributes.push(Attribute::Record {
            name_index: 0,
            records: vec![component],
        });

        let result = verify(&class_file);
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("Invalid attribute"));
        }
    }

    // Field descriptor validation tests

    #[test]
    fn test_valid_field_descriptors() {
        assert!(is_valid_field_descriptor("B"));
        assert!(is_valid_field_descriptor("C"));
        assert!(is_valid_field_descriptor("D"));
        assert!(is_valid_field_descriptor("F"));
        assert!(is_valid_field_descriptor("I"));
        assert!(is_valid_field_descriptor("J"));
        assert!(is_valid_field_descriptor("S"));
        assert!(is_valid_field_descriptor("Z"));
        assert!(is_valid_field_descriptor("Ljava/lang/String;"));
        assert!(is_valid_field_descriptor("Ljava/util/List;"));
        assert!(is_valid_field_descriptor("[I"));
        assert!(is_valid_field_descriptor("[[I"));
        assert!(is_valid_field_descriptor("[Ljava/lang/String;"));
        assert!(is_valid_field_descriptor("[[Ljava/lang/Object;"));
    }

    #[test]
    fn test_invalid_field_descriptors() {
        assert!(!is_valid_field_descriptor(""));
        assert!(!is_valid_field_descriptor("V")); // void is not valid for fields
        assert!(!is_valid_field_descriptor("X"));
        assert!(!is_valid_field_descriptor("Ljava/lang/String")); // missing semicolon
        assert!(!is_valid_field_descriptor("L;")); // empty class name
        assert!(!is_valid_field_descriptor("[")); // array with no type
        assert!(!is_valid_field_descriptor("II")); // extra characters
    }

    #[test]
    fn test_record_all_primitive_types() {
        let mut class_file = ClassFile::default();
        let components = vec![
            create_record_component(&mut class_file, "b", "B"),
            create_record_component(&mut class_file, "c", "C"),
            create_record_component(&mut class_file, "d", "D"),
            create_record_component(&mut class_file, "f", "F"),
            create_record_component(&mut class_file, "i", "I"),
            create_record_component(&mut class_file, "j", "J"),
            create_record_component(&mut class_file, "s", "S"),
            create_record_component(&mut class_file, "z", "Z"),
        ];
        class_file.attributes.push(Attribute::Record {
            name_index: 0,
            records: components,
        });

        assert!(verify(&class_file).is_ok());
    }
}
