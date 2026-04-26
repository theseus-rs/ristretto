use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WInputMethod.createNativeContext()I", Any)]
#[async_method]
pub async fn create_native_context<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethod.createNativeContext()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WInputMethod.destroyNativeContext(I)V", Any)]
#[async_method]
pub async fn destroy_native_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _context = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethod.destroyNativeContext(I)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WInputMethod.disableNativeIME(Lsun/awt/windows/WComponentPeer;)V",
    Any
)]
#[async_method]
pub async fn disable_native_ime<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _peer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethod.disableNativeIME(Lsun/awt/windows/WComponentPeer;)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WInputMethod.enableNativeIME(Lsun/awt/windows/WComponentPeer;IZ)V",
    Any
)]
#[async_method]
pub async fn enable_native_ime<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _use_native_comp_window = parameters.pop_bool()?;
    let _context = parameters.pop_int()?;
    let _peer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethod.enableNativeIME(Lsun/awt/windows/WComponentPeer;IZ)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WInputMethod.endCompositionNative(IZ)V", Any)]
#[async_method]
pub async fn end_composition_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flag = parameters.pop_bool()?;
    let _context = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethod.endCompositionNative(IZ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WInputMethod.getConversionStatus(I)I", Any)]
#[async_method]
pub async fn get_conversion_status<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _context = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethod.getConversionStatus(I)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WInputMethod.getNativeIMMDescription()Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_native_immdescription<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethod.getNativeIMMDescription()Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WInputMethod.getNativeLocale()Ljava/util/Locale;",
    Any
)]
#[async_method]
pub async fn get_native_locale<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethod.getNativeLocale()Ljava/util/Locale;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WInputMethod.getOpenStatus(I)Z", Any)]
#[async_method]
pub async fn get_open_status<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _context = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethod.getOpenStatus(I)Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WInputMethod.handleNativeIMEEvent(Lsun/awt/windows/WComponentPeer;Ljava/awt/AWTEvent;)V",
    Any
)]
#[async_method]
pub async fn handle_native_imeevent<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _event = parameters.pop_reference()?;
    let _peer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WInputMethod.handleNativeIMEEvent(Lsun/awt/windows/WComponentPeer;Ljava/awt/AWTEvent;)V".to_string()).into())
}
#[intrinsic_method("sun/awt/windows/WInputMethod.isCompositionStringAvailable(I)Z", Any)]
#[async_method]
pub async fn is_composition_string_available<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _context = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethod.isCompositionStringAvailable(I)Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WInputMethod.openCandidateWindow(Lsun/awt/windows/WComponentPeer;II)V",
    Any
)]
#[async_method]
pub async fn open_candidate_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _peer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethod.openCandidateWindow(Lsun/awt/windows/WComponentPeer;II)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WInputMethod.setConversionStatus(II)V", Any)]
#[async_method]
pub async fn set_conversion_status<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _request = parameters.pop_int()?;
    let _context = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethod.setConversionStatus(II)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WInputMethod.setNativeLocale(Ljava/lang/String;Z)Z",
    Any
)]
#[async_method]
pub async fn set_native_locale<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _on_activate = parameters.pop_bool()?;
    let _locale_string = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethod.setNativeLocale(Ljava/lang/String;Z)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WInputMethod.setOpenStatus(IZ)V", Any)]
#[async_method]
pub async fn set_open_status<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flag = parameters.pop_bool()?;
    let _context = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethod.setOpenStatus(IZ)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WInputMethod.setStatusWindowVisible(Lsun/awt/windows/WComponentPeer;Z)V",
    Any
)]
#[async_method]
pub async fn set_status_window_visible<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _visible = parameters.pop_bool()?;
    let _peer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethod.setStatusWindowVisible(Lsun/awt/windows/WComponentPeer;Z)V"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_native_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_native_context(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.createNativeContext()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_destroy_native_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = destroy_native_context(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.destroyNativeContext(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_disable_native_ime() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = disable_native_ime(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.disableNativeIME(Lsun/awt/windows/WComponentPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_enable_native_ime() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enable_native_ime(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.enableNativeIME(Lsun/awt/windows/WComponentPeer;IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_end_composition_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = end_composition_native(
            thread,
            Parameters::new(vec![Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.endCompositionNative(IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_conversion_status() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_conversion_status(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.getConversionStatus(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_native_immdescription() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_immdescription(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.getNativeIMMDescription()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_native_locale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_locale(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.getNativeLocale()Ljava/util/Locale;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_open_status() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_open_status(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.getOpenStatus(I)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_handle_native_imeevent() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = handle_native_imeevent(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.handleNativeIMEEvent(Lsun/awt/windows/WComponentPeer;Ljava/awt/AWTEvent;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_composition_string_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            is_composition_string_available(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.isCompositionStringAvailable(I)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_open_candidate_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_candidate_window(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.openCandidateWindow(Lsun/awt/windows/WComponentPeer;II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_conversion_status() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_conversion_status(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.setConversionStatus(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_native_locale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_native_locale(
            thread,
            Parameters::new(vec![Value::Object(None), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.setNativeLocale(Ljava/lang/String;Z)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_open_status() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_open_status(
            thread,
            Parameters::new(vec![Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.setOpenStatus(IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_status_window_visible() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_status_window_visible(
            thread,
            Parameters::new(vec![Value::Object(None), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WInputMethod.setStatusWindowVisible(Lsun/awt/windows/WComponentPeer;Z)V",
            result.unwrap_err().to_string()
        );
    }
}
