use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CPrinterSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CPrinterSurfaceData";
    registry.register(class_name, "_flush", "()V", flush);
    registry.register(
        class_name,
        "initOps",
        "(JLjava/nio/ByteBuffer;[Ljava/lang/Object;II)V",
        init_ops,
    );
}

#[async_recursion(?Send)]
async fn flush(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterSurfaceData._flush()V")
}

#[async_recursion(?Send)]
async fn init_ops(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterSurfaceData.initOps(JLjava/nio/ByteBuffer;[Ljava/lang/Object;II)V")
}
