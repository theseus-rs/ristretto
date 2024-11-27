use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.jules.JulesPathBuf`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/jules/JulesPathBuf";
    registry.register(
        class_name,
        "tesselateFillNative",
        "([I[BII[IIIIIII)[I",
        tesselate_fill_native,
    );
    registry.register(
        class_name,
        "tesselateStrokeNative",
        "([I[BII[IIDIID[DIDDDDDDDIIII)[I",
        tesselate_stroke_native,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn tesselate_fill_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn tesselate_stroke_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
