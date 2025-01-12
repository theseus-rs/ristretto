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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CMenu";
        assert!(registry
            .method(class_name, "nativeCreateMenu", "(JZI)J")
            .is_some());
        assert!(registry
            .method(class_name, "nativeCreateSubMenu", "(J)J")
            .is_some());
        assert!(registry
            .method(class_name, "nativeDeleteItem", "(JI)V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeGetNSMenu", "(J)J")
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetMenuTitle", "(JLjava/lang/String;)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CMenu.nativeCreateMenu(JZI)J")]
    async fn test_native_create_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_menu(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CMenu.nativeCreateSubMenu(J)J")]
    async fn test_native_create_sub_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_sub_menu(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CMenu.nativeDeleteItem(JI)V")]
    async fn test_native_delete_item() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_delete_item(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CMenu.nativeGetNSMenu(J)J")]
    async fn test_native_get_ns_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_ns_menu(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CMenu.nativeSetMenuTitle(JLjava/lang/String;)V")]
    async fn test_native_set_menu_title() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_menu_title(thread, Arguments::default()).await;
    }
}
