use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `sun.awt.CGraphicsEnvironment`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/CGraphicsEnvironment";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "initCocoa", "()V", init_cocoa);
    }

    registry.register(
        class_name,
        "deregisterDisplayReconfiguration",
        "(J)V",
        deregister_display_reconfiguration,
    );
    registry.register(class_name, "getDisplayIDs", "()[I", get_display_i_ds);
    registry.register(class_name, "getMainDisplayID", "()I", get_main_display_id);
    registry.register(
        class_name,
        "registerDisplayReconfiguration",
        "()J",
        register_display_reconfiguration,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn deregister_display_reconfiguration(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_display_i_ds(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_main_display_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_cocoa(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_display_reconfiguration(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
