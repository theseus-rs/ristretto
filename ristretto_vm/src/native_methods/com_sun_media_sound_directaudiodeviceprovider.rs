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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_get_num_devices(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_new_direct_audio_device_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
