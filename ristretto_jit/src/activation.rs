//! Independently own generated scalar state while no native frame is active.
use crate::Error::InternalError;
use crate::{Function, JitValue, Result, Value};
use std::num::NonZeroU32;

#[derive(Clone, Copy, Debug)]
pub(crate) struct ContinuationLayout {
    pub words: usize,
    pub instructions: usize,
}

impl ContinuationLayout {
    // Resume token, fuel, and two reserved words for future request/context state.
    pub const HEADER_WORDS: usize = 4;
}

/// A bounded native slice either requests a scheduling boundary or completes.
#[derive(Debug, PartialEq)]
pub enum ExecutionStep {
    Yielded,
    Returned(Option<Value>),
}

/// Owns generated code, arguments and spilled scalar locals/operands.
/// Dropping the activation cancels it without re-entering generated code.
/// This first capability slice admits no references or runtime helper operations.
#[derive(Debug)]
pub struct Activation {
    function: Function,
    arguments: Box<[JitValue]>,
    state: Box<[u64]>,
    started: bool,
    finished: bool,
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
        })
    }

    /// Byte offset of the next instruction after a yield; absent before entry/after return.
    #[must_use]
    pub fn suspended_bci(&self) -> Option<usize> {
        if !self.started || self.finished {
            return None;
        }
        usize::try_from(self.state.first()?.checked_sub(1)?).ok()
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

    /// Resume scalar code. The context is reserved for later owning request capabilities;
    /// it is neither dereferenced nor retained by this capability slice.
    ///
    /// # Errors
    /// Returns the same errors as [`Self::resume`].
    pub fn resume_with_context(
        &mut self,
        backedge_budget: NonZeroU32,
        _context: *const u8,
    ) -> Result<ExecutionStep> {
        if self.finished {
            return Err(InternalError("invocation already completed".to_owned()));
        }
        self.function.check_native_stack()?;
        *self
            .state
            .get_mut(1)
            .ok_or_else(|| InternalError("missing continuation budget".to_owned()))? =
            u64::from(backedge_budget.get());
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
        if self.state.first() == Some(&u64::MAX) {
            self.finished = true;
            Ok(ExecutionStep::Returned(result.try_into()?))
        } else {
            Ok(ExecutionStep::Yielded)
        }
    }
}
