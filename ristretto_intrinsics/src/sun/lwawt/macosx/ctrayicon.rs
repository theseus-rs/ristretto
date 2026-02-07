use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{
    Any, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CTrayIcon.nativeCreate()J", Any)]
#[async_method]
pub async fn native_create<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTrayIcon.nativeCreate()J")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTrayIcon.nativeGetIconLocation(J)Ljava/awt/geom/Point2D;",
    Any
)]
#[async_method]
pub async fn native_get_icon_location<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTrayIcon.nativeGetIconLocation(J)Ljava/awt/geom/Point2D;")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTrayIcon.nativeSetToolTip(JLjava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn native_set_tool_tip<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTrayIcon.nativeSetToolTip(JLjava/lang/String;)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTrayIcon.nativeShowNotification(JLjava/lang/String;Ljava/lang/String;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_show_notification<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CTrayIcon.nativeShowNotification(JLjava/lang/String;Ljava/lang/String;J)V"
    )
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTrayIcon.setNativeImage(JJZ)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_native_image_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTrayIcon.setNativeImage(JJZ)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTrayIcon.setNativeImage(JJZZ)V",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn set_native_image_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTrayIcon.setNativeImage(JJZZ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CTrayIcon.nativeCreate()J")]
    async fn test_native_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CTrayIcon.nativeGetIconLocation(J)Ljava/awt/geom/Point2D;"
    )]
    async fn test_native_get_icon_location() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_icon_location(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CTrayIcon.nativeSetToolTip(JLjava/lang/String;)V"
    )]
    async fn test_native_set_tool_tip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_tool_tip(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CTrayIcon.nativeShowNotification(JLjava/lang/String;Ljava/lang/String;J)V"
    )]
    async fn test_native_show_notification() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_show_notification(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CTrayIcon.setNativeImage(JJZ)V"
    )]
    async fn test_set_native_image_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_native_image_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CTrayIcon.setNativeImage(JJZZ)V"
    )]
    async fn test_set_native_image_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_native_image_1(thread, Parameters::default()).await;
    }
}
