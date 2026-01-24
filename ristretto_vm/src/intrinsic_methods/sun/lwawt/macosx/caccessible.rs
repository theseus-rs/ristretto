use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CAccessible.menuClosed(J)V", Any)]
#[async_method]
pub(crate) async fn menu_closed(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.menuClosed(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CAccessible.menuItemSelected(J)V", Any)]
#[async_method]
pub(crate) async fn menu_item_selected(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.menuItemSelected(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CAccessible.menuOpened(J)V", Any)]
#[async_method]
pub(crate) async fn menu_opened(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.menuOpened(J)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CAccessible.selectedCellsChanged(J)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn selected_cells_changed(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.selectedCellsChanged(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CAccessible.selectedTextChanged(J)V", Any)]
#[async_method]
pub(crate) async fn selected_text_changed(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.selectedTextChanged(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CAccessible.selectionChanged(J)V", Any)]
#[async_method]
pub(crate) async fn selection_changed(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.selectionChanged(J)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CAccessible.tableContentCacheClear(J)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn table_content_cache_clear(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.tableContentCacheClear(J)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CAccessible.titleChanged(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn title_changed(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.titleChanged(J)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CAccessible.treeNodeCollapsed(J)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn tree_node_collapsed(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.treeNodeCollapsed(J)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CAccessible.treeNodeExpanded(J)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn tree_node_expanded(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.treeNodeExpanded(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CAccessible.unregisterFromCocoaAXSystem(J)V", Any)]
#[async_method]
pub(crate) async fn unregister_from_cocoa_ax_system(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.unregisterFromCocoaAXSystem(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CAccessible.valueChanged(J)V", Any)]
#[async_method]
pub(crate) async fn value_changed(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.valueChanged(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CAccessible.menuClosed(J)V")]
    async fn test_menu_closed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = menu_closed(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CAccessible.menuItemSelected(J)V"
    )]
    async fn test_menu_item_selected() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = menu_item_selected(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CAccessible.menuOpened(J)V")]
    async fn test_menu_opened() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = menu_opened(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CAccessible.selectedCellsChanged(J)V"
    )]
    async fn test_selected_cells_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = selected_cells_changed(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CAccessible.selectedTextChanged(J)V"
    )]
    async fn test_selected_text_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = selected_text_changed(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CAccessible.selectionChanged(J)V"
    )]
    async fn test_selection_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = selection_changed(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CAccessible.tableContentCacheClear(J)V"
    )]
    async fn test_table_content_cache_clear() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = table_content_cache_clear(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CAccessible.titleChanged(J)V")]
    async fn test_title_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = title_changed(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CAccessible.treeNodeCollapsed(J)V"
    )]
    async fn test_tree_node_collapsed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = tree_node_collapsed(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CAccessible.treeNodeExpanded(J)V"
    )]
    async fn test_tree_node_expanded() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = tree_node_expanded(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CAccessible.unregisterFromCocoaAXSystem(J)V"
    )]
    async fn test_unregister_from_cocoa_ax_system() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unregister_from_cocoa_ax_system(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CAccessible.valueChanged(J)V")]
    async fn test_value_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = value_changed(thread, Parameters::default()).await;
    }
}
