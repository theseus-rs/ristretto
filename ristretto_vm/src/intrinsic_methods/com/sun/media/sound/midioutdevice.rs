use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/sun/media/sound/MidiOutDevice";

/// Register all intrinsic methods for `com.sun.media.sound.MidiOutDevice`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "nClose", "(J)V", n_close);
    registry.register(CLASS_NAME, "nGetTimeStamp", "(J)J", n_get_time_stamp);
    registry.register(CLASS_NAME, "nOpen", "(I)J", n_open);
    registry.register(
        CLASS_NAME,
        "nSendLongMessage",
        "(J[BIJ)V",
        n_send_long_message,
    );
    registry.register(
        CLASS_NAME,
        "nSendShortMessage",
        "(JIJ)V",
        n_send_short_message,
    );
}

#[async_recursion(?Send)]
async fn n_close(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiOutDevice.nClose(J)V")
}

#[async_recursion(?Send)]
async fn n_get_time_stamp(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiOutDevice.nGetTimeStamp(J)J")
}

#[async_recursion(?Send)]
async fn n_open(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiOutDevice.nOpen(I)J")
}

#[async_recursion(?Send)]
async fn n_send_long_message(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiOutDevice.nSendLongMessage(J[BIJ)V")
}

#[async_recursion(?Send)]
async fn n_send_short_message(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiOutDevice.nSendShortMessage(JIJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.MidiOutDevice.nClose(J)V")]
    async fn test_n_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_close(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiOutDevice.nGetTimeStamp(J)J"
    )]
    async fn test_n_get_time_stamp() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_time_stamp(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.MidiOutDevice.nOpen(I)J")]
    async fn test_n_open() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_open(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiOutDevice.nSendLongMessage(J[BIJ)V"
    )]
    async fn test_n_send_long_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_send_long_message(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiOutDevice.nSendShortMessage(JIJ)V"
    )]
    async fn test_n_send_short_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_send_short_message(thread, Parameters::default()).await;
    }
}
