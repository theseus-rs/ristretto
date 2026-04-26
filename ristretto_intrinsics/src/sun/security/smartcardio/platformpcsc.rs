use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/smartcardio/PlatformPCSC.initialize(Ljava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn initialize<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_lib_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PlatformPCSC.initialize(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.security.smartcardio.PlatformPCSC.initialize(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }
}
