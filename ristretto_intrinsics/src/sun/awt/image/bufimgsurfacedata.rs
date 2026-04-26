use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/image/BufImgSurfaceData.initIDs(Ljava/lang/Class;Ljava/lang/Class;)V",
    Any
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/awt/image/BufImgSurfaceData.initRaster(Ljava/lang/Object;IIIIIILjava/awt/image/IndexColorModel;)V",
    Any
)]
#[async_method]
pub async fn init_raster<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _icm = parameters.pop_reference()?;
    let _scan_str = parameters.pop_int()?;
    let _pix_str = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _bitoffset = parameters.pop_int()?;
    let _offset = parameters.pop_int()?;
    let _array = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.awt.image.BufImgSurfaceData.initRaster(Ljava/lang/Object;IIIIIILjava/awt/image/IndexColorModel;)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_raster(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.awt.image.BufImgSurfaceData.initRaster(Ljava/lang/Object;IIIIIILjava/awt/image/IndexColorModel;)V",
            result.unwrap_err().to_string()
        );
    }
}
