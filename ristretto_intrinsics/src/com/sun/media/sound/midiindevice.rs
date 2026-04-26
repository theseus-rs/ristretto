use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nClose(J)V", Any)]
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
    let mut guard = audio.midi_in_handles().write();
    if let Some(handle) = guard.remove(&handle_id) {
        handle
            .close_signal
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nGetMessages(J)V", Any)]
#[async_method]
pub async fn n_get_messages<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    #[cfg(not(target_family = "wasm"))]
    {
        let guard = audio.midi_in_handles().read();
        if let Some(handle) = guard.get(&handle_id) {
            let mut msgs = handle.messages.lock().map_err(|_| {
                ristretto_types::Error::InternalError("midi message lock poisoned".into())
            })?;
            msgs.clear();
        }
    }
    #[cfg(target_family = "wasm")]
    {
        let mut guard = audio.midi_in_handles().write();
        if let Some(handle) = guard.get_mut(&handle_id) {
            handle.messages.clear();
        }
    }
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nGetTimeStamp(J)J", Any)]
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
    let guard = audio.midi_in_handles().read();
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

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nOpen(I)J", Any)]
#[cfg_attr(not(target_family = "wasm"), expect(clippy::cast_sign_loss))]
#[expect(clippy::too_many_lines)]
#[async_method]
pub async fn n_open<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;

    #[cfg(not(target_family = "wasm"))]
    {
        use std::collections::VecDeque;
        use std::sync::Mutex;
        use std::sync::atomic::AtomicBool;

        let messages: Arc<Mutex<VecDeque<Vec<u8>>>> = Arc::new(Mutex::new(VecDeque::new()));
        let msgs_clone = Arc::clone(&messages);
        let close_signal = Arc::new(AtomicBool::new(false));
        let close_clone = Arc::clone(&close_signal);
        let (tx, rx) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            let Ok(midi_in) = midir::MidiInput::new("ristretto") else {
                let _ = tx.send(false);
                return;
            };

            let ports = midi_in.ports();
            let Some(port) = ports.get(index as usize) else {
                let _ = tx.send(false);
                return;
            };

            let Ok(connection) = midi_in.connect(
                port,
                "ristretto-midi-in",
                move |_stamp, message, data| {
                    if let Ok(mut queue) = data.lock() {
                        queue.push_back(message.to_vec());
                    }
                },
                msgs_clone,
            ) else {
                let _ = tx.send(false);
                return;
            };

            let _ = tx.send(true);

            while !close_clone.load(std::sync::atomic::Ordering::Relaxed) {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            drop(connection);
        });

        let success = rx.recv().unwrap_or(false);
        if !success {
            return Ok(Some(Value::Long(0)));
        }

        let audio = thread
            .vm()?
            .extensions()
            .get_or_init(super::audio_state::AudioState::new);
        let handle_id = audio.next_midi_in_id();
        let midi_handle = super::audio_state::MidiInHandle {
            messages,
            start_time: std::time::Instant::now(),
            is_started: false,
            close_signal,
        };

        audio
            .midi_in_handles()
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
        let handle_id = audio.next_midi_in_id();
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

        // Attempt to get MIDI input via Web MIDI API
        let mut midi_input = None;
        'midi: {
            let Some(window) = web_sys::window() else {
                break 'midi;
            };
            let navigator = window.navigator();
            let Ok(midi_promise) = navigator.request_midi_access() else {
                break 'midi;
            };
            let Ok(midi_access) = wasm_bindgen_futures::JsFuture::from(midi_promise).await else {
                break 'midi;
            };
            let Some(access) = midi_access.dyn_ref::<web_sys::MidiAccess>() else {
                break 'midi;
            };
            let inputs = access.inputs();
            let Ok(Some(iter)) = js_sys::try_iter(&inputs) else {
                break 'midi;
            };
            let mut port_idx = 0i32;
            for entry in iter {
                let Ok(val) = entry else {
                    continue;
                };
                if port_idx == index {
                    let arr = js_sys::Array::from(&val);
                    if let Some(input_val) = arr.get(1).dyn_ref::<web_sys::MidiInput>() {
                        midi_input = Some(super::audio_state::WasmValue::new(input_val.clone()));
                    }
                    break;
                }
                port_idx += 1;
            }
        }

        let midi_handle = super::audio_state::MidiInHandle {
            start_time_ms,
            messages: Vec::new(),
            is_started: false,
            close_signal: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            midi_input,
        };

        audio
            .midi_in_handles()
            .write()
            .insert(handle_id, midi_handle);

        Ok(Some(Value::Long(handle_id)))
    }
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nStart(J)V", Any)]
#[async_method]
pub async fn n_start<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    let mut guard = audio.midi_in_handles().write();
    if let Some(handle) = guard.get_mut(&handle_id) {
        handle.is_started = true;
    }
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/MidiInDevice.nStop(J)V", Any)]
#[async_method]
pub async fn n_stop<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    let mut guard = audio.midi_in_handles().write();
    if let Some(handle) = guard.get_mut(&handle_id) {
        handle.is_started = false;
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
    async fn test_n_get_messages() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        let result = n_get_messages(thread, params).await?;
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
    async fn test_n_start() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        let result = n_start(thread, params).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_stop() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        let result = n_stop(thread, params).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
