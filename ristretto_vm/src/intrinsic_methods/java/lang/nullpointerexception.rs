use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/NullPointerException.getExtendedNPEMessage()Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_extended_npe_message(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.NullPointerException.getExtendedNPEMessage()Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.NullPointerException.getExtendedNPEMessage()Ljava/lang/String;"
    )]
    async fn test_get_extended_npe_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_extended_npe_message(thread, Parameters::default()).await;
    }
}
