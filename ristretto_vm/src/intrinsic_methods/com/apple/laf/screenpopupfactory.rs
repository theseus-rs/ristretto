use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/apple/laf/ScreenPopupFactory";

/// Register all intrinsic methods for `com.apple.laf.ScreenPopupFactory`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "_getHeavyWeightPopup",
        "(Ljava/awt/Component;Ljava/awt/Component;II)Ljavax/swing/Popup;",
        get_heavy_weight_popup,
    );
}

#[async_recursion(?Send)]
async fn get_heavy_weight_popup(
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
