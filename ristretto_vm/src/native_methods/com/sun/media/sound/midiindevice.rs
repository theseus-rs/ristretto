use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/sun/media/sound/MidiInDevice";

/// Register all native methods for `com.sun.media.sound.MidiInDevice`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "nClose", "(J)V", n_close);
    registry.register(CLASS_NAME, "nGetMessages", "(J)V", n_get_messages);
    registry.register(CLASS_NAME, "nGetTimeStamp", "(J)J", n_get_time_stamp);
    registry.register(CLASS_NAME, "nOpen", "(I)J", n_open);
    registry.register(CLASS_NAME, "nStart", "(J)V", n_start);
    registry.register(CLASS_NAME, "nStop", "(J)V", n_stop);
}

#[async_recursion(?Send)]
async fn n_close(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDevice.nClose(J)V")
}

#[async_recursion(?Send)]
async fn n_get_messages(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDevice.nGetMessages(J)V")
}

#[async_recursion(?Send)]
async fn n_get_time_stamp(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDevice.nGetTimeStamp(J)J")
}

#[async_recursion(?Send)]
async fn n_open(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDevice.nOpen(I)J")
}

#[async_recursion(?Send)]
async fn n_start(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDevice.nStart(J)V")
}

#[async_recursion(?Send)]
async fn n_stop(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
