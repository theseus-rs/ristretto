//! Integer execution for verified scalar entries.

use super::{Error, Result};
use ristretto_classfile::attributes::Instruction;
use ristretto_classloader::Method;

#[derive(Clone, Copy)]
pub(super) enum Comparison {
    Equal,
    Unequal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl Comparison {
    fn test(self, a: i32, b: i32) -> bool {
        match self {
            Self::Equal => a == b,
            Self::Unequal => a != b,
            Self::Less => a < b,
            Self::LessEqual => a <= b,
            Self::Greater => a > b,
            Self::GreaterEqual => a >= b,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum Operation {
    Constant(i32),
    Load(usize),
    Store(usize),
    Increment(usize, i32),
    Add,
    Subtract,
    Multiply,
    Negate,
    BranchZero(Comparison),
    BranchPair(Comparison),
    Jump,
    Return,
}

impl Operation {
    pub(super) fn decode(instruction: &Instruction) -> Option<Self> {
        use Instruction as I;
        Some(match instruction {
            I::Iconst_m1 => Self::Constant(-1),
            I::Iconst_0 => Self::Constant(0),
            I::Iconst_1 => Self::Constant(1),
            I::Iconst_2 => Self::Constant(2),
            I::Iconst_3 => Self::Constant(3),
            I::Iconst_4 => Self::Constant(4),
            I::Iconst_5 => Self::Constant(5),
            I::Bipush(n) => Self::Constant(i32::from(*n)),
            I::Sipush(n) => Self::Constant(i32::from(*n)),
            I::Iload_0 => Self::Load(0),
            I::Iload_1 => Self::Load(1),
            I::Iload_2 => Self::Load(2),
            I::Iload_3 => Self::Load(3),
            I::Iload(n) => Self::Load(usize::from(*n)),
            I::Iload_w(n) => Self::Load(usize::from(*n)),
            I::Istore_0 => Self::Store(0),
            I::Istore_1 => Self::Store(1),
            I::Istore_2 => Self::Store(2),
            I::Istore_3 => Self::Store(3),
            I::Istore(n) => Self::Store(usize::from(*n)),
            I::Istore_w(n) => Self::Store(usize::from(*n)),
            I::Iinc(n, inc) => Self::Increment(usize::from(*n), i32::from(*inc)),
            I::Iinc_w(n, inc) => Self::Increment(usize::from(*n), i32::from(*inc)),
            I::Iadd => Self::Add,
            I::Isub => Self::Subtract,
            I::Imul => Self::Multiply,
            I::Ineg => Self::Negate,
            I::Ifeq(_) => Self::BranchZero(Comparison::Equal),
            I::Ifne(_) => Self::BranchZero(Comparison::Unequal),
            I::Iflt(_) => Self::BranchZero(Comparison::Less),
            I::Ifle(_) => Self::BranchZero(Comparison::LessEqual),
            I::Ifgt(_) => Self::BranchZero(Comparison::Greater),
            I::Ifge(_) => Self::BranchZero(Comparison::GreaterEqual),
            I::If_icmpeq(_) => Self::BranchPair(Comparison::Equal),
            I::If_icmpne(_) => Self::BranchPair(Comparison::Unequal),
            I::If_icmplt(_) => Self::BranchPair(Comparison::Less),
            I::If_icmple(_) => Self::BranchPair(Comparison::LessEqual),
            I::If_icmpgt(_) => Self::BranchPair(Comparison::Greater),
            I::If_icmpge(_) => Self::BranchPair(Comparison::GreaterEqual),
            I::Goto(_) | I::Goto_w(_) => Self::Jump,
            I::Ireturn => Self::Return,
            _ => return None,
        })
    }
}

pub(super) fn run(method: &Method, argument: i32, limit: usize) -> Result<i32> {
    let mut locals = vec![0_i32; method.max_locals()];
    let mut stack = Vec::with_capacity(method.max_stack());
    let mut pc = 0;
    let invalid = || Error::Invalid(method.signature());
    *locals.first_mut().ok_or_else(invalid)? = argument;
    for _ in 0..limit {
        let instruction = method
            .code()
            .get(pc)
            .and_then(Operation::decode)
            .ok_or_else(invalid)?;
        let mut jump = false;
        match instruction {
            Operation::Constant(value) => stack.push(value),
            Operation::Load(index) => stack.push(*locals.get(index).ok_or_else(invalid)?),
            Operation::Store(index) => {
                *locals.get_mut(index).ok_or_else(invalid)? = stack.pop().ok_or_else(invalid)?;
            }
            Operation::Increment(index, delta) => {
                let slot = locals.get_mut(index).ok_or_else(invalid)?;
                *slot = slot.wrapping_add(delta);
            }
            Operation::Negate => {
                let value = stack.last_mut().ok_or_else(invalid)?;
                *value = value.wrapping_neg();
            }
            Operation::Add | Operation::Subtract | Operation::Multiply => {
                let right = stack.pop().ok_or_else(invalid)?;
                let left = stack.last_mut().ok_or_else(invalid)?;
                *left = match instruction {
                    Operation::Add => left.wrapping_add(right),
                    Operation::Subtract => left.wrapping_sub(right),
                    _ => left.wrapping_mul(right),
                };
            }
            Operation::BranchZero(test) => jump = test.test(stack.pop().ok_or_else(invalid)?, 0),
            Operation::BranchPair(test) => {
                let right = stack.pop().ok_or_else(invalid)?;
                jump = test.test(stack.pop().ok_or_else(invalid)?, right);
            }
            Operation::Jump => jump = true,
            Operation::Return => return stack.pop().ok_or_else(invalid),
        }
        pc = if jump {
            method.branch_target_index(pc).ok_or_else(invalid)?
        } else {
            pc + 1
        };
    }
    Err(Error::Limit("instructions"))
}
