#![expect(clippy::panic_in_result_fn, reason = "regression assertions")]

use ristretto_classfile::attributes::{Attribute, AttributeKind, ModuleAccessFlags};
use ristretto_classfile::{ClassAccessFlags, ClassFile, JAVA_9, JAVA_25, Result};

fn module() -> Result<ClassFile<'static>> {
    let mut class = ClassFile {
        version: JAVA_25,
        access_flags: ClassAccessFlags::MODULE,
        ..ClassFile::default()
    };
    class.this_class = class.constant_pool.add_class("module-info")?;
    class.attributes.push(Attribute::Module {
        name_index: class.constant_pool.add_utf8(AttributeKind::Module.name())?,
        module: Box::new(ristretto_classfile::attributes::ModuleAttribute {
            module_name_index: class.constant_pool.add_module("java.base")?,
            flags: ModuleAccessFlags::empty(),
            version_index: 0,
            requires: vec![],
            exports: vec![],
            opens: vec![],
            uses: vec![],
            provides: vec![],
        }),
    });
    Ok(class)
}

#[test]
fn forbidden_module_attributes_are_rejected_even_as_raw_attributes() -> Result<()> {
    for kind in [
        AttributeKind::ConstantValue,
        AttributeKind::Code,
        AttributeKind::StackMapTable,
        AttributeKind::Exceptions,
        AttributeKind::EnclosingMethod,
        AttributeKind::Synthetic,
        AttributeKind::Signature,
        AttributeKind::LineNumberTable,
        AttributeKind::LocalVariableTable,
        AttributeKind::LocalVariableTypeTable,
        AttributeKind::Deprecated,
        AttributeKind::RuntimeVisibleParameterAnnotations,
        AttributeKind::RuntimeInvisibleParameterAnnotations,
        AttributeKind::RuntimeVisibleTypeAnnotations,
        AttributeKind::RuntimeInvisibleTypeAnnotations,
        AttributeKind::AnnotationDefault,
        AttributeKind::BootstrapMethods,
        AttributeKind::MethodParameters,
        AttributeKind::NestHost,
        AttributeKind::NestMembers,
        AttributeKind::Record,
        AttributeKind::PermittedSubclasses,
    ] {
        let name = kind.name();
        let mut class = module()?;
        class.attributes.push(Attribute::Unknown {
            name_index: class.constant_pool.add_utf8(name)?,
            info: vec![],
        });
        assert!(class.verify().is_err(), "{name}");
    }
    Ok(())
}

#[test]
fn parsed_module_cannot_hide_a_predefined_attribute_in_the_wrong_location() -> Result<()> {
    for kind in [
        AttributeKind::Deprecated,
        AttributeKind::Synthetic,
        AttributeKind::Code,
        AttributeKind::ConstantValue,
    ] {
        let name = kind.name();
        let mut class = module()?;
        class.attributes.push(Attribute::Unknown {
            name_index: class.constant_pool.add_utf8(name)?,
            info: vec![],
        });
        let mut bytes = Vec::new();
        class.to_bytes(&mut bytes)?;
        let parsed = ClassFile::from_bytes(&bytes)?;
        assert!(parsed.verify().is_err(), "{name}");
    }
    Ok(())
}

#[test]
fn parsed_module_cannot_hide_a_predefined_attribute_from_a_newer_version() -> Result<()> {
    for kind in [
        AttributeKind::NestHost,
        AttributeKind::NestMembers,
        AttributeKind::Record,
        AttributeKind::PermittedSubclasses,
    ] {
        let mut class = module()?;
        class.version = JAVA_9;
        class.attributes.push(Attribute::Unknown {
            name_index: class.constant_pool.add_utf8(kind.name())?,
            info: vec![],
        });
        let mut bytes = Vec::new();
        class.to_bytes(&mut bytes)?;
        let parsed = ClassFile::from_bytes(&bytes)?;
        assert!(matches!(
            parsed.attributes.last(),
            Some(Attribute::Unknown { .. })
        ));
        assert!(parsed.verify().is_err(), "{kind:?}");
    }
    Ok(())
}

#[test]
fn vendor_attribute_names_are_preserved_without_matching_predefined_kinds() -> Result<()> {
    for name in [
        "VendorExtension",
        "Unknown",
        "code",
        "CodeExtension",
        "Code\0",
    ] {
        let mut class = module()?;
        let attribute = Attribute::Unknown {
            name_index: class.constant_pool.add_utf8(name)?,
            info: vec![1, 2, 3],
        };
        class.attributes.push(attribute.clone());
        class.verify()?;
        let mut bytes = Vec::new();
        class.to_bytes(&mut bytes)?;
        let parsed = ClassFile::from_bytes(&bytes)?;
        parsed.verify()?;
        assert_eq!(parsed.attributes.last(), Some(&attribute), "{name:?}");
    }
    Ok(())
}

#[test]
fn allowed_module_attributes_and_vendor_extensions_remain_valid() -> Result<()> {
    let mut class = module()?;
    class.attributes.extend([
        Attribute::ModulePackages {
            name_index: class
                .constant_pool
                .add_utf8(AttributeKind::ModulePackages.name())?,
            package_indexes: vec![],
        },
        Attribute::ModuleMainClass {
            name_index: class
                .constant_pool
                .add_utf8(AttributeKind::ModuleMainClass.name())?,
            main_class_index: class.constant_pool.add_class("example/Main")?,
        },
        Attribute::InnerClasses {
            name_index: class
                .constant_pool
                .add_utf8(AttributeKind::InnerClasses.name())?,
            classes: vec![],
        },
        Attribute::SourceFile {
            name_index: class
                .constant_pool
                .add_utf8(AttributeKind::SourceFile.name())?,
            source_file_index: class.constant_pool.add_utf8("module-info.java")?,
        },
        Attribute::SourceDebugExtension {
            name_index: class
                .constant_pool
                .add_utf8(AttributeKind::SourceDebugExtension.name())?,
            debug_extension: String::new(),
        },
        Attribute::RuntimeVisibleAnnotations {
            name_index: class
                .constant_pool
                .add_utf8(AttributeKind::RuntimeVisibleAnnotations.name())?,
            annotations: vec![],
        },
        Attribute::RuntimeInvisibleAnnotations {
            name_index: class
                .constant_pool
                .add_utf8(AttributeKind::RuntimeInvisibleAnnotations.name())?,
            annotations: vec![],
        },
        Attribute::Unknown {
            name_index: class.constant_pool.add_utf8("VendorExtension")?,
            info: vec![1, 2, 3],
        },
    ]);
    class.verify()?;
    let mut bytes = Vec::new();
    class.to_bytes(&mut bytes)?;
    ClassFile::from_bytes(&bytes)?.verify()?;
    Ok(())
}
