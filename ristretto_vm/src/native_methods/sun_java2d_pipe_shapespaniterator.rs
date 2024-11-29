use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.pipe.ShapeSpanIterator`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/pipe/ShapeSpanIterator";
    registry.register(class_name, "addSegment", "(I[F)V", add_segment);
    registry.register(class_name, "appendPoly", "([I[IIII)V", append_poly);
    registry.register(class_name, "closePath", "()V", close_path);
    registry.register(class_name, "curveTo", "(FFFFFF)V", curve_to);
    registry.register(class_name, "dispose", "()V", dispose);
    registry.register(class_name, "getNativeConsumer", "()J", get_native_consumer);
    registry.register(class_name, "getNativeIterator", "()J", get_native_iterator);
    registry.register(class_name, "getPathBox", "([I)V", get_path_box);
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "intersectClipBox",
        "(IIII)V",
        intersect_clip_box,
    );
    registry.register(class_name, "lineTo", "(FF)V", line_to);
    registry.register(class_name, "moveTo", "(FF)V", move_to);
    registry.register(class_name, "nextSpan", "([I)Z", next_span);
    registry.register(class_name, "pathDone", "()V", path_done);
    registry.register(class_name, "quadTo", "(FFFF)V", quad_to);
    registry.register(class_name, "setNormalize", "(Z)V", set_normalize);
    registry.register(
        class_name,
        "setOutputAreaXYXY",
        "(IIII)V",
        set_output_area_xyxy,
    );
    registry.register(class_name, "setRule", "(I)V", set_rule);
    registry.register(class_name, "skipDownTo", "(I)V", skip_down_to);
}

#[async_recursion(?Send)]
async fn add_segment(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn append_poly(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn close_path(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn curve_to(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn dispose(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_native_consumer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_native_iterator(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_path_box(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn intersect_clip_box(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn line_to(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn move_to(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn next_span(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn path_done(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn quad_to(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_normalize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_output_area_xyxy(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_rule(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn skip_down_to(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
