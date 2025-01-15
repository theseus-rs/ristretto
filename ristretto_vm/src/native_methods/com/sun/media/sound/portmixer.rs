use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/sun/media/sound/PortMixer";

/// Register all native methods for `com.sun.media.sound.PortMixer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "nClose", "(J)V", n_close);
    registry.register(
        CLASS_NAME,
        "nControlGetFloatValue",
        "(J)F",
        n_control_get_float_value,
    );
    registry.register(
        CLASS_NAME,
        "nControlGetIntValue",
        "(J)I",
        n_control_get_int_value,
    );
    registry.register(
        CLASS_NAME,
        "nControlSetFloatValue",
        "(JF)V",
        n_control_set_float_value,
    );
    registry.register(
        CLASS_NAME,
        "nControlSetIntValue",
        "(JI)V",
        n_control_set_int_value,
    );
    registry.register(
        CLASS_NAME,
        "nGetControls",
        "(JILjava/util/Vector;)V",
        n_get_controls,
    );
    registry.register(CLASS_NAME, "nGetPortCount", "(J)I", n_get_port_count);
    registry.register(
        CLASS_NAME,
        "nGetPortName",
        "(JI)Ljava/lang/String;",
        n_get_port_name,
    );
    registry.register(CLASS_NAME, "nGetPortType", "(JI)I", n_get_port_type);
    registry.register(CLASS_NAME, "nOpen", "(I)J", n_open);
}

#[async_recursion(?Send)]
async fn n_close(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nClose(J)V")
}

#[async_recursion(?Send)]
async fn n_control_get_float_value(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nControlGetFloatValue(J)F")
}

#[async_recursion(?Send)]
async fn n_control_get_int_value(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nControlGetIntValue(J)I")
}

#[async_recursion(?Send)]
async fn n_control_set_float_value(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nControlSetFloatValue(JF)V")
}

#[async_recursion(?Send)]
async fn n_control_set_int_value(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nControlSetIntValue(JI)V")
}

#[async_recursion(?Send)]
async fn n_get_controls(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nGetControls(JILjava/util/Vector;)V")
}

#[async_recursion(?Send)]
async fn n_get_port_count(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nGetPortCount(J)I")
}

#[async_recursion(?Send)]
async fn n_get_port_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nGetPortName(JI)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn n_get_port_type(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nGetPortType(JI)I")
}

#[async_recursion(?Send)]
async fn n_open(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nOpen(I)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.PortMixer.nClose(J)V")]
    async fn test_n_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_close(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nControlGetFloatValue(J)F"
    )]
    async fn test_n_control_get_float_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_control_get_float_value(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nControlGetIntValue(J)I"
    )]
    async fn test_n_control_get_int_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_control_get_int_value(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nControlSetFloatValue(JF)V"
    )]
    async fn test_n_control_set_float_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_control_set_float_value(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nControlSetIntValue(JI)V"
    )]
    async fn test_n_control_set_int_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_control_set_int_value(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nGetControls(JILjava/util/Vector;)V"
    )]
    async fn test_n_get_controls() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_controls(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nGetPortCount(J)I"
    )]
    async fn test_n_get_port_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_port_count(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nGetPortName(JI)Ljava/lang/String;"
    )]
    async fn test_n_get_port_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_port_name(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nGetPortType(JI)I"
    )]
    async fn test_n_get_port_type() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_port_type(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.PortMixer.nOpen(I)J")]
    async fn test_n_open() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_open(thread, Arguments::default()).await;
    }
}
