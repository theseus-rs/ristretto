//! Opt-in execution coverage. These records deliberately retain method/code metadata;
//! diagnostic builds must not be substituted for uninstrumented timing runs.

use ahash::AHashMap;
use ristretto_classloader::{Class, Method};
use ristretto_gc::sync::Mutex;
use ristretto_jit::Function;
use serde::Serialize;
use std::sync::Arc;
use std::sync::Weak;
use std::time::Duration;
#[cfg(not(target_family = "wasm"))]
use std::time::Instant;

/// A diagnostic snapshot. Entries cover the VM's entire lifetime through the snapshot.
#[derive(Debug, Serialize)]
pub struct ExecutionProfile {
    pub schema_version: u32,
    pub diagnostic_only: bool,
    pub qualification: bool,
    pub methods: Vec<MethodExecutionProfile>,
    pub limitations: [&'static str; 4],
}

/// One method instance; identical names from different loaders remain separate records.
#[derive(Clone, Debug, Serialize)]
pub struct MethodExecutionProfile {
    pub instance: usize,
    pub class: String,
    pub name: String,
    pub descriptor: String,
    pub rejection_reason: Option<&'static str>,
    /// JVM byte offset and opcode for an instruction-level production rejection.
    pub rejected_bytecode: Option<(usize, u8)>,
    /// Explicit opt-in preparation; the production rejection reason is retained.
    pub experimental_preparation: bool,
    pub compilation_state: &'static str,
    pub queued_requests: u64,
    pub compilation_attempts: u64,
    pub compilation_successes: u64,
    pub compilation_failures: u64,
    pub compilation_ns: u128,
    pub last_error: Option<String>,
    pub interpreter_entries: u64,
    pub intrinsic_entries: u64,
    pub compiled_entries: u64,
    pub compiled_backedges: u64,
    pub compiled_resumes: u64,
    pub generated_code_bytes: usize,
}

/// Opaque diagnostic counter boundary. Dropping it has no execution side effects.
/// It belongs to one VM recorder and does not keep that VM alive.
#[derive(Debug)]
pub struct ExecutionCheckpoint {
    owner: Weak<Recorder>,
    label: String,
    profile: ExecutionProfile,
    #[cfg(not(target_family = "wasm"))]
    started: Instant,
    #[cfg(not(target_family = "wasm"))]
    finished: Instant,
}

/// Counter differences between two snapshots, not an atomic VM-wide time cut.
/// Native boundary costs are explicit; Wasm needs no host clock for this API.
#[derive(Debug, Serialize)]
pub struct ExecutionInterval {
    pub schema_version: u32,
    pub diagnostic_only: bool,
    pub qualification: bool,
    pub scope: &'static str,
    pub label: String,
    pub boundary_span_ns: Option<u128>,
    pub start_snapshot_ns: Option<u128>,
    pub end_snapshot_ns: Option<u128>,
    pub methods: Vec<MethodExecutionInterval>,
    pub limitations: [&'static str; 4],
}

/// Activity in the interval, retaining each method/loader instance separately.
#[derive(Debug, Serialize)]
pub struct MethodExecutionInterval {
    #[serde(flatten)]
    pub activity: MethodExecutionProfile,
    pub present_at_start: bool,
    pub compiled_before_start: bool,
    pub code_bytes_at_start: usize,
    pub code_bytes_at_end: usize,
}

struct Record {
    // Retention prevents pointer-key ABA, without retaining VM/thread owners.
    _method: Arc<Method>,
    data: MethodExecutionProfile,
    functions: Vec<Arc<Function>>,
}

#[derive(Default)]
pub(crate) struct Recorder {
    records: Mutex<AHashMap<usize, Record>>,
    #[cfg(all(
        feature = "experimental-execution-measurement",
        target_os = "linux",
        target_has_atomic = "64"
    ))]
    pub(crate) measurement: crate::execution_measurement::State,
}

impl std::fmt::Debug for Recorder {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("Recorder").finish_non_exhaustive()
    }
}

