//! Own generated state while a native frame is absent from the stack.
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
    pub requests_only: bool,
}

impl ContinuationLayout {
    // Word indices shared by generated code and the owning activation.
    pub const RESUME_TOKEN: usize = 0;
    pub const FUEL: usize = 1;
    pub const CONTEXT: usize = 2;
    // Async request dispatch mode, never an owning context pointer.
    pub const ASYNC_MODE: usize = 3;
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
    MonitorEnter = 4,
    MonitorExit = 5,
    NewObject = 6,
    NewPrimitiveArray = 7,
    NewReferenceArray = 8,
    NewMultiArray = 9,
    GetStatic = 10,
    PutStatic = 11,
    GetField = 12,
    PutField = 13,
    CheckCast = 14,
    InstanceOf = 15,
    ReferenceArrayStore = 16,
    ArrayAccess = 17,
    Constant = 18,
    ExceptionMatch = 19,
}

/// Java invocation form, resolved by the embedding after the native caller returns.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallKind {
    Static,
    Special,
    Virtual,
    Interface,
}

/// A pending call. Arguments remain in the activation until completion or cancellation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CallRequest {
    pub kind: CallKind,
    pub bci: usize,
    pub index: u16,
}

/// Monitor operation dispatched with no generated frame active.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MonitorKind {
    Enter,
    Exit,
}

/// A monitor request; its one argument is a context-rooted object identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MonitorRequest {
    pub kind: MonitorKind,
    pub bci: usize,
}

/// Allocation performed after the generated caller returns. Array sizes live in
/// the activation's argument cells; a multidimensional request has one per dimension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AllocationKind {
    Object(u16),
    PrimitiveArray(ArrayType),
    ReferenceArray(u16),
    MultiArray(u16),
}

/// Allocation kind and exact bytecode offset for errors and initialization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllocationRequest {
    pub kind: AllocationKind,
    pub bci: usize,
}

/// Field instruction dispatched after its generated frame returns.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FieldAccess {
    GetStatic,
    PutStatic,
    GetField,
    PutField,
}

/// `GetStatic` has no outgoing cells; `PutStatic` has one value cell. Instance
/// accesses add a receiver before any value. Reference cells are opaque tokens;
/// the embedding must retain their owning roots. Cells are never payload or
/// static-storage pointers.
///
/// Before completing a static request, the embedding must resolve its declaring
/// class, enforce access/final/type rules, and finish that class's initialization.
/// The requesting activation stays parked while initializer activations run;
/// cancellation/error teardown and class initialization state are host-owned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldRequest {
    pub kind: FieldAccess,
    pub index: u16,
    pub bci: usize,
}

/// Reference type check dispatched after its generated frame returns.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeCheckKind {
    CheckCast,
    InstanceOf,
}

/// The outgoing cell contains a context-rooted object identity (or null).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypeCheckRequest {
    pub kind: TypeCheckKind,
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

/// Reference-array store; outgoing cells contain array, signed index and value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArrayStoreRequest {
    pub bci: usize,
}

/// String or class constant resolved after the generated frame returns.
/// The request has no arguments; its result must be owned by the runtime context.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConstantRequest {
    pub index: u16,
    pub bci: usize,
}

/// Typed handler lookup with the throwable owned by the runtime context.
/// The result is 1 for a match, 0 for a mismatch, or -1 to propagate a resolution error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExceptionMatchRequest {
    pub index: u16,
    pub bci: usize,
}

/// Result of one bounded native slice. Yielding preserves the exact next bytecode.
#[derive(Debug, PartialEq)]
pub enum ExecutionStep {
    Yielded,
    Call(CallRequest),
    Monitor(MonitorRequest),
    Allocation(AllocationRequest),
    Field(FieldRequest),
    TypeCheck(TypeCheckRequest),
    Constant(ConstantRequest),
    ExceptionMatch(ExceptionMatchRequest),
    ArrayStore(ArrayStoreRequest),
    ArrayAccess(ArrayAccessRequest),
    Returned(Option<Value>),
}

