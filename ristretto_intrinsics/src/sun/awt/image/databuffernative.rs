use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/image/DataBufferNative.getElem(IILsun/java2d/SurfaceData;)I",
    Any
)]
#[async_method]
pub async fn get_elem<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _s_data = parameters.pop_reference()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.image.DataBufferNative.getElem(IILsun/java2d/SurfaceData;)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/image/DataBufferNative.setElem(IIILsun/java2d/SurfaceData;)V",
    Any
)]
#[async_method]
pub async fn set_elem<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _s_data = parameters.pop_reference()?;
    let _val = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.image.DataBufferNative.setElem(IIILsun/java2d/SurfaceData;)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_elem() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_elem(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.awt.image.DataBufferNative.getElem(IILsun/java2d/SurfaceData;)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_elem() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_elem(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.awt.image.DataBufferNative.setElem(IIILsun/java2d/SurfaceData;)V",
            result.unwrap_err().to_string()
        );
    }
}
