//! Backward reference-local liveness at bytecode boundaries. Exception edges
//! conservatively include every protected instruction, independently of local
//! definitions on its normal successor. Operand roots are supplied by lowering.
use crate::Error::{InternalError, InvalidBlockAddress, UnsupportedMethod};
use crate::Result;
use crate::control_flow_graph::falls_through;
use ahash::AHashMap;
use ristretto_classfile::attributes::{ExceptionTableEntry, Instruction};
use std::collections::VecDeque;

const MAX_STATE_BYTES: usize = 8 * 1024 * 1024;
const MAX_EDGES: usize = 1_000_000;

pub(super) struct ReferenceLiveness {
    locals: Vec<usize>,
    addresses: AHashMap<usize, usize>,
    words: usize,
    live: Vec<u64>,
}

impl ReferenceLiveness {
    pub(super) fn new(
        instructions: &[Instruction],
        exceptions: &[ExceptionTableEntry],
        locals: Vec<usize>,
    ) -> Result<Self> {
        let addresses: AHashMap<_, _> = Instruction::with_offsets(instructions)
            .enumerate()
            .map(|(index, (address, _))| (address, index))
            .collect();
        let words = locals.len().div_ceil(64);
        let cells = instructions.len().checked_mul(words).ok_or_else(|| {
            UnsupportedMethod("reference liveness state exceeds analysis limit".to_owned())
        })?;
        if cells > MAX_STATE_BYTES / size_of::<u64>() {
            return Err(UnsupportedMethod(
                "reference liveness state exceeds analysis limit".to_owned(),
            ));
        }
        let mut analysis = Self {
            locals,
            addresses,
            words,
            live: vec![0; cells],
        };
        if words == 0 {
            return Ok(analysis);
        }
        let graph = Successors::new(instructions, exceptions, &analysis.addresses)?;
        analysis.solve(instructions, &graph)?;
        Ok(analysis)
    }

    pub(super) fn at(&self, address: usize) -> Result<impl Iterator<Item = usize> + '_> {
        let row = *self
            .addresses
            .get(&address)
            .ok_or(InvalidBlockAddress(address))?;
        let bits = self
            .live
            .get(row * self.words..(row + 1) * self.words)
            .ok_or_else(|| InternalError("invalid reference liveness row".to_owned()))?;
        Ok(self.locals.chunks(64).zip(bits).flat_map(|(locals, bits)| {
            locals
                .iter()
                .enumerate()
                .filter_map(move |(bit, local)| (bits & (1 << bit) != 0).then_some(*local))
        }))
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "private graph indices are validated instruction indices; dense local bits and checked row dimensions bound every access"
    )]
    fn solve(&mut self, instructions: &[Instruction], graph: &Successors) -> Result<()> {
        let mut predecessors = vec![Vec::new(); instructions.len()];
        for index in 0..instructions.len() {
            for &target in graph.normal[index].iter().chain(&graph.exception[index]) {
                predecessors[target].push(index);
            }
        }
        let uses = instructions
            .iter()
            .map(|instruction| {
                reference_use(instruction)
                    .map(|local| {
                        self.locals.binary_search(&local).map_err(|_| {
                            InternalError(format!("reference local {local} was not declared"))
                        })
                    })
                    .transpose()
            })
            .collect::<Result<Vec<_>>>()?;
        let definitions = instructions
            .iter()
            .map(|instruction| {
                local_definitions(instruction)
                    .map(|local| local.and_then(|local| self.locals.binary_search(&local).ok()))
            })
            .collect::<Vec<_>>();
        let mut pending: VecDeque<_> = (0..instructions.len()).rev().collect();
        let mut queued = vec![true; instructions.len()];
        let mut next = vec![0; self.words];
        while let Some(index) = pending.pop_front() {
            queued[index] = false;
            next.fill(0);
            for &target in &graph.normal[index] {
                union(
                    &mut next,
                    &self.live[target * self.words..(target + 1) * self.words],
                );
            }
            for bit in definitions[index].iter().flatten() {
                next[bit / 64] &= !(1 << (bit % 64));
            }
            if let Some(bit) = uses[index] {
                next[bit / 64] |= 1 << (bit % 64);
            }
            // A throw observes incoming locals. Normal definitions cannot kill
            // references needed only by a handler; merge these after the kills.
            for &target in &graph.exception[index] {
                union(
                    &mut next,
                    &self.live[target * self.words..(target + 1) * self.words],
                );
            }
            let current = &mut self.live[index * self.words..(index + 1) * self.words];
            if current != next {
                current.copy_from_slice(&next);
                for &predecessor in &predecessors[index] {
                    if !queued[predecessor] {
                        queued[predecessor] = true;
                        pending.push_back(predecessor);
                    }
                }
            }
        }
        Ok(())
    }
}

fn union(destination: &mut [u64], source: &[u64]) {
    for (destination, source) in destination.iter_mut().zip(source) {
        *destination |= source;
    }
}

struct Successors {
    normal: Vec<Vec<usize>>,
    exception: Vec<Vec<usize>>,
}

