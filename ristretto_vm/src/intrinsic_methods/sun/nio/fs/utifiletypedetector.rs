use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/fs/UTIFileTypeDetector.probe0(Ljava/lang/String;)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn probe_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UTIFileTypeDetector.probe0(Ljava/lang/String;)Ljava/lang/String;");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UTIFileTypeDetector.probe0(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_probe_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = probe_0(thread, Parameters::default()).await;
    }
}
