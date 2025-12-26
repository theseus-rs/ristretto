//! NestHost and NestMembers attribute verification according to [JVMS §4.7.28](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.28) and [§4.7.29](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.29).
//!
//! This module validates the mutual consistency of NestHost and NestMembers attributes.
//! A class can have at most one of these attributes, and they must be mutually consistent.

use crate::attributes::Attribute;
use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::verifiers::error::Result;
use crate::verifiers::error::VerifyError::{
    InvalidConstantPoolIndex, InvalidConstantPoolIndexType, VerificationError,
};

/// Verify `NestHost` and `NestMembers` attributes for a class.
///
/// According to JVMS:
/// - A class file must not have both `NestHost` and `NestMembers` attributes.
/// - `NestHost.host_class_index` must be a valid `CONSTANT_Class_info`.
/// - Each entry in `NestMembers.class_indexes` must be a valid `CONSTANT_Class_info`.
/// - No duplicates are allowed in `NestMembers.class_indexes`.
///
/// # Errors
/// Returns an error if the nest attributes are invalid or inconsistent.
pub(crate) fn verify(class_file: &ClassFile) -> Result<()> {
    let mut has_nest_host = false;
    let mut has_nest_members = false;

    for attribute in &class_file.attributes {
        match attribute {
            Attribute::NestHost {
                host_class_index, ..
            } => {
                if has_nest_host {
                    return Err(VerificationError {
                        context: "NestHost".to_string(),
                        message: "Multiple NestHost attributes are not allowed".to_string(),
                    });
                }
                has_nest_host = true;

                if has_nest_members {
                    return Err(VerificationError {
                        context: "NestHost".to_string(),
                        message: "Class cannot have both NestHost and NestMembers attributes"
                            .to_string(),
                    });
                }

                // Verify host_class_index points to a valid CONSTANT_Class_info
                verify_class_index(class_file, *host_class_index, "NestHost.host_class_index")?;
            }
            Attribute::NestMembers { class_indexes, .. } => {
                if has_nest_members {
                    return Err(VerificationError {
                        context: "NestMembers".to_string(),
                        message: "Multiple NestMembers attributes are not allowed".to_string(),
                    });
                }
                has_nest_members = true;

                if has_nest_host {
                    return Err(VerificationError {
                        context: "NestMembers".to_string(),
                        message: "Class cannot have both NestHost and NestMembers attributes"
                            .to_string(),
                    });
                }

                // Verify each class index
                let mut seen_indexes = std::collections::HashSet::new();
                for (i, &class_index) in class_indexes.iter().enumerate() {
                    verify_class_index(
                        class_file,
                        class_index,
                        &format!("NestMembers.class_indexes[{i}]"),
                    )?;

                    // Check for duplicates
                    if !seen_indexes.insert(class_index) {
                        return Err(VerificationError {
                            context: "NestMembers".to_string(),
                            message: format!("Duplicate class index {class_index} in NestMembers"),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

/// Verify that an index points to a valid `CONSTANT_Class_info` entry.
fn verify_class_index(class_file: &ClassFile, index: u16, _context: &str) -> Result<()> {
    match class_file.constant_pool.get(index) {
        Some(Constant::Class(_)) => Ok(()),
        Some(_) => Err(InvalidConstantPoolIndexType(index)),
        None => Err(InvalidConstantPoolIndex(index)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_class_file_with_attributes(attributes: Vec<Attribute>) -> ClassFile {
        ClassFile {
            attributes,
            ..Default::default()
        }
    }

    // NestHost tests

    #[test]
    fn test_valid_nest_host() {
        let mut class_file = ClassFile::default();
        let class_index = class_file.constant_pool.add_class("OuterClass").unwrap();
        class_file.attributes.push(Attribute::NestHost {
            name_index: 0,
            host_class_index: class_index,
        });

        assert!(verify(&class_file).is_ok());
    }

    #[test]
    fn test_nest_host_invalid_index() {
        let class_file = create_class_file_with_attributes(vec![Attribute::NestHost {
            name_index: 0,
            host_class_index: 9999,
        }]);

        let result = verify(&class_file);
        assert!(matches!(result, Err(InvalidConstantPoolIndex(9999))));
    }

    #[test]
    fn test_nest_host_wrong_index_type() {
        let mut class_file = ClassFile::default();
        let utf8_index = class_file.constant_pool.add_utf8("NotAClass").unwrap();
        class_file.attributes.push(Attribute::NestHost {
            name_index: 0,
            host_class_index: utf8_index,
        });

        let result = verify(&class_file);
        assert!(matches!(result, Err(InvalidConstantPoolIndexType(_))));
    }

    #[test]
    fn test_multiple_nest_host_attributes() {
        let mut class_file = ClassFile::default();
        let class_index = class_file.constant_pool.add_class("OuterClass").unwrap();
        class_file.attributes.push(Attribute::NestHost {
            name_index: 0,
            host_class_index: class_index,
        });
        class_file.attributes.push(Attribute::NestHost {
            name_index: 0,
            host_class_index: class_index,
        });

        let result = verify(&class_file);
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("Multiple NestHost"));
        }
    }

    // NestMembers tests

    #[test]
    fn test_valid_nest_members() {
        let mut class_file = ClassFile::default();
        let class1 = class_file.constant_pool.add_class("Inner1").unwrap();
        let class2 = class_file.constant_pool.add_class("Inner2").unwrap();
        class_file.attributes.push(Attribute::NestMembers {
            name_index: 0,
            class_indexes: vec![class1, class2],
        });

        assert!(verify(&class_file).is_ok());
    }

    #[test]
    fn test_nest_members_empty() {
        let class_file = create_class_file_with_attributes(vec![Attribute::NestMembers {
            name_index: 0,
            class_indexes: vec![],
        }]);

        // Empty NestMembers is allowed
        assert!(verify(&class_file).is_ok());
    }

    #[test]
    fn test_nest_members_invalid_index() {
        let class_file = create_class_file_with_attributes(vec![Attribute::NestMembers {
            name_index: 0,
            class_indexes: vec![9999],
        }]);

        let result = verify(&class_file);
        assert!(matches!(result, Err(InvalidConstantPoolIndex(9999))));
    }

    #[test]
    fn test_nest_members_wrong_index_type() {
        let mut class_file = ClassFile::default();
        let utf8_index = class_file.constant_pool.add_utf8("NotAClass").unwrap();
        class_file.attributes.push(Attribute::NestMembers {
            name_index: 0,
            class_indexes: vec![utf8_index],
        });

        let result = verify(&class_file);
        assert!(matches!(result, Err(InvalidConstantPoolIndexType(_))));
    }

    #[test]
    fn test_nest_members_duplicate_entries() {
        let mut class_file = ClassFile::default();
        let class_index = class_file.constant_pool.add_class("Inner").unwrap();
        class_file.attributes.push(Attribute::NestMembers {
            name_index: 0,
            class_indexes: vec![class_index, class_index],
        });

        let result = verify(&class_file);
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("Duplicate"));
        }
    }

    #[test]
    fn test_multiple_nest_members_attributes() {
        let mut class_file = ClassFile::default();
        let class_index = class_file.constant_pool.add_class("Inner").unwrap();
        class_file.attributes.push(Attribute::NestMembers {
            name_index: 0,
            class_indexes: vec![class_index],
        });
        class_file.attributes.push(Attribute::NestMembers {
            name_index: 0,
            class_indexes: vec![class_index],
        });

        let result = verify(&class_file);
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("Multiple NestMembers"));
        }
    }

    // Mutual exclusivity tests

    #[test]
    fn test_nest_host_and_nest_members_conflict_host_first() {
        let mut class_file = ClassFile::default();
        let class_index = class_file.constant_pool.add_class("SomeClass").unwrap();
        class_file.attributes.push(Attribute::NestHost {
            name_index: 0,
            host_class_index: class_index,
        });
        class_file.attributes.push(Attribute::NestMembers {
            name_index: 0,
            class_indexes: vec![class_index],
        });

        let result = verify(&class_file);
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("both NestHost and NestMembers"));
        }
    }

    #[test]
    fn test_nest_host_and_nest_members_conflict_members_first() {
        let mut class_file = ClassFile::default();
        let class_index = class_file.constant_pool.add_class("SomeClass").unwrap();
        class_file.attributes.push(Attribute::NestMembers {
            name_index: 0,
            class_indexes: vec![class_index],
        });
        class_file.attributes.push(Attribute::NestHost {
            name_index: 0,
            host_class_index: class_index,
        });

        let result = verify(&class_file);
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("both NestHost and NestMembers"));
        }
    }

    // No nest attributes (valid)

    #[test]
    fn test_no_nest_attributes() {
        let class_file = ClassFile::default();
        assert!(verify(&class_file).is_ok());
    }
}
