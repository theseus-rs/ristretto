#![cfg(not(target_family = "wasm"))]
#![expect(
    clippy::indexing_slicing,
    clippy::panic_in_result_fn,
    reason = "integration tests use direct fixture indexing and assertions with Result-returning tests"
)]
#[path = "integration/util.rs"]
mod util;

mod functions {
    #[path = "../functions/double_is_nan.rs"]
    mod double_is_nan;
    #[path = "../functions/float_is_nan.rs"]
    mod float_is_nan;
    #[path = "../functions/hash_code.rs"]
    mod hash_code;
    #[path = "../functions/math_max.rs"]
    mod math_max;
    #[path = "../functions/math_multiply_high.rs"]
    mod math_multiply_high;
    #[path = "../functions/object_init.rs"]
    mod object_init;
}

mod instructions {
    #[path = "../instructions/array.rs"]
    mod array;
    #[path = "../instructions/branch.rs"]
    mod branch;
    #[path = "../instructions/byte.rs"]
    mod byte;
    #[path = "../instructions/char.rs"]
    mod char;
    #[path = "../instructions/convert.rs"]
    mod convert;
    #[path = "../instructions/debug.rs"]
    mod debug;
    #[path = "../instructions/double.rs"]
    mod double;
    #[path = "../instructions/exception.rs"]
    mod exception;
    #[path = "../instructions/field.rs"]
    mod field;
    #[path = "../instructions/float.rs"]
    mod float;
    #[path = "../instructions/integer.rs"]
    mod integer;
    #[path = "../instructions/ldc.rs"]
    mod ldc;
    #[path = "../instructions/long.rs"]
    mod long;
    #[path = "../instructions/monitor.rs"]
    mod monitor;
    #[path = "../instructions/nop.rs"]
    mod nop;
    #[path = "../instructions/object.rs"]
    mod object;
    #[path = "../instructions/push.rs"]
    mod push;
    #[path = "../instructions/short.rs"]
    mod short;
    #[path = "../instructions/stack.rs"]
    mod stack;
}
