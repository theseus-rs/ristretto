//! Multiple entries and independent local Java tasks on one canonical catalog.
use super::objects::{Execution, Value};
use super::statics::States;
use super::{Error, Limits, Program, Report};
use ristretto_classfile::MethodAccessFlags;
use ristretto_classloader::Method;
use ristretto_gc::GarbageCollector;
use ristretto_gc::canonical::{Budget, Configuration, PrimaryCycles, Runtime, Work};
use std::sync::Arc;

/// Counts for one entry. Helpers can reclaim objects allocated by another entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Invocation {
    pub result: i32,
    pub allocations: usize,
    pub helper_polls: usize,
    pub released_objects: usize,
    pub internal_allocations: usize,
    pub static_storage_allocations: usize,
    pub initialized_classes: usize,
    pub class_initializer_calls: usize,
    pub class_initialization_waits: usize,
    pub entry_initialization: super::InitializationWork,
}

impl Invocation {
    pub(super) fn completed(result: Option<Value>, report: &Report) -> Result<Self, Error> {
        Ok(Self {
            result: result
                .ok_or_else(|| Error::Invalid("missing entry result".into()))?
                .int()?,
            allocations: report.allocations,
            helper_polls: report.helper_polls,
            released_objects: report.released_objects,
            internal_allocations: report.internal_allocations,
            static_storage_allocations: report.static_storage_allocations,
            initialized_classes: report.initialized_classes,
            class_initializer_calls: report.class_initializer_calls,
            class_initialization_waits: report.class_initialization_waits,
            entry_initialization: report.entry_initialization,
        })
    }
}

/// Successful teardown proves zero managed payload and refunded catalog credit.
#[derive(Debug)]
pub struct HeapCompletion<T> {
    pub value: T,
    pub cleanup_polls: usize,
    pub released_during_cleanup: usize,
}

/// One admitted program and catalog shared by independently scheduled Java tasks.
#[derive(Debug)]
pub struct JavaHeap<'p, 'gc> {
    pub(super) program: &'p Program,
    // Static roots drop before the catalog during outer-future cancellation.
    statics: States,
    runtime: Runtime<'gc>,
    collector: &'gc GarbageCollector,
}

/// A reusable interpreter context with its own mutator and reserved helper.
pub struct JavaThread<'p, 'h, 'gc> {
    execution: Execution<'p, 'h, 'gc>,
}

impl std::fmt::Debug for JavaThread<'_, '_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JavaThread")
            .field("limits", &self.execution.limits)
            .finish_non_exhaustive()
    }
}

impl Program {
    pub(super) fn entry_method(
        &self,
        class: &str,
        method: &str,
    ) -> Result<(usize, Arc<Method>), Error> {
        let class = self.class_index(class)?;
        let method = self.layout(class)?.class.try_get_method(method, "(I)I")?;
        if !method.access_flags().contains(MethodAccessFlags::STATIC) {
            return Err(Error::Unsupported("entry must be static (I)I".into()));
        }
        Ok((class, method))
    }

    /// Run entries on one private collector and catalog, then reclaim their heap.
    /// Each attached thread reserves its own mutator and helper. Invocation roots
    /// drop on return or cancellation; unrelated threads can continue collecting.
    ///
    /// # Errors
    /// Reports configuration, workload, cleanup limit or retained-credit failures.
    pub async fn with_heap<'p, T>(
        &'p self,
        configuration: Configuration,
        max_payload_bytes: usize,
        cleanup_limit: usize,
        run: impl AsyncFnOnce(&JavaHeap<'p, '_>) -> Result<T, Error>,
    ) -> Result<HeapCompletion<T>, Error> {
        if self.classes.iter().any(|layout| {
            layout.references > configuration.edges_per_object
                || layout.statics.references > configuration.edges_per_object
        }) {
            return Err(Error::Unsupported("insufficient edge positions".into()));
        }
        self.static_capacity(configuration)?;
        let budget = Budget::new(configuration.charged_bytes()?);
        let collector = GarbageCollector::with_config(ristretto_gc::Configuration {
            max_heap_bytes: max_payload_bytes,
            ..Default::default()
        });
        let mut heap = JavaHeap {
            program: self,
            statics: States::heap(self)?,
            runtime: Runtime::new(&collector, configuration, &budget)?,
            collector: &collector,
        };
        let result = run(&heap).await;
        if heap.runtime.registered() != (0, 0) {
            return Err(Error::Invalid(
                "heap still has registered participants".into(),
            ));
        }
        heap.statics.close();
        let (cleanup_polls, released_during_cleanup) = heap.drain(cleanup_limit).await?;
        drop(heap);
        if budget.used() != 0 {
            return Err(Error::Invalid(
                "heap retained catalog credit after drop".into(),
            ));
        }
        Ok(HeapCompletion {
            value: result?,
            cleanup_polls,
            released_during_cleanup,
        })
    }
}

