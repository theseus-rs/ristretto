use crate::attributes::{ExceptionTableEntry, Instruction};
use crate::verifiers::VerifyError;
use crate::verifiers::bytecode::control_flow::{
    CodeInfo, compute_successors, validate_exception_table,
};
use crate::{ConstantPool, Error, Result};
use ahash::AHashMap;
use std::collections::{BTreeMap, VecDeque};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Call {
    target: usize,
    return_pc: usize,
}

// Separate calling contexts retain the correct return-address locals for each
// invocation of a shared legacy subroutine. Heights still agree across contexts.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location {
    pc: usize,
    calls: Vec<Call>,
}

#[derive(Clone, Debug, Default)]
struct StackAnalysisState {
    depth: u16,
    // Only legacy return addresses need value tracking; other slots are opaque.
    stack_addresses: BTreeMap<u16, usize>,
    local_addresses: BTreeMap<u16, usize>,
}

fn error(pc: usize, message: impl Into<String>) -> Error {
    VerifyError::VerificationError {
        context: format!("instruction {pc}"),
        message: message.into(),
    }
    .into()
}

pub(super) fn analyze(
    code: &[Instruction],
    pool: &ConstantPool<'_>,
    handlers: &[ExceptionTableEntry],
) -> Result<u16> {
    // ControlFlow's offsets are logical indices here, so conversions are identity
    // mappings. There is no need to serialize bytecode just to follow branches.
    let length = u16::try_from(code.len())?;
    let info = CodeInfo::new((0..length).collect(), length);
    validate_exception_table(handlers, &info)?;
    if code.is_empty() {
        return Ok(0);
    }

    if code.iter().any(|instruction| {
        matches!(
            instruction,
            Instruction::Jsr(_)
                | Instruction::Jsr_w(_)
                | Instruction::Ret(_)
                | Instruction::Ret_w(_)
        )
    }) {
        analyze_legacy(code, pool, handlers, &info)
    } else {
        analyze_depths(code, pool, handlers, &info)
    }
}

// Without jsr/ret, the incoming height is the entire state of an instruction.
// Record it directly by logical index, and visit each reachable instruction once.
fn analyze_depths(
    code: &[Instruction],
    pool: &ConstantPool<'_>,
    handlers: &[ExceptionTableEntry],
    info: &CodeInfo,
) -> Result<u16> {
    let mut depths = vec![None; code.len()];
    let mut pending = Vec::new();
    merge_depth(0, 0, &mut depths, &mut pending)?;
    let mut maximum = 0;
    while let Some(pc) = pending.pop() {
        let depth = depths
            .get(pc)
            .copied()
            .flatten()
            .ok_or_else(|| error(pc, "missing analysis state"))?;
        let instruction = code
            .get(pc)
            .ok_or_else(|| error(pc, "branch target outside code"))?;
        let offset = u16::try_from(pc)?;
        maximum = maximum.max(depth);
        for handler in handlers {
            if handler.range_pc.contains(&offset) {
                merge_depth(
                    usize::from(handler.handler_pc),
                    1,
                    &mut depths,
                    &mut pending,
                )?;
            }
        }

        let (consumed, produced) = instruction.stack_effect(pool)?;
        let remaining = depth.checked_sub(consumed).ok_or_else(|| {
            error(
                pc,
                format!("stack underflow: need {consumed} slots, have {depth}"),
            )
        })?;
        let next = remaining
            .checked_add(produced)
            .ok_or_else(|| error(pc, "stack depth exceeds u16::MAX"))?;
        maximum = maximum.max(next);
        let (successors, _) = compute_successors(offset, instruction, offset + 1, info)?;
        for successor in successors {
            merge_depth(usize::from(successor), next, &mut depths, &mut pending)?;
        }
    }
    Ok(maximum)
}

fn merge_depth(
    pc: usize,
    depth: u16,
    depths: &mut [Option<u16>],
    pending: &mut Vec<usize>,
) -> Result<()> {
    let previous = depths
        .get_mut(pc)
        .ok_or_else(|| error(pc, "branch target outside code"))?;
    match previous {
        Some(previous) if *previous != depth => Err(error(
            pc,
            format!("incompatible stack heights: {previous} and {depth}"),
        )),
        Some(_) => Ok(()),
        None => {
            *previous = Some(depth);
            pending.push(pc);
            Ok(())
        }
    }
}

