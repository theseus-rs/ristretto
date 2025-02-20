use crate::java_object::JavaObject;
use crate::native_methods::registry::{MethodRegistry, JAVA_17, JAVA_21};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/loader/NativeLibraries";

/// Register all native methods for `jdk.internal.loader.NativeLibraries`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() == JAVA_17 {
        registry.register(
            CLASS_NAME,
            "findEntry0",
            "(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;)J",
            find_entry_0,
        );
        registry.register(
            CLASS_NAME,
            "load",
            "(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;ZZZ)Z",
            load,
        );
        registry.register(CLASS_NAME, "unload", "(Ljava/lang/String;ZZJ)V", unload);
    }
    if registry.java_major_version() >= JAVA_21 {
        registry.register(
            CLASS_NAME,
            "load",
            "(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;ZZ)Z",
            load,
        );
        registry.register(CLASS_NAME, "unload", "(Ljava/lang/String;ZJ)V", unload);
    }

    registry.register(
        CLASS_NAME,
        "findBuiltinLib",
        "(Ljava/lang/String;)Ljava/lang/String;",
        find_builtin_lib,
    );
}

#[async_recursion(?Send)]
async fn find_entry_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.loader.NativeLibraries.findEntry0(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn find_builtin_lib(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop_object()?;
    let vm = thread.vm()?;
    let library_file_name: String = object.try_into()?;
    let library_path = vm
        .java_home()
        .join("lib")
        .join(library_file_name)
        .to_string_lossy()
        .to_string();
    let vm = thread.vm()?;
    let library_name = library_path.to_object(&vm).await?;
    Ok(Some(library_name))
}

#[async_recursion(?Send)]
async fn load(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(Some(Value::Int(1)))
}

#[async_recursion(?Send)]
async fn unload(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    let _is_builtin = parameters.pop_bool()?;
    let _name: String = parameters.pop_object()?.try_into()?;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.loader.NativeLibraries.findEntry0(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;)J"
    )]
    async fn test_find_entry_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_entry_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_load() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = load(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_unload() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let name = "foo".to_object(&vm).await?;
        let parameters = Parameters::new(vec![name, Value::Int(1), Value::Long(2)]);
        let result = unload(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
