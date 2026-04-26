use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;
use std::sync::atomic::Ordering;

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nAvailable(JZ)I", Any)]
#[async_method]
pub async fn n_available<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_int()?; // boolean as int
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    let handles = audio.audio_handles();
    let guard = handles.read();
    if let Some(handle) = guard.get(&handle_id) {
        let buf = handle
            .buffer
            .lock()
            .map_err(|_| ristretto_types::Error::InternalError("buffer lock poisoned".into()))?;
        let available = if handle.is_source {
            // For source (output): how many bytes can be written
            handle.buffer_size_bytes.saturating_sub(buf.len())
        } else {
            // For target (input): how many bytes can be read
            buf.len()
        };
        return Ok(Some(Value::Int(
            i32::try_from(available).unwrap_or(i32::MAX),
        )));
    }
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nClose(JZ)V", Any)]
#[async_method]
pub async fn n_close<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    let handles = audio.audio_handles();
    let mut guard = handles.write();
    if let Some(handle) = guard.remove(&handle_id) {
        handle.close_signal.store(true, Ordering::Relaxed);
        #[cfg(target_family = "wasm")]
        if let Some(ctx) = handle.audio_context {
            let _ = ctx.inner().close();
        }
    }
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nFlush(JZ)V", Any)]
#[async_method]
pub async fn n_flush<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    let handles = audio.audio_handles();
    let guard = handles.read();
    if let Some(handle) = guard.get(&handle_id) {
        let mut buf = handle
            .buffer
            .lock()
            .map_err(|_| ristretto_types::Error::InternalError("buffer lock poisoned".into()))?;
        buf.clear();
    }
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nGetBufferSize(JZ)I", Any)]
#[async_method]
#[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub async fn n_get_buffer_size<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    let handles = audio.audio_handles();
    let guard = handles.read();
    if let Some(handle) = guard.get(&handle_id) {
        return Ok(Some(Value::Int(handle.buffer_size_bytes as i32)));
    }
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nGetBytePosition(JZJ)J", Any)]
#[async_method]
pub async fn n_get_byte_position<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _java_pos = parameters.pop_long()?;
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    let handles = audio.audio_handles();
    let guard = handles.read();
    if let Some(handle) = guard.get(&handle_id) {
        let pos = handle.byte_position.load(Ordering::Relaxed);
        return Ok(Some(Value::Long(pos)));
    }
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method(
    "com/sun/media/sound/DirectAudioDevice.nGetFormats(IIZLjava/util/Vector;)V",
    Any
)]
#[async_method]
pub async fn n_get_formats<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let vector = parameters.pop()?;
    let _is_source = parameters.pop_int()?;
    let _device_id = parameters.pop_int()?;
    let _mixer_index = parameters.pop_int()?;

    // Standard PCM formats: sample rates, bit depths, channels
    let sample_rates: &[f32] = &[8000.0, 11025.0, 16000.0, 22050.0, 44100.0, 48000.0];
    let bit_depths: &[i32] = &[8, 16];
    let channels_list: &[i32] = &[1, 2];

    let vector_class = thread.class("java/util/Vector").await?;
    let add_method = vector_class
        .method("addElement", "(Ljava/lang/Object;)V")
        .ok_or_else(|| {
            ristretto_types::Error::InternalError("Vector.addElement not found".to_string())
        })?;

    for &sample_rate in sample_rates {
        for &bits in bit_depths {
            for &channels in channels_list {
                // AudioFormat(float sampleRate, int sampleSizeInBits, int channels,
                //             boolean signed, boolean bigEndian)
                let format = thread
                    .object(
                        "javax/sound/sampled/AudioFormat",
                        "FIIZZ",
                        &[
                            Value::Float(sample_rate),
                            Value::Int(bits),
                            Value::Int(channels),
                            Value::Int(1), // signed = true
                            Value::Int(0), // bigEndian = false
                        ],
                    )
                    .await?;

                thread
                    .execute(&vector_class, &add_method, &[vector.clone(), format])
                    .await?;
            }
        }
    }

    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nIsStillDraining(JZ)Z", Any)]
#[async_method]
pub async fn n_is_still_draining<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    let handles = audio.audio_handles();
    let guard = handles.read();
    if let Some(handle) = guard.get(&handle_id) {
        let buf = handle
            .buffer
            .lock()
            .map_err(|_| ristretto_types::Error::InternalError("buffer lock poisoned".into()))?;
        let draining = handle.is_running.load(Ordering::Relaxed) && !buf.is_empty();
        return Ok(Some(Value::from(draining)));
    }
    Ok(Some(Value::from(false)))
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nOpen(IIZIFIIIZZI)J", Any)]
#[expect(
    clippy::too_many_lines,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
