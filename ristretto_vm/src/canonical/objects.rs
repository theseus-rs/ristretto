//! Execution of verified closed object methods on a dedicated incremental heap.
use super::{Error, Layout, Program, Result, exact_symbol};
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

/// Actual execution counters. This report contains no timing or pause claim.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Report {
    pub result: i32,
    pub instructions: usize,
    pub allocations: usize,
    pub reference_reads: usize,
    pub reference_writes: usize,
    pub calls: usize,
    pub helper_polls: usize,
    pub released_objects: usize,
    pub payload_bytes_after_cleanup: usize,
    pub catalog_bytes_after_drop: usize,
}

impl Program {
    fn layout(&self, class: usize) -> Result<&Layout> {
        self.classes
            .get(class)
            .ok_or_else(|| Error::Invalid("class index".into()))
    }
    fn class_index(&self, name: &str) -> Result<usize> {
        self.names
            .get(JavaStr::cow_from_str(name).as_ref())
            .copied()
            .ok_or_else(|| Error::Unsupported(format!("class {name}")))
    }
    fn class_constant(&self, owner: usize, index: u16) -> Result<usize> {
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
    fn member(&self, owner: usize, index: u16, field: bool) -> Result<(String, String, String)> {
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

    fn field(&self, owner: usize, index: u16) -> Result<(usize, usize)> {
        let (name, field, descriptor) = self.member(owner, index, true)?;
        let class = self.class_index(&name)?;
        let slot = self
            .layout(class)?
            .slot(JavaStr::cow_from_str(&field).as_ref())
            .filter(|_| descriptor == "I")
            .ok_or_else(|| Error::Unsupported(format!("field {name}.{field} {descriptor}")))?;
        Ok((class, slot))
    }

    pub(super) fn admit_object_method(&self, owner: usize, method: &Method) -> Result<()> {
        use Instruction as I;
        let constructor = method.name() == "<init>"
            && !method.is_static()
            && matches!(method.descriptor(), "()V" | "(I)V");
        if !(constructor || (method.is_static() && method.descriptor() == "(I)I"))
            || method.code().is_empty()
        {
            return Err(Error::Unsupported(method.signature()));
        }
        for op in method.code() {
            match op {
                I::New(index) => {
                    self.class_constant(owner, *index)?;
                }
                I::Getfield(index) | I::Putfield(index) => {
                    self.field(owner, *index)?;
                }
                I::Invokespecial(index) => {
                    let (class, name, desc) = self.member(owner, *index, false)?;
                    if name != "<init>" || !matches!(desc.as_str(), "()V" | "(I)V") {
                        return Err(Error::Unsupported("special call".into()));
                    }
                    if class == "java/lang/Object" && desc == "()V" {
                        continue;
                    }
                    let target = self
                        .layout(self.class_index(&class)?)?
                        .class
                        .try_get_method(&name, &desc)?;
                    if target.is_static() {
                        return Err(Error::Unsupported("static constructor".into()));
                    }
                }
                I::Aload_0
                | I::Aload_1
                | I::Aload_2
                | I::Aload_3
                | I::Aload(_)
                | I::Iload_0
                | I::Iload_1
                | I::Iload_2
                | I::Iload_3
                | I::Iload(_)
                | I::Astore_0
                | I::Astore_1
                | I::Astore_2
                | I::Astore_3
                | I::Astore(_)
                | I::Iconst_m1
                | I::Iconst_0
                | I::Iconst_1
                | I::Iconst_2
                | I::Iconst_3
                | I::Iconst_4
                | I::Iconst_5
                | I::Bipush(_)
                | I::Sipush(_)
                | I::Aconst_null
                | I::Dup
                | I::Return
                | I::Ireturn => {}
                _ => return Err(Error::Unsupported(format!("object instruction {op}"))),
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
    ) -> Result<Report> {
        let class = self.class_index(class)?;
        let method = self.layout(class)?.class.try_get_method(method, "(I)I")?;
        if !method.access_flags().contains(MethodAccessFlags::STATIC) {
            return Err(Error::Unsupported("entry must be static (I)I".to_owned()));
        }
        if configuration.mutators != 1 || configuration.helpers != 1 {
            return Err(Error::Unsupported(
                "single caller/helper or insufficient edge positions".to_owned(),
            ));
        }
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
        };
        let pool = self.layout(class)?.class.constant_pool();
        let result = if method
            .code()
            .iter()
            .all(|op| super::integer::admitted(op, pool))
        {
            execution.report.calls = 1;
            super::integer::run_counted(&method, pool, argument, limits.instructions).and_then(
                |(value, count)| {
                    execution.report.instructions = count;
                    Ok(Some(Value::Int(value.int()?)))
                },
            )
        } else {
            execution
                .call(class, method, vec![Value::Int(argument)], 0)
                .await
        };
        // Even execution errors release all frame roots before cleanup.
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
struct Payload {
    class: usize,
    integers: [AtomicI32; 8],
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
enum Value {
    Int(i32),
    Reference(Option<Root<Payload>>),
    Unused,
}

impl Value {
    fn int(self) -> Result<i32> {
        if let Self::Int(value) = self {
            Ok(value)
        } else {
            Err(Error::Invalid("expected int".to_owned()))
        }
    }
    fn reference(self) -> Result<Option<Root<Payload>>> {
        if let Self::Reference(value) = self {
            Ok(value)
        } else {
            Err(Error::Invalid("expected reference".to_owned()))
        }
    }
    fn object(self) -> Result<Root<Payload>> {
        self.reference()?
            .ok_or_else(|| Error::Invalid("NullPointerException".to_owned()))
    }
}

struct Execution<'p, 'r, 'gc> {
    program: &'p Program,
    runtime: &'r Runtime<'gc>,
    mutator: Mutator<'gc>,
    helper: Helper<'gc>,
    limits: Limits,
    report: Report,
}

async fn complete<T>(
    helper: &mut Helper<'_>,
    report: &mut Report,
    limit: usize,
    mut poll: impl FnMut() -> std::result::Result<Poll<T>, ristretto_gc::canonical::Error>,
) -> Result<T> {
    for _ in 0..limit {
        if let Poll::Ready(value) = poll()? {
            return Ok(value);
        }
        help(helper, report)?;
        tokio::task::yield_now().await;
    }
    Err(Error::Limit("request poll"))
}

fn help(helper: &mut Helper<'_>, report: &mut Report) -> Result<()> {
    let work = helper.poll()?;
    report.helper_polls += 1;
    if work == Work::ReleasedObjectSlot {
        report.released_objects += 1;
    }
    Ok(())
}

impl Execution<'_, '_, '_> {
    async fn cleanup(&mut self, collector: &GarbageCollector) -> Result<()> {
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

    fn check_type(&self, value: &Value, field: &FieldType) -> Result<()> {
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

    fn check_class(&self, root: &Root<Payload>, class: usize) -> Result<()> {
        if self.runtime.borrow(root)?.class == class {
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
    async fn call(
        &mut self,
        class: usize,
        method: Arc<Method>,
        arguments: Vec<Value>,
        depth: usize,
    ) -> Result<Option<Value>> {
        use Instruction as I;
        if depth >= self.limits.frames {
            return Err(Error::Limit("frame"));
        }
        let is_static = method.access_flags().contains(MethodAccessFlags::STATIC);
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
                I::Aconst_null => stack.push(Value::Reference(None)),
                I::Iconst_m1 => stack.push(Value::Int(-1)),
                I::Iconst_0 => stack.push(Value::Int(0)),
                I::Iconst_1 => stack.push(Value::Int(1)),
                I::Iconst_2 => stack.push(Value::Int(2)),
                I::Iconst_3 => stack.push(Value::Int(3)),
                I::Iconst_4 => stack.push(Value::Int(4)),
                I::Iconst_5 => stack.push(Value::Int(5)),
                I::Bipush(value) => stack.push(Value::Int(i32::from(*value))),
                I::Sipush(value) => stack.push(Value::Int(i32::from(*value))),
                I::Iload(index) | I::Aload(index) => {
                    load(&locals, &mut stack, usize::from(*index))?;
                }
                I::Iload_0 | I::Aload_0 => load(&locals, &mut stack, 0)?,
                I::Iload_1 | I::Aload_1 => load(&locals, &mut stack, 1)?,
                I::Iload_2 | I::Aload_2 => load(&locals, &mut stack, 2)?,
                I::Iload_3 | I::Aload_3 => load(&locals, &mut stack, 3)?,
                I::Istore(index) | I::Astore(index) => {
                    store(&mut locals, &mut stack, usize::from(*index))?;
                }
                I::Istore_0 | I::Astore_0 => store(&mut locals, &mut stack, 0)?,
                I::Istore_1 | I::Astore_1 => store(&mut locals, &mut stack, 1)?,
                I::Istore_2 | I::Astore_2 => store(&mut locals, &mut stack, 2)?,
                I::Istore_3 | I::Astore_3 => store(&mut locals, &mut stack, 3)?,
                I::Dup => {
                    let value = stack
                        .last()
                        .cloned()
                        .ok_or_else(|| Error::Invalid("stack underflow".to_owned()))?;
                    stack.push(value);
                }
                I::New(index) => {
                    let class = self.program.class_constant(class, *index)?;
                    let payload = Payload {
                        class,
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
                I::Getfield(index) => {
                    let (owner, field) = self.program.field(class, *index)?;
                    let root = pop(&mut stack)?.object()?;
                    self.check_class(&root, owner)?;
                    let value = Value::Int(
                        self.runtime
                            .borrow(&root)?
                            .integers
                            .get(field)
                            .ok_or_else(|| Error::Invalid("integer field index".into()))?
                            .load(Ordering::Relaxed),
                    );
                    stack.push(value);
                }
                I::Putfield(index) => {
                    let (owner, field) = self.program.field(class, *index)?;
                    let value = pop(&mut stack)?;
                    let root = pop(&mut stack)?.object()?;
                    self.check_class(&root, owner)?;
                    self.runtime
                        .borrow(&root)?
                        .integers
                        .get(field)
                        .ok_or_else(|| Error::Invalid("integer field index".into()))?
                        .store(value.int()?, Ordering::Relaxed);
                }
                I::Invokespecial(index) => {
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
                            + usize::from(matches!(instruction, I::Invokespecial(_)));
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
                I::Return | I::Ireturn => {
                    let value = if matches!(instruction, I::Return) {
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
                other => return Err(Error::Unsupported(format!("object instruction {other}"))),
            }
            if stack.len() > method.max_stack() {
                return Err(Error::Invalid("operand stack overflow".to_owned()));
            }
            pc += 1;
        }
    }
}

fn pop(stack: &mut Vec<Value>) -> Result<Value> {
    stack
        .pop()
        .ok_or_else(|| Error::Invalid("stack underflow".to_owned()))
}
fn load(locals: &[Value], stack: &mut Vec<Value>, index: usize) -> Result<()> {
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
fn store(locals: &mut [Value], stack: &mut Vec<Value>, index: usize) -> Result<()> {
    *locals
        .get_mut(index)
        .ok_or_else(|| Error::Invalid("local index".to_owned()))? = pop(stack)?;
    Ok(())
}
