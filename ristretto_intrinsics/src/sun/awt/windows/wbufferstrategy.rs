use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WBufferStrategy.getDrawBuffer(Ljava/awt/Component;)Ljava/awt/Image;",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn get_draw_buffer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WBufferStrategy.getDrawBuffer(Ljava/awt/Component;)Ljava/awt/Image;"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WBufferStrategy.initIDs(Ljava/lang/Class;)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WBufferStrategy.initIDs(Ljava/lang/Class;)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_draw_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_draw_buffer(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WBufferStrategy.getDrawBuffer(Ljava/awt/Component;)Ljava/awt/Image;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WBufferStrategy.initIDs(Ljava/lang/Class;)V",
            result.unwrap_err().to_string()
        );
    }
}
