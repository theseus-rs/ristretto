use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::Error;

#[test]
fn monitorenter_rejects_long_operand() {
    let instructions = vec![
        Instruction::Lconst_0,
        Instruction::Monitorenter,
        Instruction::Return,
    ];
    assert!(
        matches!(create_function("()V", &instructions), Err(Error::InternalError(message)) if message.contains("Expected Reference"))
    );
}

#[test]
fn monitorexit_rejects_long_operand() {
    let instructions = vec![
        Instruction::Lconst_0,
        Instruction::Monitorexit,
        Instruction::Return,
    ];
    assert!(
        matches!(create_function("()V", &instructions), Err(Error::InternalError(message)) if message.contains("Expected Reference"))
    );
}
