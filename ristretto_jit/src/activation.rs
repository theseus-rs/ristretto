//! Independently own generated scalar state while no native frame is active.
use crate::Error::InternalError;
use crate::{Function, JitValue, Result, Value};
use ristretto_classfile::attributes::{ArrayType, Instruction};
use ristretto_classfile::byte_reader::ByteReader;
use std::num::NonZeroU32;
use zerocopy::TryFromBytes;

#[derive(Clone, Copy, Debug)]
pub(crate) struct ContinuationLayout {
    pub words: usize,
    pub instructions: usize,
    pub requires_context: bool,
    pub call_offset: Option<usize>,
    pub call_capacity: usize,
}

impl ContinuationLayout {
    // Word indices shared by generated code and the owning activation.
    pub const RESUME_TOKEN: usize = 0;
    pub const FUEL: usize = 1;
    pub const CONTEXT: usize = 2;
    // The fourth header word is reserved.
    pub const HEADER_WORDS: usize = 4;
    pub const INITIAL_TOKEN: u64 = 0;
    pub const BACKEDGE_TOKEN_BASE: u64 = 1;
    pub const CALL_TOKEN_BASE: u64 = 1 << u32::BITS;
    pub const RETURNED_TOKEN: u64 = u64::MAX;

    pub const CALL_KIND: usize = 0;
    pub const CALL_BCI: usize = 1;
    pub const CALL_INDEX: usize = 2;
    pub const CALL_COUNT: usize = 3;
    pub const CALL_RESULT: usize = 4;
    pub const CALL_HEADER_WORDS: usize = Self::CALL_RESULT + 1;
}

/// Tag stored in the first word of a pending runtime request.
#[derive(Clone, Copy, TryFromBytes)]
#[repr(u64)]
pub(crate) enum RequestKind {
    InvokeStatic = 0,
    InvokeSpecial = 1,
    InvokeVirtual = 2,
    InvokeInterface = 3,
    NewObject = 6,
    NewPrimitiveArray = 7,
    NewReferenceArray = 8,
    GetStatic = 10,
    PutStatic = 11,
    GetField = 12,
    PutField = 13,
    CheckCast = 14,
    ReferenceArrayStore = 16,
    ArrayAccess = 17,
}

/// Invocation form resolved by the embedding after the native caller returns.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallKind {
    Static,
    Special,
    Virtual,
    Interface,
}

/// Constant-pool member reference and exact bytecode offset of the parked call.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CallRequest {
    pub kind: CallKind,
    pub bci: usize,
    pub index: u16,
}

/// Allocation resolved by the embedding with no native frame on the stack.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AllocationKind {
    Object(u16),
    PrimitiveArray(ArrayType),
    ReferenceArray(u16),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllocationRequest {
    pub kind: AllocationKind,
    pub bci: usize,
}
/// Field operation resolved by the host. Static requests require declaring-class
/// initialization before completion; reference cells remain owning-context tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FieldAccess {
    GetStatic,
    PutStatic,
    GetField,
    PutField,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldRequest {
    pub kind: FieldAccess,
    pub index: u16,
    pub bci: usize,
}

/// Array operation completed after returning the generated frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArrayAccessKind {
    IntLoad,
    LongLoad,
    FloatLoad,
    DoubleLoad,
    ReferenceLoad,
    ByteLoad,
    CharLoad,
    ShortLoad,
    IntStore,
    LongStore,
    FloatStore,
    DoubleStore,
    ByteStore,
    CharStore,
    ShortStore,
    Length,
}

impl ArrayAccessKind {
    pub(crate) fn from_instruction(instruction: &Instruction) -> Option<Self> {
        Some(match instruction {
            Instruction::Aaload => Self::ReferenceLoad,
            Instruction::Iaload => Self::IntLoad,
            Instruction::Laload => Self::LongLoad,
            Instruction::Faload => Self::FloatLoad,
            Instruction::Daload => Self::DoubleLoad,
            Instruction::Baload => Self::ByteLoad,
            Instruction::Caload => Self::CharLoad,
            Instruction::Saload => Self::ShortLoad,
            Instruction::Iastore => Self::IntStore,
            Instruction::Lastore => Self::LongStore,
            Instruction::Fastore => Self::FloatStore,
            Instruction::Dastore => Self::DoubleStore,
            Instruction::Bastore => Self::ByteStore,
            Instruction::Castore => Self::CharStore,
            Instruction::Sastore => Self::ShortStore,
            Instruction::Arraylength => Self::Length,
            _ => return None,
        })
    }

