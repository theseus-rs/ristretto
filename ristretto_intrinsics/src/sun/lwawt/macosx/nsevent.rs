use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/NSEvent.nsKeyModifiersToJavaKeyInfo([I[I)V", Any)]
#[async_method]
pub async fn ns_key_modifiers_to_java_key_info<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out = parameters.pop_reference()?;
    let _in_ = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.NSEvent.nsKeyModifiersToJavaKeyInfo([I[I)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/NSEvent.nsToJavaChar(CI)C", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn ns_to_java_char_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.NSEvent.nsToJavaChar(CI)C".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/lwawt/macosx/NSEvent.nsToJavaChar(CIZ)C", GreaterThan(JAVA_8))]
#[async_method]
pub async fn ns_to_java_char_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _space_key_typed = parameters.pop_bool()?;
    let _modifier_flags = parameters.pop_int()?;
    let _ns_char = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.NSEvent.nsToJavaChar(CIZ)C".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/lwawt/macosx/NSEvent.nsToJavaKeyInfo([I[I)Z", Any)]
#[async_method]
pub async fn ns_to_java_key_info<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out = parameters.pop_reference()?;
    let _in_ = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.NSEvent.nsToJavaKeyInfo([I[I)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/NSEvent.nsToJavaKeyModifiers(I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn ns_to_java_key_modifiers<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.NSEvent.nsToJavaKeyModifiers(I)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/NSEvent.nsToJavaModifiers(I)I", GreaterThan(JAVA_8))]
#[async_method]
pub async fn ns_to_java_modifiers<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _modifier_flags = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.NSEvent.nsToJavaModifiers(I)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/NSEvent.nsToJavaMouseModifiers(II)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn ns_to_java_mouse_modifiers<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.NSEvent.nsToJavaMouseModifiers(II)I".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ns_key_modifiers_to_java_key_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ns_key_modifiers_to_java_key_info(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.NSEvent.nsKeyModifiersToJavaKeyInfo([I[I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_ns_to_java_char_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            ns_to_java_char_0(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.NSEvent.nsToJavaChar(CI)C",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_ns_to_java_char_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ns_to_java_char_1(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.NSEvent.nsToJavaChar(CIZ)C",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_ns_to_java_key_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ns_to_java_key_info(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.NSEvent.nsToJavaKeyInfo([I[I)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_ns_to_java_key_modifiers() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = ns_to_java_key_modifiers(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.NSEvent.nsToJavaKeyModifiers(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_ns_to_java_modifiers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ns_to_java_modifiers(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.NSEvent.nsToJavaModifiers(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_ns_to_java_mouse_modifiers() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            ns_to_java_mouse_modifiers(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "sun.lwawt.macosx.NSEvent.nsToJavaMouseModifiers(II)I",
            result.unwrap_err().to_string()
        );
    }
}
