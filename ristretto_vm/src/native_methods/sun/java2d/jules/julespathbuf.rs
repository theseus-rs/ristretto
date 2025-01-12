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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/jules/JulesPathBuf";
        assert!(registry
            .method(class_name, "tesselateFillNative", "([I[BII[IIIIIII)[I")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "tesselateStrokeNative",
                "([I[BII[IIDIID[DIDDDDDDDIIII)[I"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.jules.JulesPathBuf.tesselateFillNative([I[BII[IIIIIII)[I"
    )]
    async fn test_tesselate_fill_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = tesselate_fill_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.jules.JulesPathBuf.tesselateStrokeNative([I[BII[IIDIID[DIDDDDDDDIIII)[I"
    )]
    async fn test_tesselate_stroke_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = tesselate_stroke_native(thread, Arguments::default()).await;
    }
}