    fn from_code(code: u16) -> Result<Self> {
        u8::try_from(code)
            .ok()
            .and_then(|byte| Instruction::from_bytes(&mut ByteReader::new(&[byte])).ok())
            .and_then(|instruction| Self::from_instruction(&instruction))
            .ok_or_else(|| InternalError("invalid array access kind".to_owned()))
    }

    /// Number of logical eight-byte cells: array, optional index and optional value.
    #[must_use]
    pub const fn argument_count(self) -> usize {
        if self.is_store() {
            3
        } else if matches!(self, Self::Length) {
            1
        } else {
            2
        }
    }

    pub(crate) const fn is_store(self) -> bool {
        matches!(
            self,
            Self::IntStore
                | Self::LongStore
                | Self::FloatStore
                | Self::DoubleStore
                | Self::ByteStore
                | Self::CharStore
                | Self::ShortStore
        )
    }
}

/// Array identity, signed index and raw value bits remain in owning argument cells.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArrayAccessRequest {
    pub kind: ArrayAccessKind,
    pub bci: usize,
}

/// Reference-array store; cells are array token, signed index and value token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArrayStoreRequest {
    pub bci: usize,
}

/// Checked reference conversion resolved by the owning embedding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeCheckKind {
    CheckCast,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypeCheckRequest {
    pub kind: TypeCheckKind,
    pub index: u16,
    pub bci: usize,
}

/// A bounded native slice either requests a scheduling boundary or completes.
#[derive(Debug, PartialEq)]
pub enum ExecutionStep {
    Yielded,
    Call(CallRequest),
    Allocation(AllocationRequest),
    Field(FieldRequest),
    TypeCheck(TypeCheckRequest),
    ArrayAccess(ArrayAccessRequest),
    ArrayStore(ArrayStoreRequest),
    Returned(Option<Value>),
}

/// Owns generated code, arguments and spilled scalar locals/operands.
/// Dropping the activation cancels it without re-entering generated code.
/// Reference identities are opaque tokens. The embedding must keep the same owning
/// frame context alive across all suspensions, returns and cancellation.
#[derive(Debug)]
pub struct Activation {
    function: Function,
    arguments: Box<[JitValue]>,
    state: Box<[u64]>,
    started: bool,
    finished: bool,
    awaiting_call: bool,
}

impl Activation {
    pub(super) fn new(function: &Function, arguments: &[Value]) -> Result<Self> {
        let layout = function.continuation.ok_or_else(|| {
            crate::Error::UnsupportedMethod("function has no continuation ABI".to_owned())
        })?;
        if arguments.len() != function.argument_types.len() {
            return Err(crate::Error::InvalidArgumentCount {
                expected: function.argument_types.len(),
                actual: arguments.len(),
            });
        }
        let arguments = arguments
            .iter()
            .zip(function.argument_types.iter())
            .enumerate()
            .map(|(index, (argument, expected))| JitValue::argument(argument, *expected, index))
            .collect::<Result<Vec<_>>>()?
            .into_boxed_slice();
        Ok(Self {
            function: function.clone(),
            arguments,
            state: vec![0; layout.words].into_boxed_slice(),
            started: false,
            finished: false,
            awaiting_call: false,
        })
    }

    /// Byte offset of the next instruction after a yield; absent before entry/after return.
    #[must_use]
    pub fn suspended_bci(&self) -> Option<usize> {
        if !self.started || self.finished {
            return None;
        }
        let token = *self.state.get(ContinuationLayout::RESUME_TOKEN)?;
        if token >= ContinuationLayout::CALL_TOKEN_BASE {
            let (offset, _) = self.call_layout().ok()?;
            return usize::try_from(*self.state.get(offset + ContinuationLayout::CALL_BCI)?).ok();
        }
        usize::try_from(token.checked_sub(ContinuationLayout::BACKEDGE_TOKEN_BASE)?).ok()
    }

