#![allow(unsafe_code)]
#![expect(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_ptr_alignment,
    clippy::cast_lossless,
    clippy::ptr_as_ptr,
    reason = "JIT FFI stubs intentionally use raw pointer casts matching JVM semantics"
)]

use ahash::AHashMap;
use ristretto_classfile::attributes::{Attribute, Instruction, MaxLocals, MaxStack};
use ristretto_classfile::{ClassAccessFlags, ClassFile, ConstantPool, MethodAccessFlags};
use ristretto_jit::{Compiler, Function, Result};
use std::alloc::{Layout, alloc_zeroed};
use std::cell::UnsafeCell;
use std::sync::{Mutex, OnceLock};

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
    constant_pool: ConstantPool,
    descriptor: &str,
    instructions: &[Instruction],
) -> Result<Function> {
    create_function_with_exception_table(constant_pool, descriptor, instructions, Vec::new())
}

/// Creates a function with the specified constant pool, exception table, descriptor, and
/// instructions.
///
/// # Errors
///
/// If there is an error creating the function
pub fn create_function_with_exception_table(
    mut constant_pool: ConstantPool,
    descriptor: &str,
    instructions: &[Instruction],
    exception_table: Vec<ristretto_classfile::attributes::ExceptionTableEntry>,
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
        exception_table,
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

extern "C" fn jit_arraylength(_ctx: *const u8, _bci: i32, array_ptr: i64) -> i32 {
    unsafe { *(array_ptr as *const i64) as i32 }
}

extern "C" fn jit_baload(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32) -> i32 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const i8;
    unsafe { *data.add(index as usize) as i32 }
}
extern "C" fn jit_bastore(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32, value: i32) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut i8;
    unsafe { *data.add(index as usize) = value as i8 };
}
extern "C" fn jit_caload(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32) -> i32 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const u16;
    unsafe { *data.add(index as usize) as i32 }
}
extern "C" fn jit_castore(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32, value: i32) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut u16;
    unsafe { *data.add(index as usize) = value as u16 };
}
extern "C" fn jit_saload(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32) -> i32 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const i16;
    unsafe { *data.add(index as usize) as i32 }
}
extern "C" fn jit_sastore(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32, value: i32) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut i16;
    unsafe { *data.add(index as usize) = value as i16 };
}
extern "C" fn jit_iaload(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32) -> i32 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const i32;
    unsafe { *data.add(index as usize) }
}
extern "C" fn jit_iastore(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32, value: i32) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut i32;
    unsafe { *data.add(index as usize) = value };
}
extern "C" fn jit_laload(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32) -> i64 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const i64;
    unsafe { *data.add(index as usize) }
}
extern "C" fn jit_lastore(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32, value: i64) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut i64;
    unsafe { *data.add(index as usize) = value };
}
extern "C" fn jit_faload(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32) -> f32 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const f32;
    unsafe { *data.add(index as usize) }
}
extern "C" fn jit_fastore(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32, value: f32) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut f32;
    unsafe { *data.add(index as usize) = value };
}
extern "C" fn jit_daload(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32) -> f64 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const f64;
    unsafe { *data.add(index as usize) }
}
extern "C" fn jit_dastore(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32, value: f64) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut f64;
    unsafe { *data.add(index as usize) = value };
}
extern "C" fn jit_aaload(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32) -> i64 {
    let data = (array_ptr as usize + HEADER_SIZE) as *const i64;
    unsafe { *data.add(index as usize) }
}
extern "C" fn jit_aastore(_ctx: *const u8, _bci: i32, array_ptr: i64, index: i32, value: i64) {
    let data = (array_ptr as usize + HEADER_SIZE) as *mut i64;
    unsafe { *data.add(index as usize) = value };
}

fn test_runtime_helper_symbols() -> Vec<(&'static str, *const u8)> {
    let mut symbols = vec![
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
    ];
    symbols.extend(new_helper_symbols());
    symbols
}

