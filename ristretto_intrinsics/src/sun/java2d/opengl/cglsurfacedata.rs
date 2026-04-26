use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/opengl/CGLSurfaceData.clearWindow()V", Any)]
#[async_method]
pub async fn clear_window<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.CGLSurfaceData.clearWindow()".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/opengl/CGLSurfaceData.createCGLContextOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn create_cgl_context_on_surface<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _shared_context = parameters.pop_long()?;
    let _sd = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.opengl.CGLSurfaceData.createCGLContextOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)J".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/opengl/CGLSurfaceData.destroyCGLContext(J)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn destroy_cgl_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ctx = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.CGLSurfaceData.destroyCGLContext(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/opengl/CGLSurfaceData.initOps(Lsun/java2d/opengl/OGLGraphicsConfig;JJJIIZ)V",
    Any
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
        "sun.java2d.opengl.CGLSurfaceData.initOps(Lsun/java2d/opengl/OGLGraphicsConfig;JJJIIZ)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/opengl/CGLSurfaceData.makeCGLContextCurrentOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)Z",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn make_cgl_context_current_on_surface<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ctx = parameters.pop_long()?;
    let _sd = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.opengl.CGLSurfaceData.makeCGLContextCurrentOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)Z".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/opengl/CGLSurfaceData.validate(IIIIZ)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn validate<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_opaque = parameters.pop_bool()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _yoff = parameters.pop_int()?;
    let _xoff = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.CGLSurfaceData.validate(IIIIZ)V".to_string(),
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
            "sun.java2d.opengl.CGLSurfaceData.clearWindow()",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create_cgl_context_on_surface() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = create_cgl_context_on_surface(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.opengl.CGLSurfaceData.createCGLContextOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_destroy_cgl_context() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = destroy_cgl_context(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.opengl.CGLSurfaceData.destroyCGLContext(J)V",
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
            "sun.java2d.opengl.CGLSurfaceData.initOps(Lsun/java2d/opengl/OGLGraphicsConfig;JJJIIZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_make_cgl_context_current_on_surface() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = make_cgl_context_current_on_surface(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.opengl.CGLSurfaceData.makeCGLContextCurrentOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_validate() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = validate(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.opengl.CGLSurfaceData.validate(IIIIZ)V",
            result.unwrap_err().to_string()
        );
    }
}
