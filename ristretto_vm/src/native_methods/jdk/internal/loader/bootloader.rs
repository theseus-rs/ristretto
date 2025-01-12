use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.loader.BootLoader`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/loader/BootLoader";
    registry.register(
        class_name,
        "getSystemPackageLocation",
        "(Ljava/lang/String;)Ljava/lang/String;",
        get_system_package_location,
    );
    registry.register(
        class_name,
        "getSystemPackageNames",
        "()[Ljava/lang/String;",
        get_system_package_names,
    );
    registry.register(
        class_name,
        "setBootLoaderUnnamedModule0",
        "(Ljava/lang/Module;)V",
        set_boot_loader_unnamed_module_0,
    );
}

#[async_recursion(?Send)]
async fn get_system_package_location(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.loader.BootLoader.getSystemPackageLocation(Ljava/lang/String;)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_system_package_names(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.loader.BootLoader.getSystemPackageNames()[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn set_boot_loader_unnamed_module_0(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let _object = arguments.pop_reference()?;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "jdk/internal/loader/BootLoader";
        assert!(registry
            .method(
                class_name,
                "getSystemPackageLocation",
                "(Ljava/lang/String;)Ljava/lang/String;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "getSystemPackageNames", "()[Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "setBootLoaderUnnamedModule0",
                "(Ljava/lang/Module;)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "jdk.internal.loader.BootLoader.getSystemPackageLocation(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_get_system_package_location() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_package_location(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "jdk.internal.loader.BootLoader.getSystemPackageNames()[Ljava/lang/String;"
    )]
    async fn test_get_system_package_names() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_package_names(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_set_boot_loader_unnamed_module_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Object(None)]);
        let result = set_boot_loader_unnamed_module_0(thread, arguments).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
