use crate::arguments::Arguments;
use crate::native_methods::java;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register optimization methods for `java.lang.Math`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Math";
    registry.register(class_name, "IEEEremainder", "(DD)D", ieee_remainder);
    registry.register(class_name, "abs", "(D)D", abs_d);
    registry.register(class_name, "abs", "(F)F", abs_f);
    registry.register(class_name, "abs", "(I)I", abs_i);
    registry.register(class_name, "abs", "(J)J", abs_j);
    registry.register(class_name, "absExact", "(I)I", abs_exact_i);
    registry.register(class_name, "absExact", "(J)J", abs_exact_j);
    registry.register(class_name, "acos", "(D)D", acos);
    registry.register(class_name, "addExact", "(II)I", add_exact_i);
    registry.register(class_name, "addExact", "(JJ)J", add_exact_j);
    registry.register(class_name, "asin", "(D)D", asin);
    registry.register(class_name, "atan", "(D)D", atan);
    registry.register(class_name, "atan2", "(DD)D", atan_2);
    registry.register(class_name, "cbrt", "(D)D", cbrt);
    registry.register(class_name, "ceil", "(D)D", ceil);
    registry.register(class_name, "ceilDiv", "(II)I", ceil_div_i);
    registry.register(class_name, "ceilDiv", "(JI)J", ceil_div_j_1);
    registry.register(class_name, "ceilDiv", "(JJ)J", ceil_div_j_2);
    registry.register(class_name, "ceilDivExact", "(II)I", ceil_div_exact_i);
    registry.register(class_name, "ceilDivExact", "(JJ)J", ceil_div_exact_j);
    registry.register(class_name, "ceilMod", "(II)I", ceil_mod_i);
    registry.register(class_name, "ceilMod", "(JI)I", ceil_mod_j_1);
    registry.register(class_name, "ceilMod", "(JJ)J", ceil_mod_j_2);
    registry.register(class_name, "clamp", "(DDD)D", clamp_d);
    registry.register(class_name, "clamp", "(FFF)F", clamp_f);
    registry.register(class_name, "clamp", "(JII)I", clamp_i);
    registry.register(class_name, "clamp", "(JJJ)J", clamp_j);
    registry.register(class_name, "copySign", "(DD)D", copy_sign_d);
    registry.register(class_name, "copySign", "(FF)F", copy_sign_f);
    registry.register(class_name, "cos", "(D)D", cos);
    registry.register(class_name, "cosh", "(D)D", cosh);
    registry.register(class_name, "decrementExact", "(I)I", decrement_exact_i);
    registry.register(class_name, "decrementExact", "(J)J", decrement_exact_j);
    registry.register(class_name, "divideExact", "(II)I", divide_exact_i);
    registry.register(class_name, "divideExact", "(JJ)J", divide_exact_j);
    registry.register(class_name, "exp", "(D)D", exp);
    registry.register(class_name, "expm1", "(D)D", expm_1);
    registry.register(class_name, "floor", "(D)D", floor);
    registry.register(class_name, "floorDiv", "(II)I", floor_div_i);
    registry.register(class_name, "floorDiv", "(JI)J", floor_div_j_1);
    registry.register(class_name, "floorDiv", "(JJ)J", floor_div_j_2);
    registry.register(class_name, "floorDivExact", "(II)I", floor_div_exact_i);
    registry.register(class_name, "floorDivExact", "(JJ)J", floor_div_exact_j);
    registry.register(class_name, "floorMod", "(II)I", floor_mod_i);
    registry.register(class_name, "floorMod", "(JI)I", floor_mod_j_1);
    registry.register(class_name, "floorMod", "(JJ)J", floor_mod_j_2);
    registry.register(class_name, "fma", "(DDD)D", fma_d);
    registry.register(class_name, "fma", "(FFF)F", fma_f);
    registry.register(class_name, "getExponent", "(D)I", get_exponent_d);
    registry.register(class_name, "getExponent", "(F)I", get_exponent_f);
    registry.register(class_name, "hypot", "(DD)D", hypot);
    registry.register(class_name, "incrementExact", "(I)I", increment_exact_i);
    registry.register(class_name, "incrementExact", "(J)J", increment_exact_j);
    registry.register(class_name, "log", "(D)D", log);
    registry.register(class_name, "log10", "(D)D", log_10);
    registry.register(class_name, "log1p", "(D)D", log_1p);
    registry.register(class_name, "max", "(DD)D", max_d);
    registry.register(class_name, "max", "(FF)F", max_f);
    registry.register(class_name, "max", "(II)I", max_i);
    registry.register(class_name, "max", "(JJ)J", max_j);
    registry.register(class_name, "min", "(DD)D", min_d);
    registry.register(class_name, "min", "(FF)F", min_f);
    registry.register(class_name, "min", "(II)I", min_i);
    registry.register(class_name, "min", "(JJ)J", min_j);
    registry.register(class_name, "multiplyExact", "(II)I", multiply_exact_i);
    registry.register(class_name, "multiplyExact", "(JI)J", multiply_exact_j_1);
    registry.register(class_name, "multiplyExact", "(JJ)J", multiply_exact_j_2);
    registry.register(class_name, "multiplyFull", "(II)J", multiply_full);
    registry.register(class_name, "multiplyHigh", "(JJ)J", multiply_high);
    registry.register(class_name, "negateExact", "(I)I", negate_exact_i);
    registry.register(class_name, "negateExact", "(J)J", negate_exact_j);
    registry.register(class_name, "nextAfter", "(DD)D", next_after_d);
    registry.register(class_name, "nextAfter", "(FD)F", next_after_f);
    registry.register(class_name, "nextDown", "(D)D", next_down_d);
    registry.register(class_name, "nextDown", "(F)F", next_down_f);
    registry.register(class_name, "nextUp", "(D)D", next_up_d);
    registry.register(class_name, "nextUp", "(F)F", next_up_f);
    registry.register(class_name, "pow", "(DD)D", pow);
    registry.register(class_name, "powerOfTwoD", "(I)D", power_of_two_d);
    registry.register(class_name, "powerOfTwoF", "(I)F", power_of_two_f);
    registry.register(class_name, "random", "()D", random);
    registry.register(class_name, "rint", "(D)D", rint);
    registry.register(class_name, "round", "(D)J", round_d);
    registry.register(class_name, "round", "(F)I", round_f);
    registry.register(class_name, "scalb", "(DI)D", scalb_d);
    registry.register(class_name, "scalb", "(FI)F", scalb_f);
    registry.register(class_name, "signum", "(D)D", signum_d);
    registry.register(class_name, "signum", "(F)F", signum_f);
    registry.register(class_name, "sin", "(D)D", sin);
    registry.register(class_name, "sinh", "(D)D", sinh);
    registry.register(class_name, "sqrt", "(D)D", sqrt);
    registry.register(class_name, "subtractExact", "(II)I", subtract_exact_i);
    registry.register(class_name, "subtractExact", "(JJ)J", subtract_exact_j);
    registry.register(class_name, "tan", "(D)D", tan);
    registry.register(class_name, "tanh", "(D)D", tanh);
    registry.register(class_name, "toDegrees", "(D)D", to_degrees);
    registry.register(class_name, "toIntExact", "(J)I", to_int_exact);
    registry.register(class_name, "toRadians", "(D)D", to_radians);
    registry.register(class_name, "ulp", "(D)D", ulp_d);
    registry.register(class_name, "ulp", "(F)F", ulp_f);
    registry.register(
        class_name,
        "unsignedMultiplyHigh",
        "(JJ)J",
        unsigned_multiply_high,
    );
}

