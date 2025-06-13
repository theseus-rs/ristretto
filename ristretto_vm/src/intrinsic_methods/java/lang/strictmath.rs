use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classfile::{JAVA_8, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::ops::Rem;
use std::sync::Arc;

#[intrinsic_method("java/lang/StrictMath.IEEEremainder(DD)D", LessThanOrEqual(JAVA_17))]
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

#[intrinsic_method("java/lang/StrictMath.acos(D)D", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn acos(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.acos();
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.asin(D)D", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn asin(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.asin();
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.atan(D)D", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn atan(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.atan();
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.atan2(DD)D", LessThanOrEqual(JAVA_17))]
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

#[intrinsic_method("java/lang/StrictMath.cbrt(D)D", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn cbrt(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.cbrt();
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.cos(D)D", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn cos(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.cos();
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.cosh(D)D", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn cosh(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.cosh();
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.exp(D)D", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn exp(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.exp();
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.expm1(D)D", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn expm_1(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.exp_m1();
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.hypot(DD)D", LessThanOrEqual(JAVA_8))]
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

#[intrinsic_method("java/lang/StrictMath.log(D)D", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn log(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.ln();
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.log10(D)D", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn log_10(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.log10();
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.log1p(D)D", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn log_1p(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.ln_1p();
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.pow(DD)D", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn pow(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let b = parameters.pop_double()?;
    let a = parameters.pop_double()?;
    let result = a.powf(b);
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.sin(D)D", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn sin(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.sin();
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.sinh(D)D", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn sinh(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.sinh();
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.sqrt(D)D", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn sqrt(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.sqrt();
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.tan(D)D", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn tan(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let a = parameters.pop_double()?;
    let result = a.tan();
    Ok(Some(Value::Double(result)))
}

#[intrinsic_method("java/lang/StrictMath.tanh(D)D", LessThanOrEqual(JAVA_17))]
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
