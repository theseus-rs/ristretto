use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/X11/XToolkit.awt_output_flush()V", Any)]
#[async_method]
pub async fn awt_output_flush<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XToolkit.awt_output_flush()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XToolkit.awt_toolkit_init()V", Any)]
#[async_method]
pub async fn awt_toolkit_init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XToolkit.awt_toolkit_init()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XToolkit.getDefaultXColormap()J", Any)]
#[async_method]
pub async fn get_default_xcolormap<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XToolkit.getDefaultXColormap()J".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/X11/XToolkit.getEnv(Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_env<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _key = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XToolkit.getEnv(Ljava/lang/String;)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XToolkit.getNumberOfButtonsImpl()I", Any)]
#[async_method]
pub async fn get_number_of_buttons_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XToolkit.getNumberOfButtonsImpl()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XToolkit.getTrayIconDisplayTimeout()J", Any)]
#[async_method]
pub async fn get_tray_icon_display_timeout<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XToolkit.getTrayIconDisplayTimeout()J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XToolkit.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/XToolkit.initIDs()V".to_string()).into())
}
#[intrinsic_method("sun/awt/X11/XToolkit.nativeLoadSystemColors([I)V", Any)]
#[async_method]
pub async fn native_load_system_colors<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _system_colors = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XToolkit.nativeLoadSystemColors([I)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XToolkit.waitForEvents(J)V", Any)]
#[async_method]
pub async fn wait_for_events<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _next_task_time = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XToolkit.waitForEvents(J)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XToolkit.wakeup_poll()V", Any)]
#[async_method]
pub async fn wakeup_poll<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/XToolkit.wakeup_poll()V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_awt_output_flush() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = awt_output_flush(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XToolkit.awt_output_flush()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_awt_toolkit_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = awt_toolkit_init(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XToolkit.awt_toolkit_init()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_default_xcolormap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_default_xcolormap(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XToolkit.getDefaultXColormap()J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_env() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_env(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/X11/XToolkit.getEnv(Ljava/lang/String;)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_number_of_buttons_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_number_of_buttons_impl(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XToolkit.getNumberOfButtonsImpl()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_tray_icon_display_timeout() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_tray_icon_display_timeout(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XToolkit.getTrayIconDisplayTimeout()J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XToolkit.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_load_system_colors() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_load_system_colors(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/X11/XToolkit.nativeLoadSystemColors([I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_wait_for_events() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = wait_for_events(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XToolkit.waitForEvents(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_wakeup_poll() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = wakeup_poll(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XToolkit.wakeup_poll()V",
            result.unwrap_err().to_string()
        );
    }
}
