use super::super::{
    Budget, Configuration, GarbageCollector, Limits, Program, Report, Runtime, Value,
};
use super::*;
use ristretto_classfile::ClassFile;
use std::future::Future;
use std::task::{Context, Waker};

fn program() -> Result<Program, Error> {
    Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../../tests/fixtures/canonical/CanonicalGraphWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../../tests/fixtures/canonical/CanonicalGraphWorkload$Node.class"
        ))?,
    ])
}
fn config(objects: usize, roots: usize) -> Configuration {
    Configuration {
        objects,
        roots,
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
        recording: crate::canonical::recording::Recording::default(),
    })
}

#[tokio::test]
async fn catalog_elements_preserve_aliases_cycles_and_release_every_page() -> Result<(), Error> {
    let program = program()?;
    let collector = GarbageCollector::new();
    let configuration = config(16, 16);
    let budget = Budget::new(configuration.charged_bytes()?);
    let runtime = Runtime::new(&collector, configuration, &budget)?;
    assert_eq!(runtime.configuration(), configuration);
    let mut execution = execution(&program, &runtime)?;
    let array = execution
        .allocate_reference_array(Component::Object, 9)
        .await?;
    assert_eq!(execution.report.internal_allocations, 2);
    assert_eq!(execution.report.allocations, 3);
    let bytes =
        CanonicalPayload::<Payload>::allocation_size(&Payload::ReferencePage { height: 1 })?;
    assert_eq!(collector.managed_bytes(), bytes * 3);
    let object = execution.allocate_object(0).await?;
    execution
        .write_reference_array(&array, 8, Some(&object))
        .await?;
    execution
        .write_reference_array(&array, 0, Some(&array))
        .await?;
    let alias = execution
        .read_reference_array(&array, 8)
        .await?
        .ok_or(Error::Boundary("missing element"))?;
    assert!(alias.same_object(&object));
    drop((alias, object));
    for _ in 0..5000 {
        super::super::help(&mut execution.helper, &mut execution.report)?;
        if execution.mutator.primary_cycles()?.lower >= 2 {
            break;
        }
    }
    assert!(execution.mutator.primary_cycles()?.lower >= 2);
    assert_eq!(collector.managed_bytes(), bytes * 4);
    let alias = execution
        .read_reference_array(&array, 8)
        .await?
        .ok_or(Error::Boundary("lost element"))?;
    assert!(matches!(
        &*runtime.borrow(&alias)?,
        Payload::Object { class: 0, .. }
    ));
    drop(alias);
    drop(array);
    execution.cleanup(&collector).await?;
    assert_eq!(collector.managed_bytes(), 0);
    assert_eq!(
        execution.report.released_objects,
        execution.report.allocations
    );
    drop(execution);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    Ok(())
}

#[tokio::test]
async fn exact_payload_charge_and_temporary_root_preflight() -> Result<(), Error> {
    let program = program()?;
    for (length, roots, byte_delta, succeeds) in [
        (9, 2, 0, false),
        (9, 3, -1, false),
        (9, 3, 0, true),
        // Full-size minimal-root admission is covered natively and on WASI;
        // Miri/Tree use the same path with the small complete tree above.
        #[cfg(not(miri))]
        (2048, 5, 0, true),
    ] {
        let collector = GarbageCollector::new();
        let nodes = reference_array::Layout::new(length)?.nodes;
        let configuration = config(nodes, roots);
        let budget = Budget::new(configuration.charged_bytes()?);
        let runtime = Runtime::new(&collector, configuration, &budget)?;
        let mut execution = execution(&program, &runtime)?;
        let required =
            CanonicalPayload::<Payload>::allocation_size(&Payload::ReferencePage { height: 1 })?
                .checked_mul(nodes)
                .ok_or(Error::Limit("test payload size"))?;
        execution.limits.array_bytes = required
            .checked_add_signed(byte_delta)
            .ok_or(Error::Limit("test byte boundary"))?;
        let result = execution
            .allocate_reference_array(Component::Object, length)
            .await;
        if succeeds {
            let root = result?;
            assert_eq!(collector.managed_bytes(), required);
            assert_eq!(execution.report.allocations, nodes);
            assert_eq!(execution.report.internal_allocations, nodes - 1);
            drop(root);
            execution.cleanup(&collector).await?;
        } else {
            assert!(matches!(result, Err(Error::Limit(_))));
            assert_eq!(execution.report.allocations, 0);
        }
        assert_eq!(collector.managed_bytes(), 0);
        drop(execution);
        drop(runtime);
        assert_eq!(budget.used(), 0);
    }
    Ok(())
}

