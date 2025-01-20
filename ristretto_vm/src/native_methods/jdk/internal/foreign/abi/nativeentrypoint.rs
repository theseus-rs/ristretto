use crate::native_methods::registry::{MethodRegistry, JAVA_19, JAVA_20, JAVA_21};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/foreign/abi/NativeEntryPoint";

/// Register all native methods for `jdk.internal.foreign.abi.NativeEntryPoint`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() == JAVA_19 {
        registry.register(CLASS_NAME, "makeDowncallStub", "(Ljava/lang/invoke/MethodType;Ljdk/internal/foreign/abi/ABIDescriptor;[Ljdk/internal/foreign/abi/VMStorage;[Ljdk/internal/foreign/abi/VMStorage;Z)J", make_downcall_stub);
    }
    if registry.java_major_version() == JAVA_20 {
        registry.register(CLASS_NAME, "makeDowncallStub", "(Ljava/lang/invoke/MethodType;Ljdk/internal/foreign/abi/ABIDescriptor;[Ljdk/internal/foreign/abi/VMStorage;[Ljdk/internal/foreign/abi/VMStorage;ZI)J", make_downcall_stub);
    }
    if registry.java_major_version() >= JAVA_21 {
        registry.register(CLASS_NAME, "makeDowncallStub", "(Ljava/lang/invoke/MethodType;Ljdk/internal/foreign/abi/ABIDescriptor;[Ljdk/internal/foreign/abi/VMStorage;[Ljdk/internal/foreign/abi/VMStorage;ZIZ)J", make_downcall_stub);
    }

    registry.register(
        CLASS_NAME,
        "freeDowncallStub0",
        "(J)Z",
        free_downcall_stub_0,
    );
    registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn free_downcall_stub_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.NativeEntryPoint.freeDowncallStub0(J)Z")
}

#[async_recursion(?Send)]
async fn make_downcall_stub(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.NativeEntryPoint.makeDowncallStub(Ljava/lang/invoke/MethodType;Ljdk/internal/foreign/abi/ABIDescriptor;[Ljdk/internal/foreign/abi/VMStorage;[Ljdk/internal/foreign/abi/VMStorage;Z)J")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.NativeEntryPoint.freeDowncallStub0(J)Z"
    )]
    async fn test_free_downcall_stub_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_downcall_stub_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.NativeEntryPoint.makeDowncallStub(Ljava/lang/invoke/MethodType;Ljdk/internal/foreign/abi/ABIDescriptor;[Ljdk/internal/foreign/abi/VMStorage;[Ljdk/internal/foreign/abi/VMStorage;Z)J"
    )]
    async fn test_make_downcall_stub() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = make_downcall_stub(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
