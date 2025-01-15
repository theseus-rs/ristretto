use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/net/PortConfig";

/// Register all native methods for `sun.net.PortConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "getLower0", "()I", get_lower_0);
    registry.register(CLASS_NAME, "getUpper0", "()I", get_upper_0);
}

#[async_recursion(?Send)]
async fn get_lower_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.net.PortConfig.getLower0()I")
}

#[async_recursion(?Send)]
async fn get_upper_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.net.PortConfig.getUpper0()I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.net.PortConfig.getLower0()I")]
    async fn test_get_lower_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_lower_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.net.PortConfig.getUpper0()I")]
    async fn test_get_upper_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_upper_0(thread, Arguments::default()).await;
    }
}
