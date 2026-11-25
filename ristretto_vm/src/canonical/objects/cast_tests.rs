use super::super::{
    Budget, Configuration, GarbageCollector, Kind, Limits, Payload, Report, Runtime, complete,
};
use super::*;
use ristretto_classfile::ClassFile;

fn program() -> Result<Program, Error> {
    Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../../tests/fixtures/canonical/CanonicalCastWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../../tests/fixtures/canonical/CanonicalCastWorkload$Node.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../../tests/fixtures/canonical/CanonicalCastWorkload$Other.class"
        ))?,
    ])
}
fn configuration() -> Configuration {
    Configuration {
        objects: 8,
        roots: 16,
        mutators: 1,
        helpers: 1,
        edges_per_object: 8,
    }
}
fn execution<'p, 'r, 'gc>(
    program: &'p Program,
    runtime: &'r Runtime<'gc>,
) -> Result<Execution<'p, 'r, 'gc>, Error> {
    Ok(Execution {
        program,
        runtime,
        mutator: runtime.mutator()?,
        helper: runtime.helper()?,
        limits: Limits::default(),
        report: Report::default(),
        statics: crate::canonical::statics::States::default(),
    })
}

#[test]
fn cast_targets_are_closed_and_linked_per_owner() -> Result<(), Error> {
    let program = program()?;
    let mut targets = Vec::new();
    let other = program.class_index("CanonicalCastWorkload$Other")?;
    for (owner, layout) in program.classes.iter().enumerate() {
        for (index, target) in &layout.casts {
            assert_eq!(program.cast_target(owner, *index)?, *target);
            assert_eq!(program.resolve_cast_target(owner, *index)?, *target);
            assert!(program.cast_target(other, *index).is_err());
            if !targets.contains(target) {
                targets.push(*target);
            }
        }
        assert!(program.cast_target(owner, 0).is_err());
        let name = layout
            .class
            .class_file()
            .methods
            .first()
            .ok_or(Error::Boundary("missing test method"))?
            .name_index;
        assert!(program.resolve_cast_target(owner, name).is_err());
    }
    // Eight primitive arrays, two reference-array components and the Node class.
    assert_eq!(targets.len(), 11);
    assert!(program.cast_target(usize::MAX, 1).is_err());
    Ok(())
}

#[tokio::test]
async fn casts_preserve_identity_components_and_gc_counters() -> Result<(), Error> {
    let program = program()?;
    let node_class = program.class_index("CanonicalCastWorkload$Node")?;
    let other_class = program.class_index("CanonicalCastWorkload$Other")?;
    let collector = GarbageCollector::new();
    let configuration = configuration();
    let budget = Budget::new(configuration.charged_bytes()?);
    let runtime = Runtime::new(&collector, configuration, &budget)?;
    let mut execution = execution(&program, &runtime)?;
    let node = execution.allocate_object(node_class).await?;
    let other = execution.allocate_object(other_class).await?;
    let bytes = execution.allocate_array(Kind::Byte, 3).await?;
    let nodes = execution
        .allocate_reference_array(Component::Class(node_class), 9)
        .await?;
    execution
        .write_reference_array(&nodes, 8, Some(&node))
        .await?;
    let before = execution.report.clone();
    let managed = collector.managed_bytes();
    for (root, target) in [
        (&node, ReferenceType::Object(node_class)),
        (&node, ReferenceType::AnyObject),
        (&bytes, ReferenceType::Array(Kind::Byte)),
        (&bytes, ReferenceType::AnyObject),
        (
            &nodes,
            ReferenceType::ReferenceArray(Component::Class(node_class)),
        ),
        (&nodes, ReferenceType::ReferenceArray(Component::Object)),
        (&nodes, ReferenceType::AnyObject),
    ] {
        let cast = execution
            .cast_reference(Value::Reference(Some(root.clone())), target)?
            .object()?;
        assert!(cast.same_object(root));
    }
    for target in [
        ReferenceType::AnyObject,
        ReferenceType::Object(node_class),
        ReferenceType::Array(Kind::Byte),
        ReferenceType::ReferenceArray(Component::Class(node_class)),
    ] {
        assert!(
            execution
                .cast_reference(Value::Reference(None), target)?
                .reference()?
                .is_none()
        );
    }
    assert!(matches!(
        execution.cast_reference(
            Value::Reference(Some(node.clone())),
            ReferenceType::Object(other_class)
        ),
        Err(Error::TypeFault("ClassCastException"))
    ));
    assert!(matches!(
        execution.cast_reference(
            Value::Reference(Some(bytes.clone())),
            ReferenceType::ReferenceArray(Component::Object)
        ),
        Err(Error::TypeFault("ClassCastException"))
    ));
    assert_eq!(execution.report, before);
    assert_eq!(collector.managed_bytes(), managed);
    assert!(matches!(
        execution
            .write_reference_array(&nodes, 8, Some(&other))
            .await,
        Err(Error::ArrayFault("ArrayStoreException"))
    ));
    let retained = execution
        .read_reference_array(&nodes, 8)
        .await?
        .ok_or(Error::Boundary("lost element"))?;
    assert!(retained.same_object(&node));
    drop((retained, node, other, bytes, nodes));
    execution.cleanup(&collector).await?;
    assert_eq!(collector.managed_bytes(), 0);
    drop(execution);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    Ok(())
}

