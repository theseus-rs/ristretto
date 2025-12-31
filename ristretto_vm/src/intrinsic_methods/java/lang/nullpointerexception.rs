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
    // TODO: implement extended NPE messages
    // Return null to indicate no extended message is available. The JDK will fall back to the
    // standard NPE message.
    Ok(Some(Value::Object(None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_extended_npe_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_extended_npe_message(thread, Parameters::default()).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(matches!(value, Some(Value::Object(None))));
    }
}
