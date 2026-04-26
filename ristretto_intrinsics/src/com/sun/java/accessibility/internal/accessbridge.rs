use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.caretUpdate(Ljavax/swing/event/CaretEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn caret_update<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.caretUpdate(Ljavax/swing/event/CaretEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.focusGained(Ljava/awt/event/FocusEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn focus_gained<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.focusGained(Ljava/awt/event/FocusEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.focusLost(Ljava/awt/event/FocusEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn focus_lost<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.focusLost(Ljava/awt/event/FocusEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.isSysWow()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn is_sys_wow<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/accessibility/internal/AccessBridge.isSysWow()Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.javaShutdown()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn java_shutdown<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/accessibility/internal/AccessBridge.javaShutdown()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.jawtGetComponentFromNativeWindowHandle(I)Ljava/awt/Component;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn jawt_get_component_from_native_window_handle<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_handle = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.jawtGetComponentFromNativeWindowHandle(I)Ljava/awt/Component;".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.jawtGetNativeWindowHandleFromComponent(Ljava/awt/Component;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn jawt_get_native_window_handle_from_component<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _component = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.jawtGetNativeWindowHandleFromComponent(Ljava/awt/Component;)I".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.menuCanceled(Ljavax/swing/event/MenuEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn menu_canceled<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.menuCanceled(Ljavax/swing/event/MenuEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.menuDeselected(Ljavax/swing/event/MenuEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn menu_deselected<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.menuDeselected(Ljavax/swing/event/MenuEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.menuSelected(Ljavax/swing/event/MenuEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn menu_selected<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.menuSelected(Ljavax/swing/event/MenuEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.mouseClicked(Ljava/awt/event/MouseEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn mouse_clicked<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.mouseClicked(Ljava/awt/event/MouseEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.mouseEntered(Ljava/awt/event/MouseEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn mouse_entered<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.mouseEntered(Ljava/awt/event/MouseEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.mouseExited(Ljava/awt/event/MouseEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn mouse_exited<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.mouseExited(Ljava/awt/event/MouseEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.mousePressed(Ljava/awt/event/MouseEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn mouse_pressed<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.mousePressed(Ljava/awt/event/MouseEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.mouseReleased(Ljava/awt/event/MouseEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn mouse_released<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.mouseReleased(Ljava/awt/event/MouseEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.popupMenuCanceled(Ljavax/swing/event/PopupMenuEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn popup_menu_canceled<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.popupMenuCanceled(Ljavax/swing/event/PopupMenuEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.popupMenuWillBecomeInvisible(Ljavax/swing/event/PopupMenuEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn popup_menu_will_become_invisible<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.popupMenuWillBecomeInvisible(Ljavax/swing/event/PopupMenuEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.popupMenuWillBecomeVisible(Ljavax/swing/event/PopupMenuEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn popup_menu_will_become_visible<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.popupMenuWillBecomeVisible(Ljavax/swing/event/PopupMenuEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.propertyActiveDescendentChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljavax/accessibility/AccessibleContext;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn property_active_descendent_change<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _new_value = parameters.pop_reference()?;
    let _old_value = parameters.pop_reference()?;
    let _source = parameters.pop_reference()?;
    let _event = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.propertyActiveDescendentChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljavax/accessibility/AccessibleContext;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.propertyCaretChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn property_caret_change<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _new_value = parameters.pop_int()?;
    let _old_value = parameters.pop_int()?;
    let _source = parameters.pop_reference()?;
    let _event = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.propertyCaretChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;II)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.propertyChildChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljavax/accessibility/AccessibleContext;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn property_child_change<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _new_value = parameters.pop_reference()?;
    let _old_value = parameters.pop_reference()?;
    let _source = parameters.pop_reference()?;
    let _event = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.propertyChildChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljavax/accessibility/AccessibleContext;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.propertyDescriptionChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljava/lang/String;Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn property_description_change<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _new_value = parameters.pop_reference()?;
    let _old_value = parameters.pop_reference()?;
    let _source = parameters.pop_reference()?;
    let _event = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.propertyDescriptionChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljava/lang/String;Ljava/lang/String;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.propertyNameChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljava/lang/String;Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn property_name_change<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _new_value = parameters.pop_reference()?;
    let _old_value = parameters.pop_reference()?;
    let _source = parameters.pop_reference()?;
    let _event = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.propertyNameChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljava/lang/String;Ljava/lang/String;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.propertySelectionChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn property_selection_change<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _source = parameters.pop_reference()?;
    let _event = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.propertySelectionChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.propertyStateChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljava/lang/String;Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn property_state_change<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _new_value = parameters.pop_reference()?;
    let _old_value = parameters.pop_reference()?;
    let _source = parameters.pop_reference()?;
    let _event = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.propertyStateChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljava/lang/String;Ljava/lang/String;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.propertyTextChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn property_text_change<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _source = parameters.pop_reference()?;
    let _event = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.propertyTextChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.propertyValueChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljava/lang/String;Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn property_value_change<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _new_value = parameters.pop_reference()?;
    let _old_value = parameters.pop_reference()?;
    let _source = parameters.pop_reference()?;
    let _event = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.propertyValueChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljava/lang/String;Ljava/lang/String;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.propertyVisibleDataChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn property_visible_data_change<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _source = parameters.pop_reference()?;
    let _event = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/accessibility/internal/AccessBridge.propertyVisibleDataChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;)V".to_string()).into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.runDLL()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn run_dll<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/accessibility/internal/AccessBridge.runDLL()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "com/sun/java/accessibility/internal/AccessBridge.sendDebugString(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn send_debug_string<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _debug_str = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/accessibility/internal/AccessBridge.sendDebugString(Ljava/lang/String;)V"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_caret_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = caret_update(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.caretUpdate(Ljavax/swing/event/CaretEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_focus_gained() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = focus_gained(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.focusGained(Ljava/awt/event/FocusEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_focus_lost() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = focus_lost(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.focusLost(Ljava/awt/event/FocusEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_sys_wow() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_sys_wow(thread, Parameters::default()).await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.isSysWow()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_java_shutdown() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = java_shutdown(thread, Parameters::default()).await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.javaShutdown()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_jawt_get_component_from_native_window_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = jawt_get_component_from_native_window_handle(
            thread,
            Parameters::new(vec![Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.jawtGetComponentFromNativeWindowHandle(I)Ljava/awt/Component;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_jawt_get_native_window_handle_from_component() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = jawt_get_native_window_handle_from_component(
            thread,
            Parameters::new(vec![Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.jawtGetNativeWindowHandleFromComponent(Ljava/awt/Component;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_menu_canceled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = menu_canceled(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.menuCanceled(Ljavax/swing/event/MenuEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_menu_deselected() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = menu_deselected(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.menuDeselected(Ljavax/swing/event/MenuEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_menu_selected() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = menu_selected(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.menuSelected(Ljavax/swing/event/MenuEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_mouse_clicked() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_clicked(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.mouseClicked(Ljava/awt/event/MouseEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_mouse_entered() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_entered(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.mouseEntered(Ljava/awt/event/MouseEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_mouse_exited() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_exited(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.mouseExited(Ljava/awt/event/MouseEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_mouse_pressed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_pressed(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.mousePressed(Ljava/awt/event/MouseEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_mouse_released() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_released(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.mouseReleased(Ljava/awt/event/MouseEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_popup_menu_canceled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = popup_menu_canceled(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.popupMenuCanceled(Ljavax/swing/event/PopupMenuEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_popup_menu_will_become_invisible() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = popup_menu_will_become_invisible(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.popupMenuWillBecomeInvisible(Ljavax/swing/event/PopupMenuEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_popup_menu_will_become_visible() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = popup_menu_will_become_visible(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.popupMenuWillBecomeVisible(Ljavax/swing/event/PopupMenuEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_property_active_descendent_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = property_active_descendent_change(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.propertyActiveDescendentChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljavax/accessibility/AccessibleContext;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_property_caret_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = property_caret_change(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.propertyCaretChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_property_child_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = property_child_change(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.propertyChildChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljavax/accessibility/AccessibleContext;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_property_description_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = property_description_change(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.propertyDescriptionChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljava/lang/String;Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_property_name_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = property_name_change(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.propertyNameChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljava/lang/String;Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_property_selection_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = property_selection_change(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.propertySelectionChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_property_state_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = property_state_change(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.propertyStateChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljava/lang/String;Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_property_text_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = property_text_change(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.propertyTextChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_property_value_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = property_value_change(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.propertyValueChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;Ljava/lang/String;Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_property_visible_data_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = property_visible_data_change(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.propertyVisibleDataChange(Ljava/beans/PropertyChangeEvent;Ljavax/accessibility/AccessibleContext;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_run_dll() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = run_dll(thread, Parameters::default()).await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.runDLL()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_send_debug_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = send_debug_string(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "com/sun/java/accessibility/internal/AccessBridge.sendDebugString(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }
}
