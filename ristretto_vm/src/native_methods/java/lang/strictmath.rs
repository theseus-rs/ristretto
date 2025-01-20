use crate::native_methods::registry::{MethodRegistry, JAVA_20, JAVA_8};
use crate::parameters::Parameters;
use crate::thread::Thread;
#[cfg(target_arch = "wasm32")]
use crate::Error::InternalError;
use crate::JavaError::{ArithmeticException, IllegalArgumentException};
use crate::Result;
use async_recursion::async_recursion;
#[cfg(not(target_arch = "wasm32"))]
use rand::Rng;
use ristretto_classloader::Value;
use std::ops::Rem;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/StrictMath";

/// Register all native methods for `java.lang.StrictMath`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let use_optimizations = registry.use_optimizations();

    if use_optimizations || registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "cbrt", "(D)D", cbrt);
        registry.register(CLASS_NAME, "exp", "(D)D", exp);
        registry.register(CLASS_NAME, "hypot", "(DD)D", hypot);
        registry.register(CLASS_NAME, "pow", "(DD)D", pow);
    }

    if use_optimizations || registry.java_major_version() <= JAVA_20 {
        registry.register(CLASS_NAME, "IEEEremainder", "(DD)D", ieee_remainder);
        registry.register(CLASS_NAME, "acos", "(D)D", acos);
        registry.register(CLASS_NAME, "asin", "(D)D", asin);
        registry.register(CLASS_NAME, "atan", "(D)D", atan);
        registry.register(CLASS_NAME, "atan2", "(DD)D", atan_2);
        registry.register(CLASS_NAME, "cos", "(D)D", cos);
        registry.register(CLASS_NAME, "cosh", "(D)D", cosh);
        registry.register(CLASS_NAME, "expm1", "(D)D", expm_1);
        registry.register(CLASS_NAME, "log", "(D)D", log);
        registry.register(CLASS_NAME, "log10", "(D)D", log_10);
        registry.register(CLASS_NAME, "log1p", "(D)D", log_1p);
        registry.register(CLASS_NAME, "sin", "(D)D", sin);
        registry.register(CLASS_NAME, "sinh", "(D)D", sinh);
        registry.register(CLASS_NAME, "sqrt", "(D)D", sqrt);
        registry.register(CLASS_NAME, "tan", "(D)D", tan);
        registry.register(CLASS_NAME, "tanh", "(D)D", tanh);
    }

    if use_optimizations {
        registry.register(CLASS_NAME, "abs", "(D)D", abs_d);
        registry.register(CLASS_NAME, "abs", "(F)F", abs_f);
        registry.register(CLASS_NAME, "abs", "(I)I", abs_i);
        registry.register(CLASS_NAME, "abs", "(J)J", abs_j);
        registry.register(CLASS_NAME, "absExact", "(I)I", abs_exact_i);
        registry.register(CLASS_NAME, "absExact", "(J)J", abs_exact_j);
        registry.register(CLASS_NAME, "addExact", "(II)I", add_exact_i);
        registry.register(CLASS_NAME, "addExact", "(JJ)J", add_exact_j);
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
        registry.register(CLASS_NAME, "decrementExact", "(I)I", decrement_exact_i);
        registry.register(CLASS_NAME, "decrementExact", "(J)J", decrement_exact_j);
        registry.register(CLASS_NAME, "divideExact", "(II)I", divide_exact_i);
        registry.register(CLASS_NAME, "divideExact", "(JJ)J", divide_exact_j);
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
        registry.register(CLASS_NAME, "incrementExact", "(I)I", increment_exact_i);
        registry.register(CLASS_NAME, "incrementExact", "(J)J", increment_exact_j);
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
        registry.register(CLASS_NAME, "random", "()D", random);
        registry.register(CLASS_NAME, "rint", "(D)D", rint);
        registry.register(CLASS_NAME, "round", "(D)J", round_d);
        registry.register(CLASS_NAME, "round", "(F)I", round_f);
        registry.register(CLASS_NAME, "scalb", "(DI)D", scalb_d);
        registry.register(CLASS_NAME, "scalb", "(FI)F", scalb_f);
        registry.register(CLASS_NAME, "signum", "(D)D", signum_d);
        registry.register(CLASS_NAME, "signum", "(F)F", signum_f);
        registry.register(CLASS_NAME, "subtractExact", "(II)I", subtract_exact_i);
        registry.register(CLASS_NAME, "subtractExact", "(JJ)J", subtract_exact_j);
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
}

