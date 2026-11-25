//! Root-borrowing graph operations. Copied identities never expose payloads.
use super::{
    EdgeCommand, EdgeKey, GraphError, GraphMutator, GraphReadValue, GraphRoot, LeafRecord,
    PhantomData, PreparedRoot, Progress, RequestError, RootHandle,
};
use crate::canonical_catalog as gc_catalog;
use gc_catalog::bounded::RecordKey;

impl<'gc> GraphMutator<'gc> {
    pub(in crate::gc::publication_owner) fn write<'a, S, T>(
        &'a mut self,
        source: &'a GraphRoot<S>,
        index: usize,
        target: Option<&'a GraphRoot<T>>,
    ) -> Result<GraphWrite<'a, 'gc>, GraphError> {
        self.validate(source, index)?;
        if target.is_some_and(|target| !target.root.belongs_to(&self.catalog)) {
            return Err(GraphError::Foreign);
        }
        Ok(GraphWrite {
            mutator: self,
            source: &source.root,
            target: target.map(|t| &*t.root),
            index,
            edge: None,
            submitted: false,
            finished: false,
        })
    }
    pub(in crate::gc::publication_owner) fn child<'a, S, T: 'static>(
        &'a mut self,
        source: &'a GraphRoot<S>,
        index: usize,
    ) -> Result<GraphRead<'a, 'gc, T>, GraphError> {
        self.validate(source, index)?;
        self.prepared.get_or_insert_with(PreparedRoot::new);
        Ok(GraphRead {
            mutator: self,
            source: &source.root,
            index,
            stage: ReadStage::Observe,
            edge: None,
            child: None,
            finished: false,
            value: PhantomData,
        })
    }
}

pub(in crate::gc::publication_owner) struct GraphWrite<'a, 'gc> {
    mutator: &'a mut GraphMutator<'gc>,
    source: &'a RootHandle<LeafRecord>,
    target: Option<&'a RootHandle<LeafRecord>>,
    index: usize,
    edge: Option<EdgeKey>,
    submitted: bool,
    finished: bool,
}
impl GraphWrite<'_, '_> {
    pub(in crate::gc::publication_owner) fn poll(&mut self) -> Result<bool, GraphError> {
        if self.finished {
            return Err(GraphError::Finished);
        }
        let result = self.step();
        if result.is_err() {
            self.finished = true;
            self.mutator.worker.discard_request();
        }
        result
    }
    fn step(&mut self) -> Result<bool, GraphError> {
        if self.submitted {
            return match self.mutator.worker.poll_in(self.mutator.collector) {
                Progress::Pending => Ok(false),
                Progress::Committed => {
                    self.finished = true;
                    Ok(true)
                }
                Progress::Rejected(e) => Err(GraphError::Catalog(e)),
                _ => Err(GraphError::Protocol),
            };
        }
        if let Some(edge) = self.edge {
            self.mutator
                .worker
                .submit_edge(EdgeCommand::Replace {
                    edge,
                    source: self.source.key(),
                    target: self.target.map(RootHandle::key),
                })
                .map_err(GraphError::Catalog)?;
            self.submitted = true;
        } else {
            match self.mutator.observe(self.source, self.index) {
                Ok(observed) => self.edge = Some(observed.key),
                Err(RequestError::Contended) => {}
                Err(e) => return Err(GraphError::Catalog(e)),
            }
        }
        Ok(false)
    }
}
impl Drop for GraphWrite<'_, '_> {
    fn drop(&mut self) {
        if !self.finished {
            self.mutator.worker.discard_request();
        }
    }
}

