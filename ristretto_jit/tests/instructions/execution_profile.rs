//! Execute generated code: declarations or compiled-but-unused code cannot count.
use crate::util::create_function;
use indexmap::IndexMap;
use ristretto_classfile::attributes::{Instruction, LookupSwitch, TableSwitch};
use ristretto_jit::{Result, Value};

#[test]
fn taken_conditional_edges_and_clones_share_exact_counts() -> Result<()> {
    let function = create_function(
        "(I)I",
        &[
            Instruction::Iinc(0, -1), // 0: a backedge to bytecode 0 must not reload ABI arguments
            Instruction::Iload_0,     // 3
            Instruction::Ifgt(-4),    // 4 -> 0 only while positive
            Instruction::Iload_0,     // 7
            Instruction::Ireturn,
        ],
    )?;
    assert_eq!(function.backedge_count(), 0);
    let clone = function.clone();
    for count in [1, 2, 7] {
        assert_eq!(
            clone.execute(&[Value::I32(count)], std::ptr::null())?,
            Some(Value::I32(0))
        );
    }
    assert_eq!(function.execution_count(), 3);
    assert_eq!(function.backedge_count(), 7);
    drop(function);
    assert_eq!(
        clone.execute(&[Value::I32(3)], std::ptr::null())?,
        Some(Value::I32(0))
    );
    assert_eq!(clone.backedge_count(), 9);
    Ok(())
}

#[test]
fn goto_and_goto_w_count_only_traversed_backedges() -> Result<()> {
    for wide in [false, true] {
        let function = create_function(
            "(I)I",
            &[
                Instruction::Iload_0,                         // 0
                Instruction::Ifle(if wide { 11 } else { 9 }), // 1 -> return path
                Instruction::Iinc(0, -1),                     // 4
                if wide {
                    Instruction::Goto_w(-7)
                } else {
                    Instruction::Goto(-7)
                }, // 7 -> 0
                Instruction::Iload_0,
                Instruction::Ireturn,
            ],
        )?;
        for count in [0, 3, 7] {
            assert_eq!(
                function.execute(&[Value::I32(count)], std::ptr::null())?,
                Some(Value::I32(0))
            );
        }
        assert_eq!(function.execution_count(), 3);
        assert_eq!(function.backedge_count(), 10);
    }
    Ok(())
}

#[test]
fn switches_preserve_operands_and_count_one_selected_backward_destination() -> Result<()> {
    for switch in [
        Instruction::Tableswitch(Box::new(TableSwitch {
            low: 0,
            high: 1,
            default: -5,
            offsets: vec![-5, 23],
        })),
        Instruction::Lookupswitch(Box::new(LookupSwitch {
            default: -5,
            pairs: IndexMap::from([(0, -5), (1, 27)]),
        })),
    ] {
        let function = create_function(
            "(I)I",
            &[
                Instruction::Iconst_5, // 0: operand below the switch key
                Instruction::Goto(7),  // 1 -> 8
                Instruction::Ireturn,  // 4: backward destination preserves 5
                Instruction::Nop,
                Instruction::Nop,
                Instruction::Nop,
                Instruction::Iload_0, // 8
                switch,               // 9 -> 4 or forward return
                Instruction::Iconst_2,
                Instruction::Iadd,
                Instruction::Ireturn,
            ],
        )?;
        for (key, result, backedges) in [(1, 7, 0), (0, 5, 1), (99, 5, 2), (1, 7, 2)] {
            assert_eq!(
                function.execute(&[Value::I32(key)], std::ptr::null())?,
                Some(Value::I32(result))
            );
            assert_eq!(function.backedge_count(), backedges);
        }
    }
    Ok(())
}

#[test]
fn concurrent_calls_keep_all_backedges() -> Result<()> {
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
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let function = &function;
            scope.spawn(move || {
                for _ in 0..50 {
                    assert_eq!(
                        function
                            .execute(&[Value::I32(101)], std::ptr::null())
                            .expect("generated loop"),
                        Some(Value::I32(0))
                    );
                }
            });
        }
    });
    assert_eq!(function.execution_count(), 200);
    assert_eq!(function.backedge_count(), 20_000);
    Ok(())
}
