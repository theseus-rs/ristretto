use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/fs/RegistryFileTypeDetector.queryStringValue(JJ)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn query_string_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _name_address = parameters.pop_long()?;
    let _key_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/RegistryFileTypeDetector.queryStringValue(JJ)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_query_string_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = query_string_value(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/RegistryFileTypeDetector.queryStringValue(JJ)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }
}
