use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPrinterJobDialog.showDialog()Z", Any)]
#[async_method]
pub async fn show_dialog<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPrinterJobDialog.showDialog()Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_show_dialog() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = show_dialog(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
