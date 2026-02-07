use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
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
#[async_method]
async fn invoke_var_handle_access_mode<T: ristretto_types::Thread + 'static>(
    thread: &Arc<T>,
    parameters: Parameters,
    access_mode: AccessMode,
) -> Result<Option<Value>> {
    // Get all parameters as a vector
    let all_params = parameters.into_vec();
    if all_params.is_empty() {
        return Err(ristretto_types::Error::InternalError(
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
        ristretto_types::Error::InternalError("AccessMode.values() returned null".to_string())
    })?;

    // Get the AccessMode enum constant at the ordinal
    let access_mode_value = {
        let values_ref = values_array.as_reference()?;
        let (_, elements) = values_ref.as_class_vec_ref()?;
        let ordinal = access_mode as usize;
        if ordinal >= elements.len() {
            return Err(ristretto_types::Error::InternalError(format!(
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
        ristretto_types::Error::InternalError("VarHandle.toMethodHandle returned null".to_string())
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
#[async_method]
pub async fn compare_and_exchange<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::CompareAndExchange).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.compareAndExchangeAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn compare_and_exchange_acquire<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::CompareAndExchangeAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.compareAndExchangeRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn compare_and_exchange_release<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::CompareAndExchangeRelease).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.compareAndSet([Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn compare_and_set<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::CompareAndSet).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.get([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::Get).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_acquire<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndAdd([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_add<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndAdd).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndAddAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_add_acquire<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndAddAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndAddRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_add_release<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndAddRelease).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseAnd([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_and<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseAnd).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseAndAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_and_acquire<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseAndAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseAndRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_and_release<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseAndRelease).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseOr([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_or<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseOr).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseOrAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_or_acquire<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseOrAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseOrRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_or_release<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseOrRelease).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseXor([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_xor<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseXor).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseXorAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_xor_acquire<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseXorAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseXorRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_xor_release<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseXorRelease).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndSet([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_set<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndSet).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndSetAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_set_acquire<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndSetAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndSetRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_set_release<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndSetRelease).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getOpaque([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_opaque<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetOpaque).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getVolatile([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_volatile<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetVolatile).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.set([Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::Set).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.setOpaque([Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_opaque<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::SetOpaque).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.setRelease([Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_release<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::SetRelease).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.setVolatile([Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_volatile<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::SetVolatile).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.weakCompareAndSet([Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn weak_compare_and_set<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::WeakCompareAndSet).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.weakCompareAndSetAcquire([Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn weak_compare_and_set_acquire<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::WeakCompareAndSetAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.weakCompareAndSetPlain([Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn weak_compare_and_set_plain<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::WeakCompareAndSetPlain).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.weakCompareAndSetRelease([Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn weak_compare_and_set_release<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::WeakCompareAndSetRelease).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compare_and_exchange_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = compare_and_exchange(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_compare_and_exchange_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = compare_and_exchange_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_compare_and_exchange_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = compare_and_exchange_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_compare_and_set_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = compare_and_set(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get(thread, Parameters::default()).await;
        // With no arguments, get should return an error
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_add_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_add(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_add_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_add_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_add_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_add_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_and_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_and(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_and_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_and_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_and_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_and_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_or_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_or(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_or_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_or_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_or_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_or_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_xor_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_xor(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_xor_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_xor_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_xor_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_xor_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_set_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_set(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_set_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_set_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_set_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_set_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_opaque_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_opaque(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_volatile_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_volatile(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_opaque_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_opaque(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_volatile_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_volatile(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_weak_compare_and_set_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = weak_compare_and_set(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_weak_compare_and_set_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = weak_compare_and_set_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_weak_compare_and_set_plain_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = weak_compare_and_set_plain(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_weak_compare_and_set_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = weak_compare_and_set_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
