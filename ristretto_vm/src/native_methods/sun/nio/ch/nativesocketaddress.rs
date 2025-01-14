use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.ch.NativeSocketAddress`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/NativeSocketAddress";
    registry.register(class_name, "AFINET", "()I", afinet);
    registry.register(class_name, "AFINET6", "()I", afinet_6);
    registry.register(class_name, "offsetFamily", "()I", offset_family);
    registry.register(class_name, "offsetSin4Addr", "()I", offset_sin_4_addr);
    registry.register(class_name, "offsetSin4Port", "()I", offset_sin_4_port);
    registry.register(class_name, "offsetSin6Addr", "()I", offset_sin_6_addr);
    registry.register(
        class_name,
        "offsetSin6FlowInfo",
        "()I",
        offset_sin_6_flow_info,
    );
    registry.register(class_name, "offsetSin6Port", "()I", offset_sin_6_port);
    registry.register(
        class_name,
        "offsetSin6ScopeId",
        "()I",
        offset_sin_6_scope_id,
    );
    registry.register(class_name, "sizeofFamily", "()I", sizeof_family);
    registry.register(class_name, "sizeofSockAddr4", "()I", sizeof_sock_addr_4);
    registry.register(class_name, "sizeofSockAddr6", "()I", sizeof_sock_addr_6);
}

#[async_recursion(?Send)]
async fn afinet(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.AF_INET()I");
}

#[async_recursion(?Send)]
async fn afinet_6(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.AF_INET6()I");
}

#[async_recursion(?Send)]
async fn offset_family(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetFamily()I");
}

#[async_recursion(?Send)]
async fn offset_sin_4_addr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin4Addr()I");
}

#[async_recursion(?Send)]
async fn offset_sin_4_port(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin4Port()I");
}

#[async_recursion(?Send)]
async fn offset_sin_6_addr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin6Addr()I");
}

#[async_recursion(?Send)]
async fn offset_sin_6_flow_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin6FlowInfo()I");
}

#[async_recursion(?Send)]
async fn offset_sin_6_port(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin6Port()I");
}

#[async_recursion(?Send)]
async fn offset_sin_6_scope_id(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin6ScopeId()I");
}

#[async_recursion(?Send)]
async fn sizeof_family(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.sizeofFamily()I");
}

#[async_recursion(?Send)]
async fn sizeof_sock_addr_4(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.sizeofSockAddr4()I");
}

#[async_recursion(?Send)]
async fn sizeof_sock_addr_6(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.sizeofSockAddr6()I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/nio/ch/NativeSocketAddress";
        assert!(registry.method(class_name, "AFINET", "()I").is_some());
        assert!(registry.method(class_name, "AFINET6", "()I").is_some());
        assert!(registry.method(class_name, "offsetFamily", "()I").is_some());
        assert!(registry
            .method(class_name, "offsetSin4Addr", "()I")
            .is_some());
        assert!(registry
            .method(class_name, "offsetSin4Port", "()I")
            .is_some());
        assert!(registry
            .method(class_name, "offsetSin6Addr", "()I")
            .is_some());
        assert!(registry
            .method(class_name, "offsetSin6FlowInfo", "()I")
            .is_some());
        assert!(registry
            .method(class_name, "offsetSin6Port", "()I")
            .is_some());
        assert!(registry
            .method(class_name, "offsetSin6ScopeId", "()I")
            .is_some());
        assert!(registry.method(class_name, "sizeofFamily", "()I").is_some());
        assert!(registry
            .method(class_name, "sizeofSockAddr4", "()I")
            .is_some());
        assert!(registry
            .method(class_name, "sizeofSockAddr6", "()I")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.NativeSocketAddress.AF_INET()I")]
    async fn test_afinet() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = afinet(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.NativeSocketAddress.AF_INET6()I")]
    async fn test_afinet_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = afinet_6(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.NativeSocketAddress.offsetFamily()I")]
    async fn test_offset_family() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = offset_family(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.NativeSocketAddress.offsetSin4Addr()I")]
    async fn test_offset_sin_4_addr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = offset_sin_4_addr(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.NativeSocketAddress.offsetSin4Port()I")]
    async fn test_offset_sin_4_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = offset_sin_4_port(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.NativeSocketAddress.offsetSin6Addr()I")]
    async fn test_offset_sin_6_addr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = offset_sin_6_addr(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.NativeSocketAddress.offsetSin6FlowInfo()I")]
    async fn test_offset_sin_6_flow_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = offset_sin_6_flow_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.NativeSocketAddress.offsetSin6Port()I")]
    async fn test_offset_sin_6_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = offset_sin_6_port(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.NativeSocketAddress.offsetSin6ScopeId()I")]
    async fn test_offset_sin_6_scope_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = offset_sin_6_scope_id(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.NativeSocketAddress.sizeofFamily()I")]
    async fn test_sizeof_family() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sizeof_family(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.NativeSocketAddress.sizeofSockAddr4()I")]
    async fn test_sizeof_sock_addr_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sizeof_sock_addr_4(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.NativeSocketAddress.sizeofSockAddr6()I")]
    async fn test_sizeof_sock_addr_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sizeof_sock_addr_6(thread, Arguments::default()).await;
    }
}
