use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/misc/PreviewFeatures.isPreviewEnabled()Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn is_preview_enabled<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;

    let preview_features = vm.preview_features();
    Ok(Some(Value::from(preview_features)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_preview_enabled() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = is_preview_enabled(thread, Parameters::default()).await?;
        assert_eq!(value, Some(Value::from(false)));
        Ok(())
    }
}
