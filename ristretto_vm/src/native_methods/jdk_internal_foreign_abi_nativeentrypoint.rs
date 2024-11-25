use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_19: Version = Version::Java19 { minor: 0 };

/// Register all native methods for `jdk.internal.foreign.abi.NativeEntryPoint`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/foreign/abi/NativeEntryPoint";
    let java_version = registry.java_version();

    if java_version >= &JAVA_19 {
        registry.register(class_name, "makeDowncallStub", "(Ljava/lang/invoke/MethodType;Ljdk/internal/foreign/abi/ABIDescriptor;[Ljdk/internal/foreign/abi/VMStorage;[Ljdk/internal/foreign/abi/VMStorage;Z)J", make_downcall_stub);
    } else {
        registry.register(class_name, "makeDowncallStub", "(Ljava/lang/invoke/MethodType;Ljdk/internal/foreign/abi/ABIDescriptor;[Ljdk/internal/foreign/abi/VMStorage;[Ljdk/internal/foreign/abi/VMStorage;ZI)J", make_downcall_stub);
    }

    registry.register(
        class_name,
        "freeDowncallStub0",
        "(J)Z",
        free_downcall_stub_0,
    );
    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn free_downcall_stub_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn make_downcall_stub(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
