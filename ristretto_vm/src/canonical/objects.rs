//! Execution of verified closed object methods on a dedicated incremental heap.
use super::layout::Field;
use super::statics::States;
use super::{Error, Layout, Program, exact_symbol};
use ristretto_classfile::{
    BaseType, FieldType, JavaStr, MethodAccessFlags, attributes::Instruction,
};
use ristretto_classloader::Method;
use ristretto_gc::canonical::{Budget, Configuration, Helper, Mutator, Root, Runtime, Work};
use ristretto_gc::{GarbageCollector, Trace};
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::task::Poll;
/// Caller policy limits; these bound work counts, never wall-clock latency.
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub instructions: usize,
    pub request_polls: usize,
    pub cleanup_polls: usize,
    pub frames: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            instructions: 1_000_000,
            request_polls: 100_000,
            cleanup_polls: 1_000_000,
            frames: 64,
        }
    }
}

/// Work performed before the entry body, included in the full Report counters.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InitializationWork {
    pub instructions: usize,
    pub allocations: usize,
    pub helper_polls: usize,
    pub reference_reads: usize,
    pub reference_writes: usize,
    pub calls: usize,
    pub class_initialization_waits: usize,
}
impl InitializationWork {
    pub(super) fn from_report(report: &Report) -> Self {
        Self {
            instructions: report.instructions,
            allocations: report.allocations,
            helper_polls: report.helper_polls,
            reference_reads: report.reference_reads,
            reference_writes: report.reference_writes,
            calls: report.calls,
            class_initialization_waits: report.class_initialization_waits,
        }
    }
}

/// Actual execution counters. This report contains no timing or pause claim.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Report {
    pub result: i32,
    pub instructions: usize,
    pub allocations: usize,
    pub internal_allocations: usize,
    pub static_storage_allocations: usize,
    pub initialized_classes: usize,
    pub class_initializer_calls: usize,
    pub class_initialization_waits: usize,
    pub entry_initialization: InitializationWork,
    pub reference_reads: usize,
    pub reference_writes: usize,
    pub calls: usize,
    pub helper_polls: usize,
    pub released_objects: usize,
    pub payload_bytes_after_cleanup: usize,
    pub catalog_bytes_after_drop: usize,
}

impl Program {
    pub(super) fn layout(&self, class: usize) -> Result<&Layout, Error> {
        self.classes
            .get(class)
            .ok_or_else(|| Error::Invalid("class index".into()))
    }
    pub(super) fn class_index(&self, name: &str) -> Result<usize, Error> {
        self.names
            .get(JavaStr::cow_from_str(name).as_ref())
            .copied()
            .ok_or_else(|| Error::Unsupported(format!("class {name}")))
    }
    fn class_constant(&self, owner: usize, index: u16) -> Result<usize, Error> {
        let name = self
            .layout(owner)?
            .class
            .constant_pool()
            .try_get_class(index)?;
        let target = self
            .names
            .get(name)
            .copied()
            .ok_or_else(|| Error::Unsupported(format!("class {name}")))?;
        let a = &self.layout(owner)?.class;
        let b = &self.layout(target)?.class;
        if !b
            .class_file()
            .access_flags
            .contains(ristretto_classfile::ClassAccessFlags::PUBLIC)
            && a.name().rsplit_once('/').map(|pair| pair.0)
                != b.name().rsplit_once('/').map(|pair| pair.0)
        {
            return Err(Error::Unsupported("inaccessible class".into()));
        }
        Ok(target)
    }
    pub(super) fn member(
        &self,
        owner: usize,
        index: u16,
        field: bool,
    ) -> Result<(String, String, String), Error> {
        let pool = self.layout(owner)?.class.constant_pool();
        let (class, name_type) = if field {
            pool.try_get_field_ref(index)?
        } else {
            pool.try_get_method_ref(index)?
        };
        let (name, descriptor) = pool.try_get_name_and_type(*name_type)?;
        Ok((
            exact_symbol(pool.try_get_class(*class)?)
                .map_err(|e| Error::Invalid(e.to_string()))?
                .into_owned(),
            exact_symbol(pool.try_get_utf8(*name)?)
                .map_err(|e| Error::Invalid(e.to_string()))?
                .into_owned(),
            exact_symbol(pool.try_get_utf8(*descriptor)?)
                .map_err(|e| Error::Invalid(e.to_string()))?
                .into_owned(),
        ))
    }

