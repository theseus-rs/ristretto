use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.Package`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Package";
    registry.register(
        class_name,
        "getSystemPackage0",
        "(Ljava/lang/String;)Ljava/lang/String;",
        get_system_package_0,
    );
    registry.register(
        class_name,
        "getSystemPackages0",
        "()[Ljava/lang/String;",
        get_system_packages_0,
    );
}

#[async_recursion(?Send)]
async fn get_system_package_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.Package.getSystemPackage0(Ljava/lang/String;)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_system_packages_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.Package.getSystemPackages0()[Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/Package";
        assert!(registry
            .method(
                class_name,
                "getSystemPackage0",
                "(Ljava/lang/String;)Ljava/lang/String;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "getSystemPackages0", "()[Ljava/lang/String;")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Package.getSystemPackage0(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_get_system_package_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_package_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Package.getSystemPackages0()[Ljava/lang/String;"
    )]
    async fn test_get_system_packages_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_packages_0(thread, Arguments::default()).await;
    }
}
