use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;

#[intrinsic_method("com/sun/media/sound/MidiOutDevice.nClose(J)V", Any)]
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
    audio.midi_out_handles().write().remove(&handle_id);
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/MidiOutDevice.nGetTimeStamp(J)J", Any)]
#[async_method]
#[expect(clippy::cast_possible_truncation)]
pub async fn n_get_time_stamp<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    let guard = audio.midi_out_handles().read();
    if let Some(handle) = guard.get(&handle_id) {
        #[cfg(not(target_family = "wasm"))]
        {
            let elapsed = handle.start_time.elapsed();
            let micros = elapsed.as_micros() as i64;
            return Ok(Some(Value::Long(micros)));
        }
        #[cfg(target_family = "wasm")]
        {
            let elapsed_micros = super::audio_state::performance_now_micros()
                - (handle.start_time_ms * 1000.0) as i64;
            return Ok(Some(Value::Long(elapsed_micros)));
        }
    }
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("com/sun/media/sound/MidiOutDevice.nOpen(I)J", Any)]
#[async_method]
#[cfg_attr(not(target_family = "wasm"), expect(clippy::cast_sign_loss))]
pub async fn n_open<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;

    #[cfg(not(target_family = "wasm"))]
    {
        let midi_out = midir::MidiOutput::new("ristretto");
        let Ok(midi_out) = midi_out else {
            return Ok(Some(Value::Long(0)));
        };

        let ports = midi_out.ports();
        let Some(port) = ports.get(index as usize) else {
            return Ok(Some(Value::Long(0)));
        };

        let connection = midi_out.connect(port, "ristretto-midi-out");
        let Ok(connection) = connection else {
            return Ok(Some(Value::Long(0)));
        };

        let audio = thread
            .vm()?
            .extensions()
            .get_or_init(super::audio_state::AudioState::new);
        let handle_id = audio.next_midi_out_id();
        let midi_handle = super::audio_state::MidiOutHandle {
            connection: std::sync::Mutex::new(Some(connection)),
            start_time: std::time::Instant::now(),
        };

        audio
            .midi_out_handles()
            .write()
            .insert(handle_id, midi_handle);

        Ok(Some(Value::Long(handle_id)))
    }

    #[cfg(target_family = "wasm")]
    {
        use wasm_bindgen::JsCast;

        let audio = thread
            .vm()?
            .extensions()
            .get_or_init(super::audio_state::AudioState::new);
        let handle_id = audio.next_midi_out_id();
        let start_time_ms = web_sys::window()
            .and_then(|w| {
                js_sys::Reflect::get(&w, &"performance".into())
                    .ok()
                    .and_then(|p| {
                        p.dyn_ref::<web_sys::Performance>()
                            .map(web_sys::Performance::now)
                    })
            })
            .unwrap_or(0.0);

        // Attempt to get MIDI output via Web MIDI API
        let mut midi_output = None;
        if let Some(window) = web_sys::window()
            && let Ok(midi_promise) = window.navigator().request_midi_access()
            && let Ok(midi_access) = wasm_bindgen_futures::JsFuture::from(midi_promise).await
            && let Some(access) = midi_access.dyn_ref::<web_sys::MidiAccess>()
            && let Ok(Some(iter)) = js_sys::try_iter(&access.outputs())
        {
            let mut port_idx = 0i32;
            for entry in iter {
                let Ok(val) = entry else {
                    continue;
                };
                if port_idx == index {
                    let arr = js_sys::Array::from(&val);
                    if let Some(output_val) = arr.get(1).dyn_ref::<web_sys::MidiOutput>() {
                        midi_output = Some(super::audio_state::WasmValue::new(output_val.clone()));
                    }
                    break;
                }
                port_idx += 1;
            }
        }

        let midi_handle = super::audio_state::MidiOutHandle {
            start_time_ms,
            midi_output,
        };
        audio
            .midi_out_handles()
            .write()
            .insert(handle_id, midi_handle);

        Ok(Some(Value::Long(handle_id)))
    }
}

