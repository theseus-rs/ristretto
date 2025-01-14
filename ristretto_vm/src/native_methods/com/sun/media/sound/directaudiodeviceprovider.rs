use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.media.sound.DirectAudioDeviceProvider`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/media/sound/DirectAudioDeviceProvider";
    registry.register(class_name, "nGetNumDevices", "()I", n_get_num_devices);
    registry.register(
        class_name,
        "nNewDirectAudioDeviceInfo",
        "(I)Lcom/sun/media/sound/DirectAudioDeviceProvider$DirectAudioDeviceInfo;",
        n_new_direct_audio_device_info,
    );
}

#[async_recursion(?Send)]
async fn n_get_num_devices(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDeviceProvider.nGetNumDevices()I")
}

#[async_recursion(?Send)]
async fn n_new_direct_audio_device_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDeviceProvider.nNewDirectAudioDeviceInfo(I)Lcom/sun/media/sound/DirectAudioDeviceProvider$DirectAudioDeviceInfo;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/sun/media/sound/DirectAudioDeviceProvider";
        assert!(registry
            .method(class_name, "nGetNumDevices", "()I")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nNewDirectAudioDeviceInfo",
                "(I)Lcom/sun/media/sound/DirectAudioDeviceProvider$DirectAudioDeviceInfo;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDeviceProvider.nGetNumDevices()I"
    )]
    async fn test_n_get_num_devices() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_num_devices(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDeviceProvider.nNewDirectAudioDeviceInfo(I)Lcom/sun/media/sound/DirectAudioDeviceProvider$DirectAudioDeviceInfo;"
    )]
    async fn test_n_new_direct_audio_device_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_new_direct_audio_device_info(thread, Arguments::default()).await;
    }
}