#[async_recursion(?Send)]
async fn ieee_remainder(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::ieee_remainder(thread, arguments).await
}

#[async_recursion(?Send)]
async fn abs_d(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::abs_d(thread, arguments).await
}

#[async_recursion(?Send)]
async fn abs_f(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::abs_f(thread, arguments).await
}

#[async_recursion(?Send)]
async fn abs_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::abs_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn abs_j(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::abs_j(thread, arguments).await
}

#[async_recursion(?Send)]
async fn abs_exact_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::abs_exact_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn abs_exact_j(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::abs_exact_j(thread, arguments).await
}

#[async_recursion(?Send)]
async fn acos(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::acos(thread, arguments).await
}

#[async_recursion(?Send)]
async fn add_exact_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::add_exact_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn add_exact_j(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::add_exact_j(thread, arguments).await
}

#[async_recursion(?Send)]
async fn asin(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::asin(thread, arguments).await
}

#[async_recursion(?Send)]
async fn atan(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::atan(thread, arguments).await
}

#[async_recursion(?Send)]
async fn atan_2(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::atan_2(thread, arguments).await
}

#[async_recursion(?Send)]
async fn cbrt(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::cbrt(thread, arguments).await
}

#[async_recursion(?Send)]
async fn ceil(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::ceil(thread, arguments).await
}

