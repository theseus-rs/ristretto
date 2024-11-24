use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.SunLayoutEngine`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/SunLayoutEngine";
    registry.register(class_name, "initGVIDs", "()V", init_gvi_ds);
    registry.register(class_name, "nativeLayout", "(Lsun/font/Font2D;Lsun/font/FontStrike;[FII[CIIIIIIILjava/awt/geom/Point2D$Float;Lsun/font/GlyphLayout$GVData;JJ)V", native_layout);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_gvi_ds(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_layout(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
