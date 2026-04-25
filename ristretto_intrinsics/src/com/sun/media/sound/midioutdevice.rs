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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("com.sun.media.sound.MidiOutDevice.nClose(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("com/sun/media/sound/MidiOutDevice.nGetTimeStamp(J)J", Any)]
#[async_method]
pub async fn n_get_time_stamp<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.MidiOutDevice.nGetTimeStamp(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/MidiOutDevice.nOpen(I)J", Any)]
#[async_method]
pub async fn n_open<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("com.sun.media.sound.MidiOutDevice.nOpen(I)J".to_string())
            .into(),
    )
}

#[intrinsic_method("com/sun/media/sound/MidiOutDevice.nSendLongMessage(J[BIJ)V", Any)]
#[async_method]
pub async fn n_send_long_message<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _time_stamp = parameters.pop_long()?;
    let _size = parameters.pop_int()?;
    let _data = parameters.pop_reference()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.MidiOutDevice.nSendLongMessage(J[BIJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/MidiOutDevice.nSendShortMessage(JIJ)V", Any)]
#[async_method]
pub async fn n_send_short_message<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _time_stamp = parameters.pop_long()?;
    let _packed_msg = parameters.pop_int()?;
    let _id = parameters.pop_long()?;
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
        let result = n_close(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.media.sound.MidiOutDevice.nClose(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_get_time_stamp() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_time_stamp(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.media.sound.MidiOutDevice.nGetTimeStamp(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_open() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_open(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com.sun.media.sound.MidiOutDevice.nOpen(I)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_send_long_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_send_long_message(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.MidiOutDevice.nSendLongMessage(J[BIJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_send_short_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_send_short_message(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.MidiOutDevice.nSendShortMessage(JIJ)V",
            result.unwrap_err().to_string()
        );
    }
}
