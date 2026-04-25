use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CAccessible.menuClosed(J)V", Any)]
#[async_method]
pub async fn menu_closed<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CAccessible.menuClosed(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/lwawt/macosx/CAccessible.menuItemSelected(J)V", Any)]
#[async_method]
pub async fn menu_item_selected<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CAccessible.menuItemSelected(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CAccessible.menuOpened(J)V", Any)]
#[async_method]
pub async fn menu_opened<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CAccessible.menuOpened(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/lwawt/macosx/CAccessible.selectedCellsChanged(J)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn selected_cells_changed<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CAccessible.selectedCellsChanged(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CAccessible.selectedTextChanged(J)V", Any)]
#[async_method]
pub async fn selected_text_changed<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CAccessible.selectedTextChanged(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CAccessible.selectionChanged(J)V", Any)]
#[async_method]
pub async fn selection_changed<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CAccessible.selectionChanged(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CAccessible.tableContentCacheClear(J)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn table_content_cache_clear<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CAccessible.tableContentCacheClear(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CAccessible.titleChanged(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn title_changed<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CAccessible.titleChanged(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CAccessible.treeNodeCollapsed(J)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn tree_node_collapsed<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CAccessible.treeNodeCollapsed(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CAccessible.treeNodeExpanded(J)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn tree_node_expanded<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CAccessible.treeNodeExpanded(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CAccessible.unregisterFromCocoaAXSystem(J)V", Any)]
#[async_method]
pub async fn unregister_from_cocoa_ax_system<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CAccessible.unregisterFromCocoaAXSystem(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CAccessible.valueChanged(J)V", Any)]
#[async_method]
pub async fn value_changed<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CAccessible.valueChanged(J)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_menu_closed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = menu_closed(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CAccessible.menuClosed(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_menu_item_selected() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = menu_item_selected(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CAccessible.menuItemSelected(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_menu_opened() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = menu_opened(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CAccessible.menuOpened(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_selected_cells_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = selected_cells_changed(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CAccessible.selectedCellsChanged(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_selected_text_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = selected_text_changed(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CAccessible.selectedTextChanged(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_selection_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = selection_changed(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CAccessible.selectionChanged(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_table_content_cache_clear() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = table_content_cache_clear(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CAccessible.tableContentCacheClear(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_title_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = title_changed(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CAccessible.titleChanged(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_tree_node_collapsed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = tree_node_collapsed(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CAccessible.treeNodeCollapsed(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_tree_node_expanded() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = tree_node_expanded(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CAccessible.treeNodeExpanded(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_unregister_from_cocoa_ax_system() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            unregister_from_cocoa_ax_system(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CAccessible.unregisterFromCocoaAXSystem(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_value_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value_changed(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CAccessible.valueChanged(J)V",
            result.unwrap_err().to_string()
        );
    }
}
