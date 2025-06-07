use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{GreaterThan, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.alignof_double()I",
    GreaterThan(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn alignof_double(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.alignof_double()I")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.alignof_long_long()I",
    GreaterThan(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn alignof_long_long(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.alignof_long_long()I")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.createClosure(JLjava/lang/Object;[J)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_closure(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.createClosure(JLjava/lang/Object;[J)I")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.doDowncall(JJJJJI)V",
    LessThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn do_downcall_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.doDowncall(JJJJJI)V")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.doDowncall(JJJJLjava/lang/Object;JI[Ljava/lang/Object;I)V",
    GreaterThan(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn do_downcall_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.foreign.abi.fallback.LibFallback.doDowncall(JJJJLjava/lang/Object;JI[Ljava/lang/Object;I)V"
    )
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_default_abi()I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_default_abi(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_default_abi()I")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_get_struct_offsets(IJJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_get_struct_offsets(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_get_struct_offsets(IJJ)I")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_prep_cif(JIIJJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_prep_cif(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_prep_cif(JIIJJ)I")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_prep_cif_var(JIIIJJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_prep_cif_var(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_prep_cif_var(JIIIJJ)I")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_sizeof_int()I",
    GreaterThan(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_sizeof_int(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_int()I")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_sizeof_long()I",
    GreaterThan(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_sizeof_long(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_long()I")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_sizeof_short()I",
    GreaterThan(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_sizeof_short(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_short()I")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_sizeof_wchar()I",
    GreaterThan(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_sizeof_wchar(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_wchar()I")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_double()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_type_double(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_double()J")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_float()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_type_float(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_float()J")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_pointer()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_type_pointer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_pointer()J")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_sint16()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_type_sint_16(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint16()J")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_sint32()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_type_sint_32(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint32()J")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_sint64()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_type_sint_64(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint64()J")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_sint8()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_type_sint_8(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint8()J")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_struct()S",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_type_struct(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_struct()S")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_uint16()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_type_uint_16(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint16()J")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_uint32()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_type_uint_32(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint32()J")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_uint64()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_type_uint_64(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint64()J")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_uint8()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_type_uint_8(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint8()J")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.ffi_type_void()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ffi_type_void(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_void()J")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.freeClosure(JJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn free_closure(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.freeClosure(JJ)V")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.init()Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/fallback/LibFallback.sizeofCif()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn sizeof_cif(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.sizeofCif()J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.alignof_double()I"
    )]
    async fn test_alignof_double() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = alignof_double(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.alignof_long_long()I"
    )]
    async fn test_alignof_long_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = alignof_long_long(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.createClosure(JLjava/lang/Object;[J)I"
    )]
    async fn test_create_closure() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_closure(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.doDowncall(JJJJJI)V"
    )]
    async fn test_do_downcall_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_downcall_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.doDowncall(JJJJLjava/lang/Object;JI[Ljava/lang/Object;I)V"
    )]
    async fn test_do_downcall_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_downcall_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_default_abi()I"
    )]
    async fn test_ffi_default_abi() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_default_abi(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_get_struct_offsets(IJJ)I"
    )]
    async fn test_ffi_get_struct_offsets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_get_struct_offsets(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_prep_cif(JIIJJ)I"
    )]
    async fn test_ffi_prep_cif() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_prep_cif(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_prep_cif_var(JIIIJJ)I"
    )]
    async fn test_ffi_prep_cif_var() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_prep_cif_var(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_int()I"
    )]
    async fn test_ffi_sizeof_int() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_sizeof_int(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_long()I"
    )]
    async fn test_ffi_sizeof_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_sizeof_long(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_short()I"
    )]
    async fn test_ffi_sizeof_short() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_sizeof_short(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_wchar()I"
    )]
    async fn test_ffi_sizeof_wchar() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_sizeof_wchar(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_double()J"
    )]
    async fn test_ffi_type_double() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_double(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_float()J"
    )]
    async fn test_ffi_type_float() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_float(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_pointer()J"
    )]
    async fn test_ffi_type_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_pointer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint16()J"
    )]
    async fn test_ffi_type_sint16() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_sint_16(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint32()J"
    )]
    async fn test_ffi_type_sint32() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_sint_32(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint64()J"
    )]
    async fn test_ffi_type_sint64() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_sint_64(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint8()J"
    )]
    async fn test_ffi_type_sint8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_sint_8(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_struct()S"
    )]
    async fn test_ffi_type_struct() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_struct(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint16()J"
    )]
    async fn test_ffi_type_uint16() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_uint_16(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint32()J"
    )]
    async fn test_ffi_type_uint32() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_uint_32(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint64()J"
    )]
    async fn test_ffi_type_uint64() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_uint_64(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint8()J"
    )]
    async fn test_ffi_type_uint8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_uint_8(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_void()J"
    )]
    async fn test_ffi_type_void() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_void(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.freeClosure(JJ)V"
    )]
    async fn test_free_closure() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_closure(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.sizeofCif()J"
    )]
    async fn test_sizeof_cif() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sizeof_cif(thread, Parameters::default()).await;
    }
}
