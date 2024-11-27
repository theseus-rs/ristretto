use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.media.sound.PortMixer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/media/sound/PortMixer";
    registry.register(class_name, "nClose", "(J)V", n_close);
    registry.register(
        class_name,
        "nControlGetFloatValue",
        "(J)F",
        n_control_get_float_value,
    );
    registry.register(
        class_name,
        "nControlGetIntValue",
        "(J)I",
        n_control_get_int_value,
    );
    registry.register(
        class_name,
        "nControlSetFloatValue",
        "(JF)V",
        n_control_set_float_value,
    );
    registry.register(
        class_name,
        "nControlSetIntValue",
        "(JI)V",
        n_control_set_int_value,
    );
    registry.register(
        class_name,
        "nGetControls",
        "(JILjava/util/Vector;)V",
        n_get_controls,
    );
    registry.register(class_name, "nGetPortCount", "(J)I", n_get_port_count);
    registry.register(
        class_name,
        "nGetPortName",
        "(JI)Ljava/lang/String;",
        n_get_port_name,
    );
    registry.register(class_name, "nGetPortType", "(JI)I", n_get_port_type);
    registry.register(class_name, "nOpen", "(I)J", n_open);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_close(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_control_get_float_value(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_control_get_int_value(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_control_set_float_value(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_control_set_int_value(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_get_controls(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_get_port_count(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_get_port_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_get_port_type(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn n_open(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
