use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.media.sound.MidiOutDeviceProvider`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/media/sound/MidiOutDeviceProvider";
    registry.register(
        class_name,
        "nGetDescription",
        "(I)Ljava/lang/String;",
        n_get_description,
    );
    registry.register(class_name, "nGetName", "(I)Ljava/lang/String;", n_get_name);
    registry.register(class_name, "nGetNumDevices", "()I", n_get_num_devices);
    registry.register(
        class_name,
        "nGetVendor",
        "(I)Ljava/lang/String;",
        n_get_vendor,
    );
    registry.register(
        class_name,
        "nGetVersion",
        "(I)Ljava/lang/String;",
        n_get_version,
    );
}

#[async_recursion(?Send)]
async fn n_get_description(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiOutDeviceProvider.nGetDescription(I)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn n_get_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiOutDeviceProvider.nGetName(I)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn n_get_num_devices(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiOutDeviceProvider.nGetNumDevices()I")
}

#[async_recursion(?Send)]
async fn n_get_vendor(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiOutDeviceProvider.nGetVendor(I)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn n_get_version(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.MidiOutDeviceProvider.nGetVersion(I)Ljava/lang/String;")
}
