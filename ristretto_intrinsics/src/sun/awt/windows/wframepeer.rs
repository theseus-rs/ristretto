use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WFramePeer.clearMaximizedBounds()V", Any)]
#[async_method]
pub async fn clear_maximized_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WFramePeer.clearMaximizedBounds()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WFramePeer.createAwtFrame(Lsun/awt/windows/WComponentPeer;)V",
    Any
)]
#[async_method]
pub async fn create_awt_frame<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _parent = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WFramePeer.createAwtFrame(Lsun/awt/windows/WComponentPeer;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WFramePeer.getState()I", Any)]
#[async_method]
pub async fn get_state<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WFramePeer.getState()I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WFramePeer.getSysMenuHeight()I", Any)]
#[async_method]
pub async fn get_sys_menu_height<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WFramePeer.getSysMenuHeight()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WFramePeer.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WFramePeer.initIDs()V".to_string()).into())
}
#[intrinsic_method("sun/awt/windows/WFramePeer.pSetIMMOption(Ljava/lang/String;)V", Any)]
#[async_method]
pub async fn p_set_immoption<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _option = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WFramePeer.pSetIMMOption(Ljava/lang/String;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WFramePeer.setMaximizedBounds(IIII)V", Any)]
#[async_method]
pub async fn set_maximized_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WFramePeer.setMaximizedBounds(IIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WFramePeer.setMenuBar0(Lsun/awt/windows/WMenuBarPeer;)V",
    Any
)]
#[async_method]
pub async fn set_menu_bar0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _mb_peer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WFramePeer.setMenuBar0(Lsun/awt/windows/WMenuBarPeer;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WFramePeer.setState(I)V", Any)]
#[async_method]
pub async fn set_state<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _state = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WFramePeer.setState(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WFramePeer.synthesizeWmActivate(Z)V", Any)]
#[async_method]
pub async fn synthesize_wm_activate<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _do_activate = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WFramePeer.synthesizeWmActivate(Z)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_clear_maximized_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = clear_maximized_bounds(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WFramePeer.clearMaximizedBounds()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_awt_frame() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_awt_frame(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WFramePeer.createAwtFrame(Lsun/awt/windows/WComponentPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_state(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WFramePeer.getState()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_sys_menu_height() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_sys_menu_height(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WFramePeer.getSysMenuHeight()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WFramePeer.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_p_set_immoption() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = p_set_immoption(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WFramePeer.pSetIMMOption(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_maximized_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_maximized_bounds(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WFramePeer.setMaximizedBounds(IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_menu_bar0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_menu_bar0(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WFramePeer.setMenuBar0(Lsun/awt/windows/WMenuBarPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_state(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WFramePeer.setState(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_synthesize_wm_activate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            synthesize_wm_activate(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/windows/WFramePeer.synthesizeWmActivate(Z)V",
            result.unwrap_err().to_string()
        );
    }
}
