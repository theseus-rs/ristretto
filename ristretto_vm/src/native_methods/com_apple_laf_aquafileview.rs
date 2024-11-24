use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.apple.laf.AquaFileView`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/apple/laf/AquaFileView";
    registry.register(
        class_name,
        "getNativeDisplayName",
        "([BZ)Ljava/lang/String;",
        get_native_display_name,
    );
    registry.register(class_name, "getNativeLSInfo", "([BZ)I", get_native_ls_info);
    registry.register(
        class_name,
        "getNativeMachineName",
        "()Ljava/lang/String;",
        get_native_machine_name,
    );
    registry.register(
        class_name,
        "getNativePathForResolvedAlias",
        "([BZ)Ljava/lang/String;",
        get_native_path_for_resolved_alias,
    );
    registry.register(
        class_name,
        "getNativePathToSharedJDKBundle",
        "()Ljava/lang/String;",
        get_native_path_to_shared_jdk_bundle,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_native_display_name(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_native_ls_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_native_machine_name(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_native_path_for_resolved_alias(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_native_path_to_shared_jdk_bundle(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