// ---------------------------------------------------------------------------
// Test-only runtime helper stubs for JIT field, allocation, and type-check ops.
//
// The production implementations live in `ristretto_vm` and use the GC-managed
// object model. The stubs below mirror the helper ABI declared in
// `runtime_helpers.rs` using simple in-memory bookkeeping so that the JIT
// emitter's codegen can be exercised end-to-end in unit tests.
// ---------------------------------------------------------------------------

/// Tagged representation of a stored field value.
#[derive(Clone, Copy, Debug)]
pub enum TestFieldValue {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(i64),
}

/// A test-only context passed as the JIT context pointer. Holds a pending
/// exception slot that helpers can read and write, plus a class-id namespace
/// (sentinel values used to model instanceof checks).
#[derive(Debug)]
pub struct TestContext {
    pending_exception: UnsafeCell<i64>,
}

impl TestContext {
    #[must_use]
    pub fn new() -> Self {
        Self {
            pending_exception: UnsafeCell::new(0),
        }
    }

    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        std::ptr::from_ref(self).cast::<u8>()
    }

    pub fn pending_exception(&self) -> i64 {
        unsafe { *self.pending_exception.get() }
    }

    pub fn clear_pending(&self) {
        unsafe { *self.pending_exception.get() = 0 };
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Key: (`ctx_ptr`, `cp_field_ref_index`). The ctx pointer namespaces storage by
/// `TestContext` instance so concurrently-running tests cannot observe each other's
/// writes when they happen to receive identical CP indices.
fn static_storage() -> &'static Mutex<AHashMap<(usize, u32), TestFieldValue>> {
    static STATIC: OnceLock<Mutex<AHashMap<(usize, u32), TestFieldValue>>> = OnceLock::new();
    STATIC.get_or_init(|| Mutex::new(AHashMap::new()))
}

/// Key: (`obj_ptr`, `cp_field_ref_index`). Instance fields keyed on object identity.
fn instance_storage() -> &'static Mutex<AHashMap<(i64, u32), TestFieldValue>> {
    static STORAGE: OnceLock<Mutex<AHashMap<(i64, u32), TestFieldValue>>> = OnceLock::new();
    STORAGE.get_or_init(|| Mutex::new(AHashMap::new()))
}

/// Monotonically increasing object-id allocator for test new/anewarray/multianewarray helpers.
fn next_object_id() -> i64 {
    use std::sync::atomic::{AtomicI64, Ordering};
    static NEXT: AtomicI64 = AtomicI64::new(1);
    NEXT.fetch_add(1, Ordering::SeqCst)
}

fn set_pending(ctx: *const u8, value: i64) {
    if ctx.is_null() {
        return;
    }
    // SAFETY: ctx points at a TestContext for the duration of the test.
    let ctx = unsafe { &*(ctx as *const TestContext) };
    unsafe { *ctx.pending_exception.get() = value };
}

fn pending(ctx: *const u8) -> i64 {
    if ctx.is_null() {
        return 0;
    }
    let ctx = unsafe { &*(ctx as *const TestContext) };
    unsafe { *ctx.pending_exception.get() }
}

extern "C" fn jit_new(_ctx: *const u8, _bci: i32, _cp_class_index: i32) -> i64 {
    next_object_id()
}

extern "C" fn jit_anewarray(_ctx: *const u8, _bci: i32, _cp_class_index: i32, count: i32) -> i64 {
    unsafe { alloc_array(count, 8) }
}

extern "C" fn jit_multianewarray(
    _ctx: *const u8,
    _bci: i32,
    _cp_class_index: i32,
    dims_ptr: *const i32,
    dims_len: i32,
) -> i64 {
    // For tests, just allocate a 1-D array sized to the outermost dimension.
    if dims_len == 0 {
        return 0;
    }
    let outermost = unsafe { *dims_ptr };
    unsafe { alloc_array(outermost, 8) }
}

