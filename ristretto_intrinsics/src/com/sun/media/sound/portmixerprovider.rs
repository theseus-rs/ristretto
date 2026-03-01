use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/sun/media/sound/PortMixerProvider.nGetNumDevices()I", Any)]
#[async_method]
pub async fn n_get_num_devices<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Return 0 because we don't have native port mixer support.
    // PortMixerInfo objects can't be created without native library support.
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "com/sun/media/sound/PortMixerProvider.nNewPortMixerInfo(I)Lcom/sun/media/sound/PortMixerProvider$PortMixerInfo;",
    Any
)]
#[async_method]
pub async fn n_new_port_mixer_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    // PortMixerInfo is constructed by Java code using the native index.
    // Return null to indicate the info should be constructed by Java.
    Ok(Some(Value::Object(None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_n_get_num_devices() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_num_devices(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_new_port_mixer_info() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Int(0));
        let result = n_new_port_mixer_info(thread, params).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }
}