#[async_recursion(?Send)]
async fn ceil_div_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_div_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn ceil_div_j_1(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_div_j_1(thread, arguments).await
}

#[async_recursion(?Send)]
async fn ceil_div_j_2(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_div_j_2(thread, arguments).await
}

#[async_recursion(?Send)]
async fn ceil_div_exact_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_div_exact_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn ceil_div_exact_j(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_div_exact_j(thread, arguments).await
}

#[async_recursion(?Send)]
async fn ceil_mod_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_mod_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn ceil_mod_j_1(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_mod_j_1(thread, arguments).await
}

#[async_recursion(?Send)]
async fn ceil_mod_j_2(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_mod_j_2(thread, arguments).await
}

#[async_recursion(?Send)]
async fn clamp_d(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::clamp_d(thread, arguments).await
}

#[async_recursion(?Send)]
async fn clamp_f(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::clamp_f(thread, arguments).await
}

#[async_recursion(?Send)]
async fn clamp_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::clamp_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn clamp_j(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::clamp_j(thread, arguments).await
}

#[async_recursion(?Send)]
async fn copy_sign_d(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::copy_sign_d(thread, arguments).await
}

#[async_recursion(?Send)]
async fn copy_sign_f(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::copy_sign_f(thread, arguments).await
}

#[async_recursion(?Send)]
async fn cos(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::cos(thread, arguments).await
}

#[async_recursion(?Send)]
async fn cosh(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::cosh(thread, arguments).await
}

#[async_recursion(?Send)]
async fn decrement_exact_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::decrement_exact_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn decrement_exact_j(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::decrement_exact_j(thread, arguments).await
}

#[async_recursion(?Send)]
async fn divide_exact_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::divide_exact_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn divide_exact_j(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::divide_exact_j(thread, arguments).await
}

#[async_recursion(?Send)]
async fn exp(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::exp(thread, arguments).await
}

#[async_recursion(?Send)]
async fn expm_1(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::expm_1(thread, arguments).await
}

#[async_recursion(?Send)]
async fn floor(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::floor(thread, arguments).await
}

#[async_recursion(?Send)]
async fn floor_div_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::floor_div_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn floor_div_j_1(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::floor_div_j_1(thread, arguments).await
}

#[async_recursion(?Send)]
async fn floor_div_j_2(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::floor_div_j_2(thread, arguments).await
}

#[async_recursion(?Send)]
async fn floor_div_exact_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::floor_div_exact_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn floor_div_exact_j(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::floor_div_exact_j(thread, arguments).await
}

#[async_recursion(?Send)]
async fn floor_mod_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::floor_mod_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn floor_mod_j_1(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::floor_mod_j_1(thread, arguments).await
}

#[async_recursion(?Send)]
async fn floor_mod_j_2(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::floor_mod_j_2(thread, arguments).await
}

#[async_recursion(?Send)]
async fn fma_d(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::fma_d(thread, arguments).await
}

#[async_recursion(?Send)]
async fn fma_f(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::fma_f(thread, arguments).await
}

#[async_recursion(?Send)]
async fn get_exponent_d(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::get_exponent_d(thread, arguments).await
}

#[async_recursion(?Send)]
async fn get_exponent_f(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::get_exponent_f(thread, arguments).await
}

#[async_recursion(?Send)]
async fn hypot(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::hypot(thread, arguments).await
}

#[async_recursion(?Send)]
async fn increment_exact_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::increment_exact_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn increment_exact_j(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::increment_exact_j(thread, arguments).await
}

#[async_recursion(?Send)]
async fn log(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::log(thread, arguments).await
}

#[async_recursion(?Send)]
async fn log_10(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::log_10(thread, arguments).await
}

#[async_recursion(?Send)]
async fn log_1p(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::log_1p(thread, arguments).await
}

#[async_recursion(?Send)]
async fn max_d(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::max_d(thread, arguments).await
}

#[async_recursion(?Send)]
async fn max_f(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::max_f(thread, arguments).await
}

#[async_recursion(?Send)]
async fn max_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::max_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn max_j(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::max_j(thread, arguments).await
}

#[async_recursion(?Send)]
async fn min_d(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::min_d(thread, arguments).await
}

