use crate::Result;
use crate::native_methods::registry::{JAVA_8, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/CFont";

/// Register all native methods for `sun.font.CFont`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(
            CLASS_NAME,
            "getLayoutTableCacheNative",
            "(J)J",
            get_layout_table_cache_native,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "getCGFontPtrNative",
            "(J)J",
            get_cg_font_ptr_native,
        );
    }

    registry.register(CLASS_NAME, "disposeNativeFont", "(J)V", dispose_native_font);
    registry.register(
        CLASS_NAME,
        "createNativeFont",
        "(Ljava/lang/String;I)J",
        create_native_font,
    );
    registry.register(
        CLASS_NAME,
        "getCascadeList",
        "(JLjava/util/ArrayList;)V",
        get_cascade_list,
    );
    registry.register(
        CLASS_NAME,
        "getTableBytesNative",
        "(JI)[B",
        get_table_bytes_native,
    );
    registry.register(CLASS_NAME, "getWeightNative", "(J)F", get_weight_native);
    registry.register(CLASS_NAME, "getWidthNative", "(J)F", get_width_native);
}

#[async_recursion(?Send)]
async fn create_native_font(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.createNativeFont(Ljava/lang/String;I)J")
}

#[async_recursion(?Send)]
async fn dispose_native_font(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.disposeNativeFont(J)V")
}

#[async_recursion(?Send)]
async fn get_cg_font_ptr_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getCGFontPtrNative(J)J")
}

#[async_recursion(?Send)]
async fn get_cascade_list(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getCascadeList(JLjava/util/ArrayList;)V")
}

#[async_recursion(?Send)]
async fn get_layout_table_cache_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getLayoutTableCacheNative(J)J")
}

#[async_recursion(?Send)]
async fn get_table_bytes_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getTableBytesNative(JI)[B")
}

#[async_recursion(?Send)]
async fn get_weight_native(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getWeightNative(J)F")
}

#[async_recursion(?Send)]
async fn get_width_native(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
