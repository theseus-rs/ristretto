use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/sun/media/sound/PortMixer.nClose(J)V", Any)]
#[async_method]
pub async fn n_close<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("com.sun.media.sound.PortMixer.nClose(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nControlGetFloatValue(J)F", Any)]
#[async_method]
pub async fn n_control_get_float_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _control_id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.PortMixer.nControlGetFloatValue(J)F".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nControlGetIntValue(J)I", Any)]
#[async_method]
pub async fn n_control_get_int_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _control_id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.PortMixer.nControlGetIntValue(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nControlSetFloatValue(JF)V", Any)]
#[async_method]
pub async fn n_control_set_float_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_float()?;
    let _control_id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.PortMixer.nControlSetFloatValue(JF)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nControlSetIntValue(JI)V", Any)]
#[async_method]
pub async fn n_control_set_int_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_int()?;
    let _control_id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.PortMixer.nControlSetIntValue(JI)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/media/sound/PortMixer.nGetControls(JILjava/util/Vector;)V",
    Any
)]
#[async_method]
pub async fn n_get_controls<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _vector = parameters.pop_reference()?;
    let _port_index = parameters.pop_int()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.PortMixer.nGetControls(JILjava/util/Vector;)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nGetPortCount(J)I", Any)]
#[async_method]
pub async fn n_get_port_count<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.PortMixer.nGetPortCount(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/media/sound/PortMixer.nGetPortName(JI)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn n_get_port_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port_index = parameters.pop_int()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.PortMixer.nGetPortName(JI)Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nGetPortType(JI)I", Any)]
#[async_method]
pub async fn n_get_port_type<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port_index = parameters.pop_int()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.PortMixer.nGetPortType(JI)I".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/PortMixer.nOpen(I)J", Any)]
#[async_method]
pub async fn n_open<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _mixer_index = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("com.sun.media.sound.PortMixer.nOpen(I)J".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_n_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_close(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.media.sound.PortMixer.nClose(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_control_get_float_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_control_get_float_value(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.media.sound.PortMixer.nControlGetFloatValue(J)F",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_control_get_int_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_control_get_int_value(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.media.sound.PortMixer.nControlGetIntValue(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_control_set_float_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_control_set_float_value(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Float(0.0)]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.PortMixer.nControlSetFloatValue(JF)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_control_set_int_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            n_control_set_int_value(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "com.sun.media.sound.PortMixer.nControlSetIntValue(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_get_controls() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_controls(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com.sun.media.sound.PortMixer.nGetControls(JILjava/util/Vector;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_get_port_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_port_count(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.sun.media.sound.PortMixer.nGetPortCount(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_get_port_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            n_get_port_name(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "com.sun.media.sound.PortMixer.nGetPortName(JI)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_get_port_type() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            n_get_port_type(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "com.sun.media.sound.PortMixer.nGetPortType(JI)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_open() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_open(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com.sun.media.sound.PortMixer.nOpen(I)J",
            result.unwrap_err().to_string()
        );
    }
}