fn analyze_legacy(
    code: &[Instruction],
    pool: &ConstantPool<'_>,
    handlers: &[ExceptionTableEntry],
    info: &CodeInfo,
) -> Result<u16> {
    let mut states = AHashMap::new();
    let mut pending = VecDeque::new();
    merge(
        Location {
            pc: 0,
            calls: Vec::new(),
        },
        StackAnalysisState::default(),
        &mut states,
        &mut pending,
    )?;
    let mut maximum = 0;
    let mut depths = vec![None; code.len()];
    while let Some(location) = pending.pop_front() {
        let state = states
            .get(&location)
            .cloned()
            .ok_or_else(|| error(location.pc, "missing analysis state"))?;
        let instruction = code
            .get(location.pc)
            .ok_or_else(|| error(location.pc, "branch target outside code"))?;
        let previous_depth = depths
            .get_mut(location.pc)
            .ok_or_else(|| error(location.pc, "branch target outside code"))?;
        if previous_depth.is_some_and(|depth| depth != state.depth) {
            return Err(error(
                location.pc,
                "incompatible stack heights across subroutine calls",
            ));
        }
        *previous_depth = Some(state.depth);
        maximum = maximum.max(state.depth);
        for handler in handlers {
            if handler.range_pc.contains(&u16::try_from(location.pc)?) {
                merge(
                    Location {
                        pc: usize::from(handler.handler_pc),
                        calls: location.calls.clone(),
                    },
                    StackAnalysisState {
                        depth: 1,
                        stack_addresses: BTreeMap::new(),
                        local_addresses: state.local_addresses.clone(),
                    },
                    &mut states,
                    &mut pending,
                )?;
            }
        }

        let (consumed, produced) = instruction.stack_effect(pool)?;
        let remaining = state.depth.checked_sub(consumed).ok_or_else(|| {
            error(
                location.pc,
                format!(
                    "stack underflow: need {consumed} slots, have {}",
                    state.depth
                ),
            )
        })?;
        let depth = remaining
            .checked_add(produced)
            .ok_or_else(|| error(location.pc, "stack depth exceeds u16::MAX"))?;
        let mut next = state;
        next.apply(instruction, remaining, depth);
        maximum = maximum.max(depth);
        propagate(instruction, location, next, info, &mut states, &mut pending)?;
    }
    Ok(maximum)
}

fn merge(
    location: Location,
    state: StackAnalysisState,
    states: &mut AHashMap<Location, StackAnalysisState>,
    pending: &mut VecDeque<Location>,
) -> Result<()> {
    if let Some(previous) = states.get_mut(&location) {
        if previous.depth != state.depth {
            return Err(error(
                location.pc,
                format!(
                    "incompatible stack heights: {} and {}",
                    previous.depth, state.depth
                ),
            ));
        }
        let before = previous.stack_addresses.len() + previous.local_addresses.len();
        // A return address must be known on every incoming path to be usable by ret.
        previous
            .stack_addresses
            .retain(|index, address| state.stack_addresses.get(index) == Some(address));
        previous
            .local_addresses
            .retain(|index, address| state.local_addresses.get(index) == Some(address));
        if before == previous.stack_addresses.len() + previous.local_addresses.len() {
            return Ok(());
        }
    } else {
        states.insert(location.clone(), state);
    }
    pending.push_back(location);
    Ok(())
}

fn propagate(
    instruction: &Instruction,
    mut location: Location,
    mut next: StackAnalysisState,
    info: &CodeInfo,
    states: &mut AHashMap<Location, StackAnalysisState>,
    pending: &mut VecDeque<Location>,
) -> Result<()> {
    match instruction {
        Instruction::Jsr(_) | Instruction::Jsr_w(_) => {
            let target = match instruction {
                Instruction::Jsr(target) => usize::from(*target),
                Instruction::Jsr_w(target) => usize::try_from(*target)?,
                _ => return Err(error(location.pc, "invalid subroutine call")),
            };
            if target >= info.instruction_count() || location.pc + 1 >= info.instruction_count() {
                return Err(error(
                    location.pc,
                    "invalid subroutine target or return address",
                ));
            }
            if location.calls.iter().any(|call| call.target == target) {
                return Err(error(location.pc, "recursive subroutine call"));
            }
            let return_pc = location.pc + 1;
            next.stack_addresses.insert(next.depth - 1, return_pc);
            location.calls.push(Call { target, return_pc });
            location.pc = target;
            merge(location, next, states, pending)
        }
        Instruction::Ret(_) | Instruction::Ret_w(_) => {
            let local = match instruction {
                Instruction::Ret(local) => u16::from(*local),
                Instruction::Ret_w(local) => *local,
                _ => return Err(error(location.pc, "invalid subroutine return")),
            };
            let return_pc = *next
                .local_addresses
                .get(&local)
                .ok_or_else(|| error(location.pc, "ret local does not contain a return address"))?;
            let call_index = location
                .calls
                .iter()
                .rposition(|call| call.return_pc == return_pc)
                .ok_or_else(|| error(location.pc, "ret does not match an active subroutine"))?;
            // Returning to an outer subroutine also invalidates skipped calls.
            for call in location.calls.drain(call_index..) {
                next.local_addresses
                    .retain(|_, address| *address != call.return_pc);
                next.stack_addresses
                    .retain(|_, address| *address != call.return_pc);
            }
            location.pc = return_pc;
            merge(location, next, states, pending)
        }
        _ => {
            let (successors, _) = compute_successors(
                u16::try_from(location.pc)?,
                instruction,
                u16::try_from(location.pc + 1)?,
                info,
            )?;
            for pc in successors {
                merge(
                    Location {
                        pc: usize::from(pc),
                        calls: location.calls.clone(),
                    },
                    next.clone(),
                    states,
                    pending,
                )?;
            }
            Ok(())
        }
    }
}

