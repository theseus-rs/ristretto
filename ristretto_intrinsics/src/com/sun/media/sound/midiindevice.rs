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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("com.sun.media.sound.MidiInDevice.nClose(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nGetMessages(J)V", Any)]
#[async_method]
pub async fn n_get_messages<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.MidiInDevice.nGetMessages(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nGetTimeStamp(J)J", Any)]
#[async_method]
pub async fn n_get_time_stamp<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.MidiInDevice.nGetTimeStamp(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nOpen(I)J", Any)]
#[async_method]
pub async fn n_open<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("com.sun.media.sound.MidiInDevice.nOpen(I)J".to_string())
            .into(),
    )
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nStart(J)V", Any)]
#[async_method]
pub async fn n_start<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("com.sun.media.sound.MidiInDevice.nStart(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nStop(J)V", Any)]
#[async_method]
pub async fn n_stop<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
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
        let result = n_close(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.media.sound.MidiInDevice.nClose(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_get_messages() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_messages(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.media.sound.MidiInDevice.nGetMessages(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_get_time_stamp() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_time_stamp(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.media.sound.MidiInDevice.nGetTimeStamp(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_open() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_open(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com.sun.media.sound.MidiInDevice.nOpen(I)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_start() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_start(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.media.sound.MidiInDevice.nStart(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_stop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_stop(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.media.sound.MidiInDevice.nStop(J)V",
            result.unwrap_err().to_string()
        );
    }
}
