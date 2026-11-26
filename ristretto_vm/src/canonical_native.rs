//! Application-declared scalar native calls into a verified canonical program.
use super::{Error, Limits, Program};
use crate::intrinsic_methods::MethodRegistry;
use crate::thread::MonitorGuard;
use crate::{JavaError, Thread};
use ristretto_classfile::JavaStr;
use ristretto_classloader::{Class, ClassLoaderType, Method, Value};
use ristretto_gc::GarbageCollector;
use ristretto_gc::canonical::Configuration;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct Binding {
    class: Arc<Class>,
    method: Arc<Method>,
    program: Program,
    target: String,
    entry: String,
}

impl Binding {
    pub(crate) async fn prepare(
        thread: &Thread,
        intrinsics: &MethodRegistry,
        bridge_class: &str,
        bridge_method: &str,
        target_class: &str,
        target_method: &str,
        companions: &[String],
    ) -> crate::Result<Self> {
        let class = application_class(thread, bridge_class).await?;
        let method = class.try_get_method(bridge_method, "(I)I")?;
        if !method.is_static()
            || !method.is_native()
            || intrinsics
                .method(class.name(), bridge_method, "(I)I")
                .is_some()
        {
            return Err(crate::Error::ConfigurationError(format!(
                "{}.{bridge_method} must declare an unbound static native (I)I method",
                class.name()
            )));
        }

        let mut files = Vec::with_capacity(1 + companions.len());
        let target = target_class.replace('.', "/");
        for name in std::iter::once(target.as_str()).chain(companions.iter().map(String::as_str)) {
            let application_class = application_class(thread, name).await?;
            files.push(application_class.class_file().clone());
        }
        let configuration_error =
            |error: Error| crate::Error::ConfigurationError(error.to_string());
        let program = Program::from_classfiles(files).map_err(configuration_error)?;
        program
            .entry_method(&target, target_method)
            .map_err(configuration_error)?;
        if program.needs_initialization() {
            return Err(crate::Error::ConfigurationError(
                "a per-call canonical native target cannot own static fields or class initializers"
                    .into(),
            ));
        }
        Ok(Self {
            class,
            method,
            program,
            target,
            entry: target_method.to_owned(),
        })
    }

    pub(crate) fn matches(&self, class: &Arc<Class>, method: &Arc<Method>) -> bool {
        Arc::ptr_eq(&self.class, class) && Arc::ptr_eq(&self.method, method)
    }

    async fn run(&self, collector: &GarbageCollector, argument: i32) -> Result<i32, Error> {
        let configuration = Configuration {
            objects: 128,
            roots: 128,
            helpers: 1,
            mutators: 1,
            edges_per_object: 8,
        };
        let limits = Limits::default();
        let completion = self
            .program
            .with_collector(
                collector,
                configuration,
                limits.cleanup_polls,
                async |heap| {
                    let mut thread = heap.attach(limits)?;
                    Ok(thread
                        .invoke(&self.target, &self.entry, argument)
                        .await?
                        .result)
                },
            )
            .await?;
        Ok(completion.value)
    }

    #[cfg(not(target_family = "wasm"))]
    pub(crate) async fn invoke(
        self: &Arc<Self>,
        collector: &Arc<GarbageCollector>,
        argument: i32,
        monitor: Option<MonitorGuard>,
    ) -> Result<i32, Error> {
        let binding = self.clone();
        let collector = collector.clone();
        run_on_worker(monitor, move || async move {
            binding.run(&collector, argument).await
        })
        .await
    }

    #[cfg(target_family = "wasm")]
    pub(crate) async fn invoke(
        &self,
        collector: &Arc<GarbageCollector>,
        argument: i32,
        _monitor: Option<MonitorGuard>,
    ) -> Result<i32, Error> {
        self.run(collector, argument).await
    }
}

