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
    "sun/misc/MessageUtils.toStderr(Ljava/lang/String;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn to_stderr(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.MessageUtils.toStderr(Ljava/lang/String;)V")
}

#[intrinsic_method(
    "sun/misc/MessageUtils.toStdout(Ljava/lang/String;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn to_stdout(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.MessageUtils.toStdout(Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.MessageUtils.toStderr(Ljava/lang/String;)V"
    )]
    async fn test_to_stderr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = to_stderr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.MessageUtils.toStdout(Ljava/lang/String;)V"
    )]
    async fn test_to_stdout() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = to_stdout(thread, Parameters::default()).await;
    }
}
