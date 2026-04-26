use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJLsun/lwawt/macosx/CImage;III[JLjava/util/Map;)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_native_drag_source_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jformatmap = parameters.pop_reference()?;
    let _jformats = parameters.pop_reference()?;
    let _jsourceactions = parameters.pop_int()?;
    let _jdragimageoffsety = parameters.pop_int()?;
    let _jdragimageoffsetx = parameters.pop_int()?;
    let _nsdragimageptr = parameters.pop_reference()?;
    let _jtimestamp = parameters.pop_long()?;
    let _jclickcount = parameters.pop_int()?;
    let _jextmodifiers = parameters.pop_int()?;
    let _jdragposy = parameters.pop_int()?;
    let _jdragposx = parameters.pop_int()?;
    let _jtrigger = parameters.pop_reference()?;
    let _jtransferable = parameters.pop_reference()?;
    let _jnativepeer = parameters.pop_long()?;
    let _jcomponent = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJLsun/lwawt/macosx/CImage;III[JLjava/util/Map;)J".to_string()).into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJJIII[JLjava/util/Map;)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn create_native_drag_source_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jformatmap = parameters.pop_reference()?;
    let _jformats = parameters.pop_reference()?;
    let _jsourceactions = parameters.pop_int()?;
    let _jdragimageoffsety = parameters.pop_int()?;
    let _jdragimageoffsetx = parameters.pop_int()?;
    let _nsdragimageptr = parameters.pop_long()?;
    let _jtimestamp = parameters.pop_long()?;
    let _jclickcount = parameters.pop_int()?;
    let _jextmodifiers = parameters.pop_int()?;
    let _jdragposy = parameters.pop_int()?;
    let _jdragposx = parameters.pop_int()?;
    let _jtrigger = parameters.pop_reference()?;
    let _jtransferable = parameters.pop_reference()?;
    let _jnativepeer = parameters.pop_long()?;
    let _jcomponent = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJJIII[JLjava/util/Map;)J".to_string()).into())
}

#[intrinsic_method("sun/lwawt/macosx/CDragSourceContextPeer.doDragging(J)V", Any)]
#[async_method]
pub async fn do_dragging<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _native_drag_source = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDragSourceContextPeer.doDragging(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CDragSourceContextPeer.releaseNativeDragSource(J)V",
    Any
)]
#[async_method]
pub async fn release_native_drag_source<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _native_drag_source = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDragSourceContextPeer.releaseNativeDragSource(J)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_native_drag_source_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = create_native_drag_source_0(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJLsun/lwawt/macosx/CImage;III[JLjava/util/Map;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create_native_drag_source_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_native_drag_source_1(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJJIII[JLjava/util/Map;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_do_dragging() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_dragging(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CDragSourceContextPeer.doDragging(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_release_native_drag_source() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            release_native_drag_source(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CDragSourceContextPeer.releaseNativeDragSource(J)V",
            result.unwrap_err().to_string()
        );
    }
}
