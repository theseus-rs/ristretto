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
    todo!()
}

#[async_recursion(?Send)]
async fn dispose_native_font(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_cg_font_ptr_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_cascade_list(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_layout_table_cache_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_table_bytes_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_weight_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_width_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
