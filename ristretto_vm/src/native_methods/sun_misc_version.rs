use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.misc.Version`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/Version";
    registry.register(
        class_name,
        "getJdkSpecialVersion",
        "()Ljava/lang/String;",
        get_jdk_special_version,
    );
    registry.register(class_name, "getJdkVersionInfo", "()V", get_jdk_version_info);
    registry.register(
        class_name,
        "getJvmSpecialVersion",
        "()Ljava/lang/String;",
        get_jvm_special_version,
    );
    registry.register(class_name, "getJvmVersionInfo", "()Z", get_jvm_version_info);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_jdk_special_version(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_jdk_version_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_jvm_special_version(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_jvm_version_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