extern "C" fn jit_getstatic_int(ctx: *const u8, _bci: i32, field_ref: i32) -> i32 {
    match static_storage()
        .lock()
        .expect("test")
        .get(&(ctx as usize, field_ref as u32))
        .copied()
    {
        Some(TestFieldValue::Int(v)) => v,
        _ => 0,
    }
}
extern "C" fn jit_getstatic_long(ctx: *const u8, _bci: i32, field_ref: i32) -> i64 {
    match static_storage()
        .lock()
        .expect("test")
        .get(&(ctx as usize, field_ref as u32))
        .copied()
    {
        Some(TestFieldValue::Long(v)) => v,
        _ => 0,
    }
}
extern "C" fn jit_getstatic_float(ctx: *const u8, _bci: i32, field_ref: i32) -> f32 {
    match static_storage()
        .lock()
        .expect("test")
        .get(&(ctx as usize, field_ref as u32))
        .copied()
    {
        Some(TestFieldValue::Float(v)) => v,
        _ => 0.0,
    }
}
extern "C" fn jit_getstatic_double(ctx: *const u8, _bci: i32, field_ref: i32) -> f64 {
    match static_storage()
        .lock()
        .expect("test")
        .get(&(ctx as usize, field_ref as u32))
        .copied()
    {
        Some(TestFieldValue::Double(v)) => v,
        _ => 0.0,
    }
}
extern "C" fn jit_getstatic_object(ctx: *const u8, _bci: i32, field_ref: i32) -> i64 {
    match static_storage()
        .lock()
        .expect("test")
        .get(&(ctx as usize, field_ref as u32))
        .copied()
    {
        Some(TestFieldValue::Object(v)) => v,
        _ => 0,
    }
}

extern "C" fn jit_putstatic_int(ctx: *const u8, _bci: i32, field_ref: i32, value: i32) {
    static_storage()
        .lock()
        .expect("test")
        .insert((ctx as usize, field_ref as u32), TestFieldValue::Int(value));
}
extern "C" fn jit_putstatic_long(ctx: *const u8, _bci: i32, field_ref: i32, value: i64) {
    static_storage().lock().expect("test").insert(
        (ctx as usize, field_ref as u32),
        TestFieldValue::Long(value),
    );
}
extern "C" fn jit_putstatic_float(ctx: *const u8, _bci: i32, field_ref: i32, value: f32) {
    static_storage().lock().expect("test").insert(
        (ctx as usize, field_ref as u32),
        TestFieldValue::Float(value),
    );
}
extern "C" fn jit_putstatic_double(ctx: *const u8, _bci: i32, field_ref: i32, value: f64) {
    static_storage().lock().expect("test").insert(
        (ctx as usize, field_ref as u32),
        TestFieldValue::Double(value),
    );
}
extern "C" fn jit_putstatic_object(ctx: *const u8, _bci: i32, field_ref: i32, value: i64) {
    static_storage().lock().expect("test").insert(
        (ctx as usize, field_ref as u32),
        TestFieldValue::Object(value),
    );
}

