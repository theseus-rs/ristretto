use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nClose(J)V", Any)]
#[async_method]
pub async fn n_close<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("com.sun.media.sound.MidiInDevice.nClose(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nGetMessages(J)V", Any)]
#[async_method]
pub async fn n_get_messages<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.MidiInDevice.nGetMessages(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nGetTimeStamp(J)J", Any)]
#[async_method]
pub async fn n_get_time_stamp<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.MidiInDevice.nGetTimeStamp(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nOpen(I)J", Any)]
#[async_method]
pub async fn n_open<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("com.sun.media.sound.MidiInDevice.nOpen(I)J".to_string())
            .into(),
    )
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nStart(J)V", Any)]
#[async_method]
pub async fn n_start<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("com.sun.media.sound.MidiInDevice.nStart(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nStop(J)V", Any)]
#[async_method]
pub async fn n_stop<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("com.sun.media.sound.MidiInDevice.nStop(J)V".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_n_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_close(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_n_get_messages() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_messages(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_n_get_time_stamp() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_time_stamp(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_n_open() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_open(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_n_start() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_start(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_n_stop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_stop(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
