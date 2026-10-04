//! A constructor that throws must leave every saved receiver alias unusable.
#![expect(
    clippy::panic_in_result_fn,
    reason = "regression tests use assertions with fallible fixture setup"
)]
use ristretto_classfile::attributes::{Attribute, ExceptionTableEntry, Instruction};
use ristretto_classfile::verifiers::bytecode::{VerifierConfig, verify_class};
use ristretto_classfile::verifiers::context::VerificationContext;
use ristretto_classfile::verifiers::error::Result as VerifyResult;
use ristretto_classfile::{ClassAccessFlags, ClassFile, JAVA_5, Method, MethodAccessFlags, Result};

struct Context;
impl VerificationContext for Context {
    fn is_subclass(&self, source: &str, target: &str) -> VerifyResult<bool> {
        self.is_assignable(target, source)
    }
    fn is_assignable(&self, target: &str, source: &str) -> VerifyResult<bool> {
        Ok(source == target || target == "java/lang/Object")
    }
    fn common_superclass(&self, a: &str, b: &str) -> VerifyResult<String> {
        Ok(if a == b { a } else { "java/lang/Object" }.to_string())
    }
}

fn constructor_handler(retry_alias: Option<Instruction>) -> Result<ClassFile<'static>> {
    use Instruction::{Astore_0, Astore_1, Dup, Invokespecial, New, Pop, Return};
    let mut class = ClassFile {
        version: JAVA_5,
        access_flags: ClassAccessFlags::PUBLIC,
        ..ClassFile::default()
    };
    class.this_class = class.constant_pool.add_class("ConstructorAliases")?;
    class.super_class = class.constant_pool.add_class("java/lang/Object")?;
    let init = class
        .constant_pool
        .add_method_ref(class.super_class, "<init>", "()V")?;
    let mut code = vec![
        New(class.super_class),
        Dup,
        Astore_0,
        Dup,
        Astore_1,
        Invokespecial(init),
        Return,
        Pop,
    ];
    if let Some(load) = retry_alias {
        code.extend([load, Invokespecial(init)]);
    }
    code.push(Return);
    class.methods.push(Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: class.constant_pool.add_utf8("create")?,
        descriptor_index: class.constant_pool.add_utf8("()V")?,
        attributes: vec![Attribute::Code {
            name_index: class.constant_pool.add_utf8("Code")?,
            max_stack: 2,
            max_locals: 2,
            code,
            exception_table: vec![ExceptionTableEntry {
                range_pc: 5..6,
                handler_pc: 7,
                catch_type: 0,
            }],
            attributes: vec![],
        }],
    });
    Ok(class)
}

#[test]
fn failed_constructor_cannot_retry_first_receiver_alias() -> Result<()> {
    let class = constructor_handler(Some(Instruction::Aload_0))?;
    assert!(verify_class(&class, &Context, &VerifierConfig::strict()).is_err());
    Ok(())
}

#[test]
fn failed_constructor_cannot_retry_second_receiver_alias() -> Result<()> {
    let class = constructor_handler(Some(Instruction::Aload_1))?;
    assert!(verify_class(&class, &Context, &VerifierConfig::strict()).is_err());
    Ok(())
}

#[test]
fn failed_constructor_handler_can_discard_receiver_aliases() -> Result<()> {
    let class = constructor_handler(None)?;
    let result = verify_class(&class, &Context, &VerifierConfig::strict());
    assert!(result.is_ok(), "{result:?}");
    Ok(())
}