#[cfg_attr(
    miri,
    ignore = "expensive full-size cancellation; use --ignored explicitly"
)]
#[tokio::test]
async fn cancellation_drops_incrementally_published_prefixes() -> Result<(), Error> {
    let program = program()?;
    for polls in [1, 64, 512, 2048] {
        let collector = GarbageCollector::new();
        let configuration = config(512, 16);
        let budget = Budget::new(configuration.charged_bytes()?);
        let runtime = Runtime::new(&collector, configuration, &budget)?;
        let mut execution = execution(&program, &runtime)?;
        let mut future = Box::pin(execution.allocate_reference_array(Component::Object, 2048));
        let mut context = Context::from_waker(Waker::noop());
        for _ in 0..polls {
            assert!(future.as_mut().poll(&mut context).is_pending());
        }
        assert!(collector.managed_bytes() > 0);
        drop(future);
        if polls >= 512 {
            assert!(execution.report.internal_allocations > 0);
        }
        assert_eq!(execution.report.reference_array_allocations, 0);
        execution.cleanup(&collector).await?;
        assert_eq!(collector.managed_bytes(), 0);
        drop(execution);
        drop(runtime);
        assert_eq!(budget.used(), 0);
    }
    Ok(())
}

#[tokio::test]
async fn preflight_and_mid_construction_quota_failures_release_credit() -> Result<(), Error> {
    let program = program()?;
    let collector = GarbageCollector::new();
    let configuration = config(3, 8);
    let budget = Budget::new(configuration.charged_bytes()?);
    let runtime = Runtime::new(&collector, configuration, &budget)?;
    let mut execution = execution(&program, &runtime)?;
    for length in [-1, 2048, i32::MAX] {
        assert!(
            execution
                .allocate_reference_array(Component::Object, length)
                .await
                .is_err()
        );
        assert_eq!(collector.managed_bytes(), 0);
    }
    execution.limits.array_bytes = 0;
    assert!(
        execution
            .allocate_reference_array(Component::Object, 0)
            .await
            .is_err()
    );
    assert_eq!(collector.managed_bytes(), 0);
    execution.limits = Limits {
        request_polls: 2048,
        ..Limits::default()
    };
    let occupied = execution.allocate_object(0).await?;
    // Three nominal tree slots fit the configuration, but one is live already.
    assert!(
        execution
            .allocate_reference_array(Component::Object, 9)
            .await
            .is_err()
    );
    assert!(execution.report.internal_allocations > 0);
    assert_eq!(execution.report.reference_array_allocations, 0);
    drop(occupied);
    execution.cleanup(&collector).await?;
    assert_eq!(collector.managed_bytes(), 0);
    drop(execution);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    Ok(())
}

