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
    todo!()
}

#[async_recursion(?Send)]
async fn get_system_packages_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
