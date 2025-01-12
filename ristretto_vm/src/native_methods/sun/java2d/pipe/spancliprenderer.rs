use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.pipe.SpanClipRenderer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/pipe/SpanClipRenderer";
    registry.register(
        class_name,
        "eraseTile",
        "(Lsun/java2d/pipe/RegionIterator;[BII[I)V",
        erase_tile,
    );
    registry.register(
        class_name,
        "fillTile",
        "(Lsun/java2d/pipe/RegionIterator;[BII[I)V",
        fill_tile,
    );
    registry.register(
        class_name,
        "initIDs",
        "(Ljava/lang/Class;Ljava/lang/Class;)V",
        init_ids,
    );
}

#[async_recursion(?Send)]
async fn erase_tile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.SpanClipRenderer.eraseTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V")
}

#[async_recursion(?Send)]
async fn fill_tile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.SpanClipRenderer.fillTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/pipe/SpanClipRenderer";
        assert!(registry
            .method(
                class_name,
                "eraseTile",
                "(Lsun/java2d/pipe/RegionIterator;[BII[I)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "fillTile",
                "(Lsun/java2d/pipe/RegionIterator;[BII[I)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "initIDs",
                "(Ljava/lang/Class;Ljava/lang/Class;)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.pipe.SpanClipRenderer.eraseTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V"
    )]
    async fn test_erase_tile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = erase_tile(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.pipe.SpanClipRenderer.fillTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V"
    )]
    async fn test_fill_tile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fill_tile(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