#[tokio::test]
async fn internal_pages_never_become_java_values_and_failed_stores_preserve_edges()
-> Result<(), Error> {
    let program = program()?;
    let collector = GarbageCollector::new();
    let configuration = config(8, 16);
    let budget = Budget::new(configuration.charged_bytes()?);
    let runtime = Runtime::new(&collector, configuration, &budget)?;
    let mut execution = execution(&program, &runtime)?;
    let array = execution
        .allocate_reference_array(Component::Class(0), 1)
        .await?;
    let object = execution.allocate_object(0).await?;
    let wrong = execution.allocate_object(1).await?;
    let page = execution
        .allocate_reference_payload(Payload::ReferencePage { height: 1 }, 0, true)
        .await?;
    execution
        .write_reference_array(&array, 0, Some(&object))
        .await?;
    assert!(matches!(
        execution
            .write_reference_array(&array, -1, Some(&wrong))
            .await,
        Err(Error::ArrayFault("ArrayIndexOutOfBoundsException"))
    ));
    assert!(matches!(
        execution
            .write_reference_array(&array, 0, Some(&wrong))
            .await,
        Err(Error::ArrayFault("ArrayStoreException"))
    ));
    assert!(matches!(
        execution
            .write_reference_array(&array, 0, Some(&page))
            .await,
        Err(Error::Boundary(_))
    ));
    assert!(
        execution
            .check_reference(&page, ReferenceType::AnyObject)
            .is_err()
    );
    assert!(execution.array_length(&page).is_err());
    let retained = execution
        .read_reference_array(&array, 0)
        .await?
        .ok_or(Error::Boundary("lost prior element"))?;
    assert!(retained.same_object(&object));
    drop(retained);
    // Simulate a corrupt catalog edge: the read boundary must also fail closed.
    execution.write_edge(&array, 0, Some(&page)).await?;
    assert!(matches!(
        execution.read_reference_array(&array, 0).await,
        Err(Error::Boundary(_))
    ));
    assert!(
        execution
            .check_type(
                &Value::Reference(Some(page.clone())),
                &ristretto_classfile::FieldType::Object("java/lang/Object".into())
            )
            .is_err()
    );
    drop((array, object, wrong, page));
    execution.cleanup(&collector).await?;
    assert_eq!(collector.managed_bytes(), 0);
    drop(execution);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    Ok(())
}

#[tokio::test]
async fn finite_reference_tree_pending_registration_and_linked_prefix_cancellation()
-> Result<(), Error> {
    let program = program()?;
    for physical_stage in 1..=3 {
        let collector = GarbageCollector::new();
        let configuration = config(3, 8);
        let budget = Budget::new(configuration.charged_bytes()?);
        let runtime = Runtime::new(&collector, configuration, &budget)?;
        let mut execution = execution(&program, &runtime)?;
        let unit =
            CanonicalPayload::<Payload>::allocation_size(&Payload::ReferencePage { height: 1 })?;
        let mut future = Box::pin(execution.allocate_reference_array(Component::Object, 9));
        let mut context = Context::from_waker(Waker::noop());
        let mut prefix = None;
        for poll in 1..=512 {
            assert!(future.as_mut().poll(&mut context).is_pending());
            if collector.managed_bytes() == unit * physical_stage {
                prefix = Some(poll);
                break;
            }
        }
        let prefix = prefix.ok_or(Error::Limit("finite cancellation stage"))?;
        drop(future);
        assert_eq!(execution.report.reference_array_allocations, 0);
        if physical_stage == 3 {
            // Header and first page returned Ready, and the first catalog link
            // completed before the second page's pending registration installed.
            assert!(execution.report.internal_allocations >= 1);
            assert!(execution.report.reference_writes >= 1);
        }
        println!(
            "finite cancellation: physical_stage={physical_stage} poll_prefix={prefix} completed_allocations={} internal_allocations={} completed_links={}",
            execution.report.allocations,
            execution.report.internal_allocations,
            execution.report.reference_writes
        );
        execution.cleanup(&collector).await?;
        assert_eq!(collector.managed_bytes(), 0);
        let completed = execution.report.allocations;
        let released = execution.report.released_objects;
        assert!(released >= completed);
        assert!(released - completed <= 1);
        drop(execution);
        drop(runtime);
        assert_eq!(budget.used(), 0);
    }
    Ok(())
}
