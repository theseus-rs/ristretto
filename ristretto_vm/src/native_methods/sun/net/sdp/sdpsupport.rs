use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/net/sdp/SdpSupport";

/// Register all native methods for `sun.net.sdp.SdpSupport`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "convert0", "(I)V", convert_0);
    registry.register(CLASS_NAME, "create0", "()I", create_0);
}

#[async_recursion(?Send)]
async fn convert_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.net.sdp.SdpSupport.convert0(I)V")
}

#[async_recursion(?Send)]
async fn create_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