#[tokio::test]
async fn broad_casts_reject_hidden_pages_and_foreign_roots() -> Result<(), Error> {
    let program = program()?;
    let collector = GarbageCollector::new();
    let foreign_collector = GarbageCollector::new();
    let configuration = configuration();
    let budget = Budget::new(configuration.charged_bytes()?);
    let foreign_budget = Budget::new(configuration.charged_bytes()?);
    let runtime = Runtime::new(&collector, configuration, &budget)?;
    let foreign_runtime = Runtime::new(&foreign_collector, configuration, &foreign_budget)?;
    let mut execution = execution(&program, &runtime)?;
    let mut foreign = self::execution(&program, &foreign_runtime)?;
    let foreign_root = foreign.allocate_object(0).await?;
    assert!(matches!(
        execution.cast_reference(
            Value::Reference(Some(foreign_root.clone())),
            ReferenceType::AnyObject
        ),
        Err(Error::Collector(_))
    ));
    let mut request = execution
        .mutator
        .allocate(Payload::ReferencePage { height: 1 })?;
    let page = complete(&mut execution.helper, &mut execution.report, 10_000, || {
        request.poll()
    })
    .await?;
    drop(request);
    assert!(matches!(
        execution.cast_reference(
            Value::Reference(Some(page.clone())),
            ReferenceType::AnyObject
        ),
        Err(Error::Boundary("internal page as Java reference"))
    ));
    assert!(
        execution
            .cast_reference(Value::Int(0), ReferenceType::AnyObject)
            .is_err()
    );
    drop((page, foreign_root));
    execution.cleanup(&collector).await?;
    foreign.cleanup(&foreign_collector).await?;
    assert_eq!(collector.managed_bytes(), 0);
    assert_eq!(foreign_collector.managed_bytes(), 0);
    drop((execution, foreign));
    drop((runtime, foreign_runtime));
    assert_eq!(budget.used(), 0);
    assert_eq!(foreign_budget.used(), 0);
    Ok(())
}

#[tokio::test]
async fn successful_cast_needs_no_spare_root_permit() -> Result<(), Error> {
    let program = program()?;
    let node_class = program.class_index("CanonicalCastWorkload$Node")?;
    let collector = GarbageCollector::new();
    let configuration = Configuration {
        objects: 1,
        roots: 1,
        ..configuration()
    };
    let budget = Budget::new(configuration.charged_bytes()?);
    let runtime = Runtime::new(&collector, configuration, &budget)?;
    let mut execution = execution(&program, &runtime)?;
    let root = execution.allocate_object(node_class).await?;
    let cast = execution
        .cast_reference(
            Value::Reference(Some(root.clone())),
            ReferenceType::Object(node_class),
        )?
        .object()?;
    assert!(cast.same_object(&root));
    assert_eq!(execution.report.allocations, 1);
    drop((root, cast));
    execution.cleanup(&collector).await?;
    assert_eq!(collector.managed_bytes(), 0);
    drop(execution);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    Ok(())
}