    fn call_layout(&self) -> Result<(usize, usize)> {
        let layout = self
            .function
            .continuation
            .ok_or_else(|| InternalError("missing continuation layout".to_owned()))?;
        Ok((
            layout
                .call_offset
                .ok_or_else(|| InternalError("no call storage".to_owned()))?,
            layout.call_capacity,
        ))
    }

    /// Borrow stable argument bits while a runtime request is pending. References are rooted by the context.
    /// # Errors
    /// Rejects calls in the wrong phase or corrupt generated argument counts.
    pub fn call_arguments(&self) -> Result<&[i64]> {
        if !self.awaiting_call {
            return Err(InternalError("no pending call".to_owned()));
        }
        let (offset, capacity) = self.call_layout()?;
        let count = usize::try_from(
            *self
                .state
                .get(offset + ContinuationLayout::CALL_COUNT)
                .ok_or_else(|| InternalError("missing call count".to_owned()))?,
        )?;
        if count > capacity {
            return Err(InternalError("call argument overflow".to_owned()));
        }
        let start = offset + ContinuationLayout::CALL_HEADER_WORDS;
        let cells = self
            .state
            .get(start..start + count)
            .ok_or_else(|| InternalError("missing call arguments".to_owned()))?;
        // SAFETY: u64/i64 have identical alignment and size, and every bit pattern is valid.
        Ok(unsafe { std::slice::from_raw_parts(cells.as_ptr().cast::<i64>(), cells.len()) })
    }

    /// Fuel left after a pending runtime request. `None` means the embedding must
    /// yield before resuming with a fresh budget; otherwise carry this remainder.
    /// Requests and taken loop/handler edges each consume a unit, so a sequence of
    /// immediately-ready helpers cannot reset the budget indefinitely.
    /// # Errors
    /// Rejects use outside the pending-request phase or invalid generated fuel.
    pub fn remaining_budget(&self) -> Result<Option<NonZeroU32>> {
        if !self.awaiting_call {
            return Err(InternalError("no pending runtime request".to_owned()));
        }
        let remaining = self
            .state
            .get(ContinuationLayout::FUEL)
            .ok_or_else(|| InternalError("missing continuation fuel".to_owned()))?;
        Ok(NonZeroU32::new(u32::try_from(*remaining)?))
    }

    /// Complete a pending runtime request once. The context must own any reference result. On a Java or embedding error, drop the activation instead of completing it.
    /// # Errors
    /// Rejects completion outside the pending-call phase.
    pub fn complete_call(&mut self, bits: i64) -> Result<()> {
        if !self.awaiting_call {
            return Err(InternalError("no pending call".to_owned()));
        }
        let (offset, _) = self.call_layout()?;
        *self
            .state
            .get_mut(offset + ContinuationLayout::CALL_RESULT)
            .ok_or_else(|| InternalError("missing call result".to_owned()))? =
            u64::from_ne_bytes(bits.to_ne_bytes());
        self.awaiting_call = false;
        Ok(())
    }

