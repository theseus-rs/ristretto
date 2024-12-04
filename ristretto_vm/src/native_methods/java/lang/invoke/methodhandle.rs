use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `java.lang.invoke.MethodHandle`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/invoke/MethodHandle";
    let java_version = registry.java_version();

    if java_version >= &JAVA_17 {
        registry.register(
            class_name,
            "linkToNative",
            "([Ljava/lang/Object;)Ljava/lang/Object;",
            link_to_native,
        );
    }

    registry.register(
        class_name,
        "invoke",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        invoke,
    );
    registry.register(
        class_name,
        "invokeBasic",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_basic,
    );
    registry.register(
        class_name,
        "invokeExact",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_exact,
    );
    registry.register(
        class_name,
        "linkToInterface",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        link_to_interface,
    );
    registry.register(
        class_name,
        "linkToSpecial",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        link_to_special,
    );
    registry.register(
        class_name,
        "linkToStatic",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        link_to_static,
    );
    registry.register(
        class_name,
        "linkToVirtual",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        link_to_virtual,
    );
}

#[async_recursion(?Send)]
async fn invoke(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn invoke_basic(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn invoke_exact(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn link_to_interface(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn link_to_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn link_to_special(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn link_to_static(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn link_to_virtual(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
