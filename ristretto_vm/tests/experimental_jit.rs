//! Shared javac workload through the ordinary dispatcher, without cache injection.
#![cfg(all(
    not(target_family = "wasm"),
    feature = "experimental-reference-dispatch",
    feature = "execution-profile"
))]
#![expect(
    clippy::panic_in_result_fn,
    reason = "test assertions report contract failures"
)]

mod common;

use ristretto_gc::{GarbageCollector, GcWeak, ReferenceKind, RootStorage};
use ristretto_vm::{ClassPath, ConfigurationBuilder, Error, Reference, Thread, VM, Value};
use std::future::Future;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use std::time::Duration;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;
const GRAPH: &str = "CanonicalGraphWorkload";
const NODE: &str = "CanonicalGraphWorkload$Node";
const LIFECYCLE: &str = "ExperimentalJitLifecycle";
const CHECKSUM: &str = "(LCanonicalGraphWorkload$Node;LCanonicalGraphWorkload$Node;)I";

fn fixtures(home: &Path, output: &Path) -> TestResult {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut javac = common::java::command(home, "javac");
    javac
        .args(["--release", "17", "-g", "-d"])
        .arg(output)
        .arg(root.join("canonical/CanonicalGraphWorkload.java"))
        .arg(root.join("jit/ExperimentalJitLifecycle.java"))
        .arg(root.join("jit/ExperimentalJitDriver.java"));
    common::process::run(&mut javac, Duration::from_secs(30))?.require_success()?;
    // Both execution tiers consume exactly the checked-in shared bytecode.
    for name in [
        "CanonicalGraphWorkload.class",
        "CanonicalGraphWorkload$Node.class",
    ] {
        assert_eq!(
            std::fs::read(output.join(name))?,
            std::fs::read(root.join("canonical").join(name))?
        );
    }
    for rounds in [-1, 0, 1, 8, 17] {
        let mut java = common::java::command(home, "java");
        java.args(["-Xverify:all", "-cp"])
            .arg(output)
            .args(["ExperimentalJitDriver", &rounds.to_string()]);
        let result = common::process::run(&mut java, Duration::from_secs(30))?;
        result.require_success()?;
        let expected = if rounds < 0 { 0 } else { rounds * rounds };
        assert_eq!(result.stdout.trim(), format!("{expected}\n19"));
    }
    Ok(())
}

async fn vm(
    home: &Path,
    classes: &Path,
    concurrent: bool,
    interpreted: bool,
) -> TestResult<Arc<VM>> {
    let gc = GarbageCollector::with_config(
        ristretto_gc::Configuration::builder()
            .threads(0)
            .root_storage(RootStorage::Contexts)
            .concurrent_publication(concurrent)
            .max_heap_bytes(32 * 1024 * 1024)
            .build(),
    );
    let vm = VM::new(
        ConfigurationBuilder::new()
            .java_home(home.to_owned())
            .class_path(ClassPath::from(&[classes]))
            .garbage_collector(gc)
            .batch_compilation(false)
            .interpreted(interpreted)
            .build()?,
    )
    .await?;
    vm.garbage_collector().stop()?;
    Ok(vm)
}

async fn prepare(vm: &VM, callees: bool) -> TestResult {
    vm.prepare_experimental_jit(GRAPH, "run", "(I)I").await?;
    if callees {
        vm.prepare_experimental_jit(NODE, "<init>", "(I)V").await?;
        vm.prepare_experimental_jit(GRAPH, "checksum", CHECKSUM)
            .await?;
    }
    Ok(())
}