impl<'p, 'gc> JavaHeap<'p, 'gc> {
    /// Claim a mutator and helper until the returned context is dropped.
    ///
    /// # Errors
    /// Fails if either registration quota is exhausted.
    pub fn attach(&self, limits: Limits) -> Result<JavaThread<'p, '_, 'gc>, Error> {
        Ok(JavaThread {
            execution: self.execution(limits)?,
        })
    }

    /// Logical managed payload bytes across all entries on this heap.
    #[must_use]
    pub fn managed_bytes(&self) -> usize {
        self.collector.managed_bytes()
    }

    pub(super) fn execution(&self, limits: Limits) -> Result<Execution<'p, '_, 'gc>, Error> {
        Ok(Execution {
            program: self.program,
            runtime: &self.runtime,
            mutator: self.runtime.mutator()?,
            helper: self.runtime.helper()?,
            limits,
            report: Report::default(),
            statics: self.statics.attach()?,
        })
    }

    async fn drain(&self, limit: usize) -> Result<(usize, usize), Error> {
        let mut helper = self.runtime.helper()?;
        let mut released = 0;
        let mut polls = 0;
        while self.managed_bytes() != 0 {
            if polls == limit {
                return Err(Error::Limit("heap cleanup poll"));
            }
            if helper.poll()? == Work::ReleasedObjectSlot {
                released += 1;
            }
            polls += 1;
            if polls.is_multiple_of(64) {
                tokio::task::yield_now().await;
            }
        }
        Ok((polls, released))
    }
}

impl JavaThread<'_, '_, '_> {
    /// Observe completed primary-cycle bounds between Java entries.
    ///
    /// # Errors
    /// Returns a collector error if observation is busy or contended.
    pub fn primary_cycles(&mut self) -> Result<PrimaryCycles, Error> {
        Ok(self.execution.mutator.primary_cycles()?)
    }

    /// Invoke a static `(I)I` entry. Limits and counts restart on every call,
    /// including after a previous call failed or its future was cancelled.
    ///
    /// # Errors
    /// Reports entry resolution, execution, quota or collector failures.
    pub async fn invoke(
        &mut self,
        class: &str,
        method: &str,
        argument: i32,
    ) -> Result<Invocation, Error> {
        let (class, method) = self.execution.program.entry_method(class, method)?;
        self.execution.report = Report::default();
        let result = self.execution.entry(class, method, argument).await?;
        Invocation::completed(result, &self.execution.report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classfile::ClassFile;
    use std::future::Future;
    use std::task::{Context, Waker};

    #[test]
    #[expect(clippy::panic_in_result_fn, reason = "lifetime assertions")]
    fn cancelling_outer_future_refunds_payload_and_catalog() -> Result<(), Error> {
        let program = Program::from_classfiles([
            ClassFile::from_bytes(include_bytes!(
                "../tests/fixtures/canonical/CanonicalGraphWorkload.class"
            ))?,
            ClassFile::from_bytes(include_bytes!(
                "../tests/fixtures/canonical/CanonicalGraphWorkload$Node.class"
            ))?,
        ])?;
        let config = Configuration {
            objects: 8,
            roots: 24,
            mutators: 1,
            helpers: 1,
            edges_per_object: 2,
        };
        for prefix in [1, 16, 100] {
            let budget = Budget::new(config.charged_bytes()?);
            let collector = GarbageCollector::new();
            let mut future = Box::pin(async {
                let heap = JavaHeap {
                    program: &program,
                    statics: States::heap(&program)?,
                    runtime: Runtime::new(&collector, config, &budget)?,
                    collector: &collector,
                };
                let mut thread = heap.attach(Limits::default())?;
                thread.invoke("CanonicalGraphWorkload", "run", 50).await?;
                std::future::pending::<()>().await;
                Ok::<(), Error>(())
            });
            for _ in 0..prefix {
                assert!(
                    future
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            assert!(collector.managed_bytes() > 0);
            assert!(budget.used() > 0);
            drop(future);
            assert_eq!(collector.managed_bytes(), 0);
            assert_eq!(budget.used(), 0);
        }
        Ok(())
    }
}
