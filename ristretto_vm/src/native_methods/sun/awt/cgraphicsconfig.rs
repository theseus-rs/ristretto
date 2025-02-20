use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/CGraphicsConfig";

/// Register all native methods for `sun.awt.CGraphicsConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeGetBounds",
        "(I)Ljava/awt/geom/Rectangle2D;",
        native_get_bounds,
    );
}

#[async_recursion(?Send)]
async fn native_get_bounds(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsConfig.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.CGraphicsConfig.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;"
    )]
    async fn test_native_get_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_bounds(thread, Parameters::default()).await;
    }
}