impl Recorder {
    pub(crate) fn checkpoint(self: &Arc<Self>, label: String) -> ExecutionCheckpoint {
        #[cfg(not(target_family = "wasm"))]
        let started = Instant::now();
        let profile = self.snapshot();
        ExecutionCheckpoint {
            owner: Arc::downgrade(self),
            label,
            profile,
            #[cfg(not(target_family = "wasm"))]
            started,
            #[cfg(not(target_family = "wasm"))]
            finished: Instant::now(),
        }
    }

    pub(crate) fn since(
        &self,
        checkpoint: ExecutionCheckpoint,
    ) -> crate::Result<ExecutionInterval> {
        if !std::ptr::eq(self, checkpoint.owner.as_ptr()) {
            return Err(crate::Error::InternalError(
                "execution checkpoint belongs to another VM".to_owned(),
            ));
        }
        #[cfg(not(target_family = "wasm"))]
        let started = Instant::now();
        let profile = self.snapshot();
        #[cfg(not(target_family = "wasm"))]
        let finished = Instant::now();
        let before: AHashMap<_, _> = checkpoint
            .profile
            .methods
            .into_iter()
            .map(|method| (method.instance, method))
            .collect();
        let mut methods = Vec::with_capacity(profile.methods.len());
        for mut activity in profile.methods {
            let previous = before.get(&activity.instance);
            let code_bytes_at_start = previous.map_or(0, |method| method.generated_code_bytes);
            let code_bytes_at_end = activity.generated_code_bytes;
            let present_at_start = previous.is_some();
            let compiled_before_start =
                previous.is_some_and(|method| method.compilation_successes > 0);
            macro_rules! delta {
                ($($field:ident),+ $(,)?) => {$(
                    activity.$field = activity.$field.checked_sub(previous.map_or(0, |method| method.$field))
                        .ok_or_else(|| crate::Error::InternalError(
                            concat!("execution counter decreased: ", stringify!($field)).to_owned()
                        ))?;
                )+};
            }
            delta!(
                queued_requests,
                compilation_attempts,
                compilation_successes,
                compilation_failures,
                compilation_ns,
                interpreter_entries,
                intrinsic_entries,
                compiled_entries,
                compiled_backedges,
                compiled_resumes,
                generated_code_bytes
            );
            methods.push(MethodExecutionInterval {
                activity,
                present_at_start,
                compiled_before_start,
                code_bytes_at_start,
                code_bytes_at_end,
            });
        }
        #[cfg(not(target_family = "wasm"))]
        let timing = (
            Some(finished.duration_since(checkpoint.started).as_nanos()),
            Some(
                checkpoint
                    .finished
                    .duration_since(checkpoint.started)
                    .as_nanos(),
            ),
            Some(finished.duration_since(started).as_nanos()),
        );
        #[cfg(target_family = "wasm")]
        let timing = (None, None, None);
        Ok(ExecutionInterval {
            schema_version: 1,
            diagnostic_only: true,
            qualification: false,
            scope: "checkpoint_interval",
            label: checkpoint.label,
            boundary_span_ns: timing.0,
            start_snapshot_ns: timing.1,
            end_snapshot_ns: timing.2,
            methods,
            limitations: [
                "per-method snapshot cuts are not an atomic VM-wide measurement boundary",
                "concurrent calls and queued compilation are not awaited or stopped",
                "compilation time is charged when compilation finishes, including work begun earlier",
                "diagnostic recorder overhead and retained metadata exclude timing qualification",
            ],
        })
    }

