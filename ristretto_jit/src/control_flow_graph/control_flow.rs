use ristretto_classfile::attributes::Instruction;

pub(crate) trait InstructionControlFlow {
    /// Return true if the instruction changes the program control flow.
    fn changes_control_flow(&self) -> bool;
}

impl InstructionControlFlow for Instruction {
    /// Return true if the instruction changes the program control flow.
    fn changes_control_flow(&self) -> bool {
        matches!(
            self,
            Instruction::Ifeq(..)
                | Instruction::Ifne(..)
                | Instruction::Iflt(..)
                | Instruction::Ifge(..)
                | Instruction::Ifgt(..)
                | Instruction::Ifle(..)
                | Instruction::If_icmpeq(..)
                | Instruction::If_icmpne(..)
                | Instruction::If_icmplt(..)
                | Instruction::If_icmpge(..)
                | Instruction::If_icmpgt(..)
                | Instruction::If_icmple(..)
                | Instruction::If_acmpeq(..)
                | Instruction::If_acmpne(..)
                | Instruction::Goto(..)
                | Instruction::Goto_w(..)
                | Instruction::Jsr(..)
                | Instruction::Jsr_w(..)
                | Instruction::Tableswitch { .. }
                | Instruction::Lookupswitch { .. }
                | Instruction::Ret(..)
                | Instruction::Ret_w(..)
                | Instruction::Return
                | Instruction::Ireturn
                | Instruction::Lreturn
                | Instruction::Freturn
                | Instruction::Dreturn
                | Instruction::Areturn
                | Instruction::Athrow
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use ristretto_classfile::attributes::Instruction;

    #[test]
    fn test_changes_control_flow() {
        let instructions = vec![
            Instruction::Ifeq(0),
            Instruction::Ifne(0),
            Instruction::Iflt(0),
            Instruction::Ifge(0),
            Instruction::Ifgt(0),
            Instruction::Ifle(0),
            Instruction::If_icmpeq(0),
            Instruction::If_icmpne(0),
            Instruction::If_icmplt(0),
            Instruction::If_icmpge(0),
            Instruction::If_icmpgt(0),
            Instruction::If_icmple(0),
            Instruction::If_acmpeq(0),
            Instruction::If_acmpne(0),
            Instruction::Goto(0),
            Instruction::Goto_w(0),
            Instruction::Jsr(0),
            Instruction::Jsr_w(0),
            Instruction::Tableswitch {
                default: 0,
                low: 0,
                high: 0,
                offsets: vec![0],
            },
            Instruction::Lookupswitch {
                default: 0,
                pairs: IndexMap::new(),
            },
            Instruction::Ret(0),
            Instruction::Ret_w(0),
            Instruction::Return,
            Instruction::Ireturn,
            Instruction::Lreturn,
            Instruction::Freturn,
            Instruction::Dreturn,
            Instruction::Areturn,
            Instruction::Athrow,
        ];
        for instruction in instructions {
            assert!(instruction.changes_control_flow());
        }
    }

    #[test]
    fn test_does_not_change_control_flow() {
        let instructions = vec![
            Instruction::Nop,
            Instruction::Aconst_null,
            Instruction::Iconst_m1,
            Instruction::Iconst_0,
            Instruction::Iconst_1,
            Instruction::Iconst_2,
            Instruction::Iconst_3,
            Instruction::Iconst_4,
            Instruction::Iconst_5,
            Instruction::Lconst_0,
            Instruction::Lconst_1,
            Instruction::Fconst_0,
            Instruction::Fconst_1,
            Instruction::Fconst_2,
            Instruction::Dconst_0,
            Instruction::Dconst_1,
        ];
        for instruction in instructions {
            assert!(!instruction.changes_control_flow());
        }
    }
}
