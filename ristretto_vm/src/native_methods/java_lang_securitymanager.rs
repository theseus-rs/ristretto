use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for java.lang.SecurityManager.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/SecurityManager";
    // TODO: this is only required for Java 11
    registry.register(
        class_name,
        "addNonExportedPackages",
        "(Ljava/lang/ModuleLayer;)V",
        add_non_exported_packages,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn add_non_exported_packages(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}
