//! Request-only numeric conversions must not trap or require legacy helpers.
use super::*;

#[path = "../tests/data/numeric_conversion_cases.rs"]
mod numeric_cases;

fn execute(function: &Function, argument: Value, expected: Value) -> Result<()> {
    let context = Context::new();
    let mut activation = function.activation(&[argument])?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Returned(Some(expected))
    );
    assert!(!context.failed.load(Ordering::Relaxed));
    Ok(())
}

#[test]
fn float_conversion_boundaries_execute_through_request_only_activations() -> Result<()> {
    use Instruction as I;
    let int = primitive_arrays::fixture("(F)I", vec![I::Fload_0, I::F2i, I::Ireturn])?;
    let long = primitive_arrays::fixture("(F)J", vec![I::Fload_0, I::F2l, I::Lreturn])?;
    let int = compile(&int, "run")?;
    let long = compile(&long, "run")?;
    for (value, expected_int, expected_long) in numeric_cases::float_cases() {
        execute(&int, Value::F32(value), Value::I32(expected_int))?;
        execute(&long, Value::F32(value), Value::I64(expected_long))?;
    }
    Ok(())
}

#[test]
fn double_conversion_boundaries_execute_through_request_only_activations() -> Result<()> {
    use Instruction as I;
    let int = primitive_arrays::fixture("(D)I", vec![I::Dload_0, I::D2i, I::Ireturn])?;
    let long = primitive_arrays::fixture("(D)J", vec![I::Dload_0, I::D2l, I::Lreturn])?;
    let int = compile(&int, "run")?;
    let long = compile(&long, "run")?;
    for (value, expected_int, expected_long) in numeric_cases::double_cases() {
        execute(&int, Value::F64(value), Value::I32(expected_int))?;
        execute(&long, Value::F64(value), Value::I64(expected_long))?;
    }
    Ok(())
}
