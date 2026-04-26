use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPlatformComponent.nativeCreateComponent(J)J", Any)]
#[async_method]
pub async fn native_create_component<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_layer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformComponent.nativeCreateComponent(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformComponent.nativeSetBounds(JIIII)V", Any)]
#[async_method]
pub async fn native_set_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformComponent.nativeSetBounds(JIIII)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_create_component() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_component(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CPlatformComponent.nativeCreateComponent(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_set_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_bounds(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CPlatformComponent.nativeSetBounds(JIIII)V",
            result.unwrap_err().to_string()
        );
    }
}