extern "C" fn jit_getfield_int(ctx: *const u8, _bci: i32, field_ref: i32, obj: i64) -> i32 {
    if obj == 0 {
        set_pending(ctx, next_object_id());
        return 0;
    }
    match instance_storage()
        .lock()
        .expect("test")
        .get(&(obj, field_ref as u32))
        .copied()
    {
        Some(TestFieldValue::Int(v)) => v,
        _ => 0,
    }
}
extern "C" fn jit_getfield_long(ctx: *const u8, _bci: i32, field_ref: i32, obj: i64) -> i64 {
    if obj == 0 {
        set_pending(ctx, next_object_id());
        return 0;
    }
    match instance_storage()
        .lock()
        .expect("test")
        .get(&(obj, field_ref as u32))
        .copied()
    {
        Some(TestFieldValue::Long(v)) => v,
        _ => 0,
    }
}
extern "C" fn jit_getfield_float(ctx: *const u8, _bci: i32, field_ref: i32, obj: i64) -> f32 {
    if obj == 0 {
        set_pending(ctx, next_object_id());
        return 0.0;
    }
    match instance_storage()
        .lock()
        .expect("test")
        .get(&(obj, field_ref as u32))
        .copied()
    {
        Some(TestFieldValue::Float(v)) => v,
        _ => 0.0,
    }
}
extern "C" fn jit_getfield_double(ctx: *const u8, _bci: i32, field_ref: i32, obj: i64) -> f64 {
    if obj == 0 {
        set_pending(ctx, next_object_id());
        return 0.0;
    }
    match instance_storage()
        .lock()
        .expect("test")
        .get(&(obj, field_ref as u32))
        .copied()
    {
        Some(TestFieldValue::Double(v)) => v,
        _ => 0.0,
    }
}
extern "C" fn jit_getfield_object(ctx: *const u8, _bci: i32, field_ref: i32, obj: i64) -> i64 {
    if obj == 0 {
        set_pending(ctx, next_object_id());
        return 0;
    }
    match instance_storage()
        .lock()
        .expect("test")
        .get(&(obj, field_ref as u32))
        .copied()
    {
        Some(TestFieldValue::Object(v)) => v,
        _ => 0,
    }
}

extern "C" fn jit_putfield_int(ctx: *const u8, _bci: i32, field_ref: i32, obj: i64, value: i32) {
    if obj == 0 {
        set_pending(ctx, next_object_id());
        return;
    }
    instance_storage()
        .lock()
        .expect("test")
        .insert((obj, field_ref as u32), TestFieldValue::Int(value));
}
extern "C" fn jit_putfield_long(ctx: *const u8, _bci: i32, field_ref: i32, obj: i64, value: i64) {
    if obj == 0 {
        set_pending(ctx, next_object_id());
        return;
    }
    instance_storage()
        .lock()
        .expect("test")
        .insert((obj, field_ref as u32), TestFieldValue::Long(value));
}
extern "C" fn jit_putfield_float(ctx: *const u8, _bci: i32, field_ref: i32, obj: i64, value: f32) {
    if obj == 0 {
        set_pending(ctx, next_object_id());
        return;
    }
    instance_storage()
        .lock()
        .expect("test")
        .insert((obj, field_ref as u32), TestFieldValue::Float(value));
}
extern "C" fn jit_putfield_double(ctx: *const u8, _bci: i32, field_ref: i32, obj: i64, value: f64) {
    if obj == 0 {
        set_pending(ctx, next_object_id());
        return;
    }
    instance_storage()
        .lock()
        .expect("test")
        .insert((obj, field_ref as u32), TestFieldValue::Double(value));
}
extern "C" fn jit_putfield_object(ctx: *const u8, _bci: i32, field_ref: i32, obj: i64, value: i64) {
    if obj == 0 {
        set_pending(ctx, next_object_id());
        return;
    }
    instance_storage()
        .lock()
        .expect("test")
        .insert((obj, field_ref as u32), TestFieldValue::Object(value));
}

/// Test-only checkcast: succeeds unless `cp_class_index` is 0xFFFF, in which case it sets a
/// pending exception. This is sufficient to exercise the JIT's pending exception check path.
extern "C" fn jit_checkcast(ctx: *const u8, _bci: i32, _obj: i64, cp_class_index: i32) -> i32 {
    if cp_class_index == 0xFFFF {
        set_pending(ctx, next_object_id());
        return 0;
    }
    1
}

/// Test-only instanceof: returns 1 if `obj != 0` and `cp_class_index != 0`, else 0.
extern "C" fn jit_instanceof(_ctx: *const u8, _bci: i32, obj: i64, cp_class_index: i32) -> i32 {
    i32::from(obj != 0 && cp_class_index != 0)
}

