use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.management.ClassLoadingImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/management/ClassLoadingImpl";
    registry.register(class_name, "setVerboseClass", "(Z)V", set_verbose_class);
}

#[async_recursion(?Send)]
async fn set_verbose_class(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.ClassLoadingImpl.setVerboseClass(Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/management/ClassLoadingImpl";
        assert!(registry
            .method(class_name, "setVerboseClass", "(Z)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.ClassLoadingImpl.setVerboseClass(Z)V")]
    async fn test_set_verbose_class() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_verbose_class(thread, Arguments::default()).await;
    }
}
