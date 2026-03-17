use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardBeginTransaction(J)V", Any)]
#[async_method]
pub async fn s_card_begin_transaction<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardBeginTransaction(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/smartcardio/PCSC.SCardConnect(JLjava/lang/String;II)J",
    Any
)]
#[async_method]
pub async fn s_card_connect<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardConnect(JLjava/lang/String;II)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardControl(JI[B)[B", Any)]
#[async_method]
pub async fn s_card_control<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardControl(JI[B)[B".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardDisconnect(JI)V", Any)]
#[async_method]
pub async fn s_card_disconnect<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardDisconnect(JI)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardEndTransaction(JI)V", Any)]
#[async_method]
pub async fn s_card_end_transaction<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardEndTransaction(JI)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardEstablishContext(I)J", Any)]
#[async_method]
pub async fn s_card_establish_context<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardEstablishContext(I)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/smartcardio/PCSC.SCardGetStatusChange(JJ[I[Ljava/lang/String;)[I",
    Any
)]
#[async_method]
pub async fn s_card_get_status_change<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardGetStatusChange(JJ[I[Ljava/lang/String;)[I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/smartcardio/PCSC.SCardListReaders(J)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn s_card_list_readers<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardListReaders(J)[Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardStatus(J[B)[B", Any)]
#[async_method]
pub async fn s_card_status<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardStatus(J[B)[B".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardTransmit(JI[BII)[B", Any)]
#[async_method]
pub async fn s_card_transmit<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardTransmit(JI[BII)[B".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_s_card_begin_transaction() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_begin_transaction(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_s_card_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_connect(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_s_card_control() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_control(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_s_card_disconnect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_disconnect(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_s_card_end_transaction() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_end_transaction(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_s_card_establish_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_establish_context(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_s_card_get_status_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_get_status_change(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_s_card_list_readers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_list_readers(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_s_card_status() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_status(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_s_card_transmit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_transmit(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
