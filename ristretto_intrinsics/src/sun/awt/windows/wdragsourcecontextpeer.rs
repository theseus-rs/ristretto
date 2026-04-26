use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WDragSourceContextPeer.createDragSource(Ljava/awt/Component;Ljava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;I[JLjava/util/Map;)J",
    Any
)]
#[async_method]
pub async fn create_drag_source<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _format_map = parameters.pop_reference()?;
    let _formats = parameters.pop_reference()?;
    let _actions = parameters.pop_int()?;
    let _trigger = parameters.pop_reference()?;
    let _transferable = parameters.pop_reference()?;
    let _component = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WDragSourceContextPeer.createDragSource(Ljava/awt/Component;Ljava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;I[JLjava/util/Map;)J".to_string()).into())
}
#[intrinsic_method(
    "sun/awt/windows/WDragSourceContextPeer.doDragDrop(JLjava/awt/Cursor;[IIIII)V",
    Any
)]
#[async_method]
pub async fn do_drag_drop<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _image_height = parameters.pop_int()?;
    let _image_width = parameters.pop_int()?;
    let _image_data = parameters.pop_reference()?;
    let _cursor = parameters.pop_reference()?;
    let _native_ctxt = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDragSourceContextPeer.doDragDrop(JLjava/awt/Cursor;[IIIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WDragSourceContextPeer.setNativeCursor(JLjava/awt/Cursor;I)V",
    Any
)]
#[async_method]
pub async fn set_native_cursor<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _type_ = parameters.pop_int()?;
    let _cursor = parameters.pop_reference()?;
    let _native_ctxt = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDragSourceContextPeer.setNativeCursor(JLjava/awt/Cursor;I)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_drag_source() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_drag_source(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WDragSourceContextPeer.createDragSource(Ljava/awt/Component;Ljava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;I[JLjava/util/Map;)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_do_drag_drop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_drag_drop(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WDragSourceContextPeer.doDragDrop(JLjava/awt/Cursor;[IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_native_cursor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_native_cursor(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WDragSourceContextPeer.setNativeCursor(JLjava/awt/Cursor;I)V",
            result.unwrap_err().to_string()
        );
    }
}
