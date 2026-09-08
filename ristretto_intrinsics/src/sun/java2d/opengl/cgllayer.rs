use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/opengl/CGLLayer.blitTexture(J)V", Any)]
pub fn blit_texture<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _layer_ptr = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.opengl.CGLLayer.blitTexture(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/java2d/opengl/CGLLayer.nativeCreateLayer()J", Any)]
pub fn native_create_layer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.CGLLayer.nativeCreateLayer()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/opengl/CGLLayer.nativeSetScale(JD)V", Any)]
pub fn native_set_scale<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _scale = parameters.pop_double()?;
    let _layer_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.CGLLayer.nativeSetScale(JD)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/opengl/CGLLayer.validate(JLsun/java2d/opengl/CGLSurfaceData;)V",
    Any
)]
pub fn validate<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cglsd = parameters.pop_reference()?;
    let _layer_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.CGLLayer.validate(JLsun/java2d/opengl/CGLSurfaceData;)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_blit_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = blit_texture(thread, Parameters::new(vec![Value::Long(0)]));
        assert_eq!(
            "sun.java2d.opengl.CGLLayer.blitTexture(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_create_layer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_layer(thread, Parameters::default());
        assert_eq!(
            "sun.java2d.opengl.CGLLayer.nativeCreateLayer()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_set_scale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_scale(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Double(0.0)]),
        );
        assert_eq!(
            "sun.java2d.opengl.CGLLayer.nativeSetScale(JD)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_validate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = validate(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        );
        assert_eq!(
            "sun.java2d.opengl.CGLLayer.validate(JLsun/java2d/opengl/CGLSurfaceData;)V",
            result.unwrap_err().to_string()
        );
    }
}
