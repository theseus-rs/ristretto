use super::*;

type Result<T> = std::result::Result<T, Error>;
use crate::canonical::recording::Recording;
use crate::canonical::{ClassFile, Limits, Report};
use ristretto_gc::canonical::{Budget, Configuration, Runtime};
use ristretto_gc::{CanonicalPayload, GarbageCollector};
use std::future::Future;
use std::task::{Context, Waker};

fn small() -> Result<Program> {
    Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../tests/fixtures/canonical/CanonicalStaticRootsSmall.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../tests/fixtures/canonical/CanonicalStaticRootsSmall$Node.class"
        ))?,
    ])
}
fn protocol() -> Result<Program> {
    let bytes: [&[u8]; 8] = [
        include_bytes!("../tests/fixtures/canonical/CanonicalStaticProtocol.class"),
        include_bytes!("../tests/fixtures/canonical/CanonicalStaticProtocol$Journal.class"),
        include_bytes!("../tests/fixtures/canonical/CanonicalStaticProtocol$A.class"),
        include_bytes!("../tests/fixtures/canonical/CanonicalStaticProtocol$B.class"),
        include_bytes!("../tests/fixtures/canonical/CanonicalStaticProtocol$Lazy.class"),
        include_bytes!("../tests/fixtures/canonical/CanonicalStaticProtocol$Fieldless.class"),
        include_bytes!("../tests/fixtures/canonical/CanonicalStaticProtocol$Bad.class"),
        include_bytes!("../tests/fixtures/canonical/CanonicalStaticProtocol$NestedBad.class"),
    ];
    Program::from_classfiles(
        bytes
            .into_iter()
            .map(ClassFile::from_bytes)
            .collect::<std::result::Result<Vec<_>, _>>()?,
    )
}
fn configuration() -> Configuration {
    Configuration {
        objects: 32,
        roots: 16,
        mutators: 1,
        helpers: 1,
        edges_per_object: 8,
    }
}
fn execution<'p, 'r, 'gc>(
    program: &'p Program,
    runtime: &'r Runtime<'gc>,
) -> Result<Execution<'p, 'r, 'gc>> {
    Ok(Execution {
        program,
        runtime,
        mutator: runtime.mutator()?,
        helper: runtime.helper()?,
        limits: Limits::default(),
        report: Report::default(),
        recording: Recording::default(),
        statics: States::default(),
    })
}

