use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `sun.lwawt.macosx.CMenu`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CMenu";
    let java_version = registry.java_version();

    if java_version <= &JAVA_11 {
        registry.register(
            class_name,
            "nativeAddSeparator",
            "(J)V",
            native_add_separator,
        );
    }

    registry.register(class_name, "nativeCreateMenu", "(JZI)J", native_create_menu);
    registry.register(
        class_name,
        "nativeCreateSubMenu",
        "(J)J",
        native_create_sub_menu,
    );
    registry.register(class_name, "nativeDeleteItem", "(JI)V", native_delete_item);
    registry.register(class_name, "nativeGetNSMenu", "(J)J", native_get_ns_menu);
    registry.register(
        class_name,
        "nativeSetMenuTitle",
        "(JLjava/lang/String;)V",
        native_set_menu_title,
    );
}

#[async_recursion(?Send)]
async fn native_add_separator(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeAddSeparator(J)V")
}

#[async_recursion(?Send)]
async fn native_create_menu(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeCreateMenu(JZI)J")
}

#[async_recursion(?Send)]
async fn native_create_sub_menu(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeCreateSubMenu(J)J")
}

#[async_recursion(?Send)]
async fn native_delete_item(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeDeleteItem(JI)V")
}

#[async_recursion(?Send)]
async fn native_get_ns_menu(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeGetNSMenu(J)J")
}

#[async_recursion(?Send)]
async fn native_set_menu_title(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeSetMenuTitle(JLjava/lang/String;)V")
}
