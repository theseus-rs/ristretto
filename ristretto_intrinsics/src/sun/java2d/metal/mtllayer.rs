use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/metal/MTLLayer.blitTexture(J)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn blit_texture<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _layer_ptr = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.metal.MTLLayer.blitTexture(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/java2d/metal/MTLLayer.nativeCreateLayer()J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn native_create_layer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLLayer.nativeCreateLayer()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLLayer.nativeSetInsets(JII)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn native_set_insets<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _left = parameters.pop_int()?;
    let _top = parameters.pop_int()?;
    let _layer_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLLayer.nativeSetInsets(JII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLLayer.nativeSetOpaque(JZ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn native_set_opaque<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _opaque = parameters.pop_bool()?;
    let _layer_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLLayer.nativeSetOpaque(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLLayer.nativeSetScale(JD)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn native_set_scale<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _scale = parameters.pop_double()?;
    let _layer_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLLayer.nativeSetScale(JD)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLLayer.validate(JLsun/java2d/metal/MTLSurfaceData;)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn validate<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _mtlsd = parameters.pop_reference()?;
    let _layer_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLLayer.validate(JLsun/java2d/metal/MTLSurfaceData;)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_blit_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = blit_texture(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.metal.MTLLayer.blitTexture(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_create_layer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_layer(thread, Parameters::default()).await;
        assert_eq!(
            "sun.java2d.metal.MTLLayer.nativeCreateLayer()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_set_insets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_insets(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.metal.MTLLayer.nativeSetInsets(JII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_set_opaque() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_opaque(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.metal.MTLLayer.nativeSetOpaque(JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_set_scale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_scale(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Double(0.0)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.metal.MTLLayer.nativeSetScale(JD)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_validate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = validate(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.metal.MTLLayer.validate(JLsun/java2d/metal/MTLSurfaceData;)V",
            result.unwrap_err().to_string()
        );
    }
}
