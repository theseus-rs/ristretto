use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "apple/launcher/JavaAppLauncher.nativeConvertAndRelease(J)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_convert_and_release<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.launcher.JavaAppLauncher.nativeConvertAndRelease(J)Ljava/lang/Object;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "apple/launcher/JavaAppLauncher.nativeInvokeNonPublic(Ljava/lang/Class;Ljava/lang/reflect/Method;[Ljava/lang/String;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_invoke_non_public<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_reference()?;
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("apple.launcher.JavaAppLauncher.nativeInvokeNonPublic(Ljava/lang/Class;Ljava/lang/reflect/Method;[Ljava/lang/String;)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_convert_and_release() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            native_convert_and_release(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "apple.launcher.JavaAppLauncher.nativeConvertAndRelease(J)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_invoke_non_public() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_invoke_non_public(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "apple.launcher.JavaAppLauncher.nativeInvokeNonPublic(Ljava/lang/Class;Ljava/lang/reflect/Method;[Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }
}
