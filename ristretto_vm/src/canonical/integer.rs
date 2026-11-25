//! Integer execution for verified scalar entries.

use super::{Error, ScalarValue, load, pop, scalar, store};
use ristretto_classfile::ConstantPool;
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
        Some(match instruction {
            Instruction::Iconst_m1 => Self::Constant(-1),
            Instruction::Iconst_0 => Self::Constant(0),
            Instruction::Iconst_1 => Self::Constant(1),
            Instruction::Iconst_2 => Self::Constant(2),
            Instruction::Iconst_3 => Self::Constant(3),
            Instruction::Iconst_4 => Self::Constant(4),
            Instruction::Iconst_5 => Self::Constant(5),
            Instruction::Bipush(n) => Self::Constant(i32::from(*n)),
            Instruction::Sipush(n) => Self::Constant(i32::from(*n)),
            Instruction::Iload_0 => Self::Load(0),
            Instruction::Iload_1 => Self::Load(1),
            Instruction::Iload_2 => Self::Load(2),
            Instruction::Iload_3 => Self::Load(3),
            Instruction::Iload(n) => Self::Load(usize::from(*n)),
            Instruction::Iload_w(n) => Self::Load(usize::from(*n)),
            Instruction::Istore_0 => Self::Store(0),
            Instruction::Istore_1 => Self::Store(1),
            Instruction::Istore_2 => Self::Store(2),
            Instruction::Istore_3 => Self::Store(3),
            Instruction::Istore(n) => Self::Store(usize::from(*n)),
            Instruction::Istore_w(n) => Self::Store(usize::from(*n)),
            Instruction::Iinc(n, inc) => Self::Increment(usize::from(*n), i32::from(*inc)),
            Instruction::Iinc_w(n, inc) => Self::Increment(usize::from(*n), i32::from(*inc)),
            Instruction::Iadd => Self::Add,
            Instruction::Isub => Self::Subtract,
            Instruction::Imul => Self::Multiply,
            Instruction::Ineg => Self::Negate,
            Instruction::Ifeq(_) => Self::BranchZero(Comparison::Equal),
            Instruction::Ifne(_) => Self::BranchZero(Comparison::Unequal),
            Instruction::Iflt(_) => Self::BranchZero(Comparison::Less),
            Instruction::Ifle(_) => Self::BranchZero(Comparison::LessEqual),
            Instruction::Ifgt(_) => Self::BranchZero(Comparison::Greater),
            Instruction::Ifge(_) => Self::BranchZero(Comparison::GreaterEqual),
            Instruction::If_icmpeq(_) => Self::BranchPair(Comparison::Equal),
            Instruction::If_icmpne(_) => Self::BranchPair(Comparison::Unequal),
            Instruction::If_icmplt(_) => Self::BranchPair(Comparison::Less),
            Instruction::If_icmple(_) => Self::BranchPair(Comparison::LessEqual),
            Instruction::If_icmpgt(_) => Self::BranchPair(Comparison::Greater),
            Instruction::If_icmpge(_) => Self::BranchPair(Comparison::GreaterEqual),
            Instruction::Goto(_) | Instruction::Goto_w(_) => Self::Jump,
            Instruction::Ireturn
            | Instruction::Lreturn
            | Instruction::Freturn
            | Instruction::Dreturn => Self::Return,
            _ => return None,
        })
    }
}

pub(super) fn admitted(op: &Instruction, pool: &ConstantPool<'_>) -> bool {
    match op {
        Instruction::Ldc(index) => scalar::constant(pool, u16::from(*index), false).is_ok(),
        Instruction::Ldc_w(index) => scalar::constant(pool, *index, false).is_ok(),
        Instruction::Ldc2_w(index) => scalar::constant(pool, *index, true).is_ok(),
        _ => Operation::decode(op).is_some() || scalar::supports(op),
    }
}

pub(super) fn run(
    method: &Method,
    pool: &ConstantPool<'_>,
    argument: i32,
    limit: usize,
) -> Result<ScalarValue, Error> {
    run_counted(method, pool, argument, limit).map(|(value, _)| value)
}

pub(super) fn run_counted(
    method: &Method,
    pool: &ConstantPool<'_>,
    argument: i32,
    limit: usize,
) -> Result<(ScalarValue, usize), Error> {
    let mut locals = vec![None; method.max_locals()];
    let mut stack = Vec::with_capacity(method.max_stack());
    let mut pc = 0;
    let invalid = || Error::Invalid(method.signature());
    *locals.first_mut().ok_or_else(invalid)? = Some(ScalarValue::Int(argument));
    for count in 0..limit {
        let op = method.code().get(pc).ok_or_else(invalid)?;
        let mut jump = false;
        match Operation::decode(op) {
            Some(Operation::Constant(value)) => stack.push(ScalarValue::Int(value)),
            Some(Operation::Load(index)) => load(&locals, &mut stack, index)?,
            Some(Operation::Store(index)) => store(&mut locals, &mut stack, index)?,
            Some(Operation::Increment(index, delta)) => {
                let slot = locals
                    .get_mut(index)
                    .and_then(Option::as_mut)
                    .ok_or_else(invalid)?;
                *slot = ScalarValue::Int(slot.int()?.wrapping_add(delta));
            }
            Some(Operation::Negate) => {
                let value = pop(&mut stack)?.int()?.wrapping_neg();
                stack.push(ScalarValue::Int(value));
            }
            Some(operation @ (Operation::Add | Operation::Subtract | Operation::Multiply)) => {
                let right = pop(&mut stack)?.int()?;
                let left = pop(&mut stack)?.int()?;
                stack.push(ScalarValue::Int(match operation {
                    Operation::Add => left.wrapping_add(right),
                    Operation::Subtract => left.wrapping_sub(right),
                    _ => left.wrapping_mul(right),
                }));
            }
            Some(Operation::BranchZero(test)) => jump = test.test(pop(&mut stack)?.int()?, 0),
            Some(Operation::BranchPair(test)) => {
                let right = pop(&mut stack)?.int()?;
                jump = test.test(pop(&mut stack)?.int()?, right);
            }
            Some(Operation::Jump) => jump = true,
            Some(Operation::Return) => {
                return Ok((
                    scalar::normalize_return(
                        pop(&mut stack)?,
                        method.return_type().ok_or_else(invalid)?,
                    ),
                    count + 1,
                ));
            }
            None => match op {
                Instruction::Ldc(index) => {
                    stack.push(scalar::constant(pool, u16::from(*index), false)?);
                }
                Instruction::Ldc_w(index) => stack.push(scalar::constant(pool, *index, false)?),
                Instruction::Ldc2_w(index) => stack.push(scalar::constant(pool, *index, true)?),
                _ => scalar::execute(op, &mut locals, &mut stack)?,
            },
        }
        pc = if jump {
            method.branch_target_index(pc).ok_or_else(invalid)?
        } else {
            pc + 1
        };
    }
    Err(Error::Limit("instructions"))
}