#[intrinsic_method("com/sun/media/sound/MidiOutDevice.nSendLongMessage(J[BIJ)V", Any)]
#[async_method]
#[expect(clippy::cast_sign_loss)]
pub async fn n_send_long_message<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timestamp = parameters.pop_long()?;
    let length = parameters.pop_int()?;
    let data = parameters.pop()?; // byte array
    let handle_id = parameters.pop_long()?;

    #[cfg(not(target_family = "wasm"))]
    {
        let audio = thread
            .vm()?
            .extensions()
            .get_or_init(super::audio_state::AudioState::new);
        let guard = audio.midi_out_handles().read();
        if let Some(handle) = guard.get(&handle_id)
            && let Ok(mut conn_guard) = handle.connection.lock()
            && let Some(ref mut conn) = *conn_guard
            && let Ok(bytes) = data.as_bytes()
        {
            let end = (length as usize).min(bytes.len());
            let msg: Vec<u8> = bytes[..end].to_vec();
            let _ = conn.send(&msg);
        }
    }
    #[cfg(target_family = "wasm")]
    {
        let audio = thread
            .vm()?
            .extensions()
            .get_or_init(super::audio_state::AudioState::new);
        let guard = audio.midi_out_handles().read();
        if let Some(handle) = guard.get(&handle_id)
            && let Some(ref output) = handle.midi_output
            && let Ok(bytes) = data.as_bytes()
        {
            let end = (length as usize).min(bytes.len());
            let msg = &bytes[..end];
            let js_array = js_sys::Uint8Array::from(msg);
            let _ = output.inner().send(&js_array);
        }
    }

    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/MidiOutDevice.nSendShortMessage(JIJ)V", Any)]
#[async_method]
#[expect(clippy::cast_sign_loss)]
pub async fn n_send_short_message<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timestamp = parameters.pop_long()?;
    let packed_msg = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;

    #[cfg(not(target_family = "wasm"))]
    {
        let audio = thread
            .vm()?
            .extensions()
            .get_or_init(super::audio_state::AudioState::new);
        let guard = audio.midi_out_handles().read();
        if let Some(handle) = guard.get(&handle_id)
            && let Ok(mut conn_guard) = handle.connection.lock()
            && let Some(ref mut conn) = *conn_guard
        {
            let status = (packed_msg & 0xFF) as u8;
            let data1 = ((packed_msg >> 8) & 0xFF) as u8;
            let data2 = ((packed_msg >> 16) & 0xFF) as u8;
            let msg = [status, data1, data2];
            let _ = conn.send(&msg);
        }
    }
    #[cfg(target_family = "wasm")]
    {
        let audio = thread
            .vm()?
            .extensions()
            .get_or_init(super::audio_state::AudioState::new);
        let guard = audio.midi_out_handles().read();
        if let Some(handle) = guard.get(&handle_id)
            && let Some(ref output) = handle.midi_output
        {
            let status = (packed_msg & 0xFF) as u8;
            let data1 = ((packed_msg >> 8) & 0xFF) as u8;
            let data2 = ((packed_msg >> 16) & 0xFF) as u8;
            let msg = [status, data1, data2];
            let js_array = js_sys::Uint8Array::from(&msg[..]);
            let _ = output.inner().send(&js_array);
        }
    }

    Ok(None)
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
    async fn test_n_get_time_stamp() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        let result = n_get_time_stamp(thread, params).await?;
        assert_eq!(result, Some(Value::Long(0)));
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

    #[tokio::test]
    async fn test_n_send_long_message() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0)); // handle
        params.push(Value::Object(None)); // byte array
        params.push(Value::Int(0)); // length
        params.push(Value::Long(0)); // timestamp
        let result = n_send_long_message(thread, params).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_send_short_message() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0)); // handle
        params.push(Value::Int(0)); // packed message
        params.push(Value::Long(0)); // timestamp
        let result = n_send_short_message(thread, params).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
