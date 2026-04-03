//! Runtime helper functions callable from JIT-compiled code.
//!
//! These `extern "C"` functions are registered as symbols with the JIT module's
//! `JITBuilder` so that Cranelift-generated code can call them directly. They bridge
//! JIT-compiled code back into the GC-managed heap, performing array allocation
//! and element access through the same `Gc<RwLock<Reference>>` path the interpreter uses.

#![expect(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "JIT FFI functions intentionally cast between JVM i32 indices and Rust usize"
)]

use crate::Thread;
use crate::VM;
use parking_lot::RwLock;
use ristretto_classloader::{Reference, Value};
use ristretto_gc::{GarbageCollector, Gc};
use std::sync::Arc;

/// Runtime context passed to JIT-compiled code as an opaque pointer.
///
/// Contains everything the runtime helper functions need to interact with the VM:
/// the garbage collector for allocations, and the VM/Thread for method resolution
/// and invocation.
///
/// This struct is stored on the stack of `jit::execute` and a pointer to it is passed
/// as the 4th parameter to JIT-compiled functions. The pointer is then forwarded to
/// runtime helpers which reconstruct the context via `GarbageCollector::from_raw_ptr`
/// style accessors.
#[repr(C)]
pub(crate) struct RuntimeContext {
    gc: *const u8,
    vm: *const u8,
    thread: *const u8,
}

impl RuntimeContext {
    /// Creates a new `RuntimeContext` from the given VM components.
    pub fn new(gc: &GarbageCollector, vm: &Arc<VM>, thread: &Arc<Thread>) -> Self {
        Self {
            gc: std::ptr::from_ref::<GarbageCollector>(gc).cast::<u8>(),
            vm: Arc::as_ptr(vm).cast::<u8>(),
            thread: Arc::as_ptr(thread).cast::<u8>(),
        }
    }

    /// Returns the context as a raw pointer suitable for passing to JIT-compiled code.
    pub fn as_ptr(&self) -> *const u8 {
        std::ptr::from_ref::<Self>(self).cast::<u8>()
    }
}

/// Returns the list of (`symbol_name`, `function_pointer`) pairs to register with the JIT module.
pub(crate) fn symbols() -> [(&'static str, *const u8); 25] {
    [
        ("jit_new_bool_array", jit_new_bool_array as *const u8),
        ("jit_new_byte_array", jit_new_byte_array as *const u8),
        ("jit_new_char_array", jit_new_char_array as *const u8),
        ("jit_new_short_array", jit_new_short_array as *const u8),
        ("jit_new_int_array", jit_new_int_array as *const u8),
        ("jit_new_long_array", jit_new_long_array as *const u8),
        ("jit_new_float_array", jit_new_float_array as *const u8),
        ("jit_new_double_array", jit_new_double_array as *const u8),
        ("jit_arraylength", jit_arraylength as *const u8),
        ("jit_baload", jit_baload as *const u8),
        ("jit_bastore", jit_bastore as *const u8),
        ("jit_caload", jit_caload as *const u8),
        ("jit_castore", jit_castore as *const u8),
        ("jit_saload", jit_saload as *const u8),
        ("jit_sastore", jit_sastore as *const u8),
        ("jit_iaload", jit_iaload as *const u8),
        ("jit_iastore", jit_iastore as *const u8),
        ("jit_laload", jit_laload as *const u8),
        ("jit_lastore", jit_lastore as *const u8),
        ("jit_faload", jit_faload as *const u8),
        ("jit_fastore", jit_fastore as *const u8),
        ("jit_daload", jit_daload as *const u8),
        ("jit_dastore", jit_dastore as *const u8),
        ("jit_aaload", jit_aaload as *const u8),
        ("jit_aastore", jit_aastore as *const u8),
    ]
}

// ---------------------------------------------------------------------------
// Array allocation helpers
// ---------------------------------------------------------------------------

/// Reconstructs a `&GarbageCollector` from the raw context pointer passed by JIT code.
///
/// The context pointer is a `*const RuntimeContext`, and the GC is extracted from
/// its first field via `GarbageCollector::from_context_struct_ptr`.
fn gc_from_context(context: *const u8) -> &'static GarbageCollector {
    GarbageCollector::from_context_struct_ptr(context)
}

/// Wraps a `Reference` in a `Gc<RwLock<Reference>>` and returns the raw pointer as i64.
///
/// The object is allocated through the GC. The `GcRootGuard` returned by `Gc::new` is
/// dropped at the end of this function, removing the root. This is safe because newly
/// allocated objects start with `marked = true` (allocation-color-black), so they survive
/// any in-flight GC cycle. By the next GC cycle, the pointer will either be reachable from
/// a rooted object (e.g., stored in a field or on the operand stack traced from a root)
/// or it is truly garbage and should be collected.
fn alloc_reference(gc: &GarbageCollector, reference: Reference) -> i64 {
    let guard = Gc::new(gc, RwLock::new(reference));
    let gc_ref = guard.clone_gc();
    gc_ref.as_ptr_i64()
}

