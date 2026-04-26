use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/net/DualStackPlainSocketImpl.accept0(I[Ljava/net/InetSocketAddress;)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn accept0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainSocketImpl.accept0(I[Ljava/net/InetSocketAddress;)I".to_string(),
    )
    .into())
}
#[intrinsic_method("java/net/DualStackPlainSocketImpl.available0(I)I", Equal(JAVA_8))]
#[async_method]
pub async fn available0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainSocketImpl.available0(I)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainSocketImpl.bind0(ILjava/net/InetAddress;IZ)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn bind0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg3 = parameters.pop_bool()?;
    let _arg2 = parameters.pop_int()?;
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainSocketImpl.bind0(ILjava/net/InetAddress;IZ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("java/net/DualStackPlainSocketImpl.close0(I)V", Equal(JAVA_8))]
#[async_method]
pub async fn close0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("java/net/DualStackPlainSocketImpl.close0(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "java/net/DualStackPlainSocketImpl.configureBlocking(IZ)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn configure_blocking<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_bool()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainSocketImpl.configureBlocking(IZ)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainSocketImpl.connect0(ILjava/net/InetAddress;I)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn connect0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_int()?;
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainSocketImpl.connect0(ILjava/net/InetAddress;I)I".to_string(),
    )
    .into())
}
#[intrinsic_method("java/net/DualStackPlainSocketImpl.getIntOption(II)I", Equal(JAVA_8))]
#[async_method]
pub async fn get_int_option<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainSocketImpl.getIntOption(II)I".to_string(),
    )
    .into())
}
#[intrinsic_method("java/net/DualStackPlainSocketImpl.initIDs()V", Equal(JAVA_8))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("java/net/DualStackPlainSocketImpl.initIDs()V".to_string())
            .into(),
    )
}
#[intrinsic_method("java/net/DualStackPlainSocketImpl.listen0(II)V", Equal(JAVA_8))]
#[async_method]
pub async fn listen0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainSocketImpl.listen0(II)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainSocketImpl.localAddress(ILjava/net/InetAddressContainer;)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn local_address<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainSocketImpl.localAddress(ILjava/net/InetAddressContainer;)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method("java/net/DualStackPlainSocketImpl.localPort0(I)I", Equal(JAVA_8))]
#[async_method]
pub async fn local_port0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainSocketImpl.localPort0(I)I".to_string(),
    )
    .into())
}
#[intrinsic_method("java/net/DualStackPlainSocketImpl.sendOOB(II)V", Equal(JAVA_8))]
#[async_method]
pub async fn send_oob<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainSocketImpl.sendOOB(II)V".to_string(),
    )
    .into())
}
#[intrinsic_method("java/net/DualStackPlainSocketImpl.setIntOption(III)V", Equal(JAVA_8))]
#[async_method]
pub async fn set_int_option<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_int()?;
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainSocketImpl.setIntOption(III)V".to_string(),
    )
    .into())
}
#[intrinsic_method("java/net/DualStackPlainSocketImpl.shutdown0(II)V", Equal(JAVA_8))]
#[async_method]
pub async fn shutdown0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainSocketImpl.shutdown0(II)V".to_string(),
    )
    .into())
}
#[intrinsic_method("java/net/DualStackPlainSocketImpl.socket0(ZZ)I", Equal(JAVA_8))]
#[async_method]
pub async fn socket0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_bool()?;
    let _arg0 = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainSocketImpl.socket0(ZZ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("java/net/DualStackPlainSocketImpl.waitForConnect(II)V", Equal(JAVA_8))]
#[async_method]
pub async fn wait_for_connect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainSocketImpl.waitForConnect(II)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainSocketImpl.waitForNewConnection(II)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn wait_for_new_connection<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainSocketImpl.waitForNewConnection(II)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_accept0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = accept0(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.accept0(I[Ljava/net/InetSocketAddress;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_available0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = available0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.available0(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_bind0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = bind0(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.bind0(ILjava/net/InetAddress;IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.close0(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_configure_blocking() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = configure_blocking(
            thread,
            Parameters::new(vec![Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.configureBlocking(IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_connect0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = connect0(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.connect0(ILjava/net/InetAddress;I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_int_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_int_option(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.getIntOption(II)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_listen0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = listen0(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.listen0(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_local_address() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = local_address(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.localAddress(ILjava/net/InetAddressContainer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_local_port0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = local_port0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.localPort0(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_send_oob() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = send_oob(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.sendOOB(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_int_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_int_option(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.setIntOption(III)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_shutdown0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = shutdown0(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.shutdown0(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket0(
            thread,
            Parameters::new(vec![Value::from(false), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.socket0(ZZ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_wait_for_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            wait_for_connect(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.waitForConnect(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_wait_for_new_connection() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            wait_for_new_connection(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "java/net/DualStackPlainSocketImpl.waitForNewConnection(II)V",
            result.unwrap_err().to_string()
        );
    }
}
