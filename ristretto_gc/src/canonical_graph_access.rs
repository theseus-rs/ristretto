//! Root-borrowing graph operations. Copied identities never expose payloads.
use super::{
    EdgeCommand, EdgeKey, GraphError, GraphMutator, GraphRoot, LeafRecord, Progress, RequestError,
    RootHandle,
};

impl<'gc> GraphMutator<'gc> {
    pub(in crate::canonical_adapter) fn write<'a, S, T>(
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
}

pub(in crate::canonical_adapter) struct GraphWrite<'a, 'gc> {
    mutator: &'a mut GraphMutator<'gc>,
    source: &'a RootHandle<LeafRecord>,
    target: Option<&'a RootHandle<LeafRecord>>,
    index: usize,
    edge: Option<EdgeKey>,
    submitted: bool,
    finished: bool,
}
impl GraphWrite<'_, '_> {
    pub(in crate::canonical_adapter) fn poll(&mut self) -> Result<bool, GraphError> {
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
