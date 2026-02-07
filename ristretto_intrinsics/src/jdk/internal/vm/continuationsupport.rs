use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/vm/ContinuationSupport.isSupported0()Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn is_supported_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_supported_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = is_supported_0(thread, Parameters::default())
            .await?
            .expect("continuations support");
        let supports_continuations = value.as_bool()?;
        assert!(!supports_continuations);
        Ok(())
    }
}
