use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("apple/laf/JRSUIControl.disposeCFDictionary(J)V", Any)]
#[async_method]
pub(crate) async fn dispose_cf_dictionary(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.disposeCFDictionary(J)V")
}

#[intrinsic_method("apple/laf/JRSUIControl.getCFDictionary(Z)J", Any)]
#[async_method]
pub(crate) async fn get_cf_dictionary(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.getCFDictionary(Z)J")
}

#[intrinsic_method("apple/laf/JRSUIControl.getNativeHitPart(JJJDDDDDD)I", Any)]
#[async_method]
pub(crate) async fn get_native_hit_part(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.getNativeHitPart(JJJDDDDDD)I")
}

#[intrinsic_method("apple/laf/JRSUIControl.getNativePartBounds([DJJJDDDDI)V", Any)]
#[async_method]
pub(crate) async fn get_native_part_bounds(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.getNativePartBounds([DJJJDDDDI)V")
}

#[intrinsic_method(
    "apple/laf/JRSUIControl.getNativeScrollBarOffsetChange(JJJDDDDIII)D",
    Any
)]
#[async_method]
pub(crate) async fn get_native_scroll_bar_offset_change(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.getNativeScrollBarOffsetChange(JJJDDDDIII)D")
}

#[intrinsic_method("apple/laf/JRSUIControl.getPtrOfBuffer(Ljava/nio/ByteBuffer;)J", Any)]
#[async_method]
pub(crate) async fn get_ptr_of_buffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.getPtrOfBuffer(Ljava/nio/ByteBuffer;)J")
}

#[intrinsic_method("apple/laf/JRSUIControl.initNativeJRSUI()I", Any)]
#[async_method]
pub(crate) async fn init_native_jrsui(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.initNativeJRSUI()I")
}

#[intrinsic_method("apple/laf/JRSUIControl.paintChangesImage([IIIJJJDDDDJ)I", Any)]
#[async_method]
pub(crate) async fn paint_changes_image(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.paintChangesImage([IIIJJJDDDDJ)I")
}

#[intrinsic_method("apple/laf/JRSUIControl.paintChangesToCGContext(JJJJDDDDJ)I", Any)]
#[async_method]
pub(crate) async fn paint_changes_to_cg_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.paintChangesToCGContext(JJJJDDDDJ)I")
}

#[intrinsic_method("apple/laf/JRSUIControl.paintImage([IIIJJJDDDD)I", Any)]
#[async_method]
pub(crate) async fn paint_image(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.paintImage([IIIJJJDDDD)I")
}

#[intrinsic_method("apple/laf/JRSUIControl.paintToCGContext(JJJJDDDD)I", Any)]
#[async_method]
pub(crate) async fn paint_to_cg_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIControl.paintToCGContext(JJJJDDDD)I")
}

#[intrinsic_method("apple/laf/JRSUIControl.syncChanges(JJ)I", Any)]
#[async_method]
pub(crate) async fn sync_changes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
