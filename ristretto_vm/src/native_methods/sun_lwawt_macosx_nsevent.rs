use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `sun.lwawt.macosx.NSEvent`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/NSEvent";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "nsToJavaChar", "(CI)C", ns_to_java_char);
        registry.register(
            class_name,
            "nsToJavaKeyModifiers",
            "(I)I",
            ns_to_java_key_modifiers,
        );
        registry.register(
            class_name,
            "nsToJavaMouseModifiers",
            "(II)I",
            ns_to_java_mouse_modifiers,
        );
    } else {
        registry.register(class_name, "nsToJavaChar", "(CIZ)C", ns_to_java_char);
        registry.register(
            class_name,
            "nsToJavaModifiers",
            "(I)I",
            ns_to_java_modifiers,
        );
    }

    registry.register(
        class_name,
        "nsKeyModifiersToJavaKeyInfo",
        "([I[I)V",
        ns_key_modifiers_to_java_key_info,
    );
    registry.register(class_name, "nsToJavaChar", "(CIZ)C", ns_to_java_char);
    registry.register(
        class_name,
        "nsToJavaKeyInfo",
        "([I[I)Z",
        ns_to_java_key_info,
    );
    registry.register(
        class_name,
        "nsToJavaModifiers",
        "(I)I",
        ns_to_java_modifiers,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn ns_key_modifiers_to_java_key_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn ns_to_java_char(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn ns_to_java_key_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn ns_to_java_key_modifiers(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn ns_to_java_modifiers(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn ns_to_java_mouse_modifiers(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
