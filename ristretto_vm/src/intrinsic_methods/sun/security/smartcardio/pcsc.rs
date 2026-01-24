use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardBeginTransaction(J)V", Any)]
#[async_method]
pub(crate) async fn s_card_begin_transaction(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardBeginTransaction(J)V")
}

#[intrinsic_method(
    "sun/security/smartcardio/PCSC.SCardConnect(JLjava/lang/String;II)J",
    Any
)]
#[async_method]
pub(crate) async fn s_card_connect(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardConnect(JLjava/lang/String;II)J")
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardControl(JI[B)[B", Any)]
#[async_method]
pub(crate) async fn s_card_control(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardControl(JI[B)[B")
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardDisconnect(JI)V", Any)]
#[async_method]
pub(crate) async fn s_card_disconnect(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardDisconnect(JI)V")
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardEndTransaction(JI)V", Any)]
#[async_method]
pub(crate) async fn s_card_end_transaction(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardEndTransaction(JI)V")
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardEstablishContext(I)J", Any)]
#[async_method]
pub(crate) async fn s_card_establish_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardEstablishContext(I)J")
}

#[intrinsic_method(
    "sun/security/smartcardio/PCSC.SCardGetStatusChange(JJ[I[Ljava/lang/String;)[I",
    Any
)]
#[async_method]
pub(crate) async fn s_card_get_status_change(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardGetStatusChange(JJ[I[Ljava/lang/String;)[I")
}

#[intrinsic_method(
    "sun/security/smartcardio/PCSC.SCardListReaders(J)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub(crate) async fn s_card_list_readers(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardListReaders(J)[Ljava/lang/String;")
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardStatus(J[B)[B", Any)]
#[async_method]
pub(crate) async fn s_card_status(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardStatus(J[B)[B")
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardTransmit(JI[BII)[B", Any)]
#[async_method]
pub(crate) async fn s_card_transmit(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardTransmit(JI[BII)[B")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.smartcardio.PCSC.SCardBeginTransaction(J)V"
    )]
    async fn test_s_card_begin_transaction() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_begin_transaction(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.smartcardio.PCSC.SCardConnect(JLjava/lang/String;II)J"
    )]
    async fn test_s_card_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_connect(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.smartcardio.PCSC.SCardControl(JI[B)[B"
    )]
    async fn test_s_card_control() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_control(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.smartcardio.PCSC.SCardDisconnect(JI)V"
    )]
    async fn test_s_card_disconnect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_disconnect(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.smartcardio.PCSC.SCardEndTransaction(JI)V"
    )]
    async fn test_s_card_end_transaction() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_end_transaction(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.smartcardio.PCSC.SCardEstablishContext(I)J"
    )]
    async fn test_s_card_establish_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_establish_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.smartcardio.PCSC.SCardGetStatusChange(JJ[I[Ljava/lang/String;)[I"
    )]
    async fn test_s_card_get_status_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_get_status_change(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.smartcardio.PCSC.SCardListReaders(J)[Ljava/lang/String;"
    )]
    async fn test_s_card_list_readers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_list_readers(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.smartcardio.PCSC.SCardStatus(J[B)[B"
    )]
    async fn test_s_card_status() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_status(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.smartcardio.PCSC.SCardTransmit(JI[BII)[B"
    )]
    async fn test_s_card_transmit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_transmit(thread, Parameters::default()).await;
    }
}
