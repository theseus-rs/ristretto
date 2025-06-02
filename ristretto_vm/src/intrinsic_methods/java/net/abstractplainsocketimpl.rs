use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_11, JAVA_17, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/net/AbstractPlainSocketImpl";

/// Register all intrinsic methods for `java.net.AbstractPlainSocketImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_11 && registry.java_major_version() <= JAVA_17 {
        registry.register(
            CLASS_NAME,
            "isReusePortAvailable0",
            "()Z",
            is_reuse_port_available_0,
        );
    }
}

#[async_recursion(?Send)]
async fn is_reuse_port_available_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.AbstractPlainSocketImpl.isReusePortAvailable0()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.AbstractPlainSocketImpl.isReusePortAvailable0()Z"
    )]
    async fn test_is_reuse_port_available_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_reuse_port_available_0(thread, Parameters::default()).await;
    }
}
