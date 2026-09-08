use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "apple/applescript/AppleScriptEngineFactory.initNative()V",
    LessThanOrEqual(JAVA_8)
)]
pub fn init_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_native() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init_native(thread, Parameters::default())?;
        assert_eq!(None, result);
        Ok(())
    }
}