#[async_recursion(?Send)]
pub(crate) async fn ieee_remainder(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let f2 = parameters.pop_double()?;
    let f1 = parameters.pop_double()?;
    let result = f1.rem(f2);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn abs_d(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.abs();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn abs_f(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_float()?;
    let result = a.abs();
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn abs_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_int()?;
    let result = a.abs();
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn abs_j(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_long()?;
    let result = a.abs();
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn abs_exact_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_int()?;
    if a == i32::MIN {
        return Err(ArithmeticException(
            "Overflow to represent absolute value of Integer.MIN_VALUE".to_string(),
        )
        .into());
    }
    let result = a.abs();
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn abs_exact_j(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_long()?;
    if a == i64::MIN {
        return Err(ArithmeticException(
            "Overflow to represent absolute value of Long.MIN_VALUE".to_string(),
        )
        .into());
    }
    let result = a.abs();
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn add_exact_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_int()?;
    let x = parameters.pop_int()?;
    let Some(result) = x.checked_add(y) else {
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn add_exact_j(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_long()?;
    let x = parameters.pop_long()?;
    let Some(result) = x.checked_add(y) else {
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn acos(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.acos();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn asin(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.asin();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn atan(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.atan();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn atan_2(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let x = parameters.pop_double()?;
    let y = parameters.pop_double()?;
    let result = x.atan2(y);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn cbrt(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.cbrt();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn ceil(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.ceil();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn ceil_div_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_int()?;
    let x = parameters.pop_int()?;
    let Some(mut result) = x.checked_div(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Ok(Some(Value::Int(i32::MIN)));
    };
    if x % y != 0 {
        result += 1;
    }
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn ceil_div_j_1(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = i64::from(parameters.pop_int()?);
    let x = parameters.pop_long()?;
    let Some(mut result) = x.checked_div(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Ok(Some(Value::Long(i64::MIN)));
    };
    if x % y != 0 {
        result += 1;
    }
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn ceil_div_j_2(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_long()?;
    let x = parameters.pop_long()?;
    let Some(mut result) = x.checked_div(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Ok(Some(Value::Long(i64::MIN)));
    };
    if x % y != 0 {
        result += 1;
    }
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn ceil_div_exact_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_int()?;
    let x = parameters.pop_int()?;
    let Some(mut result) = x.checked_div(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    if x % y != 0 {
        result += 1;
    }
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn ceil_div_exact_j(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_long()?;
    let x = parameters.pop_long()?;
    let Some(mut result) = x.checked_div(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    if x % y != 0 {
        result += 1;
    }
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn ceil_mod_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_int()?;
    let x = parameters.pop_int()?;
    let Some(mut result) = x.checked_rem(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Ok(Some(Value::Int(i32::MIN)));
    };
    if result > 0 {
        result = y - result;
    }
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn ceil_mod_j_1(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = i64::from(parameters.pop_int()?);
    let x = parameters.pop_long()?;
    let Some(mut result) = x.checked_rem(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Ok(Some(Value::Long(i64::MIN)));
    };
    if result > 0 {
        result = y - result;
    }
    let result = i32::try_from(result)?;
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn ceil_mod_j_2(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_long()?;
    let x = parameters.pop_long()?;
    let Some(mut result) = x.checked_rem(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Ok(Some(Value::Long(i64::MIN)));
    };
    if result > 0 {
        result = y - result;
    }
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn clamp_d(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let max = parameters.pop_double()?;
    let min = parameters.pop_double()?;
    let value = parameters.pop_double()?;
    if min > max {
        return Err(IllegalArgumentException(format!("{min} > {max}")).into());
    }
    let result = value.max(min).min(max);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn clamp_f(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let max = parameters.pop_float()?;
    let min = parameters.pop_float()?;
    let value = parameters.pop_float()?;
    if min > max {
        return Err(IllegalArgumentException(format!("{min} > {max}")).into());
    }
    let result = value.max(min).min(max);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn clamp_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let max = i64::from(parameters.pop_int()?);
    let min = i64::from(parameters.pop_int()?);
    let value = parameters.pop_long()?;
    if min > max {
        return Err(IllegalArgumentException(format!("{min} > {max}")).into());
    }
    let result = i32::try_from(value.max(min).min(max))?;
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn clamp_j(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let max = parameters.pop_long()?;
    let min = parameters.pop_long()?;
    let value = parameters.pop_long()?;
    if min > max {
        return Err(IllegalArgumentException(format!("{min} > {max}")).into());
    }
    let result = value.max(min).min(max);
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn copy_sign_d(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let sign = parameters.pop_double()?;
    let magnitude = parameters.pop_double()?;
    let result = magnitude.copysign(sign);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn copy_sign_f(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let sign = parameters.pop_float()?;
    let magnitude = parameters.pop_float()?;
    let result = magnitude.copysign(sign);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn cos(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.cos();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn cosh(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.cosh();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn decrement_exact_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_int()?;
    let Some(result) = a.checked_sub(1) else {
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn decrement_exact_j(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_long()?;
    let Some(result) = a.checked_sub(1) else {
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn divide_exact_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_int()?;
    let x = parameters.pop_int()?;
    let Some(result) = x.checked_div(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn divide_exact_j(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_long()?;
    let x = parameters.pop_long()?;
    let Some(result) = x.checked_div(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn exp(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.exp();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn expm_1(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.exp_m1();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn floor(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.floor();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn floor_div_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_int()?;
    let x = parameters.pop_int()?;
    let Some(result) = x.checked_div(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Ok(Some(Value::Int(i32::MIN)));
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn floor_div_j_1(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = i64::from(parameters.pop_int()?);
    let x = parameters.pop_long()?;
    let Some(result) = x.checked_div(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Ok(Some(Value::Long(i64::MIN)));
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn floor_div_j_2(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_long()?;
    let x = parameters.pop_long()?;
    let Some(result) = x.checked_div(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Ok(Some(Value::Long(i64::MIN)));
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn floor_div_exact_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_int()?;
    let x = parameters.pop_int()?;
    let Some(result) = x.checked_div(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn floor_div_exact_j(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_long()?;
    let x = parameters.pop_long()?;
    let Some(result) = x.checked_div(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn floor_mod_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_int()?;
    let x = parameters.pop_int()?;
    let Some(mut result) = x.checked_rem(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Ok(Some(Value::Int(i32::MIN)));
    };
    if (result > 0 && y < 0) || (result < 0 && y > 0) {
        result += y;
    }
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn floor_mod_j_1(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = i64::from(parameters.pop_int()?);
    let x = parameters.pop_long()?;
    let Some(mut result) = x.checked_rem(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Ok(Some(Value::Long(i64::MIN)));
    };
    if (result > 0 && y < 0) || (result < 0 && y > 0) {
        result += y;
    }
    let result = i32::try_from(result)?;
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn floor_mod_j_2(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_long()?;
    let x = parameters.pop_long()?;
    let Some(mut result) = x.checked_rem(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Ok(Some(Value::Long(i64::MIN)));
    };
    if (result > 0 && y < 0) || (result < 0 && y > 0) {
        result += y;
    }
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn fma_d(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let c = parameters.pop_double()?;
    let b = parameters.pop_double()?;
    let a = parameters.pop_double()?;
    let result = a.mul_add(b, c);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn fma_f(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let c = parameters.pop_float()?;
    let b = parameters.pop_float()?;
    let a = parameters.pop_float()?;
    let result = a.mul_add(b, c);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn get_exponent_d(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let d = parameters.pop_double()?;
    if d.is_nan() || d.is_infinite() {
        return Ok(Some(Value::Int(1024)));
    }
    let bits = d.to_bits();
    let exponent_bits = (bits >> 52) & 0x7FF;
    let result = exponent_bits as i32 - 1023;
    Ok(Some(Value::Int(result)))
}

#[expect(clippy::cast_possible_wrap)]
#[async_recursion(?Send)]
pub(crate) async fn get_exponent_f(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let f = parameters.pop_float()?;
    if f.is_nan() || f.is_infinite() {
        return Ok(Some(Value::Int(128)));
    }
    let bits = f.to_bits();
    let exponent_bits = (bits >> 23) & 0xFF;
    let result = exponent_bits as i32 - 127;
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn hypot(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_double()?;
    let x = parameters.pop_double()?;
    let result = x.hypot(y);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn increment_exact_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_int()?;
    let Some(result) = a.checked_add(1) else {
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn increment_exact_j(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_long()?;
    let Some(result) = a.checked_add(1) else {
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn log(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.ln();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn log_10(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.log10();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn log_1p(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.ln_1p();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn max_d(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let b = parameters.pop_double()?;
    let a = parameters.pop_double()?;
    let result = a.max(b);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn max_f(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let b = parameters.pop_float()?;
    let a = parameters.pop_float()?;
    let result = a.max(b);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn max_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let b = parameters.pop_int()?;
    let a = parameters.pop_int()?;
    let result = a.max(b);
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn max_j(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let b = parameters.pop_long()?;
    let a = parameters.pop_long()?;
    let result = a.max(b);
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn min_d(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let b = parameters.pop_double()?;
    let a = parameters.pop_double()?;
    let result = a.min(b);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn min_f(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let b = parameters.pop_float()?;
    let a = parameters.pop_float()?;
    let result = a.min(b);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn min_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let b = parameters.pop_int()?;
    let a = parameters.pop_int()?;
    let result = a.min(b);
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn min_j(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let b = parameters.pop_long()?;
    let a = parameters.pop_long()?;
    let result = a.min(b);
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn multiply_exact_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_int()?;
    let x = parameters.pop_int()?;
    let Some(result) = x.checked_mul(y) else {
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn multiply_exact_j_1(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = i64::from(parameters.pop_int()?);
    let x = parameters.pop_long()?;
    let Some(result) = x.checked_mul(y) else {
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn multiply_exact_j_2(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_long()?;
    let x = parameters.pop_long()?;
    let Some(result) = x.checked_mul(y) else {
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn multiply_full(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = i64::from(parameters.pop_int()?);
    let x = i64::from(parameters.pop_int()?);
    let result = x * y;
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn multiply_high(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = i128::from(parameters.pop_long()?);
    let x = i128::from(parameters.pop_long()?);
    let product = x * y;
    let result = (product >> 64) as i64;
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn negate_exact_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_int()?;
    let Some(result) = a.checked_neg() else {
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn negate_exact_j(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_long()?;
    let Some(result) = a.checked_neg() else {
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn next_after_d(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let direction = parameters.pop_double()?;
    let start = parameters.pop_double()?;
    if start.is_nan() || direction.is_nan() {
        return Ok(Some(Value::Double(f64::NAN)));
    }
    #[expect(clippy::float_cmp)]
    if start == direction {
        return Ok(Some(Value::Double(direction)));
    }
    if start == 0.0 {
        return if direction > 0.0 {
            Ok(Some(Value::Double(f64::MIN_POSITIVE)))
        } else {
            Ok(Some(Value::Double(-f64::MIN_POSITIVE)))
        };
    }
    let bits = start.to_bits();
    let next_bits = if (start > 0.0) == (direction > start) {
        bits + 1
    } else {
        bits - 1
    };
    let result = f64::from_bits(next_bits);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn next_after_f(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    #[expect(clippy::cast_possible_truncation)]
    let direction = parameters.pop_double()? as f32;
    let start = parameters.pop_float()?;
    if start.is_nan() || direction.is_nan() {
        return Ok(Some(Value::Float(f32::NAN)));
    }
    #[expect(clippy::float_cmp)]
    if start == direction {
        return Ok(Some(Value::Float(direction)));
    }
    if start == 0.0 {
        return if direction > 0.0 {
            Ok(Some(Value::Float(f32::MIN_POSITIVE)))
        } else {
            Ok(Some(Value::Float(-f32::MIN_POSITIVE)))
        };
    }
    let bits = start.to_bits();
    let next_bits = if (start > 0.0) == (direction > start) {
        bits + 1
    } else {
        bits - 1
    };
    let result = f32::from_bits(next_bits);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn next_down_d(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let d = parameters.pop_double()?;
    if d.is_nan() {
        return Ok(Some(Value::Double(d)));
    }
    if d == 0.0 {
        return Ok(Some(Value::Double(-f64::MIN_POSITIVE)));
    }
    if d.is_infinite() && d.is_sign_positive() {
        return Ok(Some(Value::Double(f64::MAX)));
    }
    let result = f64::from_bits(d.to_bits() - 1);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn next_down_f(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let f = parameters.pop_float()?;
    if f.is_nan() {
        return Ok(Some(Value::Float(f)));
    }
    if f == 0.0 {
        return Ok(Some(Value::Float(-f32::MIN_POSITIVE)));
    }
    if f.is_infinite() && f.is_sign_positive() {
        return Ok(Some(Value::Float(f32::MAX)));
    }
    let result = f32::from_bits(f.to_bits() - 1);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn next_up_d(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let d = parameters.pop_double()?;
    if d.is_nan() || d == f64::INFINITY {
        return Ok(Some(Value::Double(d)));
    }
    if d == 0.0 {
        return Ok(Some(Value::Double(f64::MIN_POSITIVE)));
    }
    let bits = d.to_bits();
    let next_bits = if d > 0.0 { bits + 1 } else { bits - 1 };
    let result = f64::from_bits(next_bits);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn next_up_f(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let f = parameters.pop_float()?;
    if f.is_nan() || f == f32::INFINITY {
        return Ok(Some(Value::Float(f)));
    }
    if f == 0.0 {
        return Ok(Some(Value::Float(f32::MIN_POSITIVE)));
    }
    let bits = f.to_bits();
    let next_bits = if f > 0.0 { bits + 1 } else { bits - 1 };
    let result = f32::from_bits(next_bits);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn pow(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let b = parameters.pop_double()?;
    let a = parameters.pop_double()?;
    let result = a.powf(b);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn random(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    #[cfg(target_arch = "wasm32")]
    let result = {
        let mut buf = [0u8; 8];
        getrandom::getrandom(&mut buf).map_err(|error| InternalError(error.to_string()))?;
        let random_u64 = u64::from_ne_bytes(buf);
        (random_u64 as f64) / (u64::MAX as f64)
    };
    #[cfg(not(target_arch = "wasm32"))]
    let result = {
        let mut rng = rand::thread_rng();
        rng.gen_range(0.0f64..1.0f64)
    };
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn rint(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let mut rounded = a.round();
    #[expect(clippy::float_cmp)]
    if (a - rounded).abs() == 0.5 && rounded % 2.0 != 0.0 {
        rounded -= a.signum();
    }
    Ok(Some(Value::Double(rounded)))
}

#[async_recursion(?Send)]
pub(crate) async fn round_d(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    #[expect(clippy::cast_possible_truncation)]
    let result = a.round() as i64;
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn round_f(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_float()?;
    #[expect(clippy::cast_possible_truncation)]
    let result = a.round() as i32;
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn scalb_d(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let scale_factor = parameters.pop_int()?;
    let d = parameters.pop_double()?;
    let result = d * 2f64.powi(scale_factor);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn scalb_f(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let scale_factor = parameters.pop_int()?;
    let d = parameters.pop_float()?;
    let result = d * 2f32.powi(scale_factor);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn signum_d(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.signum();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn signum_f(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_float()?;
    let result = a.signum();
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn sin(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.sin();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn sinh(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.sinh();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn sqrt(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.sqrt();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn subtract_exact_i(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_int()?;
    let x = parameters.pop_int()?;
    let Some(result) = x.checked_sub(y) else {
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn subtract_exact_j(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_long()?;
    let x = parameters.pop_long()?;
    let Some(result) = x.checked_sub(y) else {
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn tan(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.tan();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn tanh(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.tanh();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn to_degrees(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.to_degrees();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn to_int_exact(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_long()?;
    let Ok(result) = i32::try_from(a) else {
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn to_radians(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.to_radians();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn ulp_d(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let d = parameters.pop_double()?;
    let result = if d.is_infinite() || d.is_nan() {
        f64::NAN
    } else {
        let abs_d = d.abs();
        let next = f64::from_bits(abs_d.to_bits() + 1);
        next - abs_d
    };
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn ulp_f(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let f = parameters.pop_float()?;
    let result = if f.is_infinite() || f.is_nan() {
        f32::NAN
    } else {
        let abs_f = f.abs();
        let next = f32::from_bits(abs_f.to_bits() + 1);
        next - abs_f
    };
    Ok(Some(Value::Float(result)))
}

#[expect(clippy::cast_possible_wrap)]
#[expect(clippy::cast_sign_loss)]
#[async_recursion(?Send)]
pub(crate) async fn unsigned_multiply_high(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let y = parameters.pop_long()? as u64;
    let x = parameters.pop_long()? as u64;
    let product = u128::from(x) * u128::from(y);
    let result = (product >> 64) as u64;
    Ok(Some(Value::Long(result as i64)))
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
        let parameters = Parameters::new(vec![Value::Double(0.0)]);
        let result = acos(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(std::f64::consts::FRAC_PI_2)));
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
        let parameters = Parameters::new(vec![Value::Double(0.0)]);
        let result = asin(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(0.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_atan() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.0)]);
        let result = atan(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(0.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_atan_2() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.0), Value::Double(0.0)]);
        let result = atan_2(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(0.0)));
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
        let parameters = Parameters::new(vec![Value::Double(1.1)]);
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
        let parameters = Parameters::new(vec![Value::Double(0.0)]);
        let result = cos(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_cosh() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.0)]);
        let result = cosh(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
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
        let parameters = Parameters::new(vec![Value::Double(0.0)]);
        let result = exp(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_expm_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.0)]);
        let result = expm_1(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(0.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_floor() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.1)]);
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
        let parameters = Parameters::new(vec![Value::Double(std::f64::consts::E)]);
        let result = log(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_log_10() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(100.0)]);
        let result = log_10(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(2.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_log_1p() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(std::f64::consts::E - 1.0)]);
        let result = log_1p(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
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
        assert_eq!(result, Some(Value::Double(1.0 + f64::EPSILON)));
        Ok(())
    }

    #[tokio::test]
    async fn test_next_after_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(1.0), Value::Double(2.0)]);
        let result = next_after_f(thread, parameters).await?;
        assert_eq!(result, Some(Value::Float(1.0 + f32::EPSILON)));
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
        let value = result.unwrap_or(Value::Float(0.0)).to_float()? - 1.047f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_next_up_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.0)]);
        let result = next_up_d(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(1.0 + f64::EPSILON)));
        Ok(())
    }

    #[tokio::test]
    async fn test_next_up_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(1.0)]);
        let result = next_up_f(thread, parameters).await?;
        assert_eq!(result, Some(Value::Float(1.0 + f32::EPSILON)));
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
        let parameters = Parameters::new(vec![Value::Double(1.1)]);
        let result = rint(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(1.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_round_d() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(1.1)]);
        let result = round_d(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_round_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(1.1)]);
        let result = round_f(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(1)));
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
        let parameters = Parameters::new(vec![Value::Double(0.0)]);
        let result = sin(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(0.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_sinh() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.0)]);
        let result = sinh(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(0.0)));
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
        let parameters = Parameters::new(vec![Value::Double(0.0)]);
        let result = tan(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(0.0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_tanh() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.0)]);
        let result = tanh(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(0.0)));
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
    async fn test_to_degrees() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(std::f64::consts::PI)]);
        let result = to_degrees(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(180.0)));
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
        assert_eq!(result, Some(Value::Double(f64::EPSILON)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ulp_f() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Float(1.0)]);
        let result = ulp_f(thread, parameters).await?;
        assert_eq!(result, Some(Value::Float(f32::EPSILON)));
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
