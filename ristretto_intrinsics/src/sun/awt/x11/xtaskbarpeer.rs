use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/X11/XTaskbarPeer.init(Ljava/lang/String;IZ)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _verbose = parameters.pop_bool()?;
    let _version = parameters.pop_int()?;
    let _jname = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XTaskbarPeer.init(Ljava/lang/String;IZ)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XTaskbarPeer.runloop()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn runloop<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/XTaskbarPeer.runloop()V".to_string()).into())
}
#[intrinsic_method("sun/awt/X11/XTaskbarPeer.setBadge(JZ)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn set_badge<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _visible = parameters.pop_bool()?;
    let _value = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XTaskbarPeer.setBadge(JZ)V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/X11/XTaskbarPeer.setNativeMenu([Ljava/awt/MenuItem;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_native_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _items = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XTaskbarPeer.setNativeMenu([Ljava/awt/MenuItem;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XTaskbarPeer.setUrgent(Z)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn set_urgent<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _urgent = parameters.pop_bool()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XTaskbarPeer.setUrgent(Z)V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/X11/XTaskbarPeer.updateProgress(DZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn update_progress<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _visible = parameters.pop_bool()?;
    let _value = parameters.pop_double()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XTaskbarPeer.updateProgress(DZ)V".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XTaskbarPeer.init(Ljava/lang/String;IZ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_runloop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = runloop(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XTaskbarPeer.runloop()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_badge() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_badge(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XTaskbarPeer.setBadge(JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_native_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_native_menu(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/X11/XTaskbarPeer.setNativeMenu([Ljava/awt/MenuItem;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_urgent() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_urgent(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/X11/XTaskbarPeer.setUrgent(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_update_progress() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = update_progress(
            thread,
            Parameters::new(vec![Value::Double(0.0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XTaskbarPeer.updateProgress(DZ)V",
            result.unwrap_err().to_string()
        );
    }
}
