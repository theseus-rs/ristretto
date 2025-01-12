use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.net.PortConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/net/PortConfig";
    registry.register(class_name, "getLower0", "()I", get_lower_0);
    registry.register(class_name, "getUpper0", "()I", get_upper_0);
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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/net/PortConfig";
        assert!(registry.method(class_name, "getLower0", "()I").is_some());
        assert!(registry.method(class_name, "getUpper0", "()I").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.net.PortConfig.getLower0()I")]
    async fn test_get_lower_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_lower_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.net.PortConfig.getUpper0()I")]
    async fn test_get_upper_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_upper_0(thread, Arguments::default()).await;
    }
}
