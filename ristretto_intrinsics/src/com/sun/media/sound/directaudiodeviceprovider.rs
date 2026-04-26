use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/sun/media/sound/DirectAudioDeviceProvider.nGetNumDevices()I", Any)]
#[async_method]
pub async fn n_get_num_devices<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.DirectAudioDeviceProvider.nGetNumDevices()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/media/sound/DirectAudioDeviceProvider.nNewDirectAudioDeviceInfo(I)Lcom/sun/media/sound/DirectAudioDeviceProvider$DirectAudioDeviceInfo;",
    Any
)]
#[async_method]
pub async fn n_new_direct_audio_device_info<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _device_index = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("com.sun.media.sound.DirectAudioDeviceProvider.nNewDirectAudioDeviceInfo(I)Lcom/sun/media/sound/DirectAudioDeviceProvider$DirectAudioDeviceInfo;".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_n_get_num_devices() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_num_devices(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDeviceProvider.nGetNumDevices()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_new_direct_audio_device_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            n_new_direct_audio_device_info(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com.sun.media.sound.DirectAudioDeviceProvider.nNewDirectAudioDeviceInfo(I)Lcom/sun/media/sound/DirectAudioDeviceProvider$DirectAudioDeviceInfo;",
            result.unwrap_err().to_string()
        );
    }
}
