use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/awt/KeyboardFocusManager.initIDs()V", LessThanOrEqual(JAVA_21))]
pub fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await?;
        let result = init_ids(thread, Parameters::default())?;
        assert_eq!(result, None);
        Ok(())
    }
}