impl Successors {
    fn new(
        instructions: &[Instruction],
        exceptions: &[ExceptionTableEntry],
        by_address: &AHashMap<usize, usize>,
    ) -> Result<Self> {
        let mut graph = Self {
            normal: vec![Vec::new(); instructions.len()],
            exception: vec![Vec::new(); instructions.len()],
        };
        let mut edge_count = 0;
        let addresses = Instruction::with_offsets(instructions)
            .map(|(address, _)| address)
            .collect::<Vec<_>>();
        for (index, ((&address, instruction), successors)) in addresses
            .iter()
            .zip(instructions)
            .zip(&mut graph.normal)
            .enumerate()
        {
            for relative in instruction.branch_offsets() {
                let target = usize::try_from(i64::try_from(address)? + i64::from(relative))?;
                let target = *by_address.get(&target).ok_or(InvalidBlockAddress(target))?;
                add_edge(successors, target, &mut edge_count)?;
            }
            if falls_through(instruction) && index + 1 < instructions.len() {
                add_edge(successors, index + 1, &mut edge_count)?;
            }
            successors.sort_unstable();
            successors.dedup();
        }
        // Iterate protected ranges directly, not every handler for every BCI.
        // Bound edge storage even for heavily overlapping exception tables.
        for entry in exceptions {
            let handler = usize::from(entry.handler_pc);
            let handler = *by_address
                .get(&handler)
                .ok_or(InvalidBlockAddress(handler))?;
            let start = addresses.partition_point(|&pc| pc < usize::from(entry.range_pc.start));
            let end = addresses.partition_point(|&pc| pc < usize::from(entry.range_pc.end));
            for successors in graph.exception.iter_mut().take(end).skip(start) {
                add_edge(successors, handler, &mut edge_count)?;
            }
        }
        for successors in &mut graph.exception {
            successors.sort_unstable();
            successors.dedup();
        }
        Ok(graph)
    }
}

fn add_edge(successors: &mut Vec<usize>, target: usize, count: &mut usize) -> Result<()> {
    if *count >= MAX_EDGES {
        return Err(UnsupportedMethod(
            "reference liveness graph exceeds analysis limit".to_owned(),
        ));
    }
    successors.push(target);
    *count += 1;
    Ok(())
}

fn reference_use(instruction: &Instruction) -> Option<usize> {
    match instruction {
        Instruction::Aload(index) => Some(usize::from(*index)),
        Instruction::Aload_w(index) => Some(usize::from(*index)),
        Instruction::Aload_0 => Some(0),
        Instruction::Aload_1 => Some(1),
        Instruction::Aload_2 => Some(2),
        Instruction::Aload_3 => Some(3),
        _ => None,
    }
}

