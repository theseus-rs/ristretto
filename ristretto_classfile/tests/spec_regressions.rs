//! Regression cases discovered by differential class-file verification against `OpenJDK`.
#![expect(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "small generated class-file test fixtures"
)]
use ristretto_classfile::attributes::*;
use ristretto_classfile::*;

use ristretto_classfile::verifiers::bytecode::{VerifierConfig, verify_class};
use ristretto_classfile::verifiers::context::VerificationContext;
use ristretto_classfile::verifiers::error::{Result as VResult, VerifyError};

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
fn full(
    offset_delta: u16,
    locals: Vec<VerificationType>,
    stack: Vec<VerificationType>,
) -> StackFrame {
    StackFrame::FullFrame {
        frame_type: 255,
        offset_delta,
        locals,
        stack,
    }
}

use Instruction::*;
use VerificationType as T;

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
fn empty_code_is_rejected() {
    let mut c = base();
    let m = method(&mut c, "f", "()V", 0, 0, vec![], None);
    c.methods.push(m);
    assert!(matches!(
        c.verify(),
        Err(Error::VerificationError(VerifyError::VerificationError { context, message }))
            if context == "Code" && message == "Code must not be empty"
    ));
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
fn oversized_frame_truncated() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "()V",
        0,
        0,
        vec![Return],
        Some(vec![full(0, vec![T::Integer], vec![T::Integer])]),
    );
    check(c, Some(m), false);
}

#[test]
fn valid_long_stackmap() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "()V",
        2,
        0,
        vec![Lconst_0, Nop, Pop2, Return],
        Some(vec![StackFrame::SameLocals1StackItemFrame {
            frame_type: 65,
            stack: vec![T::Long],
        }]),
    );
    check(c, Some(m), true);
}

#[test]
fn valid_append_frame() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "()V",
        1,
        1,
        vec![Iconst_0, Istore_0, Nop, Iload_0, Pop, Return],
        Some(vec![StackFrame::AppendFrame {
            frame_type: 252,
            offset_delta: 2,
            locals: vec![T::Integer],
        }]),
    );
    check(c, Some(m), true);
}

#[test]
fn stackmap_overflow_discarded() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "()V",
        0,
        0,
        vec![Nop, Return],
        Some(vec![full(1, vec![], vec![T::Integer])]),
    );
    check(c, Some(m), false);
}

#[test]
fn valid_handler_byte_index_mismatch() {
    let mut c = base();
    let mut m = method(
        &mut c,
        "f",
        "()V",
        1,
        0,
        vec![Bipush(0), Pop, Return, Pop, Return],
        Some(vec![full(3, vec![], vec![T::Object { cpool_index: 4 }])]),
    );
    if let Attribute::Code {
        exception_table, ..
    } = &mut m.attributes[0]
    {
        exception_table.push(ExceptionTableEntry {
            range_pc: 0..2,
            handler_pc: 3,
            catch_type: 0,
        });
    }
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

#[test]
fn chop_frame_removes_a_category_two_local() {
    let mut class = base();
    let method = method(
        &mut class,
        "f",
        "(IJ)V",
        1,
        3,
        vec![Nop, Iload_0, Pop, Return],
        Some(vec![StackFrame::ChopFrame {
            frame_type: 250,
            offset_delta: 1,
        }]),
    );
    check(class, Some(method), true);
}
