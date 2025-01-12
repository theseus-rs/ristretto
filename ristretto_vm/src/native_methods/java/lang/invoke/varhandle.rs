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
    todo!("java.lang.invoke.VarHandle.compareAndExchange([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn compare_and_exchange_acquire(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.compareAndExchangeAcquire([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn compare_and_exchange_release(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.compareAndExchangeRelease([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn compare_and_set(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.compareAndSet([Ljava/lang/Object;)Z")
}

#[async_recursion(?Send)]
async fn get(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.get([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_acquire(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAcquire([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_add(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndAdd([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_add_acquire(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndAddAcquire([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_add_release(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndAddRelease([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_bitwise_and(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndBitwiseAnd([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_bitwise_and_acquire(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseAndAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_and_bitwise_and_release(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseAndRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_and_bitwise_or(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndBitwiseOr([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_bitwise_or_acquire(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseOrAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_and_bitwise_or_release(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseOrRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_and_bitwise_xor(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndBitwiseXor([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_bitwise_xor_acquire(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseXorAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_and_bitwise_xor_release(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseXorRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_and_set(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndSet([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_set_acquire(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndSetAcquire([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_and_set_release(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndSetRelease([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_opaque(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getOpaque([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getVolatile([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn set(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.set([Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn set_opaque(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.setOpaque([Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn set_release(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.setRelease([Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn set_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.setVolatile([Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn weak_compare_and_set(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.weakCompareAndSet([Ljava/lang/Object;)Z")
}

#[async_recursion(?Send)]
async fn weak_compare_and_set_acquire(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.weakCompareAndSetAcquire([Ljava/lang/Object;)Z")
}

#[async_recursion(?Send)]
async fn weak_compare_and_set_plain(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.weakCompareAndSetPlain([Ljava/lang/Object;)Z")
}

#[async_recursion(?Send)]
async fn weak_compare_and_set_release(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.weakCompareAndSetRelease([Ljava/lang/Object;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[expect(clippy::too_many_lines)]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/invoke/VarHandle";
        assert!(registry
            .method(
                class_name,
                "compareAndExchange",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "compareAndExchangeAcquire",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "compareAndExchangeRelease",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "compareAndSet", "([Ljava/lang/Object;)Z")
            .is_some());
        assert!(registry
            .method(class_name, "get", "([Ljava/lang/Object;)Ljava/lang/Object;")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAcquire",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAndAdd",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAndAddAcquire",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAndAddRelease",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAndBitwiseAnd",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAndBitwiseAndAcquire",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAndBitwiseAndRelease",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAndBitwiseOr",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAndBitwiseOrAcquire",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAndBitwiseOrRelease",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAndBitwiseXor",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAndBitwiseXorAcquire",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAndBitwiseXorRelease",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAndSet",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAndSetAcquire",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getAndSetRelease",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getOpaque",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getVolatile",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "set", "([Ljava/lang/Object;)V")
            .is_some());
        assert!(registry
            .method(class_name, "setOpaque", "([Ljava/lang/Object;)V")
            .is_some());
        assert!(registry
            .method(class_name, "setRelease", "([Ljava/lang/Object;)V")
            .is_some());
        assert!(registry
            .method(class_name, "setVolatile", "([Ljava/lang/Object;)V")
            .is_some());
        assert!(registry
            .method(class_name, "weakCompareAndSet", "([Ljava/lang/Object;)Z")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "weakCompareAndSetAcquire",
                "([Ljava/lang/Object;)Z"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "weakCompareAndSetPlain",
                "([Ljava/lang/Object;)Z"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "weakCompareAndSetRelease",
                "([Ljava/lang/Object;)Z"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.compareAndExchange([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_compare_and_exchange() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_exchange(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.compareAndExchangeAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_compare_and_exchange_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_exchange_acquire(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.compareAndExchangeRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_compare_and_exchange_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_exchange_release(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.compareAndSet([Ljava/lang/Object;)Z"
    )]
    async fn test_compare_and_set() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_set(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.get([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_acquire(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndAdd([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_add() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_add(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndAddAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_add_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_add_acquire(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndAddRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_add_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_add_release(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseAnd([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_and() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_and(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseAndAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_and_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_and_acquire(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseAndRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_and_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_and_release(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseOr([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_or() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_or(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseOrAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_or_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_or_acquire(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseOrRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_or_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_or_release(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseXor([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_xor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_xor(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseXorAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_xor_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_xor_acquire(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndBitwiseXorRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_bitwise_xor_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_bitwise_xor_release(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndSet([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_set() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_set(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndSetAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_set_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_set_acquire(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getAndSetRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_and_set_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_set_release(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getOpaque([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_opaque() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_opaque(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.getVolatile([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_volatile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_volatile(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.set([Ljava/lang/Object;)V"
    )]
    async fn test_set() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.setOpaque([Ljava/lang/Object;)V"
    )]
    async fn test_set_opaque() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_opaque(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.setRelease([Ljava/lang/Object;)V"
    )]
    async fn test_set_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_release(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.setVolatile([Ljava/lang/Object;)V"
    )]
    async fn test_set_volatile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_volatile(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.weakCompareAndSet([Ljava/lang/Object;)Z"
    )]
    async fn test_weak_compare_and_set() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = weak_compare_and_set(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.weakCompareAndSetAcquire([Ljava/lang/Object;)Z"
    )]
    async fn test_weak_compare_and_set_acquire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = weak_compare_and_set_acquire(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.weakCompareAndSetPlain([Ljava/lang/Object;)Z"
    )]
    async fn test_weak_compare_and_set_plain() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = weak_compare_and_set_plain(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.VarHandle.weakCompareAndSetRelease([Ljava/lang/Object;)Z"
    )]
    async fn test_weak_compare_and_set_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = weak_compare_and_set_release(thread, Arguments::default()).await;
    }
}