    fn field(&self, owner: usize, index: u16) -> Result<(usize, Field), Error> {
        let (name, field, descriptor) = self.member(owner, index, true)?;
        let class = self.class_index(&name)?;
        let slot = self
            .layout(class)?
            .field(&field, &descriptor)
            .ok_or_else(|| Error::Unsupported(format!("field {name}.{field} {descriptor}")))?;
        Ok((class, slot))
    }

    fn admit_type(&self, field: &FieldType) -> Result<(), Error> {
        match field {
            FieldType::Base(BaseType::Int) => Ok(()),
            FieldType::Object(name) => self
                .names
                .get(name)
                .map(|_| ())
                .ok_or_else(|| Error::Unsupported(format!("class {name}"))),
            _ => Err(Error::Unsupported(format!("descriptor {field}"))),
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "single explicit opcode admission table"
    )]
    pub(super) fn admit_object_method(&self, class: usize, method: &Method) -> Result<(), Error> {
        if matches!(method.name(), "<clinit>" | "finalize")
            || method.access_flags().intersects(
                MethodAccessFlags::NATIVE
                    | MethodAccessFlags::ABSTRACT
                    | MethodAccessFlags::SYNCHRONIZED,
            )
            || !method.exception_table().is_empty()
            || method.code().is_empty()
        {
            return Err(Error::Unsupported(format!("method {}", method.signature())));
        }
        for field in method.parameters().iter().chain(method.return_type()) {
            self.admit_type(field)?;
        }
        for instruction in method.code() {
            match instruction {
                Instruction::New(index) => {
                    self.class_constant(class, *index)?;
                }
                Instruction::Getstatic(index) => {
                    self.static_field(class, *index)?;
                }
                Instruction::Putstatic(index) => {
                    let (owner, field) = self.static_field(class, *index)?;
                    Self::admit_static_write(class, method, owner, &field)?;
                }
                Instruction::Getfield(index) | Instruction::Putfield(index) => {
                    self.field(class, *index)?;
                }
                Instruction::Invokestatic(index) | Instruction::Invokespecial(index) => {
                    let (owner, name, descriptor) = self.member(class, *index, false)?;
                    if owner == "java/lang/Object"
                        && name == "<init>"
                        && descriptor == "()V"
                        && matches!(instruction, Instruction::Invokespecial(_))
                    {
                        continue;
                    }
                    let target = self.class_index(&owner)?;
                    let target = self
                        .layout(target)?
                        .class
                        .try_get_method(&name, &descriptor)?;
                    let is_static = target.access_flags().contains(MethodAccessFlags::STATIC);
                    if is_static != matches!(instruction, Instruction::Invokestatic(_))
                        || (!is_static && name != "<init>")
                    {
                        return Err(Error::Unsupported("call dispatch".to_owned()));
                    }
                }
                Instruction::Ldc(index) => {
                    self.layout(class)?
                        .class
                        .constant_pool()
                        .try_get_integer(u16::from(*index))?;
                }
                Instruction::Ldc_w(index) => {
                    self.layout(class)?
                        .class
                        .constant_pool()
                        .try_get_integer(*index)?;
                }
                Instruction::Nop
                | Instruction::Aconst_null
                | Instruction::Iconst_m1
                | Instruction::Iconst_0
                | Instruction::Iconst_1
                | Instruction::Iconst_2
                | Instruction::Iconst_3
                | Instruction::Iconst_4
                | Instruction::Iconst_5
                | Instruction::Bipush(_)
                | Instruction::Sipush(_)
                | Instruction::Iload(_)
                | Instruction::Iload_0
                | Instruction::Iload_1
                | Instruction::Iload_2
                | Instruction::Iload_3
                | Instruction::Aload(_)
                | Instruction::Aload_0
                | Instruction::Aload_1
                | Instruction::Aload_2
                | Instruction::Aload_3
                | Instruction::Istore(_)
                | Instruction::Istore_0
                | Instruction::Istore_1
                | Instruction::Istore_2
                | Instruction::Istore_3
                | Instruction::Astore(_)
                | Instruction::Astore_0
                | Instruction::Astore_1
                | Instruction::Astore_2
                | Instruction::Astore_3
                | Instruction::Iinc(_, _)
                | Instruction::Iadd
                | Instruction::Isub
                | Instruction::Imul
                | Instruction::Ineg
                | Instruction::Iand
                | Instruction::Ior
                | Instruction::Ixor
                | Instruction::Dup
                | Instruction::Pop
                | Instruction::Swap
                | Instruction::Goto(_)
                | Instruction::Ifeq(_)
                | Instruction::Ifne(_)
                | Instruction::Iflt(_)
                | Instruction::Ifge(_)
                | Instruction::Ifgt(_)
                | Instruction::Ifle(_)
                | Instruction::If_icmpeq(_)
                | Instruction::If_icmpne(_)
                | Instruction::If_icmplt(_)
                | Instruction::If_icmpge(_)
                | Instruction::If_icmpgt(_)
                | Instruction::If_icmple(_)
                | Instruction::If_acmpeq(_)
                | Instruction::If_acmpne(_)
                | Instruction::Ifnull(_)
                | Instruction::Ifnonnull(_)
                | Instruction::Ireturn
                | Instruction::Areturn
                | Instruction::Return => {}
                _ => return Err(Error::Unsupported(format!("bytecode {instruction:?}"))),
            }
        }
        Ok(())
    }

