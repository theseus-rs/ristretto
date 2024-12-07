use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::JavaError::{ArithmeticException, IllegalArgumentException};
use crate::Result;
use async_recursion::async_recursion;
use rand::Rng;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::ops::Rem;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };
const JAVA_20: Version = Version::Java20 { minor: 0 };

/// Register all native methods for `java.lang.StrictMath`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/StrictMath";
    let use_optimizations = registry.use_optimizations();
    let java_version = registry.java_version().clone();

    if use_optimizations || java_version <= JAVA_8 {
        registry.register(class_name, "cbrt", "(D)D", cbrt);
        registry.register(class_name, "exp", "(D)D", exp);
        registry.register(class_name, "hypot", "(DD)D", hypot);
        registry.register(class_name, "pow", "(DD)D", pow);
    }

    if use_optimizations || java_version <= JAVA_20 {
        registry.register(class_name, "IEEEremainder", "(DD)D", ieee_remainder);
        registry.register(class_name, "acos", "(D)D", acos);
        registry.register(class_name, "asin", "(D)D", asin);
        registry.register(class_name, "atan", "(D)D", atan);
        registry.register(class_name, "atan2", "(DD)D", atan_2);
        registry.register(class_name, "cos", "(D)D", cos);
        registry.register(class_name, "cosh", "(D)D", cosh);
        registry.register(class_name, "expm1", "(D)D", expm_1);
        registry.register(class_name, "log", "(D)D", log);
        registry.register(class_name, "log10", "(D)D", log_10);
        registry.register(class_name, "log1p", "(D)D", log_1p);
        registry.register(class_name, "sin", "(D)D", sin);
        registry.register(class_name, "sinh", "(D)D", sinh);
        registry.register(class_name, "sqrt", "(D)D", sqrt);
        registry.register(class_name, "tan", "(D)D", tan);
        registry.register(class_name, "tanh", "(D)D", tanh);
    }

    if use_optimizations {
        registry.register(class_name, "abs", "(D)D", abs_d);
        registry.register(class_name, "abs", "(F)F", abs_f);
        registry.register(class_name, "abs", "(I)I", abs_i);
        registry.register(class_name, "abs", "(J)J", abs_j);
        registry.register(class_name, "absExact", "(I)I", abs_exact_i);
        registry.register(class_name, "absExact", "(J)J", abs_exact_j);
        registry.register(class_name, "addExact", "(II)I", add_exact_i);
        registry.register(class_name, "addExact", "(JJ)J", add_exact_j);
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
        registry.register(class_name, "decrementExact", "(I)I", decrement_exact_i);
        registry.register(class_name, "decrementExact", "(J)J", decrement_exact_j);
        registry.register(class_name, "divideExact", "(II)I", divide_exact_i);
        registry.register(class_name, "divideExact", "(JJ)J", divide_exact_j);
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
        registry.register(class_name, "incrementExact", "(I)I", increment_exact_i);
        registry.register(class_name, "incrementExact", "(J)J", increment_exact_j);
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
        registry.register(class_name, "random", "()D", random);
        registry.register(class_name, "rint", "(D)D", rint);
        registry.register(class_name, "round", "(D)J", round_d);
        registry.register(class_name, "round", "(F)I", round_f);
        registry.register(class_name, "scalb", "(DI)D", scalb_d);
        registry.register(class_name, "scalb", "(FI)F", scalb_f);
        registry.register(class_name, "signum", "(D)D", signum_d);
        registry.register(class_name, "signum", "(F)F", signum_f);
        registry.register(class_name, "subtractExact", "(II)I", subtract_exact_i);
        registry.register(class_name, "subtractExact", "(JJ)J", subtract_exact_j);
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
}

