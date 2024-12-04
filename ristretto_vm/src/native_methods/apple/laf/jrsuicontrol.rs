use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `apple.laf.JRSUIControl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "apple/laf/JRSUIControl";
    registry.register(
        class_name,
        "disposeCFDictionary",
        "(J)V",
        dispose_cf_dictionary,
    );
    registry.register(class_name, "getCFDictionary", "(Z)J", get_cf_dictionary);
    registry.register(
        class_name,
        "getNativeHitPart",
        "(JJJDDDDDD)I",
        get_native_hit_part,
    );
    registry.register(
        class_name,
        "getNativePartBounds",
        "([DJJJDDDDI)V",
        get_native_part_bounds,
    );
    registry.register(
        class_name,
        "getNativeScrollBarOffsetChange",
        "(JJJDDDDIII)D",
        get_native_scroll_bar_offset_change,
    );
    registry.register(
        class_name,
        "getPtrOfBuffer",
        "(Ljava/nio/ByteBuffer;)J",
        get_ptr_of_buffer,
    );
    registry.register(class_name, "initNativeJRSUI", "()I", init_native_jrsui);
    registry.register(
        class_name,
        "paintChangesImage",
        "([IIIJJJDDDDJ)I",
        paint_changes_image,
    );
    registry.register(
        class_name,
        "paintChangesToCGContext",
        "(JJJJDDDDJ)I",
        paint_changes_to_cg_context,
    );
    registry.register(class_name, "paintImage", "([IIIJJJDDDD)I", paint_image);
    registry.register(
        class_name,
        "paintToCGContext",
        "(JJJJDDDD)I",
        paint_to_cg_context,
    );
    registry.register(class_name, "syncChanges", "(JJ)I", sync_changes);
}

#[async_recursion(?Send)]
async fn dispose_cf_dictionary(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_cf_dictionary(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_native_hit_part(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_native_part_bounds(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_native_scroll_bar_offset_change(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_ptr_of_buffer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_native_jrsui(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn paint_changes_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn paint_changes_to_cg_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn paint_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn paint_to_cg_context(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn sync_changes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
