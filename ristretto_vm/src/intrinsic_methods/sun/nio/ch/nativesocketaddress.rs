use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.AFINET()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn afinet(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.AF_INET()I");
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.AFINET6()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn afinet_6(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.AF_INET6()I");
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.offsetFamily()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn offset_family(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetFamily()I");
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.offsetSin4Addr()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn offset_sin_4_addr(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin4Addr()I");
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.offsetSin4Port()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn offset_sin_4_port(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin4Port()I");
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.offsetSin6Addr()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn offset_sin_6_addr(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin6Addr()I");
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.offsetSin6FlowInfo()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn offset_sin_6_flow_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin6FlowInfo()I");
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.offsetSin6Port()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn offset_sin_6_port(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin6Port()I");
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.offsetSin6ScopeId()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn offset_sin_6_scope_id(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.offsetSin6ScopeId()I");
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.sizeofFamily()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn sizeof_family(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.sizeofFamily()I");
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.sizeofSockAddr4()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn sizeof_sock_addr_4(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeSocketAddress.sizeofSockAddr4()I");
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.sizeofSockAddr6()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn sizeof_sock_addr_6(
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
