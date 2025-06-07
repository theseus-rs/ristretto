use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/font/CFont.createNativeFont(Ljava/lang/String;I)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn create_native_font(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.createNativeFont(Ljava/lang/String;I)J")
}

#[intrinsic_method("sun/font/CFont.disposeNativeFont(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn dispose_native_font(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.disposeNativeFont(J)V")
}

#[intrinsic_method("sun/font/CFont.getCGFontPtrNative(J)J", GreaterThan(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_cg_font_ptr_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getCGFontPtrNative(J)J")
}

#[intrinsic_method("sun/font/CFont.getCascadeList(JLjava/util/ArrayList;)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_cascade_list(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getCascadeList(JLjava/util/ArrayList;)V")
}

#[intrinsic_method(
    "sun/font/CFont.getLayoutTableCacheNative(J)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_layout_table_cache_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getLayoutTableCacheNative(J)J")
}

#[intrinsic_method("sun/font/CFont.getTableBytesNative(JI)[B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_table_bytes_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getTableBytesNative(JI)[B")
}

#[intrinsic_method("sun/font/CFont.getWeightNative(J)F", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_weight_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getWeightNative(J)F")
}

#[intrinsic_method("sun/font/CFont.getWidthNative(J)F", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_width_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getWidthNative(J)F")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CFont.createNativeFont(Ljava/lang/String;I)J"
    )]
    async fn test_create_native_font() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_native_font(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.CFont.disposeNativeFont(J)V")]
    async fn test_dispose_native_font() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_native_font(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.CFont.getCGFontPtrNative(J)J")]
    async fn test_get_cg_font_ptr_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cg_font_ptr_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CFont.getCascadeList(JLjava/util/ArrayList;)V"
    )]
    async fn test_get_cascade_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cascade_list(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.CFont.getLayoutTableCacheNative(J)J")]
    async fn test_get_layout_table_cache_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_layout_table_cache_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.CFont.getTableBytesNative(JI)[B")]
    async fn test_get_table_bytes_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_table_bytes_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.CFont.getWeightNative(J)F")]
    async fn test_get_weight_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_weight_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.CFont.getWidthNative(J)F")]
    async fn test_get_width_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_width_native(thread, Parameters::default()).await;
    }
}
