use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/invoke/VarHandle.compareAndExchange([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.compareAndExchange([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.compareAndExchangeAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_acquire(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.compareAndExchangeAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.compareAndExchangeRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.compareAndExchangeRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.compareAndSet([Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_set(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.compareAndSet([Ljava/lang/Object;)Z")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.get([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.get([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_acquire(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAcquire([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndAdd([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_add(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndAdd([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndAddAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_add_acquire(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndAddAcquire([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndAddRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_add_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndAddRelease([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseAnd([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_bitwise_and(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndBitwiseAnd([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseAndAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_bitwise_and_acquire(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseAndAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseAndRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_bitwise_and_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseAndRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseOr([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_bitwise_or(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndBitwiseOr([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseOrAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_bitwise_or_acquire(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseOrAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseOrRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_bitwise_or_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseOrRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseXor([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_bitwise_xor(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndBitwiseXor([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseXorAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_bitwise_xor_acquire(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseXorAcquire([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseXorRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_bitwise_xor_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.VarHandle.getAndBitwiseXorRelease([Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndSet([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_set(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndSet([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndSetAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_set_acquire(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndSetAcquire([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndSetRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_set_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getAndSetRelease([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getOpaque([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_opaque(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getOpaque([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getVolatile([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_volatile(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.getVolatile([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.set([Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn set(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.set([Ljava/lang/Object;)V")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.setOpaque([Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_opaque(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.setOpaque([Ljava/lang/Object;)V")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.setRelease([Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.setRelease([Ljava/lang/Object;)V")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.setVolatile([Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_volatile(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.setVolatile([Ljava/lang/Object;)V")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.weakCompareAndSet([Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn weak_compare_and_set(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.weakCompareAndSet([Ljava/lang/Object;)Z")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.weakCompareAndSetAcquire([Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn weak_compare_and_set_acquire(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.weakCompareAndSetAcquire([Ljava/lang/Object;)Z")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.weakCompareAndSetPlain([Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn weak_compare_and_set_plain(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.VarHandle.weakCompareAndSetPlain([Ljava/lang/Object;)Z")
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.weakCompareAndSetRelease([Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn weak_compare_and_set_release(
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
