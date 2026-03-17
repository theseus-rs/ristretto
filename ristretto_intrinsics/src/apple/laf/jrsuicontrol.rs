use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("apple/laf/JRSUIControl.disposeCFDictionary(J)V", Any)]
#[async_method]
pub async fn dispose_cf_dictionary<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.disposeCFDictionary(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.getCFDictionary(Z)J", Any)]
#[async_method]
pub async fn get_cf_dictionary<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("apple.laf.JRSUIControl.getCFDictionary(Z)J".to_string())
            .into(),
    )
}

#[intrinsic_method("apple/laf/JRSUIControl.getNativeHitPart(JJJDDDDDD)I", Any)]
#[async_method]
pub async fn get_native_hit_part<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.getNativeHitPart(JJJDDDDDD)I".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.getNativePartBounds([DJJJDDDDI)V", Any)]
#[async_method]
pub async fn get_native_part_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.getNativePartBounds([DJJJDDDDI)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "apple/laf/JRSUIControl.getNativeScrollBarOffsetChange(JJJDDDDIII)D",
    Any
)]
#[async_method]
pub async fn get_native_scroll_bar_offset_change<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.getNativeScrollBarOffsetChange(JJJDDDDIII)D".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.getPtrOfBuffer(Ljava/nio/ByteBuffer;)J", Any)]
#[async_method]
pub async fn get_ptr_of_buffer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.getPtrOfBuffer(Ljava/nio/ByteBuffer;)J".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.initNativeJRSUI()I", Any)]
#[async_method]
pub async fn init_native_jrsui<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("apple.laf.JRSUIControl.initNativeJRSUI()I".to_string())
            .into(),
    )
}

#[intrinsic_method("apple/laf/JRSUIControl.paintChangesImage([IIIJJJDDDDJ)I", Any)]
#[async_method]
pub async fn paint_changes_image<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.paintChangesImage([IIIJJJDDDDJ)I".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.paintChangesToCGContext(JJJJDDDDJ)I", Any)]
#[async_method]
pub async fn paint_changes_to_cg_context<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.paintChangesToCGContext(JJJJDDDDJ)I".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.paintImage([IIIJJJDDDD)I", Any)]
#[async_method]
pub async fn paint_image<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.paintImage([IIIJJJDDDD)I".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.paintToCGContext(JJJJDDDD)I", Any)]
#[async_method]
pub async fn paint_to_cg_context<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.paintToCGContext(JJJJDDDD)I".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.syncChanges(JJ)I", Any)]
#[async_method]
pub async fn sync_changes<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("apple.laf.JRSUIControl.syncChanges(JJ)I".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dispose_cf_dictionary() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose_cf_dictionary(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_cf_dictionary() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_cf_dictionary(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_native_hit_part() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_hit_part(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_native_part_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_part_bounds(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_native_scroll_bar_offset_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_scroll_bar_offset_change(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_ptr_of_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_ptr_of_buffer(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_native_jrsui() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_native_jrsui(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_paint_changes_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = paint_changes_image(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_paint_changes_to_cg_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = paint_changes_to_cg_context(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_paint_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = paint_image(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_paint_to_cg_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = paint_to_cg_context(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_sync_changes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = sync_changes(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
