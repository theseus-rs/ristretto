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

    // Object allocation: (ctx: ptr, cp_class_index: i32) -> i64
    pub new_object: FuncRef,
    // Reference array allocation: (ctx: ptr, cp_class_index: i32, count: i32) -> i64
    pub anewarray: FuncRef,
    // Multi-dimensional array allocation: (ctx: ptr, cp_class_index: i32, dims_ptr: ptr, dims_len: i32) -> i64
    pub multianewarray: FuncRef,

    // Static field access: (ctx: ptr, cp_field_ref_index: i32) -> T / (ctx: ptr, cp_field_ref_index: i32, value: T)
    pub getstatic_int: FuncRef,
    pub getstatic_long: FuncRef,
    pub getstatic_float: FuncRef,
    pub getstatic_double: FuncRef,
    pub getstatic_object: FuncRef,
    pub putstatic_int: FuncRef,
    pub putstatic_long: FuncRef,
    pub putstatic_float: FuncRef,
    pub putstatic_double: FuncRef,
    pub putstatic_object: FuncRef,

    // Instance field access: (ctx: ptr, cp_field_ref_index: i32, obj: i64) -> T / (ctx: ptr, cp_field_ref_index: i32, obj: i64, value: T)
    pub getfield_int: FuncRef,
    pub getfield_long: FuncRef,
    pub getfield_float: FuncRef,
    pub getfield_double: FuncRef,
    pub getfield_object: FuncRef,
    pub putfield_int: FuncRef,
    pub putfield_long: FuncRef,
    pub putfield_float: FuncRef,
    pub putfield_double: FuncRef,
    pub putfield_object: FuncRef,

    // Type checks and exceptions
    // checkcast: (ctx: ptr, obj: i64, cp_class_index: i32) -> i32 (1 on success; on failure sets
    //            pending exception and returns 0)
    pub checkcast: FuncRef,
    // instanceof: (ctx: ptr, obj: i64, cp_class_index: i32) -> i32 (1 if instance, 0 otherwise)
    pub instanceof: FuncRef,
    // athrow: (ctx: ptr, obj: i64);stores obj as pending exception
    pub athrow: FuncRef,

    // Returns the currently-pending exception pointer (0 if none).
    // pending_exception: (ctx: ptr) -> i64
    pub pending_exception: FuncRef,
    // Reads and clears the pending exception; returns the exception reference (0 if none).
    // take_pending_exception: (ctx: ptr) -> i64
    pub take_pending_exception: FuncRef,
    // Returns 1 if the pending exception is assignable to the class referenced by
    // `cp_class_index`, otherwise 0. Does not modify the pending exception slot.
    // exception_matches: (ctx: ptr, cp_class_index: i32) -> i32
    pub exception_matches: FuncRef,
    // Stores a NullPointerException in the pending exception slot and updates the top
    // frame's program counter to `bci` so the throwable's stack trace reports the BCI of
    // the dereference site. A negative `bci` leaves the program counter unchanged.
    // throw_npe: (ctx: ptr, bci: i32)
    pub throw_npe: FuncRef,
}

impl RuntimeHelpers {
    /// Declares all runtime helper function signatures and creates `FuncRefs`
    /// for use in the given function builder.
    #[expect(clippy::similar_names)]
    #[expect(clippy::too_many_lines)]
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

        // Array length: (ctx: ptr, bci: i32, array_ptr: i64) -> i32
        let arraylength = declare(
            "jit_arraylength",
            &[ptr_type, types::I32, types::I64],
            &[types::I32],
        )?;

        // Int-returning loads: (ctx: ptr, bci: i32, array_ptr: i64, index: i32) -> i32
        let int_load_params = &[ptr_type, types::I32, types::I64, types::I32];
        let int_load_returns = &[types::I32];
        let baload = declare("jit_baload", int_load_params, int_load_returns)?;
        let caload = declare("jit_caload", int_load_params, int_load_returns)?;
        let saload = declare("jit_saload", int_load_params, int_load_returns)?;
        let iaload = declare("jit_iaload", int_load_params, int_load_returns)?;

        // Int-value stores: (ctx: ptr, bci: i32, array_ptr: i64, index: i32, value: i32)
        let int_store_params = &[ptr_type, types::I32, types::I64, types::I32, types::I32];
        let bastore = declare("jit_bastore", int_store_params, &[])?;
        let castore = declare("jit_castore", int_store_params, &[])?;
        let sastore = declare("jit_sastore", int_store_params, &[])?;
        let iastore = declare("jit_iastore", int_store_params, &[])?;

        // Long array
        let laload = declare(
            "jit_laload",
            &[ptr_type, types::I32, types::I64, types::I32],
            &[types::I64],
        )?;
        let lastore = declare(
            "jit_lastore",
            &[ptr_type, types::I32, types::I64, types::I32, types::I64],
            &[],
        )?;

        // Float array
        let faload = declare(
            "jit_faload",
            &[ptr_type, types::I32, types::I64, types::I32],
            &[types::F32],
        )?;
        let fastore = declare(
            "jit_fastore",
            &[ptr_type, types::I32, types::I64, types::I32, types::F32],
            &[],
        )?;

        // Double array
        let daload = declare(
            "jit_daload",
            &[ptr_type, types::I32, types::I64, types::I32],
            &[types::F64],
        )?;
        let dastore = declare(
            "jit_dastore",
            &[ptr_type, types::I32, types::I64, types::I32, types::F64],
            &[],
        )?;

