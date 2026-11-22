//! Internal allocation-to-typed-result composition. No public Gc constructor or
//! default policy selects this backend. Host allocation and Trace preparation,
//! attachment and destruction remain outside bounded catalog polling.
use super::root_owner::{PreparedRoot, SharedRoot};
use super::{LeafAccessError, LeafRecord, LeafView};
use crate::canonical_catalog as gc_catalog;
use crate::canonical_payload::{PrepareError, PreparedPayload};
use crate::{GarbageCollector, Trace};
use gc_catalog::bounded::{CatalogHandle, GuardedCommand, Participant, Progress, RequestError};
use std::marker::PhantomData;

type Worker = Participant<LeafRecord, GuardedCommand>;

pub(super) fn prepare_record<T: Trace + Send + Sync + 'static>(
    collector: &GarbageCollector,
    value: T,
) -> Result<LeafRecord, AllocationError> {
    PreparedPayload::new(collector, value)
        .map(LeafRecord::new)
        .map_err(|error| match error {
            PrepareError::Unsupported => AllocationError::Unsupported,
            PrepareError::Heap(error) => AllocationError::Heap(error),
        })
}

#[derive(Debug)]
pub(in crate::canonical_adapter) enum AllocationError {
    Unsupported,
    Heap(crate::Error),
    Catalog(RequestError),
    Cancelled,
    Finished,
    Protocol,
}

pub(in crate::canonical_adapter) struct LeafRoot<T> {
    root: SharedRoot,
    value: PhantomData<T>,
}
impl<T> Clone for LeafRoot<T> {
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone(),
            value: PhantomData,
        }
    }
}
impl<T: 'static> LeafRoot<T> {
    pub(in crate::canonical_adapter) fn belongs_to(
        &self,
        catalog: &CatalogHandle<LeafRecord>,
    ) -> bool {
        self.root.belongs_to(catalog)
    }

    pub(in crate::canonical_adapter) fn borrow<'a>(
        &'a self,
        gc: &'a GarbageCollector,
    ) -> Result<LeafView<'a, T>, LeafAccessError> {
        LeafRecord::borrow_root(&self.root, gc)
    }
}

/// Exclusive participant access prevents interleaving another request or taking
/// an untyped result. The request borrows its matching heap through completion;
/// its returned root keeps only canonical storage, not a strong heap owner.
pub(in crate::canonical_adapter) struct LeafAllocation<'a, T> {
    worker: &'a mut Worker,
    collector: &'a GarbageCollector,
    prepared: Option<PreparedRoot>,
    finished: bool,
    value: PhantomData<T>,
}
impl<'a, T: Trace + Send + Sync + 'static> LeafAllocation<'a, T> {
    pub(in crate::canonical_adapter) fn begin(
        worker: &'a mut Worker,
        collector: &'a GarbageCollector,
        slot: usize,
        root_slot: usize,
        value: T,
    ) -> Result<Self, AllocationError> {
        Self::begin_at(worker, collector, Some((slot, root_slot)), value)
    }

    pub(in crate::canonical_adapter) fn begin_any(
        worker: &'a mut Worker,
        collector: &'a GarbageCollector,
        value: T,
    ) -> Result<Self, AllocationError> {
        Self::begin_at(worker, collector, None, value)
    }

    fn begin_at(
        worker: &'a mut Worker,
        collector: &'a GarbageCollector,
        slots: Option<(usize, usize)>,
        value: T,
    ) -> Result<Self, AllocationError> {
        let record = prepare_record(collector, value)?;
        let prepared = PreparedRoot::new();
        let submitted = match slots {
            Some((slot, root_slot)) => {
                worker.submit_rooted_record(slot, root_slot, record.address(), record)
            }
            None => worker.submit_rooted_record_any(record.address(), record),
        };
        submitted.map_err(|failure| AllocationError::Catalog(failure.error))?;
        Ok(Self {
            worker,
            collector,
            prepared: Some(prepared),
            finished: false,
            value: PhantomData,
        })
    }

    /// One catalog transition. Completion transfers a typed shared owner once;
    /// dropping an unfinished request posts cancellation without polling in Drop.
    pub(in crate::canonical_adapter) fn poll(
        &mut self,
    ) -> Result<Option<LeafRoot<T>>, AllocationError> {
        if self.finished {
            return Err(AllocationError::Finished);
        }
        match self.worker.poll_in(self.collector) {
            Progress::Pending => Ok(None),
            Progress::RootCreated(_) => {
                let root = self.worker.take_root().ok_or(AllocationError::Protocol)?;
                self.finished = true;
                Ok(Some(LeafRoot {
                    root: self
                        .prepared
                        .take()
                        .ok_or(AllocationError::Protocol)?
                        .finish(root),
                    value: PhantomData,
                }))
            }
            Progress::Rejected(error) => {
                self.finished = true;
                Err(AllocationError::Catalog(error))
            }
            Progress::Cancelled => {
                self.finished = true;
                Err(AllocationError::Cancelled)
            }
            _ => Err(AllocationError::Protocol),
        }
    }
}
impl<T> Drop for LeafAllocation<'_, T> {
    fn drop(&mut self) {
        if !self.finished {
            self.worker.discard_allocation();
        }
    }
}
