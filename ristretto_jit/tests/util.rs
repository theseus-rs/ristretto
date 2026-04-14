#![allow(unsafe_code)]
#![expect(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_ptr_alignment,
    clippy::cast_lossless,
    clippy::ptr_as_ptr,
    reason = "JIT FFI stubs intentionally use raw pointer casts matching JVM semantics"
)]

use ristretto_classfile::attributes::{Attribute, Instruction, MaxLocals, MaxStack};
use ristretto_classfile::{ClassAccessFlags, ClassFile, ConstantPool, MethodAccessFlags};
use ristretto_jit::{Compiler, Function, Result};
use std::alloc::{Layout, alloc_zeroed};

/// Creates a function from the given descriptor and instructions.
///
/// # Errors
///
/// If there is an error creating the function
pub fn create_function(descriptor: &str, instructions: &[Instruction]) -> Result<Function> {
    let constant_pool = ConstantPool::default();
    create_function_with_constant_pool(constant_pool, descriptor, instructions)
}

/// Creates a function with the specified constant pool,from the given descriptor and instructions.
///
/// # Errors
///
/// If there is an error creating the function
pub fn create_function_with_constant_pool(
    mut constant_pool: ConstantPool,
    descriptor: &str,
    instructions: &[Instruction],
) -> Result<Function> {
    let class_name_index = constant_pool.add_class("Test")?;
    let code_index = constant_pool.add_utf8("Code")?;
    let test_name_index = constant_pool.add_utf8("test")?;
    let test_descriptor_index = constant_pool.add_utf8(descriptor)?;

    let mut test_method = ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: test_name_index,
        descriptor_index: test_descriptor_index,
        attributes: Vec::new(),
    };
    let test_max_stack = instructions.max_stack(&constant_pool)?;
    let test_max_locals = instructions.max_locals(&constant_pool, &test_method)?;
    test_method.attributes.push(Attribute::Code {
        name_index: code_index,
        max_stack: test_max_stack,
        max_locals: test_max_locals,
        code: instructions.to_vec(),
        exception_table: Vec::new(),
        attributes: Vec::new(),
    });
    let class_file = ClassFile {
        constant_pool: constant_pool.clone(),
        access_flags: ClassAccessFlags::PUBLIC,
        this_class: class_name_index,
        methods: vec![test_method],
        attributes: Vec::new(),
        ..Default::default()
    };
    let test_method = &class_file.methods[0];

    let symbols = test_runtime_helper_symbols();
    let compiler = Compiler::new()?;
    let function = compiler.compile(&class_file, test_method, &symbols)?;
    Ok(function)
}

// ---------------------------------------------------------------------------
// Test-only runtime helper stubs for JIT array operations.
//
// In production, these are defined in `ristretto_vm` backed by the garbage
// collector. For tests, we use plain `alloc` with a simple layout:
// [i64 length][element data...]
// ---------------------------------------------------------------------------

const HEADER_SIZE: usize = 8;

unsafe fn alloc_array(count: i32, element_size: usize) -> i64 {
    let data_size = count as usize * element_size;
    let total_size = HEADER_SIZE + data_size;
    let layout = Layout::from_size_align(total_size, 8).expect("invalid layout");
    let ptr = unsafe { alloc_zeroed(layout) };
    unsafe { *(ptr as *mut i64) = count as i64 };
    ptr as i64
}

extern "C" fn jit_new_bool_array(_ctx: *const u8, count: i32) -> i64 {
    unsafe { alloc_array(count, 1) }
}
extern "C" fn jit_new_byte_array(_ctx: *const u8, count: i32) -> i64 {
    unsafe { alloc_array(count, 1) }
}
extern "C" fn jit_new_char_array(_ctx: *const u8, count: i32) -> i64 {
    unsafe { alloc_array(count, 2) }
}
extern "C" fn jit_new_short_array(_ctx: *const u8, count: i32) -> i64 {
    unsafe { alloc_array(count, 2) }
}
extern "C" fn jit_new_int_array(_ctx: *const u8, count: i32) -> i64 {
    unsafe { alloc_array(count, 4) }
}
extern "C" fn jit_new_long_array(_ctx: *const u8, count: i32) -> i64 {
    unsafe { alloc_array(count, 8) }
}
extern "C" fn jit_new_float_array(_ctx: *const u8, count: i32) -> i64 {
    unsafe { alloc_array(count, 4) }
}
extern "C" fn jit_new_double_array(_ctx: *const u8, count: i32) -> i64 {
    unsafe { alloc_array(count, 8) }
}

extern "C" fn jit_arraylength(array_ptr: i64) -> i32 {
    unsafe { *(array_ptr as *const i64) as i32 }
}

extern "C" fn jit_baload(array_ptr: i64, index: i32) -> i32 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const i8;
    unsafe { *data.add(index as usize) as i32 }
}
extern "C" fn jit_bastore(array_ptr: i64, index: i32, value: i32) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut i8;
    unsafe { *data.add(index as usize) = value as i8 };
}
extern "C" fn jit_caload(array_ptr: i64, index: i32) -> i32 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const u16;
    unsafe { *data.add(index as usize) as i32 }
}
extern "C" fn jit_castore(array_ptr: i64, index: i32, value: i32) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut u16;
    unsafe { *data.add(index as usize) = value as u16 };
}
extern "C" fn jit_saload(array_ptr: i64, index: i32) -> i32 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const i16;
    unsafe { *data.add(index as usize) as i32 }
}
extern "C" fn jit_sastore(array_ptr: i64, index: i32, value: i32) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut i16;
    unsafe { *data.add(index as usize) = value as i16 };
}
extern "C" fn jit_iaload(array_ptr: i64, index: i32) -> i32 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const i32;
    unsafe { *data.add(index as usize) }
}
extern "C" fn jit_iastore(array_ptr: i64, index: i32, value: i32) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut i32;
    unsafe { *data.add(index as usize) = value };
}
extern "C" fn jit_laload(array_ptr: i64, index: i32) -> i64 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const i64;
    unsafe { *data.add(index as usize) }
}
extern "C" fn jit_lastore(array_ptr: i64, index: i32, value: i64) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut i64;
    unsafe { *data.add(index as usize) = value };
}
extern "C" fn jit_faload(array_ptr: i64, index: i32) -> f32 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const f32;
    unsafe { *data.add(index as usize) }
}
extern "C" fn jit_fastore(array_ptr: i64, index: i32, value: f32) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut f32;
    unsafe { *data.add(index as usize) = value };
}
extern "C" fn jit_daload(array_ptr: i64, index: i32) -> f64 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const f64;
    unsafe { *data.add(index as usize) }
}
extern "C" fn jit_dastore(array_ptr: i64, index: i32, value: f64) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut f64;
    unsafe { *data.add(index as usize) = value };
}
extern "C" fn jit_aaload(array_ptr: i64, index: i32) -> i64 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const i64;
    unsafe { *data.add(index as usize) }
}
extern "C" fn jit_aastore(array_ptr: i64, index: i32, value: i64) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut i64;
    unsafe { *data.add(index as usize) = value };
}

fn test_runtime_helper_symbols() -> Vec<(&'static str, *const u8)> {
    vec![
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
