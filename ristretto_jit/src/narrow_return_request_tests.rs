//! Narrow ireturn semantics also apply at resumable method returns.
use super::*;

#[path = "../tests/data/narrow_return_cases.rs"]
mod narrow_cases;

#[test]
fn ireturn_narrows_request_only_results_from_verified_bytecode() -> Result<()> {
    let class = ClassFile::from_bytes(include_bytes!(
        "../../ristretto_vm/tests/fixtures/canonical_bytecode/CanonicalNarrowReturnWorkload.class"
    ))?;
    for (name, cases) in narrow_cases::cases() {
        let function = compile(&class, name)?;
        for (input, expected) in cases {
            let context = Context::new();
            let mut activation = function.activation(&[Value::I32(input)])?;
            assert_eq!(
                activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
                ExecutionStep::Returned(Some(Value::I32(expected))),
                "{name}({input})"
            );
            assert!(!context.failed.load(Ordering::Relaxed));
        }
    }
    Ok(())
}
