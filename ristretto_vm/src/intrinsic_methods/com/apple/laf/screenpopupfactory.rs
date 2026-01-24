use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "com/apple/laf/ScreenPopupFactory._getHeavyWeightPopup(Ljava/awt/Component;Ljava/awt/Component;II)Ljavax/swing/Popup;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn get_heavy_weight_popup(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.apple.laf.ScreenPopupFactory._getHeavyWeightPopup(Ljava/awt/Component;Ljava/awt/Component;II)Ljavax/swing/Popup;"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.laf.ScreenPopupFactory._getHeavyWeightPopup(Ljava/awt/Component;Ljava/awt/Component;II)Ljavax/swing/Popup;"
    )]
    async fn test_get_heavy_weight_popup() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_heavy_weight_popup(thread, Parameters::default()).await;
    }
}
