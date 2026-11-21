//! Heap-owned canonical static roots and logical-owner class initialization.
use super::layout::{Field, reference_type};
use super::objects::{Execution, Payload, Value, complete, help};
use super::{Error, Program};
use ristretto_classfile::attributes::Attribute;
use ristretto_classfile::{BaseType, ConstantPool};
use ristretto_classfile::{FieldAccessFlags, FieldType, JavaString};
use ristretto_classloader::Method;
use ristretto_gc::canonical::{Configuration, Root};
use std::collections::HashMap;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use tokio::sync::Notify;

#[derive(Clone, Debug)]
pub(super) struct StaticField {
    pub(super) field: Field,
    pub(super) final_field: bool,
}

#[derive(Debug, Default)]
pub(super) struct StaticLayout {
    pub(super) fields: HashMap<(String, String), StaticField>,
    pub(super) references: usize,
    integers: usize,
    constants: Vec<(usize, i32)>,
    pub(super) initializer: Option<Arc<Method>>,
}
impl StaticLayout {
    pub(super) fn needed(&self) -> bool {
        !self.fields.is_empty() || self.initializer.is_some()
    }
    pub(super) fn admit(
        &mut self,
        field: &ristretto_classfile::Field,
        pool: &ConstantPool<'_>,
        names: &HashMap<JavaString, usize>,
    ) -> Result<(), Error> {
        if field.access_flags.contains(FieldAccessFlags::VOLATILE) {
            return Err(Error::Unsupported("volatile static fields".to_owned()));
        }
        let slot = match &field.field_type {
            FieldType::Base(BaseType::Int) if self.integers < 8 => {
                let index = self.integers;
                self.integers += 1;
                Field::Int(index)
            }
            ty @ (FieldType::Object(_) | FieldType::Array(_)) if self.references < 8 => {
                let edge = self.references;
                self.references += 1;
                Field::Reference {
                    edge,
                    target: reference_type(ty, names)?,
                }
            }
            _ => {
                return Err(Error::Unsupported(
                    "static field type or eight-field quota".to_owned(),
                ));
            }
        };
        let mut constant = None;
        for attribute in &field.attributes {
            if let Attribute::ConstantValue {
                constant_value_index,
                ..
            } = attribute
            {
                if constant.is_some() {
                    return Err(Error::Unsupported(
                        "duplicate static ConstantValue".to_owned(),
                    ));
                }
                let Field::Int(index) = slot else {
                    return Err(Error::Unsupported(
                        "non-int static ConstantValue".to_owned(),
                    ));
                };
                constant = Some((index, *pool.try_get_integer(*constant_value_index)?));
            }
        }
        if let Some(constant) = constant {
            self.constants.push(constant);
        }
        if self
            .fields
            .insert(
                (
                    pool.try_get_utf8(field.name_index)?.to_string(),
                    field.field_type.descriptor(),
                ),
                StaticField {
                    field: slot,
                    final_field: field.access_flags.contains(FieldAccessFlags::FINAL),
                },
            )
            .is_some()
        {
            return Err(Error::Invalid("duplicate static field".to_owned()));
        }
        Ok(())
    }
}

