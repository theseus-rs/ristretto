use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WCustomCursor.createCursorIndirect([I[BIIIII)V", Any)]
#[async_method]
pub async fn create_cursor_indirect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y_hot_spot = parameters.pop_int()?;
    let _x_hot_spot = parameters.pop_int()?;
    let _n_h = parameters.pop_int()?;
    let _n_w = parameters.pop_int()?;
    let _n_ss = parameters.pop_int()?;
    let _and_mask = parameters.pop_reference()?;
    let _int_raster_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WCustomCursor.createCursorIndirect([I[BIIIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WCustomCursor.getCursorHeight()I", Any)]
#[async_method]
pub async fn get_cursor_height<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WCustomCursor.getCursorHeight()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WCustomCursor.getCursorWidth()I", Any)]
#[async_method]
pub async fn get_cursor_width<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WCustomCursor.getCursorWidth()I".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_cursor_indirect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_cursor_indirect(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WCustomCursor.createCursorIndirect([I[BIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_cursor_height() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_cursor_height(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WCustomCursor.getCursorHeight()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_cursor_width() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_cursor_width(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WCustomCursor.getCursorWidth()I",
            result.unwrap_err().to_string()
        );
    }
}
