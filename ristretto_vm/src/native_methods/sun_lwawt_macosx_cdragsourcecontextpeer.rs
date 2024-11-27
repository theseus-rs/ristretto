use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `sun.lwawt.macosx.CDragSourceContextPeer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CDragSourceContextPeer";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "createNativeDragSource", "(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJLsun/lwawt/macosx/CImage;III[JLjava/util/Map;)J", create_native_drag_source);
    } else {
        registry.register(class_name, "createNativeDragSource", "(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJJIII[JLjava/util/Map;)J", create_native_drag_source);
    }

    registry.register(class_name, "doDragging", "(J)V", do_dragging);
    registry.register(
        class_name,
        "releaseNativeDragSource",
        "(J)V",
        release_native_drag_source,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn create_native_drag_source(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn do_dragging(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn release_native_drag_source(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
