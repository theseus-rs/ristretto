use crate::native_methods::registry::{MethodRegistry, JAVA_11, JAVA_17};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CAccessible";

/// Register all native methods for `sun.lwawt.macosx.CAccessible`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_11 {
        registry.register(CLASS_NAME, "titleChanged", "(J)V", title_changed);
    }
    if registry.java_major_version() >= JAVA_17 {
        registry.register(
            CLASS_NAME,
            "selectedCellsChanged",
            "(J)V",
            selected_cells_changed,
        );
        registry.register(
            CLASS_NAME,
            "tableContentCacheClear",
            "(J)V",
            table_content_cache_clear,
        );
        registry.register(CLASS_NAME, "titleChanged", "(J)V", title_changed);
        registry.register(CLASS_NAME, "treeNodeCollapsed", "(J)V", tree_node_collapsed);
        registry.register(CLASS_NAME, "treeNodeExpanded", "(J)V", tree_node_expanded);
    }

    registry.register(CLASS_NAME, "menuClosed", "(J)V", menu_closed);
    registry.register(CLASS_NAME, "menuItemSelected", "(J)V", menu_item_selected);
    registry.register(CLASS_NAME, "menuOpened", "(J)V", menu_opened);
    registry.register(
        CLASS_NAME,
        "selectedTextChanged",
        "(J)V",
        selected_text_changed,
    );
    registry.register(CLASS_NAME, "selectionChanged", "(J)V", selection_changed);
    registry.register(
        CLASS_NAME,
        "unregisterFromCocoaAXSystem",
        "(J)V",
        unregister_from_cocoa_ax_system,
    );
    registry.register(CLASS_NAME, "valueChanged", "(J)V", value_changed);
}

#[async_recursion(?Send)]
async fn menu_closed(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.menuClosed(J)V")
}

#[async_recursion(?Send)]
async fn menu_item_selected(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.menuItemSelected(J)V")
}

#[async_recursion(?Send)]
async fn menu_opened(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.menuOpened(J)V")
}

#[async_recursion(?Send)]
async fn selected_cells_changed(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.selectedCellsChanged(J)V")
}

#[async_recursion(?Send)]
async fn selected_text_changed(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.selectedTextChanged(J)V")
}

#[async_recursion(?Send)]
async fn selection_changed(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.selectionChanged(J)V")
}

#[async_recursion(?Send)]
async fn table_content_cache_clear(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.tableContentCacheClear(J)V")
}

#[async_recursion(?Send)]
async fn title_changed(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.titleChanged(J)V")
}

#[async_recursion(?Send)]
async fn tree_node_collapsed(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.treeNodeCollapsed(J)V")
}

#[async_recursion(?Send)]
async fn tree_node_expanded(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.treeNodeExpanded(J)V")
}

#[async_recursion(?Send)]
async fn unregister_from_cocoa_ax_system(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.unregisterFromCocoaAXSystem(J)V")
}

#[async_recursion(?Send)]
async fn value_changed(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
