use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("com/sun/media/sound/DirectAudioDeviceProvider.nGetNumDevices()I", Any)]
#[async_method]
pub(crate) async fn n_get_num_devices(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.DirectAudioDeviceProvider.nGetNumDevices()I")
}

#[intrinsic_method(
    "com/sun/media/sound/DirectAudioDeviceProvider.nNewDirectAudioDeviceInfo(I)Lcom/sun/media/sound/DirectAudioDeviceProvider$DirectAudioDeviceInfo;",
    Any
)]
#[async_method]
pub(crate) async fn n_new_direct_audio_device_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.media.sound.DirectAudioDeviceProvider.nNewDirectAudioDeviceInfo(I)Lcom/sun/media/sound/DirectAudioDeviceProvider$DirectAudioDeviceInfo;"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDeviceProvider.nGetNumDevices()I"
    )]
    async fn test_n_get_num_devices() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_num_devices(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.DirectAudioDeviceProvider.nNewDirectAudioDeviceInfo(I)Lcom/sun/media/sound/DirectAudioDeviceProvider$DirectAudioDeviceInfo;"
    )]
    async fn test_n_new_direct_audio_device_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_new_direct_audio_device_info(thread, Parameters::default()).await;
    }
}
