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
    "sun/awt/windows/WColor.getDefaultColor(I)Ljava/awt/Color;",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn get_default_color<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WColor.getDefaultColor(I)Ljava/awt/Color;".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_default_color() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_default_color(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WColor.getDefaultColor(I)Ljava/awt/Color;",
            result.unwrap_err().to_string()
        );
    }
}
