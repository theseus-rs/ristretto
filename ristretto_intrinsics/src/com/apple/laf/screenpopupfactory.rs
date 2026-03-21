use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/apple/laf/ScreenPopupFactory._getHeavyWeightPopup(Ljava/awt/Component;Ljava/awt/Component;II)Ljavax/swing/Popup;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_heavy_weight_popup<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("com.apple.laf.ScreenPopupFactory._getHeavyWeightPopup(Ljava/awt/Component;Ljava/awt/Component;II)Ljavax/swing/Popup;".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_heavy_weight_popup() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_heavy_weight_popup(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
