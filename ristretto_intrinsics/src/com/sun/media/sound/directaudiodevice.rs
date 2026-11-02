use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;
use std::sync::atomic::Ordering;

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nAvailable(JZ)I", Any)]
pub fn n_available<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_int()?; // boolean as int
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .resource_manager()
        .get_or_init(super::audio_state::AudioState::new)?;
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
pub fn n_close<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .resource_manager()
        .get_or_init(super::audio_state::AudioState::new)?;
    let handles = audio.audio_handles();
    let mut guard = handles.write();
    if let Some(handle) = guard.remove(&handle_id) {
        handle.close_signal.store(true, Ordering::Relaxed);
        #[cfg(all(target_family = "wasm", target_os = "unknown"))]
        if let Some(ctx) = handle.audio_context {
            let _ = ctx.inner().close();
        }
    }
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nFlush(JZ)V", Any)]
pub fn n_flush<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .resource_manager()
        .get_or_init(super::audio_state::AudioState::new)?;
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
#[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn n_get_buffer_size<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .resource_manager()
        .get_or_init(super::audio_state::AudioState::new)?;
    let handles = audio.audio_handles();
    let guard = handles.read();
    if let Some(handle) = guard.get(&handle_id) {
        return Ok(Some(Value::Int(handle.buffer_size_bytes as i32)));
    }
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nGetBytePosition(JZJ)J", Any)]
pub fn n_get_byte_position<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _java_pos = parameters.pop_long()?;
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .resource_manager()
        .get_or_init(super::audio_state::AudioState::new)?;
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
    let add_method = vector_class.resolve_method("addElement", "(Ljava/lang/Object;)V")?;

    let receiver_class = vector.as_object_ref()?.class().clone();
    let add_method = receiver_class.select_concrete_method(&add_method)?;

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
                    .execute(
                        &add_method.declaring_class,
                        &add_method.method,
                        &[vector.clone(), format],
                    )
                    .await?;
            }
        }
    }

    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nIsStillDraining(JZ)Z", Any)]
pub fn n_is_still_draining<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .resource_manager()
        .get_or_init(super::audio_state::AudioState::new)?;
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
#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn n_open<T: ristretto_types::Thread + 'static>(
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
        let byte_position = Arc::new(portable_atomic::AtomicI64::new(0));
        let is_running = Arc::new(AtomicBool::new(false));
        let close_signal = Arc::new(AtomicBool::new(false));

        let audio = thread
            .vm()?
            .resource_manager()
            .get_or_init(super::audio_state::AudioState::new)?;
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

        start_native_audio_stream(&audio_handle);

        let handles = audio.audio_handles();
        let mut guard = handles.write();
        guard.insert(handle_id, audio_handle);

        Ok(Some(Value::Long(handle_id)))
    }

    #[cfg(target_family = "wasm")]
    {
        use portable_atomic::AtomicI64;
        use std::collections::VecDeque;
        use std::sync::Mutex;
        use std::sync::atomic::AtomicBool;

        let sample_rate_u32 = sample_rate as u32;

        // Create Web Audio AudioContext
        #[cfg(all(target_family = "wasm", target_os = "unknown"))]
        let audio_context = {
            let opts = web_sys::AudioContextOptions::new();
            opts.set_sample_rate(sample_rate);
            web_sys::AudioContext::new_with_context_options(&opts)
                .ok()
                .map(super::audio_state::WasmValue::new)
        };

        let audio = thread
            .vm()?
            .resource_manager()
            .get_or_init(super::audio_state::AudioState::new)?;
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
            #[cfg(all(target_family = "wasm", target_os = "unknown"))]
            audio_context,
            #[cfg(all(target_family = "wasm", target_os = "unknown"))]
            schedule_time: 0.0,
        };

        let handles = audio.audio_handles();
        let mut guard = handles.write();
        guard.insert(handle_id, audio_handle);

        Ok(Some(Value::Long(handle_id)))
    }
}

