use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

/// Access mode ordinals from java.lang.invoke.VarHandle.AccessMode
#[derive(Debug, Clone, Copy)]
#[repr(i32)]
enum AccessMode {
    Get = 0,
    Set = 1,
    GetVolatile = 2,
    SetVolatile = 3,
    GetAcquire = 4,
    SetRelease = 5,
    GetOpaque = 6,
    SetOpaque = 7,
    CompareAndSet = 8,
    CompareAndExchange = 9,
    CompareAndExchangeAcquire = 10,
    CompareAndExchangeRelease = 11,
    WeakCompareAndSetPlain = 12,
    WeakCompareAndSet = 13,
    WeakCompareAndSetAcquire = 14,
    WeakCompareAndSetRelease = 15,
    GetAndSet = 16,
    GetAndSetAcquire = 17,
    GetAndSetRelease = 18,
    GetAndAdd = 19,
    GetAndAddAcquire = 20,
    GetAndAddRelease = 21,
    GetAndBitwiseOr = 22,
    GetAndBitwiseOrRelease = 23,
    GetAndBitwiseOrAcquire = 24,
    GetAndBitwiseAnd = 25,
    GetAndBitwiseAndRelease = 26,
    GetAndBitwiseAndAcquire = 27,
    GetAndBitwiseXor = 28,
    GetAndBitwiseXorRelease = 29,
    GetAndBitwiseXorAcquire = 30,
}

/// Invokes a `VarHandle` access mode by getting the `MethodHandle` for that mode and invoking it.
///
/// Per JVM spec, `VarHandle` polymorphic methods dispatch to `MethodHandle`s stored in the
/// `VarHandle`'s `methodHandleTable`.
#[async_recursion(?Send)]
async fn invoke_var_handle_access_mode(
    thread: &Arc<Thread>,
    parameters: Parameters,
    access_mode: AccessMode,
) -> Result<Option<Value>> {
    // Get all parameters as a vector
    let all_params = parameters.into_vec();
    if all_params.is_empty() {
        return Err(crate::Error::InternalError(
            "VarHandle access mode requires at least the VarHandle".to_string(),
        ));
    }

    // First parameter is the VarHandle itself (this)
    let var_handle = all_params[0].clone();
    let remaining_args: Vec<Value> = all_params[1..].to_vec();

    // Get the AccessMode enum value
    let access_mode_class = thread
        .class("java/lang/invoke/VarHandle$AccessMode")
        .await?;
    let values_method =
        access_mode_class.try_get_method("values", "()[Ljava/lang/invoke/VarHandle$AccessMode;")?;
    let values_result = thread
        .execute(&access_mode_class, &values_method, &[] as &[Value])
        .await?;
    let values_array = values_result.ok_or_else(|| {
        crate::Error::InternalError("AccessMode.values() returned null".to_string())
    })?;

    // Get the AccessMode enum constant at the ordinal
    let access_mode_value = {
        let values_ref = values_array.as_reference()?;
        let (_, elements) = values_ref.as_class_vec_ref()?;
        let ordinal = access_mode as usize;
        if ordinal >= elements.len() {
            return Err(crate::Error::InternalError(format!(
                "AccessMode ordinal {ordinal} out of bounds"
            )));
        }
        elements[ordinal].clone()
    };

    // Call toMethodHandle(AccessMode) to get a MethodHandle that's bound to this VarHandle
    let var_handle_class = thread.class("java/lang/invoke/VarHandle").await?;
    let to_method_handle = var_handle_class.try_get_method(
        "toMethodHandle",
        "(Ljava/lang/invoke/VarHandle$AccessMode;)Ljava/lang/invoke/MethodHandle;",
    )?;

    let mh_result = thread
        .execute(
            &var_handle_class,
            &to_method_handle,
            &[var_handle, access_mode_value],
        )
        .await?;

    let method_handle = mh_result.ok_or_else(|| {
        crate::Error::InternalError("VarHandle.toMethodHandle returned null".to_string())
    })?;

    // Build the arguments for the MethodHandle invoke:
    // [method_handle, ...remaining_args]
    // The MethodHandle from toMethodHandle is already bound to the VarHandle
    let mut invoke_args = vec![method_handle];
    invoke_args.extend(remaining_args);

    // Invoke the MethodHandle using the same mechanism as MethodHandle.invoke
    // This calls the invoke intrinsic method which handles LambdaForm dispatch
    let invoke_params = Parameters::new(invoke_args);
    super::methodhandle::invoke(thread.clone(), invoke_params).await
}

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
pub(crate) async fn set(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::Set).await
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
    async fn test_set_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set(thread, Parameters::default()).await;
        // With no arguments, set should return an error
        assert!(result.is_err());
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