#[async_recursion(?Send)]
async fn min_f(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::min_f(thread, arguments).await
}

#[async_recursion(?Send)]
async fn min_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::min_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn min_j(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::min_j(thread, arguments).await
}

#[async_recursion(?Send)]
async fn multiply_exact_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::multiply_exact_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn multiply_exact_j_1(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::multiply_exact_j_1(thread, arguments).await
}

#[async_recursion(?Send)]
async fn multiply_exact_j_2(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::multiply_exact_j_2(thread, arguments).await
}

#[async_recursion(?Send)]
async fn multiply_full(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::multiply_full(thread, arguments).await
}

#[async_recursion(?Send)]
async fn multiply_high(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::multiply_high(thread, arguments).await
}

#[async_recursion(?Send)]
async fn negate_exact_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::negate_exact_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn negate_exact_j(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::negate_exact_j(thread, arguments).await
}

#[async_recursion(?Send)]
async fn next_after_d(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::next_after_d(thread, arguments).await
}

#[async_recursion(?Send)]
async fn next_after_f(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::next_after_f(thread, arguments).await
}

#[async_recursion(?Send)]
async fn next_down_d(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::next_down_d(thread, arguments).await
}

#[async_recursion(?Send)]
async fn next_down_f(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::next_down_f(thread, arguments).await
}

#[async_recursion(?Send)]
async fn next_up_d(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::next_up_d(thread, arguments).await
}

#[async_recursion(?Send)]
async fn next_up_f(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::next_up_f(thread, arguments).await
}

#[async_recursion(?Send)]
async fn pow(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::pow(thread, arguments).await
}