extern "C" fn jit_athrow(ctx: *const u8, _bci: i32, obj: i64) {
    set_pending(ctx, if obj == 0 { next_object_id() } else { obj });
}

extern "C" fn jit_pending_exception(ctx: *const u8) -> i64 {
    pending(ctx)
}

/// Test-stub semantics: the pending exception's class is the pending exception pointer cast to
/// u16. Tests that exercise typed handlers should throw an object whose pointer value equals the
/// `catch_type` CP class index they want to match. `catch_type == 0` is always a catch-all and
/// is handled by the JIT without calling this helper.
extern "C" fn jit_exception_matches(ctx: *const u8, cp_class_index: i32) -> i32 {
    let pending = pending(ctx);
    if pending == 0 {
        return 0;
    }
    // The catch-all case (cp_class_index == 0) is handled in the JIT without calling this helper;
    // if it ever is called here, match unconditionally.
    if cp_class_index == 0 {
        return 1;
    }
    i32::from(pending == i64::from(cp_class_index))
}

extern "C" fn jit_take_pending_exception(ctx: *const u8) -> i64 {
    // SAFETY: ctx points at a TestContext for the duration of the test.
    let test_ctx = unsafe { &*(ctx as *const TestContext) };
    let value = unsafe { *test_ctx.pending_exception.get() };
    unsafe { *test_ctx.pending_exception.get() = 0 };
    value
}

extern "C" fn jit_throw_npe(ctx: *const u8, _bci: i32) {
    // SAFETY: ctx points at a TestContext for the duration of the test.
    let test_ctx = unsafe { &*(ctx as *const TestContext) };
    unsafe { *test_ctx.pending_exception.get() = -1 };
}

fn new_helper_symbols() -> Vec<(&'static str, *const u8)> {
    vec![
        ("jit_new", jit_new as *const u8),
        ("jit_anewarray", jit_anewarray as *const u8),
        ("jit_multianewarray", jit_multianewarray as *const u8),
        ("jit_getstatic_int", jit_getstatic_int as *const u8),
        ("jit_getstatic_long", jit_getstatic_long as *const u8),
        ("jit_getstatic_float", jit_getstatic_float as *const u8),
        ("jit_getstatic_double", jit_getstatic_double as *const u8),
        ("jit_getstatic_object", jit_getstatic_object as *const u8),
        ("jit_putstatic_int", jit_putstatic_int as *const u8),
        ("jit_putstatic_long", jit_putstatic_long as *const u8),
        ("jit_putstatic_float", jit_putstatic_float as *const u8),
        ("jit_putstatic_double", jit_putstatic_double as *const u8),
        ("jit_putstatic_object", jit_putstatic_object as *const u8),
        ("jit_getfield_int", jit_getfield_int as *const u8),
        ("jit_getfield_long", jit_getfield_long as *const u8),
        ("jit_getfield_float", jit_getfield_float as *const u8),
        ("jit_getfield_double", jit_getfield_double as *const u8),
        ("jit_getfield_object", jit_getfield_object as *const u8),
        ("jit_putfield_int", jit_putfield_int as *const u8),
        ("jit_putfield_long", jit_putfield_long as *const u8),
        ("jit_putfield_float", jit_putfield_float as *const u8),
        ("jit_putfield_double", jit_putfield_double as *const u8),
        ("jit_putfield_object", jit_putfield_object as *const u8),
        ("jit_checkcast", jit_checkcast as *const u8),
        ("jit_instanceof", jit_instanceof as *const u8),
        ("jit_athrow", jit_athrow as *const u8),
        ("jit_pending_exception", jit_pending_exception as *const u8),
        (
            "jit_take_pending_exception",
            jit_take_pending_exception as *const u8,
        ),
        ("jit_exception_matches", jit_exception_matches as *const u8),
        ("jit_throw_npe", jit_throw_npe as *const u8),
    ]
}
