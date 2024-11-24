use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.CFont`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/CFont";
    registry.register(
        class_name,
        "createNativeFont",
        "(Ljava/lang/String;I)J",
        create_native_font,
    );
    registry.register(class_name, "disposeNativeFont", "(J)V", dispose_native_font);
    registry.register(
        class_name,
        "getCascadeList",
        "(JLjava/util/ArrayList;)V",
        get_cascade_list,
    );
    registry.register(
        class_name,
        "getLayoutTableCacheNative",
        "(J)J",
        get_layout_table_cache_native,
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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn create_native_font(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn dispose_native_font(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_cascade_list(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_layout_table_cache_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_table_bytes_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_weight_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_width_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
