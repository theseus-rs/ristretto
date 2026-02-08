use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/lang/StringCoding.err(Ljava/lang/String;)V", Equal(JAVA_11))]
#[async_method]
pub async fn err<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.StringCoding.err(Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StringCoding.err(Ljava/lang/String;)V"
    )]
    async fn test_err() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = err(thread, Parameters::default()).await;
    }
}
