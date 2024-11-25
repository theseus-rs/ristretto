use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `com.apple.eawt._AppMenuBarHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/apple/eawt/_AppMenuBarHandler";
    let java_version = registry.java_version();

    if java_version >= &JAVA_17 {
        registry.register(
            class_name,
            "nativeActivateDefaultMenuBar",
            "(J)V",
            native_activate_default_menu_bar,
        );
    }

    registry.register(
        class_name,
        "nativeSetDefaultMenuBar",
        "(J)V",
        native_set_default_menu_bar,
    );
    registry.register(
        class_name,
        "nativeSetMenuState",
        "(IZZ)V",
        native_set_menu_state,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_activate_default_menu_bar(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_set_default_menu_bar(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_set_menu_state(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
