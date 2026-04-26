use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/OSXOffScreenSurfaceData.clearSurfacePixels(II)Z", Any)]
#[async_method]
pub async fn clear_surface_pixels<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.OSXOffScreenSurfaceData.clearSurfacePixels(IIZ)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/OSXOffScreenSurfaceData.getSurfaceData(Ljava/awt/image/BufferedImage;)Lsun/java2d/SurfaceData;",
    Any
)]
#[async_method]
pub async fn get_surface_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buf_img = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.OSXOffScreenSurfaceData.getSurfaceData(Ljava/awt/image/BufferedImage;)Lsun/java2d/SurfaceData;".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/OSXOffScreenSurfaceData.initCustomRaster(Ljava/nio/IntBuffer;IILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V",
    Any
)]
#[async_method]
pub async fn init_custom_raster<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_image_info = parameters.pop_reference()?;
    let _j_graphics_state_object = parameters.pop_reference()?;
    let _j_graphics_state = parameters.pop_reference()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _array = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.OSXOffScreenSurfaceData.initCustomRaster(Ljava/nio/IntBuffer;IILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V".to_string()).into())
}

#[intrinsic_method("sun/java2d/OSXOffScreenSurfaceData.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/java2d/OSXOffScreenSurfaceData.initRaster(Ljava/lang/Object;IIIIILjava/awt/image/IndexColorModel;ILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V",
    Any
)]
#[async_method]
pub async fn init_raster<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_image_info = parameters.pop_reference()?;
    let _j_graphics_state_object = parameters.pop_reference()?;
    let _j_graphics_state = parameters.pop_reference()?;
    let _type_ = parameters.pop_int()?;
    let _icm = parameters.pop_reference()?;
    let _scan_stride = parameters.pop_int()?;
    let _pixel_stride = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _offset = parameters.pop_int()?;
    let _array = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.OSXOffScreenSurfaceData.initRaster(Ljava/lang/Object;IIIIILjava/awt/image/IndexColorModel;ILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/OSXOffScreenSurfaceData.setSurfaceData(Ljava/awt/image/BufferedImage;Lsun/java2d/SurfaceData;)V",
    Any
)]
#[async_method]
pub async fn set_surface_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _s_data = parameters.pop_reference()?;
    let _buf_img = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.OSXOffScreenSurfaceData.setSurfaceData(Ljava/awt/image/BufferedImage;Lsun/java2d/SurfaceData;)V".to_string()).into())
}

#[intrinsic_method("sun/java2d/OSXOffScreenSurfaceData.syncToJavaPixels()V", Any)]
#[async_method]
pub async fn sync_to_java_pixels<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.OSXOffScreenSurfaceData.syncToJavaPixels()V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/OSXOffScreenSurfaceData.xorSurfacePixels(Lsun/java2d/SurfaceData;IIIII)Z",
    Any
)]
#[async_method]
pub async fn xor_surface_pixels<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _color_xor = parameters.pop_int()?;
    let _src = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.OSXOffScreenSurfaceData.xorSurfacePixels(Lsun/java2d/SurfaceData;IIIII)Z"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_clear_surface_pixels() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            clear_surface_pixels(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.java2d.OSXOffScreenSurfaceData.clearSurfacePixels(IIZ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_surface_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_surface_data(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.java2d.OSXOffScreenSurfaceData.getSurfaceData(Ljava/awt/image/BufferedImage;)Lsun/java2d/SurfaceData;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_custom_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_custom_raster(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.OSXOffScreenSurfaceData.initCustomRaster(Ljava/nio/IntBuffer;IILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V",
            result.unwrap_err().to_string()
        );
    }

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
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.OSXOffScreenSurfaceData.initRaster(Ljava/lang/Object;IIIIILjava/awt/image/IndexColorModel;ILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_surface_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_surface_data(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.OSXOffScreenSurfaceData.setSurfaceData(Ljava/awt/image/BufferedImage;Lsun/java2d/SurfaceData;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_sync_to_java_pixels() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = sync_to_java_pixels(thread, Parameters::default()).await;
        assert_eq!(
            "sun.java2d.OSXOffScreenSurfaceData.syncToJavaPixels()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_xor_surface_pixels() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xor_surface_pixels(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.OSXOffScreenSurfaceData.xorSurfacePixels(Lsun/java2d/SurfaceData;IIIII)Z",
            result.unwrap_err().to_string()
        );
    }
}