/// Starts a native stream and waits until device initialization finishes.
#[cfg(not(target_family = "wasm"))]
fn start_native_audio_stream(handle: &super::audio_state::AudioStreamHandle) {
    let buf_clone = Arc::clone(&handle.buffer);
    let pos_clone = Arc::clone(&handle.byte_position);
    let running_clone = Arc::clone(&handle.is_running);
    let close_clone = Arc::clone(&handle.close_signal);
    let bits_per_sample = handle.sample_size_bits;
    let is_source_bool = handle.is_source;
    let channels_u16 = handle.channels;
    let sample_rate_u32 = handle.sample_rate;
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
        let supported_config = device
            .supported_output_configs()
            .ok()
            .and_then(|mut configs| {
                configs.find(|c| {
                    c.channels() >= channels_u16
                        && c.min_sample_rate() <= sample_rate_u32
                        && c.max_sample_rate() >= sample_rate_u32
                })
            });

        let (actual_channels, actual_sample_rate) = if let Some(sc) = &supported_config {
            (
                sc.channels(),
                sample_rate_u32.clamp(sc.min_sample_rate(), sc.max_sample_rate()),
            )
        } else if let Some(dc) = &default_config {
            (dc.channels(), dc.sample_rate())
        } else {
            (channels_u16, sample_rate_u32)
        };

        let config = cpal::StreamConfig {
            channels: actual_channels,
            sample_rate: actual_sample_rate,
            buffer_size: cpal::BufferSize::Default,
        };

        let source_channels = channels_u16;
        let stream = if is_source_bool {
            device.build_output_stream(
                config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    write_output_samples(
                        data,
                        &buf_clone,
                        &pos_clone,
                        &running_clone,
                        bits_per_sample,
                        source_channels,
                        actual_channels,
                    );
                },
                |err| {
                    tracing::error!("Audio output stream error: {err}");
                },
                None,
            )
        } else {
            device.build_input_stream(
                config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    read_input_samples(data, &buf_clone, &pos_clone, &running_clone);
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
}

/// Decodes one complete PCM sample, leaving incomplete samples buffered.
#[cfg(not(target_family = "wasm"))]
fn decode_sample(
    buffer: &mut std::collections::VecDeque<u8>,
    bits_per_sample: u32,
    bytes_consumed: &mut i64,
) -> f32 {
    if bits_per_sample == 16 {
        if buffer.len() < 2 {
            return 0.0;
        }
        let low = buffer.pop_front().unwrap_or(0);
        let high = buffer.pop_front().unwrap_or(0);
        *bytes_consumed += 2;
        return f32::from(i16::from_le_bytes([low, high])) / 32768.0;
    }
    let Some(byte) = buffer.pop_front() else {
        return 0.0;
    };
    *bytes_consumed += 1;
    (f32::from(byte) - 128.0) / 128.0
}

/// Fills output frames and advances the position by the number of consumed bytes.
#[cfg(not(target_family = "wasm"))]
fn write_output_samples(
    data: &mut [f32],
    buffer: &std::sync::Mutex<std::collections::VecDeque<u8>>,
    byte_position: &portable_atomic::AtomicI64,
    is_running: &std::sync::atomic::AtomicBool,
    bits_per_sample: u32,
    source_channels: u16,
    actual_channels: u16,
) {
    if !is_running.load(Ordering::Relaxed) {
        data.fill(0.0);
        return;
    }
    let Ok(mut buffer) = buffer.lock() else {
        data.fill(0.0);
        return;
    };
    let mut bytes_consumed = 0;
    for frame in data.chunks_mut(usize::from(actual_channels)) {
        let sample = decode_sample(&mut buffer, bits_per_sample, &mut bytes_consumed);
        if source_channels == 1 {
            // Mono source: duplicate to all output channels.
            frame.fill(sample);
            continue;
        }
        if let Some(first_channel) = frame.first_mut() {
            *first_channel = sample;
        }
        for channel in frame
            .iter_mut()
            .skip(1)
            .take(usize::from(source_channels - 1))
        {
            *channel = decode_sample(&mut buffer, bits_per_sample, &mut bytes_consumed);
        }
        for channel in frame.iter_mut().skip(usize::from(source_channels)) {
            *channel = 0.0;
        }
    }
    byte_position.fetch_add(bytes_consumed, Ordering::Relaxed);
}

/// Buffers captured samples and advances the input byte position.
#[cfg(not(target_family = "wasm"))]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
fn read_input_samples(
    data: &[f32],
    buffer: &std::sync::Mutex<std::collections::VecDeque<u8>>,
    byte_position: &portable_atomic::AtomicI64,
    is_running: &std::sync::atomic::AtomicBool,
) {
    if !is_running.load(Ordering::Relaxed) {
        return;
    }
    let Ok(mut buffer) = buffer.lock() else {
        return;
    };
    for &sample in data {
        let byte = ((sample * 128.0) + 128.0).clamp(0.0, 255.0) as u8;
        buffer.push_back(byte);
    }
    byte_position.fetch_add(data.len() as i64, Ordering::Relaxed);
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nRead(J[BIII)I", Any)]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
pub fn n_read<T: ristretto_types::Thread + 'static>(
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
        .resource_manager()
        .get_or_init(super::audio_state::AudioState::new)?;
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
pub fn n_requires_servicing<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nService(JZ)V", Any)]
pub fn n_service<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nSetBytePosition(JZJ)V", Any)]
pub fn n_set_byte_position<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let new_pos = parameters.pop_long()?;
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .resource_manager()
        .get_or_init(super::audio_state::AudioState::new)?;
    let handles = audio.audio_handles();
    let guard = handles.read();
    if let Some(handle) = guard.get(&handle_id) {
        handle.byte_position.store(new_pos, Ordering::Relaxed);
    }
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nStart(JZ)V", Any)]
pub fn n_start<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .resource_manager()
        .get_or_init(super::audio_state::AudioState::new)?;
    let handles = audio.audio_handles();
    let guard = handles.read();
    if let Some(handle) = guard.get(&handle_id) {
        handle.is_running.store(true, Ordering::Relaxed);
        #[cfg(all(target_family = "wasm", target_os = "unknown"))]
        if let Some(ref ctx) = handle.audio_context {
            let _ = ctx.inner().resume();
        }
    }
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nStop(JZ)V", Any)]
pub fn n_stop<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_source = parameters.pop_int()?;
    let handle_id = parameters.pop_long()?;
    let audio = thread
        .vm()?
        .resource_manager()
        .get_or_init(super::audio_state::AudioState::new)?;
    let handles = audio.audio_handles();
    let guard = handles.read();
    if let Some(handle) = guard.get(&handle_id) {
        handle.is_running.store(false, Ordering::Relaxed);
        #[cfg(all(target_family = "wasm", target_os = "unknown"))]
        if let Some(ref ctx) = handle.audio_context {
            let _ = ctx.inner().suspend();
        }
    }
    Ok(None)
}

#[intrinsic_method("com/sun/media/sound/DirectAudioDevice.nWrite(J[BIIIFF)I", Any)]
#[cfg_attr(
    not(all(target_family = "wasm", target_os = "unknown")),
    expect(clippy::cast_sign_loss)
)]
pub fn n_write<T: ristretto_types::Thread + 'static>(
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

    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    {
        let audio = thread
            .vm()?
            .resource_manager()
            .get_or_init(super::audio_state::AudioState::new)?;
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
                if let Some(bytes_to_write) = bytes.get(start..actual_end) {
                    for &byte in bytes_to_write {
                        buf.push_back(byte);
                    }
                    return Ok(Some(Value::Int(i32::try_from(to_write)?)));
                }
            }
        }
        Ok(Some(Value::Int(0)))
    }
    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    {
        let audio = thread
            .vm()?
            .resource_manager()
            .get_or_init(super::audio_state::AudioState::new)?;
        let written = wasm_write_audio(&audio, handle_id, &data, offset, len);
        Ok(Some(Value::Int(written)))
    }
}