/// A wide store invalidates both Java local slots. Primitive writes also kill
/// the previous reference kind, even though lowering uses independent SSA vars.
fn local_definitions(instruction: &Instruction) -> [Option<usize>; 2] {
    let (index, wide) = match instruction {
        Instruction::Istore(index)
        | Instruction::Fstore(index)
        | Instruction::Astore(index)
        | Instruction::Iinc(index, _) => (usize::from(*index), false),
        Instruction::Istore_w(index)
        | Instruction::Fstore_w(index)
        | Instruction::Astore_w(index)
        | Instruction::Iinc_w(index, _) => (usize::from(*index), false),
        Instruction::Istore_0 | Instruction::Fstore_0 | Instruction::Astore_0 => (0, false),
        Instruction::Istore_1 | Instruction::Fstore_1 | Instruction::Astore_1 => (1, false),
        Instruction::Istore_2 | Instruction::Fstore_2 | Instruction::Astore_2 => (2, false),
        Instruction::Istore_3 | Instruction::Fstore_3 | Instruction::Astore_3 => (3, false),
        Instruction::Lstore(index) | Instruction::Dstore(index) => (usize::from(*index), true),
        Instruction::Lstore_w(index) | Instruction::Dstore_w(index) => (usize::from(*index), true),
        Instruction::Lstore_0 | Instruction::Dstore_0 => (0, true),
        Instruction::Lstore_1 | Instruction::Dstore_1 => (1, true),
        Instruction::Lstore_2 | Instruction::Dstore_2 => (2, true),
        Instruction::Lstore_3 | Instruction::Dstore_3 => (3, true),
        _ => return [None, None],
    };
    [Some(index), wide.then_some(index + 1)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use Instruction::*;

    fn snapshot(
        code: &[Instruction],
        handlers: &[ExceptionTableEntry],
        locals: Vec<usize>,
    ) -> Result<Vec<Vec<usize>>> {
        let analysis = ReferenceLiveness::new(code, handlers, locals)?;
        Instruction::with_offsets(code)
            .map(|(pc, _)| Ok(analysis.at(pc)?.collect()))
            .collect()
    }

    #[test]
    fn last_read_releases_the_local_before_primitive_slot_reuse() -> Result<()> {
        assert_eq!(
            snapshot(&[Aload_0, Pop, Iconst_0, Istore_0, Return], &[], vec![0])?,
            vec![vec![0], vec![], vec![], vec![], vec![]],
        );
        Ok(())
    }

    #[test]
    fn branch_keeps_the_incoming_reference_only_on_the_path_that_reads_it() -> Result<()> {
        assert_eq!(
            snapshot(
                &[
                    Iload_1,     // 0
                    Ifeq(8),     // 1 -> 9
                    Aconst_null, // 4
                    Astore_0,    // 5
                    Goto(4),     // 6 -> 10
                    Nop,         // 9: retains incoming reference
                    Aload_0,     // 10: old or replacement reference
                    Pop,
                    Return,
                ],
                &[],
                vec![0],
            )?,
            vec![
                vec![0],
                vec![0],
                vec![],
                vec![],
                vec![0],
                vec![0],
                vec![0],
                vec![],
                vec![]
            ],
        );
        Ok(())
    }

    #[test]
    fn backward_edge_carries_a_reference_to_the_next_iteration() -> Result<()> {
        let code = [
            Iload_1,
            Ifeq(11),
            Aload_0,
            Pop,
            Iinc(1, -1),
            Goto(-9),
            Return,
        ];
        assert_eq!(
            snapshot(&code, &[], vec![0])?,
            vec![vec![0], vec![0], vec![0], vec![0], vec![0], vec![0], vec![]],
        );
        Ok(())
    }

    #[test]
    fn handler_only_use_is_not_killed_by_a_normal_definition() -> Result<()> {
        let code = [
            Aload_0, Pop, Iconst_0, Istore_0, Return, Pop, Aload_0, Pop, Return,
        ];
        assert_eq!(
            snapshot(
                &code,
                &[ExceptionTableEntry {
                    range_pc: 2..4,
                    handler_pc: 5,
                    catch_type: 0
                }],
                vec![0],
            )?,
            vec![
                vec![0],
                vec![0],
                vec![0],
                vec![0],
                vec![],
                vec![0],
                vec![0],
                vec![],
                vec![]
            ],
        );
        Ok(())
    }

    #[test]
    #[expect(
        clippy::indexing_slicing,
        reason = "indices derive from the fixture's two instructions per local"
    )]
    fn dense_bits_cover_sparse_java_slots_and_word_boundaries() -> Result<()> {
        let locals = (0..65).map(|index| index * 1000).collect::<Vec<_>>();
        let mut code = Vec::new();
        for local in &locals {
            code.extend([Aload_w(u16::try_from(*local)?), Pop]);
        }
        code.push(Return);
        let live = snapshot(&code, &[], locals.clone())?;
        for (index, _) in locals.iter().enumerate() {
            assert_eq!(live[index * 2], locals[index..]);
            assert_eq!(live[index * 2 + 1], locals[index + 1..]);
        }
        assert!(live.last().is_some_and(Vec::is_empty));
        Ok(())
    }

    #[test]
    fn every_local_write_invalidates_the_previous_reference_kind() {
        for (instruction, expected) in [
            (Istore(7), [Some(7), None]),
            (Fstore(7), [Some(7), None]),
            (Astore(7), [Some(7), None]),
            (Iinc(7, 1), [Some(7), None]),
            (Istore_w(256), [Some(256), None]),
            (Fstore_w(256), [Some(256), None]),
            (Astore_w(256), [Some(256), None]),
            (Iinc_w(256, 1), [Some(256), None]),
            (Lstore(7), [Some(7), Some(8)]),
            (Dstore(7), [Some(7), Some(8)]),
            (Lstore_w(256), [Some(256), Some(257)]),
            (Dstore_w(256), [Some(256), Some(257)]),
        ] {
            assert_eq!(local_definitions(&instruction), expected, "{instruction:?}");
        }
        for (index, instructions) in [
            [Istore_0, Fstore_0, Astore_0, Lstore_0, Dstore_0],
            [Istore_1, Fstore_1, Astore_1, Lstore_1, Dstore_1],
            [Istore_2, Fstore_2, Astore_2, Lstore_2, Dstore_2],
            [Istore_3, Fstore_3, Astore_3, Lstore_3, Dstore_3],
        ]
        .iter()
        .enumerate()
        {
            for (kind, instruction) in instructions.iter().enumerate() {
                assert_eq!(
                    local_definitions(instruction),
                    [Some(index), (kind >= 3).then_some(index + 1)]
                );
            }
        }
        assert_eq!(local_definitions(&Aload_0), [None, None]);
    }

    #[test]
    fn analysis_limits_fail_closed() {
        assert!(matches!(
            ReferenceLiveness::new(&vec![Return; 16_385], &[], (0..4096).collect()),
            Err(UnsupportedMethod(message)) if message.contains("state"),
        ));
        let mut code = vec![Nop; 2000];
        code.push(Return);
        let handlers = vec![
            ExceptionTableEntry {
                range_pc: 0..2000,
                handler_pc: 2000,
                catch_type: 0
            };
            501
        ];
        assert!(matches!(
            ReferenceLiveness::new(&code, &handlers, vec![0]),
            Err(UnsupportedMethod(message)) if message.contains("graph"),
        ));
    }
}
