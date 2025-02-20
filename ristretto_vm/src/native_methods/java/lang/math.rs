use crate::Result;
use crate::native_methods::java;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/Math";

/// Register optimization methods for `java.lang.Math`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "IEEEremainder", "(DD)D", ieee_remainder);
    registry.register(CLASS_NAME, "abs", "(D)D", abs_d);
    registry.register(CLASS_NAME, "abs", "(F)F", abs_f);
    registry.register(CLASS_NAME, "abs", "(I)I", abs_i);
    registry.register(CLASS_NAME, "abs", "(J)J", abs_j);
    registry.register(CLASS_NAME, "absExact", "(I)I", abs_exact_i);
    registry.register(CLASS_NAME, "absExact", "(J)J", abs_exact_j);
    registry.register(CLASS_NAME, "acos", "(D)D", acos);
    registry.register(CLASS_NAME, "addExact", "(II)I", add_exact_i);
    registry.register(CLASS_NAME, "addExact", "(JJ)J", add_exact_j);
    registry.register(CLASS_NAME, "asin", "(D)D", asin);
    registry.register(CLASS_NAME, "atan", "(D)D", atan);
    registry.register(CLASS_NAME, "atan2", "(DD)D", atan_2);
    registry.register(CLASS_NAME, "cbrt", "(D)D", cbrt);
    registry.register(CLASS_NAME, "ceil", "(D)D", ceil);
    registry.register(CLASS_NAME, "ceilDiv", "(II)I", ceil_div_i);
    registry.register(CLASS_NAME, "ceilDiv", "(JI)J", ceil_div_j_1);
    registry.register(CLASS_NAME, "ceilDiv", "(JJ)J", ceil_div_j_2);
    registry.register(CLASS_NAME, "ceilDivExact", "(II)I", ceil_div_exact_i);
    registry.register(CLASS_NAME, "ceilDivExact", "(JJ)J", ceil_div_exact_j);
    registry.register(CLASS_NAME, "ceilMod", "(II)I", ceil_mod_i);
    registry.register(CLASS_NAME, "ceilMod", "(JI)I", ceil_mod_j_1);
    registry.register(CLASS_NAME, "ceilMod", "(JJ)J", ceil_mod_j_2);
    registry.register(CLASS_NAME, "clamp", "(DDD)D", clamp_d);
    registry.register(CLASS_NAME, "clamp", "(FFF)F", clamp_f);
    registry.register(CLASS_NAME, "clamp", "(JII)I", clamp_i);
    registry.register(CLASS_NAME, "clamp", "(JJJ)J", clamp_j);
    registry.register(CLASS_NAME, "copySign", "(DD)D", copy_sign_d);
    registry.register(CLASS_NAME, "copySign", "(FF)F", copy_sign_f);
    registry.register(CLASS_NAME, "cos", "(D)D", cos);
    registry.register(CLASS_NAME, "cosh", "(D)D", cosh);
    registry.register(CLASS_NAME, "decrementExact", "(I)I", decrement_exact_i);
    registry.register(CLASS_NAME, "decrementExact", "(J)J", decrement_exact_j);
    registry.register(CLASS_NAME, "divideExact", "(II)I", divide_exact_i);
    registry.register(CLASS_NAME, "divideExact", "(JJ)J", divide_exact_j);
    registry.register(CLASS_NAME, "exp", "(D)D", exp);
    registry.register(CLASS_NAME, "expm1", "(D)D", expm_1);
    registry.register(CLASS_NAME, "floor", "(D)D", floor);
    registry.register(CLASS_NAME, "floorDiv", "(II)I", floor_div_i);
    registry.register(CLASS_NAME, "floorDiv", "(JI)J", floor_div_j_1);
    registry.register(CLASS_NAME, "floorDiv", "(JJ)J", floor_div_j_2);
    registry.register(CLASS_NAME, "floorDivExact", "(II)I", floor_div_exact_i);
    registry.register(CLASS_NAME, "floorDivExact", "(JJ)J", floor_div_exact_j);
    registry.register(CLASS_NAME, "floorMod", "(II)I", floor_mod_i);
    registry.register(CLASS_NAME, "floorMod", "(JI)I", floor_mod_j_1);
    registry.register(CLASS_NAME, "floorMod", "(JJ)J", floor_mod_j_2);
    registry.register(CLASS_NAME, "fma", "(DDD)D", fma_d);
    registry.register(CLASS_NAME, "fma", "(FFF)F", fma_f);
    registry.register(CLASS_NAME, "getExponent", "(D)I", get_exponent_d);
    registry.register(CLASS_NAME, "getExponent", "(F)I", get_exponent_f);
    registry.register(CLASS_NAME, "hypot", "(DD)D", hypot);
    registry.register(CLASS_NAME, "incrementExact", "(I)I", increment_exact_i);
    registry.register(CLASS_NAME, "incrementExact", "(J)J", increment_exact_j);
    registry.register(CLASS_NAME, "log", "(D)D", log);
    registry.register(CLASS_NAME, "log10", "(D)D", log_10);
    registry.register(CLASS_NAME, "log1p", "(D)D", log_1p);
    registry.register(CLASS_NAME, "max", "(DD)D", max_d);
    registry.register(CLASS_NAME, "max", "(FF)F", max_f);
    registry.register(CLASS_NAME, "max", "(II)I", max_i);
    registry.register(CLASS_NAME, "max", "(JJ)J", max_j);
    registry.register(CLASS_NAME, "min", "(DD)D", min_d);
    registry.register(CLASS_NAME, "min", "(FF)F", min_f);
    registry.register(CLASS_NAME, "min", "(II)I", min_i);
    registry.register(CLASS_NAME, "min", "(JJ)J", min_j);
    registry.register(CLASS_NAME, "multiplyExact", "(II)I", multiply_exact_i);
    registry.register(CLASS_NAME, "multiplyExact", "(JI)J", multiply_exact_j_1);
    registry.register(CLASS_NAME, "multiplyExact", "(JJ)J", multiply_exact_j_2);
    registry.register(CLASS_NAME, "multiplyFull", "(II)J", multiply_full);
    registry.register(CLASS_NAME, "multiplyHigh", "(JJ)J", multiply_high);
    registry.register(CLASS_NAME, "negateExact", "(I)I", negate_exact_i);
    registry.register(CLASS_NAME, "negateExact", "(J)J", negate_exact_j);
    registry.register(CLASS_NAME, "nextAfter", "(DD)D", next_after_d);
    registry.register(CLASS_NAME, "nextAfter", "(FD)F", next_after_f);
    registry.register(CLASS_NAME, "nextDown", "(D)D", next_down_d);
    registry.register(CLASS_NAME, "nextDown", "(F)F", next_down_f);
    registry.register(CLASS_NAME, "nextUp", "(D)D", next_up_d);
    registry.register(CLASS_NAME, "nextUp", "(F)F", next_up_f);
    registry.register(CLASS_NAME, "pow", "(DD)D", pow);
    registry.register(CLASS_NAME, "powerOfTwoD", "(I)D", power_of_two_d);
    registry.register(CLASS_NAME, "powerOfTwoF", "(I)F", power_of_two_f);
    registry.register(CLASS_NAME, "random", "()D", random);
    registry.register(CLASS_NAME, "rint", "(D)D", rint);
    registry.register(CLASS_NAME, "round", "(D)J", round_d);
    registry.register(CLASS_NAME, "round", "(F)I", round_f);
    registry.register(CLASS_NAME, "scalb", "(DI)D", scalb_d);
    registry.register(CLASS_NAME, "scalb", "(FI)F", scalb_f);
    registry.register(CLASS_NAME, "signum", "(D)D", signum_d);
    registry.register(CLASS_NAME, "signum", "(F)F", signum_f);
    registry.register(CLASS_NAME, "sin", "(D)D", sin);
    registry.register(CLASS_NAME, "sinh", "(D)D", sinh);
    registry.register(CLASS_NAME, "sqrt", "(D)D", sqrt);
    registry.register(CLASS_NAME, "subtractExact", "(II)I", subtract_exact_i);
    registry.register(CLASS_NAME, "subtractExact", "(JJ)J", subtract_exact_j);
    registry.register(CLASS_NAME, "tan", "(D)D", tan);
    registry.register(CLASS_NAME, "tanh", "(D)D", tanh);
    registry.register(CLASS_NAME, "toDegrees", "(D)D", to_degrees);
    registry.register(CLASS_NAME, "toIntExact", "(J)I", to_int_exact);
    registry.register(CLASS_NAME, "toRadians", "(D)D", to_radians);
    registry.register(CLASS_NAME, "ulp", "(D)D", ulp_d);
    registry.register(CLASS_NAME, "ulp", "(F)F", ulp_f);
    registry.register(
        CLASS_NAME,
        "unsignedMultiplyHigh",
        "(JJ)J",
        unsigned_multiply_high,
    );
}

