//! Independently own generated scalar state while no native frame is active.
use crate::Error::InternalError;
use crate::{Function, JitValue, Result, Value};
use std::num::NonZeroU32;

#[derive(Clone, Copy, Debug)]
pub(crate) struct ContinuationLayout {
    pub words: usize,
    pub instructions: usize,
    pub requires_context: bool,
    pub call_offset: Option<usize>,
    pub call_capacity: usize,
}

impl ContinuationLayout {
    // Resume token, fuel, and two reserved words for future request/context state.
    pub const HEADER_WORDS: usize = 4;
    pub const CALL_TOKEN_BASE: u64 = 1 << 32;
    pub const CALL_HEADER_WORDS: usize = 5;
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
    PrimitiveArray(ristretto_classfile::attributes::ArrayType),
    ReferenceArray(u16),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllocationRequest {
    pub kind: AllocationKind,
    pub bci: usize,
}
/// Instance field access; static storage is not admitted by this slice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FieldAccess {
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
    pub(crate) fn from_code(code: u16) -> Result<Self> {
        Ok(match code {
            46 => Self::IntLoad,
            47 => Self::LongLoad,
            48 => Self::FloatLoad,
            49 => Self::DoubleLoad,
            50 => Self::ReferenceLoad,
            51 => Self::ByteLoad,
            52 => Self::CharLoad,
            53 => Self::ShortLoad,
            79 => Self::IntStore,
            80 => Self::LongStore,
            81 => Self::FloatStore,
            82 => Self::DoubleStore,
            84 => Self::ByteStore,
            85 => Self::CharStore,
            86 => Self::ShortStore,
            190 => Self::Length,
            _ => return Err(InternalError("invalid array access kind".to_owned())),
        })
    }

    /// Number of logical eight-byte cells: array, optional index and optional value.
    #[must_use]
    pub const fn argument_count(self) -> usize {
        match self {
            Self::Length => 1,
            Self::IntStore
            | Self::LongStore
            | Self::FloatStore
            | Self::DoubleStore
            | Self::ByteStore
            | Self::CharStore
            | Self::ShortStore => 3,
            _ => 2,
        }
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
        let token = *self.state.first()?;
        if token >= ContinuationLayout::CALL_TOKEN_BASE {
            let (offset, _) = self.call_layout().ok()?;
            return usize::try_from(*self.state.get(offset + 1)?).ok();
        }
        usize::try_from(token.checked_sub(1)?).ok()
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
                .get(offset + 3)
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
            .get(1)
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
            .get_mut(offset + 4)
            .ok_or_else(|| InternalError("missing call result".to_owned()))? =
            u64::from_ne_bytes(bits.to_ne_bytes());
        self.awaiting_call = false;
        Ok(())
    }

    fn pending_request(&self) -> Result<ExecutionStep> {
        let (offset, _) = self.call_layout()?;
        let metadata = self
            .state
            .get(offset..offset + 3)
            .ok_or_else(|| InternalError("missing call metadata".to_owned()))?;
        let [kind, bci, index] = metadata else {
            return Err(InternalError("invalid call metadata".to_owned()));
        };
        if *kind == 14 {
            return Ok(ExecutionStep::TypeCheck(TypeCheckRequest {
                kind: TypeCheckKind::CheckCast,
                index: u16::try_from(*index)?,
                bci: usize::try_from(*bci)?,
            }));
        }
        if *kind == 16 {
            return Ok(ExecutionStep::ArrayStore(ArrayStoreRequest {
                bci: usize::try_from(*bci)?,
            }));
        }
        if *kind == 8 {
            return Ok(ExecutionStep::Allocation(AllocationRequest {
                kind: AllocationKind::ReferenceArray(u16::try_from(*index)?),
                bci: usize::try_from(*bci)?,
            }));
        }
        if *kind == 17 {
            return Ok(ExecutionStep::ArrayAccess(ArrayAccessRequest {
                kind: ArrayAccessKind::from_code(u16::try_from(*index)?)?,
                bci: usize::try_from(*bci)?,
            }));
        }
        if *kind == 7 {
            let kind = ristretto_classfile::attributes::ArrayType::from_bytes(
                &mut ristretto_classfile::byte_reader::ByteReader::new(&[u8::try_from(*index)?]),
            )?;
            return Ok(ExecutionStep::Allocation(AllocationRequest {
                kind: AllocationKind::PrimitiveArray(kind),
                bci: usize::try_from(*bci)?,
            }));
        }
        if *kind == 6 {
            return Ok(ExecutionStep::Allocation(AllocationRequest {
                kind: AllocationKind::Object(u16::try_from(*index)?),
                bci: usize::try_from(*bci)?,
            }));
        }
        if matches!(kind, 12 | 13) {
            return Ok(ExecutionStep::Field(FieldRequest {
                kind: if *kind == 12 {
                    FieldAccess::GetField
                } else {
                    FieldAccess::PutField
                },
                index: u16::try_from(*index)?,
                bci: usize::try_from(*bci)?,
            }));
        }
        let kind = match kind {
            0 => CallKind::Static,
            1 => CallKind::Special,
            2 => CallKind::Virtual,
            3 => CallKind::Interface,
            _ => return Err(InternalError("invalid invocation kind".to_owned())),
        };
        Ok(ExecutionStep::Call(CallRequest {
            kind,
            bci: usize::try_from(*bci)?,
            index: u16::try_from(*index)?,
        }))
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
            .get_mut(1)
            .ok_or_else(|| InternalError("missing continuation budget".to_owned()))? =
            u64::from(backedge_budget.get());
        *self
            .state
            .get_mut(2)
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
            .get_mut(2)
            .ok_or_else(|| InternalError("missing context cell".to_owned()))? = 0;
        if self.state.first() == Some(&u64::MAX) {
            self.finished = true;
            Ok(ExecutionStep::Returned(result.try_into()?))
        } else if self
            .state
            .first()
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
