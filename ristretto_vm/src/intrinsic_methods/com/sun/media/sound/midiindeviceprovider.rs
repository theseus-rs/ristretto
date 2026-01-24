use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/media/sound/MidiInDeviceProvider.nGetDescription(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub(crate) async fn n_get_description(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDeviceProvider.nGetDescription(I)Ljava/lang/String;")
}

#[intrinsic_method(
    "com/sun/media/sound/MidiInDeviceProvider.nGetName(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub(crate) async fn n_get_name(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDeviceProvider.nGetName(I)Ljava/lang/String;")
}

#[intrinsic_method("com/sun/media/sound/MidiInDeviceProvider.nGetNumDevices()I", Any)]
#[async_method]
pub(crate) async fn n_get_num_devices(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDeviceProvider.nGetNumDevices()I")
}

#[intrinsic_method(
    "com/sun/media/sound/MidiInDeviceProvider.nGetVendor(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub(crate) async fn n_get_vendor(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDeviceProvider.nGetVendor(I)Ljava/lang/String;")
}

#[intrinsic_method(
    "com/sun/media/sound/MidiInDeviceProvider.nGetVersion(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub(crate) async fn n_get_version(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiInDeviceProvider.nGetVersion(I)Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiInDeviceProvider.nGetDescription(I)Ljava/lang/String;"
    )]
    async fn test_n_get_description() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_description(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiInDeviceProvider.nGetName(I)Ljava/lang/String;"
    )]
    async fn test_n_get_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiInDeviceProvider.nGetNumDevices()I"
    )]
    async fn test_n_get_num_devices() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_num_devices(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiInDeviceProvider.nGetVendor(I)Ljava/lang/String;"
    )]
    async fn test_n_get_vendor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_vendor(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiInDeviceProvider.nGetVersion(I)Ljava/lang/String;"
    )]
    async fn test_n_get_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_version(thread, Parameters::default()).await;
    }
}
