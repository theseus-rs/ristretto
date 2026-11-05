//! Java names are modified UTF-8 identities, including distinct lone surrogates.
#![expect(clippy::panic_in_result_fn, reason = "regression assertions")]

use ristretto_classfile::attributes::{Attribute, Instruction};
use ristretto_classfile::verifiers::bytecode::{VerifierConfig, verify_class};
use ristretto_classfile::verifiers::context::{MemberAccess, VerificationContext};
use ristretto_classfile::verifiers::error;
use ristretto_classfile::{
    ClassAccessFlags, ClassFile, Constant, JAVA_5, JAVA_25, JavaStr, JavaString, Method,
    MethodAccessFlags, Result, Version,
};
use std::borrow::Cow;
use std::cell::RefCell;

const FIRST: &[u8] = b"A\xed\xa0\x80";
const SECOND: &[u8] = b"A\xed\xa0\x81";

#[derive(Default)]
struct Context {
    merges: RefCell<Vec<(JavaString, JavaString)>>,
    accesses: RefCell<Vec<(JavaString, JavaString, JavaString, JavaString)>>,
}

impl VerificationContext for Context {
    fn is_subclass(&self, source: &JavaStr, target: &JavaStr) -> error::Result<bool> {
        self.is_assignable(target, source)
    }

    fn is_assignable(&self, target: &JavaStr, source: &JavaStr) -> error::Result<bool> {
        Ok(target == source || target == "java/lang/Object")
    }

    fn common_superclass(&self, a: &JavaStr, b: &JavaStr) -> error::Result<JavaString> {
        self.merges.borrow_mut().push((a.to_owned(), b.to_owned()));
        Ok(if a == b {
            a.to_owned()
        } else {
            "java/lang/Object".into()
        })
    }

    fn verify_member_access(&self, access: &MemberAccess<'_>) -> error::Result<()> {
        self.accesses.borrow_mut().push((
            access.current_class.to_owned(),
            access.owner.to_owned(),
            access.name.to_owned(),
            access.descriptor.to_owned(),
        ));
        Ok(())
    }
}

fn utf8(class: &mut ClassFile<'static>, bytes: &[u8]) -> Result<u16> {
    class
        .constant_pool
        .add(Constant::Utf8(Cow::Owned(JavaString::from_mutf8(
            bytes.to_vec(),
        )?)))
}

fn class_constant(class: &mut ClassFile<'static>, bytes: &[u8]) -> Result<u16> {
    let name = utf8(class, bytes)?;
    class.constant_pool.add(Constant::Class(name))
}

fn base(version: Version) -> Result<ClassFile<'static>> {
    let mut class = ClassFile {
        version,
        access_flags: ClassAccessFlags::PUBLIC,
        ..Default::default()
    };
    class.this_class = class_constant(&mut class, FIRST)?;
    class.super_class = class.constant_pool.add_class("java/lang/Object")?;
    Ok(class)
}

fn method(
    class: &mut ClassFile<'static>,
    descriptor: &[u8],
    code: Vec<Instruction>,
    locals: u16,
) -> Result<()> {
    let name_index = class.constant_pool.add_utf8("f")?;
    let descriptor_index = utf8(class, descriptor)?;
    let code_name = class.constant_pool.add_utf8("Code")?;
    class.methods.push(Method {
        name_index,
        descriptor_index,
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        attributes: vec![Attribute::Code {
            name_index: code_name,
            max_stack: 2,
            max_locals: locals,
            code,
            exception_table: vec![],
            attributes: vec![],
        }],
    });
    Ok(())
}

fn check(class: &ClassFile<'_>, valid: bool) -> Result<Context> {
    let mut bytes = vec![];
    class.to_bytes(&mut bytes)?;
    let parsed = ClassFile::from_slice(&bytes)?;
    parsed.verify()?;
    let context = Context::default();
    for config in [VerifierConfig::strict(), VerifierConfig::permissive()] {
        let result = verify_class(&parsed, &context, &config);
        assert_eq!(result.is_ok(), valid, "{result:?}");
    }
    Ok(context)
}

#[test]
fn return_types_preserve_java_names() -> Result<()> {
    for version in [JAVA_5, JAVA_25] {
        for name in [
            FIRST,
            SECOND,
            b"A\xed\xb0\x80",
            b"A\xef\xbf\xbd",
            b"A\xc0\x80",
            b"A\xed\xa0\xbd\xed\xb8\x80",
        ] {
            let mut class = base(version)?;
            let desc = [b"(L", name, b";)L", name, b";"].concat();
            method(
                &mut class,
                &desc,
                vec![Instruction::Aload_0, Instruction::Areturn],
                1,
            )?;
            check(&class, true)?;
            let descriptor_index = utf8(&mut class, &[b"(L", name, b";)L", FIRST, b";"].concat())?;
            class
                .methods
                .first_mut()
                .ok_or_else(|| ristretto_classfile::Error::IoError("missing method".into()))?
                .descriptor_index = descriptor_index;
            check(&class, name == FIRST)?;
        }
    }
    Ok(())
}

