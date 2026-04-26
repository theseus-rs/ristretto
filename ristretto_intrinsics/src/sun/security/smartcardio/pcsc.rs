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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _card_id = parameters.pop_long()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_preferred_protocols = parameters.pop_int()?;
    let _j_share_mode = parameters.pop_int()?;
    let _j_reader_name = parameters.pop_reference()?;
    let _j_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardConnect(JLjava/lang/String;II)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardControl(JI[B)[B", Any)]
#[async_method]
pub async fn s_card_control<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _send_buffer = parameters.pop_reference()?;
    let _control_code = parameters.pop_int()?;
    let _card_id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardControl(JI[B)[B".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardDisconnect(JI)V", Any)]
#[async_method]
pub async fn s_card_disconnect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _disposition = parameters.pop_int()?;
    let _card_id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardDisconnect(JI)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardEndTransaction(JI)V", Any)]
#[async_method]
pub async fn s_card_end_transaction<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _disposition = parameters.pop_int()?;
    let _card_id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardEndTransaction(JI)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardEstablishContext(I)J", Any)]
#[async_method]
pub async fn s_card_establish_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _scope = parameters.pop_int()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_reader_names = parameters.pop_reference()?;
    let _j_current_state = parameters.pop_reference()?;
    let _j_timeout = parameters.pop_long()?;
    let _j_context = parameters.pop_long()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _context_id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardListReaders(J)[Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardStatus(J[B)[B", Any)]
#[async_method]
pub async fn s_card_status<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _status = parameters.pop_reference()?;
    let _card_id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.smartcardio.PCSC.SCardStatus(J[B)[B".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/smartcardio/PCSC.SCardTransmit(JI[BII)[B", Any)]
#[async_method]
pub async fn s_card_transmit<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _len = parameters.pop_int()?;
    let _ofs = parameters.pop_int()?;
    let _buf = parameters.pop_reference()?;
    let _protocol = parameters.pop_int()?;
    let _card_id = parameters.pop_long()?;
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
        let result = s_card_begin_transaction(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.smartcardio.PCSC.SCardBeginTransaction(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_s_card_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_connect(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.smartcardio.PCSC.SCardConnect(JLjava/lang/String;II)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_s_card_control() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_control(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.smartcardio.PCSC.SCardControl(JI[B)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_s_card_disconnect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            s_card_disconnect(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.security.smartcardio.PCSC.SCardDisconnect(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_s_card_end_transaction() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            s_card_end_transaction(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "sun.security.smartcardio.PCSC.SCardEndTransaction(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_s_card_establish_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_establish_context(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.security.smartcardio.PCSC.SCardEstablishContext(I)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_s_card_get_status_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_get_status_change(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.smartcardio.PCSC.SCardGetStatusChange(JJ[I[Ljava/lang/String;)[I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_s_card_list_readers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_list_readers(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.smartcardio.PCSC.SCardListReaders(J)[Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_s_card_status() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_status(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.smartcardio.PCSC.SCardStatus(J[B)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_s_card_transmit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = s_card_transmit(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.smartcardio.PCSC.SCardTransmit(JI[BII)[B",
            result.unwrap_err().to_string()
        );
    }
}
