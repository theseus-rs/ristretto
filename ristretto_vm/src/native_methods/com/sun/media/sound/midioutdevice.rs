use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.media.sound.MidiOutDevice`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/media/sound/MidiOutDevice";
    registry.register(class_name, "nClose", "(J)V", n_close);
    registry.register(class_name, "nGetTimeStamp", "(J)J", n_get_time_stamp);
    registry.register(class_name, "nOpen", "(I)J", n_open);
    registry.register(
        class_name,
        "nSendLongMessage",
        "(J[BIJ)V",
        n_send_long_message,
    );
    registry.register(
        class_name,
        "nSendShortMessage",
        "(JIJ)V",
        n_send_short_message,
    );
}

#[async_recursion(?Send)]
async fn n_close(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn n_get_time_stamp(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn n_open(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn n_send_long_message(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn n_send_short_message(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