async fn graph(vm: &Arc<VM>, compiled: bool, callees: bool, concurrent: bool) -> TestResult {
    if compiled {
        // Preparation must replace a previously cached production rejection.
        assert_eq!(vm.try_invoke(GRAPH, "run(I)I", &[0]).await?, Value::Int(0));
        prepare(vm, callees).await?;
    }
    let checkpoint = vm.execution_profile_checkpoint("shared Java graph invocation");
    for rounds in [-1, 0, 1, 8, 17] {
        let result = vm.try_invoke(GRAPH, "run(I)I", &[rounds]).await?.as_i32()?;
        assert_eq!(result, if rounds < 0 { 0 } else { rounds * rounds });
    }
    let class = vm.class(GRAPH).await?;
    let method = class.try_get_method("run", "(I)I")?;
    let thread = Thread::new(&Arc::downgrade(vm), 101);
    let parameters = [Value::Int(8)];
    let mut call = Box::pin(thread.execute(&class, &method, &parameters));
    let mut collections = 0;
    let result = std::future::poll_fn(|cx| match call.as_mut().poll(cx) {
        Poll::Ready(result) => Poll::Ready(result),
        Poll::Pending => {
            vm.garbage_collector().collect_full()?;
            vm.garbage_collector().collect_full()?;
            collections += 1;
            Poll::Pending
        }
    })
    .await?;
    assert_eq!(result, Some(Value::Int(64)));
    assert!(thread.frames()?.is_empty());
    let interval = vm.execution_profile_since(checkpoint)?;
    let run = interval
        .methods
        .iter()
        .find(|m| m.activity.class == GRAPH && m.activity.name == "run")
        .ok_or("missing run profile")?;
    if compiled {
        assert!(
            collections > 0,
            "native execution never returned a collection boundary"
        );
        assert!(run.compiled_before_start);
        assert_eq!(run.activity.compiled_entries, 6);
        assert_eq!(run.activity.interpreter_entries, 0);
        assert_eq!(run.activity.compiled_backedges, 34);
        assert!(run.activity.experimental_preparation);
        assert_eq!(
            run.activity.rejection_reason,
            Some("unqualified_instruction")
        );
        for (class, name, expected) in [(GRAPH, "checksum", 34), (NODE, "<init>", 68)] {
            let method = interval
                .methods
                .iter()
                .find(|m| m.activity.class == class && m.activity.name == name)
                .ok_or("missing callee profile")?;
            assert_eq!(
                method.activity.compiled_entries,
                if callees { expected } else { 0 }
            );
            assert_eq!(
                method.activity.interpreter_entries,
                if callees { 0 } else { expected }
            );
        }
    } else {
        assert_eq!(run.activity.compiled_entries, 0);
        assert_eq!(run.activity.interpreter_entries, 6);
    }
    if let Some(directory) = std::env::var_os("RISTRETTO_JIT_EVIDENCE") {
        std::fs::create_dir_all(&directory)?;
        let name = format!("graph-compiled-{compiled}-callees-{callees}-{concurrent}.json");
        std::fs::write(
            PathBuf::from(directory).join(name),
            serde_json::to_vec_pretty(&interval)?,
        )?;
    }
    Ok(())
}

