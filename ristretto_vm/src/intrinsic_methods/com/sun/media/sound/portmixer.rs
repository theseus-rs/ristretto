use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("com/sun/media/sound/PortMixer.nClose(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn n_close(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nClose(J)V")
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nControlGetFloatValue(J)F", Any)]
#[async_recursion(?Send)]
pub(crate) async fn n_control_get_float_value(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nControlGetFloatValue(J)F")
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nControlGetIntValue(J)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn n_control_get_int_value(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nControlGetIntValue(J)I")
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nControlSetFloatValue(JF)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn n_control_set_float_value(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nControlSetFloatValue(JF)V")
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nControlSetIntValue(JI)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn n_control_set_int_value(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nControlSetIntValue(JI)V")
}

#[intrinsic_method(
    "com/sun/media/sound/PortMixer.nGetControls(JILjava/util/Vector;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn n_get_controls(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nGetControls(JILjava/util/Vector;)V")
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nGetPortCount(J)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn n_get_port_count(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nGetPortCount(J)I")
}

#[intrinsic_method(
    "com/sun/media/sound/PortMixer.nGetPortName(JI)Ljava/lang/String;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn n_get_port_name(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nGetPortName(JI)Ljava/lang/String;")
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nGetPortType(JI)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn n_get_port_type(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nGetPortType(JI)I")
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nOpen(I)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn n_open(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.PortMixer.nOpen(I)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.PortMixer.nClose(J)V")]
    async fn test_n_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_close(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nControlGetFloatValue(J)F"
    )]
    async fn test_n_control_get_float_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_control_get_float_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nControlGetIntValue(J)I"
    )]
    async fn test_n_control_get_int_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_control_get_int_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nControlSetFloatValue(JF)V"
    )]
    async fn test_n_control_set_float_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_control_set_float_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nControlSetIntValue(JI)V"
    )]
    async fn test_n_control_set_int_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_control_set_int_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nGetControls(JILjava/util/Vector;)V"
    )]
    async fn test_n_get_controls() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_controls(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nGetPortCount(J)I"
    )]
    async fn test_n_get_port_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_port_count(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nGetPortName(JI)Ljava/lang/String;"
    )]
    async fn test_n_get_port_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_port_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.PortMixer.nGetPortType(JI)I"
    )]
    async fn test_n_get_port_type() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_port_type(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.PortMixer.nOpen(I)J")]
    async fn test_n_open() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_open(thread, Parameters::default()).await;
    }
}
