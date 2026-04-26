use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.abortWrite(J)V", Any)]
#[async_method]
pub async fn abort_write<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _struct_pointer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageWriter.abortWrite(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.disposeWriter(J)V", Any)]
#[async_method]
pub async fn dispose_writer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _struct_pointer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageWriter.disposeWriter(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.initJPEGImageWriter()J",
    Any
)]
#[async_method]
pub async fn init_jpeg_image_writer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageWriter.initJPEGImageWriter()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.initWriterIDs(Ljava/lang/Class;Ljava/lang/Class;)V",
    Any
)]
#[async_method]
pub async fn init_writer_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _huff_class = parameters.pop_reference()?;
    let _q_table_class = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com.sun.imageio.plugins.jpeg.JPEGImageWriter.initWriterIDs(Ljava/lang/Class;Ljava/lang/Class;)V".to_string()).into())
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.resetWriter(J)V", Any)]
#[async_method]
pub async fn reset_writer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _struct_pointer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageWriter.resetWriter(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.setDest(J)V", Any)]
#[async_method]
pub async fn set_dest<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _struct_pointer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageWriter.setDest(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.writeImage(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z",
    Any
)]
#[async_method]
pub async fn write_image<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _restart_interval = parameters.pop_int()?;
    let _have_metadata = parameters.pop_bool()?;
    let _qtable_selectors = parameters.pop_reference()?;
    let _vsampling_factors = parameters.pop_reference()?;
    let _hsampling_factors = parameters.pop_reference()?;
    let _component_ids = parameters.pop_reference()?;
    let _scans = parameters.pop_reference()?;
    let _num_scans = parameters.pop_int()?;
    let _progressive = parameters.pop_bool()?;
    let _optimize_huffman = parameters.pop_bool()?;
    let _write_dht = parameters.pop_bool()?;
    let _ac_huffman_tables = parameters.pop_reference()?;
    let _dc_huffman_tables = parameters.pop_reference()?;
    let _write_dqt = parameters.pop_bool()?;
    let _qtables = parameters.pop_reference()?;
    let _step_y = parameters.pop_int()?;
    let _step_x = parameters.pop_int()?;
    let _dest_height = parameters.pop_int()?;
    let _dest_width = parameters.pop_int()?;
    let _src_width = parameters.pop_int()?;
    let _band_sizes = parameters.pop_reference()?;
    let _num_bands = parameters.pop_int()?;
    let _out_cs_type = parameters.pop_int()?;
    let _in_cs_type = parameters.pop_int()?;
    let _data = parameters.pop_reference()?;
    let _struct_pointer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeImage(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z".to_string()).into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.writeTables(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V",
    Any
)]
#[async_method]
pub async fn write_tables<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ac_huffman_tables = parameters.pop_reference()?;
    let _dc_huffman_tables = parameters.pop_reference()?;
    let _qtables = parameters.pop_reference()?;
    let _struct_pointer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeTables(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_abort_write() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = abort_write(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageWriter.abortWrite(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_dispose_writer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose_writer(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageWriter.disposeWriter(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_jpeg_image_writer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_jpeg_image_writer(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageWriter.initJPEGImageWriter()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_writer_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_writer_ids(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageWriter.initWriterIDs(Ljava/lang/Class;Ljava/lang/Class;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_reset_writer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reset_writer(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageWriter.resetWriter(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_dest() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_dest(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageWriter.setDest(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_write_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write_image(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::from(false),
                Value::Object(None),
                Value::Object(None),
                Value::from(false),
                Value::from(false),
                Value::from(false),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::from(false),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeImage(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_write_tables() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write_tables(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeTables(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V",
            result.unwrap_err().to_string()
        );
    }
}
