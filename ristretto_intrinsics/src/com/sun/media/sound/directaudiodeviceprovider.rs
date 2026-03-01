use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::{Object, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;

#[intrinsic_method("com/sun/media/sound/DirectAudioDeviceProvider.nGetNumDevices()I", Any)]
#[async_method]
pub async fn n_get_num_devices<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Report 1 device so that AudioSystem.getClip() can find a mixer
    Ok(Some(Value::Int(1)))
}

#[intrinsic_method(
    "com/sun/media/sound/DirectAudioDeviceProvider.nNewDirectAudioDeviceInfo(I)Lcom/sun/media/sound/DirectAudioDeviceProvider$DirectAudioDeviceInfo;",
    Any
)]
#[async_method]
pub async fn n_new_direct_audio_device_info<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let device_index = parameters.pop_int()?;

    let class = thread
        .class("com/sun/media/sound/DirectAudioDeviceProvider$DirectAudioDeviceInfo")
        .await?;
    let gc = thread.vm()?.garbage_collector().clone();
    let mut object = Object::new(class)?;
    object.set_value_unchecked("index", Value::Int(device_index))?;
    object.set_value_unchecked("deviceID", Value::Int(device_index))?;
    object.set_value_unchecked("maxSimulLines", Value::Int(32))?;
    let name = thread.intern_string("Ristretto Audio Device").await?;
    object.set_value_unchecked("name", name)?;
    let vendor = thread.intern_string("Ristretto").await?;
    object.set_value_unchecked("vendor", vendor)?;
    let description = thread.intern_string("Software audio output").await?;
    object.set_value_unchecked("description", description)?;
    let version = thread.intern_string("1.0").await?;
    object.set_value_unchecked("version", version)?;

    Ok(Some(Value::from_object(&gc, object)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_n_get_num_devices() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_num_devices(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_new_direct_audio_device_info() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Int(0));
        let result = n_new_direct_audio_device_info(thread, params).await?;
        assert!(result.is_some());
        Ok(())
    }
}