    fn pending_request(&self) -> Result<ExecutionStep> {
        let (offset, _) = self.call_layout()?;
        let word = |cell: usize| {
            self.state
                .get(offset + cell)
                .copied()
                .ok_or_else(|| InternalError("missing call metadata".to_owned()))
        };
        let tag = word(ContinuationLayout::CALL_KIND)?;
        let kind: RequestKind = zerocopy::try_transmute!(tag)
            .map_err(|_| InternalError("invalid invocation kind".to_owned()))?;
        let bci = usize::try_from(word(ContinuationLayout::CALL_BCI)?)?;
        let index = u16::try_from(word(ContinuationLayout::CALL_INDEX)?)?;
        let call = |kind| ExecutionStep::Call(CallRequest { kind, bci, index });
        let field = |kind| ExecutionStep::Field(FieldRequest { kind, index, bci });
        let allocation = |kind| ExecutionStep::Allocation(AllocationRequest { kind, bci });
        Ok(match kind {
            RequestKind::InvokeStatic => call(CallKind::Static),
            RequestKind::InvokeSpecial => call(CallKind::Special),
            RequestKind::InvokeVirtual => call(CallKind::Virtual),
            RequestKind::InvokeInterface => call(CallKind::Interface),
            RequestKind::NewObject => allocation(AllocationKind::Object(index)),
            RequestKind::NewPrimitiveArray => {
                let kind = ArrayType::from_bytes(&mut ByteReader::new(&[u8::try_from(index)?]))?;
                allocation(AllocationKind::PrimitiveArray(kind))
            }
            RequestKind::NewReferenceArray => allocation(AllocationKind::ReferenceArray(index)),
            RequestKind::GetStatic => field(FieldAccess::GetStatic),
            RequestKind::PutStatic => field(FieldAccess::PutStatic),
            RequestKind::GetField => field(FieldAccess::GetField),
            RequestKind::PutField => field(FieldAccess::PutField),
            RequestKind::CheckCast => ExecutionStep::TypeCheck(TypeCheckRequest {
                kind: TypeCheckKind::CheckCast,
                index,
                bci,
            }),
            RequestKind::ReferenceArrayStore => {
                ExecutionStep::ArrayStore(ArrayStoreRequest { bci })
            }
            RequestKind::ArrayAccess => ExecutionStep::ArrayAccess(ArrayAccessRequest {
                kind: ArrayAccessKind::from_code(index)?,
                bci,
            }),
        })
    }

    /// Resume until completion or the specified number of taken backward edges.
    /// This is a work bound, not a wall-clock bound. A rejected pre-entry check
    /// leaves the activation resumable; a completed activation cannot be entered again.
    ///
    /// # Errors
    /// Rejects a completed invocation, insufficient native stack or invalid generated state.
    pub fn resume(&mut self, backedge_budget: NonZeroU32) -> Result<ExecutionStep> {
        self.resume_with_context(backedge_budget, std::ptr::null())
    }

    /// Resume with the owning frame context required by reference-bearing code.
    /// Its address is used only for this native slice and is forgotten before return.
    /// The context and its published roots must outlive the activation and result handoff.
    ///
    /// # Errors
    /// Returns the same errors as [`Self::resume`].
    pub fn resume_with_context(
        &mut self,
        backedge_budget: NonZeroU32,
        context: *const u8,
    ) -> Result<ExecutionStep> {
        if self.awaiting_call {
            return Err(InternalError("runtime call is still pending".to_owned()));
        }
        if self.finished {
            return Err(InternalError("invocation already completed".to_owned()));
        }
        if self.function.cooperation_requires_context() && context.is_null() {
            return Err(InternalError(
                "continuation requires a runtime context".to_owned(),
            ));
        }
        self.function.check_native_stack()?;
        *self
            .state
            .get_mut(ContinuationLayout::FUEL)
            .ok_or_else(|| InternalError("missing continuation budget".to_owned()))? =
            u64::from(backedge_budget.get());
        *self
            .state
            .get_mut(ContinuationLayout::CONTEXT)
            .ok_or_else(|| InternalError("missing context cell".to_owned()))? =
            context as usize as u64;
        self.started = true;
        let mut result = JitValue::new();
        // SAFETY: the owning compiler fixes every private offset into this initialized,
        // aligned allocation, and arguments were validated against its descriptor.
        // No borrowed pointer or native frame survives this call.
        unsafe {
            (self.function.entry_point)(
                self.arguments.as_ptr(),
                self.arguments.len(),
                &raw mut result,
                self.state.as_mut_ptr().cast::<u8>(),
            );
        }
        *self
            .state
            .get_mut(ContinuationLayout::CONTEXT)
            .ok_or_else(|| InternalError("missing context cell".to_owned()))? = 0;
        if self.state.get(ContinuationLayout::RESUME_TOKEN)
            == Some(&ContinuationLayout::RETURNED_TOKEN)
        {
            self.finished = true;
            Ok(ExecutionStep::Returned(result.try_into()?))
        } else if self
            .state
            .get(ContinuationLayout::RESUME_TOKEN)
            .is_some_and(|token| *token >= ContinuationLayout::CALL_TOKEN_BASE)
        {
            let request = self.pending_request()?;
            self.awaiting_call = true;
            Ok(request)
        } else {
            Ok(ExecutionStep::Yielded)
        }
    }
}
