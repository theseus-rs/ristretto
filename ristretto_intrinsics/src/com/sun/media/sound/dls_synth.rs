use coreaudio_sys as sys;
use std::fmt::{Display, Formatter};
use std::mem::{MaybeUninit, size_of};
use std::ptr;

/// An error returned by the macOS DLS synthesizer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Error {
    operation: &'static str,
    status: sys::OSStatus,
}

impl Error {
    fn new(operation: &'static str, status: sys::OSStatus) -> Self {
        Self { operation, status }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Core Audio operation {} failed with status {}",
            self.operation, self.status
        )
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
struct AudioUnit {
    instance: sys::AudioUnit,
}

unsafe impl Send for AudioUnit {}

impl AudioUnit {
    fn new(component_type: u32, component_subtype: u32) -> Result<Self, Error> {
        let description = sys::AudioComponentDescription {
            componentType: component_type,
            componentSubType: component_subtype,
            componentManufacturer: sys::kAudioUnitManufacturer_Apple,
            componentFlags: 0,
            componentFlagsMask: 0,
        };
        unsafe {
            let component = sys::AudioComponentFindNext(ptr::null_mut(), &raw const description);
            if component.is_null() {
                return Err(Error::new("AudioComponentFindNext", -1));
            }
            let mut instance = MaybeUninit::<sys::AudioUnit>::uninit();
            let status = sys::AudioComponentInstanceNew(component, instance.as_mut_ptr());
            if status != 0 {
                return Err(Error::new("AudioComponentInstanceNew", status));
            }
            Ok(Self {
                instance: instance.assume_init(),
            })
        }
    }

    fn initialize(&self) -> Result<(), Error> {
        let status = unsafe { sys::AudioUnitInitialize(self.instance) };
        if status == 0 {
            Ok(())
        } else {
            Err(Error::new("AudioUnitInitialize", status))
        }
    }
}

impl Drop for AudioUnit {
    fn drop(&mut self) {
        unsafe {
            let _ = sys::AudioUnitUninitialize(self.instance);
            let _ = sys::AudioComponentInstanceDispose(self.instance);
        }
    }
}

/// Apple's native General MIDI DLS synthesizer connected to the default audio output.
#[derive(Debug)]
pub struct DlsSynth {
    synth: AudioUnit,
    output: AudioUnit,
}

impl DlsSynth {
    /// Create and start a DLS synthesizer.
    pub fn new() -> Result<Self, Error> {
        let synth = AudioUnit::new(
            sys::kAudioUnitType_MusicDevice,
            sys::kAudioUnitSubType_DLSSynth,
        )?;
        let output = AudioUnit::new(
            sys::kAudioUnitType_Output,
            sys::kAudioUnitSubType_DefaultOutput,
        )?;
        let connection = sys::AudioUnitConnection {
            sourceAudioUnit: synth.instance,
            sourceOutputNumber: 0,
            destInputNumber: 0,
        };
        let connection_size = u32::try_from(size_of::<sys::AudioUnitConnection>())
            .map_err(|_| Error::new("AudioUnitConnection size", -1))?;
        let status = unsafe {
            sys::AudioUnitSetProperty(
                output.instance,
                sys::kAudioUnitProperty_MakeConnection,
                sys::kAudioUnitScope_Input,
                0,
                (&raw const connection).cast(),
                connection_size,
            )
        };
        if status != 0 {
            return Err(Error::new("AudioUnitSetProperty", status));
        }
        synth.initialize()?;
        output.initialize()?;
        let status = unsafe { sys::AudioOutputUnitStart(output.instance) };
        if status != 0 {
            return Err(Error::new("AudioOutputUnitStart", status));
        }
        Ok(Self { synth, output })
    }

    /// Send a three-byte MIDI channel message to the synthesizer.
    pub fn send_short_message(&mut self, status: u8, data1: u8, data2: u8) -> Result<(), Error> {
        let result = unsafe {
            sys::MusicDeviceMIDIEvent(
                self.synth.instance,
                u32::from(status),
                u32::from(data1),
                u32::from(data2),
                0,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(Error::new("MusicDeviceMIDIEvent", result))
        }
    }

    /// Send a system-exclusive MIDI message to the synthesizer.
    pub fn send_system_exclusive(&mut self, message: &[u8]) -> Result<(), Error> {
        let length = u32::try_from(message.len())
            .map_err(|_| Error::new("system-exclusive message length", -1))?;
        let result =
            unsafe { sys::MusicDeviceSysEx(self.synth.instance, message.as_ptr(), length) };
        if result == 0 {
            Ok(())
        } else {
            Err(Error::new("MusicDeviceSysEx", result))
        }
    }
}

impl Drop for DlsSynth {
    fn drop(&mut self) {
        let _ = unsafe { sys::AudioOutputUnitStop(self.output.instance) };
    }
}
