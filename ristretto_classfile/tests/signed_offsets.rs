//! JVM branch operands are signed displacements from the opcode, measured in bytes.
#![expect(
    clippy::panic_in_result_fn,
    reason = "regression assertions use fallible bytecode fixtures"
)]
use ristretto_classfile::Result;
use ristretto_classfile::attributes::{Instruction, LookupSwitch, TableSwitch};
use ristretto_classfile::byte_reader::ByteReader;
use std::io::Cursor;

#[test]
fn short_branches_preserve_signed_operands_at_every_alignment() -> Result<()> {
    let constructors = [
        Instruction::Ifeq,
        Instruction::Ifne,
        Instruction::Iflt,
        Instruction::Ifge,
        Instruction::Ifgt,
        Instruction::Ifle,
        Instruction::If_icmpeq,
        Instruction::If_icmpne,
        Instruction::If_icmplt,
        Instruction::If_icmpge,
        Instruction::If_icmpgt,
        Instruction::If_icmple,
        Instruction::If_acmpeq,
        Instruction::If_acmpne,
        Instruction::Goto,
        Instruction::Jsr,
        Instruction::Ifnull,
        Instruction::Ifnonnull,
    ];
    for constructor in constructors {
        for offset in [i16::MIN, -257, -1, 0, 1, 257, i16::MAX] {
            for position in 0..8 {
                let instruction = constructor(offset);
                let mut bytes = Cursor::new(vec![0; position]);
                bytes.set_position(u64::try_from(position)?);
                instruction.to_bytes(&mut bytes)?;
                let encoded = bytes.into_inner();
                assert_eq!(
                    encoded.get(position + 1..),
                    Some(offset.to_be_bytes().as_slice())
                );
                let mut reader = ByteReader::new(&encoded);
                reader.set_position(position);
                assert_eq!(instruction, Instruction::from_bytes(&mut reader)?);
                assert_eq!(position + instruction.byte_len(position), reader.position());
            }
        }
    }
    Ok(())
}

#[test]
fn wide_branches_preserve_the_full_signed_range() -> Result<()> {
    for constructor in [Instruction::Goto_w, Instruction::Jsr_w] {
        for offset in [i32::MIN, -65535, -1, 0, 65535, i32::MAX] {
            let code = [Instruction::Bipush(42), constructor(offset)];
            let bytes = Instruction::encode(&code)?;
            assert_eq!(bytes.get(3..), Some(offset.to_be_bytes().as_slice()));
            assert_eq!(Instruction::decode(&bytes)?, code);
        }
    }
    Ok(())
}

#[test]
fn switches_preserve_offsets_and_opcode_relative_alignment() -> Result<()> {
    for position in 0..8 {
        for instruction in [
            Instruction::Tableswitch(Box::new(TableSwitch {
                default: -1,
                low: 0,
                high: 1,
                offsets: vec![i32::MIN, i32::MAX],
            })),
            Instruction::Lookupswitch(Box::new(LookupSwitch {
                default: -1,
                pairs: [(-3, i32::MIN), (7, i32::MAX)].into(),
            })),
        ] {
            let mut code = vec![Instruction::Nop; position];
            code.push(instruction.clone());
            let bytes = Instruction::encode(&code)?;
            assert_eq!(bytes.len(), position + instruction.byte_len(position));
            assert_eq!(Instruction::decode(&bytes)?, code);
        }
    }
    Ok(())
}

#[test]
fn mixed_width_loop_round_trips_without_relocation() -> Result<()> {
    // bipush at 0, istore_0 at 2, iinc at 3, iload_0 at 6, ifgt at 7, return at 10.
    let bytes = [0x10, 3, 0x3b, 0x84, 0, 0xff, 0x1a, 0x9d, 0xff, 0xfc, 0xb1];
    let code = Instruction::decode(&bytes)?;
    assert_eq!(code.get(4), Some(&Instruction::Ifgt(-4)));
    assert_eq!(
        Instruction::with_offsets(&code)
            .map(|(pc, _)| pc)
            .collect::<Vec<_>>(),
        [0, 2, 3, 6, 7, 10]
    );
    assert_eq!(Instruction::encode(&code)?, bytes);
    Ok(())
}
