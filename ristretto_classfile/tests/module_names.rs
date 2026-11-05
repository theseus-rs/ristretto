#![expect(clippy::panic_in_result_fn, reason = "regression assertions")]

use ristretto_classfile::attributes::{Attribute, ModuleAccessFlags, Requires, RequiresFlags};
use ristretto_classfile::{ClassAccessFlags, ClassFile, JAVA_25, Result};

fn module(name: &str) -> Result<ClassFile<'static>> {
    let mut class = ClassFile {
        version: JAVA_25,
        access_flags: ClassAccessFlags::MODULE,
        ..ClassFile::default()
    };
    class.this_class = class.constant_pool.add_class("module-info")?;
    class.attributes.push(Attribute::Module {
        name_index: class.constant_pool.add_utf8("Module")?,
        module: Box::new(ristretto_classfile::attributes::ModuleAttribute {
            module_name_index: class.constant_pool.add_module(name)?,
            flags: ModuleAccessFlags::empty(),
            version_index: 0,
            requires: vec![Requires {
                index: class.constant_pool.add_module("java.base")?,
                flags: RequiresFlags::MANDATED,
                version_index: 0,
            }],
            exports: vec![],
            opens: vec![],
            uses: vec![],
            provides: vec![],
        }),
    });
    Ok(class)
}

#[test]
fn module_names_are_nonempty_before_and_after_serialization() -> Result<()> {
    for name in ["", "example.module", "escaped\\:module", "escaped\\@module"] {
        let class = module(name)?;
        assert_eq!(class.verify().is_ok(), !name.is_empty(), "{name:?}");
        let mut bytes = Vec::new();
        class.to_bytes(&mut bytes)?;
        assert_eq!(
            ClassFile::from_bytes(&bytes)?.verify().is_ok(),
            !name.is_empty(),
            "{name:?}"
        );
    }
    Ok(())
}

#[test]
fn unused_module_constants_also_require_a_name() -> Result<()> {
    let mut class = module("example.module")?;
    class.verify()?;
    class.constant_pool.add_module("")?;
    assert!(class.verify().is_err());
    Ok(())
}