#[test]
fn casts_and_arrays_preserve_target_identity() -> Result<()> {
    for version in [JAVA_5, JAVA_25] {
        for array in [false, true] {
            for target in [FIRST, SECOND] {
                let mut class = base(version)?;
                let prefix: &[u8] = if array { b"[L" } else { b"L" };
                let descriptor = [b"()", prefix, FIRST, b";"].concat();
                let target_name = if array {
                    [b"[L", target, b";"].concat()
                } else {
                    target.to_vec()
                };
                let target_index = class_constant(&mut class, &target_name)?;
                method(
                    &mut class,
                    &descriptor,
                    vec![
                        Instruction::Aconst_null,
                        Instruction::Checkcast(target_index),
                        Instruction::Areturn,
                    ],
                    0,
                )?;
                check(&class, target == FIRST)?;
            }
        }
    }
    Ok(())
}

#[test]
fn member_names_and_descriptors_reach_the_context_losslessly() -> Result<()> {
    for version in [JAVA_5, JAVA_25] {
        for argument in [FIRST, SECOND] {
            let mut class = base(version)?;
            let owner = class_constant(&mut class, SECOND)?;
            let name_index = utf8(&mut class, FIRST)?;
            let descriptor = [b"(L", FIRST, b";)L", FIRST, b";"].concat();
            let descriptor_index = utf8(&mut class, &descriptor)?;
            let name_and_type_index = class.constant_pool.add(Constant::NameAndType {
                name_index,
                descriptor_index,
            })?;
            let reference = class.constant_pool.add(Constant::MethodRef {
                class_index: owner,
                name_and_type_index,
            })?;
            method(
                &mut class,
                &[b"(L", argument, b";)L", FIRST, b";"].concat(),
                vec![
                    Instruction::Aload_0,
                    Instruction::Invokestatic(reference),
                    Instruction::Areturn,
                ],
                1,
            )?;
            let context = check(&class, argument == FIRST)?;
            let accesses = context.accesses.into_inner();
            assert_eq!(accesses.len(), 2);
            for (current, owner, name, actual_descriptor) in accesses {
                assert_eq!(current.as_bytes(), FIRST);
                assert_eq!(owner.as_bytes(), SECOND);
                assert_eq!(name.as_bytes(), FIRST);
                assert_eq!(actual_descriptor.as_bytes(), descriptor);
            }
        }
    }
    Ok(())
}

#[test]
fn constructor_owner_matches_the_exact_allocation_type() -> Result<()> {
    for version in [JAVA_5, JAVA_25] {
        for owner in [FIRST, SECOND] {
            let mut class = base(version)?;
            let allocated = class_constant(&mut class, FIRST)?;
            let owner_index = class_constant(&mut class, owner)?;
            let name_index = class.constant_pool.add_utf8("<init>")?;
            let descriptor_index = class.constant_pool.add_utf8("()V")?;
            let name_and_type_index = class.constant_pool.add(Constant::NameAndType {
                name_index,
                descriptor_index,
            })?;
            let reference = class.constant_pool.add(Constant::MethodRef {
                class_index: owner_index,
                name_and_type_index,
            })?;
            method(
                &mut class,
                &[b"()L", FIRST, b";"].concat(),
                vec![
                    Instruction::New(allocated),
                    Instruction::Dup,
                    Instruction::Invokespecial(reference),
                    Instruction::Areturn,
                ],
                0,
            )?;
            check(&class, owner == FIRST)?;
        }
    }
    Ok(())
}

#[test]
fn inference_merges_preserve_both_class_names() -> Result<()> {
    let mut class = base(JAVA_5)?;
    method(
        &mut class,
        &[b"(L", FIRST, b";L", SECOND, b";I)Ljava/lang/Object;"].concat(),
        vec![
            Instruction::Iload_2,
            Instruction::Ifeq(7),
            Instruction::Aload_0,
            Instruction::Goto(4),
            Instruction::Aload_1,
            Instruction::Areturn,
        ],
        3,
    )?;
    let context = check(&class, true)?;
    let merges = context.merges.into_inner();
    assert!(
        merges
            .iter()
            .any(|(a, b)| (a.as_bytes() == FIRST && b.as_bytes() == SECOND)
                || (a.as_bytes() == SECOND && b.as_bytes() == FIRST))
    );
    Ok(())
}
