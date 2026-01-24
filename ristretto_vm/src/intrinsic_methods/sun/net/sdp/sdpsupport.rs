use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/net/sdp/SdpSupport.convert0(I)V", Any)]
#[async_method]
pub(crate) async fn convert_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.sdp.SdpSupport.convert0(I)V")
}

#[intrinsic_method("sun/net/sdp/SdpSupport.create0()I", Any)]
#[async_method]
pub(crate) async fn create_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.sdp.SdpSupport.create0()I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.net.sdp.SdpSupport.convert0(I)V")]
    async fn test_convert_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = convert_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.net.sdp.SdpSupport.create0()I")]
    async fn test_create_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_0(thread, Parameters::default()).await;
    }
}
