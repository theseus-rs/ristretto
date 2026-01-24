use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "apple/launcher/JavaAppLauncher.nativeConvertAndRelease(J)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_convert_and_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.launcher.JavaAppLauncher.nativeConvertAndRelease(J)Ljava/lang/Object;")
}

#[intrinsic_method(
    "apple/launcher/JavaAppLauncher.nativeInvokeNonPublic(Ljava/lang/Class;Ljava/lang/reflect/Method;[Ljava/lang/String;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_invoke_non_public(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "apple.launcher.JavaAppLauncher.nativeInvokeNonPublic(Ljava/lang/Class;Ljava/lang/reflect/Method;[Ljava/lang/String;)V"
    )
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
