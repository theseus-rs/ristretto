use crate::Result;
use crate::native_methods::registry::{JAVA_8, JAVA_17, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::ops::Rem;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/StrictMath";

/// Register all native methods for `java.lang.StrictMath`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "cbrt", "(D)D", cbrt);
        registry.register(CLASS_NAME, "exp", "(D)D", exp);
        registry.register(CLASS_NAME, "hypot", "(DD)D", hypot);
        registry.register(CLASS_NAME, "pow", "(DD)D", pow);
    }

    if registry.java_major_version() <= JAVA_17 {
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
pub(crate) async fn pow(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let b = parameters.pop_double()?;
    let a = parameters.pop_double()?;
    let result = a.powf(b);
    Ok(Some(Value::Double(result)))
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
    async fn test_acos() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(0.0)]);
        let result = acos(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(std::f64::consts::FRAC_PI_2)));
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
    async fn test_hypot() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(3.0), Value::Double(4.0)]);
        let result = hypot(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(5.0)));
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
    async fn test_pow() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Double(2.0), Value::Double(3.0)]);
        let result = pow(thread, parameters).await?;
        assert_eq!(result, Some(Value::Double(8.0)));
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
}
