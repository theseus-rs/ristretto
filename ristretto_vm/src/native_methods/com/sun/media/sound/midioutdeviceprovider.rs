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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/sun/media/sound/MidiOutDeviceProvider";
        assert!(registry
            .method(class_name, "nGetDescription", "(I)Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(class_name, "nGetName", "(I)Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(class_name, "nGetNumDevices", "()I")
            .is_some());
        assert!(registry
            .method(class_name, "nGetVendor", "(I)Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(class_name, "nGetVersion", "(I)Ljava/lang/String;")
            .is_some());
    }

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
