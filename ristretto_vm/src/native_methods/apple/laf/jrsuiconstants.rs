use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `apple.laf.JRSUIConstants`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "apple/laf/JRSUIConstants";
    registry.register(
        class_name,
        "getPtrForConstant",
        "(I)J",
        get_ptr_for_constant,
    );
}

#[async_recursion(?Send)]
async fn get_ptr_for_constant(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIConstants.getPtrForConstant(I)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "apple/laf/JRSUIConstants";
        assert!(registry
            .method(class_name, "getPtrForConstant", "(I)J")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.laf.JRSUIConstants.getPtrForConstant(I)J"
    )]
    async fn test_get_ptr_for_constant() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ptr_for_constant(thread, Arguments::default()).await;
    }
}
