use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_21: Version = Version::Java21 { minor: 0 };
const JAVA_23: Version = Version::Java23 { minor: 0 };

/// Register all native methods for `jdk.internal.foreign.abi.fallback.LibFallback`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/foreign/abi/fallback/LibFallback";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_21 {
        registry.register(class_name, "doDowncall", "(JJJJJI)V", do_downcall);
    } else {
        registry.register(
            class_name,
            "doDowncall",
            "(JJJJJI[Ljava/lang/Object;I)V",
            do_downcall,
        );
        registry.register(class_name, "ffi_sizeof_int", "()I", ffi_sizeof_int);
        registry.register(class_name, "ffi_sizeof_long", "()I", ffi_sizeof_long);
        registry.register(class_name, "ffi_sizeof_short", "()I", ffi_sizeof_short);
        registry.register(class_name, "ffi_sizeof_wchar", "()I", ffi_sizeof_wchar);
    }

    if java_version >= JAVA_23 {
        registry.register(class_name, "alignof_double", "()I", alignof_double);
        registry.register(class_name, "alignof_long_long", "()I", alignof_long_long);
    }

    registry.register(
        class_name,
        "createClosure",
        "(JLjava/lang/Object;[J)I",
        create_closure,
    );
    registry.register(class_name, "ffi_default_abi", "()I", ffi_default_abi);
    registry.register(
        class_name,
        "ffi_get_struct_offsets",
        "(IJJ)I",
        ffi_get_struct_offsets,
    );
    registry.register(class_name, "ffi_prep_cif", "(JIIJJ)I", ffi_prep_cif);
    registry.register(
        class_name,
        "ffi_prep_cif_var",
        "(JIIIJJ)I",
        ffi_prep_cif_var,
    );
    registry.register(class_name, "ffi_type_double", "()J", ffi_type_double);
    registry.register(class_name, "ffi_type_float", "()J", ffi_type_float);
    registry.register(class_name, "ffi_type_pointer", "()J", ffi_type_pointer);
    registry.register(class_name, "ffi_type_sint16", "()J", ffi_type_sint_16);
    registry.register(class_name, "ffi_type_sint32", "()J", ffi_type_sint_32);
    registry.register(class_name, "ffi_type_sint64", "()J", ffi_type_sint_64);
    registry.register(class_name, "ffi_type_sint8", "()J", ffi_type_sint_8);
    registry.register(class_name, "ffi_type_struct", "()S", ffi_type_struct);
    registry.register(class_name, "ffi_type_uint16", "()J", ffi_type_uint_16);
    registry.register(class_name, "ffi_type_uint32", "()J", ffi_type_uint_32);
    registry.register(class_name, "ffi_type_uint64", "()J", ffi_type_uint_64);
    registry.register(class_name, "ffi_type_uint8", "()J", ffi_type_uint_8);
    registry.register(class_name, "ffi_type_void", "()J", ffi_type_void);
    registry.register(class_name, "freeClosure", "(JJ)V", free_closure);
    registry.register(class_name, "init", "()Z", init);
    registry.register(class_name, "sizeofCif", "()J", sizeof_cif);
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