impl StackAnalysisState {
    fn apply(&mut self, instruction: &Instruction, remaining: u16, depth: u16) {
        if let Some((index, width)) = stored_local(instruction) {
            let address = self
                .depth
                .checked_sub(1)
                .and_then(|top| self.stack_addresses.get(&top))
                .copied();
            self.local_addresses.remove(&index);
            if width == 2
                && let Some(second) = index.checked_add(1)
            {
                self.local_addresses.remove(&second);
            }
            if matches!(
                instruction,
                Instruction::Astore(_)
                    | Instruction::Astore_w(_)
                    | Instruction::Astore_0
                    | Instruction::Astore_1
                    | Instruction::Astore_2
                    | Instruction::Astore_3
            ) && let Some(address) = address
            {
                self.local_addresses.insert(index, address);
            }
        }
        let order: &[u16] = match instruction {
            Instruction::Dup => &[0, 0],
            Instruction::Dup_x1 => &[1, 0, 1],
            Instruction::Dup_x2 => &[2, 0, 1, 2],
            Instruction::Dup2 => &[0, 1, 0, 1],
            Instruction::Dup2_x1 => &[1, 2, 0, 1, 2],
            Instruction::Dup2_x2 => &[2, 3, 0, 1, 2, 3],
            Instruction::Swap => &[1, 0],
            _ => &[],
        };
        let addresses: Vec<_> = order
            .iter()
            .map(|offset| self.stack_addresses.get(&(remaining + offset)).copied())
            .collect();
        self.stack_addresses
            .retain(|position, _| *position < remaining);
        for (position, address) in (remaining..depth).zip(addresses) {
            if let Some(address) = address {
                self.stack_addresses.insert(position, address);
            }
        }
        self.depth = depth;
    }
}

fn stored_local(instruction: &Instruction) -> Option<(u16, u16)> {
    match instruction {
        Instruction::Istore(i)
        | Instruction::Fstore(i)
        | Instruction::Astore(i)
        | Instruction::Iinc(i, _) => Some((u16::from(*i), 1)),
        Instruction::Lstore(i) | Instruction::Dstore(i) => Some((u16::from(*i), 2)),
        Instruction::Istore_w(i)
        | Instruction::Fstore_w(i)
        | Instruction::Astore_w(i)
        | Instruction::Iinc_w(i, _) => Some((*i, 1)),
        Instruction::Lstore_w(i) | Instruction::Dstore_w(i) => Some((*i, 2)),
        Instruction::Istore_0 | Instruction::Fstore_0 | Instruction::Astore_0 => Some((0, 1)),
        Instruction::Istore_1 | Instruction::Fstore_1 | Instruction::Astore_1 => Some((1, 1)),
        Instruction::Istore_2 | Instruction::Fstore_2 | Instruction::Astore_2 => Some((2, 1)),
        Instruction::Istore_3 | Instruction::Fstore_3 | Instruction::Astore_3 => Some((3, 1)),
        Instruction::Lstore_0 | Instruction::Dstore_0 => Some((0, 2)),
        Instruction::Lstore_1 | Instruction::Dstore_1 => Some((1, 2)),
        Instruction::Lstore_2 | Instruction::Dstore_2 => Some((2, 2)),
        Instruction::Lstore_3 | Instruction::Dstore_3 => Some((3, 2)),
        _ => None,
    }
}