#[derive(Clone, Copy)]
enum ReadStage {
    Observe,
    Root(RecordKey),
    Creating,
    Activate,
    Activating,
}
pub(in crate::gc::publication_owner) struct GraphRead<'a, 'gc, T> {
    mutator: &'a mut GraphMutator<'gc>,
    source: &'a RootHandle<LeafRecord>,
    index: usize,
    stage: ReadStage,
    edge: Option<EdgeKey>,
    child: Option<RootHandle<LeafRecord>>,
    finished: bool,
    value: PhantomData<T>,
}
impl<T: 'static> GraphRead<'_, '_, T> {
    #[cfg(test)]
    pub(super) fn waiting_for_activation(&self) -> bool {
        matches!(self.stage, ReadStage::Activate)
    }
    pub(in crate::gc::publication_owner) fn poll(
        &mut self,
    ) -> Result<Option<GraphReadValue<T>>, GraphError> {
        if self.finished {
            return Err(GraphError::Finished);
        }
        let result = self.step();
        if result.is_err() {
            self.finished = true;
            self.mutator.worker.discard_request();
            self.child = None;
        }
        result
    }
    fn step(&mut self) -> Result<Option<GraphReadValue<T>>, GraphError> {
        match self.stage {
            ReadStage::Observe => match self.mutator.observe(self.source, self.index) {
                Ok(observed) => {
                    self.edge = Some(observed.key);
                    if let Some(target) = observed.target_key().map_err(GraphError::Catalog)? {
                        // Root creation validates the exact slot and identity.
                        // Activation then rechecks the edge under its controller.
                        self.stage = ReadStage::Root(target);
                    } else {
                        self.finished = true;
                        return Ok(Some(GraphReadValue::Null));
                    }
                }
                Err(RequestError::Contended) => {}
                Err(e) => return Err(GraphError::Catalog(e)),
            },
            ReadStage::Root(target) => {
                match self
                    .mutator
                    .worker
                    .submit_root_creation(self.mutator.next_root, target)
                {
                    Ok(()) => self.stage = ReadStage::Creating,
                    Err(RequestError::Contended | RequestError::RootLimit) => {
                        // No root was reserved. Reobserve the edge so a later
                        // null write can complete this read even while every
                        // independent root permit remains occupied.
                        self.stage = ReadStage::Observe;
                    }
                    Err(e) => return Err(GraphError::Catalog(e)),
                }
            }
            ReadStage::Creating => match self.mutator.worker.poll_in(self.mutator.collector) {
                Progress::Pending => {}
                Progress::RootCreated(_) => {
                    self.child = Some(
                        self.mutator
                            .worker
                            .take_root()
                            .ok_or(GraphError::Protocol)?,
                    );
                    self.advance_root_cursor();
                    self.stage = ReadStage::Activate;
                }
                Progress::Rejected(RequestError::Conflict | RequestError::Stale) => {
                    self.advance_root_cursor();
                    self.stage = ReadStage::Observe;
                }
                Progress::Rejected(e) => return Err(GraphError::Catalog(e)),
                _ => return Err(GraphError::Protocol),
            },
            ReadStage::Activate => {
                self.mutator
                    .worker
                    .submit_edge(EdgeCommand::RootTarget {
                        edge: self.edge.ok_or(GraphError::Protocol)?,
                        source: self.source.key(),
                        child: self.child.as_ref().ok_or(GraphError::Protocol)?.key(),
                    })
                    .map_err(GraphError::Catalog)?;
                self.stage = ReadStage::Activating;
            }
            ReadStage::Activating => match self.mutator.worker.poll_in(self.mutator.collector) {
                Progress::Pending => {}
                Progress::Committed => {
                    let child = self.child.take().ok_or(GraphError::Protocol)?;
                    // Admission, certificate and TypeId are checked before a typed
                    // owning result can escape. No child view preceded RootTarget.
                    LeafRecord::borrow_root::<T>(&child, self.mutator.collector)
                        .map_err(GraphError::Access)?;
                    self.finished = true;
                    return Ok(Some(GraphReadValue::Root(GraphRoot {
                        root: self
                            .mutator
                            .prepared
                            .take()
                            .ok_or(GraphError::Protocol)?
                            .finish(child),
                        value: PhantomData,
                    })));
                }
                Progress::Rejected(RequestError::Conflict | RequestError::Stale) => {
                    self.child = None;
                    self.stage = ReadStage::Observe;
                }
                Progress::Rejected(e) => return Err(GraphError::Catalog(e)),
                _ => return Err(GraphError::Protocol),
            },
        }
        Ok(None)
    }
    fn advance_root_cursor(&mut self) {
        self.mutator.next_root =
            (self.mutator.next_root + 1) % self.mutator.catalog.capacities().root_slots();
    }
}
impl<T> Drop for GraphRead<'_, '_, T> {
    fn drop(&mut self) {
        if !self.finished {
            self.mutator.worker.discard_request();
        }
    }
}
