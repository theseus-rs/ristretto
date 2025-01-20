use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/management/ClassLoadingImpl";

/// Register all native methods for `sun.management.ClassLoadingImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "setVerboseClass", "(Z)V", set_verbose_class);
}

#[async_recursion(?Send)]
async fn set_verbose_class(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.management.ClassLoadingImpl.setVerboseClass(Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ClassLoadingImpl.setVerboseClass(Z)V"
    )]
    async fn test_set_verbose_class() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_verbose_class(thread, Parameters::default()).await;
    }
}