/// Reconstructs a `Gc<RwLock<Reference>>` from a raw i64 pointer.
fn gc_ref_from_ptr(ptr: i64) -> Gc<RwLock<Reference>> {
    Gc::from_raw_i64(ptr)
}

extern "C" fn jit_new_bool_array(context: *const u8, count: i32) -> i64 {
    let gc = gc_from_context(context);
    let count = count.max(0) as usize;
    alloc_reference(
        gc,
        Reference::BooleanArray(vec![0i8; count].into_boxed_slice()),
    )
}

extern "C" fn jit_new_byte_array(context: *const u8, count: i32) -> i64 {
    let gc = gc_from_context(context);
    let count = count.max(0) as usize;
    alloc_reference(
        gc,
        Reference::ByteArray(vec![0i8; count].into_boxed_slice()),
    )
}

extern "C" fn jit_new_char_array(context: *const u8, count: i32) -> i64 {
    let gc = gc_from_context(context);
    let count = count.max(0) as usize;
    alloc_reference(
        gc,
        Reference::CharArray(vec![0u16; count].into_boxed_slice()),
    )
}

extern "C" fn jit_new_short_array(context: *const u8, count: i32) -> i64 {
    let gc = gc_from_context(context);
    let count = count.max(0) as usize;
    alloc_reference(
        gc,
        Reference::ShortArray(vec![0i16; count].into_boxed_slice()),
    )
}

extern "C" fn jit_new_int_array(context: *const u8, count: i32) -> i64 {
    let gc = gc_from_context(context);
    let count = count.max(0) as usize;
    alloc_reference(
        gc,
        Reference::IntArray(vec![0i32; count].into_boxed_slice()),
    )
}

extern "C" fn jit_new_long_array(context: *const u8, count: i32) -> i64 {
    let gc = gc_from_context(context);
    let count = count.max(0) as usize;
    alloc_reference(
        gc,
        Reference::LongArray(vec![0i64; count].into_boxed_slice()),
    )
}

extern "C" fn jit_new_float_array(context: *const u8, count: i32) -> i64 {
    let gc = gc_from_context(context);
    let count = count.max(0) as usize;
    alloc_reference(
        gc,
        Reference::FloatArray(vec![0.0f32; count].into_boxed_slice()),
    )
}

extern "C" fn jit_new_double_array(context: *const u8, count: i32) -> i64 {
    let gc = gc_from_context(context);
    let count = count.max(0) as usize;
    alloc_reference(
        gc,
        Reference::DoubleArray(vec![0.0f64; count].into_boxed_slice()),
    )
}

// ---------------------------------------------------------------------------
// Array length
// ---------------------------------------------------------------------------

extern "C" fn jit_arraylength(array_ptr: i64) -> i32 {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let reference = gc_ref.read();
    let len = match &*reference {
        Reference::BooleanArray(a) | Reference::ByteArray(a) => a.len(),
        Reference::CharArray(a) => a.len(),
        Reference::ShortArray(a) => a.len(),
        Reference::IntArray(a) => a.len(),
        Reference::LongArray(a) => a.len(),
        Reference::FloatArray(a) => a.len(),
        Reference::DoubleArray(a) => a.len(),
        Reference::Array(a) => a.elements.len(),
        Reference::Object(_) => panic!("arraylength called on non-array reference"),
    };
    len as i32
}

// ---------------------------------------------------------------------------
// Byte/Boolean array access
// ---------------------------------------------------------------------------

extern "C" fn jit_baload(array_ptr: i64, index: i32) -> i32 {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let reference = gc_ref.read();
    match &*reference {
        Reference::ByteArray(a) | Reference::BooleanArray(a) => i32::from(a[index as usize]),
        _ => panic!("baload: expected byte/boolean array"),
    }
}

extern "C" fn jit_bastore(array_ptr: i64, index: i32, value: i32) {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let mut reference = gc_ref.write();
    match &mut *reference {
        Reference::ByteArray(a) | Reference::BooleanArray(a) => {
            a[index as usize] = value as i8;
        }
        _ => panic!("bastore: expected byte/boolean array"),
    }
}

// ---------------------------------------------------------------------------
// Char array access
// ---------------------------------------------------------------------------

extern "C" fn jit_caload(array_ptr: i64, index: i32) -> i32 {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let reference = gc_ref.read();
    match &*reference {
        Reference::CharArray(a) => i32::from(a[index as usize]),
        _ => panic!("caload: expected char array"),
    }
}

