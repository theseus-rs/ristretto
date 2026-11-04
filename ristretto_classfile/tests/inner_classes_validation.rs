#![expect(clippy::panic_in_result_fn, reason = "regression assertions")]

use ristretto_classfile::attributes::{Attribute, InnerClass, NestedClassAccessFlags};
use ristretto_classfile::{ClassFile, JAVA_6, JAVA_7, JAVA_25, Result};

fn class(
    name: Option<&str>,
    flags: NestedClassAccessFlags,
    same_index: bool,
) -> Result<ClassFile<'static>> {
    let mut class = ClassFile {
        version: JAVA_25,
        ..ClassFile::default()
    };
    class.this_class = class.constant_pool.add_class("example/Outer")?;
    class.super_class = class.constant_pool.add_class("java/lang/Object")?;
    let inner = class.constant_pool.add_class("example/Outer$Inner")?;
    let name_index = match name {
        Some(name) => class.constant_pool.add_utf8(name)?,
        None => 0,
    };
    class.attributes.push(Attribute::InnerClasses {
        name_index: class.constant_pool.add_utf8("InnerClasses")?,
        classes: vec![InnerClass {
            class_info_index: inner,
            outer_class_info_index: if same_index {
                inner
            } else if name.is_some() {
                class.this_class
            } else {
                0
            },
            name_index,
            access_flags: flags,
        }],
    });
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
fn inner_and_outer_indexes_must_differ() -> Result<()> {
    check(
        &class(Some("Inner"), NestedClassAccessFlags::PUBLIC, false)?,
        true,
    )?;
    check(
        &class(Some("Inner"), NestedClassAccessFlags::PUBLIC, true)?,
        false,
    )
}

#[test]
fn inner_names_follow_the_unqualified_name_grammar() -> Result<()> {
    for (name, valid) in [
        ("Inner", true),
        ("内部", true),
        ("<generated>", true),
        ("", false),
        ("a/b", false),
        ("a.b", false),
        ("a;b", false),
        ("[I", false),
    ] {
        check(
            &class(Some(name), NestedClassAccessFlags::empty(), false)?,
            valid,
        )?;
    }
    check(&class(None, NestedClassAccessFlags::empty(), false)?, true)
}

#[test]
fn named_and_anonymous_classes_validate_defined_flag_combinations() -> Result<()> {
    for name in [Some("Inner"), None] {
        for flags in [
            NestedClassAccessFlags::PUBLIC | NestedClassAccessFlags::PRIVATE,
            NestedClassAccessFlags::PUBLIC | NestedClassAccessFlags::PROTECTED,
            NestedClassAccessFlags::PRIVATE | NestedClassAccessFlags::PROTECTED,
            NestedClassAccessFlags::FINAL | NestedClassAccessFlags::ABSTRACT,
            NestedClassAccessFlags::INTERFACE,
            NestedClassAccessFlags::ANNOTATION,
            NestedClassAccessFlags::INTERFACE
                | NestedClassAccessFlags::ABSTRACT
                | NestedClassAccessFlags::FINAL,
            NestedClassAccessFlags::INTERFACE
                | NestedClassAccessFlags::ABSTRACT
                | NestedClassAccessFlags::ENUM,
        ] {
            check(&class(name, flags, false)?, false)?;
        }
        for flags in [
            NestedClassAccessFlags::empty(),
            NestedClassAccessFlags::PUBLIC
                | NestedClassAccessFlags::STATIC
                | NestedClassAccessFlags::FINAL,
            NestedClassAccessFlags::PRIVATE | NestedClassAccessFlags::ABSTRACT,
            NestedClassAccessFlags::PROTECTED | NestedClassAccessFlags::ENUM,
            NestedClassAccessFlags::INTERFACE | NestedClassAccessFlags::ABSTRACT,
            NestedClassAccessFlags::INTERFACE
                | NestedClassAccessFlags::ABSTRACT
                | NestedClassAccessFlags::ANNOTATION,
            NestedClassAccessFlags::from_bits_retain(0x0080),
        ] {
            check(&class(name, flags, false)?, true)?;
        }
    }
    Ok(())
}

#[test]
fn anonymous_outer_index_rule_keeps_its_java7_boundary() -> Result<()> {
    let mut class = class(Some("Inner"), NestedClassAccessFlags::empty(), false)?;
    if let Some(Attribute::InnerClasses { classes, .. }) = class.attributes.first_mut()
        && let Some(inner) = classes.first_mut()
    {
        inner.name_index = 0;
    }
    class.version = JAVA_6;
    check(&class, true)?;
    class.version = JAVA_7;
    check(&class, false)
}
