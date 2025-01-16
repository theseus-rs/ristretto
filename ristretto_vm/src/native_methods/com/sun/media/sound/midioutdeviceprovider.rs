use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/sun/media/sound/MidiOutDeviceProvider";

/// Register all native methods for `com.sun.media.sound.MidiOutDeviceProvider`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nGetDescription",
        "(I)Ljava/lang/String;",
        n_get_description,
    );
    registry.register(CLASS_NAME, "nGetName", "(I)Ljava/lang/String;", n_get_name);
    registry.register(CLASS_NAME, "nGetNumDevices", "()I", n_get_num_devices);
    registry.register(
        CLASS_NAME,
        "nGetVendor",
        "(I)Ljava/lang/String;",
        n_get_vendor,
    );
    registry.register(
        CLASS_NAME,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiOutDeviceProvider.nGetDescription(I)Ljava/lang/String;"
    )]
    async fn test_n_get_description() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_description(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiOutDeviceProvider.nGetName(I)Ljava/lang/String;"
    )]
    async fn test_n_get_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_name(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiOutDeviceProvider.nGetNumDevices()I"
    )]
    async fn test_n_get_num_devices() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_num_devices(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiOutDeviceProvider.nGetVendor(I)Ljava/lang/String;"
    )]
    async fn test_n_get_vendor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_vendor(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.MidiOutDeviceProvider.nGetVersion(I)Ljava/lang/String;"
    )]
    async fn test_n_get_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_version(thread, Arguments::default()).await;
    }
}