#[cfg_attr(not(target_family = "wasm"), expect(clippy::cast_possible_wrap))]
#[async_method]
pub async fn n_open<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let buffer_size = parameters.pop_int()?;
    let _is_big_endian = parameters.pop_int()?; // boolean
    let _is_signed = parameters.pop_int()?; // boolean
    let channels = parameters.pop_int()?;
    let _frame_size = parameters.pop_int()?;
    let sample_size_in_bits = parameters.pop_int()?;
    let sample_rate = parameters.pop_float()?;
    let _encoding = parameters.pop_int()?;
    let is_source = parameters.pop_int()?; // boolean
    let _device_id = parameters.pop_int()?;
    let _mixer_index = parameters.pop_int()?;

    let frame_size = (sample_size_in_bits / 8 * channels) as u32;
    let buffer_size_bytes = if buffer_size > 0 {
        buffer_size as usize
    } else {
        (sample_rate as usize) * (frame_size as usize) / 2 // ~0.5 second buffer
    };

    #[cfg(not(target_family = "wasm"))]
    {
        use std::collections::VecDeque;
        use std::sync::Mutex;
        use std::sync::atomic::AtomicBool;

        let buffer: Arc<Mutex<VecDeque<u8>>> =
            Arc::new(Mutex::new(VecDeque::with_capacity(buffer_size_bytes)));
        let byte_position = Arc::new(std::sync::atomic::AtomicI64::new(0));
        let is_running = Arc::new(AtomicBool::new(false));
        let close_signal = Arc::new(AtomicBool::new(false));

        let buf_clone = Arc::clone(&buffer);
        let pos_clone = Arc::clone(&byte_position);
        let running_clone = Arc::clone(&is_running);
        let close_clone = Arc::clone(&close_signal);
        let bits_per_sample = sample_size_in_bits;

        let is_source_bool = is_source != 0;
        let channels_u16 = channels as u16;
        let sample_rate_u32 = sample_rate as u32;
        let (tx, rx) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

            let host = cpal::default_host();
            let device = if is_source_bool {
                host.default_output_device()
            } else {
                host.default_input_device()
            };

            let Some(device) = device else {
                let _ = tx.send(false);
                return;
            };

            let default_config = device.default_output_config().ok();
            let supported_config =
                device
                    .supported_output_configs()
                    .ok()
                    .and_then(|mut configs| {
                        configs.find(|c| {
                            c.channels() >= channels_u16
                                && c.min_sample_rate().0 <= sample_rate_u32
                                && c.max_sample_rate().0 >= sample_rate_u32
                        })
                    });

            let (actual_channels, actual_sample_rate) = if let Some(sc) = &supported_config {
                (
                    sc.channels(),
                    sample_rate_u32.clamp(sc.min_sample_rate().0, sc.max_sample_rate().0),
                )
            } else if let Some(dc) = &default_config {
                (dc.channels(), dc.sample_rate().0)
            } else {
                (channels_u16, sample_rate_u32)
            };

            let config = cpal::StreamConfig {
                channels: actual_channels,
                sample_rate: cpal::SampleRate(actual_sample_rate),
                buffer_size: cpal::BufferSize::Default,
            };

            let source_channels = channels_u16;
            let stream = if is_source_bool {
                device.build_output_stream(
                    &config,
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        if !running_clone.load(Ordering::Relaxed) {
                            data.fill(0.0);
                            return;
                        }
                        let Ok(mut buf) = buf_clone.lock() else {
                            data.fill(0.0);
                            return;
                        };
                        let mut bytes_consumed: i64 = 0;
                        // Process data in frames of actual_channels samples
                        for frame in data.chunks_mut(actual_channels as usize) {
                            // Decode one source sample
                            let sample = if bits_per_sample == 16 {
                                if buf.len() >= 2 {
                                    let low = buf.pop_front().unwrap_or(0);
                                    let high = buf.pop_front().unwrap_or(0);
                                    let s = i16::from_le_bytes([low, high]);
                                    bytes_consumed += 2;
                                    f32::from(s) / 32768.0
                                } else {
                                    0.0
                                }
                            } else if let Some(byte) = buf.pop_front() {
                                bytes_consumed += 1;
                                (f32::from(byte) - 128.0) / 128.0
                            } else {
                                0.0
                            };

                            if source_channels == 1 {
                                // Mono source: duplicate to all output channels
                                for ch in frame.iter_mut() {
                                    *ch = sample;
                                }
                            } else {
                                // First channel gets the decoded sample
                                frame[0] = sample;
                                // Decode remaining source channels
                                for ch in frame
                                    .iter_mut()
                                    .skip(1)
                                    .take((source_channels - 1) as usize)
                                {
                                    *ch = if bits_per_sample == 16 {
                                        if buf.len() >= 2 {
                                            let low = buf.pop_front().unwrap_or(0);
                                            let high = buf.pop_front().unwrap_or(0);
                                            let s = i16::from_le_bytes([low, high]);
                                            bytes_consumed += 2;
                                            f32::from(s) / 32768.0
                                        } else {
                                            0.0
                                        }
                                    } else if let Some(byte) = buf.pop_front() {
                                        bytes_consumed += 1;
                                        (f32::from(byte) - 128.0) / 128.0
                                    } else {
                                        0.0
                                    };
                                }
                                // Zero any extra output channels
                                for ch in frame.iter_mut().skip(source_channels as usize) {
                                    *ch = 0.0;
                                }
                            }
                        }
                        pos_clone.fetch_add(bytes_consumed, Ordering::Relaxed);
                    },
                    |err| {
                        tracing::error!("Audio output stream error: {err}");
                    },
                    None,
                )
            } else {
                device.build_input_stream(
                    &config,
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        if !running_clone.load(Ordering::Relaxed) {
                            return;
                        }
                        let Ok(mut buf) = buf_clone.lock() else {
                            return;
                        };
                        for &sample in data {
                            let byte = ((sample * 128.0) + 128.0).clamp(0.0, 255.0) as u8;
                            buf.push_back(byte);
                        }
                        pos_clone.fetch_add(data.len() as i64, Ordering::Relaxed);
                    },
                    |err| {
                        tracing::error!("Audio input stream error: {err}");
                    },
                    None,
                )
            };

            let Ok(stream) = stream else {
                let _ = tx.send(false);
                return;
            };

            let _ = stream.play();
            let _ = tx.send(true);

            while !close_clone.load(Ordering::Relaxed) {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            drop(stream);
        });

        // Even if the audio device failed, create the handle so Java can proceed.
        // Audio data will simply be discarded if no device is available.
        let _device_ok = rx.recv().unwrap_or(false);

        let audio = thread
            .vm()?
            .extensions()
            .get_or_init(super::audio_state::AudioState::new);
        let handle_id = audio.next_audio_id();
        let audio_handle = super::audio_state::AudioStreamHandle {
            buffer,
            sample_rate: sample_rate as u32,
            channels: channels as u16,
            sample_size_bits: sample_size_in_bits as u32,
            frame_size,
            byte_position,
            is_source: is_source != 0,
            is_running,
            buffer_size_bytes,
            close_signal,
        };

        let handles = audio.audio_handles();
        let mut guard = handles.write();
        guard.insert(handle_id, audio_handle);

        Ok(Some(Value::Long(handle_id)))
    }

    #[cfg(target_family = "wasm")]
    {
        use std::collections::VecDeque;
        use std::sync::Mutex;
        use std::sync::atomic::{AtomicBool, AtomicI64};

        let sample_rate_u32 = sample_rate as u32;

        // Create Web Audio AudioContext
        let audio_context = {
            let opts = web_sys::AudioContextOptions::new();
            opts.set_sample_rate(sample_rate);
            web_sys::AudioContext::new_with_context_options(&opts)
                .ok()
                .map(super::audio_state::WasmValue::new)
        };

        let audio = thread
            .vm()?
            .extensions()
            .get_or_init(super::audio_state::AudioState::new);
        let handle_id = audio.next_audio_id();

        let buffer: Arc<Mutex<VecDeque<u8>>> =
            Arc::new(Mutex::new(VecDeque::with_capacity(buffer_size_bytes)));

        let audio_handle = super::audio_state::AudioStreamHandle {
            buffer,
            sample_rate: sample_rate_u32,
            channels: channels as u16,
            sample_size_bits: sample_size_in_bits as u32,
            frame_size,
            byte_position: Arc::new(AtomicI64::new(0)),
            is_source: is_source != 0,
            is_running: Arc::new(AtomicBool::new(false)),
            buffer_size_bytes,
            close_signal: Arc::new(AtomicBool::new(false)),
            audio_context,
            schedule_time: 0.0,
        };

        let handles = audio.audio_handles();
        let mut guard = handles.write();
        guard.insert(handle_id, audio_handle);

        Ok(Some(Value::Long(handle_id)))
    }
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nRead(J[BIII)I", Any)]
#[async_method]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
pub async fn n_read<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _conversion_size = parameters.pop_int()?;
    let len = parameters.pop_int()?;
    let _offset = parameters.pop_int()?;
    let _data = parameters.pop()?; // byte array
    let handle_id = parameters.pop_long()?;

    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    let handles = audio.audio_handles();
    let guard = handles.read();
    if let Some(handle) = guard.get(&handle_id) {
        let mut buf = handle
            .buffer
            .lock()
            .map_err(|_| ristretto_types::Error::InternalError("buffer lock poisoned".into()))?;
        let available = buf.len().min(len as usize);
        let _bytes: Vec<u8> = buf.drain(..available).collect();
        return Ok(Some(Value::Int(available as i32)));
    }
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nRequiresServicing(JZ)Z", Any)]
#[async_method]
pub async fn n_requires_servicing<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nService(JZ)V", Any)]
#[async_method]
pub async fn n_service<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nSetBytePosition(JZJ)V", Any)]
#[async_method]
pub async fn n_set_byte_position<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let new_pos = parameters.pop_long()?;
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    let handles = audio.audio_handles();
    let guard = handles.read();
    if let Some(handle) = guard.get(&handle_id) {
        handle.byte_position.store(new_pos, Ordering::Relaxed);
    }
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nStart(JZ)V", Any)]
#[async_method]
pub async fn n_start<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    let handles = audio.audio_handles();
    let guard = handles.read();
    if let Some(handle) = guard.get(&handle_id) {
        handle.is_running.store(true, Ordering::Relaxed);
        #[cfg(target_family = "wasm")]
        if let Some(ref ctx) = handle.audio_context {
            let _ = ctx.inner().resume();
        }
    }
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nStop(JZ)V", Any)]
#[async_method]
pub async fn n_stop<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .extensions()
        .get_or_init(super::audio_state::AudioState::new);
    let handles = audio.audio_handles();
    let guard = handles.read();
    if let Some(handle) = guard.get(&handle_id) {
        handle.is_running.store(false, Ordering::Relaxed);
        #[cfg(target_family = "wasm")]
        if let Some(ref ctx) = handle.audio_context {
            let _ = ctx.inner().suspend();
        }
    }
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nWrite(J[BIIIFF)I", Any)]
#[async_method]
#[cfg_attr(not(target_family = "wasm"), expect(clippy::cast_sign_loss))]
pub async fn n_write<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _right = parameters.pop_float()?;
    let _left = parameters.pop_float()?;
    let _conversion_size = parameters.pop_int()?;
    let len = parameters.pop_int()?;
    let offset = parameters.pop_int()?;
    let data = parameters.pop()?; // byte array
    let handle_id = parameters.pop_long()?;

    #[cfg(not(target_family = "wasm"))]
    {
        let audio = thread
            .vm()?
            .extensions()
            .get_or_init(super::audio_state::AudioState::new);
        let handles = audio.audio_handles();
        let guard = handles.read();
        if let Some(handle) = guard.get(&handle_id)
            && let Ok(bytes) = data.as_bytes()
        {
            let start = offset as usize;
            let end = (offset + len) as usize;
            if end <= bytes.len() {
                let mut buf = handle.buffer.lock().map_err(|_| {
                    ristretto_types::Error::InternalError("buffer lock poisoned".into())
                })?;
                // Limit how much we write to available buffer space to create
                // backpressure, matching JDK native nWrite behavior.
                let available = handle.buffer_size_bytes.saturating_sub(buf.len());
                let to_write = available.min(end - start);
                let actual_end = start + to_write;
                for &b in &bytes[start..actual_end] {
                    buf.push_back(b);
                }
                return Ok(Some(Value::Int(i32::try_from(to_write)?)));
            }
        }
        Ok(Some(Value::Int(0)))
    }
    #[cfg(target_family = "wasm")]
    {
        let audio = thread
            .vm()?
            .extensions()
            .get_or_init(super::audio_state::AudioState::new);
        let written = wasm_write_audio(&audio, handle_id, &data, offset, len);
        Ok(Some(Value::Int(written)))
    }
}

/// Convert PCM bytes to float samples and schedule via Web Audio API.
#[cfg(target_family = "wasm")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
fn wasm_write_audio(
    audio: &super::audio_state::AudioState,
    handle_id: i64,
    data: &Value,
    offset: i32,
    len: i32,
) -> i32 {
    let Ok(bytes) = data.as_bytes() else {
        return 0;
    };
    let start = offset as usize;
    let end = (offset + len) as usize;
    if end > bytes.len() {
        return 0;
    }
    let pcm_bytes = &bytes[start..end];

    // Get audio format info and audio context from the handle
    let (sample_rate, channels, sample_size_bits) = {
        let handles = audio.audio_handles();
        let guard = handles.read();
        let Some(handle) = guard.get(&handle_id) else {
            return 0;
        };
        handle
            .byte_position
            .fetch_add(pcm_bytes.len() as i64, Ordering::Relaxed);
        (handle.sample_rate, handle.channels, handle.sample_size_bits)
    };

    let handles = audio.audio_handles();
    let mut guard = handles.write();
    let Some(handle) = guard.get_mut(&handle_id) else {
        return 0;
    };
    let Some(ref ctx_wrapper) = handle.audio_context else {
        return len;
    };
    let ctx = ctx_wrapper.inner();

    let num_frames = match sample_size_bits {
        8 => pcm_bytes.len() / channels as usize,
        24 => pcm_bytes.len() / (3 * channels as usize),
        _ => pcm_bytes.len() / (2 * channels as usize),
    };

    if num_frames == 0 {
        return len;
    }

    let Ok(audio_buffer) = ctx.create_buffer(
        u32::from(channels),
        num_frames as u32,
        f32::from(sample_rate as u16),
    ) else {
        return len;
    };

    // Convert PCM bytes to float samples per channel
    for ch in 0..channels as usize {
        let mut samples = vec![0.0f32; num_frames];
        for (i, sample) in samples.iter_mut().enumerate() {
            *sample = match sample_size_bits {
                8 => {
                    let idx = i * channels as usize + ch;
                    if idx < pcm_bytes.len() {
                        (f32::from(pcm_bytes[idx]) - 128.0) / 128.0
                    } else {
                        0.0
                    }
                }
                16 => {
                    let idx = (i * channels as usize + ch) * 2;
                    if idx + 1 < pcm_bytes.len() {
                        let s = i16::from_le_bytes([pcm_bytes[idx], pcm_bytes[idx + 1]]);
                        f32::from(s) / 32768.0
                    } else {
                        0.0
                    }
                }
                24 => {
                    let idx = (i * channels as usize + ch) * 3;
                    if idx + 2 < pcm_bytes.len() {
                        let s = i32::from_le_bytes([
                            0,
                            pcm_bytes[idx],
                            pcm_bytes[idx + 1],
                            pcm_bytes[idx + 2],
                        ]);
                        s as f32 / 8_388_608.0
                    } else {
                        0.0
                    }
                }
                _ => 0.0,
            };
        }
        let _ = audio_buffer.copy_to_channel(&samples, ch as i32);
    }

    let Ok(source) = ctx.create_buffer_source() else {
        return len;
    };
    source.set_buffer(Some(&audio_buffer));
    let _ = source.connect_with_audio_node(&ctx.destination());

    // Schedule for gapless playback
    let current_time = ctx.current_time();
    let start_at = if handle.schedule_time > current_time {
        handle.schedule_time
    } else {
        current_time
    };
    let _ = source.start_with_when(start_at);
    let duration = num_frames as f64 / f64::from(sample_rate as u16);
    handle.schedule_time = start_at + duration;

    len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_n_available() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1)); // is_source
        let result = n_available(thread, params).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_close() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1));
        let result = n_close(thread, params).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_flush() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1));
        let result = n_flush(thread, params).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_get_buffer_size() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1));
        let result = n_get_buffer_size(thread, params).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_get_byte_position() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1));
        params.push(Value::Long(0));
        let result = n_get_byte_position(thread, params).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_get_formats() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_formats(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_n_is_still_draining() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1));
        let result = n_is_still_draining(thread, params).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_requires_servicing() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_requires_servicing(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_service() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_service(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_set_byte_position() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1));
        params.push(Value::Long(0));
        let result = n_set_byte_position(thread, params).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_start() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1));
        let result = n_start(thread, params).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_stop() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1));
        let result = n_stop(thread, params).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_read() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Object(None)); // byte array
        params.push(Value::Int(0)); // offset
        params.push(Value::Int(0)); // len
        params.push(Value::Int(0)); // conversion_size
        let result = n_read(thread, params).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_write() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Object(None)); // byte array
        params.push(Value::Int(0)); // offset
        params.push(Value::Int(0)); // len
        params.push(Value::Int(0)); // conversion_size
        params.push(Value::Float(1.0)); // left
        params.push(Value::Float(1.0)); // right
        let result = n_write(thread, params).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }
}
