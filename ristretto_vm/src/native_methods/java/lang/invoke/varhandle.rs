use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.invoke.VarHandle`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/invoke/VarHandle";
    registry.register(
        class_name,
        "compareAndExchange",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        compare_and_exchange,
    );
    registry.register(
        class_name,
        "compareAndExchangeAcquire",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        compare_and_exchange_acquire,
    );
    registry.register(
        class_name,
        "compareAndExchangeRelease",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        compare_and_exchange_release,
    );
    registry.register(
        class_name,
        "compareAndSet",
        "([Ljava/lang/Object;)Z",
        compare_and_set,
    );
    registry.register(
        class_name,
        "get",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get,
    );
    registry.register(
        class_name,
        "getAcquire",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_acquire,
    );
    registry.register(
        class_name,
        "getAndAdd",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_add,
    );
    registry.register(
        class_name,
        "getAndAddAcquire",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_add_acquire,
    );
    registry.register(
        class_name,
        "getAndAddRelease",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_add_release,
    );
    registry.register(
        class_name,
        "getAndBitwiseAnd",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_and,
    );
    registry.register(
        class_name,
        "getAndBitwiseAndAcquire",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_and_acquire,
    );
    registry.register(
        class_name,
        "getAndBitwiseAndRelease",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_and_release,
    );
    registry.register(
        class_name,
        "getAndBitwiseOr",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_or,
    );
    registry.register(
        class_name,
        "getAndBitwiseOrAcquire",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_or_acquire,
    );
    registry.register(
        class_name,
        "getAndBitwiseOrRelease",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_or_release,
    );
    registry.register(
        class_name,
        "getAndBitwiseXor",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_xor,
    );
    registry.register(
        class_name,
        "getAndBitwiseXorAcquire",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_xor_acquire,
    );
    registry.register(
        class_name,
        "getAndBitwiseXorRelease",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_xor_release,
    );
    registry.register(
        class_name,
        "getAndSet",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_set,
    );
    registry.register(
        class_name,
        "getAndSetAcquire",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_set_acquire,
    );
    registry.register(
        class_name,
        "getAndSetRelease",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_set_release,
    );
    registry.register(
        class_name,
        "getOpaque",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_opaque,
    );
    registry.register(
        class_name,
        "getVolatile",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_volatile,
    );
    registry.register(class_name, "set", "([Ljava/lang/Object;)V", set);
    registry.register(
        class_name,
        "setOpaque",
        "([Ljava/lang/Object;)V",
        set_opaque,
    );
    registry.register(
        class_name,
        "setRelease",
        "([Ljava/lang/Object;)V",
        set_release,
    );
    registry.register(
        class_name,
        "setVolatile",
        "([Ljava/lang/Object;)V",
        set_volatile,
    );
    registry.register(
        class_name,
        "weakCompareAndSet",
        "([Ljava/lang/Object;)Z",
        weak_compare_and_set,
    );
    registry.register(
        class_name,
        "weakCompareAndSetAcquire",
        "([Ljava/lang/Object;)Z",
        weak_compare_and_set_acquire,
    );
    registry.register(
        class_name,
        "weakCompareAndSetPlain",
        "([Ljava/lang/Object;)Z",
        weak_compare_and_set_plain,
    );
    registry.register(
        class_name,
        "weakCompareAndSetRelease",
        "([Ljava/lang/Object;)Z",
        weak_compare_and_set_release,
    );
}

#[async_recursion(?Send)]
async fn compare_and_exchange(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn compare_and_exchange_acquire(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn compare_and_exchange_release(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn compare_and_set(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_acquire(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_and_add(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_and_add_acquire(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_and_add_release(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_and_bitwise_and(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_and_bitwise_and_acquire(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_and_bitwise_and_release(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_and_bitwise_or(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_and_bitwise_or_acquire(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_and_bitwise_or_release(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_and_bitwise_xor(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_and_bitwise_xor_acquire(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_and_bitwise_xor_release(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_and_set(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_and_set_acquire(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_and_set_release(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_opaque(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_opaque(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_release(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn weak_compare_and_set(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn weak_compare_and_set_acquire(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn weak_compare_and_set_plain(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn weak_compare_and_set_release(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
