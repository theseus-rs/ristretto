use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.abortRead(J)V", Any)]
#[async_method]
pub async fn abort_read<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _struct_pointer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.abortRead(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.clearNativeReadAbortFlag(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn clear_native_read_abort_flag<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _struct_pointer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.clearNativeReadAbortFlag(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.disposeReader(J)V", Any)]
#[async_method]
pub async fn dispose_reader<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _struct_pointer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.disposeReader(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.initJPEGImageReader()J",
    Any
)]
#[async_method]
pub async fn init_jpeg_image_reader<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.initJPEGImageReader()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.initReaderIDs(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V",
    Any
)]
#[async_method]
pub async fn init_reader_i_ds<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _huff_class = parameters.pop_reference()?;
    let _q_table_class = parameters.pop_reference()?;
    let _image_input_stream_class = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com.sun.imageio.plugins.jpeg.JPEGImageReader.initReaderIDs(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V".to_string()).into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.readImage(IJ[BI[I[IIIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;IIZ)Z",
    Any
)]
#[async_method]
pub async fn read_image<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _want_updates = parameters.pop_bool()?;
    let _max_progressive_pass = parameters.pop_int()?;
    let _min_progressive_pass = parameters.pop_int()?;
    let _abbrev_ac_huffman_tables = parameters.pop_reference()?;
    let _abbrev_dc_huffman_tables = parameters.pop_reference()?;
    let _abbrev_q_tables = parameters.pop_reference()?;
    let _period_y = parameters.pop_int()?;
    let _period_x = parameters.pop_int()?;
    let _source_height = parameters.pop_int()?;
    let _source_width = parameters.pop_int()?;
    let _source_y_offset = parameters.pop_int()?;
    let _source_x_offset = parameters.pop_int()?;
    let _band_sizes = parameters.pop_reference()?;
    let _src_bands = parameters.pop_reference()?;
    let _num_raster_bands = parameters.pop_int()?;
    let _buffer = parameters.pop_reference()?;
    let _struct_pointer = parameters.pop_long()?;
    let _image_index = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("com.sun.imageio.plugins.jpeg.JPEGImageReader.readImage(IJ[BI[I[IIIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;IIZ)Z".to_string()).into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.readImageHeader(JZZ)Z",
    Any
)]
#[async_method]
pub async fn read_image_header<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _reset = parameters.pop_bool()?;
    let _clear_buffer = parameters.pop_bool()?;
    let _struct_pointer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.readImageHeader(JZZ)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.resetLibraryState(J)V",
    Any
)]
#[async_method]
pub async fn reset_library_state<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _struct_pointer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.resetLibraryState(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.resetReader(J)V", Any)]
#[async_method]
pub async fn reset_reader<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _struct_pointer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.resetReader(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.setOutColorSpace(JI)V",
    Any
)]
#[async_method]
pub async fn set_out_color_space<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_int()?;
    let _struct_pointer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.setOutColorSpace(JI)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.setSource(J)V", Any)]
#[async_method]
pub async fn set_source<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _struct_pointer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.setSource(J)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_abort_read() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = abort_read(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageReader.abortRead(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_clear_native_read_abort_flag() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            clear_native_read_abort_flag(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageReader.clearNativeReadAbortFlag(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_dispose_reader() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose_reader(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageReader.disposeReader(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_jpeg_image_reader() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_jpeg_image_reader(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageReader.initJPEGImageReader()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_reader_i_ds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_reader_i_ds(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageReader.initReaderIDs(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_read_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_image(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageReader.readImage(IJ[BI[I[IIIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;IIZ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_read_image_header() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_image_header(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageReader.readImageHeader(JZZ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_reset_library_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reset_library_state(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageReader.resetLibraryState(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_reset_reader() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reset_reader(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageReader.resetReader(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_out_color_space() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_out_color_space(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageReader.setOutColorSpace(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_source() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_source(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageReader.setSource(J)V",
            result.unwrap_err().to_string()
        );
    }
}
