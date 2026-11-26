use crate::util::create_function;
use indexmap::IndexMap;
use ristretto_classfile::attributes::{Instruction, LookupSwitch, TableSwitch};
use ristretto_jit::{ExecutionStep, Result, Value};
use std::num::NonZeroU32;

#[test]
fn bci_zero_resumes_locals_without_reloading_arguments() -> Result<()> {
    let function = create_function(
        "(I)I",
        &[
            Instruction::Iinc(0, -1),
            Instruction::Iload_0,
            Instruction::Ifgt(-4),
            Instruction::Iload_0,
            Instruction::Ireturn,
        ],
    )?;
    assert!(function.supports_cooperation());
    let mut activation = function.activation(&[Value::I32(7)])?;
    assert_eq!(activation.suspended_bci(), None);
    let clone = function.clone();
    drop(function);
    for _ in 0..6 {
        assert_eq!(activation.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
        assert_eq!(activation.suspended_bci(), Some(0));
    }
    assert_eq!(
        activation.resume(NonZeroU32::MIN)?,
        ExecutionStep::Returned(Some(Value::I32(0)))
    );
    assert!(activation.resume(NonZeroU32::MIN).is_err());
    assert_eq!(activation.suspended_bci(), None);
    #[cfg(feature = "execution-profile")]
    {
        assert_eq!(clone.execution_count(), 1);
        assert_eq!(clone.resume_count(), 6);
        assert_eq!(clone.backedge_count(), 6);
    }
    drop(clone);
    Ok(())
}

#[test]
fn wide_operands_survive_each_suspended_backedge() -> Result<()> {
    let function = create_function(
        "(IJ)J",
        &[
            Instruction::Lload_1,  // 0: accumulator stays on the operand stack
            Instruction::Iload_0,  // 1
            Instruction::Ifle(11), // 2 -> 13
            Instruction::Lconst_1,
            Instruction::Ladd,        // 5, 6
            Instruction::Iinc(0, -1), // 7
            Instruction::Goto(-9),    // 10 -> 1
            Instruction::Lreturn,     // 13
        ],
    )?;
    for initial in [i64::MIN, -1, 0, i64::MAX] {
        let mut activation = function.activation(&[Value::I32(10), Value::I64(initial)])?;
        for _ in 0..10 {
            assert_eq!(activation.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
        }
        assert_eq!(
            activation.resume(NonZeroU32::MIN)?,
            ExecutionStep::Returned(Some(Value::I64(initial.wrapping_add(10))))
        );
    }
    Ok(())
}

#[test]
fn floating_operands_preserve_nan_payloads_and_signed_zero() -> Result<()> {
    for double in [false, true] {
        let function = create_function(
            if double { "(ID)D" } else { "(IF)F" },
            &[
                if double {
                    Instruction::Dload_1
                } else {
                    Instruction::Fload_1
                },
                Instruction::Iload_0,
                Instruction::Ifle(9),
                Instruction::Iinc(0, -1),
                Instruction::Goto(-7),
                if double {
                    Instruction::Dreturn
                } else {
                    Instruction::Freturn
                },
            ],
        )?;
        for seed in if double {
            vec![
                Value::F64(f64::from_bits(0x7ff8_0000_0000_0042)),
                Value::F64(-0.0),
            ]
        } else {
            vec![Value::F32(f32::from_bits(0x7fc0_0042)), Value::F32(-0.0)]
        } {
            let mut activation = function.activation(&[Value::I32(5), seed.clone()])?;
            for _ in 0..5 {
                assert_eq!(activation.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
            }
            match (activation.resume(NonZeroU32::MIN)?, seed) {
                (ExecutionStep::Returned(Some(Value::F32(actual))), Value::F32(expected)) => {
                    assert_eq!(actual.to_bits(), expected.to_bits());
                }
                (ExecutionStep::Returned(Some(Value::F64(actual))), Value::F64(expected)) => {
                    assert_eq!(actual.to_bits(), expected.to_bits());
                }
                value => panic!("unexpected result: {value:?}"),
            }
        }
    }
    Ok(())
}

#[test]
fn typed_local_reuse_survives_resumption() -> Result<()> {
    let function = create_function(
        "(I)I",
        &[
            Instruction::Iconst_0,
            Instruction::Istore_1, // 0, 1
            Instruction::Iload_0,
            Instruction::Ifle(16), // 2, 3 -> 19
            Instruction::Iload_1,
            Instruction::Fconst_1,
            Instruction::Fstore_1, // 6, 7, 8
            Instruction::Fload_1,
            Instruction::F2i,
            Instruction::Iadd,
            Instruction::Istore_1, // 9..12
            Instruction::Iinc(0, -1),
            Instruction::Goto(-14), // 13, 16 -> 2
            Instruction::Iload_1,
            Instruction::Ireturn, // 19, 20
        ],
    )?;
    let mut activation = function.activation(&[Value::I32(17)])?;
    for _ in 0..17 {
        assert_eq!(activation.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
    }
    assert_eq!(
        activation.resume(NonZeroU32::MIN)?,
        ExecutionStep::Returned(Some(Value::I32(17)))
    );
    Ok(())
}

#[test]
fn switch_backedges_resume_once_with_preserved_operands() -> Result<()> {
    for switch in [
        Instruction::Tableswitch(Box::new(TableSwitch {
            low: 0,
            high: 1,
            default: -4,
            offsets: vec![-4, 23],
        })),
        Instruction::Lookupswitch(Box::new(LookupSwitch {
            default: -4,
            pairs: IndexMap::from([(0, -4), (1, 27)]),
        })),
    ] {
        let function = create_function(
            "(I)I",
            &[
                Instruction::Iconst_5,
                Instruction::Iinc(0, 1),
                Instruction::Iload_0,
                switch,
                Instruction::Ireturn,
            ],
        )?;
        let mut activation = function.activation(&[Value::I32(-2)])?;
        for _ in 0..2 {
            assert_eq!(activation.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
        }
        assert_eq!(
            activation.resume(NonZeroU32::MIN)?,
            ExecutionStep::Returned(Some(Value::I32(5)))
        );
    }
    Ok(())
}

#[test]
fn nested_loops_share_fuel_and_restore_each_destination() -> Result<()> {
    let function = create_function(
        "(II)I",
        &[
            Instruction::Iconst_0,
            Instruction::Istore_2, // 0, 1
            Instruction::Iload_0,
            Instruction::Ifle(25), // 2, 3 -> 28
            Instruction::Iload_1,
            Instruction::Istore_3, // 6, 7
            Instruction::Iload_3,
            Instruction::Ifle(13), // 8, 9 -> 22
            Instruction::Iload_2,
            Instruction::Iconst_1,
            Instruction::Iadd,
            Instruction::Istore_2, // 12..15
            Instruction::Iinc(3, -1),
            Instruction::Goto(-11), // 16, 19 -> 8
            Instruction::Iinc(0, -1),
            Instruction::Goto(-23), // 22, 25 -> 2
            Instruction::Iload_2,
            Instruction::Ireturn, // 28, 29
        ],
    )?;
    for budget in [1, 2, 7, 100] {
        let budget = NonZeroU32::new(budget)
            .ok_or_else(|| ristretto_jit::Error::InternalError("zero test budget".to_owned()))?;
        let mut activation = function.activation(&[Value::I32(3), Value::I32(4)])?;
        let mut yields = 0;
        loop {
            match activation.resume(budget)? {
                ExecutionStep::Yielded => {
                    yields += 1;
                    assert!(yields <= 15);
                }
                result @ (ExecutionStep::Returned(_)
                | ExecutionStep::Call(_)
                | ExecutionStep::Monitor(_)
                | ExecutionStep::Allocation(_)
                | ExecutionStep::Field(_)
                | ExecutionStep::TypeCheck(_)
                | ExecutionStep::Constant(_)
                | ExecutionStep::ExceptionMatch(_)
                | ExecutionStep::ArrayAccess(_)
                | ExecutionStep::ArrayStore(_)) => {
                    assert_eq!(result, ExecutionStep::Returned(Some(Value::I32(12))));
                    break;
                }
            }
        }
        assert_eq!(yields, 15 / budget);
    }
    Ok(())
}

#[test]
fn independent_activations_move_between_threads_and_own_their_code() -> Result<()> {
    let function = create_function(
        "(I)I",
        &[
            Instruction::Iinc(0, -1),
            Instruction::Iload_0,
            Instruction::Ifgt(-4),
            Instruction::Iload_0,
            Instruction::Ireturn,
        ],
    )?;
    let mut first = function.activation(&[Value::I32(3)])?;
    let mut second = function.activation(&[Value::I32(5)])?;
    drop(function);
    assert_eq!(first.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
    std::thread::spawn(move || -> Result<()> {
        assert_eq!(first.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
        assert_eq!(
            first.resume(NonZeroU32::MIN)?,
            ExecutionStep::Returned(Some(Value::I32(0)))
        );
        for _ in 0..4 {
            assert_eq!(second.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
        }
        assert_eq!(
            second.resume(NonZeroU32::MIN)?,
            ExecutionStep::Returned(Some(Value::I32(0)))
        );
        Ok(())
    })
    .join()
    .map_err(|_| {
        ristretto_jit::Error::InternalError("native continuation worker panicked".to_owned())
    })??;
    Ok(())
}

#[test]
fn infinite_self_edge_is_bounded_and_droppable() -> Result<()> {
    let function = create_function("()V", &[Instruction::Goto(0)])?;
    assert!(function.activation(&[Value::I32(1)]).is_err());
    for _ in 0..2 {
        let mut activation = function.activation(&[])?;
        for _ in 0..10 {
            assert_eq!(activation.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
        }
        drop(activation);
    }
    Ok(())
}