    /// Run an admitted static `(I)I` entry on a dedicated canonical heap, then reclaim
    /// every object before returning. Polls and bytecode batches yield to the host.
    ///
    /// # Errors
    /// Rejects configuration/entry errors before allocation; reports Java faults,
    /// work limits, heap exhaustion, collector failures or incomplete cleanup.
    pub async fn run(
        &self,
        class: &str,
        method: &str,
        argument: i32,
        configuration: Configuration,
        limits: Limits,
    ) -> Result<Report, Error> {
        let (class, method) = self.entry_method(class, method)?;
        if configuration.mutators != 1
            || configuration.helpers != 1
            || self.classes.iter().any(|layout| {
                layout.references > configuration.edges_per_object
                    || layout.statics.references > configuration.edges_per_object
            })
        {
            return Err(Error::Unsupported(
                "single caller/helper or insufficient edge positions".to_owned(),
            ));
        }
        self.static_capacity(configuration)?;
        let charge = configuration.charged_bytes()?;
        let budget = Budget::new(charge);
        let collector = GarbageCollector::new();
        let runtime = Runtime::new(&collector, configuration, &budget)?;
        let mut execution = Execution {
            program: self,
            mutator: runtime.mutator()?,
            helper: runtime.helper()?,
            runtime: &runtime,
            limits,
            report: Report::default(),
            statics: States::default(),
        };
        let result = execution.entry(class, method, argument).await;
        // Release static storage and all frame roots before dedicated cleanup.
        execution.statics.close();
        execution.cleanup(&collector).await?;
        let mut report = execution.report.clone();
        drop(execution);
        drop(runtime);
        report.catalog_bytes_after_drop = budget.used();
        if report.catalog_bytes_after_drop != 0 {
            return Err(Error::Invalid(
                "catalog credit retained after drop".to_owned(),
            ));
        }
        report.result = result?
            .ok_or_else(|| Error::Invalid("missing result".to_owned()))?
            .int()?;
        Ok(report)
    }
}
/// No managed references, callbacks, dynamic backing or custom destruction.
/// Class metadata lives exclusively in the external immutable Program.
#[derive(Debug)]
pub(super) struct Payload {
    pub(super) class: usize,
    pub(super) static_storage: bool,
    pub(super) integers: [AtomicI32; 8],
}

// SAFETY: Payload contains only a numeric class ID and eight primitive atomics.
// Every managed field is owned exclusively by the catalog, never by this payload.
#[expect(
    unsafe_code,
    reason = "audited private reference-free canonical Java payload"
)]
impl Trace for Payload {
    fn canonical_payload() -> Option<ristretto_gc::CanonicalPayload<Self>> {
        // SAFETY: every field is primitive storage. Managed edges live in the catalog.
        Some(unsafe { ristretto_gc::CanonicalPayload::new() })
    }

    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
}

#[derive(Clone, Debug)]
pub(super) enum Value {
    Int(i32),
    Reference(Option<Root<Payload>>),
    Unused,
}

