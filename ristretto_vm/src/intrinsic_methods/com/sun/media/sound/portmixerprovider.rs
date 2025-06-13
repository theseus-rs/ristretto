use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("com/sun/media/sound/PortMixerProvider.nGetNumDevices()I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn n_get_num_devices(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixerProvider.nGetNumDevices()I")
}

#[intrinsic_method(
    "com/sun/media/sound/PortMixerProvider.nNewPortMixerInfo(I)Lcom/sun/media/sound/PortMixerProvider$PortMixerInfo;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn n_new_port_mixer_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.media.sound.PortMixerProvider.nNewPortMixerInfo(I)Lcom/sun/media/sound/PortMixerProvider$PortMixerInfo;"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixerProvider.nGetNumDevices()I"
    )]
    async fn test_n_get_num_devices() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_num_devices(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixerProvider.nNewPortMixerInfo(I)Lcom/sun/media/sound/PortMixerProvider$PortMixerInfo;"
    )]
    async fn test_n_new_port_mixer_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_new_port_mixer_info(thread, Parameters::default()).await;
    }
}
