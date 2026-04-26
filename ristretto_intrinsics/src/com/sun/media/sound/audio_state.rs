use parking_lot::RwLock;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};

/// A wrapper that allows `!Send + !Sync` types to be stored in `Send + Sync` containers.
///
/// # Safety
///
/// This is only used on WASM targets, which are single-threaded. The `Send` and `Sync`
/// implementations are safe because there is no concurrent access in a single-threaded environment.
#[cfg(target_family = "wasm")]
pub(crate) struct WasmValue<T>(T);

#[cfg(target_family = "wasm")]
// SAFETY: WASM targets are single-threaded; no concurrent access is possible.
#[expect(unsafe_code)]
unsafe impl<T> Send for WasmValue<T> {}

#[cfg(target_family = "wasm")]
// SAFETY: WASM targets are single-threaded; no concurrent access is possible.
#[expect(unsafe_code)]
unsafe impl<T> Sync for WasmValue<T> {}

#[cfg(target_family = "wasm")]
impl<T> WasmValue<T> {
    /// Create a new `WasmValue` wrapping the given value.
    pub fn new(value: T) -> Self {
        Self(value)
    }

    /// Get a reference to the inner value.
    pub fn inner(&self) -> &T {
        &self.0
    }
}

/// Internal handle for audio streams, used to manage state across FFI boundaries.
pub(crate) struct AudioStreamHandle {
    pub buffer: Arc<Mutex<VecDeque<u8>>>,
    pub sample_rate: u32,
    pub channels: u16,
    pub sample_size_bits: u32,
    pub frame_size: u32,
    pub byte_position: Arc<AtomicI64>,
    pub is_source: bool,
    pub is_running: Arc<AtomicBool>,
    pub buffer_size_bytes: usize,
    pub close_signal: Arc<AtomicBool>,
    #[cfg(target_family = "wasm")]
    pub audio_context: Option<WasmValue<web_sys::AudioContext>>,
    #[cfg(target_family = "wasm")]
    pub schedule_time: f64,
}

/// Internal handle for MIDI input streams, used to manage state across FFI boundaries.
#[cfg(not(target_family = "wasm"))]
pub(crate) struct MidiInHandle {
    pub messages: Arc<Mutex<VecDeque<Vec<u8>>>>,
    pub start_time: std::time::Instant,
    pub is_started: bool,
    pub close_signal: Arc<AtomicBool>,
}

/// Internal handle for MIDI input streams in WASM, used to manage state across FFI boundaries.
#[cfg(target_family = "wasm")]
pub(crate) struct MidiInHandle {
    pub start_time_ms: f64,
    pub messages: Vec<Vec<u8>>,
    pub is_started: bool,
    pub close_signal: Arc<AtomicBool>,
    pub midi_input: Option<WasmValue<web_sys::MidiInput>>,
}

/// Internal handle for MIDI output streams, used to manage state across FFI boundaries.
#[cfg(not(target_family = "wasm"))]
pub(crate) struct MidiOutHandle {
    pub connection: Mutex<Option<midir::MidiOutputConnection>>,
    pub start_time: std::time::Instant,
}

/// Internal handle for MIDI output streams in WASM, used to manage state across FFI boundaries.
#[cfg(target_family = "wasm")]
pub(crate) struct MidiOutHandle {
    pub start_time_ms: f64,
    pub midi_output: Option<WasmValue<web_sys::MidiOutput>>,
}

/// Internal handle for port mixers, used to manage state across FFI boundaries.
pub(crate) struct PortMixerHandle {
    pub name: String,
    pub volume: f32,
    pub mute: bool,
}

/// Per-VM audio and MIDI state.
pub(crate) struct AudioState {
    audio_handles: RwLock<HashMap<i64, AudioStreamHandle>>,
    next_audio_id: AtomicI64,
    midi_in_handles: RwLock<HashMap<i64, MidiInHandle>>,
    next_midi_in_id: AtomicI64,
    midi_out_handles: RwLock<HashMap<i64, MidiOutHandle>>,
    next_midi_out_id: AtomicI64,
    port_mixer_handles: RwLock<HashMap<i64, PortMixerHandle>>,
    next_port_mixer_id: AtomicI64,
}