impl Program {
    pub(super) fn static_capacity(&self, configuration: Configuration) -> Result<(), Error> {
        let roots = self
            .classes
            .iter()
            .filter(|layout| !layout.statics.fields.is_empty())
            .count();
        if roots > configuration.roots || roots > configuration.objects {
            return Err(Error::Limit("static storage root/object capacity"));
        }
        Ok(())
    }
    pub(super) fn needs_initialization(&self) -> bool {
        self.classes.iter().any(|layout| layout.statics.needed())
    }
    pub(super) fn resolve_static_field(
        &self,
        owner: usize,
        index: u16,
    ) -> Result<(usize, StaticField), Error> {
        let (class, name, descriptor) = self.member(owner, index, true)?;
        let class = self.class_index(&class)?;
        let field = self
            .layout(class)?
            .statics
            .fields
            .get(&(name, descriptor))
            .cloned()
            .ok_or_else(|| Error::Unsupported("static field resolution/kind".to_owned()))?;
        Ok((class, field))
    }
    pub(super) fn static_field(
        &self,
        owner: usize,
        index: u16,
    ) -> Result<(usize, StaticField), Error> {
        self.layout(owner)?
            .linked
            .statics
            .get(&index)
            .cloned()
            .ok_or(Error::Boundary("unlinked static field"))
    }
    pub(super) fn admit_static_write(
        caller: usize,
        method: &Method,
        owner: usize,
        field: &StaticField,
    ) -> Result<(), Error> {
        if field.final_field && (caller != owner || method.name() != "<clinit>") {
            return Err(Error::Unsupported(
                "final static write outside declaring initializer".to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    Uninitialized,
    Initializing(u64),
    Initialized,
    JavaFailed,
    EmbeddingAborted,
}
const FIRST_OWNER: u64 = 4;
#[derive(Debug)]
struct PhaseCell(AtomicU64);
impl PhaseCell {
    fn get(&self) -> Phase {
        match self.0.load(Ordering::Acquire) {
            0 => Phase::Uninitialized,
            1 => Phase::Initialized,
            2 => Phase::JavaFailed,
            3 => Phase::EmbeddingAborted,
            owner => Phase::Initializing(owner),
        }
    }
}
#[derive(Debug)]
struct ClassState {
    phase: PhaseCell,
    root: OnceLock<Root<Payload>>,
    changed: Notify,
}
impl ClassState {
    fn new() -> Self {
        Self {
            phase: PhaseCell(AtomicU64::new(0)),
            root: OnceLock::new(),
            changed: Notify::new(),
        }
    }
    fn claim(&self, owner: u64) -> bool {
        if owner < FIRST_OWNER {
            return false;
        }
        self.phase
            .0
            .compare_exchange(0, owner, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }
    fn complete(&self, owner: u64, phase: Phase) -> bool {
        if owner < FIRST_OWNER {
            return false;
        }
        let value = match phase {
            Phase::Initialized => 1,
            Phase::JavaFailed => 2,
            Phase::EmbeddingAborted => 3,
            _ => return false,
        };
        if self
            .phase
            .0
            .compare_exchange(owner, value, Ordering::Release, Ordering::Relaxed)
            .is_err()
        {
            return false;
        }
        self.changed.notify_waiters();
        true
    }
}
#[derive(Debug)]
struct Registry {
    classes: Vec<Option<Arc<ClassState>>>,
    next_owner: AtomicU64,
}
impl Registry {
    fn new(program: &Program) -> Result<Arc<Self>, Error> {
        let mut classes = Vec::new();
        classes
            .try_reserve_exact(program.classes.len())
            .map_err(|_| Error::Limit("class state metadata"))?;
        for layout in &program.classes {
            classes.push(layout.statics.needed().then(|| Arc::new(ClassState::new())));
        }
        Ok(Arc::new(Self {
            classes,
            next_owner: AtomicU64::new(FIRST_OWNER),
        }))
    }
    fn owner(&self) -> Result<u64, Error> {
        // Cold context admission, not a collector poll or a native callback.
        // Registry identity is part of authority; IDs never wrap/recycle in it.
        self.next_owner
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |next| {
                next.checked_add(1)
            })
            .map_err(|_| Error::Limit("class initialization owner identity"))
    }
    fn state(&self, class: usize) -> Result<Arc<ClassState>, Error> {
        self.classes
            .get(class)
            .and_then(Option::as_ref)
            .cloned()
            .ok_or(Error::Boundary("class has no initialization state"))
    }
}
#[derive(Debug, Default)]
pub(super) struct States {
    registry: Option<Arc<Registry>>,
    owner: u64,
    closed: bool,
}
impl States {
    pub(super) fn heap(program: &Program) -> Result<Self, Error> {
        Ok(Self {
            registry: if program.needs_initialization() {
                Some(Registry::new(program)?)
            } else {
                None
            },
            ..Self::default()
        })
    }
    pub(super) fn attach(&self) -> Result<Self, Error> {
        if self.closed {
            return Err(Error::Boundary("static registry is closed"));
        }
        Ok(Self {
            registry: self.registry.clone(),
            owner: self
                .registry
                .as_ref()
                .map_or(Ok(0), |registry| registry.owner())?,
            closed: false,
        })
    }
    fn ensure(&mut self, program: &Program) -> Result<(), Error> {
        if self.closed {
            return Err(Error::Boundary("static registry is closed"));
        }
        if self.registry.is_none() {
            *self = Self::heap(program)?.attach()?;
        }
        Ok(())
    }
    fn state(&self, class: usize) -> Result<Arc<ClassState>, Error> {
        self.registry
            .as_ref()
            .ok_or(Error::Boundary("missing static registry"))?
            .state(class)
    }
    fn root(&self, class: usize) -> Result<Root<Payload>, Error> {
        self.state(class)?
            .root
            .get()
            .cloned()
            .ok_or(Error::Boundary("unprepared canonical static storage"))
    }
    fn prepared(&self, class: usize) -> Result<(), Error> {
        match self.state(class)?.phase.get() {
            Phase::Initialized => Ok(()),
            Phase::Initializing(owner) if owner == self.owner => Ok(()),
            _ => Err(Error::Boundary("static helper before owner preparation")),
        }
    }
    pub(super) fn close(&mut self) {
        self.closed = true;
        self.registry = None;
    }
}

struct InitializationGuard {
    registry: Arc<Registry>,
    state: Arc<ClassState>,
    owner: u64,
    finished: bool,
}
impl InitializationGuard {
    fn finish(&mut self, phase: Phase) -> Result<(), Error> {
        if !self.state.complete(self.owner, phase) {
            return Err(Error::Boundary("stale initializer owner"));
        }
        self.finished = true;
        Ok(())
    }
}
impl Drop for InitializationGuard {
    fn drop(&mut self) {
        if !self.finished {
            self.state.complete(self.owner, Phase::EmbeddingAborted);
        }
    }
}

/// A single-use capability tied to a registry and logical Java owner. Drop poisons
/// unfinished initialization and wakes class-local waiters. The native driver
/// executes this exact initializer, then calls `finish_initialization`.
pub(super) struct PreparedInitialization {
    class: usize,
    initializer: Option<Arc<Method>>,
    guard: InitializationGuard,
}
impl PreparedInitialization {
    pub(super) fn class(&self) -> usize {
        self.class
    }
    pub(super) fn initializer(&self) -> Option<Arc<Method>> {
        self.initializer.clone()
    }
}

/// Only admitted Java Exception categories are wrapped. Initialization faults
/// are already Java Error subclasses and must retain their original cause chain.
fn initialization_failure(error: Error) -> (Phase, Error) {
    let exception = matches!(&error, Error::Invalid(name) if name == "NullPointerException");
    if exception {
        (
            Phase::JavaFailed,
            Error::Initialization {
                kind: "ExceptionInInitializerError",
                cause: Some(Box::new(error)),
            },
        )
    } else if matches!(error, Error::Initialization { .. }) {
        (Phase::JavaFailed, error)
    } else {
        (Phase::EmbeddingAborted, error)
    }
}

impl Execution<'_, '_, '_> {
    pub(super) async fn initialize_class(
        &mut self,
        class: usize,
        depth: usize,
    ) -> Result<(), Error> {
        let Some(prepared) = self.prepare_initialization(class).await? else {
            return Ok(());
        };
        let outcome = if let Some(initializer) = prepared.initializer() {
            self.call(prepared.class(), initializer, Vec::new(), depth)
                .await
        } else {
            Ok(None)
        };
        self.finish_initialization(prepared, outcome)
    }

    pub(super) async fn prepare_initialization(
        &mut self,
        class: usize,
    ) -> Result<Option<PreparedInitialization>, Error> {
        let layout = &self.program.layout(class)?.statics;
        // Existing fieldless/no-initializer workloads allocate no registry state.
        if !layout.needed() {
            return Ok(None);
        }
        let initializer = layout.initializer.clone();
        let needs_storage = !layout.fields.is_empty();
        self.statics.ensure(self.program)?;
        let state = self.statics.state(class)?;
        let owner = self.statics.owner;
        if owner < FIRST_OWNER {
            return Err(Error::Boundary("missing class initialization owner"));
        }
        loop {
            // Completed and same-owner gates do not enter the notification queue.
            match state.phase.get() {
                Phase::Initialized => return Ok(None),
                Phase::Initializing(current) if current == owner => return Ok(None),
                Phase::JavaFailed => {
                    return Err(Error::Initialization {
                        kind: "NoClassDefFoundError",
                        cause: None,
                    });
                }
                Phase::EmbeddingAborted => {
                    return Err(Error::Boundary(
                        "class initialization was aborted by embedding",
                    ));
                }
                Phase::Uninitialized => {
                    if state.claim(owner) {
                        break;
                    }
                }
                Phase::Initializing(_) => {
                    // Registration precedes the decisive phase recheck. If the
                    // owner finished before registration, do not await a lost wake.
                    let notified = state.changed.notified();
                    tokio::pin!(notified);
                    notified.as_mut().enable();
                    if matches!(state.phase.get(), Phase::Initializing(current) if current != owner)
                    {
                        self.report.class_initialization_waits = self
                            .report
                            .class_initialization_waits
                            .checked_add(1)
                            .ok_or(Error::Limit("class wait counter"))?;
                        help(&mut self.helper, &mut self.report)?;
                        notified.await;
                    }
                }
            }
        }
        let prepared = PreparedInitialization {
            class,
            initializer,
            guard: InitializationGuard {
                registry: self
                    .statics
                    .registry
                    .as_ref()
                    .ok_or(Error::Boundary("missing static registry"))?
                    .clone(),
                state: state.clone(),
                owner,
                finished: false,
            },
        };
        if needs_storage {
            let root = self.allocate_static_storage(class).await?;
            state
                .root
                .set(root)
                .map_err(|_| Error::Boundary("static storage already prepared"))?;
            for (index, value) in &self.program.layout(class)?.statics.constants {
                let root = self.statics.root(class)?;
                let view = self.runtime.borrow(&root)?;
                static_integer(&view, class, *index)?.store(*value, Ordering::Relaxed);
            }
        }
        Ok(Some(prepared))
    }

    pub(super) fn finish_initialization(
        &mut self,
        mut prepared: PreparedInitialization,
        outcome: Result<Option<Value>, Error>,
    ) -> Result<(), Error> {
        if self.statics.owner != prepared.guard.owner
            || !self
                .statics
                .registry
                .as_ref()
                .is_some_and(|registry| Arc::ptr_eq(registry, &prepared.guard.registry))
            || !self
                .statics
                .state(prepared.class)
                .is_ok_and(|state| Arc::ptr_eq(&state, &prepared.guard.state))
        {
            return Err(Error::Boundary("foreign initialization capability"));
        }
        match outcome {
            Ok(None) => {
                self.report.initialized_classes = self
                    .report
                    .initialized_classes
                    .checked_add(1)
                    .ok_or(Error::Limit("initialized class counter"))?;
                prepared.guard.finish(Phase::Initialized)?;
                Ok(())
            }
            Ok(Some(_)) => Err(Error::Boundary("class initializer returned value")),
            Err(error) => {
                let (phase, error) = initialization_failure(error);
                prepared.guard.finish(phase)?;
                Err(error)
            }
        }
    }

    async fn allocate_static_storage(&mut self, class: usize) -> Result<Root<Payload>, Error> {
        let edges = self.program.layout(class)?.statics.references;
        let mut request = self.mutator.allocate_with_edges(
            Payload::Statics {
                class,
                integers: std::array::from_fn(|_| AtomicI32::new(0)),
            },
            edges,
        )?;
        let root = complete(
            &mut self.helper,
            &mut self.report,
            self.limits.request_polls,
            || request.poll(),
        )
        .await?;
        let allocations = self
            .report
            .allocations
            .checked_add(1)
            .ok_or(Error::Limit("allocation counter"))?;
        let internal = self
            .report
            .internal_allocations
            .checked_add(1)
            .ok_or(Error::Limit("internal allocation counter"))?;
        let statics = self
            .report
            .static_storage_allocations
            .checked_add(1)
            .ok_or(Error::Limit("static storage counter"))?;
        self.report.allocations = allocations;
        self.report.internal_allocations = internal;
        self.report.static_storage_allocations = statics;
        Ok(root)
    }

    pub(super) async fn read_static(
        &mut self,
        owner: usize,
        field: &StaticField,
        depth: usize,
    ) -> Result<Value, Error> {
        self.initialize_class(owner, depth).await?;
        self.read_prepared_static(owner, field).await
    }
    pub(super) async fn read_prepared_static(
        &mut self,
        owner: usize,
        field: &StaticField,
    ) -> Result<Value, Error> {
        self.statics.prepared(owner)?;
        let root = self.statics.root(owner)?;
        match field.field {
            Field::Int(index) => Ok(Value::Int(
                static_integer(&*self.runtime.borrow(&root)?, owner, index)?
                    .load(Ordering::Relaxed),
            )),
            Field::Reference { edge, target } => {
                self.check_static_storage(&root, owner)?;
                let value = {
                    let mut request = self.mutator.child::<Payload, Payload>(&root, edge)?;
                    let child = complete(
                        &mut self.helper,
                        &mut self.report,
                        self.limits.request_polls,
                        || request.poll(),
                    )
                    .await?;
                    self.report.reference_reads += 1;
                    child
                };
                if let Some(value) = &value {
                    self.check_reference(value, target)?;
                }
                Ok(Value::Reference(value))
            }
        }
    }
    pub(super) async fn write_static(
        &mut self,
        caller: usize,
        method: &Method,
        owner: usize,
        field: &StaticField,
        value: Value,
        depth: usize,
    ) -> Result<(), Error> {
        Program::admit_static_write(caller, method, owner, field)?;
        self.initialize_class(owner, depth).await?;
        self.write_prepared_static(caller, method, owner, field, value)
            .await
    }
    pub(super) async fn write_prepared_static(
        &mut self,
        caller: usize,
        method: &Method,
        owner: usize,
        field: &StaticField,
        value: Value,
    ) -> Result<(), Error> {
        Program::admit_static_write(caller, method, owner, field)?;
        self.statics.prepared(owner)?;
        if field.final_field
            && self.statics.state(owner)?.phase.get() != Phase::Initializing(self.statics.owner)
        {
            return Err(Error::Boundary("final static write after initialization"));
        }
        let root = self.statics.root(owner)?;
        match field.field {
            Field::Int(index) => static_integer(&*self.runtime.borrow(&root)?, owner, index)?
                .store(value.int()?, Ordering::Relaxed),
            Field::Reference { edge, target } => {
                self.check_static_storage(&root, owner)?;
                let value = value.reference()?;
                if let Some(value) = &value {
                    self.check_reference(value, target)?;
                }
                let mut request = self.mutator.write(&root, edge, value.as_ref())?;
                complete(
                    &mut self.helper,
                    &mut self.report,
                    self.limits.request_polls,
                    || request.poll(),
                )
                .await?;
                self.report.reference_writes += 1;
            }
        }
        Ok(())
    }
    fn check_static_storage(&self, root: &Root<Payload>, class: usize) -> Result<(), Error> {
        if matches!(&*self.runtime.borrow(root)?, Payload::Statics { class: actual, .. } if *actual == class)
        {
            Ok(())
        } else {
            Err(Error::Boundary("static storage class/kind"))
        }
    }
}
fn static_integer(payload: &Payload, class: usize, index: usize) -> Result<&AtomicI32, Error> {
    if let Payload::Statics {
        class: actual,
        integers,
    } = payload
        && *actual == class
    {
        integers
            .get(index)
            .ok_or(Error::Boundary("static integer index"))
    } else {
        Err(Error::Boundary("static integer class/kind"))
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Limits, Report};
    use super::*;
    use ristretto_classfile::ClassFile;
    use ristretto_gc::{
        GarbageCollector,
        canonical::{Budget, Runtime},
    };
    use std::future::Future;
    use std::task::{Context, Waker};

    fn program() -> Result<Program, Error> {
        Program::from_classfiles([
            ClassFile::from_bytes(include_bytes!(
                "../tests/fixtures/canonical/CanonicalStaticFields.class"
            ))?,
            ClassFile::from_bytes(include_bytes!(
                "../tests/fixtures/canonical/CanonicalStaticFields$Node.class"
            ))?,
        ])
    }
    fn config() -> Configuration {
        Configuration {
            objects: 5,
            roots: 12,
            mutators: 1,
            helpers: 1,
            edges_per_object: 1,
        }
    }

    #[tokio::test]
    #[expect(clippy::panic_in_result_fn, reason = "storage and quota assertions")]
    async fn constants_are_prepared_and_hidden_storage_cannot_be_a_java_object() -> Result<(), Error>
    {
        let program = program()?;
        let budget = Budget::new(config().charged_bytes()?);
        let collector = GarbageCollector::new();
        let runtime = Runtime::new(&collector, config(), &budget)?;
        let mut execution = Execution {
            program: &program,
            runtime: &runtime,
            mutator: runtime.mutator()?,
            helper: runtime.helper()?,
            limits: Limits::default(),
            report: Report::default(),
            statics: States::default(),
        };
        execution.initialize_class(0, 0).await?;
        let root = execution.statics.root(0)?;
        assert_eq!(
            static_integer(&*runtime.borrow(&root)?, 0, 1)?.load(Ordering::Relaxed),
            7
        );
        assert!(execution.check_class(&root, 0).is_err());
        assert_eq!(execution.report.static_storage_allocations, 1);
        drop(root);
        execution.statics.close();
        drop(execution);
        drop(runtime);
        assert_eq!(collector.managed_bytes(), 0);
        assert_eq!(budget.used(), 0);
        Ok(())
    }

    #[test]
    #[expect(clippy::panic_in_result_fn, reason = "cancellation assertions")]
    fn cancellation_during_static_backing_preparation_poisons_the_owner_and_refunds()
    -> Result<(), Error> {
        let program = program()?;
        let budget = Budget::new(config().charged_bytes()?);
        let collector = GarbageCollector::new();
        let runtime = Runtime::new(&collector, config(), &budget)?;
        let mut execution = Execution {
            program: &program,
            runtime: &runtime,
            mutator: runtime.mutator()?,
            helper: runtime.helper()?,
            limits: Limits::default(),
            report: Report::default(),
            statics: States::default(),
        };
        let mut future = Box::pin(execution.initialize_class(0, 0));
        assert!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        assert!(collector.managed_bytes() > 0);
        drop(future);
        assert_eq!(
            execution.statics.state(0)?.phase.get(),
            Phase::EmbeddingAborted
        );
        assert!(execution.statics.prepared(0).is_err());
        execution.statics.close();
        drop(execution);
        drop(runtime);
        assert_eq!(collector.managed_bytes(), 0);
        assert_eq!(budget.used(), 0);
        Ok(())
    }

    #[test]
    #[expect(clippy::panic_in_result_fn, reason = "zero-state assertion")]
    fn fieldless_program_has_no_registry_or_owner_allocation() -> Result<(), Error> {
        let program = Program::from_classfiles([ClassFile::from_bytes(include_bytes!(
            "../tests/fixtures/canonical/CanonicalScalarEntry.class"
        ))?])?;
        let states = States::heap(&program)?;
        assert!(states.registry.is_none());
        let attached = states.attach()?;
        assert!(attached.registry.is_none());
        assert_eq!(attached.owner, 0);
        Ok(())
    }
}
