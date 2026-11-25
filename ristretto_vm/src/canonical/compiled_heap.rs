//! Reusable local native contexts on the same program's canonical heap.
use super::{CompiledExecution, CompiledLimits, CompiledProgram, Error, Report, Table, Value};
use crate::canonical::{Invocation, JavaHeap};
use std::cell::RefCell;
use std::rc::Rc;

/// A reusable local native context with independent frames, mutator and helper.
/// Each invocation borrows this context exclusively; different contexts can wait
/// for one another's class initialization on the shared heap.
pub struct CompiledThread<'c, 'p, 'h, 'gc> {
    execution: CompiledExecution<'c, 'p, 'h, 'gc>,
}
impl std::fmt::Debug for CompiledThread<'_, '_, '_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledThread")
            .field("slice_limit", &self.execution.slice_limit)
            .finish_non_exhaustive()
    }
}
impl<'p> CompiledProgram<'p> {
    /// Attach this compilation to a heap for exactly the same admitted program.
    ///
    /// # Errors
    /// Rejects a foreign program, exhausted participant quotas or invalid frame limits.
    pub fn attach<'h, 'gc>(
        &self,
        heap: &'h JavaHeap<'p, 'gc>,
        limits: CompiledLimits,
    ) -> Result<CompiledThread<'_, 'p, 'h, 'gc>, Error> {
        if !std::ptr::eq(self.program, heap.program) {
            return Err(Error::Boundary(
                "compiled program belongs to a different heap program",
            ));
        }
        let execution = heap.execution(limits.execution_limits())?;
        let table = Table::new(
            execution.runtime.configuration().objects,
            limits.frames,
            self.roots_per_frame,
        )?;
        Ok(CompiledThread {
            execution: CompiledExecution {
                compiled: self,
                execution,
                table: Rc::new(RefCell::new(table)),
                slice_limit: limits.native_slices,
            },
        })
    }
}
impl CompiledThread<'_, '_, '_, '_> {
    /// Observe completed primary-cycle bounds between native Java entries.
    /// # Errors
    /// Reports a busy or contended collector observation.
    pub fn primary_cycles(&mut self) -> Result<ristretto_gc::canonical::PrimaryCycles, Error> {
        Ok(self.execution.execution.mutator.primary_cycles()?)
    }
    /// Invoke a static `(I)I` entry, resetting call-local limits and counters.
    /// Static storage persists until the shared heap is closed. Dropping an
    /// invocation releases its frames and poisons only initialization leases it owns.
    ///
    /// # Errors
    /// Reports entry resolution, Java faults, ownership and work-limit failures.
    pub async fn invoke(
        &mut self,
        class: &str,
        method: &str,
        argument: i32,
    ) -> Result<Invocation, Error> {
        let (class, method) = self
            .execution
            .compiled
            .program
            .entry_method(class, method)?;
        self.execution.execution.report = Report::default();
        let result = self
            .execution
            .call(class, method, vec![Value::Int(argument)], 0)
            .await?;
        Invocation::completed(result, &self.execution.execution.report)
    }
}
