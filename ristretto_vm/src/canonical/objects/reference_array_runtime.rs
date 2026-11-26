//! Incremental reference-array construction/access using only canonical edges.
use super::reference_array::{self, FANOUT, MAX_HEIGHT, span};
use super::{
    Component, Error, Execution, Operation, Payload, ReferenceType, Root, complete_recorded,
};
use ristretto_gc::CanonicalPayload;

struct ConstructionFrame {
    root: Root<Payload>,
    height: u8,
    length: u32,
    edges: usize,
    next: usize,
}

fn node_edges(height: u8, length: u32) -> Result<usize, Error> {
    usize::try_from(length.div_ceil(span(height)?))
        .map_err(|_| Error::Limit("reference array children"))
}

impl Execution<'_, '_, '_> {
    async fn allocate_reference_payload(
        &mut self,
        payload: Payload,
        edges: usize,
        internal: bool,
    ) -> Result<Root<Payload>, Error> {
        let recording = self.recording.clone();
        let event = recording.request(Operation::Allocation);
        let parent = event.id();
        let result = async {
            let mut request = recording.prepare(Operation::Allocation, parent, || {
                self.mutator.allocate_with_edges(payload, edges)
            })?;
            complete_recorded(
                &mut self.helper,
                &mut self.report,
                self.limits.request_polls,
                &recording,
                Operation::Allocation,
                parent,
                || request.poll(),
            )
            .await
        }
        .await;
        event.finish_result(&result);
        let root = result?;
        self.report.allocations = self
            .report
            .allocations
            .checked_add(1)
            .ok_or(Error::Limit("allocation counter"))?;
        if internal {
            self.report.internal_allocations = self
                .report
                .internal_allocations
                .checked_add(1)
                .ok_or(Error::Limit("internal allocation counter"))?;
        }
        Ok(root)
    }

    pub(in crate::canonical) async fn read_edge(
        &mut self,
        root: &Root<Payload>,
        edge: usize,
    ) -> Result<Option<Root<Payload>>, Error> {
        let recording = self.recording.clone();
        let event = recording.request(Operation::ChildRead);
        let parent = event.id();
        let result = async {
            let mut request = recording.prepare(Operation::ChildRead, parent, || {
                self.mutator.child::<Payload, Payload>(root, edge)
            })?;
            complete_recorded(
                &mut self.helper,
                &mut self.report,
                self.limits.request_polls,
                &recording,
                Operation::ChildRead,
                parent,
                || request.poll(),
            )
            .await
        }
        .await;
        event.finish_result(&result);
        let root = result?;
        self.report.reference_reads = self
            .report
            .reference_reads
            .checked_add(1)
            .ok_or(Error::Limit("reference read counter"))?;
        Ok(root)
    }

    pub(in crate::canonical) async fn write_edge(
        &mut self,
        root: &Root<Payload>,
        edge: usize,
        target: Option<&Root<Payload>>,
    ) -> Result<(), Error> {
        let recording = self.recording.clone();
        let event = recording.request(Operation::EdgeWrite);
        let parent = event.id();
        let result = async {
            let mut request = recording.prepare(Operation::EdgeWrite, parent, || {
                self.mutator.write(root, edge, target)
            })?;
            complete_recorded(
                &mut self.helper,
                &mut self.report,
                self.limits.request_polls,
                &recording,
                Operation::EdgeWrite,
                parent,
                || request.poll(),
            )
            .await
        }
        .await;
        event.finish_result(&result);
        result?;
        self.report.reference_writes = self
            .report
            .reference_writes
            .checked_add(1)
            .ok_or(Error::Limit("reference write counter"))?;
        Ok(())
    }

    pub(super) async fn allocate_reference_array(
        &mut self,
        component: Component,
        length: i32,
    ) -> Result<Root<Payload>, Error> {
        let layout = reference_array::Layout::new(length)?;
        let configuration = self.runtime.configuration();
        // These checks precede publication. The live heap/root/slot admission of
        // every actual request still applies; a nominal capacity is not free space.
        if configuration.edges_per_object != FANOUT {
            return Err(Error::Limit(
                "reference array requires eight edge positions",
            ));
        }
        if layout.nodes > configuration.objects {
            return Err(Error::Limit("reference array physical object slots"));
        }
        if usize::from(layout.height) + 1 > configuration.roots {
            return Err(Error::Limit("reference array temporary roots"));
        }
        let bytes =
            CanonicalPayload::<Payload>::allocation_size(&Payload::ReferencePage { height: 1 })?
                .checked_mul(layout.nodes)
                .ok_or(Error::Limit("reference array payload bytes"))?;
        if bytes > self.limits.array_bytes {
            return Err(Error::Limit("reference array payload bytes"));
        }
        let edges = node_edges(layout.height, layout.length)?;
        let root = self
            .allocate_reference_payload(Payload::ReferenceArray { component, layout }, edges, false)
            .await?;
        let mut frames: [Option<ConstructionFrame>; MAX_HEIGHT] = std::array::from_fn(|_| None);
        frames[0] = Some(ConstructionFrame {
            root: root.clone(),
            height: layout.height,
            length: layout.length,
            edges,
            next: 0,
        });
        let mut depth = 1;
        while depth != 0 {
            let current = frames
                .get_mut(depth - 1)
                .ok_or(Error::Boundary("array construction depth"))?;
            let frame = current
                .as_mut()
                .ok_or(Error::Boundary("array construction stack"))?;
            let width = span(frame.height)?;
            if frame.height == 1 || frame.next == frame.edges {
                *current = None;
                depth -= 1;
                continue;
            }
            if frame.next >= FANOUT || depth >= MAX_HEIGHT {
                return Err(Error::Boundary("array construction shape"));
            }
            let edge = frame.next;
            let offset = u32::try_from(edge)
                .map_err(|_| Error::Boundary("array edge"))?
                .checked_mul(width)
                .ok_or(Error::Boundary("array child offset"))?;
            let length = frame
                .length
                .checked_sub(offset)
                .ok_or(Error::Boundary("array child range"))?
                .min(width);
            let height = frame.height - 1;
            frame.next += 1;
            let parent = frame.root.clone();
            let edges = node_edges(height, length)?;
            let child = self
                .allocate_reference_payload(Payload::ReferencePage { height }, edges, true)
                .await?;
            self.write_edge(&parent, edge, Some(&child)).await?;
            *frames
                .get_mut(depth)
                .ok_or(Error::Boundary("array construction depth"))? = Some(ConstructionFrame {
                root: child,
                height,
                length,
                edges,
                next: 0,
            });
            depth += 1;
        }
        self.report.reference_array_allocations = self
            .report
            .reference_array_allocations
            .checked_add(1)
            .ok_or(Error::Limit("reference array counter"))?;
        Ok(root)
    }

    fn reference_array(
        &self,
        root: &Root<Payload>,
    ) -> Result<(Component, reference_array::Layout), Error> {
        match &*self.runtime.borrow(root)? {
            Payload::ReferenceArray { component, layout } => Ok((*component, *layout)),
            _ => Err(Error::ArrayFault("reference array type mismatch")),
        }
    }

    async fn reference_leaf(
        &mut self,
        root: &Root<Payload>,
        layout: reference_array::Layout,
        mut index: u32,
    ) -> Result<(Root<Payload>, usize), Error> {
        let mut parent = root.clone();
        for height in (2..=layout.height).rev() {
            let width = span(height)?;
            let edge = usize::try_from(index / width).map_err(|_| Error::Boundary("array edge"))?;
            index %= width;
            let child = self
                .read_edge(&parent, edge)
                .await?
                .ok_or(Error::Boundary("missing array page"))?;
            if !matches!(&*self.runtime.borrow(&child)?, Payload::ReferencePage { height: actual } if *actual == height - 1)
            {
                return Err(Error::Boundary("reference array page shape"));
            }
            parent = child;
        }
        Ok((
            parent,
            usize::try_from(index).map_err(|_| Error::Boundary("array leaf index"))?,
        ))
    }

    pub(super) async fn read_reference_array(
        &mut self,
        root: &Root<Payload>,
        index: i32,
    ) -> Result<Option<Root<Payload>>, Error> {
        let (component, layout) = self.reference_array(root)?;
        let index = layout.index(index)?;
        let (leaf, edge) = self.reference_leaf(root, layout, index).await?;
        let value = self.read_edge(&leaf, edge).await?;
        if let Some(value) = &value {
            self.check_reference(value, component.reference_type())?;
        }
        self.report.array_reads = self
            .report
            .array_reads
            .checked_add(1)
            .ok_or(Error::Limit("array read counter"))?;
        Ok(value)
    }

    pub(super) async fn write_reference_array(
        &mut self,
        root: &Root<Payload>,
        index: i32,
        value: Option<&Root<Payload>>,
    ) -> Result<(), Error> {
        let (component, layout) = self.reference_array(root)?;
        let index = layout.index(index)?;
        if let Some(value) = value {
            let actual = self.reference_kind(value)?;
            if !component.reference_type().accepts(actual) {
                return Err(Error::ArrayFault("ArrayStoreException"));
            }
        }
        let (leaf, edge) = self.reference_leaf(root, layout, index).await?;
        self.write_edge(&leaf, edge, value).await?;
        self.report.array_writes = self
            .report
            .array_writes
            .checked_add(1)
            .ok_or(Error::Limit("array write counter"))?;
        Ok(())
    }
}

impl Component {
    fn reference_type(self) -> ReferenceType {
        match self {
            Self::Object => ReferenceType::AnyObject,
            Self::Class(class) => ReferenceType::Object(class),
        }
    }
}

#[cfg(test)]
#[path = "reference_array_runtime_tests.rs"]
mod tests;
