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

#[async_recursion(?Send)]
async fn native_convert_and_release(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("apple.launcher.JavaAppLauncher.nativeConvertAndRelease(J)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn native_invoke_non_public(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("apple.launcher.JavaAppLauncher.nativeInvokeNonPublic(Ljava/lang/Class;Ljava/lang/reflect/Method;[Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "apple/launcher/JavaAppLauncher";
        assert!(registry
            .method(
                class_name,
                "nativeConvertAndRelease",
                "(J)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nativeInvokeNonPublic",
                "(Ljava/lang/Class;Ljava/lang/reflect/Method;[Ljava/lang/String;)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.launcher.JavaAppLauncher.nativeConvertAndRelease(J)Ljava/lang/Object;"
    )]
    async fn test_native_convert_and_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_convert_and_release(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.launcher.JavaAppLauncher.nativeInvokeNonPublic(Ljava/lang/Class;Ljava/lang/reflect/Method;[Ljava/lang/String;)V"
    )]
    async fn test_native_invoke_non_public() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_invoke_non_public(thread, Arguments::default()).await;
    }
}