        // Reference array load: (ctx: ptr, bci: i32, array_ptr: i64, index: i32) -> i64
        let aaload = declare(
            "jit_aaload",
            &[ptr_type, types::I32, types::I64, types::I32],
            &[types::I64],
        )?;
        let aastore = declare(
            "jit_aastore",
            &[ptr_type, types::I32, types::I64, types::I32, types::I64],
            &[],
        )?;

        // Object allocation: (ctx: ptr, bci: i32, cp_class_index: i32) -> i64
        let new_object = declare(
            "jit_new",
            &[ptr_type, types::I32, types::I32],
            &[types::I64],
        )?;
        // Reference array allocation: (ctx: ptr, bci: i32, cp_class_index: i32, count: i32) -> i64
        let anewarray = declare(
            "jit_anewarray",
            &[ptr_type, types::I32, types::I32, types::I32],
            &[types::I64],
        )?;
        // Multi-dimensional array allocation
        let multianewarray = declare(
            "jit_multianewarray",
            &[ptr_type, types::I32, types::I32, ptr_type, types::I32],
            &[types::I64],
        )?;

        // Static field access. Each helper takes a `bci: i32` second parameter so the
        // runtime can stamp the JIT frame's program counter before storing a pending
        // exception;this makes Java stack traces report the originating BCI for
        // resolution / `<clinit>` failures, instead of always reporting BCI 0.
        let getstatic_params = &[ptr_type, types::I32, types::I32];
        let getstatic_int = declare("jit_getstatic_int", getstatic_params, &[types::I32])?;
        let getstatic_long = declare("jit_getstatic_long", getstatic_params, &[types::I64])?;
        let getstatic_float = declare("jit_getstatic_float", getstatic_params, &[types::F32])?;
        let getstatic_double = declare("jit_getstatic_double", getstatic_params, &[types::F64])?;
        let getstatic_object = declare("jit_getstatic_object", getstatic_params, &[types::I64])?;

        let putstatic_int = declare(
            "jit_putstatic_int",
            &[ptr_type, types::I32, types::I32, types::I32],
            &[],
        )?;
        let putstatic_long = declare(
            "jit_putstatic_long",
            &[ptr_type, types::I32, types::I32, types::I64],
            &[],
        )?;
        let putstatic_float = declare(
            "jit_putstatic_float",
            &[ptr_type, types::I32, types::I32, types::F32],
            &[],
        )?;
        let putstatic_double = declare(
            "jit_putstatic_double",
            &[ptr_type, types::I32, types::I32, types::F64],
            &[],
        )?;
        let putstatic_object = declare(
            "jit_putstatic_object",
            &[ptr_type, types::I32, types::I32, types::I64],
            &[],
        )?;

        // Instance field access (with leading `bci: i32`, see static helpers above).
        let getfield_params = &[ptr_type, types::I32, types::I32, types::I64];
        let getfield_int = declare("jit_getfield_int", getfield_params, &[types::I32])?;
        let getfield_long = declare("jit_getfield_long", getfield_params, &[types::I64])?;
        let getfield_float = declare("jit_getfield_float", getfield_params, &[types::F32])?;
        let getfield_double = declare("jit_getfield_double", getfield_params, &[types::F64])?;
        let getfield_object = declare("jit_getfield_object", getfield_params, &[types::I64])?;

        let putfield_int = declare(
            "jit_putfield_int",
            &[ptr_type, types::I32, types::I32, types::I64, types::I32],
            &[],
        )?;
        let putfield_long = declare(
            "jit_putfield_long",
            &[ptr_type, types::I32, types::I32, types::I64, types::I64],
            &[],
        )?;
        let putfield_float = declare(
            "jit_putfield_float",
            &[ptr_type, types::I32, types::I32, types::I64, types::F32],
            &[],
        )?;
        let putfield_double = declare(
            "jit_putfield_double",
            &[ptr_type, types::I32, types::I32, types::I64, types::F64],
            &[],
        )?;
        let putfield_object = declare(
            "jit_putfield_object",
            &[ptr_type, types::I32, types::I32, types::I64, types::I64],
            &[],
        )?;

        // Type checks and exceptions (bci threading per H1)
        let checkcast = declare(
            "jit_checkcast",
            &[ptr_type, types::I32, types::I64, types::I32],
            &[types::I32],
        )?;
        let instanceof = declare(
            "jit_instanceof",
            &[ptr_type, types::I32, types::I64, types::I32],
            &[types::I32],
        )?;
        let athrow = declare("jit_athrow", &[ptr_type, types::I32, types::I64], &[])?;
        let pending_exception = declare("jit_pending_exception", &[ptr_type], &[types::I64])?;
        let take_pending_exception =
            declare("jit_take_pending_exception", &[ptr_type], &[types::I64])?;
        let exception_matches = declare(
            "jit_exception_matches",
            &[ptr_type, types::I32],
            &[types::I32],
        )?;
        let throw_npe = declare("jit_throw_npe", &[ptr_type, types::I32], &[])?;

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
            new_object,
            anewarray,
            multianewarray,
            getstatic_int,
            getstatic_long,
            getstatic_float,
            getstatic_double,
            getstatic_object,
            putstatic_int,
            putstatic_long,
            putstatic_float,
            putstatic_double,
            putstatic_object,
            getfield_int,
            getfield_long,
            getfield_float,
            getfield_double,
            getfield_object,
            putfield_int,
            putfield_long,
            putfield_float,
            putfield_double,
            putfield_object,
            checkcast,
            instanceof,
            athrow,
            pending_exception,
            take_pending_exception,
            exception_matches,
            throw_npe,
        })
    }
}
