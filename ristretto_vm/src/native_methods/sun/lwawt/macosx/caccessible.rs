use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `sun.lwawt.macosx.CAccessible`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CAccessible";
    let java_version = registry.java_version().clone();

    if java_version >= JAVA_11 {
        registry.register(class_name, "titleChanged", "(J)V", title_changed);
    }
    if java_version >= JAVA_17 {
        registry.register(
            class_name,
            "selectedCellsChanged",
            "(J)V",
            selected_cells_changed,
        );
        registry.register(
            class_name,
            "tableContentCacheClear",
            "(J)V",
            table_content_cache_clear,
        );
        registry.register(class_name, "titleChanged", "(J)V", title_changed);
        registry.register(class_name, "treeNodeCollapsed", "(J)V", tree_node_collapsed);
        registry.register(class_name, "treeNodeExpanded", "(J)V", tree_node_expanded);
    }

    registry.register(class_name, "menuClosed", "(J)V", menu_closed);
    registry.register(class_name, "menuItemSelected", "(J)V", menu_item_selected);
    registry.register(class_name, "menuOpened", "(J)V", menu_opened);
    registry.register(
        class_name,
        "selectedTextChanged",
        "(J)V",
        selected_text_changed,
    );
    registry.register(class_name, "selectionChanged", "(J)V", selection_changed);
    registry.register(
        class_name,
        "unregisterFromCocoaAXSystem",
        "(J)V",
        unregister_from_cocoa_ax_system,
    );
    registry.register(class_name, "valueChanged", "(J)V", value_changed);
}

#[async_recursion(?Send)]
async fn menu_closed(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.menuClosed(J)V")
}

#[async_recursion(?Send)]
async fn menu_item_selected(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.menuItemSelected(J)V")
}

#[async_recursion(?Send)]
async fn menu_opened(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.menuOpened(J)V")
}

#[async_recursion(?Send)]
async fn selected_cells_changed(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.selectedCellsChanged(J)V")
}

#[async_recursion(?Send)]
async fn selected_text_changed(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.selectedTextChanged(J)V")
}

#[async_recursion(?Send)]
async fn selection_changed(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.selectionChanged(J)V")
}

#[async_recursion(?Send)]
async fn table_content_cache_clear(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.tableContentCacheClear(J)V")
}

#[async_recursion(?Send)]
async fn title_changed(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.titleChanged(J)V")
}

#[async_recursion(?Send)]
async fn tree_node_collapsed(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.treeNodeCollapsed(J)V")
}

#[async_recursion(?Send)]
async fn tree_node_expanded(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.treeNodeExpanded(J)V")
}

#[async_recursion(?Send)]
async fn unregister_from_cocoa_ax_system(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.unregisterFromCocoaAXSystem(J)V")
}

#[async_recursion(?Send)]
async fn value_changed(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessible.valueChanged(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java17 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CAccessible";
        assert!(registry.method(class_name, "menuClosed", "(J)V").is_some());
        assert!(registry
            .method(class_name, "menuItemSelected", "(J)V")
            .is_some());
        assert!(registry.method(class_name, "menuOpened", "(J)V").is_some());
        assert!(registry
            .method(class_name, "selectedCellsChanged", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "selectedTextChanged", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "selectionChanged", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "tableContentCacheClear", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "titleChanged", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "treeNodeCollapsed", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "treeNodeExpanded", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "unregisterFromCocoaAXSystem", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "valueChanged", "(J)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CAccessible.menuClosed(J)V")]
    async fn test_menu_closed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = menu_closed(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CAccessible.menuItemSelected(J)V")]
    async fn test_menu_item_selected() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = menu_item_selected(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CAccessible.menuOpened(J)V")]
    async fn test_menu_opened() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = menu_opened(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CAccessible.selectedCellsChanged(J)V")]
    async fn test_selected_cells_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = selected_cells_changed(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CAccessible.selectedTextChanged(J)V")]
    async fn test_selected_text_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = selected_text_changed(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CAccessible.selectionChanged(J)V")]
    async fn test_selection_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = selection_changed(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CAccessible.tableContentCacheClear(J)V")]
    async fn test_table_content_cache_clear() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = table_content_cache_clear(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CAccessible.titleChanged(J)V")]
    async fn test_title_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = title_changed(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CAccessible.treeNodeCollapsed(J)V")]
    async fn test_tree_node_collapsed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = tree_node_collapsed(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CAccessible.treeNodeExpanded(J)V")]
    async fn test_tree_node_expanded() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = tree_node_expanded(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CAccessible.unregisterFromCocoaAXSystem(J)V")]
    async fn test_unregister_from_cocoa_ax_system() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unregister_from_cocoa_ax_system(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CAccessible.valueChanged(J)V")]
    async fn test_value_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = value_changed(thread, Arguments::default()).await;
    }
}
