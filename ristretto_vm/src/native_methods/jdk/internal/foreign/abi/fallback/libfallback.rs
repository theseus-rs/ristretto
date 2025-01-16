use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_21, JAVA_23};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/foreign/abi/fallback/LibFallback";

/// Register all native methods for `jdk.internal.foreign.abi.fallback.LibFallback`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_21 {
        registry.register(CLASS_NAME, "doDowncall", "(JJJJJI)V", do_downcall);
    } else {
        registry.register(
            CLASS_NAME,
            "doDowncall",
            "(JJJJJI[Ljava/lang/Object;I)V",
            do_downcall,
        );
        registry.register(CLASS_NAME, "ffi_sizeof_int", "()I", ffi_sizeof_int);
        registry.register(CLASS_NAME, "ffi_sizeof_long", "()I", ffi_sizeof_long);
        registry.register(CLASS_NAME, "ffi_sizeof_short", "()I", ffi_sizeof_short);
        registry.register(CLASS_NAME, "ffi_sizeof_wchar", "()I", ffi_sizeof_wchar);
    }

    if registry.java_major_version() >= JAVA_23 {
        registry.register(CLASS_NAME, "alignof_double", "()I", alignof_double);
        registry.register(CLASS_NAME, "alignof_long_long", "()I", alignof_long_long);
    }

    registry.register(
        CLASS_NAME,
        "createClosure",
        "(JLjava/lang/Object;[J)I",
        create_closure,
    );
    registry.register(CLASS_NAME, "ffi_default_abi", "()I", ffi_default_abi);
    registry.register(
        CLASS_NAME,
        "ffi_get_struct_offsets",
        "(IJJ)I",
        ffi_get_struct_offsets,
    );
    registry.register(CLASS_NAME, "ffi_prep_cif", "(JIIJJ)I", ffi_prep_cif);
    registry.register(
        CLASS_NAME,
        "ffi_prep_cif_var",
        "(JIIIJJ)I",
        ffi_prep_cif_var,
    );
    registry.register(CLASS_NAME, "ffi_type_double", "()J", ffi_type_double);
    registry.register(CLASS_NAME, "ffi_type_float", "()J", ffi_type_float);
    registry.register(CLASS_NAME, "ffi_type_pointer", "()J", ffi_type_pointer);
    registry.register(CLASS_NAME, "ffi_type_sint16", "()J", ffi_type_sint_16);
    registry.register(CLASS_NAME, "ffi_type_sint32", "()J", ffi_type_sint_32);
    registry.register(CLASS_NAME, "ffi_type_sint64", "()J", ffi_type_sint_64);
    registry.register(CLASS_NAME, "ffi_type_sint8", "()J", ffi_type_sint_8);
    registry.register(CLASS_NAME, "ffi_type_struct", "()S", ffi_type_struct);
    registry.register(CLASS_NAME, "ffi_type_uint16", "()J", ffi_type_uint_16);
    registry.register(CLASS_NAME, "ffi_type_uint32", "()J", ffi_type_uint_32);
    registry.register(CLASS_NAME, "ffi_type_uint64", "()J", ffi_type_uint_64);
    registry.register(CLASS_NAME, "ffi_type_uint8", "()J", ffi_type_uint_8);
    registry.register(CLASS_NAME, "ffi_type_void", "()J", ffi_type_void);
    registry.register(CLASS_NAME, "freeClosure", "(JJ)V", free_closure);
    registry.register(CLASS_NAME, "init", "()Z", init);
    registry.register(CLASS_NAME, "sizeofCif", "()J", sizeof_cif);
}

#[async_recursion(?Send)]
async fn alignof_double(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.alignof_double()I")
}

#[async_recursion(?Send)]
async fn alignof_long_long(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.alignof_long_long()I")
}

