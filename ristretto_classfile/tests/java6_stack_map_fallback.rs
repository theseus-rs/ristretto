#![expect(clippy::panic_in_result_fn, reason = "regression assertions")]

use ristretto_classfile::attributes::{Attribute, Instruction, StackFrame, VerificationType};
use ristretto_classfile::verifiers::bytecode::{
    VerificationPath, VerifierConfig, verify_class, verify_method,
};
use ristretto_classfile::verifiers::context::{MemberAccess, VerificationContext};
use ristretto_classfile::verifiers::error;
use ristretto_classfile::{
    ClassFile, JAVA_5, JAVA_6, JAVA_7, JAVA_25, JavaStr, JavaString, Method, MethodAccessFlags,
    Result, Version,
};

struct Context;
impl VerificationContext for Context {
    fn verify_member_access(&self, _access: &MemberAccess<'_>) -> error::Result<()> {
        // This fixture models accessible members; access-policy failures have dedicated cases.
        Ok(())
    }

    fn is_subclass(&self, source: &JavaStr, target: &JavaStr) -> error::Result<bool> {
        self.is_assignable(target, source)
    }

    fn is_assignable(&self, target: &JavaStr, source: &JavaStr) -> error::Result<bool> {
        Ok(source == target || target == "java/lang/Object")
    }

    fn common_superclass(&self, a: &JavaStr, b: &JavaStr) -> error::Result<JavaString> {
        Ok(if a == b {
            a.to_owned()
        } else {
            "java/lang/Object".into()
        })
    }
}

fn full(offset: u16, locals: Vec<VerificationType>, stack: Vec<VerificationType>) -> StackFrame {
    StackFrame::FullFrame {
        frame_type: 255,
        offset_delta: offset,
        locals,
        stack,
    }
}

fn fixture(version: Version, frames: Vec<StackFrame>) -> Result<ClassFile<'static>> {
    use Instruction::{New, Pop, Return, Sipush};
    let mut class = ClassFile {
        version,
        ..ClassFile::default()
    };
    class.this_class = class.constant_pool.add_class("Fallback")?;
    class.super_class = class.constant_pool.add_class("java/lang/Object")?;
    class.methods.push(Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: class.constant_pool.add_utf8("method")?,
        descriptor_index: class.constant_pool.add_utf8("()V")?,
        attributes: vec![Attribute::Code {
            name_index: class.constant_pool.add_utf8("Code")?,
            max_stack: 1,
            max_locals: 0,
            code: vec![Sipush(1), Pop, New(class.super_class), Pop, Return],
            exception_table: vec![],
            attributes: vec![Attribute::StackMapTable {
                name_index: class.constant_pool.add_utf8("StackMapTable")?,
                frames,
            }],
        }],
    });
    Ok(class)
}

fn check(class: &ClassFile<'_>, path: Option<VerificationPath>) -> Result<()> {
    let config = VerifierConfig::strict();
    let result = verify_class(class, &Context, &config);
    assert_eq!(result.is_ok(), path.is_some(), "{result:?}");
    if let (Ok(results), Some(path)) = (result, path) {
        assert_eq!(results.first().map(|r| r.path_used), Some(path));
    }
    for method in &class.methods {
        let result = verify_method(class, method, &Context, &config);
        assert_eq!(result.is_ok(), path.is_some(), "{result:?}");
        if let (Ok(result), Some(path)) = (result, path) {
            assert_eq!(result.path_used, path);
        }
    }
    let mut bytes = Vec::new();
    class.to_bytes(&mut bytes)?;
    match ClassFile::from_bytes(&bytes) {
        Ok(parsed) => {
            let result = verify_class(&parsed, &Context, &config);
            assert_eq!(result.is_ok(), path.is_some(), "{result:?}");
            if let (Ok(results), Some(path)) = (result, path) {
                assert_eq!(results.first().map(|r| r.path_used), Some(path));
            }
        }
        Err(error) => assert!(path.is_none(), "{error}"),
    }
    Ok(())
}

#[test]
fn recoverable_stack_map_failures_fall_back_only_for_java6() -> Result<()> {
    use VerificationType::{Float, Integer, Uninitialized};
    let cases = vec![
        vec![full(1, vec![], vec![])],  // Inside sipush.
        vec![full(99, vec![], vec![])], // Past code_length.
        vec![StackFrame::ChopFrame {
            frame_type: 250,
            offset_delta: 0,
        }],
        vec![full(0, vec![Integer], vec![])], // Exceeds max_locals.
        vec![full(3, vec![], vec![Integer, Integer])], // Exceeds max_stack.
        vec![full(3, vec![], vec![Float])],   // Computed stack holds int.
        vec![full(7, vec![], vec![Uninitialized { offset: 0 }])], // Not new.
        vec![full(7, vec![], vec![Uninitialized { offset: 2 }])], // Inside sipush.
        vec![full(7, vec![], vec![Uninitialized { offset: 99 }])], // Outside code.
        vec![full(0, vec![], vec![]), full(u16::MAX, vec![], vec![])],
    ];
    for frames in cases {
        check(
            &fixture(JAVA_6, frames.clone())?,
            Some(VerificationPath::Inference),
        )?;
        check(&fixture(JAVA_7, frames)?, None)?;
    }
    Ok(())
}

