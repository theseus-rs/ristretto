use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/NativeSocketAddress";

/// Register all native methods for `sun.nio.ch.NativeSocketAddress`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "AFINET", "()I", afinet);
    registry.register(CLASS_NAME, "AFINET6", "()I", afinet_6);
    registry.register(CLASS_NAME, "offsetFamily", "()I", offset_family);
    registry.register(CLASS_NAME, "offsetSin4Addr", "()I", offset_sin_4_addr);
    registry.register(CLASS_NAME, "offsetSin4Port", "()I", offset_sin_4_port);
    registry.register(CLASS_NAME, "offsetSin6Addr", "()I", offset_sin_6_addr);
    registry.register(
        CLASS_NAME,
        "offsetSin6FlowInfo",
        "()I",
        offset_sin_6_flow_info,
    );
    registry.register(CLASS_NAME, "offsetSin6Port", "()I", offset_sin_6_port);
    registry.register(
        CLASS_NAME,
        "offsetSin6ScopeId",
        "()I",
        offset_sin_6_scope_id,
    );
    registry.register(CLASS_NAME, "sizeofFamily", "()I", sizeof_family);
    registry.register(CLASS_NAME, "sizeofSockAddr4", "()I", sizeof_sock_addr_4);
    registry.register(CLASS_NAME, "sizeofSockAddr6", "()I", sizeof_sock_addr_6);
}

#[async_recursion(?Send)]
async fn afinet(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.AF_INET()I");
}

#[async_recursion(?Send)]
async fn afinet_6(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.AF_INET6()I");
}

#[async_recursion(?Send)]
async fn offset_family(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetFamily()I");
}

#[async_recursion(?Send)]
async fn offset_sin_4_addr(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin4Addr()I");
}

#[async_recursion(?Send)]
async fn offset_sin_4_port(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin4Port()I");
}

#[async_recursion(?Send)]
async fn offset_sin_6_addr(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin6Addr()I");
}

#[async_recursion(?Send)]
async fn offset_sin_6_flow_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin6FlowInfo()I");
}

#[async_recursion(?Send)]
async fn offset_sin_6_port(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin6Port()I");
}

#[async_recursion(?Send)]
async fn offset_sin_6_scope_id(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin6ScopeId()I");
}

#[async_recursion(?Send)]
async fn sizeof_family(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.sizeofFamily()I");
}

#[async_recursion(?Send)]
async fn sizeof_sock_addr_4(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.sizeofSockAddr4()I");
}

#[async_recursion(?Send)]
async fn sizeof_sock_addr_6(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.sizeofSockAddr6()I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.NativeSocketAddress.AF_INET()I")]
    async fn test_afinet() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = afinet(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.NativeSocketAddress.AF_INET6()I")]
    async fn test_afinet_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = afinet_6(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.NativeSocketAddress.offsetFamily()I"
    )]
    async fn test_offset_family() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = offset_family(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.NativeSocketAddress.offsetSin4Addr()I"
    )]
    async fn test_offset_sin_4_addr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = offset_sin_4_addr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.NativeSocketAddress.offsetSin4Port()I"
    )]
    async fn test_offset_sin_4_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = offset_sin_4_port(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.NativeSocketAddress.offsetSin6Addr()I"
    )]
    async fn test_offset_sin_6_addr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = offset_sin_6_addr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.NativeSocketAddress.offsetSin6FlowInfo()I"
    )]
    async fn test_offset_sin_6_flow_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = offset_sin_6_flow_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.NativeSocketAddress.offsetSin6Port()I"
    )]
    async fn test_offset_sin_6_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = offset_sin_6_port(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.NativeSocketAddress.offsetSin6ScopeId()I"
    )]
    async fn test_offset_sin_6_scope_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = offset_sin_6_scope_id(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.NativeSocketAddress.sizeofFamily()I"
    )]
    async fn test_sizeof_family() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sizeof_family(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.NativeSocketAddress.sizeofSockAddr4()I"
    )]
    async fn test_sizeof_sock_addr_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sizeof_sock_addr_4(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.NativeSocketAddress.sizeofSockAddr6()I"
    )]
    async fn test_sizeof_sock_addr_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sizeof_sock_addr_6(thread, Parameters::default()).await;
    }
}
