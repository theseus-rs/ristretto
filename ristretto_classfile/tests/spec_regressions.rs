//! Regression cases discovered by differential class-file verification against `OpenJDK`.
#![expect(
    clippy::unwrap_used,
    reason = "small generated class-file test fixtures"
)]
use ristretto_classfile::attributes::*;
use ristretto_classfile::*;

use ristretto_classfile::verifiers::bytecode::{VerifierConfig, verify_class};
use ristretto_classfile::verifiers::context::VerificationContext;
use ristretto_classfile::verifiers::error::Result as VResult;

struct Context;
impl VerificationContext for Context {
    fn is_subclass(&self, source: &str, target: &str) -> VResult<bool> {
        self.is_assignable(target, source)
    }
    fn is_assignable(&self, target: &str, source: &str) -> VResult<bool> {
        Ok(target == source
            || target == "java/lang/Object"
            || (source == "Audit" && target == "Parent"))
    }
    fn common_superclass(&self, a: &str, b: &str) -> VResult<String> {
        Ok(if a == b { a } else { "java/lang/Object" }.to_string())
    }
}
fn base() -> ClassFile<'static> {
    let mut c = ClassFile {
        version: JAVA_25,
        access_flags: ClassAccessFlags::PUBLIC,
        ..Default::default()
    };
    c.this_class = c.constant_pool.add_class("Audit").unwrap();
    c.super_class = c.constant_pool.add_class("java/lang/Object").unwrap();
    c
}
fn method(
    c: &mut ClassFile<'static>,
    name: &str,
    desc: &str,
    stack: u16,
    locals: u16,
    code: Vec<Instruction>,
    frames: Option<Vec<StackFrame>>,
) -> Method {
    let name_index = c.constant_pool.add_utf8(name).unwrap();
    let descriptor_index = c.constant_pool.add_utf8(desc).unwrap();
    let code_name = c.constant_pool.add_utf8("Code").unwrap();
    let attributes = frames
        .map(|frames| {
            vec![Attribute::StackMapTable {
                name_index: c.constant_pool.add_utf8("StackMapTable").unwrap(),
                frames,
            }]
        })
        .unwrap_or_default();
    Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index,
        descriptor_index,
        attributes: vec![Attribute::Code {
            name_index: code_name,
            max_stack: stack,
            max_locals: locals,
            code,
            exception_table: vec![],
            attributes,
        }],
    }
}

use Instruction::*;

fn check(mut class: ClassFile<'static>, method: Option<Method>, valid: bool) {
    if let Some(method) = method {
        class.methods.push(method);
    }
    let result = verify_class(&class, &Context, &VerifierConfig::strict());
    assert_eq!(result.is_ok(), valid, "{result:?}");
    if valid {
        let mut bytes = Vec::new();
        class.to_bytes(&mut bytes).unwrap();
        let parsed = ClassFile::from_bytes(&bytes).unwrap();
        verify_class(&parsed, &Context, &VerifierConfig::strict()).unwrap();
        let mut encoded = Vec::new();
        parsed.to_bytes(&mut encoded).unwrap();
        assert_eq!(bytes, encoded);
    }
}
#[test]
fn baseline() {
    check(base(), None, true);
}

#[test]
fn stack_underflow() {
    let mut c = base();
    let m = method(&mut c, "f", "()V", 0, 0, vec![Pop, Return], None);
    check(c, Some(m), false);
}

#[test]
fn valid_branch_stackmap() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "()V",
        0,
        0,
        vec![Goto(1), Return],
        Some(vec![StackFrame::SameFrame { frame_type: 1 }]),
    );
    check(c, Some(m), true);
}

#[test]
fn large_logical_branch_sizing() {
    let mut c = base();
    let mut code = vec![Iinc(0, 0); 18000];
    code.push(Goto(18000));
    let m = method(
        &mut c,
        "f",
        "(I)V",
        0,
        1,
        code,
        Some(vec![StackFrame::SameFrameExtended {
            frame_type: 251,
            offset_delta: 18000,
        }]),
    );
    check(c, Some(m), true);
}
