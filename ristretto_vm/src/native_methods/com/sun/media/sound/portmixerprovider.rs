use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.media.sound.PortMixerProvider`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/media/sound/PortMixerProvider";
    registry.register(class_name, "nGetNumDevices", "()I", n_get_num_devices);
    registry.register(
        class_name,
        "nNewPortMixerInfo",
        "(I)Lcom/sun/media/sound/PortMixerProvider$PortMixerInfo;",
        n_new_port_mixer_info,
    );
}

#[async_recursion(?Send)]
async fn n_get_num_devices(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixerProvider.nGetNumDevices()I")
}

#[async_recursion(?Send)]
async fn n_new_port_mixer_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixerProvider.nNewPortMixerInfo(I)Lcom/sun/media/sound/PortMixerProvider$PortMixerInfo;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/sun/media/sound/PortMixerProvider";
        assert!(registry
            .method(class_name, "nGetNumDevices", "()I")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nNewPortMixerInfo",
                "(I)Lcom/sun/media/sound/PortMixerProvider$PortMixerInfo;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixerProvider.nGetNumDevices()I"
    )]
    async fn test_n_get_num_devices() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_num_devices(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixerProvider.nNewPortMixerInfo(I)Lcom/sun/media/sound/PortMixerProvider$PortMixerInfo;"
    )]
    async fn test_n_new_port_mixer_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_new_port_mixer_info(thread, Arguments::default()).await;
    }
}