async fn lifetime(vm: &Arc<VM>, cancel: bool) -> TestResult {
    vm.prepare_experimental_jit(LIFECYCLE, "hold", "()I")
        .await?;
    let class = vm.class(LIFECYCLE).await?;
    class.set_static_value("release", Value::Int(0))?;
    let method = class.try_get_method("hold", "()I")?;
    let thread = Thread::new(&Arc::downgrade(vm), 102);
    let call_thread = thread.clone();
    let call_class = class.clone();
    let mut call = Box::pin(async move {
        call_thread
            .execute(&call_class, &method, &[] as &[Value])
            .await
    });
    let mut polls = 0;
    while class.static_value("observed")?.is_null() {
        assert!(
            call.as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        polls += 1;
        assert!(polls < 100, "no compiled reference publication");
    }
    let observed = class.static_value("observed")?;
    let Value::Object(Some(owner)) = &observed else {
        return Err("missing observed object".into());
    };
    let weak = GcWeak::new(owner, ReferenceKind::Weak, 0);
    class.set_static_value("observed", Value::Object(None))?;
    drop(observed);
    let garbage =
        Value::try_new_object(vm.garbage_collector(), Reference::IntArray(vec![99].into()))?;
    let Value::Object(Some(owner)) = &garbage else {
        return Err("missing garbage object".into());
    };
    let garbage_weak = GcWeak::new(owner, ReferenceKind::Weak, 0);
    drop(garbage);
    vm.garbage_collector().collect_full()?;
    vm.garbage_collector().collect_full()?;
    assert!(
        weak.upgrade().is_some(),
        "suspended compiled local lost its owner"
    );
    assert!(
        garbage_weak.upgrade().is_none(),
        "suspended frame prevented unrelated reclamation"
    );
    let handle = tokio::runtime::Handle::current();
    let mut call = std::thread::spawn(move || {
        let _entered = handle.enter();
        assert!(
            call.as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        call
    })
    .join()
    .map_err(|_| "migrating suspended call panicked")?;
    vm.garbage_collector().collect_full()?;
    assert!(
        weak.upgrade().is_some(),
        "migrated compiled local lost its owner"
    );
    if cancel {
        drop(call);
    } else {
        class.set_static_value("release", Value::Int(1))?;
        assert_eq!(call.as_mut().await?, Some(Value::Int(73)));
        drop(call);
    }
    assert!(thread.frames()?.is_empty());
    vm.garbage_collector().collect_full()?;
    vm.garbage_collector().collect_full()?;
    assert!(
        weak.upgrade().is_none(),
        "completed/cancelled frame retained its root"
    );
    Ok(())
}

async fn exceptions_and_rejections(vm: &Arc<VM>) -> TestResult {
    for method in ["nullRead", "propagate"] {
        vm.prepare_experimental_jit(LIFECYCLE, method, "()I")
            .await?;
    }
    for (method, descriptor, reason) in [
        ("catchNull", "()I", "experimental_exception_handlers"),
        (
            "synchronizedMethod",
            "()I",
            "experimental_synchronized_method",
        ),
        ("arrayRead", "([I)I", "experimental_unqualified_instruction"),
    ] {
        let error = vm
            .prepare_experimental_jit(LIFECYCLE, method, descriptor)
            .await
            .err()
            .ok_or("unqualified method was accepted")?;
        assert!(error.to_string().contains(reason), "{error}");
    }
    assert_eq!(
        vm.try_invoke(LIFECYCLE, "catchNull()I", &[] as &[Value])
            .await?,
        Value::Int(19)
    );
    let error = vm
        .try_invoke(LIFECYCLE, "propagate()I", &[] as &[Value])
        .await
        .err()
        .ok_or("null dereference unexpectedly returned")?;
    let Error::Throwable(value) = error else {
        return Err(format!("expected throwable: {error}").into());
    };
    assert_eq!(
        value.as_object_ref()?.class().name(),
        "java/lang/NullPointerException"
    );
    vm.garbage_collector().collect_full()?;
    assert_eq!(
        value.as_object_ref()?.class().name(),
        "java/lang/NullPointerException"
    );
    assert_eq!(vm.try_invoke(GRAPH, "run(I)I", &[1]).await?, Value::Int(1));
    Ok(())
}

#[test]
fn shared_java_graph_and_owning_experimental_dispatch() -> TestResult {
    if std::env::var_os("RISTRETTO_EXPERIMENTAL_JIT_CHILD").is_none() {
        let mut child = std::process::Command::new(std::env::current_exe()?);
        child
            .args([
                "--exact",
                "shared_java_graph_and_owning_experimental_dispatch",
                "--nocapture",
            ])
            .env("RISTRETTO_EXPERIMENTAL_JIT_CHILD", "1");
        let result = common::process::run(&mut child, Duration::from_secs(120))?;
        print!("{}", result.stdout);
        eprint!("{}", result.stderr);
        result.require_success()?;
        return Ok(());
    }
    let home = match std::env::var_os("RISTRETTO_JDK_HOME") {
        Some(home) => PathBuf::from(home),
        None => common::java::home(ristretto_classloader::JAVA_25_VERSION)?,
    };
    let directory = tempfile::tempdir()?;
    fixtures(&home, directory.path())?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?;
    runtime.block_on(async {
        for concurrent in [false, true] {
            let interpreted = vm(&home, directory.path(), concurrent, true).await?;
            assert!(
                interpreted
                    .prepare_experimental_jit(GRAPH, "run", "(I)I")
                    .await
                    .is_err()
            );
            graph(&interpreted, false, false, concurrent).await?;
            for callees in [false, true] {
                let compiled = vm(&home, directory.path(), concurrent, false).await?;
                graph(&compiled, true, callees, concurrent).await?;
                if callees {
                    lifetime(&compiled, false).await?;
                    lifetime(&compiled, true).await?;
                    exceptions_and_rejections(&compiled).await?;
                }
            }
        }
        TestResult::Ok(())
    })
}