#[tokio::test]
async fn static_roots_survive_frames_and_completed_cycles() -> Result<()> {
    let program = small()?;
    let collector = GarbageCollector::new();
    let config = configuration();
    let budget = Budget::new(config.charged_bytes()?);
    let runtime = Runtime::new(&collector, config, &budget)?;
    let mut execution = execution(&program, &runtime)?;
    let class = program.class_index("CanonicalStaticRootsSmall")?;
    execution.initialize_class(class, 0).await?;
    let fill = program
        .layout(class)?
        .class
        .try_get_method("fill", "(I)V")?;
    assert!(
        execution
            .call(class, fill, vec![Value::Int(7)], 0)
            .await?
            .is_none()
    );
    let previous = execution.mutator.primary_cycles()?.upper;
    let target = previous.checked_add(2).ok_or(Error::Limit("test epoch"))?;
    for _ in 0..50_000 {
        execution.poll_helper()?;
        if execution.mutator.primary_cycles()?.lower >= target {
            break;
        }
    }
    assert!(execution.mutator.primary_cycles()?.lower >= target);
    let audit = program
        .layout(class)?
        .class
        .try_get_method("audit", "()I")?;
    assert_eq!(
        execution
            .call(class, audit, vec![], 0)
            .await?
            .ok_or(Error::Boundary("audit"))?
            .int()?,
        99
    );
    let root = execution.statics.root(class)?;
    assert!(
        execution
            .check_reference(&root, crate::canonical::layout::ReferenceType::AnyObject)
            .is_err()
    );
    let constant = program
        .layout(class)?
        .statics
        .fields
        .get(&("SLOTS".to_owned(), "I".to_owned()))
        .ok_or(Error::Boundary("constant"))?;
    assert_eq!(execution.read_static(class, constant, 0).await?.int()?, 9);
    let initializer = program
        .layout(class)?
        .statics
        .initializer
        .clone()
        .ok_or(Error::Boundary("initializer"))?;
    assert!(matches!(
        execution
            .write_static(class, &initializer, class, constant, Value::Int(10), 0)
            .await,
        Err(Error::Boundary("final static write after initialization"))
    ));
    drop(root);
    assert!(collector.managed_bytes() > 0);
    execution.statics.close();
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
async fn initializer_errors_wrap_once_and_later_uses_do_not_replay() -> Result<()> {
    let program = protocol()?;
    for first in [
        "CanonicalStaticProtocol$Bad",
        "CanonicalStaticProtocol$NestedBad",
    ] {
        let collector = GarbageCollector::new();
        let config = configuration();
        let budget = Budget::new(config.charged_bytes()?);
        let runtime = Runtime::new(&collector, config, &budget)?;
        let mut execution = execution(&program, &runtime)?;
        let class = program.class_index(first)?;
        let error = execution
            .initialize_class(class, 0)
            .await
            .err()
            .ok_or(Error::Boundary("missing initializer error"))?;
        let Error::Initialization { kind, cause } = error else {
            return Err(Error::Boundary("wrong error category"));
        };
        assert_eq!(kind, "ExceptionInInitializerError");
        assert!(matches!(
            cause.as_deref(),
            Some(Error::ArrayFault("NegativeArraySizeException"))
        ));
        let calls = execution.report.class_initializer_calls;
        assert!(matches!(
            execution.initialize_class(class, 0).await,
            Err(Error::Initialization {
                kind: "NoClassDefFoundError",
                cause: None
            })
        ));
        assert_eq!(execution.report.class_initializer_calls, calls);
        if first.ends_with("$Bad") {
            let nested = program.class_index("CanonicalStaticProtocol$NestedBad")?;
            assert!(matches!(
                execution.initialize_class(nested, 0).await,
                Err(Error::Initialization {
                    kind: "NoClassDefFoundError",
                    cause: None
                })
            ));
        }
        execution.statics.close();
        execution.cleanup(&collector).await?;
        assert_eq!(collector.managed_bytes(), 0);
        drop(execution);
        drop(runtime);
        assert_eq!(budget.used(), 0);
    }
    Ok(())
}

#[tokio::test]
async fn cancelled_initializers_poison_state_and_release_registered_roots() -> Result<()> {
    let program = small()?;
    for stage in [0, 1] {
        let collector = GarbageCollector::new();
        let config = configuration();
        let budget = Budget::new(config.charged_bytes()?);
        let runtime = Runtime::new(&collector, config, &budget)?;
        let mut execution = execution(&program, &runtime)?;
        let class = program.class_index("CanonicalStaticRootsSmall")?;
        let bytes =
            CanonicalPayload::<Payload>::allocation_size(&Payload::ReferencePage { height: 1 })?;
        let target = if stage == 0 { 1 } else { 5 * bytes + 36 };
        let mut pending = Box::pin(execution.initialize_class(class, 0));
        let mut context = Context::from_waker(Waker::noop());
        let mut reached = false;
        for _ in 0..20_000 {
            assert!(pending.as_mut().poll(&mut context).is_pending());
            if collector.managed_bytes() >= target {
                reached = true;
                break;
            }
        }
        assert!(reached, "cancellation stage {stage}");
        drop(pending);
        assert_eq!(
            execution.statics.state(class)?.phase.get(),
            Phase::EmbeddingAborted
        );
        if stage == 1 {
            assert!(execution.report.reference_writes >= 1);
        }
        assert!(matches!(
            execution.initialize_class(class, 0).await,
            Err(Error::Boundary(
                "class initialization was aborted by embedding"
            ))
        ));
        execution.statics.close();
        execution.cleanup(&collector).await?;
        assert_eq!(collector.managed_bytes(), 0);
        assert!(execution.report.released_objects >= execution.report.allocations);
        assert!(execution.report.released_objects <= execution.report.allocations + 1);
        drop(execution);
        drop(runtime);
        assert_eq!(budget.used(), 0);
    }
    Ok(())
}

#[tokio::test]
async fn initialization_limits_are_embedding_failures_and_cleanup_is_complete() -> Result<()> {
    let program = small()?;
    for limit in ["request", "frame", "instruction"] {
        let collector = GarbageCollector::new();
        let config = configuration();
        let budget = Budget::new(config.charged_bytes()?);
        let runtime = Runtime::new(&collector, config, &budget)?;
        let mut execution = execution(&program, &runtime)?;
        match limit {
            "request" => execution.limits.request_polls = 0,
            "frame" => execution.limits.frames = 0,
            _ => execution.limits.instructions = 1,
        }
        let class = program.class_index("CanonicalStaticRootsSmall")?;
        assert!(matches!(
            execution.initialize_class(class, 0).await,
            Err(Error::Limit(_))
        ));
        assert_eq!(
            execution.statics.state(class)?.phase.get(),
            Phase::EmbeddingAborted
        );
        execution.statics.close();
        execution.cleanup(&collector).await?;
        assert_eq!(collector.managed_bytes(), 0);
        drop(execution);
        drop(runtime);
        assert_eq!(budget.used(), 0);
    }
    Ok(())
}

#[tokio::test]
async fn initialization_lease_is_registry_bound_and_drop_needs_no_root_borrow() -> Result<()> {
    let program = protocol()?;
    let collector = GarbageCollector::new();
    let config = configuration();
    let budget = Budget::new(config.charged_bytes()?);
    let runtime = Runtime::new(&collector, config, &budget)?;
    let mut execution = execution(&program, &runtime)?;
    let class = program.class_index("CanonicalStaticProtocol$Journal")?;
    let prepared = execution
        .prepare_initialization(class)
        .await?
        .ok_or(Error::Boundary("lease"))?;
    let state = execution.statics.state(class)?;
    let original = std::mem::take(&mut execution.statics);
    let stored = state.root.get().ok_or(Error::Boundary("prepared root"))?;
    assert!(stored.same_object(stored));
    assert!(matches!(
        execution.finish_initialization(prepared, Ok(None)),
        Err(Error::Boundary("foreign initialization capability"))
    ));
    assert_eq!(state.phase.get(), Phase::EmbeddingAborted);
    drop(state);
    drop(original);
    execution.cleanup(&collector).await?;
    assert_eq!(collector.managed_bytes(), 0);
    drop(execution);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    Ok(())
}

#[test]
fn exception_classification_preserves_errors_and_embedding_failures() {
    let (phase, error) = initialization_failure(Error::Initialization {
        kind: "ExceptionInInitializerError",
        cause: Some(Box::new(Error::TypeFault("ClassCastException"))),
    });
    assert_eq!(phase, Phase::JavaFailed);
    assert!(matches!(
        error,
        Error::Initialization {
            kind: "ExceptionInInitializerError",
            ..
        }
    ));
    for error in [
        Error::Limit("host"),
        Error::Boundary("host"),
        Error::Invalid("unknown host fault".to_owned()),
    ] {
        let (phase, _) = initialization_failure(error);
        assert_eq!(phase, Phase::EmbeddingAborted);
    }
}

#[tokio::test]
async fn no_static_or_initializer_uses_no_registry_or_managed_storage() -> Result<()> {
    let program = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../tests/fixtures/canonical/CanonicalGraphWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../tests/fixtures/canonical/CanonicalGraphWorkload$Node.class"
        ))?,
    ])?;
    let collector = GarbageCollector::new();
    let config = configuration();
    let budget = Budget::new(config.charged_bytes()?);
    let runtime = Runtime::new(&collector, config, &budget)?;
    let mut execution = execution(&program, &runtime)?;
    for class in 0..program.classes.len() {
        execution.initialize_class(class, 0).await?;
    }
    assert!(execution.statics.registry.is_none());
    assert_eq!(execution.report.allocations, 0);
    assert_eq!(collector.managed_bytes(), 0);
    drop(execution);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    Ok(())
}

