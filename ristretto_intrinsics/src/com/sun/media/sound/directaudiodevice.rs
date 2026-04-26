use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nAvailable(JZ)I", Any)]
#[async_method]
pub async fn n_available<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_bool()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDevice.nAvailable(JZ)I".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nClose(JZ)V", Any)]
#[async_method]
pub async fn n_close<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_bool()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDevice.nClose(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nFlush(JZ)V", Any)]
#[async_method]
pub async fn n_flush<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_bool()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDevice.nFlush(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nGetBufferSize(JZ)I", Any)]
#[async_method]
pub async fn n_get_buffer_size<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_bool()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDevice.nGetBufferSize(JZ)I".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nGetBytePosition(JZJ)J", Any)]
#[async_method]
pub async fn n_get_byte_position<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _java_pos = parameters.pop_long()?;
    let _is_source = parameters.pop_bool()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDevice.nGetBytePosition(JZJ)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/media/sound/DirectAudioDevice.nGetFormats(IIZLjava/util/Vector;)V",
    Any
)]
#[async_method]
pub async fn n_get_formats<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _formats = parameters.pop_reference()?;
    let _is_source = parameters.pop_bool()?;
    let _device_id = parameters.pop_int()?;
    let _mixer_index = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDevice.nGetFormats(IIZLjava/util/Vector;)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nIsStillDraining(JZ)Z", Any)]
#[async_method]
pub async fn n_is_still_draining<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_bool()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDevice.nIsStillDraining(JZ)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nOpen(IIZIFIIIZZI)J", Any)]
#[async_method]
pub async fn n_open<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buffer_size = parameters.pop_int()?;
    let _big_endian = parameters.pop_bool()?;
    let _signed = parameters.pop_bool()?;
    let _channels = parameters.pop_int()?;
    let _frame_size = parameters.pop_int()?;
    let _sample_size_in_bits = parameters.pop_int()?;
    let _sample_rate = parameters.pop_float()?;
    let _encoding = parameters.pop_int()?;
    let _is_source = parameters.pop_bool()?;
    let _device_id = parameters.pop_int()?;
    let _mixer_index = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDevice.nOpen(IIZIFIIIZZI)J".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nRead(J[BIII)I", Any)]
#[async_method]
pub async fn n_read<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _conversion_size = parameters.pop_int()?;
    let _len = parameters.pop_int()?;
    let _off = parameters.pop_int()?;
    let _b = parameters.pop_reference()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDevice.nRead(J[BIII)I".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nRequiresServicing(JZ)Z", Any)]
#[async_method]
pub async fn n_requires_servicing<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_bool()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDevice.nRequiresServicing(JZ)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nService(JZ)V", Any)]
#[async_method]
pub async fn n_service<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_bool()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDevice.nService(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nSetBytePosition(JZJ)V", Any)]
#[async_method]
pub async fn n_set_byte_position<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pos = parameters.pop_long()?;
    let _is_source = parameters.pop_bool()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDevice.nSetBytePosition(JZJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nStart(JZ)V", Any)]
#[async_method]
pub async fn n_start<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_bool()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDevice.nStart(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nStop(JZ)V", Any)]
#[async_method]
pub async fn n_stop<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_bool()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDevice.nStop(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nWrite(J[BIIIFF)I", Any)]
#[async_method]
pub async fn n_write<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _vol_right = parameters.pop_float()?;
    let _vol_left = parameters.pop_float()?;
    let _conversion_size = parameters.pop_int()?;
    let _len = parameters.pop_int()?;
    let _off = parameters.pop_int()?;
    let _b = parameters.pop_reference()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDevice.nWrite(J[BIIIFF)I".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_n_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_available(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDevice.nAvailable(JZ)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_close(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDevice.nClose(JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_flush() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_flush(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDevice.nFlush(JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_get_buffer_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_buffer_size(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDevice.nGetBufferSize(JZ)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_get_byte_position() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_byte_position(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDevice.nGetBytePosition(JZJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_get_formats() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_formats(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDevice.nGetFormats(IIZLjava/util/Vector;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_is_still_draining() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_is_still_draining(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDevice.nIsStillDraining(JZ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_open() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_open(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
                Value::Int(0),
                Value::Float(0.0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
                Value::from(false),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDevice.nOpen(IIZIFIIIZZI)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_read() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_read(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDevice.nRead(J[BIII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_requires_servicing() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_requires_servicing(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDevice.nRequiresServicing(JZ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_service() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_service(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDevice.nService(JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_set_byte_position() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_set_byte_position(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDevice.nSetBytePosition(JZJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_start() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_start(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDevice.nStart(JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_stop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_stop(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDevice.nStop(JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_write() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_write(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Float(0.0),
                Value::Float(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDevice.nWrite(J[BIIIFF)I",
            result.unwrap_err().to_string()
        );
    }
}
