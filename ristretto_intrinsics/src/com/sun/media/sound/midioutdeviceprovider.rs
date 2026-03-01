use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{JavaObject, Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/media/sound/MidiOutDeviceProvider.nGetDescription(I)Ljava/lang/String;",
    Any
)]
#[async_method]
#[cfg_attr(not(target_family = "wasm"), expect(clippy::cast_sign_loss))]
pub async fn n_get_description<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    #[cfg(not(target_family = "wasm"))]
    {
        if let Some(name) = super::audio_state::get_midi_out_port_name(index as usize) {
            return Ok(Some(name.to_object(&thread).await?));
        }
        Ok(Some(Value::Object(None)))
    }
    #[cfg(target_family = "wasm")]
    {
        let _ = index;
        Ok(Some("Web MIDI Output".to_object(&thread).await?))
    }
}

#[intrinsic_method(
    "com/sun/media/sound/MidiOutDeviceProvider.nGetName(I)Ljava/lang/String;",
    Any
)]
#[async_method]
#[cfg_attr(not(target_family = "wasm"), expect(clippy::cast_sign_loss))]
pub async fn n_get_name<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    #[cfg(not(target_family = "wasm"))]
    {
        if let Some(name) = super::audio_state::get_midi_out_port_name(index as usize) {
            return Ok(Some(name.to_object(&thread).await?));
        }
        Ok(Some(Value::Object(None)))
    }
    #[cfg(target_family = "wasm")]
    {
        let _ = index;
        Ok(Some("Web MIDI Output".to_object(&thread).await?))
    }
}

#[intrinsic_method("com/sun/media/sound/MidiOutDeviceProvider.nGetNumDevices()I", Any)]
#[async_method]
#[cfg_attr(
    not(target_family = "wasm"),
    expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)
)]
pub async fn n_get_num_devices<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
        let count = super::audio_state::get_midi_out_port_count() as i32;
        Ok(Some(Value::Int(count)))
    }
    #[cfg(target_family = "wasm")]
    {
        Ok(Some(Value::Int(0)))
    }
}

#[intrinsic_method(
    "com/sun/media/sound/MidiOutDeviceProvider.nGetVendor(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn n_get_vendor<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
        return Ok(Some("midir".to_object(&thread).await?));
    }
    #[cfg(target_family = "wasm")]
    {
        return Ok(Some("Web MIDI API".to_object(&thread).await?));
    }
}

#[intrinsic_method(
    "com/sun/media/sound/MidiOutDeviceProvider.nGetVersion(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn n_get_version<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
        return Ok(Some("0.10".to_object(&thread).await?));
    }
    #[cfg(target_family = "wasm")]
    {
        return Ok(Some("1.0".to_object(&thread).await?));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_n_get_num_devices() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_num_devices(thread, Parameters::default()).await?;
        match result {
            Some(Value::Int(count)) => assert!(count >= 0),
            _ => panic!("Expected Value::Int"),
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_n_get_description() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Int(0));
        let result = n_get_description(thread, params).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_n_get_name() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Int(0));
        let result = n_get_name(thread, params).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_n_get_vendor() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Int(0));
        let result = n_get_vendor(thread, params).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_n_get_version() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Int(0));
        let result = n_get_version(thread, params).await?;
        assert!(result.is_some());
        Ok(())
    }
}
