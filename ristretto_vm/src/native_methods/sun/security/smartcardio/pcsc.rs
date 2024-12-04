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
    todo!()
}

#[async_recursion(?Send)]
async fn s_card_connect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn s_card_control(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn s_card_disconnect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn s_card_end_transaction(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn s_card_establish_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn s_card_get_status_change(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn s_card_list_readers(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn s_card_status(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn s_card_transmit(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