impl Value {
    pub(super) fn int(self) -> Result<i32, Error> {
        if let Self::Int(value) = self {
            Ok(value)
        } else {
            Err(Error::Invalid("expected int".to_owned()))
        }
    }
    pub(super) fn reference(self) -> Result<Option<Root<Payload>>, Error> {
        if let Self::Reference(value) = self {
            Ok(value)
        } else {
            Err(Error::Invalid("expected reference".to_owned()))
        }
    }
    fn object(self) -> Result<Root<Payload>, Error> {
        self.reference()?
            .ok_or_else(|| Error::Invalid("NullPointerException".to_owned()))
    }
}

pub(super) struct Execution<'p, 'r, 'gc> {
    pub(super) program: &'p Program,
    pub(super) runtime: &'r Runtime<'gc>,
    pub(super) mutator: Mutator<'gc>,
    pub(super) helper: Helper<'gc>,
    pub(super) limits: Limits,
    pub(super) report: Report,
    pub(super) statics: States,
}

pub(super) async fn complete<T>(
    helper: &mut Helper<'_>,
    report: &mut Report,
    limit: usize,
    mut poll: impl FnMut() -> Result<Poll<T>, ristretto_gc::canonical::Error>,
) -> Result<T, Error> {
    for _ in 0..limit {
        if let Poll::Ready(value) = poll()? {
            return Ok(value);
        }
        help(helper, report)?;
        tokio::task::yield_now().await;
    }
    Err(Error::Limit("request poll"))
}

pub(super) fn help(helper: &mut Helper<'_>, report: &mut Report) -> Result<(), Error> {
    let work = helper.poll()?;
    report.helper_polls += 1;
    if work == Work::ReleasedObjectSlot {
        report.released_objects += 1;
    }
    Ok(())
}

