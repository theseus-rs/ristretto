use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{GreaterThan, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.alignof_double()I",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub async fn alignof_double<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.alignof_double()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.alignof_long_long()I",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub async fn alignof_long_long<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.alignof_long_long()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.createClosure(JLjava/lang/Object;[J)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn create_closure<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.createClosure(JLjava/lang/Object;[J)I"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.doDowncall(JJJJJI)V",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn do_downcall_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.doDowncall(JJJJJI)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.doDowncall(JJJJLjava/lang/Object;JI[Ljava/lang/Object;I)V",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub async fn do_downcall_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.foreign.abi.fallback.LibFallback.doDowncall(JJJJLjava/lang/Object;JI[Ljava/lang/Object;I)V".to_string()).into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_default_abi()I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_default_abi<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_default_abi()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_get_struct_offsets(IJJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_get_struct_offsets<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_get_struct_offsets(IJJ)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_prep_cif(JIIJJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_prep_cif<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_prep_cif(JIIJJ)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_prep_cif_var(JIIIJJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_prep_cif_var<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_prep_cif_var(JIIIJJ)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_sizeof_int()I",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub async fn ffi_sizeof_int<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_int()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_sizeof_long()I",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub async fn ffi_sizeof_long<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_long()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_sizeof_short()I",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub async fn ffi_sizeof_short<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_short()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_sizeof_wchar()I",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub async fn ffi_sizeof_wchar<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_wchar()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_double()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_type_double<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_double()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_float()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_type_float<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_float()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_pointer()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_type_pointer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_pointer()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_sint16()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_type_sint_16<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint16()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_sint32()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_type_sint_32<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint32()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_sint64()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_type_sint_64<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint64()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_sint8()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_type_sint_8<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint8()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_struct()S",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_type_struct<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_struct()S".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_uint16()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_type_uint_16<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint16()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_uint32()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_type_uint_32<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint32()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_uint64()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_type_uint_64<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint64()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_uint8()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_type_uint_8<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint8()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_void()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ffi_type_void<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_void()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.freeClosure(JJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn free_closure<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.freeClosure(JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.init()Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.sizeofCif()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn sizeof_cif<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.fallback.LibFallback.sizeofCif()J".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_alignof_double() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = alignof_double(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_alignof_long_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = alignof_long_long(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_closure() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_closure(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_downcall_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_downcall_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_downcall_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_downcall_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_default_abi() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_default_abi(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_get_struct_offsets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_get_struct_offsets(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_prep_cif() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_prep_cif(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_prep_cif_var() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_prep_cif_var(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_sizeof_int() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_sizeof_int(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_sizeof_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_sizeof_long(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_sizeof_short() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_sizeof_short(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_sizeof_wchar() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_sizeof_wchar(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_type_double() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_type_double(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_type_float() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_type_float(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_type_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_type_pointer(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_type_sint16() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_type_sint_16(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_type_sint32() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_type_sint_32(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_type_sint64() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_type_sint_64(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_type_sint8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_type_sint_8(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_type_struct() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_type_struct(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_type_uint16() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_type_uint_16(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_type_uint32() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_type_uint_32(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_type_uint64() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_type_uint_64(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_type_uint8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_type_uint_8(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ffi_type_void() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ffi_type_void(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_free_closure() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = free_closure(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_sizeof_cif() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = sizeof_cif(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
