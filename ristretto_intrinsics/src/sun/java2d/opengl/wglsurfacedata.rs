use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/opengl/WGLSurfaceData.initOps(Lsun/java2d/opengl/OGLGraphicsConfig;JLsun/awt/windows/WComponentPeer;J)V",
    Any
)]
#[async_method]
pub async fn init_ops<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _hwnd = parameters.pop_long()?;
    let _peer = parameters.pop_reference()?;
    let _p_config_info = parameters.pop_long()?;
    let _gc = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/opengl/WGLSurfaceData.initOps(Lsun/java2d/opengl/OGLGraphicsConfig;JLsun/awt/windows/WComponentPeer;J)V".to_string()).into())
}
#[intrinsic_method(
    "sun/java2d/opengl/WGLSurfaceData.updateWindowAccelImpl(JLsun/awt/windows/WComponentPeer;II)Z",
    Any
)]
#[async_method]
pub async fn update_window_accel_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _peer = parameters.pop_reference()?;
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/opengl/WGLSurfaceData.updateWindowAccelImpl(JLsun/awt/windows/WComponentPeer;II)Z".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ops(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Long(0),
                Value::Object(None),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/opengl/WGLSurfaceData.initOps(Lsun/java2d/opengl/OGLGraphicsConfig;JLsun/awt/windows/WComponentPeer;J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_update_window_accel_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = update_window_accel_impl(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/opengl/WGLSurfaceData.updateWindowAccelImpl(JLsun/awt/windows/WComponentPeer;II)Z",
            result.unwrap_err().to_string()
        );
    }
}
