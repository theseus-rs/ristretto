use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/invoke/VarHandle";

/// Register all native methods for `java.lang.invoke.VarHandle`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "compareAndExchange",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        compare_and_exchange,
    );
    registry.register(
        CLASS_NAME,
        "compareAndExchangeAcquire",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        compare_and_exchange_acquire,
    );
    registry.register(
        CLASS_NAME,
        "compareAndExchangeRelease",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        compare_and_exchange_release,
    );
    registry.register(
        CLASS_NAME,
        "compareAndSet",
        "([Ljava/lang/Object;)Z",
        compare_and_set,
    );
    registry.register(
        CLASS_NAME,
        "get",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get,
    );
    registry.register(
        CLASS_NAME,
        "getAcquire",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_acquire,
    );
    registry.register(
        CLASS_NAME,
        "getAndAdd",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_add,
    );
    registry.register(
        CLASS_NAME,
        "getAndAddAcquire",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_add_acquire,
    );
    registry.register(
        CLASS_NAME,
        "getAndAddRelease",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_add_release,
    );
    registry.register(
        CLASS_NAME,
        "getAndBitwiseAnd",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_and,
    );
    registry.register(
        CLASS_NAME,
        "getAndBitwiseAndAcquire",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_and_acquire,
    );
    registry.register(
        CLASS_NAME,
        "getAndBitwiseAndRelease",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_and_release,
    );
    registry.register(
        CLASS_NAME,
        "getAndBitwiseOr",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_or,
    );
    registry.register(
        CLASS_NAME,
        "getAndBitwiseOrAcquire",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_or_acquire,
    );
    registry.register(
        CLASS_NAME,
        "getAndBitwiseOrRelease",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_or_release,
    );
    registry.register(
        CLASS_NAME,
        "getAndBitwiseXor",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_xor,
    );
    registry.register(
        CLASS_NAME,
        "getAndBitwiseXorAcquire",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_xor_acquire,
    );
    registry.register(
        CLASS_NAME,
        "getAndBitwiseXorRelease",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_bitwise_xor_release,
    );
    registry.register(
        CLASS_NAME,
        "getAndSet",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_set,
    );
    registry.register(
        CLASS_NAME,
        "getAndSetAcquire",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_set_acquire,
    );
    registry.register(
        CLASS_NAME,
        "getAndSetRelease",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_and_set_release,
    );
    registry.register(
        CLASS_NAME,
        "getOpaque",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_opaque,
    );
    registry.register(
        CLASS_NAME,
        "getVolatile",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        get_volatile,
    );
    registry.register(CLASS_NAME, "set", "([Ljava/lang/Object;)V", set);
    registry.register(
        CLASS_NAME,
        "setOpaque",
        "([Ljava/lang/Object;)V",
        set_opaque,
    );
    registry.register(
        CLASS_NAME,
        "setRelease",
        "([Ljava/lang/Object;)V",
        set_release,
    );
    registry.register(
        CLASS_NAME,
        "setVolatile",
        "([Ljava/lang/Object;)V",
        set_volatile,
    );
    registry.register(
        CLASS_NAME,
        "weakCompareAndSet",
        "([Ljava/lang/Object;)Z",
        weak_compare_and_set,
    );
    registry.register(
        CLASS_NAME,
        "weakCompareAndSetAcquire",
        "([Ljava/lang/Object;)Z",
        weak_compare_and_set_acquire,
    );
    registry.register(
        CLASS_NAME,
        "weakCompareAndSetPlain",
        "([Ljava/lang/Object;)Z",
        weak_compare_and_set_plain,
    );
    registry.register(
        CLASS_NAME,
        "weakCompareAndSetRelease",
        "([Ljava/lang/Object;)Z",
        weak_compare_and_set_release,
    );
}

#[async_recursion(?Send)]
async fn compare_and_exchange(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.compareAndExchange([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn compare_and_exchange_acquire(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.compareAndExchangeAcquire([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn compare_and_exchange_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.compareAndExchangeRelease([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn compare_and_set(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.compareAndSet([Ljava/lang/Object;)Z")
}

#[async_recursion(?Send)]
async fn get(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.get([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_acquire(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAcquire([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_add(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndAdd([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_add_acquire(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndAddAcquire([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_add_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndAddRelease([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_bitwise_and(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndBitwiseAnd([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_bitwise_and_acquire(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseAndAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_and_bitwise_and_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseAndRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_and_bitwise_or(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndBitwiseOr([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_bitwise_or_acquire(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseOrAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_and_bitwise_or_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseOrRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_and_bitwise_xor(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndBitwiseXor([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_bitwise_xor_acquire(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseXorAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_and_bitwise_xor_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseXorRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_and_set(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndSet([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_set_acquire(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndSetAcquire([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_set_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndSetRelease([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_opaque(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getOpaque([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_volatile(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getVolatile([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn set(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.set([Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn set_opaque(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.setOpaque([Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn set_release(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.setRelease([Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn set_volatile(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.setVolatile([Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn weak_compare_and_set(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.weakCompareAndSet([Ljava/lang/Object;)Z")
}

#[async_recursion(?Send)]
async fn weak_compare_and_set_acquire(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.weakCompareAndSetAcquire([Ljava/lang/Object;)Z")
}

#[async_recursion(?Send)]
async fn weak_compare_and_set_plain(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.weakCompareAndSetPlain([Ljava/lang/Object;)Z")
}

#[async_recursion(?Send)]
async fn weak_compare_and_set_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.weakCompareAndSetRelease([Ljava/lang/Object;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.compareAndExchange([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_compare_and_exchange() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_exchange(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.compareAndExchangeAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_compare_and_exchange_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_exchange_acquire(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.compareAndExchangeRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_compare_and_exchange_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_exchange_release(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.compareAndSet([Ljava/lang/Object;)Z"
    )]
    async fn test_compare_and_set() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_set(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.get([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_acquire(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndAdd([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_add() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_add(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndAddAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_add_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_add_acquire(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndAddRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_add_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_add_release(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseAnd([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_and() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_and(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseAndAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_and_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_and_acquire(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseAndRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_and_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_and_release(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseOr([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_or() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_or(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseOrAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_or_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_or_acquire(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseOrRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_or_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_or_release(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseXor([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_xor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_xor(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseXorAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_xor_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_xor_acquire(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseXorRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_xor_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_xor_release(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndSet([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_set() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_set(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndSetAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_set_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_set_acquire(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndSetRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_set_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_set_release(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getOpaque([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_opaque() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_opaque(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getVolatile([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_volatile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_volatile(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.set([Ljava/lang/Object;)V"
    )]
    async fn test_set() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.setOpaque([Ljava/lang/Object;)V"
    )]
    async fn test_set_opaque() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_opaque(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.setRelease([Ljava/lang/Object;)V"
    )]
    async fn test_set_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_release(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.setVolatile([Ljava/lang/Object;)V"
    )]
    async fn test_set_volatile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_volatile(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.weakCompareAndSet([Ljava/lang/Object;)Z"
    )]
    async fn test_weak_compare_and_set() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = weak_compare_and_set(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.weakCompareAndSetAcquire([Ljava/lang/Object;)Z"
    )]
    async fn test_weak_compare_and_set_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = weak_compare_and_set_acquire(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.weakCompareAndSetPlain([Ljava/lang/Object;)Z"
    )]
    async fn test_weak_compare_and_set_plain() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = weak_compare_and_set_plain(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.weakCompareAndSetRelease([Ljava/lang/Object;)Z"
    )]
    async fn test_weak_compare_and_set_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = weak_compare_and_set_release(thread, Parameters::default()).await;
    }
}
