use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WLightweightFramePeer.overrideNativeHandle(J)V", Any)]
#[async_method]
pub async fn override_native_handle<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _hwnd = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WLightweightFramePeer.overrideNativeHandle(J)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_override_native_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = override_native_handle(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WLightweightFramePeer.overrideNativeHandle(J)V",
            result.unwrap_err().to_string()
        );
    }
}
