use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/provider/NativeSeedGenerator.nativeGenerateSeed([B)Z",
    Any
)]
#[async_method]
pub async fn native_generate_seed<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _rand_array = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/provider/NativeSeedGenerator.nativeGenerateSeed([B)Z".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_native_generate_seed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_generate_seed(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/security/provider/NativeSeedGenerator.nativeGenerateSeed([B)Z",
            result.unwrap_err().to_string()
        );
    }
}
