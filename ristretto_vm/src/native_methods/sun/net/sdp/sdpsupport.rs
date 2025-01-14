use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.net.sdp.SdpSupport`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/net/sdp/SdpSupport";
    registry.register(class_name, "convert0", "(I)V", convert_0);
    registry.register(class_name, "create0", "()I", create_0);
}

#[async_recursion(?Send)]
async fn convert_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.net.sdp.SdpSupport.convert0(I)V")
}

#[async_recursion(?Send)]
async fn create_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.net.sdp.SdpSupport.create0()I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/net/sdp/SdpSupport";
        assert!(registry.method(class_name, "convert0", "(I)V").is_some());
        assert!(registry.method(class_name, "create0", "()I").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.net.sdp.SdpSupport.convert0(I)V")]
    async fn test_convert_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = convert_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.net.sdp.SdpSupport.create0()I")]
    async fn test_create_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_0(thread, Arguments::default()).await;
    }
}
