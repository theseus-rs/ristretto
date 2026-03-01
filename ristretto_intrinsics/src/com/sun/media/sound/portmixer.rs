use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{JavaObject, Parameters, Result, VM};
use std::sync::Arc;

#[intrinsic_method("com/sun/media/sound/PortMixer.nClose(J)V", Any)]
#[async_method]
pub async fn n_close<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    audio.port_mixer_handles().write().remove(&handle_id);
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nControlGetFloatValue(J)F", Any)]
#[async_method]
pub async fn n_control_get_float_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _control_id = parameters.pop_long()?;
    // Default volume is 1.0 (full volume)
    Ok(Some(Value::Float(1.0)))
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nControlGetIntValue(J)I", Any)]
#[async_method]
pub async fn n_control_get_int_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _control_id = parameters.pop_long()?;
    // Default mute state: 0 (not muted)
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nControlSetFloatValue(JF)V", Any)]
#[async_method]
pub async fn n_control_set_float_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_float()?;
    let _control_id = parameters.pop_long()?;
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nControlSetIntValue(JI)V", Any)]
#[async_method]
pub async fn n_control_set_int_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_int()?;
    let _control_id = parameters.pop_long()?;
    Ok(None)
}

#[intrinsic_method(
    "com/sun/media/sound/PortMixer.nGetControls(JILjava/util/Vector;)V",
    Any
)]
#[async_method]
pub async fn n_get_controls<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Controls (volume, mute, etc.) would be added to the Vector parameter.
    // For now, leave the vector empty — the Java side handles no-controls gracefully.
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nGetPortCount(J)I", Any)]
#[async_method]
pub async fn n_get_port_count<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    let guard = audio.port_mixer_handles().read();
    if guard.contains_key(&handle_id) {
        return Ok(Some(Value::Int(2)));
    }
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "com/sun/media/sound/PortMixer.nGetPortName(JI)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn n_get_port_name<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let port_index = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let name = {
        let audio = thread
            .vm()?
            .extensions()
            .get_or_init(super::audio_state::AudioState::new);
        let guard = audio.port_mixer_handles().read();
        guard.get(&handle_id).map(|handle| {
            if port_index == 0 {
                format!("{} Source", handle.name)
            } else {
                format!("{} Target", handle.name)
            }
        })
    };
    if let Some(name) = name {
        return Ok(Some(name.to_object(&thread).await?));
    }
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nGetPortType(JI)I", Any)]
#[async_method]
pub async fn n_get_port_type<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let port_index = parameters.pop_int()?;
    let _handle_id = parameters.pop_long()?;
    // Port types: 1 = SPEAKER, 2 = HEADPHONE, 3 = LINE_OUT, 4 = COMPACT_DISC, 5 = LINE_IN, 6 = MICROPHONE
    let port_type = if port_index == 0 { 1 } else { 6 }; // Source=SPEAKER, Target=MICROPHONE
    Ok(Some(Value::Int(port_type)))
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nOpen(I)J", Any)]
#[async_method]
#[cfg_attr(not(target_family = "wasm"), expect(clippy::cast_sign_loss))]
pub async fn n_open<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;

    #[cfg(not(target_family = "wasm"))]
    {
        use cpal::traits::DeviceTrait;
        if let Some(device) = super::audio_state::get_audio_device_by_index(index as usize) {
            let name = device.name().unwrap_or_else(|_| format!("Port {index}"));
            let audio = thread
                .vm()?
                .extensions()
                .get_or_init(super::audio_state::AudioState::new);
            let handle_id = audio.next_port_mixer_id();
            let mixer_handle = super::audio_state::PortMixerHandle {
                name,
                volume: 1.0,
                mute: false,
            };

            audio
                .port_mixer_handles()
                .write()
                .insert(handle_id, mixer_handle);

            return Ok(Some(Value::Long(handle_id)));
        }
        Ok(Some(Value::Long(0)))
    }
    #[cfg(target_family = "wasm")]
    {
        let name = format!("Web Audio Port {index}");
        let audio = thread
            .vm()?
            .extensions()
            .get_or_init(super::audio_state::AudioState::new);
        let handle_id = audio.next_port_mixer_id();
        let mixer_handle = super::audio_state::PortMixerHandle {
            name,
            volume: 1.0,
            mute: false,
        };

        audio
            .port_mixer_handles()
            .write()
            .insert(handle_id, mixer_handle);

        Ok(Some(Value::Long(handle_id)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_n_close() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        let result = n_close(thread, params).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_control_get_float_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        let result = n_control_get_float_value(thread, params).await?;
        assert_eq!(result, Some(Value::Float(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_control_get_int_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        let result = n_control_get_int_value(thread, params).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_control_set_float_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Float(0.5));
        let result = n_control_set_float_value(thread, params).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_control_set_int_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1));
        let result = n_control_set_int_value(thread, params).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_get_controls() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_controls(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_get_port_count() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        let result = n_get_port_count(thread, params).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_get_port_name() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(0));
        let result = n_get_port_name(thread, params).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_get_port_type() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(0));
        let result = n_get_port_type(thread, params).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_open() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Int(0));
        let result = n_open(thread, params).await?;
        assert!(result.is_some());
        Ok(())
    }
}
