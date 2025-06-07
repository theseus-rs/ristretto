use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/misc/PreviewFeatures.isPreviewEnabled()Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn is_preview_enabled(
    thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let configuration = vm.configuration();
    let preview_features = configuration.preview_features();
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
