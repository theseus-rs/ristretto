use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/Package";

/// Register all intrinsic methods for `java.lang.Package`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getSystemPackage0",
        "(Ljava/lang/String;)Ljava/lang/String;",
        get_system_package_0,
    );
    registry.register(
        CLASS_NAME,
        "getSystemPackages0",
        "()[Ljava/lang/String;",
        get_system_packages_0,
    );
}

#[async_recursion(?Send)]
async fn get_system_package_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Package.getSystemPackage0(Ljava/lang/String;)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_system_packages_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Package.getSystemPackages0()[Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Package.getSystemPackage0(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_get_system_package_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_package_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Package.getSystemPackages0()[Ljava/lang/String;"
    )]
    async fn test_get_system_packages_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_packages_0(thread, Parameters::default()).await;
    }
}