/// Convert PCM bytes to float samples and schedule via Web Audio API.
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
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
    let Some(pcm_bytes) = bytes.get(start..end) else {
        return 0;
    };

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
                    let Some(&byte) = pcm_bytes.get(idx) else {
                        continue;
                    };
                    (f32::from(byte) - 128.0) / 128.0
                }
                16 => {
                    let idx = (i * channels as usize + ch) * 2;
                    let Some(bytes) = pcm_bytes.get(idx..idx + 2) else {
                        continue;
                    };
                    let Ok([first, second]) = <[u8; 2]>::try_from(bytes) else {
                        continue;
                    };
                    let sample = i16::from_le_bytes([first, second]);
                    f32::from(sample) / 32768.0
                }
                24 => {
                    let idx = (i * channels as usize + ch) * 3;
                    let Some(bytes) = pcm_bytes.get(idx..idx + 3) else {
                        continue;
                    };
                    let Ok([first, second, third]) = <[u8; 3]>::try_from(bytes) else {
                        continue;
                    };
                    let sample = i32::from_le_bytes([0, first, second, third]);
                    sample as f32 / 8_388_608.0
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

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn test_output_sample_decoding_and_channels() {
        use std::collections::VecDeque;
        use std::sync::Mutex;
        use std::sync::atomic::AtomicBool;

        let cases = [
            (vec![0, 128, 0, 64], 16, 2, 2, vec![-1.0, 0.5], 4),
            (vec![0, 128, 0, 64], 16, 1, 2, vec![-1.0, -1.0, 0.5, 0.5], 4),
            (vec![0, 192], 8, 2, 3, vec![-1.0, 0.5, 0.0], 2),
            (vec![0, 128, 42], 16, 2, 2, vec![-1.0, 0.0], 2),
            (vec![], 16, 1, 2, vec![0.0, 0.0], 0),
            (vec![192], 8, 1, 2, vec![0.5, 0.5, 0.0, 0.0], 1),
        ];
        for (bytes, bits, source_channels, actual_channels, expected, consumed) in cases {
            let remaining: VecDeque<_> = bytes.iter().skip(consumed).copied().collect();
            let buffer = Mutex::new(VecDeque::from(bytes));
            let position = portable_atomic::AtomicI64::new(7);
            let mut output = vec![42.0; expected.len()];
            write_output_samples(
                &mut output,
                &buffer,
                &position,
                &AtomicBool::new(true),
                bits,
                source_channels,
                actual_channels,
            );
            assert_eq!(output, expected);
            assert_eq!(
                position.load(Ordering::Relaxed),
                7 + i64::try_from(consumed).unwrap()
            );
            assert_eq!(*buffer.lock().unwrap(), remaining);
        }
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn test_paused_audio_preserves_buffer_and_position() {
        use std::collections::VecDeque;
        use std::sync::Mutex;
        use std::sync::atomic::AtomicBool;

        let buffer = Mutex::new(VecDeque::from([192]));
        let position = portable_atomic::AtomicI64::new(7);
        let running = AtomicBool::new(false);
        let mut output = [42.0; 2];
        write_output_samples(&mut output, &buffer, &position, &running, 8, 1, 2);
        read_input_samples(&[0.5], &buffer, &position, &running);
        assert_eq!(output, [0.0; 2]);
        assert_eq!(*buffer.lock().unwrap(), VecDeque::from([192]));
        assert_eq!(position.load(Ordering::Relaxed), 7);
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn test_input_samples_clamp_and_advance_position() {
        use std::collections::VecDeque;
        use std::sync::Mutex;
        use std::sync::atomic::AtomicBool;

        let buffer = Mutex::new(VecDeque::new());
        let position = portable_atomic::AtomicI64::new(7);
        read_input_samples(
            &[-2.0, -1.0, 0.0, 0.5, 1.0, 2.0],
            &buffer,
            &position,
            &AtomicBool::new(true),
        );
        assert_eq!(
            *buffer.lock().unwrap(),
            VecDeque::from([0, 0, 128, 192, 255, 255])
        );
        assert_eq!(position.load(Ordering::Relaxed), 13);
    }

    #[tokio::test]
    async fn test_n_available() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1)); // is_source
        let result = n_available(thread, params)?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_close() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1));
        let result = n_close(thread, params)?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_flush() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1));
        let result = n_flush(thread, params)?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_get_buffer_size() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1));
        let result = n_get_buffer_size(thread, params)?;
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
        let result = n_get_byte_position(thread, params)?;
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
        let result = n_is_still_draining(thread, params)?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_requires_servicing() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_requires_servicing(thread, Parameters::default())?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_service() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_service(thread, Parameters::default())?;
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
        let result = n_set_byte_position(thread, params)?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_start() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1));
        let result = n_start(thread, params)?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_n_stop() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Int(1));
        let result = n_stop(thread, params)?;
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
        let result = n_read(thread, params)?;
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
        let result = n_write(thread, params)?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }
}