#[test]
fn valid_java6_stack_maps_still_use_type_checking() -> Result<()> {
    let class = fixture(
        JAVA_6,
        vec![full(
            7,
            vec![],
            vec![VerificationType::Uninitialized { offset: 4 }],
        )],
    )?;
    check(&class, Some(VerificationPath::FastPath))
}

#[test]
fn malformed_frame_encodings_and_constant_pool_references_do_not_fall_back() -> Result<()> {
    let class = fixture(JAVA_6, vec![StackFrame::SameFrame { frame_type: 128 }])?;
    assert!(verify_class(&class, &Context, &VerifierConfig::strict()).is_err());
    for index in [1, u16::MAX] {
        // Utf8 entry and missing entry.
        let class = fixture(
            JAVA_6,
            vec![full(
                7,
                vec![],
                vec![VerificationType::Object { cpool_index: index }],
            )],
        )?;
        check(&class, None)?;
    }
    let valid = fixture(JAVA_6, vec![StackFrame::SameFrame { frame_type: 0 }])?;
    let mut bytes = Vec::new();
    valid.to_bytes(&mut bytes)?;
    // A one-byte frame is followed only by the two-byte class attributes_count.
    if let Some(tag) = bytes.iter_mut().rev().nth(2) {
        *tag = 128;
    }
    assert!(ClassFile::from_bytes(&bytes).is_err());
    Ok(())
}

#[test]
fn inference_still_rejects_invalid_bytecode_with_stale_stack_maps() -> Result<()> {
    for code in [
        vec![Instruction::Pop, Instruction::Return],
        vec![Instruction::Goto(1), Instruction::Return],
    ] {
        let mut class = fixture(
            JAVA_6,
            vec![StackFrame::ChopFrame {
                frame_type: 250,
                offset_delta: 0,
            }],
        )?;
        if let Some(method) = class.methods.first_mut()
            && let Some(Attribute::Code {
                code: instructions, ..
            }) = method.attributes.first_mut()
        {
            *instructions = code;
        }
        check(&class, None)?;
    }
    Ok(())
}

#[test]
fn java6_fallback_does_not_relax_other_code_metadata() -> Result<()> {
    let mut class = fixture(JAVA_6, vec![full(1, vec![], vec![])])?;
    let name_index = class.constant_pool.add_utf8("LineNumberTable")?;
    if let Some(method) = class.methods.first_mut()
        && let Some(Attribute::Code { attributes, .. }) = method.attributes.first_mut()
    {
        attributes.push(Attribute::LineNumberTable {
            name_index,
            line_numbers: vec![ristretto_classfile::attributes::LineNumber {
                start_pc: 1, // Inside sipush, not an instruction boundary.
                line_number: 1,
            }],
        });
    }
    assert!(verify_class(&class, &Context, &VerifierConfig::strict()).is_err());
    let mut bytes = Vec::new();
    class.to_bytes(&mut bytes)?;
    assert!(ClassFile::from_bytes(&bytes).is_err());
    Ok(())
}

#[test]
fn configuration_cannot_bypass_versioned_type_checking() -> Result<()> {
    use ristretto_classfile::verifiers::bytecode::FallbackStrategy;
    for version in [JAVA_5, JAVA_6, Version::Java6 { minor: 1 }, JAVA_7, JAVA_25] {
        for strategy in [
            FallbackStrategy::Strict,
            FallbackStrategy::FallbackToInference,
            FallbackStrategy::AlwaysInference,
        ] {
            for threshold in [0, 50, u16::MAX] {
                let config = VerifierConfig {
                    fallback_strategy: strategy,
                    stackmap_required_version: threshold,
                    ..VerifierConfig::strict()
                };
                let mut class = fixture(version, vec![])?;
                if let Some(method) = class.methods.first_mut()
                    && let Some(Attribute::Code { code, .. }) = method.attributes.first_mut()
                {
                    // The second return requires a stack-map frame after the first transfer.
                    *code = vec![Instruction::Return, Instruction::Return];
                }
                let expected = version <= JAVA_6;
                let mut bytes = vec![];
                class.to_bytes(&mut bytes)?;
                let parsed = ClassFile::from_bytes(&bytes)?;
                assert_eq!(
                    verify_class(&parsed, &Context, &config).is_ok(),
                    expected,
                    "{version:?} {config:?}"
                );
                for method in &parsed.methods {
                    assert_eq!(
                        verify_method(&parsed, method, &Context, &config).is_ok(),
                        expected,
                        "{version:?} {config:?}"
                    );
                }
                if let Some(method) = class.methods.first_mut()
                    && let Some(Attribute::Code { code, .. }) = method.attributes.first_mut()
                {
                    *code = vec![Instruction::Return];
                }
                let results = verify_class(&class, &Context, &config)?;
                let expected_path = if version < JAVA_6 {
                    VerificationPath::Inference
                } else {
                    VerificationPath::FastPath
                };
                assert_eq!(results.first().map(|r| r.path_used), Some(expected_path));
            }
        }
    }
    Ok(())
}
