use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.media.sound.DirectAudioDevice`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/media/sound/DirectAudioDevice";
    registry.register(class_name, "nAvailable", "(JZ)I", n_available);
    registry.register(class_name, "nClose", "(JZ)V", n_close);
    registry.register(class_name, "nFlush", "(JZ)V", n_flush);
    registry.register(class_name, "nGetBufferSize", "(JZ)I", n_get_buffer_size);
    registry.register(
        class_name,
        "nGetBytePosition",
        "(JZJ)J",
        n_get_byte_position,
    );
    registry.register(
        class_name,
        "nGetFormats",
        "(IIZLjava/util/Vector;)V",
        n_get_formats,
    );
    registry.register(class_name, "nIsStillDraining", "(JZ)Z", n_is_still_draining);
    registry.register(class_name, "nOpen", "(IIZIFIIIZZI)J", n_open);
    registry.register(class_name, "nRead", "(J[BIII)I", n_read);
    registry.register(
        class_name,
        "nRequiresServicing",
        "(JZ)Z",
        n_requires_servicing,
    );
    registry.register(class_name, "nService", "(JZ)V", n_service);
    registry.register(
        class_name,
        "nSetBytePosition",
        "(JZJ)V",
        n_set_byte_position,
    );
    registry.register(class_name, "nStart", "(JZ)V", n_start);
    registry.register(class_name, "nStop", "(JZ)V", n_stop);
    registry.register(class_name, "nWrite", "(J[BIIIFF)I", n_write);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_available(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_close(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_flush(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_get_buffer_size(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_get_byte_position(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_get_formats(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_is_still_draining(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_open(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_read(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_requires_servicing(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_service(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_set_byte_position(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_start(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_stop(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_write(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
