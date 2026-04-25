use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/X11/XWindow.getAWTKeyCodeForKeySym(I)I", Any)]
#[async_method]
pub async fn get_awtkey_code_for_key_sym<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _keysym = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XWindow.getAWTKeyCodeForKeySym(I)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XWindow.getKeySymForAWTKeyCode(I)I", Any)]
#[async_method]
pub async fn get_key_sym_for_awtkey_code<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _keycode = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XWindow.getKeySymForAWTKeyCode(I)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/X11/XWindow.getNativeColor(Ljava/awt/Color;Ljava/awt/GraphicsConfiguration;)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_native_color<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _gc_object = parameters.pop_reference()?;
    let _color = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XWindow.getNativeColor(Ljava/awt/Color;Ljava/awt/GraphicsConfiguration;)I"
            .to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XWindow.getTopWindow(JJ)J", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn get_top_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _root_win = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/XWindow.getTopWindow(JJ)J".to_string()).into())
}
#[intrinsic_method("sun/awt/X11/XWindow.getWMInsets(JJJJJJ)V", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn get_wminsets<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _border = parameters.pop_long()?;
    let _bottom = parameters.pop_long()?;
    let _right = parameters.pop_long()?;
    let _top = parameters.pop_long()?;
    let _left = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XWindow.getWMInsets(JJJJJJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/X11/XWindow.getWindowBounds(JJJJJ)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_window_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_long()?;
    let _width = parameters.pop_long()?;
    let _y = parameters.pop_long()?;
    let _x = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XWindow.getWindowBounds(JJJJJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XWindow.haveCurrentX11InputMethodInstance()Z", Any)]
#[async_method]
pub async fn have_current_x11_input_method_instance<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XWindow.haveCurrentX11InputMethodInstance()Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XWindow.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/XWindow.initIDs()V".to_string()).into())
}
#[intrinsic_method("sun/awt/X11/XWindow.x11inputMethodLookupString(J[J)Z", Any)]
#[async_method]
pub async fn x11input_method_lookup_string<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _keysym_array = parameters.pop_reference()?;
    let _event = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XWindow.x11inputMethodLookupString(J[J)Z".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_awtkey_code_for_key_sym() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_awtkey_code_for_key_sym(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11/XWindow.getAWTKeyCodeForKeySym(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_key_sym_for_awtkey_code() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_key_sym_for_awtkey_code(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11/XWindow.getKeySymForAWTKeyCode(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_native_color() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_color(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XWindow.getNativeColor(Ljava/awt/Color;Ljava/awt/GraphicsConfiguration;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_top_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_top_window(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XWindow.getTopWindow(JJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_wminsets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_wminsets(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XWindow.getWMInsets(JJJJJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_window_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_window_bounds(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XWindow.getWindowBounds(JJJJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_have_current_x11_input_method_instance() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = have_current_x11_input_method_instance(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XWindow.haveCurrentX11InputMethodInstance()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XWindow.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_x11input_method_lookup_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = x11input_method_lookup_string(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XWindow.x11inputMethodLookupString(J[J)Z",
            result.unwrap_err().to_string()
        );
    }
}