    fn update(&self, class: &Class, method: &Arc<Method>, update: impl FnOnce(&mut Record)) {
        let mut records = self.records.lock();
        #[cfg(all(
            feature = "experimental-execution-measurement",
            target_os = "linux",
            target_has_atomic = "64"
        ))]
        if !self.measurement.admit_method(
            records.len(),
            records.contains_key(&(Arc::as_ptr(method) as usize)),
            class.name(),
            method.name(),
            method.descriptor(),
        ) {
            return;
        }
        let instance = records.len() + 1;
        let record = records
            .entry(Arc::as_ptr(method) as usize)
            .or_insert_with(|| Record {
                _method: method.clone(),
                data: MethodExecutionProfile {
                    instance,
                    class: class.name().to_owned(),
                    name: method.name().to_owned(),
                    descriptor: method.descriptor().to_owned(),
                    rejection_reason: ristretto_jit::Compiler::rejection_reason(
                        method.definition(),
                    ),
                    rejected_bytecode: ristretto_jit::Compiler::rejected_bytecode(
                        method.definition(),
                    ),
                    experimental_preparation: false,
                    compilation_state: "not_requested",
                    queued_requests: 0,
                    compilation_attempts: 0,
                    compilation_successes: 0,
                    compilation_failures: 0,
                    compilation_ns: 0,
                    last_error: None,
                    interpreter_entries: 0,
                    intrinsic_entries: 0,
                    compiled_entries: 0,
                    compiled_backedges: 0,
                    compiled_resumes: 0,
                    generated_code_bytes: 0,
                },
                functions: Vec::new(),
            });
        update(record);
    }

    pub(crate) fn interpreted(&self, class: &Class, method: &Arc<Method>) {
        self.update(class, method, |r| r.data.interpreter_entries += 1);
    }

    pub(crate) fn intrinsic(&self, class: &Class, method: &Arc<Method>) {
        self.update(class, method, |r| r.data.intrinsic_entries += 1);
    }

    #[cfg(not(target_family = "wasm"))]
    pub(crate) fn queued(&self, class: &Class, method: &Arc<Method>) {
        self.update(class, method, |r| {
            r.data.queued_requests += 1;
            r.data.compilation_state = "queued";
        });
    }

    pub(crate) fn rejected(&self, class: &Class, method: &Arc<Method>) {
        self.update(class, method, |r| r.data.compilation_state = "rejected");
    }

    pub(crate) fn starting(&self, class: &Class, method: &Arc<Method>) {
        self.update(class, method, |r| {
            r.data.compilation_attempts += 1;
            r.data.compilation_state = "compiling";
        });
    }

    #[cfg(feature = "experimental-reference-dispatch")]
    pub(crate) fn starting_experimental(&self, class: &Class, method: &Arc<Method>) {
        self.starting(class, method);
        self.update(class, method, |r| r.data.experimental_preparation = true);
    }

    pub(crate) fn finished(
        &self,
        class: &Class,
        method: &Arc<Method>,
        result: Result<&Arc<Function>, String>,
        duration: Duration,
    ) {
        self.update(class, method, |r| {
            r.data.compilation_ns += duration.as_nanos();
            match result {
                Ok(function) => {
                    r.data.compilation_successes += 1;
                    r.data.compilation_state = "compiled";
                    #[cfg(all(
                        feature = "experimental-execution-measurement",
                        target_os = "linux",
                        target_has_atomic = "64"
                    ))]
                    if !self
                        .measurement
                        .admit_function(r.functions.len(), function.code_bytes())
                    {
                        return;
                    }
                    r.functions.push(function.clone());
                }
                Err(error) => {
                    r.data.compilation_failures += 1;
                    r.data.compilation_state = "failed";
                    #[cfg(all(
                        feature = "experimental-execution-measurement",
                        target_os = "linux",
                        target_has_atomic = "64"
                    ))]
                    if !self.measurement.admit_error(error.len()) {
                        return;
                    }
                    r.data.last_error = Some(error);
                }
            }
        });
    }

    #[cfg(all(
        feature = "experimental-execution-measurement",
        target_os = "linux",
        target_has_atomic = "64"
    ))]
    pub(crate) fn arm_measurement(
        &self,
        limits: crate::execution_measurement::Limits,
    ) -> crate::Result<()> {
        let records = self.records.lock();
        self.measurement.arm(limits)?;
        for record in records.values() {
            self.measurement.admit_method(
                records.len(),
                true,
                &record.data.class,
                &record.data.name,
                &record.data.descriptor,
            );
            if record.functions.len() > limits.functions_per_method {
                self.measurement.lose();
            }
            for function in &record.functions {
                self.measurement.retain_code(function.code_bytes());
            }
            if let Some(error) = &record.data.last_error {
                self.measurement.admit_error(error.len());
            }
        }
        if records.len() > limits.methods || self.measurement.losses() != 0 {
            return Err(crate::Error::InternalError(
                "existing execution profile exceeds measurement limits".to_owned(),
            ));
        }
        Ok(())
    }

    pub(crate) fn snapshot(&self) -> ExecutionProfile {
        let records = self.records.lock();
        let mut methods: Vec<_> = records
            .values()
            .map(|r| MethodExecutionProfile {
                instance: r.data.instance,
                class: r.data.class.clone(),
                name: r.data.name.clone(),
                descriptor: r.data.descriptor.clone(),
                rejection_reason: r.data.rejection_reason,
                rejected_bytecode: r.data.rejected_bytecode,
                experimental_preparation: r.data.experimental_preparation,
                compilation_state: r.data.compilation_state,
                queued_requests: r.data.queued_requests,
                compilation_attempts: r.data.compilation_attempts,
                compilation_successes: r.data.compilation_successes,
                compilation_failures: r.data.compilation_failures,
                compilation_ns: r.data.compilation_ns,
                last_error: r.data.last_error.clone(),
                interpreter_entries: r.data.interpreter_entries,
                intrinsic_entries: r.data.intrinsic_entries,
                compiled_entries: r.functions.iter().map(|f| f.execution_count()).sum(),
                compiled_backedges: r.functions.iter().map(|f| f.backedge_count()).sum(),
                compiled_resumes: r.functions.iter().map(|f| f.resume_count()).sum(),
                generated_code_bytes: r.functions.iter().map(|f| f.code_bytes()).sum(),
            })
            .collect();
        methods.sort_by(|a, b| {
            (&a.class, &a.name, &a.descriptor, a.instance).cmp(&(
                &b.class,
                &b.name,
                &b.descriptor,
                b.instance,
            ))
        });
        ExecutionProfile {
            schema_version: 1,
            diagnostic_only: true,
            qualification: false,
            methods,
            limitations: [
                "lifetime totals include startup, warmup and reporting",
                "taken backward branches are not loop iterations, bytecodes, CPU time or inlining",
                "snapshot does not wait for queued compilation or concurrent calls",
                "recording retains metadata and adds locks/atomics; use separate timing builds",
            ],
        }
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use crate::{Result, Value};
    use ristretto_classfile::attributes::{Attribute, Instruction};
    use ristretto_classfile::{ClassFile, ConstantPool, MethodAccessFlags};

    fn fixture(code: Vec<Instruction>, descriptor: &str) -> Result<Arc<Class>> {
        let mut pool = ConstantPool::default();
        let this_class = pool.add_class("ExecutionProfileFixture")?;
        let name_index = pool.add_utf8("run")?;
        let descriptor_index = pool.add_utf8(descriptor)?;
        let code_name = pool.add_utf8("Code")?;
        Ok(Class::from(
            None,
            ClassFile {
                constant_pool: pool,
                this_class,
                methods: vec![ristretto_classfile::Method {
                    access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                    name_index,
                    descriptor_index,
                    attributes: vec![Attribute::Code {
                        name_index: code_name,
                        max_stack: 1,
                        max_locals: 0,
                        code,
                        exception_table: vec![],
                        attributes: vec![],
                    }],
                }],
                ..Default::default()
            },
        )?)
    }

    #[cfg(all(
        feature = "experimental-execution-measurement",
        target_os = "linux",
        target_has_atomic = "64"
    ))]
    mod measurement_tests {
        use super::*;
        use crate::execution_measurement::Limits;
        fn stamp() -> Result<u64> {
            ristretto_gc::clock::monotonic_time_ns()
                .ok_or_else(|| crate::Error::InternalError("clock unavailable".to_owned()))
        }
        #[test]
        fn explicit_window_excludes_warmup_reporting_and_rejects_reuse() -> Result<()> {
            let class = fixture(vec![Instruction::Return], "()V")?;
            let method = class.try_get_method("run", "()V")?;
            let recorder = Arc::new(Recorder::default());
            for _ in 0..9 {
                recorder.interpreted(&class, &method);
            }
            recorder.arm_measurement(Limits::default())?;
            recorder.measurement_boundary(1, true, stamp)?;
            for _ in 0..7 {
                recorder.interpreted(&class, &method);
            }
            recorder.measurement_boundary(1, false, stamp)?;
            for _ in 0..11 {
                recorder.interpreted(&class, &method);
            }
            let capture = recorder.measurement.take();
            assert_eq!(capture.state, "closed");
            assert_eq!(capture.protocol_errors, 0);
            let interval = capture.interval.expect("completed interval");
            assert_eq!(interval.methods.len(), 1);
            assert_eq!(
                interval
                    .methods
                    .first()
                    .ok_or_else(|| crate::Error::InternalError(
                        "missing fixture method".to_owned()
                    ))?
                    .activity
                    .interpreter_entries,
                7
            );
            let repeated = recorder.measurement.take();
            assert_eq!(repeated.state, "taken");
            assert_eq!(repeated.protocol_errors, 1);
            Ok(())
        }
        #[test]
        fn duplicates_foreign_actors_missing_end_and_reversed_clocks_are_explicit() -> Result<()> {
            for mode in 0..4 {
                let recorder = Arc::new(Recorder::default());
                recorder.arm_measurement(Limits::default())?;
                if mode == 0 {
                    assert!(recorder.measurement_boundary(1, false, stamp).is_err());
                } else {
                    recorder.measurement_boundary(1, true, stamp)?;
                    match mode {
                        1 => assert!(recorder.measurement_boundary(1, true, stamp).is_err()),
                        2 => assert!(recorder.measurement_boundary(2, false, stamp).is_err()),
                        _ => assert!(recorder.measurement_boundary(1, false, || Ok(0)).is_err()),
                    }
                }
                let capture = recorder.measurement.take();
                assert_eq!(capture.protocol_errors, 1);
                assert_ne!(capture.state, "closed");
            }
            let recorder = Arc::new(Recorder::default());
            recorder.arm_measurement(Limits::default())?;
            recorder.measurement_boundary(1, true, stamp)?;
            assert_eq!(recorder.measurement.take().state, "open");
            Ok(())
        }
        #[test]
        fn method_limits_and_counter_regression_cannot_look_complete() -> Result<()> {
            let first = fixture(vec![Instruction::Return], "()V")?;
            let second = fixture(vec![Instruction::Return], "()V")?;
            let first_method = first.try_get_method("run", "()V")?;
            let second_method = second.try_get_method("run", "()V")?;
            let recorder = Arc::new(Recorder::default());
            recorder.arm_measurement(Limits {
                methods: 1,
                ..Limits::default()
            })?;
            recorder.interpreted(&first, &first_method);
            recorder.measurement_boundary(1, true, stamp)?;
            recorder.interpreted(&second, &second_method);
            recorder.measurement_boundary(1, false, stamp)?;
            let capture = recorder.measurement.take();
            assert_eq!(capture.evidence_losses, 1);
            assert_eq!(recorder.snapshot().methods.len(), 1);
            let recorder = Arc::new(Recorder::default());
            recorder.interpreted(&first, &first_method);
            recorder.arm_measurement(Limits::default())?;
            recorder.measurement_boundary(1, true, stamp)?;
            recorder
                .records
                .lock()
                .values_mut()
                .next()
                .expect("record")
                .data
                .interpreter_entries = 0;
            assert!(recorder.measurement_boundary(1, false, stamp).is_err());
            assert_eq!(recorder.measurement.take().state, "failed");
            Ok(())
        }
    }

    #[tokio::test]
    async fn compilation_is_not_execution_and_failure_is_not_pending() -> Result<()> {
        let class = fixture(vec![Instruction::Bipush(42), Instruction::Ireturn], "()I")?;
        let method = class.try_get_method("run", "()I")?;
        let compiler = crate::jit::Compiler::new(false, false).expect("native compiler");
        let function = compiler.compile(&class, &method).await?.expect("compiled");
        let before = compiler.execution_recorder.snapshot();
        let record = before.methods.first().expect("record");
        assert_eq!(record.compilation_state, "compiled");
        assert_eq!(record.compilation_successes, 1);
        assert_eq!(record.compiled_entries, 0);
        assert!(record.generated_code_bytes > 0);
        for _ in 0..3 {
            assert_eq!(
                function.execute(&[], std::ptr::null())?,
                Some(ristretto_jit::Value::I32(42))
            );
        }
        assert_eq!(
            compiler
                .execution_recorder
                .snapshot()
                .methods
                .first()
                .expect("record")
                .compiled_entries,
            3
        );

        let invalid = fixture(vec![Instruction::Ireturn], "()I")?;
        let invalid_method = invalid.try_get_method("run", "()I")?;
        assert!(compiler.compile(&invalid, &invalid_method).await?.is_none());
        let snapshot = compiler.execution_recorder.snapshot();
        assert_eq!(
            snapshot.methods.len(),
            2,
            "same names must not merge method instances"
        );
        let failed = snapshot
            .methods
            .iter()
            .find(|r| r.compilation_state == "failed")
            .expect("failed record");
        assert_eq!(failed.compilation_attempts, 1);
        assert_eq!(failed.compilation_failures, 1);
        assert_eq!(failed.compiled_entries, 0);
        assert!(failed.last_error.is_some());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn queued_methods_finish_without_being_counted_as_executed() -> Result<()> {
        let class = fixture(vec![Instruction::Bipush(42), Instruction::Ireturn], "()I")?;
        let method = class.try_get_method("run", "()I")?;
        let compiler = crate::jit::Compiler::new(true, false).expect("native compiler");
        assert!(compiler.compile(&class, &method).await?.is_none());
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                if compiler.compile(&class, &method).await?.is_some() {
                    return Ok::<_, crate::Error>(());
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("compilation completes")?;
        let snapshot = compiler.execution_recorder.snapshot();
        let record = snapshot.methods.first().expect("record");
        assert_eq!(record.queued_requests, 1);
        assert_eq!(record.compilation_successes, 1);
        assert_eq!(record.compilation_state, "compiled");
        assert_eq!(record.compiled_entries, 0);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn live_vm_records_compiled_and_interpreted_entries() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        assert!(vm.is_jit_enabled());
        let compiled = fixture(vec![Instruction::Bipush(42), Instruction::Ireturn], "()I")?;
        let method = compiled.try_get_method("run", "()I")?;
        for _ in 0..3 {
            assert_eq!(
                thread.execute(&compiled, &method, &[] as &[Value]).await?,
                Some(Value::Int(42))
            );
        }
        let fallback = fixture(
            vec![
                Instruction::Bipush(7),
                Instruction::Pop,
                Instruction::Aconst_null,
                Instruction::Areturn,
            ],
            "()Ljava/lang/Object;",
        )?;
        let method = fallback.try_get_method("run", "()Ljava/lang/Object;")?;
        assert_eq!(
            thread.execute(&fallback, &method, &[] as &[Value]).await?,
            Some(Value::Object(None))
        );
        let report = vm.execution_profile();
        let methods: Vec<_> = report
            .methods
            .iter()
            .filter(|r| r.class == "ExecutionProfileFixture")
            .collect();
        assert_eq!(methods.len(), 2);
        let compiled = methods
            .iter()
            .find(|r| r.descriptor == "()I")
            .expect("compiled record");
        assert_eq!(compiled.compiled_entries, 3);
        assert_eq!(compiled.interpreter_entries, 0);
        let fallback = methods
            .iter()
            .find(|r| r.descriptor != "()I")
            .expect("fallback record");
        assert_eq!(fallback.compiled_entries, 0);
        assert_eq!(fallback.interpreter_entries, 1);
        assert_eq!(fallback.rejection_reason, Some("unqualified_instruction"));
        assert_eq!(
            fallback.rejected_bytecode,
            Some((3, Instruction::Aconst_null.code()))
        );
        assert_eq!(compiled.rejected_bytecode, None);
        assert!(report.methods.iter().any(|r| r.intrinsic_entries > 0));
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn intervals_exclude_prior_calls_and_preserve_start_compilation_state() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let class = fixture(vec![Instruction::Bipush(42), Instruction::Ireturn], "()I")?;
        let method = class.try_get_method("run", "()I")?;
        // Compile and execute before the measured counter interval.
        thread.execute(&class, &method, &[] as &[Value]).await?;
        let checkpoint = vm.execution_profile_checkpoint("after warmup");
        for _ in 0..3 {
            thread.execute(&class, &method, &[] as &[Value]).await?;
        }
        // A method first compiled during the interval must not appear ready at its start.
        let late = fixture(vec![Instruction::Bipush(7), Instruction::Ireturn], "()I")?;
        let late_method = late.try_get_method("run", "()I")?;
        thread.execute(&late, &late_method, &[] as &[Value]).await?;
        let report = vm.execution_profile_since(checkpoint)?;
        let methods: Vec<_> = report
            .methods
            .iter()
            .filter(|m| m.activity.class == "ExecutionProfileFixture")
            .collect();
        assert_eq!(methods.len(), 2);
        let warm = methods
            .iter()
            .find(|m| m.present_at_start)
            .expect("warm method");
        assert!(warm.compiled_before_start);
        assert_eq!(warm.activity.compiled_entries, 3);
        assert_eq!(warm.activity.compilation_successes, 0);
        assert_eq!(warm.activity.generated_code_bytes, 0);
        assert!(warm.code_bytes_at_start > 0);
        assert_eq!(warm.code_bytes_at_start, warm.code_bytes_at_end);
        let late = methods
            .iter()
            .find(|m| !m.present_at_start)
            .expect("late method");
        assert!(!late.compiled_before_start);
        assert_eq!(late.activity.compiled_entries, 1);
        assert_eq!(late.activity.compilation_successes, 1);
        assert_eq!(late.code_bytes_at_start, 0);
        assert!(report.diagnostic_only);
        assert!(!report.qualification);
        assert_eq!(report.scope, "checkpoint_interval");
        assert_eq!(report.label, "after warmup");
        assert!(report.boundary_span_ns >= report.start_snapshot_ns);
        assert!(report.boundary_span_ns >= report.end_snapshot_ns);
        let lifetime = vm.execution_profile();
        assert_eq!(
            lifetime
                .methods
                .iter()
                .find(|m| m.instance == warm.activity.instance)
                .expect("lifetime record")
                .compiled_entries,
            4
        );
        Ok(())
    }
}

#[cfg(test)]
mod portable_tests {
    use super::*;

    #[test]
    fn checkpoints_reject_foreign_recorders_and_do_not_keep_them_alive() {
        let recorder = Arc::new(Recorder::default());
        let other = Arc::new(Recorder::default());
        let checkpoint = recorder.checkpoint("foreign".to_owned());
        assert!(other.since(checkpoint).is_err());
        let checkpoint = recorder.checkpoint("abandoned".to_owned());
        let weak = Arc::downgrade(&recorder);
        drop(recorder);
        assert!(weak.upgrade().is_none());
        drop(checkpoint);
    }

    #[test]
    fn independent_intervals_have_explicit_clock_availability() -> crate::Result<()> {
        let recorder = Arc::new(Recorder::default());
        let outer = recorder.checkpoint("outer".to_owned());
        let inner = recorder.checkpoint("inner".to_owned());
        let inner = recorder.since(inner)?;
        let outer = recorder.since(outer)?;
        assert_eq!(inner.label, "inner");
        assert_eq!(outer.label, "outer");
        assert!(inner.methods.is_empty());
        assert!(!inner.qualification);
        #[cfg(not(target_family = "wasm"))]
        {
            assert!(inner.boundary_span_ns.is_some());
            assert!(outer.boundary_span_ns >= inner.boundary_span_ns);
        }
        #[cfg(target_family = "wasm")]
        {
            assert!(inner.boundary_span_ns.is_none());
            assert!(inner.start_snapshot_ns.is_none());
            assert!(inner.end_snapshot_ns.is_none());
        }
        Ok(())
    }
}
