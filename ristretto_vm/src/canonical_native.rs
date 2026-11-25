//! Application-declared scalar native calls into a verified canonical program.
use super::{Error, Limits, Program};
use crate::intrinsic_methods::MethodRegistry;
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
    ) -> Result<i32, Error> {
        // Canonical frame owners are task-local. The regular VM's Send future
        // owns only the JoinHandle and the cancellation sender across its await.
        let binding = self.clone();
        let collector = collector.clone();
        let runtime = tokio::runtime::Handle::current();
        let (cancel, cancelled) = tokio::sync::oneshot::channel::<()>();
        let worker = tokio::task::spawn_blocking(move || {
            runtime.block_on(async {
                tokio::select! {
                    biased;
                    _ = cancelled => Err(Error::Boundary("canonical native call cancelled")),
                    result = binding.run(&collector, argument) => result,
                }
            })
        });
        let result = worker
            .await
            .map_err(|_| Error::Boundary("canonical native worker failed"))?;
        drop(cancel);
        result
    }

    #[cfg(target_family = "wasm")]
    pub(crate) async fn invoke(
        &self,
        collector: &Arc<GarbageCollector>,
        argument: i32,
    ) -> Result<i32, Error> {
        self.run(collector, argument).await
    }
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
        )) => return JavaError::OutOfMemoryError("Java heap space".into()).into(),
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
    async fn cancelling_native_call_after_allocation_refunds_its_worker_payloads()
    -> Result<(), Box<dyn std::error::Error>> {
        let binding = binding()?;
        let collector = GarbageCollector::new();
        let ordinary = Value::new_object(&collector, vec![31_i32].into());
        let baseline = collector.managed_bytes();
        let mut call = Box::pin(binding.invoke(&collector, i32::MAX));
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
        assert_eq!(binding.invoke(&collector, 3).await?, 9);
        assert_eq!(collector.managed_bytes(), baseline);
        Ok(())
    }
}
