use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `apple.launcher.JavaAppLauncher`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "apple/launcher/JavaAppLauncher";
    registry.register(
        class_name,
        "nativeConvertAndRelease",
        "(J)Ljava/lang/Object;",
        native_convert_and_release,
    );
    registry.register(
        class_name,
        "nativeInvokeNonPublic",
        "(Ljava/lang/Class;Ljava/lang/reflect/Method;[Ljava/lang/String;)V",
        native_invoke_non_public,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_convert_and_release(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_invoke_non_public(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
