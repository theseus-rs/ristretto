use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("apple/laf/JRSUIFocus.beginNativeFocus(JI)I", Any)]
#[async_method]
pub async fn begin_native_focus<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ring_style = parameters.pop_int()?;
    let _cg_context = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("apple.laf.JRSUIFocus.beginNativeFocus(JI)I".to_string())
            .into(),
    )
}

#[intrinsic_method("apple/laf/JRSUIFocus.endNativeFocus(J)I", Any)]
#[async_method]
pub async fn end_native_focus<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cg_context = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("apple.laf.JRSUIFocus.endNativeFocus(J)I".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_begin_native_focus() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            begin_native_focus(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "apple.laf.JRSUIFocus.beginNativeFocus(JI)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_end_native_focus() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = end_native_focus(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "apple.laf.JRSUIFocus.endNativeFocus(J)I",
            result.unwrap_err().to_string()
        );
    }
}
