use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.media.sound.MidiOutDevice`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/media/sound/MidiOutDevice";
    registry.register(class_name, "nClose", "(J)V", n_close);
    registry.register(class_name, "nGetTimeStamp", "(J)J", n_get_time_stamp);
    registry.register(class_name, "nOpen", "(I)J", n_open);
    registry.register(
        class_name,
        "nSendLongMessage",
        "(J[BIJ)V",
        n_send_long_message,
    );
    registry.register(
        class_name,
        "nSendShortMessage",
        "(JIJ)V",
        n_send_short_message,
    );
}

#[async_recursion(?Send)]
async fn n_close(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiOutDevice.nClose(J)V")
}

#[async_recursion(?Send)]
async fn n_get_time_stamp(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiOutDevice.nGetTimeStamp(J)J")
}

#[async_recursion(?Send)]
async fn n_open(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiOutDevice.nOpen(I)J")
}

#[async_recursion(?Send)]
async fn n_send_long_message(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiOutDevice.nSendLongMessage(J[BIJ)V")
}

#[async_recursion(?Send)]
async fn n_send_short_message(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiOutDevice.nSendShortMessage(JIJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/sun/media/sound/MidiOutDevice";
        assert!(registry.method(class_name, "nClose", "(J)V").is_some());
        assert!(registry
            .method(class_name, "nGetTimeStamp", "(J)J")
            .is_some());
        assert!(registry.method(class_name, "nOpen", "(I)J").is_some());
        assert!(registry
            .method(class_name, "nSendLongMessage", "(J[BIJ)V")
            .is_some());
        assert!(registry
            .method(class_name, "nSendShortMessage", "(JIJ)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.MidiOutDevice.nClose(J)V")]
    async fn test_n_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_close(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiOutDevice.nGetTimeStamp(J)J"
    )]
    async fn test_n_get_time_stamp() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_time_stamp(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.MidiOutDevice.nOpen(I)J")]
    async fn test_n_open() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_open(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiOutDevice.nSendLongMessage(J[BIJ)V"
    )]
    async fn test_n_send_long_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_send_long_message(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiOutDevice.nSendShortMessage(JIJ)V"
    )]
    async fn test_n_send_short_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_send_short_message(thread, Arguments::default()).await;
    }
}
