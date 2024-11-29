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
    todo!()
}

#[async_recursion(?Send)]
async fn fill_tile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
