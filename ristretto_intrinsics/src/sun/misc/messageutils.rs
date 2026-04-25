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
    "sun/misc/MessageUtils.toStderr(Ljava/lang/String;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn to_stderr<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.misc.MessageUtils.toStderr(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/misc/MessageUtils.toStdout(Ljava/lang/String;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn to_stdout<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.misc.MessageUtils.toStdout(Ljava/lang/String;)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_to_stderr() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = to_stderr(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.misc.MessageUtils.toStderr(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_to_stdout() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = to_stdout(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.misc.MessageUtils.toStdout(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }
}