#[async_recursion(?Send)]
pub(crate) async fn ieee_remainder(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let f2 = arguments.pop_double()?;
    let f1 = arguments.pop_double()?;
    let result = f1.rem(f2);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn abs_d(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.abs();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn abs_f(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_float()?;
    let result = a.abs();
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn abs_i(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_int()?;
    let result = a.abs();
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn abs_j(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_long()?;
    let result = a.abs();
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn abs_exact_i(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_int()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_long()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_int()?;
    let x = arguments.pop_int()?;
    let Some(result) = x.checked_add(y) else {
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn add_exact_j(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_long()?;
    let x = arguments.pop_long()?;
    let Some(result) = x.checked_add(y) else {
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn acos(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.acos();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn asin(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.asin();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn atan(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.atan();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn atan_2(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_double()?;
    let y = arguments.pop_double()?;
    let result = x.atan2(y);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn cbrt(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.cbrt();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn ceil(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.ceil();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn ceil_div_i(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_int()?;
    let x = arguments.pop_int()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = i64::from(arguments.pop_int()?);
    let x = arguments.pop_long()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_long()?;
    let x = arguments.pop_long()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_int()?;
    let x = arguments.pop_int()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_long()?;
    let x = arguments.pop_long()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_int()?;
    let x = arguments.pop_int()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = i64::from(arguments.pop_int()?);
    let x = arguments.pop_long()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_long()?;
    let x = arguments.pop_long()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let max = arguments.pop_double()?;
    let min = arguments.pop_double()?;
    let value = arguments.pop_double()?;
    if min > max {
        return Err(IllegalArgumentException(format!("{min} > {max}")).into());
    }
    let result = value.max(min).min(max);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn clamp_f(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let max = arguments.pop_float()?;
    let min = arguments.pop_float()?;
    let value = arguments.pop_float()?;
    if min > max {
        return Err(IllegalArgumentException(format!("{min} > {max}")).into());
    }
    let result = value.max(min).min(max);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn clamp_i(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let max = arguments.pop_int()?;
    let min = arguments.pop_int()?;
    let value = arguments.pop_int()?;
    if min > max {
        return Err(IllegalArgumentException(format!("{min} > {max}")).into());
    }
    let result = value.max(min).min(max);
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn clamp_j(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let max = arguments.pop_long()?;
    let min = arguments.pop_long()?;
    let value = arguments.pop_long()?;
    if min > max {
        return Err(IllegalArgumentException(format!("{min} > {max}")).into());
    }
    let result = value.max(min).min(max);
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn copy_sign_d(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let sign = arguments.pop_double()?;
    let magnitude = arguments.pop_double()?;
    let result = magnitude.copysign(sign);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn copy_sign_f(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let sign = arguments.pop_float()?;
    let magnitude = arguments.pop_float()?;
    let result = magnitude.copysign(sign);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn cos(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.cos();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn cosh(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.cosh();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn decrement_exact_i(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_int()?;
    let Some(result) = a.checked_sub(1) else {
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn decrement_exact_j(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_long()?;
    let Some(result) = a.checked_sub(1) else {
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn divide_exact_i(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_int()?;
    let x = arguments.pop_int()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_long()?;
    let x = arguments.pop_long()?;
    let Some(result) = x.checked_div(y) else {
        if y == 0 {
            return Err(ArithmeticException("/ by zero".to_string()).into());
        }
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn exp(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.exp();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn expm_1(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.exp_m1();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn floor(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.floor();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn floor_div_i(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_int()?;
    let x = arguments.pop_int()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = i64::from(arguments.pop_int()?);
    let x = arguments.pop_long()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_long()?;
    let x = arguments.pop_long()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_int()?;
    let x = arguments.pop_int()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_long()?;
    let x = arguments.pop_long()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_int()?;
    let x = arguments.pop_int()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = i64::from(arguments.pop_int()?);
    let x = arguments.pop_long()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_long()?;
    let x = arguments.pop_long()?;
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
pub(crate) async fn fma_d(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let c = arguments.pop_double()?;
    let b = arguments.pop_double()?;
    let a = arguments.pop_double()?;
    let result = a.mul_add(b, c);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn fma_f(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let c = arguments.pop_float()?;
    let b = arguments.pop_float()?;
    let a = arguments.pop_float()?;
    let result = a.mul_add(b, c);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn get_exponent_d(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let d = arguments.pop_double()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let f = arguments.pop_float()?;
    if f.is_nan() || f.is_infinite() {
        return Ok(Some(Value::Int(128)));
    }
    let bits = f.to_bits();
    let exponent_bits = (bits >> 23) & 0xFF;
    let result = exponent_bits as i32 - 127;
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn hypot(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let y = arguments.pop_double()?;
    let x = arguments.pop_double()?;
    let result = x.hypot(y);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn increment_exact_i(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_int()?;
    let Some(result) = a.checked_add(1) else {
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn increment_exact_j(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_long()?;
    let Some(result) = a.checked_add(1) else {
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn log(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.ln();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn log_10(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.log10();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn log_1p(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.ln_1p();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn max_d(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let b = arguments.pop_double()?;
    let a = arguments.pop_double()?;
    let result = a.max(b);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn max_f(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let b = arguments.pop_float()?;
    let a = arguments.pop_float()?;
    let result = a.max(b);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn max_i(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let b = arguments.pop_int()?;
    let a = arguments.pop_int()?;
    let result = a.max(b);
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn max_j(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let b = arguments.pop_long()?;
    let a = arguments.pop_long()?;
    let result = a.max(b);
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn min_d(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let b = arguments.pop_double()?;
    let a = arguments.pop_double()?;
    let result = a.min(b);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn min_f(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let b = arguments.pop_float()?;
    let a = arguments.pop_float()?;
    let result = a.min(b);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn min_i(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let b = arguments.pop_int()?;
    let a = arguments.pop_int()?;
    let result = a.min(b);
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn min_j(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let b = arguments.pop_long()?;
    let a = arguments.pop_long()?;
    let result = a.min(b);
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn multiply_exact_i(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_int()?;
    let x = arguments.pop_int()?;
    let Some(result) = x.checked_mul(y) else {
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn multiply_exact_j_1(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = i64::from(arguments.pop_int()?);
    let x = arguments.pop_long()?;
    let Some(result) = x.checked_mul(y) else {
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn multiply_exact_j_2(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_long()?;
    let x = arguments.pop_long()?;
    let Some(result) = x.checked_mul(y) else {
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn multiply_full(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = i64::from(arguments.pop_int()?);
    let x = i64::from(arguments.pop_int()?);
    let result = x * y;
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn multiply_high(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = i128::from(arguments.pop_long()?);
    let x = i128::from(arguments.pop_long()?);
    let product = x * y;
    let result = (product >> 64) as i64;
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn negate_exact_i(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_int()?;
    let Some(result) = a.checked_neg() else {
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn negate_exact_j(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_long()?;
    let Some(result) = a.checked_neg() else {
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn next_after_d(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let direction = arguments.pop_double()?;
    let start = arguments.pop_double()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    #[expect(clippy::cast_possible_truncation)]
    let direction = arguments.pop_double()? as f32;
    let start = arguments.pop_float()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let d = arguments.pop_double()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let f = arguments.pop_float()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let d = arguments.pop_double()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let f = arguments.pop_float()?;
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
pub(crate) async fn pow(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let b = arguments.pop_double()?;
    let a = arguments.pop_double()?;
    let result = a.powf(b);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn random(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let mut rng = rand::thread_rng();
    let result = rng.gen_range(0.0f64..1.0f64);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn rint(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    #[expect(clippy::cast_possible_truncation)]
    let result = a.round() as i64;
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn round_f(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_float()?;
    #[expect(clippy::cast_possible_truncation)]
    let result = a.round() as i32;
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn scalb_d(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let scale_factor = arguments.pop_int()?;
    let d = arguments.pop_double()?;
    let result = d * 2f64.powi(scale_factor);
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn scalb_f(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let scale_factor = arguments.pop_int()?;
    let d = arguments.pop_float()?;
    let result = d * 2f32.powi(scale_factor);
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn signum_d(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.signum();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn signum_f(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_float()?;
    let result = a.signum();
    Ok(Some(Value::Float(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn sin(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.sin();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn sinh(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.sinh();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn sqrt(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.sqrt();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn subtract_exact_i(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_int()?;
    let x = arguments.pop_int()?;
    let Some(result) = x.checked_sub(y) else {
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn subtract_exact_j(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_long()?;
    let x = arguments.pop_long()?;
    let Some(result) = x.checked_sub(y) else {
        return Err(ArithmeticException("long overflow".to_string()).into());
    };
    Ok(Some(Value::Long(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn tan(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.tan();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn tanh(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.tanh();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn to_degrees(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.to_degrees();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn to_int_exact(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_long()?;
    let Ok(result) = i32::try_from(a) else {
        return Err(ArithmeticException("integer overflow".to_string()).into());
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn to_radians(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let a = arguments.pop_double()?;
    let result = a.to_radians();
    Ok(Some(Value::Double(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn ulp_d(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let d = arguments.pop_double()?;
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
pub(crate) async fn ulp_f(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let f = arguments.pop_float()?;
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let y = arguments.pop_long()? as u64;
    let x = arguments.pop_long()? as u64;
    let product = u128::from(x) * u128::from(y);
    let result = (product >> 64) as u64;
    Ok(Some(Value::Long(result as i64)))
}
