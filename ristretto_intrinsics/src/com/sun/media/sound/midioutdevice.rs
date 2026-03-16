use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/sun/media/sound/MidiOutDevice.nClose(J)V", Any)]
#[async_method]
pub async fn n_close<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("com.sun.media.sound.MidiOutDevice.nClose(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("com/sun/media/sound/MidiOutDevice.nGetTimeStamp(J)J", Any)]
#[async_method]
pub async fn n_get_time_stamp<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.MidiOutDevice.nGetTimeStamp(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/MidiOutDevice.nOpen(I)J", Any)]
#[async_method]
pub async fn n_open<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("com.sun.media.sound.MidiOutDevice.nOpen(I)J".to_string())
            .into(),
    )
}

#[intrinsic_method("com/sun/media/sound/MidiOutDevice.nSendLongMessage(J[BIJ)V", Any)]
#[async_method]
pub async fn n_send_long_message<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.MidiOutDevice.nSendLongMessage(J[BIJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/MidiOutDevice.nSendShortMessage(JIJ)V", Any)]
#[async_method]
pub async fn n_send_short_message<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.MidiOutDevice.nSendShortMessage(JIJ)V".to_string(),
    )
    .into())
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
    async fn test_n_send_long_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_send_long_message(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_n_send_short_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_send_short_message(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
