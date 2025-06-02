use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "apple/laf/JRSUIControl";

/// Register all intrinsic methods for `apple.laf.JRSUIControl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "disposeCFDictionary",
        "(J)V",
        dispose_cf_dictionary,
    );
    registry.register(CLASS_NAME, "getCFDictionary", "(Z)J", get_cf_dictionary);
    registry.register(
        CLASS_NAME,
        "getNativeHitPart",
        "(JJJDDDDDD)I",
        get_native_hit_part,
    );
    registry.register(
        CLASS_NAME,
        "getNativePartBounds",
        "([DJJJDDDDI)V",
        get_native_part_bounds,
    );
    registry.register(
        CLASS_NAME,
        "getNativeScrollBarOffsetChange",
        "(JJJDDDDIII)D",
        get_native_scroll_bar_offset_change,
    );
    registry.register(
        CLASS_NAME,
        "getPtrOfBuffer",
        "(Ljava/nio/ByteBuffer;)J",
        get_ptr_of_buffer,
    );
    registry.register(CLASS_NAME, "initNativeJRSUI", "()I", init_native_jrsui);
    registry.register(
        CLASS_NAME,
        "paintChangesImage",
        "([IIIJJJDDDDJ)I",
        paint_changes_image,
    );
    registry.register(
        CLASS_NAME,
        "paintChangesToCGContext",
        "(JJJJDDDDJ)I",
        paint_changes_to_cg_context,
    );
    registry.register(CLASS_NAME, "paintImage", "([IIIJJJDDDD)I", paint_image);
    registry.register(
        CLASS_NAME,
        "paintToCGContext",
        "(JJJJDDDD)I",
        paint_to_cg_context,
    );
    registry.register(CLASS_NAME, "syncChanges", "(JJ)I", sync_changes);
}

#[async_recursion(?Send)]
async fn dispose_cf_dictionary(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.disposeCFDictionary(J)V")
}

#[async_recursion(?Send)]
async fn get_cf_dictionary(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.getCFDictionary(Z)J")
}

#[async_recursion(?Send)]
async fn get_native_hit_part(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.getNativeHitPart(JJJDDDDDD)I")
}

#[async_recursion(?Send)]
async fn get_native_part_bounds(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.getNativePartBounds([DJJJDDDDI)V")
}

#[async_recursion(?Send)]
async fn get_native_scroll_bar_offset_change(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.getNativeScrollBarOffsetChange(JJJDDDDIII)D")
}

#[async_recursion(?Send)]
async fn get_ptr_of_buffer(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.getPtrOfBuffer(Ljava/nio/ByteBuffer;)J")
}

#[async_recursion(?Send)]
async fn init_native_jrsui(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.initNativeJRSUI()I")
}

#[async_recursion(?Send)]
async fn paint_changes_image(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.paintChangesImage([IIIJJJDDDDJ)I")
}

#[async_recursion(?Send)]
async fn paint_changes_to_cg_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.paintChangesToCGContext(JJJJDDDDJ)I")
}

#[async_recursion(?Send)]
async fn paint_image(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.paintImage([IIIJJJDDDD)I")
}

#[async_recursion(?Send)]
async fn paint_to_cg_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.paintToCGContext(JJJJDDDD)I")
}

#[async_recursion(?Send)]
async fn sync_changes(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.syncChanges(JJ)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.laf.JRSUIControl.disposeCFDictionary(J)V"
    )]
    async fn test_dispose_cf_dictionary() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_cf_dictionary(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: apple.laf.JRSUIControl.getCFDictionary(Z)J")]
    async fn test_get_cf_dictionary() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cf_dictionary(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.laf.JRSUIControl.getNativeHitPart(JJJDDDDDD)I"
    )]
    async fn test_get_native_hit_part() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_hit_part(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.laf.JRSUIControl.getNativePartBounds([DJJJDDDDI)V"
    )]
    async fn test_get_native_part_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_part_bounds(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.laf.JRSUIControl.getNativeScrollBarOffsetChange(JJJDDDDIII)D"
    )]
    async fn test_get_native_scroll_bar_offset_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_scroll_bar_offset_change(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.laf.JRSUIControl.getPtrOfBuffer(Ljava/nio/ByteBuffer;)J"
    )]
    async fn test_get_ptr_of_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ptr_of_buffer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: apple.laf.JRSUIControl.initNativeJRSUI()I")]
    async fn test_init_native_jrsui() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_native_jrsui(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.laf.JRSUIControl.paintChangesImage([IIIJJJDDDDJ)I"
    )]
    async fn test_paint_changes_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = paint_changes_image(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.laf.JRSUIControl.paintChangesToCGContext(JJJJDDDDJ)I"
    )]
    async fn test_paint_changes_to_cg_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = paint_changes_to_cg_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.laf.JRSUIControl.paintImage([IIIJJJDDDD)I"
    )]
    async fn test_paint_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = paint_image(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.laf.JRSUIControl.paintToCGContext(JJJJDDDD)I"
    )]
    async fn test_paint_to_cg_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = paint_to_cg_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: apple.laf.JRSUIControl.syncChanges(JJ)I")]
    async fn test_sync_changes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sync_changes(thread, Parameters::default()).await;
    }
}
