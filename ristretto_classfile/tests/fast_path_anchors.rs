//! Inferred anchors must account for every predecessor and propagate changed types.
#![expect(clippy::panic_in_result_fn, reason = "regression assertions")]
use ristretto_classfile::attributes::{Attribute, Instruction, StackFrame};
use ristretto_classfile::verifiers;
use ristretto_classfile::verifiers::bytecode::{VerifierConfig, verify_class};
use ristretto_classfile::verifiers::context::VerificationContext;
use ristretto_classfile::{ClassFile, JAVA_6, Method, MethodAccessFlags, Result};

struct Context;
impl VerificationContext for Context {
    fn is_subclass(&self, a: &str, b: &str) -> verifiers::Result<bool> {
        Ok(a == b || b == "java/lang/Object")
    }

    fn is_assignable(&self, target: &str, source: &str) -> verifiers::Result<bool> {
        self.is_subclass(source, target)
    }

    fn common_superclass(&self, _: &str, _: &str) -> verifiers::Result<String> {
        Ok("java/lang/Object".to_string())
    }
}

fn fixture(code: Vec<Instruction>) -> Result<ClassFile<'static>> {
    let mut class = ClassFile {
        version: JAVA_6,
        ..ClassFile::default()
    };
    class.this_class = class.constant_pool.add_class("Anchors")?;
    class.super_class = class.constant_pool.add_class("java/lang/Object")?;
    class.methods.push(Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: class.constant_pool.add_utf8("method")?,
        descriptor_index: class.constant_pool.add_utf8("()V")?,
        attributes: vec![Attribute::Code {
            name_index: class.constant_pool.add_utf8("Code")?,
            max_stack: 1,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![Attribute::StackMapTable {
                name_index: class.constant_pool.add_utf8("StackMapTable")?,
                frames: vec![StackFrame::SameFrame { frame_type: 0 }],
            }],
        }],
    });
    Ok(class)
}

#[test]
fn fallthrough_types_are_merged_with_branch_types() -> Result<()> {
    use Instruction::{Aconst_null, Astore_0, Iconst_0, Ifeq, Iload_0, Istore_0, Pop, Return};
    for (value, store, valid) in [(Aconst_null, Astore_0, false), (Iconst_0, Istore_0, true)] {
        let class = fixture(vec![
            Iconst_0,
            Istore_0,
            Iconst_0,
            Ifeq(5),
            value,
            store,
            Iload_0,
            Pop,
            Return,
        ])?;
        let result = verify_class(&class, &Context, &VerifierConfig::permissive());
        assert_eq!(result.is_ok(), valid, "{result:?}");
    }
    Ok(())
}

#[test]
fn backedges_recheck_instructions_after_local_types_change() -> Result<()> {
    use Instruction::{Aconst_null, Astore_0, Iconst_0, Ifeq, Iload_0, Istore_0, Nop, Pop, Return};
    for (value, store, valid) in [(Aconst_null, Astore_0, false), (Iconst_0, Istore_0, true)] {
        let class = fixture(vec![
            Iconst_0,
            Istore_0,
            Iconst_0,
            Ifeq(4),
            Nop,
            Iload_0,
            Pop,
            value,
            store,
            Iconst_0,
            Ifeq(-5),
            Return,
        ])?;
        let result = verify_class(&class, &Context, &VerifierConfig::permissive());
        assert_eq!(result.is_ok(), valid, "{result:?}");
    }
    Ok(())
}
