//! Bootstrap tables require invocation handles and loadable constant arguments.
#![expect(
    clippy::panic_in_result_fn,
    reason = "regression tests use assertions with fallible fixture setup"
)]
use ristretto_classfile::attributes::{Attribute, BootstrapMethod};
use ristretto_classfile::{ClassAccessFlags, ClassFile, Constant, JAVA_25, ReferenceKind, Result};

fn bootstrap_class(kind: ReferenceKind, loadable: bool) -> Result<ClassFile<'static>> {
    let mut class = ClassFile {
        version: JAVA_25,
        access_flags: ClassAccessFlags::PUBLIC,
        ..ClassFile::default()
    };
    class.this_class = class.constant_pool.add_class("BootstrapConstraints")?;
    class.super_class = class.constant_pool.add_class("java/lang/Object")?;
    let target = class
        .constant_pool
        .add_method_ref(class.this_class, "bootstrap", "()V")?;
    let handle = class.constant_pool.add(Constant::MethodHandle {
        reference_kind: kind,
        reference_index: target,
    })?;
    let argument = if loadable {
        class.constant_pool.add_integer(42)?
    } else {
        class.constant_pool.add_utf8("not a loadable constant")?
    };
    class.attributes.push(Attribute::BootstrapMethods {
        name_index: class.constant_pool.add_utf8("BootstrapMethods")?,
        methods: vec![BootstrapMethod {
            bootstrap_method_ref: handle,
            arguments: vec![argument],
        }],
    });
    Ok(class)
}

#[test]
fn bootstrap_rejects_virtual_invocation_handle() -> Result<()> {
    let class = bootstrap_class(ReferenceKind::InvokeVirtual, true)?;
    assert!(class.verify().is_err());
    Ok(())
}

#[test]
fn bootstrap_rejects_non_loadable_argument() -> Result<()> {
    let class = bootstrap_class(ReferenceKind::InvokeStatic, false)?;
    assert!(class.verify().is_err());
    Ok(())
}

#[test]
fn bootstrap_accepts_static_handle_and_loadable_argument() -> Result<()> {
    let class = bootstrap_class(ReferenceKind::InvokeStatic, true)?;
    let result = class.verify();
    assert!(result.is_ok(), "{result:?}");
    Ok(())
}
