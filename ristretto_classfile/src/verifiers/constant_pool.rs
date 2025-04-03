use crate::Error::{
    BootstrapMethodsNotDefined, InvalidBootstrapMethodIndex, InvalidVersionConstant,
};
use crate::Error::{InvalidConstantPoolIndex, InvalidConstantPoolIndexType};
use crate::Result;
use crate::attributes::Attribute;
use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::reference_kind::ReferenceKind;
use crate::version::Version;

const JAVA_8: Version = Version::Java8 { minor: 0 };
const JAVA_9: Version = Version::Java9 { minor: 0 };

/// Verify the `ClassFile` `ConstantPool`.
pub fn verify(class_file: &ClassFile) -> Result<()> {
    verify_version_constants(class_file)?;
    verify_constant_indexes(class_file)?;
    Ok(())
}

/// Verify the `ClassFile` `ConstantPool` for version specific constants.
fn verify_version_constants(class_file: &ClassFile) -> Result<()> {
    for constant in &class_file.constant_pool {
        if !constant.valid_for_version(&class_file.version) {
            let tag = constant.tag();
            return Err(InvalidVersionConstant(tag));
        }
    }

    Ok(())
}

/// Verify the `ClassFile` `ConstantPool` indexes.
#[expect(clippy::too_many_lines)]
fn verify_constant_indexes(class_file: &ClassFile) -> Result<()> {
    let constant_pool = &class_file.constant_pool;
    for (index, constant) in constant_pool.iter().enumerate() {
        let index = u16::try_from(index)?;
        match constant {
            Constant::Class(name_index)
            | Constant::Module(name_index)
            | Constant::Package(name_index) => {
                match constant_pool.get(*name_index) {
                    Some(Constant::Utf8 { .. }) => {} // valid index
                    None => return Err(InvalidConstantPoolIndex(index)),
                    _ => return Err(InvalidConstantPoolIndexType(index)),
                }
            }
            Constant::String(string_index) => {
                match constant_pool.get(*string_index) {
                    Some(Constant::Utf8 { .. }) => {} // valid index
                    None => return Err(InvalidConstantPoolIndex(index)),
                    _ => return Err(InvalidConstantPoolIndexType(index)),
                }
            }
            Constant::FieldRef {
                class_index,
                name_and_type_index,
            }
            | Constant::MethodRef {
                class_index,
                name_and_type_index,
            }
            | Constant::InterfaceMethodRef {
                class_index,
                name_and_type_index,
            } => {
                match constant_pool.get(*class_index) {
                    Some(Constant::Class { .. }) => {} // valid index
                    None => return Err(InvalidConstantPoolIndex(index)),
                    _ => return Err(InvalidConstantPoolIndexType(index)),
                }
                match constant_pool.get(*name_and_type_index) {
                    Some(Constant::NameAndType { .. }) => {} // valid index
                    None => return Err(InvalidConstantPoolIndex(index)),
                    _ => return Err(InvalidConstantPoolIndexType(index)),
                }
            }
            Constant::NameAndType {
                name_index,
                descriptor_index,
            } => {
                match constant_pool.get(*name_index) {
                    Some(Constant::Utf8 { .. }) => {} // valid index
                    None => return Err(InvalidConstantPoolIndex(index)),
                    _ => return Err(InvalidConstantPoolIndexType(index)),
                }
                match constant_pool.get(*descriptor_index) {
                    Some(Constant::Utf8 { .. }) => {} // valid index
                    None => return Err(InvalidConstantPoolIndex(index)),
                    _ => return Err(InvalidConstantPoolIndexType(index)),
                }
            }
            Constant::MethodHandle {
                reference_kind,
                reference_index,
            } => match reference_kind {
                ReferenceKind::GetField
                | ReferenceKind::GetStatic
                | ReferenceKind::PutField
                | ReferenceKind::PutStatic => {
                    match constant_pool.get(*reference_index) {
                        Some(Constant::FieldRef { .. }) => {} // valid index
                        None => return Err(InvalidConstantPoolIndex(index)),
                        _ => return Err(InvalidConstantPoolIndexType(index)),
                    }
                }
                ReferenceKind::InvokeVirtual | ReferenceKind::NewInvokeSpecial => {
                    match constant_pool.get(*reference_index) {
                        Some(Constant::MethodRef { .. }) => {} // valid index
                        None => return Err(InvalidConstantPoolIndex(index)),
                        _ => return Err(InvalidConstantPoolIndexType(index)),
                    }
                }
                ReferenceKind::InvokeStatic | ReferenceKind::InvokeSpecial => {
                    match constant_pool.get(*reference_index) {
                        Some(Constant::MethodRef { .. }) => {} // valid index
                        Some(Constant::InterfaceMethodRef { .. })
                            if class_file.version >= JAVA_8 => {} // valid index
                        None => return Err(InvalidConstantPoolIndex(index)),
                        _ => return Err(InvalidConstantPoolIndexType(index)),
                    }
                }
                ReferenceKind::InvokeInterface => {
                    match constant_pool.get(*reference_index) {
                        Some(Constant::InterfaceMethodRef { .. }) => {} // valid index
                        None => return Err(InvalidConstantPoolIndex(index)),
                        _ => return Err(InvalidConstantPoolIndexType(index)),
                    }
                }
            },
            Constant::MethodType(descriptor_index) => {
                match constant_pool.get(*descriptor_index) {
                    Some(Constant::Utf8 { .. }) => {} // valid index
                    None => return Err(InvalidConstantPoolIndex(index)),
                    _ => return Err(InvalidConstantPoolIndexType(index)),
                }
            }
            Constant::Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            }
            | Constant::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                let bootstrap_methods = class_file
                    .attributes
                    .iter()
                    .find(|attribute| matches!(attribute, Attribute::BootstrapMethods { .. }));

                match bootstrap_methods {
                    Some(Attribute::BootstrapMethods { methods, .. }) => {
                        if *bootstrap_method_attr_index as usize >= methods.len() {
                            return Err(InvalidBootstrapMethodIndex(
                                *bootstrap_method_attr_index as usize,
                            ));
                        }
                    }
                    _ => return Err(BootstrapMethodsNotDefined),
                }

                match constant_pool.get(*name_and_type_index) {
                    Some(Constant::NameAndType { .. }) => {} // valid index
                    None => return Err(InvalidConstantPoolIndex(index)),
                    _ => return Err(InvalidConstantPoolIndexType(index)),
                }
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::attributes::BootstrapMethod;
    use crate::class_file::ClassFile;
    use crate::constant::Constant;
    use std::io::Cursor;

    fn get_class_file() -> Result<ClassFile> {
        let class_bytes = include_bytes!("../../../classes/Minimum.class");
        let expected_bytes = class_bytes.to_vec();
        ClassFile::from_bytes(&mut Cursor::new(expected_bytes.clone()))
    }

    fn get_utf8_index(class_file: &mut ClassFile) -> Result<u16> {
        class_file
            .constant_pool
            .push(Constant::Utf8("foo".to_string()));
        Ok(u16::try_from(class_file.constant_pool.len())?)
    }

    fn get_integer_index(class_file: &mut ClassFile) -> Result<u16> {
        class_file.constant_pool.push(Constant::Integer(42));
        Ok(u16::try_from(class_file.constant_pool.len())?)
    }

    fn get_class_index(class_file: &mut ClassFile) -> Result<u16> {
        let utf8_index = get_utf8_index(class_file)?;
        class_file.constant_pool.push(Constant::Class(utf8_index));
        Ok(u16::try_from(class_file.constant_pool.len())?)
    }

    fn get_name_and_type_index(class_file: &mut ClassFile) -> Result<u16> {
        let utf8_index = get_utf8_index(class_file)?;
        class_file.constant_pool.push(Constant::NameAndType {
            name_index: utf8_index,
            descriptor_index: utf8_index,
        });
        Ok(u16::try_from(class_file.constant_pool.len())?)
    }

    fn get_bootstrap_methods_index(class_file: &mut ClassFile) -> Result<u16> {
        let utf8_index = get_utf8_index(class_file)?;
        let bootstrap_method = BootstrapMethod {
            bootstrap_method_ref: 0,
            arguments: vec![],
        };
        class_file.attributes.push(Attribute::BootstrapMethods {
            name_index: utf8_index,
            methods: vec![bootstrap_method],
        });
        Ok(0)
    }

    #[test]
    fn test_verify() -> Result<()> {
        let class_bytes = include_bytes!("../../../classes/Simple.class");
        let expected_bytes = class_bytes.to_vec();
        let class_file = ClassFile::from_bytes(&mut Cursor::new(expected_bytes.clone()))?;

        assert_eq!(Ok(()), verify(&class_file));
        Ok(())
    }

    fn test_version_constants_error(version: Version, constant: Constant) -> Result<()> {
        let mut class_file = get_class_file()?;
        let tag = constant.tag();

        class_file.version = version;
        class_file.constant_pool.push(constant);

        assert_eq!(
            verify_version_constants(&class_file),
            Err(InvalidVersionConstant(tag))
        );
        Ok(())
    }

    #[test]
    fn test_version_constants_method_type() -> Result<()> {
        test_version_constants_error(Version::Java6 { minor: 0 }, Constant::MethodType(1))
    }

    #[test]
    fn test_version_constants_dynamic() -> Result<()> {
        test_version_constants_error(
            Version::Java10 { minor: 0 },
            Constant::Dynamic {
                bootstrap_method_attr_index: 1,
                name_and_type_index: 2,
            },
        )
    }

    #[test]
    fn test_version_constants_invoke_dynamic() -> Result<()> {
        test_version_constants_error(
            Version::Java6 { minor: 0 },
            Constant::InvokeDynamic {
                bootstrap_method_attr_index: 1,
                name_and_type_index: 2,
            },
        )
    }

    #[test]
    fn test_version_constants_module() -> Result<()> {
        test_version_constants_error(Version::Java8 { minor: 0 }, Constant::Module(1))
    }

    #[test]
    fn test_version_constants_package() -> Result<()> {
        test_version_constants_error(Version::Java8 { minor: 0 }, Constant::Package(1))
    }

    fn test_indexes_index_error(mut class_file: ClassFile, constant: Constant) -> Result<()> {
        class_file.constant_pool.push(constant);
        let index = u16::try_from(class_file.constant_pool.len() - 1)?;
        assert_eq!(
            verify_constant_indexes(&class_file),
            Err(InvalidConstantPoolIndex(index))
        );
        Ok(())
    }

    fn test_indexes_index_type_error(mut class_file: ClassFile, constant: Constant) -> Result<()> {
        class_file.constant_pool.push(constant);
        let index = u16::try_from(class_file.constant_pool.len() - 1)?;
        assert_eq!(
            verify_constant_indexes(&class_file),
            Err(InvalidConstantPoolIndexType(index))
        );
        Ok(())
    }

    fn test_indexes_utf8_index_errors(constant: &Constant) -> Result<()> {
        let class_file = get_class_file()?;
        test_indexes_index_error(class_file.clone(), constant.clone())?;
        test_indexes_utf8_index_type_error(constant)
    }

    fn test_indexes_utf8_index_type_error(constant: &Constant) -> Result<()> {
        let class_file = &mut get_class_file()?;
        let integer_index = get_integer_index(class_file)?;

        let constant = match constant {
            Constant::Class { .. } => Constant::Class(integer_index),
            Constant::Module { .. } => Constant::Module(integer_index),
            Constant::Package { .. } => Constant::Package(integer_index),
            Constant::String { .. } => Constant::String(integer_index),
            _ => Constant::MethodType(integer_index),
        };

        test_indexes_index_type_error(class_file.clone(), constant.clone())?;
        Ok(())
    }

    #[test]
    fn test_indexes_class_errors() -> Result<()> {
        test_indexes_utf8_index_errors(&Constant::Class(u16::MAX))
    }

    #[test]
    fn test_indexes_module_errors() -> Result<()> {
        test_indexes_utf8_index_errors(&Constant::Module(u16::MAX))
    }

    #[test]
    fn test_indexes_package_errors() -> Result<()> {
        test_indexes_utf8_index_errors(&Constant::Package(u16::MAX))
    }

    #[test]
    fn test_indexes_string_errors() -> Result<()> {
        test_indexes_utf8_index_errors(&Constant::String(u16::MAX))
    }

    #[test]
    fn test_indexes_method_type_errors() -> Result<()> {
        test_indexes_utf8_index_errors(&Constant::MethodType(u16::MAX))
    }

    fn get_ref_constant(
        constant: &Constant,
        class_index: u16,
        name_and_type_index: u16,
    ) -> Constant {
        match constant {
            Constant::FieldRef { .. } => Constant::FieldRef {
                class_index,
                name_and_type_index,
            },
            Constant::MethodRef { .. } => Constant::MethodRef {
                class_index,
                name_and_type_index,
            },
            _ => Constant::InterfaceMethodRef {
                class_index,
                name_and_type_index,
            },
        }
    }

    fn test_indexes_ref_errors(constant: &Constant) -> Result<()> {
        let class_file = &mut get_class_file()?;
        let utf8_index = get_utf8_index(class_file)?;
        let class_index = get_class_index(class_file)?;
        let name_and_type_index = get_name_and_type_index(class_file)?;

        test_indexes_index_error(
            class_file.clone(),
            get_ref_constant(constant, u16::MAX, name_and_type_index),
        )?;
        test_indexes_index_type_error(
            class_file.clone(),
            get_ref_constant(constant, utf8_index, name_and_type_index),
        )?;
        test_indexes_index_error(
            class_file.clone(),
            get_ref_constant(constant, class_index, u16::MAX),
        )?;
        test_indexes_index_type_error(
            class_file.clone(),
            get_ref_constant(constant, class_index, utf8_index),
        )?;
        Ok(())
    }

    #[test]
    fn test_indexes_field_ref_errors() -> Result<()> {
        test_indexes_ref_errors(&Constant::FieldRef {
            class_index: 0,
            name_and_type_index: 0,
        })
    }

    #[test]
    fn test_indexes_method_ref_errors() -> Result<()> {
        test_indexes_ref_errors(&Constant::MethodRef {
            class_index: 0,
            name_and_type_index: 0,
        })
    }

    #[test]
    fn test_indexes_interface_method_ref_errors() -> Result<()> {
        test_indexes_ref_errors(&Constant::InterfaceMethodRef {
            class_index: 0,
            name_and_type_index: 0,
        })
    }

    #[test]
    fn test_indexes_name_and_type_errors() -> Result<()> {
        let class_file = &mut get_class_file()?;
        let integer_index = get_integer_index(class_file)?;
        let utf8_index = get_utf8_index(class_file)?;

        test_indexes_index_error(
            class_file.clone(),
            Constant::NameAndType {
                name_index: u16::MAX,
                descriptor_index: utf8_index,
            },
        )?;
        test_indexes_index_type_error(
            class_file.clone(),
            Constant::NameAndType {
                name_index: integer_index,
                descriptor_index: utf8_index,
            },
        )?;
        test_indexes_index_error(
            class_file.clone(),
            Constant::NameAndType {
                name_index: utf8_index,
                descriptor_index: u16::MAX,
            },
        )?;
        test_indexes_index_type_error(
            class_file.clone(),
            Constant::NameAndType {
                name_index: utf8_index,
                descriptor_index: integer_index,
            },
        )?;
        Ok(())
    }

    #[test]
    fn test_indexes_method_handle_errors() -> Result<()> {
        let class_file = &mut get_class_file()?;
        let integer_index = get_integer_index(class_file)?;

        for reference_kind in ReferenceKind::all() {
            test_indexes_index_error(
                class_file.clone(),
                Constant::MethodHandle {
                    reference_kind: reference_kind.clone(),
                    reference_index: u16::MAX,
                },
            )?;
            test_indexes_index_type_error(
                class_file.clone(),
                Constant::MethodHandle {
                    reference_kind: reference_kind.clone(),
                    reference_index: integer_index,
                },
            )?;
        }
        Ok(())
    }

    fn test_indexes_no_bootstrap_methods_error(constant: Constant) -> Result<()> {
        let class_file = &mut get_class_file()?;
        class_file.constant_pool.push(constant);
        assert_eq!(
            verify_constant_indexes(class_file),
            Err(BootstrapMethodsNotDefined)
        );
        Ok(())
    }

    fn test_indexes_bootstrap_method_index_error(
        mut class_file: ClassFile,
        constant: Constant,
        index: usize,
    ) {
        class_file.constant_pool.push(constant);
        assert_eq!(
            verify_constant_indexes(&class_file),
            Err(InvalidBootstrapMethodIndex(index))
        );
    }

    #[test]
    fn test_indexes_dynamic_errors() -> Result<()> {
        let class_file = &mut get_class_file()?;
        let bootstrap_method_index = get_bootstrap_methods_index(class_file)?;
        let integer_index = get_integer_index(class_file)?;
        let name_and_type_index = get_name_and_type_index(class_file)?;

        test_indexes_no_bootstrap_methods_error(Constant::Dynamic {
            bootstrap_method_attr_index: 0,
            name_and_type_index,
        })?;
        test_indexes_bootstrap_method_index_error(
            class_file.clone(),
            Constant::Dynamic {
                bootstrap_method_attr_index: u16::MAX,
                name_and_type_index,
            },
            u16::MAX as usize,
        );
        test_indexes_index_error(
            class_file.clone(),
            Constant::Dynamic {
                bootstrap_method_attr_index: bootstrap_method_index,
                name_and_type_index: u16::MAX,
            },
        )?;
        test_indexes_index_type_error(
            class_file.clone(),
            Constant::Dynamic {
                bootstrap_method_attr_index: bootstrap_method_index,
                name_and_type_index: integer_index,
            },
        )?;
        Ok(())
    }

    #[test]
    fn test_indexes_invoke_dynamic_errors() -> Result<()> {
        let class_file = &mut get_class_file()?;
        let bootstrap_method_index = get_bootstrap_methods_index(class_file)?;
        let integer_index = get_integer_index(class_file)?;
        let name_and_type_index = get_name_and_type_index(class_file)?;

        test_indexes_no_bootstrap_methods_error(Constant::InvokeDynamic {
            bootstrap_method_attr_index: 0,
            name_and_type_index,
        })?;
        test_indexes_bootstrap_method_index_error(
            class_file.clone(),
            Constant::InvokeDynamic {
                bootstrap_method_attr_index: u16::MAX,
                name_and_type_index,
            },
            u16::MAX as usize,
        );
        test_indexes_index_error(
            class_file.clone(),
            Constant::Dynamic {
                bootstrap_method_attr_index: bootstrap_method_index,
                name_and_type_index: u16::MAX,
            },
        )?;
        test_indexes_index_type_error(
            class_file.clone(),
            Constant::Dynamic {
                bootstrap_method_attr_index: bootstrap_method_index,
                name_and_type_index: integer_index,
            },
        )?;
        Ok(())
    }
}