#[test]
fn owner_identity_is_registry_bound_nonwrapping_and_never_a_terminal_tag() -> Result<()> {
    let program = small()?;
    let heap = States::heap(&program)?;
    let first = heap.attach()?;
    let second = heap.attach()?;
    assert!(first.owner >= FIRST_OWNER);
    assert_ne!(first.owner, second.owner);
    let class = program.class_index("CanonicalStaticRootsSmall")?;
    let state = first.state(class)?;
    for invalid in 0..FIRST_OWNER {
        assert!(!state.claim(invalid));
        assert!(!state.complete(invalid, Phase::Initialized));
    }
    assert_eq!(state.phase.get(), Phase::Uninitialized);
    let registry = heap
        .registry
        .as_ref()
        .ok_or(Error::Boundary("test registry"))?;
    registry.next_owner.store(u64::MAX - 1, Ordering::Relaxed);
    assert_eq!(heap.attach()?.owner, u64::MAX - 1);
    for _ in 0..2 {
        assert!(matches!(
            heap.attach(),
            Err(Error::Limit("class initialization owner identity"))
        ));
    }
    assert_eq!(registry.next_owner.load(Ordering::Relaxed), u64::MAX);
    Ok(())
}

#[tokio::test]
async fn a_different_owner_cannot_complete_another_owners_initializer_lease() -> Result<()> {
    let program = protocol()?;
    let collector = GarbageCollector::new();
    let config = configuration();
    let budget = Budget::new(config.charged_bytes()?);
    let runtime = Runtime::new(&collector, config, &budget)?;
    let mut execution = execution(&program, &runtime)?;
    let class = program.class_index("CanonicalStaticProtocol$Journal")?;
    let prepared = execution
        .prepare_initialization(class)
        .await?
        .ok_or(Error::Boundary("lease"))?;
    let replacement = execution.statics.attach()?;
    let original = std::mem::replace(&mut execution.statics, replacement);
    assert!(matches!(
        execution.finish_initialization(prepared, Ok(None)),
        Err(Error::Boundary("foreign initialization capability"))
    ));
    assert_eq!(original.state(class)?.phase.get(), Phase::EmbeddingAborted);
    drop(original);
    execution.statics.close();
    execution.cleanup(&collector).await?;
    drop(execution);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    Ok(())
}
