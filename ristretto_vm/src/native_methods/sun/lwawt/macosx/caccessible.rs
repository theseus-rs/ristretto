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
