use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.apple.laf.ScreenPopupFactory`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/apple/laf/ScreenPopupFactory";
    registry.register(
        class_name,
        "_getHeavyWeightPopup",
        "(Ljava/awt/Component;Ljava/awt/Component;II)Ljavax/swing/Popup;",
        get_heavy_weight_popup,
    );
}

#[async_recursion(?Send)]
async fn get_heavy_weight_popup(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.ScreenPopupFactory._getHeavyWeightPopup(Ljava/awt/Component;Ljava/awt/Component;II)Ljavax/swing/Popup;")
}
