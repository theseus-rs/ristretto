use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.media.sound.DirectAudioDevice`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/media/sound/DirectAudioDevice";
    registry.register(class_name, "nAvailable", "(JZ)I", n_available);
    registry.register(class_name, "nClose", "(JZ)V", n_close);
    registry.register(class_name, "nFlush", "(JZ)V", n_flush);
    registry.register(class_name, "nGetBufferSize", "(JZ)I", n_get_buffer_size);
    registry.register(
        class_name,
        "nGetBytePosition",
        "(JZJ)J",
        n_get_byte_position,
    );
    registry.register(
        class_name,
        "nGetFormats",
        "(IIZLjava/util/Vector;)V",
        n_get_formats,
    );
    registry.register(class_name, "nIsStillDraining", "(JZ)Z", n_is_still_draining);
    registry.register(class_name, "nOpen", "(IIZIFIIIZZI)J", n_open);
    registry.register(class_name, "nRead", "(J[BIII)I", n_read);
    registry.register(
        class_name,
        "nRequiresServicing",
        "(JZ)Z",
        n_requires_servicing,
    );
    registry.register(class_name, "nService", "(JZ)V", n_service);
    registry.register(
        class_name,
        "nSetBytePosition",
        "(JZJ)V",
        n_set_byte_position,
    );
    registry.register(class_name, "nStart", "(JZ)V", n_start);
    registry.register(class_name, "nStop", "(JZ)V", n_stop);
    registry.register(class_name, "nWrite", "(J[BIIIFF)I", n_write);
}

#[async_recursion(?Send)]
async fn n_available(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nAvailable(JZ)I")
}

#[async_recursion(?Send)]
async fn n_close(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nClose(JZ)V")
}

#[async_recursion(?Send)]
async fn n_flush(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nFlush(JZ)V")
}

#[async_recursion(?Send)]
async fn n_get_buffer_size(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nGetBufferSize(JZ)I")
}

#[async_recursion(?Send)]
async fn n_get_byte_position(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nGetBytePosition(JZJ)J")
}

#[async_recursion(?Send)]
async fn n_get_formats(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nGetFormats(IIZLjava/util/Vector;)V")
}

#[async_recursion(?Send)]
async fn n_is_still_draining(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nIsStillDraining(JZ)Z")
}

#[async_recursion(?Send)]
async fn n_open(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nOpen(IIZIFIIIZZI)J")
}

#[async_recursion(?Send)]
async fn n_read(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nRead(J[BIII)I")
}

#[async_recursion(?Send)]
async fn n_requires_servicing(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nRequiresServicing(JZ)Z")
}

#[async_recursion(?Send)]
async fn n_service(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nService(JZ)V")
}

#[async_recursion(?Send)]
async fn n_set_byte_position(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nSetBytePosition(JZJ)V")
}

#[async_recursion(?Send)]
async fn n_start(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nStart(JZ)V")
}

#[async_recursion(?Send)]
async fn n_stop(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nStop(JZ)V")
}

#[async_recursion(?Send)]
async fn n_write(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDevice.nWrite(J[BIIIFF)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/sun/media/sound/DirectAudioDevice";
        assert!(registry.method(class_name, "nAvailable", "(JZ)I").is_some());
        assert!(registry.method(class_name, "nClose", "(JZ)V").is_some());
        assert!(registry.method(class_name, "nFlush", "(JZ)V").is_some());
        assert!(registry
            .method(class_name, "nGetBufferSize", "(JZ)I")
            .is_some());
        assert!(registry
            .method(class_name, "nGetBytePosition", "(JZJ)J")
            .is_some());
        assert!(registry
            .method(class_name, "nGetFormats", "(IIZLjava/util/Vector;)V")
            .is_some());
        assert!(registry
            .method(class_name, "nIsStillDraining", "(JZ)Z")
            .is_some());
        assert!(registry
            .method(class_name, "nOpen", "(IIZIFIIIZZI)J")
            .is_some());
        assert!(registry.method(class_name, "nRead", "(J[BIII)I").is_some());
        assert!(registry
            .method(class_name, "nRequiresServicing", "(JZ)Z")
            .is_some());
        assert!(registry.method(class_name, "nService", "(JZ)V").is_some());
        assert!(registry
            .method(class_name, "nSetBytePosition", "(JZJ)V")
            .is_some());
        assert!(registry.method(class_name, "nStart", "(JZ)V").is_some());
        assert!(registry.method(class_name, "nStop", "(JZ)V").is_some());
        assert!(registry
            .method(class_name, "nWrite", "(J[BIIIFF)I")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nAvailable(JZ)I"
    )]
    async fn test_n_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_available(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nClose(JZ)V"
    )]
    async fn test_n_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_close(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nFlush(JZ)V"
    )]
    async fn test_n_flush() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_flush(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nGetBufferSize(JZ)I"
    )]
    async fn test_n_get_buffer_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_buffer_size(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nGetBytePosition(JZJ)J"
    )]
    async fn test_n_get_byte_position() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_byte_position(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nGetFormats(IIZLjava/util/Vector;)V"
    )]
    async fn test_n_get_formats() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_formats(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nIsStillDraining(JZ)Z"
    )]
    async fn test_n_is_still_draining() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_is_still_draining(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nOpen(IIZIFIIIZZI)J"
    )]
    async fn test_n_open() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_open(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nRead(J[BIII)I"
    )]
    async fn test_n_read() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_read(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nRequiresServicing(JZ)Z"
    )]
    async fn test_n_requires_servicing() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_requires_servicing(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nService(JZ)V"
    )]
    async fn test_n_service() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_service(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nSetBytePosition(JZJ)V"
    )]
    async fn test_n_set_byte_position() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_set_byte_position(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nStart(JZ)V"
    )]
    async fn test_n_start() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_start(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nStop(JZ)V"
    )]
    async fn test_n_stop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_stop(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDevice.nWrite(J[BIIIFF)I"
    )]
    async fn test_n_write() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_write(thread, Arguments::default()).await;
    }
}
