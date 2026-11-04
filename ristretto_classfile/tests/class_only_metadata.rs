#![expect(clippy::panic_in_result_fn, reason = "regression assertions")]

use ristretto_classfile::attributes::{
    Attribute, InnerClass, ModuleAccessFlags, NestedClassAccessFlags, Provides,
};
use ristretto_classfile::{
    ClassAccessFlags, ClassFile, JAVA_25, Method, MethodAccessFlags, Result,
};

fn class() -> Result<ClassFile<'static>> {
    let mut class = ClassFile {
        version: JAVA_25,
        ..ClassFile::default()
    };
    class.this_class = class.constant_pool.add_class("example/Owner")?;
    class.super_class = class.constant_pool.add_class("java/lang/Object")?;
    Ok(class)
}

fn check(class: &ClassFile<'_>, valid: bool) -> Result<()> {
    assert_eq!(class.verify().is_ok(), valid);
    let mut bytes = Vec::new();
    class.to_bytes(&mut bytes)?;
    assert_eq!(ClassFile::from_bytes(&bytes)?.verify().is_ok(), valid);
    Ok(())
}

#[test]
fn class_metadata_requires_non_array_names() -> Result<()> {
    for kind in [
        "Exceptions",
        "InnerClasses",
        "OuterClass",
        "EnclosingMethod",
        "NestHost",
        "NestMembers",
        "PermittedSubclasses",
    ] {
        for name in ["example/Type", "[I", "[[Lexample/Type;"] {
            let mut class = class()?;
            let index = class.constant_pool.add_class(name)?;
            let name_index = class.constant_pool.add_utf8(if kind == "OuterClass" {
                "InnerClasses"
            } else {
                kind
            })?;
            let attribute = match kind {
                "Exceptions" => Attribute::Exceptions {
                    name_index,
                    exception_indexes: vec![index],
                },
                "InnerClasses" | "OuterClass" => Attribute::InnerClasses {
                    name_index,
                    classes: vec![InnerClass {
                        class_info_index: if kind == "InnerClasses" {
                            index
                        } else {
                            class.this_class
                        },
                        outer_class_info_index: if kind == "OuterClass" { index } else { 0 },
                        name_index: class.constant_pool.add_utf8("Type")?,
                        access_flags: NestedClassAccessFlags::PUBLIC,
                    }],
                },
                "EnclosingMethod" => Attribute::EnclosingMethod {
                    name_index,
                    class_index: index,
                    method_index: 0,
                },
                "NestHost" => Attribute::NestHost {
                    name_index,
                    host_class_index: index,
                },
                "NestMembers" => Attribute::NestMembers {
                    name_index,
                    class_indexes: vec![index],
                },
                _ => Attribute::PermittedSubclasses {
                    name_index,
                    class_indexes: vec![index],
                },
            };
            if kind == "Exceptions" {
                class.methods.push(Method {
                    access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::NATIVE,
                    name_index: class.constant_pool.add_utf8("method")?,
                    descriptor_index: class.constant_pool.add_utf8("()V")?,
                    attributes: vec![attribute],
                });
            } else {
                class.attributes.push(attribute);
            }
            check(&class, !name.starts_with('['))?;
        }
    }
    Ok(())
}

#[test]
fn module_service_and_main_class_names_cannot_be_arrays() -> Result<()> {
    for kind in ["uses", "provides", "with", "main"] {
        for name in ["example/Type", "[I", "[Lexample/Type;"] {
            let mut class = class()?;
            class.access_flags = ClassAccessFlags::MODULE;
            class.this_class = class.constant_pool.add_class("module-info")?;
            class.super_class = 0;
            let index = class.constant_pool.add_class(name)?;
            let service = class.constant_pool.add_class("example/Service")?;
            class.attributes.push(Attribute::Module {
                name_index: class.constant_pool.add_utf8("Module")?,
                module_name_index: class.constant_pool.add_module("java.base")?,
                flags: ModuleAccessFlags::empty(),
                version_index: 0,
                requires: vec![],
                exports: vec![],
                opens: vec![],
                uses: if kind == "uses" { vec![index] } else { vec![] },
                provides: match kind {
                    "provides" => vec![Provides {
                        index,
                        with_index: vec![service],
                    }],
                    "with" => vec![Provides {
                        index: service,
                        with_index: vec![index],
                    }],
                    _ => vec![],
                },
            });
            if kind == "main" {
                class.attributes.push(Attribute::ModuleMainClass {
                    name_index: class.constant_pool.add_utf8("ModuleMainClass")?,
                    main_class_index: index,
                });
            }
            check(&class, !name.starts_with('['))?;
        }
    }
    Ok(())
}

#[test]
fn array_constants_and_member_reference_owners_remain_valid() -> Result<()> {
    let mut class = class()?;
    let array = class.constant_pool.add_class("[I")?;
    class.constant_pool.add_field_ref(array, "field", "I")?;
    class
        .constant_pool
        .add_method_ref(array, "clone", "()Ljava/lang/Object;")?;
    class
        .constant_pool
        .add_interface_method_ref(array, "method", "()V")?;
    check(&class, true)
}
