use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nClose(J)V", Any)]
#[async_method]
pub async fn n_close<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDevice.nClose(J)V")
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nGetMessages(J)V", Any)]
#[async_method]
pub async fn n_get_messages<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDevice.nGetMessages(J)V")
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nGetTimeStamp(J)J", Any)]
#[async_method]
pub async fn n_get_time_stamp<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDevice.nGetTimeStamp(J)J")
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nOpen(I)J", Any)]
#[async_method]
pub async fn n_open<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDevice.nOpen(I)J")
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nStart(J)V", Any)]
#[async_method]
pub async fn n_start<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDevice.nStart(J)V")
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nStop(J)V", Any)]
#[async_method]
pub async fn n_stop<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDevice.nStop(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.MidiInDevice.nClose(J)V")]
    async fn test_n_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_close(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiInDevice.nGetMessages(J)V"
    )]
    async fn test_n_get_messages() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_messages(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiInDevice.nGetTimeStamp(J)J"
    )]
    async fn test_n_get_time_stamp() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_time_stamp(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.MidiInDevice.nOpen(I)J")]
    async fn test_n_open() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_open(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.MidiInDevice.nStart(J)V")]
    async fn test_n_start() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_start(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.MidiInDevice.nStop(J)V")]
    async fn test_n_stop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_stop(thread, Parameters::default()).await;
    }
}
