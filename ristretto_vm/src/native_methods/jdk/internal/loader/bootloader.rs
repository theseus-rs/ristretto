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
