use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.security.smartcardio.PCSC`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/security/smartcardio/PCSC";
    registry.register(
        class_name,
        "SCardBeginTransaction",
        "(J)V",
        s_card_begin_transaction,
    );
    registry.register(
        class_name,
        "SCardConnect",
        "(JLjava/lang/String;II)J",
        s_card_connect,
    );
    registry.register(class_name, "SCardControl", "(JI[B)[B", s_card_control);
    registry.register(class_name, "SCardDisconnect", "(JI)V", s_card_disconnect);
    registry.register(
        class_name,
        "SCardEndTransaction",
        "(JI)V",
        s_card_end_transaction,
    );
    registry.register(
        class_name,
        "SCardEstablishContext",
        "(I)J",
        s_card_establish_context,
    );
    registry.register(
        class_name,
        "SCardGetStatusChange",
        "(JJ[I[Ljava/lang/String;)[I",
        s_card_get_status_change,
    );
    registry.register(
        class_name,
        "SCardListReaders",
        "(J)[Ljava/lang/String;",
        s_card_list_readers,
    );
    registry.register(class_name, "SCardStatus", "(J[B)[B", s_card_status);
    registry.register(class_name, "SCardTransmit", "(JI[BII)[B", s_card_transmit);
}

#[async_recursion(?Send)]
async fn s_card_begin_transaction(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardBeginTransaction(J)V")
}

#[async_recursion(?Send)]
async fn s_card_connect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardConnect(JLjava/lang/String;II)J")
}

#[async_recursion(?Send)]
async fn s_card_control(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardControl(JI[B)[B")
}

#[async_recursion(?Send)]
async fn s_card_disconnect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardDisconnect(JI)V")
}

#[async_recursion(?Send)]
async fn s_card_end_transaction(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardEndTransaction(JI)V")
}

#[async_recursion(?Send)]
async fn s_card_establish_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardEstablishContext(I)J")
}

#[async_recursion(?Send)]
async fn s_card_get_status_change(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardGetStatusChange(JJ[I[Ljava/lang/String;)[I")
}

#[async_recursion(?Send)]
async fn s_card_list_readers(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardListReaders(J)[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn s_card_status(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardStatus(J[B)[B")
}

#[async_recursion(?Send)]
async fn s_card_transmit(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PCSC.SCardTransmit(JI[BII)[B")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/security/smartcardio/PCSC";
        assert!(registry
            .method(class_name, "SCardBeginTransaction", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "SCardConnect", "(JLjava/lang/String;II)J")
            .is_some());
        assert!(registry
            .method(class_name, "SCardControl", "(JI[B)[B")
            .is_some());
        assert!(registry
            .method(class_name, "SCardDisconnect", "(JI)V")
            .is_some());
        assert!(registry
            .method(class_name, "SCardEndTransaction", "(JI)V")
            .is_some());
        assert!(registry
            .method(class_name, "SCardEstablishContext", "(I)J")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "SCardGetStatusChange",
                "(JJ[I[Ljava/lang/String;)[I"
            )
            .is_some());
        assert!(registry
            .method(class_name, "SCardListReaders", "(J)[Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(class_name, "SCardStatus", "(J[B)[B")
            .is_some());
        assert!(registry
            .method(class_name, "SCardTransmit", "(JI[BII)[B")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.security.smartcardio.PCSC.SCardBeginTransaction(J)V")]
    async fn test_s_card_begin_transaction() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_begin_transaction(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.security.smartcardio.PCSC.SCardConnect(JLjava/lang/String;II)J")]
    async fn test_s_card_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_connect(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.security.smartcardio.PCSC.SCardControl(JI[B)[B")]
    async fn test_s_card_control() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_control(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.security.smartcardio.PCSC.SCardDisconnect(JI)V")]
    async fn test_s_card_disconnect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_disconnect(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.security.smartcardio.PCSC.SCardEndTransaction(JI)V")]
    async fn test_s_card_end_transaction() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_end_transaction(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.security.smartcardio.PCSC.SCardEstablishContext(I)J")]
    async fn test_s_card_establish_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_establish_context(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.security.smartcardio.PCSC.SCardGetStatusChange(JJ[I[Ljava/lang/String;)[I"
    )]
    async fn test_s_card_get_status_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_get_status_change(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.security.smartcardio.PCSC.SCardListReaders(J)[Ljava/lang/String;"
    )]
    async fn test_s_card_list_readers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_list_readers(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.security.smartcardio.PCSC.SCardStatus(J[B)[B")]
    async fn test_s_card_status() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_status(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.security.smartcardio.PCSC.SCardTransmit(JI[BII)[B")]
    async fn test_s_card_transmit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = s_card_transmit(thread, Arguments::default()).await;
    }
}
