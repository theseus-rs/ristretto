// Backend shape fixtures only; both runtime admission gates exclude monitors.
use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::Result;

#[test]
fn monitorenter() -> Result<()> {
    let instructions = vec![
        Instruction::Aconst_null,
        Instruction::Monitorenter,
        Instruction::Return,
    ];
    let function = create_function("()V", &instructions)?;
    let value = function.execute(&[], std::ptr::null())?;
    assert_eq!(value, None);
    Ok(())
}

#[test]
fn monitorexit() -> Result<()> {
    let instructions = vec![
        Instruction::Aconst_null,
        Instruction::Monitorexit,
        Instruction::Return,
    ];
    let function = create_function("()V", &instructions)?;
    let value = function.execute(&[], std::ptr::null())?;
    assert_eq!(value, None);
    Ok(())
}

#[test]
fn monitor_operand_must_be_a_reference() {
    assert!(
        crate::util::create_internal_function(
            "()V",
            &[
                Instruction::Lconst_0,
                Instruction::Monitorenter,
                Instruction::Return
            ],
            2
        )
        .is_err()
    );
}
