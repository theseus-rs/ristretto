use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/image/ImagingLib.convolveBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;Ljava/awt/image/Kernel;I)I",
    Any
)]
#[async_method]
pub async fn convolve_bi<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _edge_hint = parameters.pop_int()?;
    let _kernel = parameters.pop_reference()?;
    let _dst = parameters.pop_reference()?;
    let _src = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.awt.image.ImagingLib.convolveBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;Ljava/awt/image/Kernel;I)I".to_string()).into())
}

#[intrinsic_method(
    "sun/awt/image/ImagingLib.convolveRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;Ljava/awt/image/Kernel;I)I",
    Any
)]
#[async_method]
pub async fn convolve_raster<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _edge_hint = parameters.pop_int()?;
    let _kernel = parameters.pop_reference()?;
    let _dst = parameters.pop_reference()?;
    let _src = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.awt.image.ImagingLib.convolveRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;Ljava/awt/image/Kernel;I)I".to_string()).into())
}

#[intrinsic_method("sun/awt/image/ImagingLib.init()Z", Any)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.awt.image.ImagingLib.init()Z".to_string()).into())
}

#[intrinsic_method(
    "sun/awt/image/ImagingLib.lookupByteBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[[B)I",
    Any
)]
#[async_method]
pub async fn lookup_byte_bi<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _table = parameters.pop_reference()?;
    let _dst = parameters.pop_reference()?;
    let _src = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.awt.image.ImagingLib.lookupByteBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[[B)I".to_string()).into())
}

#[intrinsic_method(
    "sun/awt/image/ImagingLib.lookupByteRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[[B)I",
    Any
)]
#[async_method]
pub async fn lookup_byte_raster<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _table = parameters.pop_reference()?;
    let _dst = parameters.pop_reference()?;
    let _src = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.awt.image.ImagingLib.lookupByteRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[[B)I".to_string()).into())
}

#[intrinsic_method(
    "sun/awt/image/ImagingLib.transformBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[DI)I",
    Any
)]
#[async_method]
pub async fn transform_bi<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _interp_type = parameters.pop_int()?;
    let _matrix = parameters.pop_reference()?;
    let _dst = parameters.pop_reference()?;
    let _src = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.awt.image.ImagingLib.transformBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[DI)I".to_string()).into())
}

#[intrinsic_method(
    "sun/awt/image/ImagingLib.transformRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[DI)I",
    Any
)]
#[async_method]
pub async fn transform_raster<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _interp_type = parameters.pop_int()?;
    let _matrix = parameters.pop_reference()?;
    let _dst = parameters.pop_reference()?;
    let _src = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.awt.image.ImagingLib.transformRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[DI)I".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_convolve_bi() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = convolve_bi(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.awt.image.ImagingLib.convolveBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;Ljava/awt/image/Kernel;I)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_convolve_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = convolve_raster(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.awt.image.ImagingLib.convolveRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;Ljava/awt/image/Kernel;I)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.image.ImagingLib.init()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_lookup_byte_bi() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lookup_byte_bi(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.awt.image.ImagingLib.lookupByteBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[[B)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_lookup_byte_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lookup_byte_raster(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.awt.image.ImagingLib.lookupByteRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[[B)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_transform_bi() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = transform_bi(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.awt.image.ImagingLib.transformBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[DI)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_transform_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = transform_raster(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.awt.image.ImagingLib.transformRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[DI)I",
            result.unwrap_err().to_string()
        );
    }
}
