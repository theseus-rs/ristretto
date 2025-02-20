use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/SurfaceData";

/// Register all native methods for `sun.java2d.SurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(
        CLASS_NAME,
        "isOpaqueGray",
        "(Ljava/awt/image/IndexColorModel;)Z",
        is_opaque_gray,
    );
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_opaque_gray(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.SurfaceData.isOpaqueGray(Ljava/awt/image/IndexColorModel;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.SurfaceData.isOpaqueGray(Ljava/awt/image/IndexColorModel;)Z"
    )]
    async fn test_is_opaque_gray() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_opaque_gray(thread, Parameters::default()).await;
    }
}
