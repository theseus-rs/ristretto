use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{
    Any, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CTrayIcon.nativeCreate()J", Any)]
#[async_method]
pub async fn native_create<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CTrayIcon.nativeCreate()J".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTrayIcon.nativeGetIconLocation(J)Ljava/awt/geom/Point2D;",
    Any
)]
#[async_method]
pub async fn native_get_icon_location<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _tray_icon_model = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CTrayIcon.nativeGetIconLocation(J)Ljava/awt/geom/Point2D;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTrayIcon.nativeSetToolTip(JLjava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn native_set_tool_tip<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jtooltip = parameters.pop_reference()?;
    let _model = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CTrayIcon.nativeSetToolTip(JLjava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTrayIcon.nativeShowNotification(JLjava/lang/String;Ljava/lang/String;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_show_notification<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _nsimage = parameters.pop_long()?;
    let _jtext = parameters.pop_reference()?;
    let _jcaption = parameters.pop_reference()?;
    let _model = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CTrayIcon.nativeShowNotification(JLjava/lang/String;Ljava/lang/String;J)V".to_string()).into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTrayIcon.setNativeImage(JJZ)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_native_image_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _autosize = parameters.pop_bool()?;
    let _nsimage = parameters.pop_long()?;
    let _model = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CTrayIcon.setNativeImage(JJZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTrayIcon.setNativeImage(JJZZ)V",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn set_native_image_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _template = parameters.pop_bool()?;
    let _autosize = parameters.pop_bool()?;
    let _nsimage = parameters.pop_long()?;
    let _model = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CTrayIcon.setNativeImage(JJZZ)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.CTrayIcon.nativeCreate()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_icon_location() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_icon_location(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CTrayIcon.nativeGetIconLocation(J)Ljava/awt/geom/Point2D;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_set_tool_tip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_tool_tip(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CTrayIcon.nativeSetToolTip(JLjava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_show_notification() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_show_notification(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CTrayIcon.nativeShowNotification(JLjava/lang/String;Ljava/lang/String;J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_native_image_0() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = set_native_image_0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CTrayIcon.setNativeImage(JJZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_native_image_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_native_image_1(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::from(false),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CTrayIcon.setNativeImage(JJZZ)V",
            result.unwrap_err().to_string()
        );
    }
}
