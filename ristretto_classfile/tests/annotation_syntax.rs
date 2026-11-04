#![expect(clippy::panic_in_result_fn, reason = "regression assertions")]

use ristretto_classfile::attributes::{
    Annotation, AnnotationElement, AnnotationValuePair, Attribute, TargetType, TypeAnnotation,
};
use ristretto_classfile::{ClassFile, JAVA_25, Result};

fn annotated(kind: &str, text: &str) -> Result<ClassFile<'static>> {
    let mut class = ClassFile {
        version: JAVA_25,
        ..ClassFile::default()
    };
    class.this_class = class.constant_pool.add_class("example/Owner")?;
    class.super_class = class.constant_pool.add_class("java/lang/Object")?;
    let type_index = class
        .constant_pool
        .add_utf8(if kind.ends_with("annotation_type") {
            text
        } else {
            "Lexample/Annotation;"
        })?;
    let elements = vec![
        AnnotationValuePair {
            name_index: class
                .constant_pool
                .add_utf8(if kind.ends_with("element_name") {
                    text
                } else {
                    "value"
                })?,
            value: AnnotationElement::Enum {
                type_name_index: class.constant_pool.add_utf8(if kind == "enum_type" {
                    text
                } else {
                    "Lexample/Enum;"
                })?,
                const_name_index: class.constant_pool.add_utf8(if kind == "enum_name" {
                    text
                } else {
                    "VALUE"
                })?,
            },
        },
        AnnotationValuePair {
            name_index: class.constant_pool.add_utf8("classValue")?,
            value: AnnotationElement::Class {
                class_info_index: class.constant_pool.add_utf8(if kind == "class_literal" {
                    text
                } else {
                    "Ljava/lang/Object;"
                })?,
            },
        },
    ];
    let attribute = if kind.starts_with("type_") {
        Attribute::RuntimeVisibleTypeAnnotations {
            name_index: class
                .constant_pool
                .add_utf8("RuntimeVisibleTypeAnnotations")?,
            type_annotations: vec![TypeAnnotation {
                target_type: TargetType::SuperType {
                    target_type: 0x10,
                    supertype_index: u16::MAX,
                },
                type_path: vec![],
                type_index,
                elements,
            }],
        }
    } else {
        Attribute::RuntimeVisibleAnnotations {
            name_index: class.constant_pool.add_utf8("RuntimeVisibleAnnotations")?,
            annotations: vec![Annotation {
                type_index,
                elements,
            }],
        }
    };
    class.attributes.push(attribute);
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
fn annotation_and_enum_types_require_complete_field_descriptors() -> Result<()> {
    for kind in ["annotation_type", "type_annotation_type", "enum_type"] {
        for invalid in [
            "",
            "example/Type",
            "Lexample/Type",
            "L;",
            "Lbad.name;",
            "[V",
            "()V",
            "V",
            "Lexample/Type;extra",
        ] {
            check(&annotated(kind, invalid)?, false)?;
        }
        check(&annotated(kind, "Lexample/Type;")?, true)?;
    }
    Ok(())
}

#[test]
fn element_names_use_method_rules_and_enum_constants_use_field_rules() -> Result<()> {
    for kind in ["element_name", "type_element_name", "enum_name"] {
        for invalid in ["", "a/b", "a.b", "a;b", "a[b"] {
            check(&annotated(kind, invalid)?, false)?;
        }
        for valid in ["value", "$generated", "値"] {
            check(&annotated(kind, valid)?, true)?;
        }
        check(&annotated(kind, "<generated>")?, kind == "enum_name")?;
    }
    Ok(())
}

#[test]
fn class_literals_use_return_descriptors_including_void_and_arrays() -> Result<()> {
    for valid in [
        "V",
        "B",
        "C",
        "D",
        "F",
        "I",
        "J",
        "S",
        "Z",
        "Ljava/lang/String;",
        "[I",
        "[[Ljava/lang/String;",
    ] {
        check(&annotated("class_literal", valid)?, true)?;
    }
    for invalid in [
        "",
        "void",
        "java/lang/String",
        "()V",
        "[V",
        "Vextra",
        "Ljava/lang/String;extra",
    ] {
        check(&annotated("class_literal", invalid)?, false)?;
    }
    Ok(())
}

#[test]
fn nested_annotations_and_array_values_cannot_hide_invalid_descriptors() -> Result<()> {
    let mut class = annotated("annotation_type", "Lexample/Annotation;")?;
    let invalid_type = class.constant_pool.add_utf8("bad/Type")?;
    if let Some(Attribute::RuntimeVisibleAnnotations { annotations, .. }) =
        class.attributes.first_mut()
        && let Some(annotation) = annotations.first_mut()
        && let Some(pair) = annotation.elements.first_mut()
    {
        pair.value = AnnotationElement::Array {
            values: vec![AnnotationElement::Annotation {
                annotation: Annotation {
                    type_index: invalid_type,
                    elements: vec![],
                },
            }],
        };
    }
    check(&class, false)
}