impl Execution<'_, '_, '_> {
    pub(super) async fn entry(
        &mut self,
        class: usize,
        method: Arc<Method>,
        argument: i32,
    ) -> Result<Option<Value>, Error> {
        let initialized = self.initialize_class(class, 0).await;
        self.report.entry_initialization = InitializationWork::from_report(&self.report);
        initialized?;
        let pool = self.program.layout(class)?.class.constant_pool();
        if method
            .code()
            .iter()
            .all(|op| super::integer::admitted(op, pool))
        {
            self.report.calls += 1;
            let (value, count) =
                super::integer::run_counted(&method, pool, argument, self.limits.instructions)?;
            self.report.instructions += count;
            Ok(Some(Value::Int(value.int()?)))
        } else {
            self.call(class, method, vec![Value::Int(argument)], 0)
                .await
        }
    }

    async fn cleanup(&mut self, collector: &GarbageCollector) -> Result<(), Error> {
        for index in 0..self.limits.cleanup_polls {
            help(&mut self.helper, &mut self.report)?;
            let bytes = collector.managed_bytes();
            if bytes == 0 {
                self.report.payload_bytes_after_cleanup = bytes;
                return Ok(());
            }
            if index % 64 == 0 {
                tokio::task::yield_now().await;
            }
        }
        Err(Error::Limit("cleanup poll"))
    }

    fn check_type(&self, value: &Value, field: &FieldType) -> Result<(), Error> {
        match (value, field) {
            (Value::Int(_), FieldType::Base(BaseType::Int))
            | (Value::Reference(None), FieldType::Object(_)) => Ok(()),
            (Value::Reference(Some(root)), FieldType::Object(name)) => {
                let class = self.program.class_index(name.to_str_lossy().as_ref())?;
                self.check_class(root, class)
            }
            _ => Err(Error::Invalid("method argument/return type".to_owned())),
        }
    }

    pub(super) fn check_class(&self, root: &Root<Payload>, class: usize) -> Result<(), Error> {
        let view = self.runtime.borrow(root)?;
        if !view.static_storage && view.class == class {
            Ok(())
        } else {
            Err(Error::Invalid("receiver/field class mismatch".to_owned()))
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "single explicit restricted bytecode dispatch table"
    )]
    #[async_recursion::async_recursion(?Send)]
    pub(super) async fn call(
        &mut self,
        class: usize,
        method: Arc<Method>,
        arguments: Vec<Value>,
        depth: usize,
    ) -> Result<Option<Value>, Error> {
        if depth >= self.limits.frames {
            return Err(Error::Limit("frame"));
        }
        let is_static = method.access_flags().contains(MethodAccessFlags::STATIC);
        if is_static && method.name() != "<clinit>" {
            self.initialize_class(class, depth).await?;
        }
        let receiver_count = usize::from(!is_static);
        if arguments.len() != method.parameters().len() + receiver_count {
            return Err(Error::Invalid("argument count".to_owned()));
        }
        if !is_static {
            let root = arguments
                .first()
                .cloned()
                .ok_or_else(|| Error::Invalid("receiver".to_owned()))?
                .object()?;
            self.check_class(&root, class)?;
        }
        for (argument, field) in arguments
            .iter()
            .skip(receiver_count)
            .zip(method.parameters())
        {
            self.check_type(argument, field)?;
        }
        let mut locals = vec![Value::Unused; method.max_locals()];
        for (index, argument) in arguments.into_iter().enumerate() {
            *locals
                .get_mut(index)
                .ok_or_else(|| Error::Invalid("argument local".to_owned()))? = argument;
        }
        let mut stack = Vec::with_capacity(method.max_stack());
        let mut pc = 0;
        self.report.calls += 1;
        loop {
            if self.report.instructions >= self.limits.instructions {
                return Err(Error::Limit("instruction"));
            }
            let instruction = method
                .code()
                .get(pc)
                .ok_or_else(|| Error::Invalid("program counter".to_owned()))?;
            self.report.instructions += 1;
            help(&mut self.helper, &mut self.report)?;
            if self.report.instructions.is_multiple_of(64) {
                tokio::task::yield_now().await;
            }
            match instruction {
                Instruction::Nop => {}
                Instruction::Aconst_null => stack.push(Value::Reference(None)),
                Instruction::Iconst_m1 => stack.push(Value::Int(-1)),
                Instruction::Iconst_0 => stack.push(Value::Int(0)),
                Instruction::Iconst_1 => stack.push(Value::Int(1)),
                Instruction::Iconst_2 => stack.push(Value::Int(2)),
                Instruction::Iconst_3 => stack.push(Value::Int(3)),
                Instruction::Iconst_4 => stack.push(Value::Int(4)),
                Instruction::Iconst_5 => stack.push(Value::Int(5)),
                Instruction::Bipush(value) => stack.push(Value::Int(i32::from(*value))),
                Instruction::Sipush(value) => stack.push(Value::Int(i32::from(*value))),
                Instruction::Ldc(index) => stack.push(Value::Int(
                    *self
                        .program
                        .layout(class)?
                        .class
                        .constant_pool()
                        .try_get_integer(u16::from(*index))?,
                )),
                Instruction::Ldc_w(index) => stack.push(Value::Int(
                    *self
                        .program
                        .layout(class)?
                        .class
                        .constant_pool()
                        .try_get_integer(*index)?,
                )),
                Instruction::Iload(index) | Instruction::Aload(index) => {
                    load(&locals, &mut stack, usize::from(*index))?;
                }
                Instruction::Iload_0 | Instruction::Aload_0 => load(&locals, &mut stack, 0)?,
                Instruction::Iload_1 | Instruction::Aload_1 => load(&locals, &mut stack, 1)?,
                Instruction::Iload_2 | Instruction::Aload_2 => load(&locals, &mut stack, 2)?,
                Instruction::Iload_3 | Instruction::Aload_3 => load(&locals, &mut stack, 3)?,
                Instruction::Istore(index) | Instruction::Astore(index) => {
                    store(&mut locals, &mut stack, usize::from(*index))?;
                }
                Instruction::Istore_0 | Instruction::Astore_0 => store(&mut locals, &mut stack, 0)?,
                Instruction::Istore_1 | Instruction::Astore_1 => store(&mut locals, &mut stack, 1)?,
                Instruction::Istore_2 | Instruction::Astore_2 => store(&mut locals, &mut stack, 2)?,
                Instruction::Istore_3 | Instruction::Astore_3 => store(&mut locals, &mut stack, 3)?,
                Instruction::Iinc(index, amount) => {
                    let value = locals
                        .get_mut(usize::from(*index))
                        .ok_or_else(|| Error::Invalid("local index".to_owned()))?;
                    *value = Value::Int(value.clone().int()?.wrapping_add(i32::from(*amount)));
                }
                Instruction::Iadd
                | Instruction::Isub
                | Instruction::Imul
                | Instruction::Iand
                | Instruction::Ior
                | Instruction::Ixor => {
                    let right = pop(&mut stack)?.int()?;
                    let left = pop(&mut stack)?.int()?;
                    let value = match instruction {
                        Instruction::Iadd => left.wrapping_add(right),
                        Instruction::Isub => left.wrapping_sub(right),
                        Instruction::Imul => left.wrapping_mul(right),
                        Instruction::Iand => left & right,
                        Instruction::Ior => left | right,
                        _ => left ^ right,
                    };
                    stack.push(Value::Int(value));
                }
                Instruction::Ineg => {
                    let value = pop(&mut stack)?.int()?;
                    stack.push(Value::Int(value.wrapping_neg()));
                }
                Instruction::Dup => {
                    let value = stack
                        .last()
                        .cloned()
                        .ok_or_else(|| Error::Invalid("stack underflow".to_owned()))?;
                    stack.push(value);
                }
                Instruction::Pop => {
                    pop(&mut stack)?;
                }
                Instruction::Swap => {
                    let a = pop(&mut stack)?;
                    let b = pop(&mut stack)?;
                    stack.push(a);
                    stack.push(b);
                }
                Instruction::New(index) => {
                    let class = self.program.class_constant(class, *index)?;
                    self.initialize_class(class, depth + 1).await?;
                    let payload = Payload {
                        class,
                        static_storage: false,
                        integers: std::array::from_fn(|_| AtomicI32::new(0)),
                    };
                    let mut request = self.mutator.allocate(payload)?;
                    let root = complete(
                        &mut self.helper,
                        &mut self.report,
                        self.limits.request_polls,
                        || request.poll(),
                    )
                    .await?;
                    self.report.allocations += 1;
                    stack.push(Value::Reference(Some(root)));
                }
                Instruction::Getstatic(index) => {
                    let (owner, field) = self.program.static_field(class, *index)?;
                    stack.push(self.read_static(owner, &field, depth + 1).await?);
                }
                Instruction::Putstatic(index) => {
                    let (owner, field) = self.program.static_field(class, *index)?;
                    let value = pop(&mut stack)?;
                    self.write_static(class, &method, owner, &field, value, depth + 1)
                        .await?;
                }
                Instruction::Getfield(index) => {
                    let (owner, field) = self.program.field(class, *index)?;
                    let root = pop(&mut stack)?.object()?;
                    self.check_class(&root, owner)?;
                    let value = match field {
                        Field::Int(index) => Value::Int(
                            self.runtime
                                .borrow(&root)?
                                .integers
                                .get(index)
                                .ok_or_else(|| Error::Invalid("integer field index".to_owned()))?
                                .load(Ordering::Relaxed),
                        ),
                        Field::Reference { edge, .. } => {
                            let mut request =
                                self.mutator.child::<Payload, Payload>(&root, edge)?;
                            let child = complete(
                                &mut self.helper,
                                &mut self.report,
                                self.limits.request_polls,
                                || request.poll(),
                            )
                            .await?;
                            self.report.reference_reads += 1;
                            Value::Reference(child)
                        }
                    };
                    stack.push(value);
                }
                Instruction::Putfield(index) => {
                    let (owner, field) = self.program.field(class, *index)?;
                    let value = pop(&mut stack)?;
                    let root = pop(&mut stack)?.object()?;
                    self.check_class(&root, owner)?;
                    match field {
                        Field::Int(index) => self
                            .runtime
                            .borrow(&root)?
                            .integers
                            .get(index)
                            .ok_or_else(|| Error::Invalid("integer field index".to_owned()))?
                            .store(value.int()?, Ordering::Relaxed),
                        Field::Reference { edge, class } => {
                            let target = value.reference()?;
                            if let Some(target) = &target {
                                self.check_class(target, class)?;
                            }
                            let mut request = self.mutator.write(&root, edge, target.as_ref())?;
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
                }
                Instruction::Invokespecial(index) | Instruction::Invokestatic(index) => {
                    let (owner, name, descriptor) = self.program.member(class, *index, false)?;
                    if owner == "java/lang/Object" {
                        // Admission permits only Object.<init>()V, whose body is empty.
                        pop(&mut stack)?.object()?;
                    } else {
                        let owner = self.program.class_index(&owner)?;
                        let target = self
                            .program
                            .layout(owner)?
                            .class
                            .try_get_method(&name, &descriptor)?;
                        let count = target.parameters().len()
                            + usize::from(matches!(instruction, Instruction::Invokespecial(_)));
                        let start = stack
                            .len()
                            .checked_sub(count)
                            .ok_or_else(|| Error::Invalid("call stack underflow".to_owned()))?;
                        let arguments = stack.split_off(start);
                        if let Some(value) = self.call(owner, target, arguments, depth + 1).await? {
                            stack.push(value);
                        }
                    }
                }
                Instruction::Return | Instruction::Ireturn | Instruction::Areturn => {
                    let value = if matches!(instruction, Instruction::Return) {
                        None
                    } else {
                        Some(pop(&mut stack)?)
                    };
                    match (value.as_ref(), method.return_type()) {
                        (None, None) => {}
                        (Some(value), Some(field)) => self.check_type(value, field)?,
                        _ => return Err(Error::Invalid("return type".to_owned())),
                    }
                    return Ok(value);
                }
                instruction => {
                    if branch(instruction, &mut stack)? {
                        pc = method
                            .branch_target_index(pc)
                            .ok_or_else(|| Error::Invalid("branch target".to_owned()))?;
                        continue;
                    }
                }
            }
            if stack.len() > method.max_stack() {
                return Err(Error::Invalid("operand stack overflow".to_owned()));
            }
            pc += 1;
        }
    }
}

fn pop(stack: &mut Vec<Value>) -> Result<Value, Error> {
    stack
        .pop()
        .ok_or_else(|| Error::Invalid("stack underflow".to_owned()))
}
fn load(locals: &[Value], stack: &mut Vec<Value>, index: usize) -> Result<(), Error> {
    let value = locals
        .get(index)
        .ok_or_else(|| Error::Invalid("local index".to_owned()))?
        .clone();
    if matches!(value, Value::Unused) {
        return Err(Error::Invalid("uninitialized local".to_owned()));
    }
    stack.push(value);
    Ok(())
}
fn store(locals: &mut [Value], stack: &mut Vec<Value>, index: usize) -> Result<(), Error> {
    *locals
        .get_mut(index)
        .ok_or_else(|| Error::Invalid("local index".to_owned()))? = pop(stack)?;
    Ok(())
}

fn branch(instruction: &Instruction, stack: &mut Vec<Value>) -> Result<bool, Error> {
    Ok(match instruction {
        Instruction::Goto(_) => true,
        Instruction::Ifeq(_) => pop(stack)?.int()? == 0,
        Instruction::Ifne(_) => pop(stack)?.int()? != 0,
        Instruction::Iflt(_) => pop(stack)?.int()? < 0,
        Instruction::Ifge(_) => pop(stack)?.int()? >= 0,
        Instruction::Ifgt(_) => pop(stack)?.int()? > 0,
        Instruction::Ifle(_) => pop(stack)?.int()? <= 0,
        Instruction::Ifnull(_) => pop(stack)?.reference()?.is_none(),
        Instruction::Ifnonnull(_) => pop(stack)?.reference()?.is_some(),
        Instruction::If_icmpeq(_)
        | Instruction::If_icmpne(_)
        | Instruction::If_icmplt(_)
        | Instruction::If_icmpge(_)
        | Instruction::If_icmpgt(_)
        | Instruction::If_icmple(_) => {
            let right = pop(stack)?.int()?;
            let left = pop(stack)?.int()?;
            match instruction {
                Instruction::If_icmpeq(_) => left == right,
                Instruction::If_icmpne(_) => left != right,
                Instruction::If_icmplt(_) => left < right,
                Instruction::If_icmpge(_) => left >= right,
                Instruction::If_icmpgt(_) => left > right,
                _ => left <= right,
            }
        }
        Instruction::If_acmpeq(_) | Instruction::If_acmpne(_) => {
            let right = pop(stack)?.reference()?;
            let left = pop(stack)?.reference()?;
            let same = match (&left, &right) {
                (None, None) => true,
                (Some(a), Some(b)) => a.same_object(b),
                _ => false,
            };
            if matches!(instruction, Instruction::If_acmpeq(_)) {
                same
            } else {
                !same
            }
        }
        _ => return Err(Error::Unsupported(format!("bytecode {instruction:?}"))),
    })
}
