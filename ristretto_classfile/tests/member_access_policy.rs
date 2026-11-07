//! Every member-access opcode consults the context's explicit access policy.
#![expect(clippy::panic_in_result_fn, reason = "regression assertions")]

use ristretto_classfile::attributes::{Attribute, Instruction};
use ristretto_classfile::verifiers::bytecode::{VerifierConfig, verify_class, verify_method};
use ristretto_classfile::verifiers::context::{MemberAccess, VerificationContext};
use ristretto_classfile::verifiers::error::{self, VerifyError};
use ristretto_classfile::{
    ClassFile, Constant, JAVA_5, JAVA_25, Method, MethodAccessFlags, Result, Version,
};
use ristretto_classfile::{JavaStr, JavaString};
use std::cell::Cell;

struct Policy {
    allow: bool,
    calls: Cell<usize>,
    instruction: Instruction,
    receiver: Option<&'static str>,
    descriptor: &'static str,
}

impl VerificationContext for Policy {
    fn is_subclass(&self, source: &JavaStr, target: &JavaStr) -> error::Result<bool> {
        self.is_assignable(target, source)
    }

    fn is_assignable(&self, target: &JavaStr, source: &JavaStr) -> error::Result<bool> {
        Ok(target == source || target == "java/lang/Object")
    }

    fn common_superclass(&self, a: &JavaStr, b: &JavaStr) -> error::Result<JavaString> {
        Ok(if a == b {
            a.to_owned()
        } else {
            "java/lang/Object".into()
        })
    }

    fn verify_member_access(&self, access: &MemberAccess<'_>) -> error::Result<()> {
        self.calls.set(self.calls.get() + 1);
        assert_eq!(access.current_class, "AccessFixture");
        assert_eq!(access.owner, "AccessFixture");
        assert_eq!(access.name, "member");
        assert_eq!(access.descriptor, self.descriptor);
        assert_eq!(
            access.receiver.map(|receiver| receiver.to_str_lossy()),
            self.receiver.map(std::borrow::Cow::Borrowed)
        );
        assert_eq!(access.instruction, &self.instruction);
        if self.allow {
            Ok(())
        } else {
            Err(VerifyError::IllegalAccessError(
                "member denied by context".into(),
            ))
        }
    }
}

fn fixture(version: Version, opcode: u8) -> Result<(ClassFile<'static>, Policy)> {
    use Instruction::*;
    let mut class = ClassFile {
        version,
        ..Default::default()
    };
    class.this_class = class.constant_pool.add_class("AccessFixture")?;
    class.super_class = class.constant_pool.add_class("java/lang/Object")?;
    let field = opcode < 4;
    let descriptor = if field { "J" } else { "()V" };
    let name_index = class.constant_pool.add_utf8("member")?;
    let descriptor_index = class.constant_pool.add_utf8(descriptor)?;
    let name_and_type_index = class.constant_pool.add(Constant::NameAndType {
        name_index,
        descriptor_index,
    })?;
    let reference = class.constant_pool.add(if field {
        Constant::FieldRef {
            class_index: class.this_class,
            name_and_type_index,
        }
    } else if opcode == 7 {
        Constant::InterfaceMethodRef {
            class_index: class.this_class,
            name_and_type_index,
        }
    } else {
        Constant::MethodRef {
            class_index: class.this_class,
            name_and_type_index,
        }
    })?;
    let instruction = match opcode {
        0 => Getfield(reference),
        1 => Putfield(reference),
        2 => Getstatic(reference),
        3 => Putstatic(reference),
        4 => Invokevirtual(reference),
        5 => Invokespecial(reference),
        6 => Invokestatic(reference),
        _ => Invokeinterface(reference, 1),
    };
    let instance = matches!(opcode, 0 | 1 | 4 | 5 | 7);
    let mut code = vec![];
    if instance {
        code.push(Aload_0);
    }
    if matches!(opcode, 1 | 3) {
        code.push(Lconst_0);
    }
    code.push(instruction.clone());
    if matches!(opcode, 0 | 2) {
        code.push(Pop2);
    }
    code.push(Return);
    class.methods.push(Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: class.constant_pool.add_utf8("f")?,
        descriptor_index: class.constant_pool.add_utf8("(LAccessFixture;)V")?,
        attributes: vec![Attribute::Code {
            name_index: class.constant_pool.add_utf8("Code")?,
            max_stack: 3,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![],
        }],
    });
    Ok((
        class,
        Policy {
            allow: true,
            calls: Cell::new(0),
            instruction,
            descriptor,
            receiver: instance.then_some("LAccessFixture;"),
        },
    ))
}

#[test]
fn member_access_policy_is_enforced_by_both_verifiers() -> Result<()> {
    for version in [JAVA_5, JAVA_25] {
        for opcode in 0..8 {
            let (class, mut policy) = fixture(version, opcode)?;
            let config = VerifierConfig::strict();
            verify_class(&class, &policy, &config)?;
            for method in &class.methods {
                verify_method(&class, method, &policy, &config)?;
            }
            assert_eq!(policy.calls.get(), 2);
            policy.allow = false;
            assert_eq!(
                verify_class(&class, &policy, &config).unwrap_err(),
                VerifyError::IllegalAccessError("member denied by context".into())
            );
            for method in &class.methods {
                assert_eq!(
                    verify_method(&class, method, &policy, &config).unwrap_err(),
                    VerifyError::IllegalAccessError("member denied by context".into())
                );
            }
            assert_eq!(policy.calls.get(), 4);
        }
    }
    Ok(())
}
