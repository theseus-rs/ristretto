use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/d3d/D3DSurfaceData.dbGetPixelNative(JII)I", Any)]
#[async_method]
pub async fn db_get_pixel_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DSurfaceData.dbGetPixelNative(JII)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/java2d/d3d/D3DSurfaceData.dbSetPixelNative(JIII)V", Any)]
#[async_method]
pub async fn db_set_pixel_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixel = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DSurfaceData.dbSetPixelNative(JIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/java2d/d3d/D3DSurfaceData.getNativeResourceNative(JI)J", Any)]
#[async_method]
pub async fn get_native_resource_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _res_type = parameters.pop_int()?;
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DSurfaceData.getNativeResourceNative(JI)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/java2d/d3d/D3DSurfaceData.initFlipBackbuffer(JJIII)Z", Any)]
#[async_method]
pub async fn init_flip_backbuffer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _v_sync_type = parameters.pop_int()?;
    let _swap_effect = parameters.pop_int()?;
    let _num_buffers = parameters.pop_int()?;
    let _p_peer_data = parameters.pop_long()?;
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DSurfaceData.initFlipBackbuffer(JJIII)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/java2d/d3d/D3DSurfaceData.initOps(III)V", Any)]
#[async_method]
pub async fn init_ops<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _gdi_screen = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/java2d/d3d/D3DSurfaceData.initOps(III)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/java2d/d3d/D3DSurfaceData.initRTSurface(JZ)Z", Any)]
#[async_method]
pub async fn init_rtsurface<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_opaque = parameters.pop_bool()?;
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DSurfaceData.initRTSurface(JZ)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/java2d/d3d/D3DSurfaceData.initTexture(JZZ)Z", Any)]
#[async_method]
pub async fn init_texture<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_opaque = parameters.pop_bool()?;
    let _is_rtt = parameters.pop_bool()?;
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DSurfaceData.initTexture(JZZ)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/java2d/d3d/D3DSurfaceData.updateWindowAccelImpl(JJII)Z", Any)]
#[async_method]
pub async fn update_window_accel_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _p_data = parameters.pop_long()?;
    let _pd3dsd = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DSurfaceData.updateWindowAccelImpl(JJII)Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_get_pixel_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = db_get_pixel_native(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/d3d/D3DSurfaceData.dbGetPixelNative(JII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_db_set_pixel_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = db_set_pixel_native(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/d3d/D3DSurfaceData.dbSetPixelNative(JIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_native_resource_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_resource_native(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/d3d/D3DSurfaceData.getNativeResourceNative(JI)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_flip_backbuffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_flip_backbuffer(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/d3d/D3DSurfaceData.initFlipBackbuffer(JJIII)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ops(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/d3d/D3DSurfaceData.initOps(III)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_rtsurface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_rtsurface(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/d3d/D3DSurfaceData.initRTSurface(JZ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_texture(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/d3d/D3DSurfaceData.initTexture(JZZ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_update_window_accel_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = update_window_accel_impl(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/d3d/D3DSurfaceData.updateWindowAccelImpl(JJII)Z",
            result.unwrap_err().to_string()
        );
    }
}
