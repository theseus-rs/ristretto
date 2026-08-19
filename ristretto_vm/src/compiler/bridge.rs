use super::{CompiledClasses, CompilerError, JavaSource};
use crate::{Class, Error, VM, Value};
use ristretto_classfile::ClassFile;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::sync::Arc;

pub(super) const BRIDGE_CLASS: &str = "org.ristretto.compiler.CompilerBridge";
pub(super) const BRIDGE_COMPILE_METHOD: &str = concat!(
    "compile([Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/String;)",
    "[Ljava/lang/Object;"
);

const BRIDGE_CLASSES: &[&[u8]] = &[
    include_bytes!("classes/org/ristretto/compiler/CompilerBridge.class"),
    include_bytes!("classes/org/ristretto/compiler/CompilerBridge$MemoryFileManager.class"),
    include_bytes!("classes/org/ristretto/compiler/CompilerBridge$MemoryOutput.class"),
    include_bytes!("classes/org/ristretto/compiler/CompilerBridge$Source.class"),
];

pub(super) fn validate_memory_arguments<O>(
    sources: &[JavaSource],
    options: &[O],
) -> Result<(), CompilerError>
where
    O: AsRef<OsStr>,
{
    if sources.is_empty()
        || sources
            .iter()
            .any(|source| source.class_name.trim().is_empty())
    {
        return Err(CompilerError::InvalidArguments);
    }

    if options.iter().any(|option| {
        let option = option.as_ref();
        option == "-d" || option == "-s" || option == "-h"
    }) {
        return Err(CompilerError::InvalidArguments);
    }
    Ok(())
}

pub(super) fn parse_bridge_result(result: &Value) -> Result<CompiledClasses, CompilerError> {
    let (_, values) = result.as_class_vec_ref().map_err(Error::from)?;
    let Some(status) = values.first() else {
        return Err(Error::InternalError("compiler bridge returned no status".to_string()).into());
    };
    let status = status.as_i32().map_err(Error::from)?;
    CompilerError::from_exit_code(status)?;
    if values.len() % 2 == 0 {
        return Err(
            Error::InternalError("compiler bridge returned malformed output".to_string()).into(),
        );
    }

    let entries = values.get(1..).ok_or_else(|| {
        Error::InternalError("compiler bridge returned malformed output".to_string())
    })?;
    let mut classes = BTreeMap::new();
    for entry in entries.chunks_exact(2) {
        let [class_name, bytecode] = entry else {
            return Err(Error::InternalError(
                "compiler bridge returned malformed output".to_string(),
            )
            .into());
        };
        let class_name = class_name.as_string().map_err(Error::from)?;
        let bytecode = bytecode
            .as_byte_vec_ref()
            .map_err(Error::from)?
            .iter()
            .map(|byte| u8::from_ne_bytes(byte.to_ne_bytes()))
            .collect();
        classes.insert(class_name, bytecode);
    }
    Ok(CompiledClasses(classes))
}

pub(super) async fn register_bridge_classes(vm: &Arc<VM>) -> Result<(), Error> {
    let class_loader_lock = vm.class_loader();
    let class_loader = class_loader_lock.read().await;
    for bytecode in BRIDGE_CLASSES {
        let class_file = ClassFile::from_bytes(bytecode)?;
        let class =
            Class::from(Some(Arc::downgrade(&class_loader)), class_file).map_err(Error::from)?;
        class_loader.register(class).await.map_err(Error::from)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_bridge_classes_are_valid() {
        for bytecode in BRIDGE_CLASSES {
            let class_file = ClassFile::from_bytes(bytecode).expect("valid compiler bridge class");
            assert!(
                class_file
                    .class_name()
                    .expect("compiler bridge class name")
                    .to_rust_string()
                    .starts_with("org/ristretto/compiler/CompilerBridge")
            );
        }
    }

    #[test]
    fn validate_memory_output_options() {
        let sources = [JavaSource::new("Test", "class Test {}")];
        assert!(validate_memory_arguments(&sources, &[] as &[&str]).is_ok());
        for option in ["-d", "-s", "-h"] {
            let result = validate_memory_arguments(&sources, &[option]);
            assert!(matches!(result, Err(CompilerError::InvalidArguments)));
        }
    }
}