/// Keep the non-Send canonical frame on the blocking worker that creates it.
#[cfg(not(target_family = "wasm"))]
async fn run_on_worker<F: Future<Output = Result<i32, Error>>>(
    monitor: Option<MonitorGuard>,
    run: impl FnOnce() -> F + Send + 'static,
) -> Result<i32, Error> {
    let runtime = tokio::runtime::Handle::current();
    let (cancel, cancelled) = tokio::sync::oneshot::channel::<()>();
    // A queued task must release a cancelled caller's monitor immediately.
    // Once started, the worker holds it through the drop of the execution future.
    let monitor = monitor.map(Arc::new);
    let queued_monitor = monitor.as_ref().map(Arc::downgrade);
    let worker = tokio::task::spawn_blocking(move || {
        let _running_monitor = match queued_monitor {
            Some(weak) => Some(
                weak.upgrade()
                    .ok_or(Error::Boundary("canonical native call cancelled"))?,
            ),
            None => None,
        };
        runtime.block_on(async {
            tokio::select! {
                biased;
                _ = cancelled => Err(Error::Boundary("canonical native call cancelled")),
                result = async { run().await } => result,
            }
        })
    });
    let result = worker
        .await
        .map_err(|_| Error::Boundary("canonical native worker failed"))?;
    drop(cancel);
    drop(monitor);
    result
}

async fn application_class(thread: &Thread, name: &str) -> crate::Result<Arc<Class>> {
    let name = name.replace('.', "/");
    let class = thread
        .load_and_link_class(&JavaStr::cow_from_str(&name))
        .await?;
    if class
        .class_loader()?
        .and_then(|loader| loader.loader_type())
        != Some(ClassLoaderType::System)
    {
        return Err(crate::Error::ConfigurationError(format!(
            "canonical native binding requires an application class: {name}"
        )));
    }
    Ok(class)
}