#[async_recursion(?Send)]
async fn ieee_remainder(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::ieee_remainder(thread, parameters).await
}

#[async_recursion(?Send)]
async fn abs_d(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::abs_d(thread, parameters).await
}

#[async_recursion(?Send)]
async fn abs_f(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::abs_f(thread, parameters).await
}

#[async_recursion(?Send)]
async fn abs_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::abs_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn abs_j(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::abs_j(thread, parameters).await
}

#[async_recursion(?Send)]
async fn abs_exact_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::abs_exact_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn abs_exact_j(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::abs_exact_j(thread, parameters).await
}

#[async_recursion(?Send)]
async fn acos(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::acos(thread, parameters).await
}

#[async_recursion(?Send)]
async fn add_exact_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::add_exact_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn add_exact_j(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::add_exact_j(thread, parameters).await
}

#[async_recursion(?Send)]
async fn asin(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::asin(thread, parameters).await
}

#[async_recursion(?Send)]
async fn atan(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::atan(thread, parameters).await
}

#[async_recursion(?Send)]
async fn atan_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::atan_2(thread, parameters).await
}

#[async_recursion(?Send)]
async fn cbrt(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::cbrt(thread, parameters).await
}

#[async_recursion(?Send)]
async fn ceil(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::ceil(thread, parameters).await
}

