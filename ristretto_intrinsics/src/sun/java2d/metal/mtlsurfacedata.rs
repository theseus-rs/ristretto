use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.clearWindow()V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn clear_window<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLSurfaceData.clearWindow()V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.getMTLTexturePointer(J)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_mtl_texture_pointer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLSurfaceData.getMTLTexturePointer(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.initFlipBackbuffer(J)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_flip_backbuffer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLSurfaceData.initFlipBackbuffer(J)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.initOps(Lsun/java2d/metal/MTLGraphicsConfig;JJJIIZ)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_ops<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_opaque = parameters.pop_bool()?;
    let _yoff = parameters.pop_int()?;
    let _xoff = parameters.pop_int()?;
    let _layer_ptr = parameters.pop_long()?;
    let _p_peer_data = parameters.pop_long()?;
    let _p_config_info = parameters.pop_long()?;
    let _gc = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLSurfaceData.initOps(Lsun/java2d/metal/MTLGraphicsConfig;JJJIIZ)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.initRTexture(JZII)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_r_texture<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _is_opaque = parameters.pop_bool()?;
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLSurfaceData.initRTexture(JZII)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.initTexture(JZII)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_texture<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _is_opaque = parameters.pop_bool()?;
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLSurfaceData.initTexture(JZII)Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_clear_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = clear_window(thread, Parameters::default()).await;
        assert_eq!(
            "sun.java2d.metal.MTLSurfaceData.clearWindow()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_mtl_texture_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_mtl_texture_pointer(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.metal.MTLSurfaceData.getMTLTexturePointer(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_flip_backbuffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_flip_backbuffer(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.metal.MTLSurfaceData.initFlipBackbuffer(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ops(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.metal.MTLSurfaceData.initOps(Lsun/java2d/metal/MTLGraphicsConfig;JJJIIZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_r_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_r_texture(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::from(false),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.metal.MTLSurfaceData.initRTexture(JZII)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_texture(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::from(false),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.metal.MTLSurfaceData.initTexture(JZII)Z",
            result.unwrap_err().to_string()
        );
    }
}
