use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.SurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/SurfaceData";
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "isOpaqueGray",
        "(Ljava/awt/image/IndexColorModel;)Z",
        is_opaque_gray,
    );
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_opaque_gray(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.SurfaceData.isOpaqueGray(Ljava/awt/image/IndexColorModel;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/SurfaceData";
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry
            .method(
                class_name,
                "isOpaqueGray",
                "(Ljava/awt/image/IndexColorModel;)Z"
            )
            .is_some());
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.SurfaceData.isOpaqueGray(Ljava/awt/image/IndexColorModel;)Z"
    )]
    async fn test_is_opaque_gray() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_opaque_gray(thread, Arguments::default()).await;
    }
}
