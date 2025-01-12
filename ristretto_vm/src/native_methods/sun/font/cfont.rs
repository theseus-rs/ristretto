use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };
const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `sun.font.CFont`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/CFont";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_8 {
        registry.register(
            class_name,
            "getLayoutTableCacheNative",
            "(J)J",
            get_layout_table_cache_native,
        );
    } else {
        registry.register(
            class_name,
            "getCGFontPtrNative",
            "(J)J",
            get_cg_font_ptr_native,
        );
    }

    if java_version <= JAVA_11 {
        registry.register(class_name, "disposeNativeFont", "(J)V", dispose_native_font);
    }

    registry.register(
        class_name,
        "createNativeFont",
        "(Ljava/lang/String;I)J",
        create_native_font,
    );
    registry.register(
        class_name,
        "getCascadeList",
        "(JLjava/util/ArrayList;)V",
        get_cascade_list,
    );
    registry.register(
        class_name,
        "getTableBytesNative",
        "(JI)[B",
        get_table_bytes_native,
    );
    registry.register(class_name, "getWeightNative", "(J)F", get_weight_native);
    registry.register(class_name, "getWidthNative", "(J)F", get_width_native);
}

#[async_recursion(?Send)]
async fn create_native_font(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.CFont.createNativeFont(Ljava/lang/String;I)J")
}

#[async_recursion(?Send)]
async fn dispose_native_font(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.CFont.disposeNativeFont(J)V")
}

#[async_recursion(?Send)]
async fn get_cg_font_ptr_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getCGFontPtrNative(J)J")
}

#[async_recursion(?Send)]
async fn get_cascade_list(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getCascadeList(JLjava/util/ArrayList;)V")
}

#[async_recursion(?Send)]
async fn get_layout_table_cache_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getLayoutTableCacheNative(J)J")
}

#[async_recursion(?Send)]
async fn get_table_bytes_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getTableBytesNative(JI)[B")
}

#[async_recursion(?Send)]
async fn get_weight_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getWeightNative(J)F")
}

#[async_recursion(?Send)]
async fn get_width_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.CFont.getWidthNative(J)F")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java8 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "sun/font/CFont";
        assert!(registry
            .method(class_name, "getLayoutTableCacheNative", "(J)J")
            .is_some());
        assert!(registry
            .method(class_name, "disposeNativeFont", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "createNativeFont", "(Ljava/lang/String;I)J")
            .is_some());
        assert!(registry
            .method(class_name, "getCascadeList", "(JLjava/util/ArrayList;)V")
            .is_some());
        assert!(registry
            .method(class_name, "getTableBytesNative", "(JI)[B")
            .is_some());
        assert!(registry
            .method(class_name, "getWeightNative", "(J)F")
            .is_some());
        assert!(registry
            .method(class_name, "getWidthNative", "(J)F")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CFont.createNativeFont(Ljava/lang/String;I)J")]
    async fn test_create_native_font() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_native_font(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CFont.disposeNativeFont(J)V")]
    async fn test_dispose_native_font() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_native_font(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CFont.getCGFontPtrNative(J)J")]
    async fn test_get_cg_font_ptr_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cg_font_ptr_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CFont.getCascadeList(JLjava/util/ArrayList;)V")]
    async fn test_get_cascade_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cascade_list(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CFont.getLayoutTableCacheNative(J)J")]
    async fn test_get_layout_table_cache_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_layout_table_cache_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CFont.getTableBytesNative(JI)[B")]
    async fn test_get_table_bytes_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_table_bytes_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CFont.getWeightNative(J)F")]
    async fn test_get_weight_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_weight_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CFont.getWidthNative(J)F")]
    async fn test_get_width_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_width_native(thread, Arguments::default()).await;
    }
}
