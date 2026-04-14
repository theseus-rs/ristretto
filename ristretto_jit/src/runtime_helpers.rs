use crate::Error::InternalError;
use crate::Result;
use cranelift::codegen::ir::FuncRef;
use cranelift::jit::JITModule;
use cranelift::module::{Linkage, Module};
use cranelift::prelude::*;

/// References to runtime helper functions that are callable from JIT-compiled code.
///
/// These helpers are `extern "C"` functions defined in the VM crate and registered
/// as symbols with the JIT module. They allow JIT-compiled code to interact with
/// the GC-managed heap for array allocation and element access.
pub(crate) struct RuntimeHelpers {
    // Array allocation: (gc_ptr: ptr, count: i32) -> i64
    pub new_bool_array: FuncRef,
    pub new_byte_array: FuncRef,
    pub new_char_array: FuncRef,
    pub new_short_array: FuncRef,
    pub new_int_array: FuncRef,
    pub new_long_array: FuncRef,
    pub new_float_array: FuncRef,
    pub new_double_array: FuncRef,
    // Array length: (array_ptr: i64) -> i32
    pub arraylength: FuncRef,
    // Byte array access: (array_ptr: i64, index: i32) -> i32 / (array_ptr: i64, index: i32, value: i32)
    pub baload: FuncRef,
    pub bastore: FuncRef,
    // Char array access
    pub caload: FuncRef,
    pub castore: FuncRef,
    // Short array access
    pub saload: FuncRef,
    pub sastore: FuncRef,
    // Int array access
    pub iaload: FuncRef,
    pub iastore: FuncRef,
    // Long array access: returns/takes i64
    pub laload: FuncRef,
    pub lastore: FuncRef,
    // Float array access: returns/takes f32
    pub faload: FuncRef,
    pub fastore: FuncRef,
    // Double array access: returns/takes f64
    pub daload: FuncRef,
    pub dastore: FuncRef,
    // Reference array access: (array_ptr: i64, index: i32) -> i64 / (array_ptr: i64, index: i32, value: i64)
    pub aaload: FuncRef,
    pub aastore: FuncRef,
}

impl RuntimeHelpers {
    /// Declares all runtime helper function signatures and creates `FuncRefs`
    /// for use in the given function builder.
    #[expect(clippy::similar_names)]
    pub fn declare(
        jit_module: &mut JITModule,
        function_builder: &mut FunctionBuilder,
    ) -> Result<Self> {
        let ptr_type = jit_module.target_config().pointer_type();

        // Helper to declare and import a function
        let mut declare = |name: &str, params: &[Type], returns: &[Type]| -> Result<FuncRef> {
            let mut sig = jit_module.make_signature();
            for &p in params {
                sig.params.push(AbiParam::new(p));
            }
            for &r in returns {
                sig.returns.push(AbiParam::new(r));
            }
            let id = jit_module
                .declare_function(name, Linkage::Import, &sig)
                .map_err(|e| InternalError(format!("Failed to declare {name}: {e}")))?;
            Ok(jit_module.declare_func_in_func(id, function_builder.func))
        };

        // Array allocation: (gc_ptr: ptr, count: i32) -> i64 (Gc pointer)
        let alloc_params = &[ptr_type, types::I32];
        let alloc_returns = &[types::I64];

        let new_bool_array = declare("jit_new_bool_array", alloc_params, alloc_returns)?;
        let new_byte_array = declare("jit_new_byte_array", alloc_params, alloc_returns)?;
        let new_char_array = declare("jit_new_char_array", alloc_params, alloc_returns)?;
        let new_short_array = declare("jit_new_short_array", alloc_params, alloc_returns)?;
        let new_int_array = declare("jit_new_int_array", alloc_params, alloc_returns)?;
        let new_long_array = declare("jit_new_long_array", alloc_params, alloc_returns)?;
        let new_float_array = declare("jit_new_float_array", alloc_params, alloc_returns)?;
        let new_double_array = declare("jit_new_double_array", alloc_params, alloc_returns)?;

        // Array length: (array_ptr: i64) -> i32
        let arraylength = declare("jit_arraylength", &[types::I64], &[types::I32])?;

        // Int-returning loads: (array_ptr: i64, index: i32) -> i32
        let int_load_params = &[types::I64, types::I32];
        let int_load_returns = &[types::I32];
        let baload = declare("jit_baload", int_load_params, int_load_returns)?;
        let caload = declare("jit_caload", int_load_params, int_load_returns)?;
        let saload = declare("jit_saload", int_load_params, int_load_returns)?;
        let iaload = declare("jit_iaload", int_load_params, int_load_returns)?;

        // Int-value stores: (array_ptr: i64, index: i32, value: i32)
        let int_store_params = &[types::I64, types::I32, types::I32];
        let bastore = declare("jit_bastore", int_store_params, &[])?;
        let castore = declare("jit_castore", int_store_params, &[])?;
        let sastore = declare("jit_sastore", int_store_params, &[])?;
        let iastore = declare("jit_iastore", int_store_params, &[])?;

        // Long array: (array_ptr: i64, index: i32) -> i64 / (array_ptr: i64, index: i32, value: i64)
        let laload = declare("jit_laload", &[types::I64, types::I32], &[types::I64])?;
        let lastore = declare("jit_lastore", &[types::I64, types::I32, types::I64], &[])?;

        // Float array: (array_ptr: i64, index: i32) -> f32 / (array_ptr: i64, index: i32, value: f32)
        let faload = declare("jit_faload", &[types::I64, types::I32], &[types::F32])?;
        let fastore = declare("jit_fastore", &[types::I64, types::I32, types::F32], &[])?;

        // Double array: (array_ptr: i64, index: i32) -> f64 / (array_ptr: i64, index: i32, value: f64)
        let daload = declare("jit_daload", &[types::I64, types::I32], &[types::F64])?;
        let dastore = declare("jit_dastore", &[types::I64, types::I32, types::F64], &[])?;

        // Reference array: (array_ptr: i64, index: i32) -> i64 / (array_ptr: i64, index: i32, value: i64)
        let aaload = declare("jit_aaload", &[types::I64, types::I32], &[types::I64])?;
        let aastore = declare("jit_aastore", &[types::I64, types::I32, types::I64], &[])?;

        Ok(Self {
            new_bool_array,
            new_byte_array,
            new_char_array,
            new_short_array,
            new_int_array,
            new_long_array,
            new_float_array,
            new_double_array,
            arraylength,
            baload,
            bastore,
            caload,
            castore,
            saload,
            sastore,
            iaload,
            iastore,
            laload,
            lastore,
            faload,
            fastore,
            daload,
            dastore,
            aaload,
            aastore,
        })
    }
}
