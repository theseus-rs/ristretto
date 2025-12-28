//! PermittedSubclasses attribute verification for sealed classes according to [JVMS §4.7.31](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.31).
//!
//! This module validates the PermittedSubclasses attribute which is used to implement
//! sealed classes feature introduced in Java 17.

use crate::attributes::Attribute;
use crate::class_access_flags::ClassAccessFlags;
use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::verifiers::error::Result;
use crate::verifiers::error::VerifyError::{
    InvalidConstantPoolIndex, InvalidConstantPoolIndexType, VerificationError,
};

/// Verify `PermittedSubclasses` attributes for a class.
///
/// According to [JVMS §4.7.31](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.31):
/// - The `PermittedSubclasses` attribute may appear at most once in a class.
/// - A class with `PermittedSubclasses` must not be final.
/// - Each entry in `class_indexes` must be a valid `CONSTANT_Class_info`.
/// - No duplicate entries are allowed.
///
/// # Errors
/// Returns an error if the `PermittedSubclasses` attribute is invalid.
pub(crate) fn verify(class_file: &ClassFile) -> Result<()> {
    let mut has_permitted_subclasses = false;

    for attribute in &class_file.attributes {
        if let Attribute::PermittedSubclasses { class_indexes, .. } = attribute {
            if has_permitted_subclasses {
                return Err(VerificationError {
                    context: "PermittedSubclasses".to_string(),
                    message: "Multiple PermittedSubclasses attributes are not allowed".to_string(),
                });
            }
            has_permitted_subclasses = true;

            // A sealed class cannot be final
            if class_file.access_flags.contains(ClassAccessFlags::FINAL) {
                return Err(VerificationError {
                    context: "PermittedSubclasses".to_string(),
                    message: "A sealed class (with PermittedSubclasses) cannot be final"
                        .to_string(),
                });
            }

            // Verify each class index
            let mut seen_indexes = std::collections::HashSet::new();
            for (i, &class_index) in class_indexes.iter().enumerate() {
                // Verify index points to a valid CONSTANT_Class_info
                match class_file.constant_pool.get(class_index) {
                    Some(Constant::Class(_)) => {}
                    Some(_) => return Err(InvalidConstantPoolIndexType(class_index)),
                    None => return Err(InvalidConstantPoolIndex(class_index)),
                }

                // Check for duplicates
                if !seen_indexes.insert(class_index) {
                    return Err(VerificationError {
                        context: "PermittedSubclasses".to_string(),
                        message: format!(
                            "Duplicate class index {class_index} at position {i} in PermittedSubclasses"
                        ),
                    });
                }
            }

            // PermittedSubclasses must have at least one entry
            if class_indexes.is_empty() {
                return Err(VerificationError {
                    context: "PermittedSubclasses".to_string(),
                    message: "PermittedSubclasses must have at least one permitted subclass"
                        .to_string(),
                });
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_class_file_with_permitted_subclasses(
        class_indexes: Vec<u16>,
        is_final: bool,
    ) -> ClassFile {
        let mut class_file = ClassFile::default();
        if is_final {
            class_file.access_flags = ClassAccessFlags::FINAL;
        }
        class_file.attributes.push(Attribute::PermittedSubclasses {
            name_index: 0,
            class_indexes,
        });
        class_file
    }

    #[test]
    fn test_valid_permitted_subclasses() {
        let mut class_file = ClassFile::default();
        let class1 = class_file.constant_pool.add_class("Subclass1").unwrap();
        let class2 = class_file.constant_pool.add_class("Subclass2").unwrap();
        class_file.attributes.push(Attribute::PermittedSubclasses {
            name_index: 0,
            class_indexes: vec![class1, class2],
        });

        assert!(verify(&class_file).is_ok());
    }

    #[test]
    fn test_permitted_subclasses_single_entry() {
        let mut class_file = ClassFile::default();
        let class_index = class_file.constant_pool.add_class("OnlySubclass").unwrap();
        class_file.attributes.push(Attribute::PermittedSubclasses {
            name_index: 0,
            class_indexes: vec![class_index],
        });

        assert!(verify(&class_file).is_ok());
    }

    #[test]
    fn test_permitted_subclasses_empty() {
        let class_file = create_class_file_with_permitted_subclasses(vec![], false);

        let result = verify(&class_file);
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("at least one"));
        }
    }

    #[test]
    fn test_permitted_subclasses_invalid_index() {
        let class_file = create_class_file_with_permitted_subclasses(vec![9999], false);

        let result = verify(&class_file);
        assert!(matches!(result, Err(InvalidConstantPoolIndex(9999))));
    }

    #[test]
    fn test_permitted_subclasses_wrong_index_type() {
        let mut class_file = ClassFile::default();
        let utf8_index = class_file.constant_pool.add_utf8("NotAClass").unwrap();
        class_file.attributes.push(Attribute::PermittedSubclasses {
            name_index: 0,
            class_indexes: vec![utf8_index],
        });

        let result = verify(&class_file);
        assert!(matches!(result, Err(InvalidConstantPoolIndexType(_))));
    }

    #[test]
    fn test_permitted_subclasses_duplicate_entries() {
        let mut class_file = ClassFile::default();
        let class_index = class_file.constant_pool.add_class("Subclass").unwrap();
        class_file.attributes.push(Attribute::PermittedSubclasses {
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
    fn test_permitted_subclasses_final_class() {
        let mut class_file = ClassFile {
            access_flags: ClassAccessFlags::FINAL,
            ..Default::default()
        };
        let class_index = class_file.constant_pool.add_class("Subclass").unwrap();
        class_file.attributes.push(Attribute::PermittedSubclasses {
            name_index: 0,
            class_indexes: vec![class_index],
        });

        let result = verify(&class_file);
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("cannot be final"));
        }
    }

    #[test]
    fn test_multiple_permitted_subclasses_attributes() {
        let mut class_file = ClassFile::default();
        let class_index = class_file.constant_pool.add_class("Subclass").unwrap();
        class_file.attributes.push(Attribute::PermittedSubclasses {
            name_index: 0,
            class_indexes: vec![class_index],
        });
        class_file.attributes.push(Attribute::PermittedSubclasses {
            name_index: 0,
            class_indexes: vec![class_index],
        });

        let result = verify(&class_file);
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("Multiple PermittedSubclasses"));
        }
    }

    #[test]
    fn test_no_permitted_subclasses_attribute() {
        let class_file = ClassFile::default();
        assert!(verify(&class_file).is_ok());
    }

    #[test]
    fn test_permitted_subclasses_interface() {
        // Interfaces can also be sealed
        let mut class_file = ClassFile {
            access_flags: ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT,
            ..Default::default()
        };
        let class_index = class_file
            .constant_pool
            .add_class("Implementation")
            .unwrap();
        class_file.attributes.push(Attribute::PermittedSubclasses {
            name_index: 0,
            class_indexes: vec![class_index],
        });

        assert!(verify(&class_file).is_ok());
    }

    #[test]
    fn test_permitted_subclasses_many_entries() {
        let mut class_file = ClassFile::default();
        let mut class_indexes = Vec::new();
        for i in 0..10 {
            let class_index = class_file
                .constant_pool
                .add_class(format!("Subclass{i}"))
                .unwrap();
            class_indexes.push(class_index);
        }
        class_file.attributes.push(Attribute::PermittedSubclasses {
            name_index: 0,
            class_indexes,
        });

        assert!(verify(&class_file).is_ok());
    }
}
