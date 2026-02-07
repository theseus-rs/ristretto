use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nAvailable(JZ)I", Any)]
#[async_method]
pub async fn n_available<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nAvailable(JZ)I")
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nClose(JZ)V", Any)]
#[async_method]
pub async fn n_close<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nClose(JZ)V")
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nFlush(JZ)V", Any)]
#[async_method]
pub async fn n_flush<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nFlush(JZ)V")
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nGetBufferSize(JZ)I", Any)]
#[async_method]
pub async fn n_get_buffer_size<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nGetBufferSize(JZ)I")
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nGetBytePosition(JZJ)J", Any)]
#[async_method]
pub async fn n_get_byte_position<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nGetBytePosition(JZJ)J")
}

#[intrinsic_method(
    "com/sun/media/sound/DirectAudioDevice.nGetFormats(IIZLjava/util/Vector;)V",
    Any
)]
#[async_method]
pub async fn n_get_formats<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nGetFormats(IIZLjava/util/Vector;)V")
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nIsStillDraining(JZ)Z", Any)]
#[async_method]
pub async fn n_is_still_draining<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nIsStillDraining(JZ)Z")
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nOpen(IIZIFIIIZZI)J", Any)]
#[async_method]
pub async fn n_open<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nOpen(IIZIFIIIZZI)J")
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nRead(J[BIII)I", Any)]
#[async_method]
pub async fn n_read<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nRead(J[BIII)I")
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nRequiresServicing(JZ)Z", Any)]
#[async_method]
pub async fn n_requires_servicing<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nRequiresServicing(JZ)Z")
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nService(JZ)V", Any)]
#[async_method]
pub async fn n_service<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nService(JZ)V")
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nSetBytePosition(JZJ)V", Any)]
#[async_method]
pub async fn n_set_byte_position<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nSetBytePosition(JZJ)V")
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nStart(JZ)V", Any)]
#[async_method]
pub async fn n_start<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nStart(JZ)V")
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nStop(JZ)V", Any)]
#[async_method]
pub async fn n_stop<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nStop(JZ)V")
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nWrite(J[BIIIFF)I", Any)]
#[async_method]
pub async fn n_write<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nWrite(J[BIIIFF)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nAvailable(JZ)I"
    )]
    async fn test_n_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_available(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nClose(JZ)V"
    )]
    async fn test_n_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_close(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nFlush(JZ)V"
    )]
    async fn test_n_flush() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_flush(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nGetBufferSize(JZ)I"
    )]
    async fn test_n_get_buffer_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_buffer_size(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nGetBytePosition(JZJ)J"
    )]
    async fn test_n_get_byte_position() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_byte_position(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nGetFormats(IIZLjava/util/Vector;)V"
    )]
    async fn test_n_get_formats() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_formats(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nIsStillDraining(JZ)Z"
    )]
    async fn test_n_is_still_draining() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_is_still_draining(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nOpen(IIZIFIIIZZI)J"
    )]
    async fn test_n_open() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_open(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nRead(J[BIII)I"
    )]
    async fn test_n_read() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_read(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nRequiresServicing(JZ)Z"
    )]
    async fn test_n_requires_servicing() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_requires_servicing(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nService(JZ)V"
    )]
    async fn test_n_service() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_service(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nSetBytePosition(JZJ)V"
    )]
    async fn test_n_set_byte_position() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_set_byte_position(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nStart(JZ)V"
    )]
    async fn test_n_start() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_start(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nStop(JZ)V"
    )]
    async fn test_n_stop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_stop(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nWrite(J[BIIIFF)I"
    )]
    async fn test_n_write() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_write(thread, Parameters::default()).await;
    }
}
