use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/d3d/D3DGraphicsDevice.configDisplayModeNative(IJIIII)V",
    Any
)]
#[async_method]
pub async fn config_display_mode_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _refresh_rate = parameters.pop_int()?;
    let _bit_depth = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _window = parameters.pop_long()?;
    let _gdi_screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DGraphicsDevice.configDisplayModeNative(IJIIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/java2d/d3d/D3DGraphicsDevice.enterFullScreenExclusiveNative(IJ)Z",
    Any
)]
#[async_method]
pub async fn enter_full_screen_exclusive_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    let _gdi_screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DGraphicsDevice.enterFullScreenExclusiveNative(IJ)Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/java2d/d3d/D3DGraphicsDevice.enumDisplayModesNative(ILjava/util/ArrayList;)V",
    Any
)]
#[async_method]
pub async fn enum_display_modes_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _array_list = parameters.pop_reference()?;
    let _gdi_screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DGraphicsDevice.enumDisplayModesNative(ILjava/util/ArrayList;)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/java2d/d3d/D3DGraphicsDevice.exitFullScreenExclusiveNative(I)Z",
    Any
)]
#[async_method]
pub async fn exit_full_screen_exclusive_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _gdi_screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DGraphicsDevice.exitFullScreenExclusiveNative(I)Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/java2d/d3d/D3DGraphicsDevice.getAvailableAcceleratedMemoryNative(I)J",
    Any
)]
#[async_method]
pub async fn get_available_accelerated_memory_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _gdi_screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DGraphicsDevice.getAvailableAcceleratedMemoryNative(I)J".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/java2d/d3d/D3DGraphicsDevice.getCurrentDisplayModeNative(I)Ljava/awt/DisplayMode;",
    Any
)]
#[async_method]
pub async fn get_current_display_mode_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _gdi_screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DGraphicsDevice.getCurrentDisplayModeNative(I)Ljava/awt/DisplayMode;"
            .to_string(),
    )
    .into())
}
#[intrinsic_method("sun/java2d/d3d/D3DGraphicsDevice.getDeviceCapsNative(I)I", Any)]
#[async_method]
pub async fn get_device_caps_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _gdi_screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DGraphicsDevice.getDeviceCapsNative(I)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/java2d/d3d/D3DGraphicsDevice.getDeviceIdNative(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_device_id_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _gdi_screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DGraphicsDevice.getDeviceIdNative(I)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/java2d/d3d/D3DGraphicsDevice.initD3D()Z", Any)]
#[async_method]
pub async fn init_d3_d<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/java2d/d3d/D3DGraphicsDevice.initD3D()Z".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/java2d/d3d/D3DGraphicsDevice.isD3DAvailableOnDeviceNative(I)Z",
    Any
)]
#[async_method]
pub async fn is_d3_davailable_on_device_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _gdi_screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DGraphicsDevice.isD3DAvailableOnDeviceNative(I)Z".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_config_display_mode_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = config_display_mode_native(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/d3d/D3DGraphicsDevice.configDisplayModeNative(IJIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_enter_full_screen_exclusive_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enter_full_screen_exclusive_native(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/d3d/D3DGraphicsDevice.enterFullScreenExclusiveNative(IJ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_enum_display_modes_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enum_display_modes_native(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/d3d/D3DGraphicsDevice.enumDisplayModesNative(ILjava/util/ArrayList;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_exit_full_screen_exclusive_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            exit_full_screen_exclusive_native(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/java2d/d3d/D3DGraphicsDevice.exitFullScreenExclusiveNative(I)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_available_accelerated_memory_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_available_accelerated_memory_native(thread, Parameters::new(vec![Value::Int(0)]))
                .await;
        assert_eq!(
            "sun/java2d/d3d/D3DGraphicsDevice.getAvailableAcceleratedMemoryNative(I)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_current_display_mode_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_current_display_mode_native(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/java2d/d3d/D3DGraphicsDevice.getCurrentDisplayModeNative(I)Ljava/awt/DisplayMode;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_device_caps_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_device_caps_native(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/java2d/d3d/D3DGraphicsDevice.getDeviceCapsNative(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_device_id_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_device_id_native(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/java2d/d3d/D3DGraphicsDevice.getDeviceIdNative(I)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_d3_d() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_d3_d(thread, Parameters::default()).await;
        assert_eq!(
            "sun/java2d/d3d/D3DGraphicsDevice.initD3D()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_d3_davailable_on_device_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            is_d3_davailable_on_device_native(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/java2d/d3d/D3DGraphicsDevice.isD3DAvailableOnDeviceNative(I)Z",
            result.unwrap_err().to_string()
        );
    }
}