#[async_recursion(?Send)]
async fn create_closure(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.createClosure(JLjava/lang/Object;[J)I")
}

#[async_recursion(?Send)]
async fn do_downcall(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.doDowncall(JJJJJ[I)V")
}

#[async_recursion(?Send)]
async fn ffi_default_abi(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_default_abi()I")
}

#[async_recursion(?Send)]
async fn ffi_get_struct_offsets(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_get_struct_offsets(IJJ)I")
}

#[async_recursion(?Send)]
async fn ffi_prep_cif(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_prep_cif(JIIJJ)I")
}

#[async_recursion(?Send)]
async fn ffi_prep_cif_var(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_prep_cif_var(JIIIJJ)I")
}

#[async_recursion(?Send)]
async fn ffi_sizeof_int(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_int()I")
}

#[async_recursion(?Send)]
async fn ffi_sizeof_long(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_long()I")
}

#[async_recursion(?Send)]
async fn ffi_sizeof_short(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_short()I")
}

#[async_recursion(?Send)]
async fn ffi_sizeof_wchar(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_wchar()I")
}

#[async_recursion(?Send)]
async fn ffi_type_double(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_double()J")
}

#[async_recursion(?Send)]
async fn ffi_type_float(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_float()J")
}

#[async_recursion(?Send)]
async fn ffi_type_pointer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_pointer()J")
}

#[async_recursion(?Send)]
async fn ffi_type_sint_16(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint16()J")
}

#[async_recursion(?Send)]
async fn ffi_type_sint_32(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint32()J")
}

#[async_recursion(?Send)]
async fn ffi_type_sint_64(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint64()J")
}

#[async_recursion(?Send)]
async fn ffi_type_sint_8(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint8()J")
}

#[async_recursion(?Send)]
async fn ffi_type_struct(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_struct()S")
}

#[async_recursion(?Send)]
async fn ffi_type_uint_16(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint16()J")
}

#[async_recursion(?Send)]
async fn ffi_type_uint_32(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint32()J")
}

#[async_recursion(?Send)]
async fn ffi_type_uint_64(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint64()J")
}

#[async_recursion(?Send)]
async fn ffi_type_uint_8(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint8()J")
}

#[async_recursion(?Send)]
async fn ffi_type_void(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_void()J")
}

#[async_recursion(?Send)]
async fn free_closure(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.freeClosure(JJ)V")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.fallback.LibFallback.init()Z")
}

#[async_recursion(?Send)]
async fn sizeof_cif(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
        let _ = alignof_double(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.alignof_long_long()I"
    )]
    async fn test_alignof_long_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = alignof_long_long(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.createClosure(JLjava/lang/Object;[J)I"
    )]
    async fn test_create_closure() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_closure(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.doDowncall(JJJJJ[I)V"
    )]
    async fn test_do_downcall() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_downcall(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_default_abi()I"
    )]
    async fn test_ffi_default_abi() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_default_abi(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_get_struct_offsets(IJJ)I"
    )]
    async fn test_ffi_get_struct_offsets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_get_struct_offsets(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_prep_cif(JIIJJ)I"
    )]
    async fn test_ffi_prep_cif() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_prep_cif(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_prep_cif_var(JIIIJJ)I"
    )]
    async fn test_ffi_prep_cif_var() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_prep_cif_var(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_int()I"
    )]
    async fn test_ffi_sizeof_int() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_sizeof_int(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_long()I"
    )]
    async fn test_ffi_sizeof_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_sizeof_long(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_short()I"
    )]
    async fn test_ffi_sizeof_short() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_sizeof_short(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_sizeof_wchar()I"
    )]
    async fn test_ffi_sizeof_wchar() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_sizeof_wchar(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_double()J"
    )]
    async fn test_ffi_type_double() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_double(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_float()J"
    )]
    async fn test_ffi_type_float() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_float(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_pointer()J"
    )]
    async fn test_ffi_type_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_pointer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint16()J"
    )]
    async fn test_ffi_type_sint16() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_sint_16(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint32()J"
    )]
    async fn test_ffi_type_sint32() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_sint_32(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint64()J"
    )]
    async fn test_ffi_type_sint64() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_sint_64(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_sint8()J"
    )]
    async fn test_ffi_type_sint8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_sint_8(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_struct()S"
    )]
    async fn test_ffi_type_struct() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_struct(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint16()J"
    )]
    async fn test_ffi_type_uint16() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_uint_16(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint32()J"
    )]
    async fn test_ffi_type_uint32() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_uint_32(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint64()J"
    )]
    async fn test_ffi_type_uint64() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_uint_64(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_uint8()J"
    )]
    async fn test_ffi_type_uint8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_uint_8(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.ffi_type_void()J"
    )]
    async fn test_ffi_type_void() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ffi_type_void(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.freeClosure(JJ)V"
    )]
    async fn test_free_closure() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_closure(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.init()Z"
    )]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.fallback.LibFallback.sizeofCif()J"
    )]
    async fn test_sizeof_cif() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sizeof_cif(thread, Arguments::default()).await;
    }
}