impl AudioState {
    /// Create a new `AudioState` with empty maps and ID counters starting at 1.
    pub fn new() -> Self {
        Self {
            audio_handles: RwLock::new(HashMap::new()),
            next_audio_id: AtomicI64::new(1),
            midi_in_handles: RwLock::new(HashMap::new()),
            next_midi_in_id: AtomicI64::new(1),
            midi_out_handles: RwLock::new(HashMap::new()),
            next_midi_out_id: AtomicI64::new(1),
            port_mixer_handles: RwLock::new(HashMap::new()),
            next_port_mixer_id: AtomicI64::new(1),
        }
    }

    /// Get a reference to the audio stream handles map.
    pub fn audio_handles(&self) -> &RwLock<HashMap<i64, AudioStreamHandle>> {
        &self.audio_handles
    }

    /// Generate a new unique ID for an audio stream.
    pub fn next_audio_id(&self) -> i64 {
        self.next_audio_id.fetch_add(1, Ordering::Relaxed)
    }

    /// Get a reference to the MIDI input stream handles map.
    pub fn midi_in_handles(&self) -> &RwLock<HashMap<i64, MidiInHandle>> {
        &self.midi_in_handles
    }

    /// Generate a new unique ID for a MIDI input stream.
    pub fn next_midi_in_id(&self) -> i64 {
        self.next_midi_in_id.fetch_add(1, Ordering::Relaxed)
    }

    /// Get a reference to the MIDI output stream handles map.
    pub fn midi_out_handles(&self) -> &RwLock<HashMap<i64, MidiOutHandle>> {
        &self.midi_out_handles
    }

    /// Generate a new unique ID for a MIDI output stream.
    pub fn next_midi_out_id(&self) -> i64 {
        self.next_midi_out_id.fetch_add(1, Ordering::Relaxed)
    }

    /// Get a reference to the port mixer handles map.
    pub fn port_mixer_handles(&self) -> &RwLock<HashMap<i64, PortMixerHandle>> {
        &self.port_mixer_handles
    }

    pub fn next_port_mixer_id(&self) -> i64 {
        self.next_port_mixer_id.fetch_add(1, Ordering::Relaxed)
    }
}

/// Get a list of all available audio devices on the system.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn get_all_audio_devices() -> Vec<cpal::Device> {
    use cpal::traits::HostTrait;
    let host = cpal::default_host();
    let mut devices = Vec::new();
    if let Ok(output_devices) = host.output_devices() {
        devices.extend(output_devices);
    }
    if let Ok(input_devices) = host.input_devices() {
        devices.extend(input_devices);
    }
    devices
}

/// Get the default audio output device, if available.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn get_audio_device_by_index(index: usize) -> Option<cpal::Device> {
    let devices = get_all_audio_devices();
    devices.into_iter().nth(index)
}

/// Get the name of the audio device at the specified index, if available.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn get_midi_in_port_count() -> usize {
    midir::MidiInput::new("ristretto-probe")
        .map(|midi_in| midi_in.port_count())
        .unwrap_or(0)
}

/// Get the name of the MIDI input port at the specified index, if available.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn get_midi_in_port_name(index: usize) -> Option<String> {
    let midi_in = midir::MidiInput::new("ristretto-probe").ok()?;
    let ports = midi_in.ports();
    let port = ports.get(index)?;
    midi_in.port_name(port).ok()
}

/// Get the number of available MIDI output ports on the system.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn get_midi_out_port_count() -> usize {
    midir::MidiOutput::new("ristretto-probe")
        .map(|midi_out| midi_out.port_count())
        .unwrap_or(0)
}

/// Get the name of the MIDI output port at the specified index, if available.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn get_midi_out_port_name(index: usize) -> Option<String> {
    let midi_out = midir::MidiOutput::new("ristretto-probe").ok()?;
    let ports = midi_out.ports();
    let port = ports.get(index)?;
    midi_out.port_name(port).ok()
}

/// Get the current time in microseconds from the browser's Performance API.
#[cfg(target_family = "wasm")]
pub(crate) fn performance_now_micros() -> i64 {
    use wasm_bindgen::JsCast;
    web_sys::window()
        .and_then(|w| {
            js_sys::Reflect::get(&w, &"performance".into())
                .ok()
                .and_then(|p| {
                    p.dyn_ref::<web_sys::Performance>().map(|p| {
                        #[expect(clippy::cast_possible_truncation)]
                        let micros = (p.now() * 1000.0) as i64;
                        micros
                    })
                })
        })
        .unwrap_or(0)
}