#[async_recursion(?Send)]
async fn ceil_div_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_div_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn ceil_div_j_1(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_div_j_1(thread, parameters).await
}

#[async_recursion(?Send)]
async fn ceil_div_j_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_div_j_2(thread, parameters).await
}

#[async_recursion(?Send)]
async fn ceil_div_exact_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_div_exact_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn ceil_div_exact_j(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_div_exact_j(thread, parameters).await
}

#[async_recursion(?Send)]
async fn ceil_mod_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_mod_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn ceil_mod_j_1(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_mod_j_1(thread, parameters).await
}

#[async_recursion(?Send)]
async fn ceil_mod_j_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::ceil_mod_j_2(thread, parameters).await
}

#[async_recursion(?Send)]
async fn clamp_d(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::clamp_d(thread, parameters).await
}

#[async_recursion(?Send)]
async fn clamp_f(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::clamp_f(thread, parameters).await
}

#[async_recursion(?Send)]
async fn clamp_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::clamp_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn clamp_j(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::clamp_j(thread, parameters).await
}

#[async_recursion(?Send)]
async fn copy_sign_d(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::copy_sign_d(thread, parameters).await
}

#[async_recursion(?Send)]
async fn copy_sign_f(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::copy_sign_f(thread, parameters).await
}

#[async_recursion(?Send)]
async fn cos(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::cos(thread, parameters).await
}

#[async_recursion(?Send)]
async fn cosh(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::cosh(thread, parameters).await
}

#[async_recursion(?Send)]
async fn decrement_exact_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::decrement_exact_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn decrement_exact_j(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::decrement_exact_j(thread, parameters).await
}