#[async_recursion(?Send)]
async fn power_of_two_d(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let n = arguments.pop_int()?;
    let result = 2f64.powi(n);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
async fn power_of_two_f(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let n = arguments.pop_int()?;
    let result = 2f32.powi(n);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
async fn random(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::random(thread, arguments).await
}

#[async_recursion(?Send)]
async fn rint(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::rint(thread, arguments).await
}

#[async_recursion(?Send)]
async fn round_d(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::round_d(thread, arguments).await
}

#[async_recursion(?Send)]
async fn round_f(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::round_f(thread, arguments).await
}

#[async_recursion(?Send)]
async fn scalb_d(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::scalb_d(thread, arguments).await
}

#[async_recursion(?Send)]
async fn scalb_f(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::scalb_f(thread, arguments).await
}

#[async_recursion(?Send)]
async fn signum_d(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::signum_d(thread, arguments).await
}

#[async_recursion(?Send)]
async fn signum_f(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::signum_f(thread, arguments).await
}

#[async_recursion(?Send)]
async fn sin(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::sin(thread, arguments).await
}

#[async_recursion(?Send)]
async fn sinh(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::sinh(thread, arguments).await
}

#[async_recursion(?Send)]
async fn sqrt(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::sqrt(thread, arguments).await
}

#[async_recursion(?Send)]
async fn subtract_exact_i(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::subtract_exact_i(thread, arguments).await
}

#[async_recursion(?Send)]
async fn subtract_exact_j(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::subtract_exact_j(thread, arguments).await
}

#[async_recursion(?Send)]
async fn tan(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::tan(thread, arguments).await
}

#[async_recursion(?Send)]
async fn tanh(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::tanh(thread, arguments).await
}

#[async_recursion(?Send)]
async fn to_degrees(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::to_degrees(thread, arguments).await
}

#[async_recursion(?Send)]
async fn to_int_exact(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::to_int_exact(thread, arguments).await
}

#[async_recursion(?Send)]
async fn to_radians(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::to_radians(thread, arguments).await
}

#[async_recursion(?Send)]
async fn ulp_d(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::ulp_d(thread, arguments).await
}

#[async_recursion(?Send)]
async fn ulp_f(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    java::lang::strictmath::ulp_f(thread, arguments).await
}

#[async_recursion(?Send)]
async fn unsigned_multiply_high(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    java::lang::strictmath::unsigned_multiply_high(thread, arguments).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[expect(clippy::too_many_lines)]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/Math";
        assert!(registry
            .method(class_name, "IEEEremainder", "(DD)D")
            .is_some());
        assert!(registry.method(class_name, "abs", "(D)D").is_some());
        assert!(registry.method(class_name, "abs", "(F)F").is_some());
        assert!(registry.method(class_name, "abs", "(I)I").is_some());
        assert!(registry.method(class_name, "abs", "(J)J").is_some());
        assert!(registry.method(class_name, "absExact", "(I)I").is_some());
        assert!(registry.method(class_name, "absExact", "(J)J").is_some());
        assert!(registry.method(class_name, "acos", "(D)D").is_some());
        assert!(registry.method(class_name, "addExact", "(II)I").is_some());
        assert!(registry.method(class_name, "addExact", "(JJ)J").is_some());
        assert!(registry.method(class_name, "asin", "(D)D").is_some());
        assert!(registry.method(class_name, "atan", "(D)D").is_some());
        assert!(registry.method(class_name, "atan2", "(DD)D").is_some());
        assert!(registry.method(class_name, "cbrt", "(D)D").is_some());
        assert!(registry.method(class_name, "ceil", "(D)D").is_some());
        assert!(registry.method(class_name, "ceilDiv", "(II)I").is_some());
        assert!(registry.method(class_name, "ceilDiv", "(JI)J").is_some());
        assert!(registry.method(class_name, "ceilDiv", "(JJ)J").is_some());
        assert!(registry
            .method(class_name, "ceilDivExact", "(II)I")
            .is_some());
        assert!(registry
            .method(class_name, "ceilDivExact", "(JJ)J")
            .is_some());
        assert!(registry.method(class_name, "ceilMod", "(II)I").is_some());
        assert!(registry.method(class_name, "ceilMod", "(JI)I").is_some());
        assert!(registry.method(class_name, "ceilMod", "(JJ)J").is_some());
        assert!(registry.method(class_name, "clamp", "(DDD)D").is_some());
        assert!(registry.method(class_name, "clamp", "(FFF)F").is_some());
        assert!(registry.method(class_name, "clamp", "(JII)I").is_some());
        assert!(registry.method(class_name, "clamp", "(JJJ)J").is_some());
        assert!(registry.method(class_name, "copySign", "(DD)D").is_some());
        assert!(registry.method(class_name, "copySign", "(FF)F").is_some());
        assert!(registry.method(class_name, "cos", "(D)D").is_some());
        assert!(registry.method(class_name, "cosh", "(D)D").is_some());
        assert!(registry
            .method(class_name, "decrementExact", "(I)I")
            .is_some());
        assert!(registry
            .method(class_name, "decrementExact", "(J)J")
            .is_some());
        assert!(registry
            .method(class_name, "divideExact", "(II)I")
            .is_some());
        assert!(registry
            .method(class_name, "divideExact", "(JJ)J")
            .is_some());
        assert!(registry.method(class_name, "exp", "(D)D").is_some());
        assert!(registry.method(class_name, "expm1", "(D)D").is_some());
        assert!(registry.method(class_name, "floor", "(D)D").is_some());
        assert!(registry.method(class_name, "floorDiv", "(II)I").is_some());
        assert!(registry.method(class_name, "floorDiv", "(JI)J").is_some());
        assert!(registry.method(class_name, "floorDiv", "(JJ)J").is_some());
        assert!(registry
            .method(class_name, "floorDivExact", "(II)I")
            .is_some());
        assert!(registry
            .method(class_name, "floorDivExact", "(JJ)J")
            .is_some());
        assert!(registry.method(class_name, "floorMod", "(II)I").is_some());
        assert!(registry.method(class_name, "floorMod", "(JI)I").is_some());
        assert!(registry.method(class_name, "floorMod", "(JJ)J").is_some());
        assert!(registry.method(class_name, "fma", "(DDD)D").is_some());
        assert!(registry.method(class_name, "fma", "(FFF)F").is_some());
        assert!(registry.method(class_name, "getExponent", "(D)I").is_some());
        assert!(registry.method(class_name, "getExponent", "(F)I").is_some());
        assert!(registry.method(class_name, "hypot", "(DD)D").is_some());
        assert!(registry
            .method(class_name, "incrementExact", "(I)I")
            .is_some());
        assert!(registry
            .method(class_name, "incrementExact", "(J)J")
            .is_some());
        assert!(registry.method(class_name, "log", "(D)D").is_some());
        assert!(registry.method(class_name, "log10", "(D)D").is_some());
        assert!(registry.method(class_name, "log1p", "(D)D").is_some());
        assert!(registry.method(class_name, "max", "(DD)D").is_some());
        assert!(registry.method(class_name, "max", "(FF)F").is_some());
        assert!(registry.method(class_name, "max", "(II)I").is_some());
        assert!(registry.method(class_name, "max", "(JJ)J").is_some());
        assert!(registry.method(class_name, "min", "(DD)D").is_some());
        assert!(registry.method(class_name, "min", "(FF)F").is_some());
        assert!(registry.method(class_name, "min", "(II)I").is_some());
        assert!(registry.method(class_name, "min", "(JJ)J").is_some());
        assert!(registry
            .method(class_name, "multiplyExact", "(II)I")
            .is_some());
        assert!(registry
            .method(class_name, "multiplyExact", "(JI)J")
            .is_some());
        assert!(registry
            .method(class_name, "multiplyExact", "(JJ)J")
            .is_some());
        assert!(registry
            .method(class_name, "multiplyFull", "(II)J")
            .is_some());
        assert!(registry
            .method(class_name, "multiplyHigh", "(JJ)J")
            .is_some());
        assert!(registry.method(class_name, "negateExact", "(I)I").is_some());
        assert!(registry.method(class_name, "negateExact", "(J)J").is_some());
        assert!(registry.method(class_name, "nextAfter", "(DD)D").is_some());
        assert!(registry.method(class_name, "nextAfter", "(FD)F").is_some());
        assert!(registry.method(class_name, "nextDown", "(D)D").is_some());
        assert!(registry.method(class_name, "nextDown", "(F)F").is_some());
        assert!(registry.method(class_name, "nextUp", "(D)D").is_some());
        assert!(registry.method(class_name, "nextUp", "(F)F").is_some());
        assert!(registry.method(class_name, "pow", "(DD)D").is_some());
        assert!(registry.method(class_name, "powerOfTwoD", "(I)D").is_some());
        assert!(registry.method(class_name, "powerOfTwoF", "(I)F").is_some());
        assert!(registry.method(class_name, "random", "()D").is_some());
        assert!(registry.method(class_name, "rint", "(D)D").is_some());
        assert!(registry.method(class_name, "round", "(D)J").is_some());
        assert!(registry.method(class_name, "round", "(F)I").is_some());
        assert!(registry.method(class_name, "scalb", "(DI)D").is_some());
        assert!(registry.method(class_name, "scalb", "(FI)F").is_some());
        assert!(registry.method(class_name, "signum", "(D)D").is_some());
        assert!(registry.method(class_name, "signum", "(F)F").is_some());
        assert!(registry.method(class_name, "sin", "(D)D").is_some());
        assert!(registry.method(class_name, "sinh", "(D)D").is_some());
        assert!(registry.method(class_name, "sqrt", "(D)D").is_some());
        assert!(registry
            .method(class_name, "subtractExact", "(II)I")
            .is_some());
        assert!(registry
            .method(class_name, "subtractExact", "(JJ)J")
            .is_some());
        assert!(registry.method(class_name, "tan", "(D)D").is_some());
        assert!(registry.method(class_name, "tanh", "(D)D").is_some());
        assert!(registry.method(class_name, "toDegrees", "(D)D").is_some());
        assert!(registry.method(class_name, "toIntExact", "(J)I").is_some());
        assert!(registry.method(class_name, "toRadians", "(D)D").is_some());
        assert!(registry.method(class_name, "ulp", "(D)D").is_some());
        assert!(registry.method(class_name, "ulp", "(F)F").is_some());
        assert!(registry
            .method(class_name, "unsignedMultiplyHigh", "(JJ)J")
            .is_some());
    }

    #[tokio::test]
    async fn test_ieee_remainder() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.0), Value::Double(2.0)]);
        let result = ieee_remainder(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_abs_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(-1.0)]);
        let result = abs_d(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_abs_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Float(-1.0)]);
        let result = abs_f(thread, arguments).await?;
        assert_eq!(result, Some(Value::Float(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_abs_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(-1)]);
        let result = abs_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_abs_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(-1)]);
        let result = abs_j(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_abs_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(-1)]);
        let result = abs_exact_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_abs_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(-1)]);
        let result = abs_exact_j(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_acos() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(0.5)]);
        let result = acos(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 1.047f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_add_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(1), Value::Int(2)]);
        let result = add_exact_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(3)));
        Ok(())
    }

    #[tokio::test]
    async fn test_add_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(1), Value::Long(2)]);
        let result = add_exact_j(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(3)));
        Ok(())
    }

    #[tokio::test]
    async fn test_asin() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(0.5)]);
        let result = asin(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.523f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_atan() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(0.5)]);
        let result = atan(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.463f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_atan_2() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.0), Value::Double(1.0)]);
        let result = atan_2(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.785f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_cbrt() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(8.0)]);
        let result = cbrt(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.5)]);
        let result = ceil(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_div_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(5), Value::Int(2)]);
        let result = ceil_div_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(3)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_div_j_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(5), Value::Int(2)]);
        let result = ceil_div_j_1(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(3)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_div_j_2() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(5), Value::Long(2)]);
        let result = ceil_div_j_2(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(3)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_div_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(5), Value::Int(2)]);
        let result = ceil_div_exact_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(3)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_div_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(5), Value::Long(2)]);
        let result = ceil_div_exact_j(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(3)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_mod_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(5), Value::Int(2)]);
        let result = ceil_mod_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_mod_j_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(5), Value::Int(2)]);
        let result = ceil_mod_j_1(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_mod_j_2() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(5), Value::Long(2)]);
        let result = ceil_mod_j_2(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_clamp_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![
            Value::Double(1.0),
            Value::Double(2.0),
            Value::Double(3.0),
        ]);
        let result = clamp_d(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_clamp_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![
            Value::Float(1.0),
            Value::Float(2.0),
            Value::Float(3.0),
        ]);
        let result = clamp_f(thread, arguments).await?;
        assert_eq!(result, Some(Value::Float(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_clamp_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(1), Value::Int(2), Value::Int(3)]);
        let result = clamp_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_clamp_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(1), Value::Long(2), Value::Long(3)]);
        let result = clamp_j(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_sign_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(-1.0), Value::Double(2.0)]);
        let result = copy_sign_d(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_sign_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Float(-1.0), Value::Float(2.0)]);
        let result = copy_sign_f(thread, arguments).await?;
        assert_eq!(result, Some(Value::Float(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_cos() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(0.5)]);
        let result = cos(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.877f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_cosh() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(0.5)]);
        let result = cosh(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 1.127f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_decrement_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(1)]);
        let result = decrement_exact_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_decrement_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(1)]);
        let result = decrement_exact_j(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_divide_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(5), Value::Int(2)]);
        let result = divide_exact_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_divide_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(5), Value::Long(2)]);
        let result = divide_exact_j(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_exp() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.0)]);
        let result = exp(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - std::f64::consts::E;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_expm_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.0)]);
        let result = expm_1(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 1.718f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_floor() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.5)]);
        let result = floor(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_div_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(5), Value::Int(2)]);
        let result = floor_div_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_div_j_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(5), Value::Int(2)]);
        let result = floor_div_j_1(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_div_j_2() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(5), Value::Long(2)]);
        let result = floor_div_j_2(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_div_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(5), Value::Int(2)]);
        let result = floor_div_exact_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_div_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(5), Value::Long(2)]);
        let result = floor_div_exact_j(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_mod_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(5), Value::Int(2)]);
        let result = floor_mod_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_mod_j_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(5), Value::Int(2)]);
        let result = floor_mod_j_1(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_mod_j_2() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(5), Value::Long(2)]);
        let result = floor_mod_j_2(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_fma_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![
            Value::Double(1.0),
            Value::Double(2.0),
            Value::Double(3.0),
        ]);
        let result = fma_d(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(5.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_fma_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![
            Value::Float(1.0),
            Value::Float(2.0),
            Value::Float(3.0),
        ]);
        let result = fma_f(thread, arguments).await?;
        assert_eq!(result, Some(Value::Float(5.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_exponent_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.0)]);
        let result = get_exponent_d(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_exponent_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Float(1.0)]);
        let result = get_exponent_f(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_hypot() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(3.0), Value::Double(4.0)]);
        let result = hypot(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(5.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_increment_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(1)]);
        let result = increment_exact_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_increment_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(1)]);
        let result = increment_exact_j(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_log() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.0)]);
        let result = log(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(0.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_log_10() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.0)]);
        let result = log_10(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(0.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_log_1p() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.0)]);
        let result = log_1p(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.693f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_max_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.0), Value::Double(2.0)]);
        let result = max_d(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_max_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Float(1.0), Value::Float(2.0)]);
        let result = max_f(thread, arguments).await?;
        assert_eq!(result, Some(Value::Float(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_max_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(1), Value::Int(2)]);
        let result = max_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_max_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(1), Value::Long(2)]);
        let result = max_j(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_min_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.0), Value::Double(2.0)]);
        let result = min_d(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_min_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Float(1.0), Value::Float(2.0)]);
        let result = min_f(thread, arguments).await?;
        assert_eq!(result, Some(Value::Float(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_min_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(1), Value::Int(2)]);
        let result = min_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_min_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(1), Value::Long(2)]);
        let result = min_j(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_multiply_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(2), Value::Int(3)]);
        let result = multiply_exact_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(6)));
        Ok(())
    }

    #[tokio::test]
    async fn test_multiply_exact_j_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(2), Value::Int(3)]);
        let result = multiply_exact_j_1(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(6)));
        Ok(())
    }

    #[tokio::test]
    async fn test_multiply_exact_j_2() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(2), Value::Long(3)]);
        let result = multiply_exact_j_2(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(6)));
        Ok(())
    }

    #[tokio::test]
    async fn test_multiply_full() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(2), Value::Int(3)]);
        let result = multiply_full(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(6)));
        Ok(())
    }

    #[tokio::test]
    async fn test_multiply_high() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(2), Value::Long(3)]);
        let result = multiply_high(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_negate_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(1)]);
        let result = negate_exact_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(-1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_negate_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(1)]);
        let result = negate_exact_j(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(-1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_next_after_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.0), Value::Double(2.0)]);
        let result = next_after_d(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 1.000f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_next_after_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Float(1.0), Value::Double(2.0)]);
        let result = next_after_f(thread, arguments).await?;
        let value = result.unwrap_or(Value::Float(0.0)).to_float()? - 1.000f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_next_down_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.0)]);
        let result = next_down_d(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 1.000f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_next_down_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Float(1.0)]);
        let result = next_down_f(thread, arguments).await?;
        let value = result.unwrap_or(Value::Float(0.0)).to_float()? - 1.000f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_next_up_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.0)]);
        let result = next_up_d(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 1.000f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_next_up_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Float(1.0)]);
        let result = next_up_f(thread, arguments).await?;
        let value = result.unwrap_or(Value::Float(0.0)).to_float()? - 1.000f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_pow() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(2.0), Value::Double(3.0)]);
        let result = pow(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(8.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_power_of_two_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(2)]);
        let result = power_of_two_d(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(4.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_power_of_two_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(2)]);
        let result = power_of_two_f(thread, arguments).await?;
        assert_eq!(result, Some(Value::Float(4.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_random() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![]);
        let result = random(thread, arguments).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_rint() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.5)]);
        let result = rint(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_round_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.5)]);
        let result = round_d(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_round_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Float(1.5)]);
        let result = round_f(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_scalb_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.0), Value::Int(2)]);
        let result = scalb_d(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(4.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_scalb_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Float(1.0), Value::Int(2)]);
        let result = scalb_f(thread, arguments).await?;
        assert_eq!(result, Some(Value::Float(4.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_signum_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(-1.0)]);
        let result = signum_d(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(-1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_signum_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Float(-1.0)]);
        let result = signum_f(thread, arguments).await?;
        assert_eq!(result, Some(Value::Float(-1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_sin() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(0.5)]);
        let result = sin(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.479f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_sinh() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(0.5)]);
        let result = sinh(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.521f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_sqrt() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(4.0)]);
        let result = sqrt(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_subtract_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Int(3), Value::Int(2)]);
        let result = subtract_exact_i(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_subtract_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(3), Value::Long(2)]);
        let result = subtract_exact_j(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_tan() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(0.5)]);
        let result = tan(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.546f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_tanh() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(0.5)]);
        let result = tanh(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.462f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_to_degrees() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(std::f64::consts::PI)]);
        let result = to_degrees(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(180.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_to_int_exact() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(1)]);
        let result = to_int_exact(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_to_radians() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(180.0)]);
        let result = to_radians(thread, arguments).await?;
        assert_eq!(result, Some(Value::Double(std::f64::consts::PI)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ulp_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Double(1.0)]);
        let result = ulp_d(thread, arguments).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()?;
        assert!(value > 0.0f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_ulp_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Float(1.0)]);
        let result = ulp_f(thread, arguments).await?;
        let value = result.unwrap_or(Value::Float(0.0)).to_float()?;
        assert!(value > 0.0f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_unsigned_multiply_high() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(2), Value::Long(3)]);
        let result = unsigned_multiply_high(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }
}
