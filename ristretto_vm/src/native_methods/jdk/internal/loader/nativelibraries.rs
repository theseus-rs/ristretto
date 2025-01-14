use crate::arguments::Arguments;
use crate::java_object::JavaObject;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.loader.NativeLibraries`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/loader/NativeLibraries";
    registry.register(
        class_name,
        "findBuiltinLib",
        "(Ljava/lang/String;)Ljava/lang/String;",
        find_builtin_lib,
    );
    registry.register(
        class_name,
        "load",
        "(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;ZZ)Z",
        load,
    );
    registry.register(class_name, "unload", "(Ljava/lang/String;ZJ)V", unload);
}

#[async_recursion(?Send)]
async fn find_builtin_lib(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
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
async fn load(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(Some(Value::Int(1)))
}

#[async_recursion(?Send)]
async fn unload(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _handle = arguments.pop_long()?;
    let _is_builtin = arguments.pop_int()? != 0;
    let _name: String = arguments.pop_object()?.try_into()?;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "jdk/internal/loader/NativeLibraries";
        assert!(registry
            .method(
                class_name,
                "findBuiltinLib",
                "(Ljava/lang/String;)Ljava/lang/String;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "load",
                "(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;ZZ)Z"
            )
            .is_some());
        assert!(registry
            .method(class_name, "unload", "(Ljava/lang/String;ZJ)V")
            .is_some());
    }

    #[tokio::test]
    async fn test_load() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = load(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_unload() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let name = "foo".to_object(&vm).await?;
        let arguments = Arguments::new(vec![name, Value::Int(1), Value::Long(2)]);
        let result = unload(thread, arguments).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