#[async_recursion(?Send)]
async fn divide_exact_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::divide_exact_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn divide_exact_j(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::divide_exact_j(thread, parameters).await
}

#[async_recursion(?Send)]
async fn exp(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::exp(thread, parameters).await
}

#[async_recursion(?Send)]
async fn expm_1(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::expm_1(thread, parameters).await
}

#[async_recursion(?Send)]
async fn floor(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::floor(thread, parameters).await
}

#[async_recursion(?Send)]
async fn floor_div_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::floor_div_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn floor_div_j_1(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::floor_div_j_1(thread, parameters).await
}

#[async_recursion(?Send)]
async fn floor_div_j_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::floor_div_j_2(thread, parameters).await
}

#[async_recursion(?Send)]
async fn floor_div_exact_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::floor_div_exact_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn floor_div_exact_j(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::floor_div_exact_j(thread, parameters).await
}

#[async_recursion(?Send)]
async fn floor_mod_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::floor_mod_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn floor_mod_j_1(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::floor_mod_j_1(thread, parameters).await
}

#[async_recursion(?Send)]
async fn floor_mod_j_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::floor_mod_j_2(thread, parameters).await
}

#[async_recursion(?Send)]
async fn fma_d(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::fma_d(thread, parameters).await
}

#[async_recursion(?Send)]
async fn fma_f(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::fma_f(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_exponent_d(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::get_exponent_d(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_exponent_f(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::get_exponent_f(thread, parameters).await
}

#[async_recursion(?Send)]
async fn hypot(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::hypot(thread, parameters).await
}

#[async_recursion(?Send)]
async fn increment_exact_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::increment_exact_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn increment_exact_j(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::increment_exact_j(thread, parameters).await
}

#[async_recursion(?Send)]
async fn log(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::log(thread, parameters).await
}

#[async_recursion(?Send)]
async fn log_10(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::log_10(thread, parameters).await
}

#[async_recursion(?Send)]
async fn log_1p(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::log_1p(thread, parameters).await
}

#[async_recursion(?Send)]
async fn max_d(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::max_d(thread, parameters).await
}

#[async_recursion(?Send)]
async fn max_f(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::max_f(thread, parameters).await
}

#[async_recursion(?Send)]
async fn max_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::max_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn max_j(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::max_j(thread, parameters).await
}

#[async_recursion(?Send)]
async fn min_d(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::min_d(thread, parameters).await
}

#[async_recursion(?Send)]
async fn min_f(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::min_f(thread, parameters).await
}

#[async_recursion(?Send)]
async fn min_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::min_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn min_j(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::min_j(thread, parameters).await
}

#[async_recursion(?Send)]
async fn multiply_exact_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::multiply_exact_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn multiply_exact_j_1(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::multiply_exact_j_1(thread, parameters).await
}

#[async_recursion(?Send)]
async fn multiply_exact_j_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::multiply_exact_j_2(thread, parameters).await
}

#[async_recursion(?Send)]
async fn multiply_full(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::multiply_full(thread, parameters).await
}

#[async_recursion(?Send)]
async fn multiply_high(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::multiply_high(thread, parameters).await
}

#[async_recursion(?Send)]
async fn negate_exact_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::negate_exact_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn negate_exact_j(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::negate_exact_j(thread, parameters).await
}

#[async_recursion(?Send)]
async fn next_after_d(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::next_after_d(thread, parameters).await
}

#[async_recursion(?Send)]
async fn next_after_f(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::next_after_f(thread, parameters).await
}

#[async_recursion(?Send)]
async fn next_down_d(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::next_down_d(thread, parameters).await
}

#[async_recursion(?Send)]
async fn next_down_f(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::next_down_f(thread, parameters).await
}

#[async_recursion(?Send)]
async fn next_up_d(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::next_up_d(thread, parameters).await
}

#[async_recursion(?Send)]
async fn next_up_f(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::next_up_f(thread, parameters).await
}

#[async_recursion(?Send)]
async fn pow(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::pow(thread, parameters).await
}

#[async_recursion(?Send)]
async fn power_of_two_d(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let n = parameters.pop_int()?;
    let result = 2f64.powi(n);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
async fn power_of_two_f(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let n = parameters.pop_int()?;
    let result = 2f32.powi(n);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
async fn random(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::random(thread, parameters).await
}

#[async_recursion(?Send)]
async fn rint(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::rint(thread, parameters).await
}

#[async_recursion(?Send)]
async fn round_d(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::round_d(thread, parameters).await
}

#[async_recursion(?Send)]
async fn round_f(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::round_f(thread, parameters).await
}

#[async_recursion(?Send)]
async fn scalb_d(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::scalb_d(thread, parameters).await
}

#[async_recursion(?Send)]
async fn scalb_f(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::scalb_f(thread, parameters).await
}

#[async_recursion(?Send)]
async fn signum_d(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::signum_d(thread, parameters).await
}

#[async_recursion(?Send)]
async fn signum_f(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::signum_f(thread, parameters).await
}

#[async_recursion(?Send)]
async fn sin(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::sin(thread, parameters).await
}

#[async_recursion(?Send)]
async fn sinh(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::sinh(thread, parameters).await
}

#[async_recursion(?Send)]
async fn sqrt(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::sqrt(thread, parameters).await
}

#[async_recursion(?Send)]
async fn subtract_exact_i(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::subtract_exact_i(thread, parameters).await
}

#[async_recursion(?Send)]
async fn subtract_exact_j(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::subtract_exact_j(thread, parameters).await
}

#[async_recursion(?Send)]
async fn tan(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::tan(thread, parameters).await
}

#[async_recursion(?Send)]
async fn tanh(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::tanh(thread, parameters).await
}

#[async_recursion(?Send)]
async fn to_degrees(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::to_degrees(thread, parameters).await
}

#[async_recursion(?Send)]
async fn to_int_exact(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::to_int_exact(thread, parameters).await
}

#[async_recursion(?Send)]
async fn to_radians(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::to_radians(thread, parameters).await
}

#[async_recursion(?Send)]
async fn ulp_d(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::ulp_d(thread, parameters).await
}

#[async_recursion(?Send)]
async fn ulp_f(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    java::lang::strictmath::ulp_f(thread, parameters).await
}

#[async_recursion(?Send)]
async fn unsigned_multiply_high(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    java::lang::strictmath::unsigned_multiply_high(thread, parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ieee_remainder() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0), Value::Double(2.0)]);
        let result = ieee_remainder(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_abs_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(-1.0)]);
        let result = abs_d(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_abs_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(-1.0)]);
        let result = abs_f(thread, parameters).await?;
        assert_eq!(result, Some(Value::Float(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_abs_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(-1)]);
        let result = abs_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_abs_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(-1)]);
        let result = abs_j(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_abs_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(-1)]);
        let result = abs_exact_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_abs_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(-1)]);
        let result = abs_exact_j(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_acos() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.5)]);
        let result = acos(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 1.047f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_add_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(1), Value::Int(2)]);
        let result = add_exact_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(3)));
        Ok(())
    }

    #[tokio::test]
    async fn test_add_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(1), Value::Long(2)]);
        let result = add_exact_j(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(3)));
        Ok(())
    }

    #[tokio::test]
    async fn test_asin() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.5)]);
        let result = asin(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.523f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_atan() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.5)]);
        let result = atan(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.463f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_atan_2() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0), Value::Double(1.0)]);
        let result = atan_2(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.785f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_cbrt() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(8.0)]);
        let result = cbrt(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.5)]);
        let result = ceil(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_div_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(5), Value::Int(2)]);
        let result = ceil_div_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(3)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_div_j_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(5), Value::Int(2)]);
        let result = ceil_div_j_1(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(3)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_div_j_2() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(5), Value::Long(2)]);
        let result = ceil_div_j_2(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(3)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_div_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(5), Value::Int(2)]);
        let result = ceil_div_exact_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(3)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_div_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(5), Value::Long(2)]);
        let result = ceil_div_exact_j(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(3)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_mod_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(5), Value::Int(2)]);
        let result = ceil_mod_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_mod_j_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(5), Value::Int(2)]);
        let result = ceil_mod_j_1(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ceil_mod_j_2() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(5), Value::Long(2)]);
        let result = ceil_mod_j_2(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_clamp_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![
            Value::Double(1.0),
            Value::Double(2.0),
            Value::Double(3.0),
        ]);
        let result = clamp_d(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_clamp_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![
            Value::Float(1.0),
            Value::Float(2.0),
            Value::Float(3.0),
        ]);
        let result = clamp_f(thread, parameters).await?;
        assert_eq!(result, Some(Value::Float(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_clamp_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(1), Value::Int(2), Value::Int(3)]);
        let result = clamp_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_clamp_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(1), Value::Long(2), Value::Long(3)]);
        let result = clamp_j(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_sign_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(-1.0), Value::Double(2.0)]);
        let result = copy_sign_d(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_sign_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(-1.0), Value::Float(2.0)]);
        let result = copy_sign_f(thread, parameters).await?;
        assert_eq!(result, Some(Value::Float(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_cos() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.5)]);
        let result = cos(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.877f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_cosh() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.5)]);
        let result = cosh(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 1.127f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_decrement_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(1)]);
        let result = decrement_exact_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_decrement_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(1)]);
        let result = decrement_exact_j(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_divide_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(5), Value::Int(2)]);
        let result = divide_exact_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_divide_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(5), Value::Long(2)]);
        let result = divide_exact_j(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_exp() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0)]);
        let result = exp(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - std::f64::consts::E;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_expm_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0)]);
        let result = expm_1(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 1.718f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_floor() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.5)]);
        let result = floor(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_div_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(5), Value::Int(2)]);
        let result = floor_div_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_div_j_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(5), Value::Int(2)]);
        let result = floor_div_j_1(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_div_j_2() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(5), Value::Long(2)]);
        let result = floor_div_j_2(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_div_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(5), Value::Int(2)]);
        let result = floor_div_exact_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_div_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(5), Value::Long(2)]);
        let result = floor_div_exact_j(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_mod_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(5), Value::Int(2)]);
        let result = floor_mod_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_mod_j_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(5), Value::Int(2)]);
        let result = floor_mod_j_1(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor_mod_j_2() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(5), Value::Long(2)]);
        let result = floor_mod_j_2(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_fma_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![
            Value::Double(1.0),
            Value::Double(2.0),
            Value::Double(3.0),
        ]);
        let result = fma_d(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(5.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_fma_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![
            Value::Float(1.0),
            Value::Float(2.0),
            Value::Float(3.0),
        ]);
        let result = fma_f(thread, parameters).await?;
        assert_eq!(result, Some(Value::Float(5.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_exponent_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0)]);
        let result = get_exponent_d(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_exponent_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(1.0)]);
        let result = get_exponent_f(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_hypot() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(3.0), Value::Double(4.0)]);
        let result = hypot(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(5.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_increment_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(1)]);
        let result = increment_exact_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_increment_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(1)]);
        let result = increment_exact_j(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_log() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0)]);
        let result = log(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(0.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_log_10() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0)]);
        let result = log_10(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(0.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_log_1p() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0)]);
        let result = log_1p(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.693f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_max_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0), Value::Double(2.0)]);
        let result = max_d(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_max_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(1.0), Value::Float(2.0)]);
        let result = max_f(thread, parameters).await?;
        assert_eq!(result, Some(Value::Float(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_max_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(1), Value::Int(2)]);
        let result = max_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_max_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(1), Value::Long(2)]);
        let result = max_j(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_min_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0), Value::Double(2.0)]);
        let result = min_d(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_min_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(1.0), Value::Float(2.0)]);
        let result = min_f(thread, parameters).await?;
        assert_eq!(result, Some(Value::Float(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_min_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(1), Value::Int(2)]);
        let result = min_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_min_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(1), Value::Long(2)]);
        let result = min_j(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_multiply_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(2), Value::Int(3)]);
        let result = multiply_exact_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(6)));
        Ok(())
    }

    #[tokio::test]
    async fn test_multiply_exact_j_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(2), Value::Int(3)]);
        let result = multiply_exact_j_1(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(6)));
        Ok(())
    }

    #[tokio::test]
    async fn test_multiply_exact_j_2() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(2), Value::Long(3)]);
        let result = multiply_exact_j_2(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(6)));
        Ok(())
    }

    #[tokio::test]
    async fn test_multiply_full() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(2), Value::Int(3)]);
        let result = multiply_full(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(6)));
        Ok(())
    }

    #[tokio::test]
    async fn test_multiply_high() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(2), Value::Long(3)]);
        let result = multiply_high(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_negate_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(1)]);
        let result = negate_exact_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(-1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_negate_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(1)]);
        let result = negate_exact_j(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(-1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_next_after_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0), Value::Double(2.0)]);
        let result = next_after_d(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 1.000f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_next_after_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(1.0), Value::Double(2.0)]);
        let result = next_after_f(thread, parameters).await?;
        let value = result.unwrap_or(Value::Float(0.0)).to_float()? - 1.000f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_next_down_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0)]);
        let result = next_down_d(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 1.000f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_next_down_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(1.0)]);
        let result = next_down_f(thread, parameters).await?;
        let value = result.unwrap_or(Value::Float(0.0)).to_float()? - 1.000f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_next_up_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0)]);
        let result = next_up_d(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 1.000f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_next_up_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(1.0)]);
        let result = next_up_f(thread, parameters).await?;
        let value = result.unwrap_or(Value::Float(0.0)).to_float()? - 1.000f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_pow() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(2.0), Value::Double(3.0)]);
        let result = pow(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(8.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_power_of_two_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(2)]);
        let result = power_of_two_d(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(4.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_power_of_two_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(2)]);
        let result = power_of_two_f(thread, parameters).await?;
        assert_eq!(result, Some(Value::Float(4.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_random() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![]);
        let result = random(thread, parameters).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_rint() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.5)]);
        let result = rint(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_round_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.5)]);
        let result = round_d(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_round_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(1.5)]);
        let result = round_f(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_scalb_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0), Value::Int(2)]);
        let result = scalb_d(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(4.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_scalb_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(1.0), Value::Int(2)]);
        let result = scalb_f(thread, parameters).await?;
        assert_eq!(result, Some(Value::Float(4.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_signum_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(-1.0)]);
        let result = signum_d(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(-1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_signum_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(-1.0)]);
        let result = signum_f(thread, parameters).await?;
        assert_eq!(result, Some(Value::Float(-1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_sin() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.5)]);
        let result = sin(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.479f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_sinh() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.5)]);
        let result = sinh(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.521f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_sqrt() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(4.0)]);
        let result = sqrt(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_subtract_exact_i() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(3), Value::Int(2)]);
        let result = subtract_exact_i(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_subtract_exact_j() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(3), Value::Long(2)]);
        let result = subtract_exact_j(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_tan() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.5)]);
        let result = tan(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.546f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_tanh() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.5)]);
        let result = tanh(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()? - 0.462f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_to_degrees() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(std::f64::consts::PI)]);
        let result = to_degrees(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(180.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_to_int_exact() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(1)]);
        let result = to_int_exact(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_to_radians() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(180.0)]);
        let result = to_radians(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(std::f64::consts::PI)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ulp_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0)]);
        let result = ulp_d(thread, parameters).await?;
        let value = result.unwrap_or(Value::Double(0.0)).to_double()?;
        assert!(value > 0.0f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_ulp_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(1.0)]);
        let result = ulp_f(thread, parameters).await?;
        let value = result.unwrap_or(Value::Float(0.0)).to_float()?;
        assert!(value > 0.0f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_unsigned_multiply_high() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(2), Value::Long(3)]);
        let result = unsigned_multiply_high(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }
}
