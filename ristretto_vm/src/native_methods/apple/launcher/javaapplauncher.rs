use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "apple/launcher/JavaAppLauncher";

/// Register all native methods for `apple.launcher.JavaAppLauncher`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeConvertAndRelease",
        "(J)Ljava/lang/Object;",
        native_convert_and_release,
    );
    registry.register(
        CLASS_NAME,
        "nativeInvokeNonPublic",
        "(Ljava/lang/Class;Ljava/lang/reflect/Method;[Ljava/lang/String;)V",
        native_invoke_non_public,
    );
}

#[async_recursion(?Send)]
async fn native_convert_and_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.launcher.JavaAppLauncher.nativeConvertAndRelease(J)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn native_invoke_non_public(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.launcher.JavaAppLauncher.nativeInvokeNonPublic(Ljava/lang/Class;Ljava/lang/reflect/Method;[Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.launcher.JavaAppLauncher.nativeConvertAndRelease(J)Ljava/lang/Object;"
    )]
    async fn test_native_convert_and_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_convert_and_release(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.launcher.JavaAppLauncher.nativeInvokeNonPublic(Ljava/lang/Class;Ljava/lang/reflect/Method;[Ljava/lang/String;)V"
    )]
    async fn test_native_invoke_non_public() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_invoke_non_public(thread, Parameters::default()).await;
    }
}
