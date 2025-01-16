use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/jules/JulesPathBuf";

/// Register all native methods for `sun.java2d.jules.JulesPathBuf`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "tesselateFillNative",
        "([I[BII[IIIIIII)[I",
        tesselate_fill_native,
    );
    registry.register(
        CLASS_NAME,
        "tesselateStrokeNative",
        "([I[BII[IIDIID[DIDDDDDDDIIII)[I",
        tesselate_stroke_native,
    );
}

#[async_recursion(?Send)]
async fn tesselate_fill_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.jules.JulesPathBuf.tesselateFillNative([I[BII[IIIIIII)[I")
}

#[async_recursion(?Send)]
async fn tesselate_stroke_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.jules.JulesPathBuf.tesselateStrokeNative([I[BII[IIDIID[DIDDDDDDDIIII)[I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.jules.JulesPathBuf.tesselateFillNative([I[BII[IIIIIII)[I"
    )]
    async fn test_tesselate_fill_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = tesselate_fill_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.jules.JulesPathBuf.tesselateStrokeNative([I[BII[IIDIID[DIDDDDDDDIIII)[I"
    )]
    async fn test_tesselate_stroke_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = tesselate_stroke_native(thread, Arguments::default()).await;
    }
}
