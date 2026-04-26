use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WFontMetrics.bytesWidth([BII)I", Any)]
#[async_method]
pub async fn bytes_width<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _len = parameters.pop_int()?;
    let _off = parameters.pop_int()?;
    let _str = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WFontMetrics.bytesWidth([BII)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WFontMetrics.charsWidth([CII)I", Any)]
#[async_method]
pub async fn chars_width<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _len = parameters.pop_int()?;
    let _off = parameters.pop_int()?;
    let _str = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WFontMetrics.charsWidth([CII)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WFontMetrics.init()V", Any)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WFontMetrics.init()V".to_string()).into())
}
#[intrinsic_method("sun/awt/windows/WFontMetrics.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WFontMetrics.initIDs()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WFontMetrics.stringWidth(Ljava/lang/String;)I", Any)]
#[async_method]
pub async fn string_width<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _str = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WFontMetrics.stringWidth(Ljava/lang/String;)I".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bytes_width() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = bytes_width(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WFontMetrics.bytesWidth([BII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_chars_width() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = chars_width(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WFontMetrics.charsWidth([CII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WFontMetrics.init()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WFontMetrics.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_string_width() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = string_width(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WFontMetrics.stringWidth(Ljava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }
}