extern "C" fn jit_castore(array_ptr: i64, index: i32, value: i32) {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let mut reference = gc_ref.write();
    match &mut *reference {
        Reference::CharArray(a) => {
            a[index as usize] = value as u16;
        }
        _ => panic!("castore: expected char array"),
    }
}

// ---------------------------------------------------------------------------
// Short array access
// ---------------------------------------------------------------------------

extern "C" fn jit_saload(array_ptr: i64, index: i32) -> i32 {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let reference = gc_ref.read();
    match &*reference {
        Reference::ShortArray(a) => i32::from(a[index as usize]),
        _ => panic!("saload: expected short array"),
    }
}

extern "C" fn jit_sastore(array_ptr: i64, index: i32, value: i32) {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let mut reference = gc_ref.write();
    match &mut *reference {
        Reference::ShortArray(a) => {
            a[index as usize] = value as i16;
        }
        _ => panic!("sastore: expected short array"),
    }
}

// ---------------------------------------------------------------------------
// Int array access
// ---------------------------------------------------------------------------

extern "C" fn jit_iaload(array_ptr: i64, index: i32) -> i32 {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let reference = gc_ref.read();
    match &*reference {
        Reference::IntArray(a) => a[index as usize],
        _ => panic!("iaload: expected int array"),
    }
}

extern "C" fn jit_iastore(array_ptr: i64, index: i32, value: i32) {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let mut reference = gc_ref.write();
    match &mut *reference {
        Reference::IntArray(a) => {
            a[index as usize] = value;
        }
        _ => panic!("iastore: expected int array"),
    }
}

// ---------------------------------------------------------------------------
// Long array access
// ---------------------------------------------------------------------------

extern "C" fn jit_laload(array_ptr: i64, index: i32) -> i64 {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let reference = gc_ref.read();
    match &*reference {
        Reference::LongArray(a) => a[index as usize],
        _ => panic!("laload: expected long array"),
    }
}

extern "C" fn jit_lastore(array_ptr: i64, index: i32, value: i64) {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let mut reference = gc_ref.write();
    match &mut *reference {
        Reference::LongArray(a) => {
            a[index as usize] = value;
        }
        _ => panic!("lastore: expected long array"),
    }
}

// ---------------------------------------------------------------------------
// Float array access
// ---------------------------------------------------------------------------

extern "C" fn jit_faload(array_ptr: i64, index: i32) -> f32 {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let reference = gc_ref.read();
    match &*reference {
        Reference::FloatArray(a) => a[index as usize],
        _ => panic!("faload: expected float array"),
    }
}

extern "C" fn jit_fastore(array_ptr: i64, index: i32, value: f32) {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let mut reference = gc_ref.write();
    match &mut *reference {
        Reference::FloatArray(a) => {
            a[index as usize] = value;
        }
        _ => panic!("fastore: expected float array"),
    }
}

// ---------------------------------------------------------------------------
// Double array access
// ---------------------------------------------------------------------------

extern "C" fn jit_daload(array_ptr: i64, index: i32) -> f64 {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let reference = gc_ref.read();
    match &*reference {
        Reference::DoubleArray(a) => a[index as usize],
        _ => panic!("daload: expected double array"),
    }
}

extern "C" fn jit_dastore(array_ptr: i64, index: i32, value: f64) {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let mut reference = gc_ref.write();
    match &mut *reference {
        Reference::DoubleArray(a) => {
            a[index as usize] = value;
        }
        _ => panic!("dastore: expected double array"),
    }
}

// ---------------------------------------------------------------------------
// Reference array access
// ---------------------------------------------------------------------------

extern "C" fn jit_aaload(array_ptr: i64, index: i32) -> i64 {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let reference = gc_ref.read();
    match &*reference {
        Reference::Array(obj_array) => {
            let element = &obj_array.elements[index as usize];
            match element {
                Value::Object(None) => 0i64,
                Value::Object(Some(gc)) => gc.as_ptr_i64(),
                _ => panic!("aaload: element is not an object reference"),
            }
        }
        _ => panic!("aaload: expected reference array"),
    }
}

extern "C" fn jit_aastore(array_ptr: i64, index: i32, value: i64) {
    let gc_ref = gc_ref_from_ptr(array_ptr);
    let mut reference = gc_ref.write();
    match &mut *reference {
        Reference::Array(obj_array) => {
            let element = &mut obj_array.elements[index as usize];
            if value == 0 {
                *element = Value::Object(None);
            } else {
                let gc: Gc<RwLock<Reference>> = Gc::from_raw_i64(value);
                *element = Value::Object(Some(gc));
            }
        }
        _ => panic!("aastore: expected reference array"),
    }
}
