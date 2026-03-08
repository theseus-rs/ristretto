use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// Platform-specific sockaddr layout constants (macOS / BSD).
#[cfg(target_os = "macos")]
mod addr_const {
    pub const AF_INET: i32 = 2;
    pub const AF_INET6: i32 = 30;
    // macOS has sin_len (u8) before sin_family (u8)
    pub const OFFSET_FAMILY: i32 = 1;
    pub const SIZEOF_FAMILY: i32 = 1;
    pub const OFFSET_SIN4_PORT: i32 = 2;
    pub const OFFSET_SIN4_ADDR: i32 = 4;
    pub const SIZEOF_SOCKADDR4: i32 = 16;
    pub const OFFSET_SIN6_PORT: i32 = 2;
    pub const OFFSET_SIN6_FLOWINFO: i32 = 4;
    pub const OFFSET_SIN6_ADDR: i32 = 8;
    pub const OFFSET_SIN6_SCOPEID: i32 = 24;
    pub const SIZEOF_SOCKADDR6: i32 = 28;
}

/// Platform-specific sockaddr layout constants (Linux).
#[cfg(target_os = "linux")]
mod addr_const {
    pub const AF_INET: i32 = 2;
    pub const AF_INET6: i32 = 10;
    // Linux has sa_family (u16) at offset 0
    pub const OFFSET_FAMILY: i32 = 0;
    pub const SIZEOF_FAMILY: i32 = 2;
    pub const OFFSET_SIN4_PORT: i32 = 2;
    pub const OFFSET_SIN4_ADDR: i32 = 4;
    pub const SIZEOF_SOCKADDR4: i32 = 16;
    pub const OFFSET_SIN6_PORT: i32 = 2;
    pub const OFFSET_SIN6_FLOWINFO: i32 = 4;
    pub const OFFSET_SIN6_ADDR: i32 = 8;
    pub const OFFSET_SIN6_SCOPEID: i32 = 24;
    pub const SIZEOF_SOCKADDR6: i32 = 28;
}

/// Fallback for other platforms.
#[cfg(not(any(target_os = "macos", target_os = "linux")))]
mod addr_const {
    pub const AF_INET: i32 = 2;
    pub const AF_INET6: i32 = 23;
    pub const OFFSET_FAMILY: i32 = 0;
    pub const SIZEOF_FAMILY: i32 = 2;
    pub const OFFSET_SIN4_PORT: i32 = 2;
    pub const OFFSET_SIN4_ADDR: i32 = 4;
    pub const SIZEOF_SOCKADDR4: i32 = 16;
    pub const OFFSET_SIN6_PORT: i32 = 2;
    pub const OFFSET_SIN6_FLOWINFO: i32 = 4;
    pub const OFFSET_SIN6_ADDR: i32 = 8;
    pub const OFFSET_SIN6_SCOPEID: i32 = 24;
    pub const SIZEOF_SOCKADDR6: i32 = 28;
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.AFINET()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn afinet<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(addr_const::AF_INET)))
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.AFINET6()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn afinet_6<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(addr_const::AF_INET6)))
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.offsetFamily()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn offset_family<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(addr_const::OFFSET_FAMILY)))
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.offsetSin4Addr()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn offset_sin_4_addr<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(addr_const::OFFSET_SIN4_ADDR)))
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.offsetSin4Port()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn offset_sin_4_port<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(addr_const::OFFSET_SIN4_PORT)))
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.offsetSin6Addr()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn offset_sin_6_addr<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(addr_const::OFFSET_SIN6_ADDR)))
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.offsetSin6FlowInfo()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn offset_sin_6_flow_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(addr_const::OFFSET_SIN6_FLOWINFO)))
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.offsetSin6Port()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn offset_sin_6_port<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(addr_const::OFFSET_SIN6_PORT)))
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.offsetSin6ScopeId()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn offset_sin_6_scope_id<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(addr_const::OFFSET_SIN6_SCOPEID)))
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.sizeofFamily()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn sizeof_family<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(addr_const::SIZEOF_FAMILY)))
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.sizeofSockAddr4()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn sizeof_sock_addr_4<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(addr_const::SIZEOF_SOCKADDR4)))
}

#[intrinsic_method(
    "sun/nio/ch/NativeSocketAddress.sizeofSockAddr6()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn sizeof_sock_addr_6<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(addr_const::SIZEOF_SOCKADDR6)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_afinet() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = afinet(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(addr_const::AF_INET)));
        Ok(())
    }

    #[tokio::test]
    async fn test_afinet_6() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = afinet_6(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(addr_const::AF_INET6)));
        Ok(())
    }

    #[tokio::test]
    async fn test_offset_family() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = offset_family(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(addr_const::OFFSET_FAMILY)));
        Ok(())
    }

    #[tokio::test]
    async fn test_offset_sin_4_addr() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = offset_sin_4_addr(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(addr_const::OFFSET_SIN4_ADDR)));
        Ok(())
    }

    #[tokio::test]
    async fn test_offset_sin_4_port() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = offset_sin_4_port(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(addr_const::OFFSET_SIN4_PORT)));
        Ok(())
    }

    #[tokio::test]
    async fn test_offset_sin_6_addr() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = offset_sin_6_addr(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(addr_const::OFFSET_SIN6_ADDR)));
        Ok(())
    }

    #[tokio::test]
    async fn test_offset_sin_6_flow_info() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = offset_sin_6_flow_info(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(addr_const::OFFSET_SIN6_FLOWINFO)));
        Ok(())
    }

    #[tokio::test]
    async fn test_offset_sin_6_port() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = offset_sin_6_port(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(addr_const::OFFSET_SIN6_PORT)));
        Ok(())
    }

    #[tokio::test]
    async fn test_offset_sin_6_scope_id() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = offset_sin_6_scope_id(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(addr_const::OFFSET_SIN6_SCOPEID)));
        Ok(())
    }

    #[tokio::test]
    async fn test_sizeof_family() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = sizeof_family(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(addr_const::SIZEOF_FAMILY)));
        Ok(())
    }

    #[tokio::test]
    async fn test_sizeof_sock_addr_4() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = sizeof_sock_addr_4(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(addr_const::SIZEOF_SOCKADDR4)));
        Ok(())
    }

    #[tokio::test]
    async fn test_sizeof_sock_addr_6() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = sizeof_sock_addr_6(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(addr_const::SIZEOF_SOCKADDR6)));
        Ok(())
    }
}