/// An independent invocation with resumable generated state.
///
/// Owns generated code, arguments and continuation storage. No native frame or
/// borrowed context pointer survives a yield; dropping this value cancels the invocation.
/// References are raw identities here. Embeddings retain owning arguments and,
/// for reference-bearing code, the same logical runtime context across all slices.
/// That context owns the published frame roots until return or cancellation.
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
    /// Suspension BCI: the next bytecode for a loop yield, or the pending call's
    /// bytecode while awaiting its callee. Absent before entry or after completion.
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
        let mut state = vec![0; layout.words].into_boxed_slice();
        *state
            .get_mut(ContinuationLayout::ASYNC_MODE)
            .ok_or_else(|| InternalError("missing continuation mode".to_owned()))? = 1;
        Ok(Self {
            function: function.clone(),
            arguments,
            state,
            started: false,
            finished: false,
            awaiting_call: false,
        })
    }

    pub(super) fn synchronous_calls(&mut self) -> Result<()> {
        if self.function.requests_only() {
            return Err(InternalError(
                "request-only code requires activation dispatch".to_owned(),
            ));
        }
        *self
            .state
            .get_mut(ContinuationLayout::ASYNC_MODE)
            .ok_or_else(|| InternalError("missing continuation mode".to_owned()))? = 0;
        Ok(())
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

    /// Complete a pending runtime request once. The context must own any reference result or exception.
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
            RequestKind::MonitorEnter | RequestKind::MonitorExit => {
                ExecutionStep::Monitor(MonitorRequest {
                    kind: if matches!(kind, RequestKind::MonitorEnter) {
                        MonitorKind::Enter
                    } else {
                        MonitorKind::Exit
                    },
                    bci,
                })
            }
            RequestKind::NewMultiArray => allocation(AllocationKind::MultiArray(index)),
            RequestKind::InstanceOf => ExecutionStep::TypeCheck(TypeCheckRequest {
                kind: TypeCheckKind::InstanceOf,
                index,
                bci,
            }),
            RequestKind::Constant => ExecutionStep::Constant(ConstantRequest { index, bci }),
            RequestKind::ExceptionMatch => {
                ExecutionStep::ExceptionMatch(ExceptionMatchRequest { index, bci })
            }
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

    /// Resume until return, a runtime request, or the selected number of poll edges.
    /// Taken backward branches and exception-handler entries each consume one unit.
    /// No bytecode is replayed. The positive budget guarantees forward progress.
    ///
    /// # Errors
    /// Returns an error when a runtime context is required, a call is still pending, the invocation completed,
    /// native stack is insufficient, or the generated return value is invalid.
    /// Pre-entry rejection leaves the state resumable.
    pub fn resume(&mut self, backedge_budget: NonZeroU32) -> Result<ExecutionStep> {
        self.resume_with_context(backedge_budget, std::ptr::null())
    }

    /// Resume with the embedding's runtime context for this slice.
    ///
    /// The context and its published roots must outlive the entire invocation,
    /// including suspension; its address may change between slices. The pointer
    /// must satisfy the registered runtime helpers' contract, as in [`Function::execute`].
    /// It is forgotten before this method returns. Primitive code needs no context.
    ///
    /// # Errors
    /// Rejects a missing required context, pending call, completed invocation, insufficient
    /// native stack, or invalid return value. Rejection before entry preserves state.
    pub fn resume_with_context(
        &mut self,
        backedge_budget: NonZeroU32,
        context: *const u8,
    ) -> Result<ExecutionStep> {
        if self.finished {
            return Err(InternalError("invocation already completed".to_owned()));
        }
        if self.awaiting_call {
            return Err(InternalError("pending call has not completed".to_owned()));
        }
        // The synchronous embedding API preserves its opaque-context contract:
        // a null context is valid only when its registered helpers accept it.
        // Owning resumable invocations still require an explicit context.
        if self.function.cooperation_requires_context()
            && context.is_null()
            && self.state.get(3) != Some(&0)
        {
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
        #[cfg(feature = "execution-profile")]
        if self.started {
            self.function
                .resumes
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        } else {
            self.function
                .entries
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        self.started = true;
        let mut result = JitValue::new();
        // SAFETY: the compiler emits this private continuation ABI only for
        // resumable bodies. The owning function fixes every offset into this
        // initialized, aligned storage and validates argument types. References
        // and the context obey the embedding contract stated above.
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
            let _ = self.call_arguments()?;
            Ok(request)
        } else {
            Ok(ExecutionStep::Yielded)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[expect(
        clippy::cast_ptr_alignment,
        clippy::indexing_slicing,
        reason = "the fixture activation owns eleven aligned initialized cells"
    )]
    unsafe extern "C" fn call_once(
        _args: *const JitValue,
        _count: usize,
        result: *mut JitValue,
        state: *const u8,
    ) {
        // SAFETY: this stub is used only with the eleven-word layout in the test below.
        unsafe {
            let cells = std::slice::from_raw_parts_mut(state.cast_mut().cast::<u64>(), 11);
            if cells[0] == 0 {
                cells[0] = ContinuationLayout::CALL_TOKEN_BASE + 9;
                cells[4..11].copy_from_slice(&[0, 9, 5, 2, 0, u64::MAX, 42]);
            } else {
                *result = JitValue::from(i64::from_ne_bytes(cells[8].to_ne_bytes()));
                cells[0] = u64::MAX;
            }
        }
    }

    #[test]
    fn pending_call_requires_one_completion_and_preserves_argument_bits() -> Result<()> {
        let mut function = Function::new(call_once, &[]);
        function.continuation = Some(ContinuationLayout {
            words: 11,
            instructions: 1,
            requires_context: false,
            call_offset: Some(4),
            call_capacity: 2,
            requests_only: false,
        });
        let mut activation = function.activation(&[])?;
        assert!(activation.call_arguments().is_err());
        assert!(activation.complete_call(0).is_err());
        assert_eq!(
            activation.resume(NonZeroU32::MIN)?,
            ExecutionStep::Call(CallRequest {
                kind: CallKind::Static,
                bci: 9,
                index: 5,
            })
        );
        assert_eq!(activation.suspended_bci(), Some(9));
        assert_eq!(activation.call_arguments()?, &[-1, 42]);
        assert!(activation.resume(NonZeroU32::MIN).is_err());
        activation.complete_call(i64::MIN)?;
        assert!(activation.complete_call(0).is_err());
        assert!(activation.call_arguments().is_err());
        assert_eq!(
            activation.resume(NonZeroU32::MIN)?,
            ExecutionStep::Returned(Some(Value::I64(i64::MIN)))
        );
        assert_eq!(activation.suspended_bci(), None);
        #[cfg(feature = "execution-profile")]
        {
            assert_eq!(function.execution_count(), 1);
            assert_eq!(function.resume_count(), 1);
        }
        Ok(())
    }

    #[expect(
        clippy::cast_ptr_alignment,
        reason = "the activation owns aligned u64 continuation cells"
    )]
    unsafe extern "C" fn complete(
        _arguments: *const JitValue,
        _count: usize,
        result: *mut JitValue,
        state: *const u8,
    ) {
        // SAFETY: only the owning activation below calls this stub with live cells/result.
        unsafe {
            *state.cast_mut().cast::<u64>() = u64::MAX;
            *result = JitValue::from(42_i32);
        }
    }

    #[test]
    fn activation_validates_arguments_and_owns_a_single_completion() -> Result<()> {
        let mut function = Function::new(complete, &[crate::jit_value::I32]);
        assert!(function.activation(&[Value::I32(1)]).is_err());
        function.continuation = Some(ContinuationLayout {
            words: 4,
            instructions: 1,
            requires_context: false,
            call_offset: None,
            call_capacity: 0,
            requests_only: false,
        });
        assert!(matches!(
            function.activation(&[]),
            Err(crate::Error::InvalidArgumentCount { .. })
        ));
        assert!(matches!(
            function.activation(&[Value::F32(1.0)]),
            Err(crate::Error::InvalidArgumentType { .. })
        ));
        let mut activation = function.activation(&[Value::I32(1)])?;
        drop(function);
        assert_eq!(
            activation.resume(NonZeroU32::MIN)?,
            ExecutionStep::Returned(Some(Value::I32(42)))
        );
        assert!(activation.resume(NonZeroU32::MIN).is_err());
        #[cfg(feature = "execution-profile")]
        {
            assert_eq!(activation.function.execution_count(), 1);
            assert_eq!(activation.function.resume_count(), 0);
        }
        Ok(())
    }

    #[cfg(all(
        not(target_family = "wasm"),
        target_endian = "little",
        not(any(
            target_arch = "mips",
            target_arch = "mips64",
            target_os = "dragonfly",
            target_os = "solaris"
        ))
    ))]
    #[test]
    fn failed_stack_check_leaves_an_activation_resumable() -> Result<()> {
        let mut function = Function::new(complete, &[]);
        function.continuation = Some(ContinuationLayout {
            words: 4,
            instructions: 1,
            requires_context: false,
            call_offset: None,
            call_capacity: 0,
            requests_only: false,
        });
        function.native_stack_bytes = usize::MAX;
        let mut activation = function.activation(&[])?;
        assert!(activation.resume(NonZeroU32::MIN).is_err());
        assert!(!activation.started);
        assert_eq!(&*activation.state, &[0, 0, 0, 1]);
        activation.function.native_stack_bytes = 0;
        assert_eq!(
            activation.resume(NonZeroU32::MIN)?,
            ExecutionStep::Returned(Some(Value::I32(42)))
        );
        Ok(())
    }
}
