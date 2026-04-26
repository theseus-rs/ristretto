use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/media/sound/MidiOutDeviceProvider.nGetDescription(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn n_get_description<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.MidiOutDeviceProvider.nGetDescription(I)Ljava/lang/String;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/media/sound/MidiOutDeviceProvider.nGetName(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn n_get_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.MidiOutDeviceProvider.nGetName(I)Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/MidiOutDeviceProvider.nGetNumDevices()I", Any)]
#[async_method]
pub async fn n_get_num_devices<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.MidiOutDeviceProvider.nGetNumDevices()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/media/sound/MidiOutDeviceProvider.nGetVendor(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn n_get_vendor<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.MidiOutDeviceProvider.nGetVendor(I)Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/media/sound/MidiOutDeviceProvider.nGetVersion(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn n_get_version<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.MidiOutDeviceProvider.nGetVersion(I)Ljava/lang/String;".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_n_get_description() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_description(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com.sun.media.sound.MidiOutDeviceProvider.nGetDescription(I)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_get_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_name(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com.sun.media.sound.MidiOutDeviceProvider.nGetName(I)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_get_num_devices() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_num_devices(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.media.sound.MidiOutDeviceProvider.nGetNumDevices()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_get_vendor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_vendor(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com.sun.media.sound.MidiOutDeviceProvider.nGetVendor(I)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_get_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_version(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com.sun.media.sound.MidiOutDeviceProvider.nGetVersion(I)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }
}