pub(crate) async fn java_error(thread: &Thread, error: Error) -> crate::Error {
    let exception = match &error {
        Error::Invalid(kind) if kind == "NullPointerException" => "java/lang/NullPointerException",
        Error::TypeFault("ClassCastException") => "java/lang/ClassCastException",
        Error::ArrayFault("NegativeArraySizeException") => "java/lang/NegativeArraySizeException",
        Error::ArrayFault("ArrayIndexOutOfBoundsException") => {
            "java/lang/ArrayIndexOutOfBoundsException"
        }
        Error::ArrayFault("ArrayStoreException") => "java/lang/ArrayStoreException",
        Error::Heap(ristretto_gc::Error::OutOfMemory)
        | Error::Collector(ristretto_gc::canonical::Error::Heap(
            ristretto_gc::Error::OutOfMemory,
        )) => return ristretto_gc::Error::OutOfMemory.into(),
        _ => return JavaError::RuntimeException(error.to_string()).into(),
    };
    // These faults carry a class but do not always retain the failing index or
    // source type. Use the real no-argument Java constructor in the caller's VM.
    match Box::pin(thread.object(exception, "", &[] as &[Value])).await {
        Ok(object) => crate::Error::Throwable(object),
        Err(error) => error,
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use ristretto_classfile::ClassFile;
    use std::future::Future;
    use std::task::{Context, Waker};
    use std::time::Duration;

    fn binding() -> Result<Arc<Binding>, Error> {
        let file = ClassFile::from_bytes(include_bytes!(
            "../tests/fixtures/canonical/CanonicalGraphWorkload.class"
        ))?;
        let class = Class::from(None, file.clone())?;
        let method = class.try_get_method("run", "(I)I")?;
        let program = Program::from_classfiles([
            file,
            ClassFile::from_bytes(include_bytes!(
                "../tests/fixtures/canonical/CanonicalGraphWorkload$Node.class"
            ))?,
        ])?;
        Ok(Arc::new(Binding {
            class,
            method,
            program,
            target: "CanonicalGraphWorkload".into(),
            entry: "run".into(),
        }))
    }

    #[tokio::test]
    async fn running_native_retains_its_java_monitor_until_worker_exit()
    -> Result<(), Box<dyn std::error::Error>> {
        let monitor = Arc::new(ristretto_types::monitor::Monitor::new());
        monitor.acquire(1).await?;
        let guard = MonitorGuard::held(
            monitor.clone(),
            1,
            ristretto_classloader::Value::Object(None),
        );
        let (entered, started) = tokio::sync::oneshot::channel();
        let (release, parked) = std::sync::mpsc::channel::<()>();
        let mut call = Box::pin(run_on_worker(Some(guard), move || async move {
            let _ = entered.send(());
            // Hold a synchronous worker instruction across cancellation.
            let _ = parked.recv_timeout(Duration::from_secs(10));
            Ok(1)
        }));
        assert!(
            call.as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        tokio::time::timeout(Duration::from_secs(10), started).await??;
        drop(call);
        let mut competitor = Box::pin(monitor.acquire(2));
        let blocked = competitor
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending();
        release.send(())?;
        assert!(
            blocked,
            "another Java thread entered while the worker was running"
        );
        tokio::time::timeout(Duration::from_secs(10), competitor).await??;
        assert!(monitor.release(2)?);
        Ok(())
    }

    #[test]
    fn queued_cancelled_native_releases_its_monitor_before_the_pool_resumes()
    -> Result<(), Box<dyn std::error::Error>> {
        tokio::runtime::Builder::new_current_thread()
            .max_blocking_threads(1)
            .enable_time()
            .build()?
            .block_on(async {
                let (release, parked) = std::sync::mpsc::channel::<()>();
                let (occupied, started) = tokio::sync::oneshot::channel();
                let blocker = tokio::task::spawn_blocking(move || {
                    let _ = occupied.send(());
                    let _ = parked.recv_timeout(Duration::from_secs(10));
                });
                tokio::time::timeout(Duration::from_secs(10), started).await??;
                let monitor = Arc::new(ristretto_types::monitor::Monitor::new());
                monitor.acquire(1).await?;
                let guard = MonitorGuard::held(
                    monitor.clone(),
                    1,
                    ristretto_classloader::Value::Object(None),
                );
                let (ran, observed) = tokio::sync::oneshot::channel();
                let mut call = Box::pin(run_on_worker(Some(guard), move || async move {
                    let _ = ran.send(());
                    Ok(1)
                }));
                assert!(
                    call.as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
                drop(call);
                let released = !monitor.is_owned_by(1);
                release.send(())?;
                blocker.await?;
                assert!(
                    released,
                    "a queued call kept the Java monitor after cancellation"
                );
                assert!(
                    tokio::time::timeout(Duration::from_secs(10), observed)
                        .await?
                        .is_err()
                );
                Ok(())
            })
    }

    #[tokio::test]
    async fn cancelling_native_call_after_allocation_refunds_its_worker_payloads()
    -> Result<(), Box<dyn std::error::Error>> {
        let binding = binding()?;
        let collector = GarbageCollector::new();
        let ordinary = Value::try_new_object(&collector, vec![31_i32].into())?;
        let baseline = collector.managed_bytes();
        let mut call = Box::pin(binding.invoke(&collector, i32::MAX, None));
        assert!(
            call.as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        tokio::time::timeout(Duration::from_secs(10), async {
            while collector.managed_bytes() == baseline {
                tokio::task::yield_now().await;
            }
        })
        .await?;
        drop(call);
        // The worker owns a Binding until the local execution future has dropped.
        // This also distinguishes temporary graph reclamation from final teardown.
        tokio::time::timeout(Duration::from_secs(10), async {
            while Arc::strong_count(&binding) != 1 {
                tokio::task::yield_now().await;
            }
        })
        .await?;
        assert_eq!(collector.managed_bytes(), baseline);
        assert_eq!(&*ordinary.as_int_vec_ref()?, &[31]);
        assert_eq!(binding.invoke(&collector, 3, None).await?, 9);
        assert_eq!(collector.managed_bytes(), baseline);
        Ok(())
    }
}
