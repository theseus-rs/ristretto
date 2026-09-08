//! Browser playground engine. `JShell` requests share a VM until the session ends.
//!
//! The component receives JSON requests and reads a Java runtime mounted at /jdk.
//! stdout carries JSON events; Java output is encoded as bytes so split UTF-8 writes survive.

use ristretto_vm::{ClassPath, ClassPathEntry, Compiler, ConfigurationBuilder, Memory, VM};
use serde::Deserialize;
use serde_json::json;
use std::cell::RefCell;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use tokio::sync::Mutex;

const OUTPUT_LIMIT: usize = 1024 * 1024;

struct RuntimePaths {
    java_home: PathBuf,
    workspace: PathBuf,
    languages: PathBuf,
}

impl Default for RuntimePaths {
    fn default() -> Self {
        // Native process tests supply isolated mounts; the browser uses the WASI defaults.
        Self {
            java_home: std::env::var_os("RISTRETTO_PLAYGROUND_JAVA_HOME")
                .map_or_else(|| PathBuf::from("/jdk"), PathBuf::from),
            workspace: std::env::var_os("RISTRETTO_PLAYGROUND_WORKSPACE")
                .map_or_else(|| PathBuf::from("/workspace"), PathBuf::from),
            languages: std::env::var_os("RISTRETTO_PLAYGROUND_LANGUAGES")
                .map_or_else(|| PathBuf::from("/languages"), PathBuf::from),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Request {
    id: u32,
    action: Action,
    java_version: u16,
    class_name: String,
    source: String,
    #[serde(default)]
    language: Language,
    #[serde(default)]
    scala_version: Option<String>,
    #[serde(default)]
    operation: ShellOperation,
    #[serde(default)]
    cursor: i32,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ShellOperation {
    #[default]
    Input,
    Complete,
    Cancel,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Action {
    Compile,
    Check,
    Run,
    Jshell,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Language {
    #[default]
    Java,
    Kotlin,
    Groovy,
    Scala,
    Clojure,
}

fn emit(event: &serde_json::Value) -> io::Result<()> {
    let mut output = io::stdout().lock();
    serde_json::to_writer(&mut output, event)?;
    output.write_all(b"\n")?;
    output.flush()
}

#[derive(Debug)]
struct EventWriter {
    id: Arc<AtomicU32>,
    stream: &'static str,
    written: Arc<AtomicUsize>,
}

impl Write for EventWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let previous = self.written.fetch_add(bytes.len(), Ordering::Relaxed);
        if previous.saturating_add(bytes.len()) > OUTPUT_LIMIT {
            emit(
                &json!({"id": self.id.load(Ordering::Relaxed), "type": "error", "message": "Output exceeded 1 MiB; execution stopped."}),
            )?;
            std::process::exit(1);
        }
        if !bytes.is_empty() {
            emit(
                &json!({"id": self.id.load(Ordering::Relaxed), "type": "output", "stream": self.stream, "bytes": bytes}),
            )?;
        }
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()
    }
}

fn configuration(
    id: &Arc<AtomicU32>,
    written: &Arc<AtomicUsize>,
    paths: &RuntimePaths,
) -> ConfigurationBuilder {
    // Rust's WASI temp_dir implementation is unsupported; the browser mounts /tmp explicitly.
    #[cfg(target_family = "wasm")]
    let temporary_directory = "/tmp".to_string();
    #[cfg(not(target_family = "wasm"))]
    let temporary_directory = std::env::temp_dir().to_string_lossy().into_owned();
    let stream = |name| {
        Arc::new(Mutex::new(EventWriter {
            id: id.clone(),
            stream: name,
            written: written.clone(),
        }))
    };
    let builder = ConfigurationBuilder::new()
        .java_home(paths.java_home.clone())
        .interpreted(true)
        .stdin(Arc::new(Mutex::new(io::empty())))
        .stdout(stream("stdout"))
        .stderr(stream("stderr"))
        .add_system_property("user.dir", paths.workspace.to_string_lossy())
        .add_system_property("java.io.tmpdir", temporary_directory)
        .add_system_property("user.language", "en")
        .add_system_property("file.encoding", "UTF-8");
    // The bundled class libraries use Linux's POSIX filesystem provider on WASI.
    #[cfg(target_family = "wasm")]
    let builder = builder.add_system_property("os.name", "Linux");
    builder
}

async fn execute(
    request: &Request,
    paths: &RuntimePaths,
) -> Result<(), Box<dyn std::error::Error>> {
    if !matches!(request.java_version, 8 | 11 | 17 | 21 | 25) {
        return Err("Unsupported Java version".into());
    }
    let id = request.id;
    let written = Arc::new(AtomicUsize::new(0));
    if request.action == Action::Jshell {
        if request.language != Language::Java {
            return Err("JShell requires Java".into());
        }
        return execute_jshell(request, paths).await;
    }
    if request.language != Language::Java {
        return execute_script(request, paths).await;
    }
    if request.action == Action::Check {
        return Err("Use Compile to check Java source".into());
    }
    let output_id = Arc::new(AtomicU32::new(id));
    emit(&json!({"id": id, "type": "phase", "phase": "compiling"}))?;
    let compiler = Compiler::new(configuration(&output_id, &written, paths).build()?).await?;
    let classes = compiler
        .compile_source_with_options(&request.class_name, &request.source, &["-proc:none", "-g"])
        // Format VM errors while the compiler still owns the throwable's GC heap.
        .await
        .map_err(|error| error.to_string())?;
    emit(&json!({"id": id, "type": "compiled", "classes": classes.len()}))?;
    // Drop the compiler VM before starting the user's VM to reduce peak memory.
    drop(compiler);
    if request.action == Action::Run {
        emit(&json!({"id": id, "type": "phase", "phase": "running"}))?;
        let memory = Memory::new("playground");
        classes.load_into(&memory).await?;
        let config = configuration(&output_id, &written, paths)
            .main_class(&request.class_name)
            .class_path(ClassPath::new(vec![ClassPathEntry::Memory(memory)]))
            .build()?;
        let vm = VM::new(config).await?;
        if let Err(error) = vm.invoke_main(&[] as &[&str]).await {
            if let ristretto_vm::Error::Throwable(ref throwable) = error {
                // Calling the Java method preserves the exception's message and source locations.
                let _result = vm
                    .invoke(
                        "java.lang.Throwable",
                        "printStackTrace()V",
                        std::slice::from_ref(throwable),
                    )
                    .await;
            }
            // A throwable belongs to this VM's heap; preserve its message before dropping it.
            return Err(error.to_string().into());
        }
    }
    emit(&json!({"id": id, "type": "done"}))?;
    Ok(())
}

async fn execute_script(
    request: &Request,
    paths: &RuntimePaths,
) -> Result<(), Box<dyn std::error::Error>> {
    if request.java_version != 25 {
        return Err("Scripts require the bundled Java 25 runtime".into());
    }
    if !matches!(request.action, Action::Check | Action::Run) {
        return Err("Scripts support Check and Run".into());
    }
    let scala_version = request.scala_version.as_deref().unwrap_or("3");
    if request.language == Language::Scala && !matches!(scala_version, "2.13" | "3") {
        return Err("Unsupported Scala version".into());
    }
    let bridge = match request.language {
        Language::Kotlin => "KotlinScript",
        Language::Groovy => "GroovyScript",
        Language::Scala => "ScalaScript",
        Language::Clojure => "ClojureScript",
        Language::Java => return Err("Expected a script language".into()),
    };
    let mut jars = std::fs::read_dir(&paths.languages)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?;
    jars.retain(|path| {
        path.extension().is_some_and(|extension| extension == "jar")
            && path.file_name().is_none_or(|name| name != "jdk-api.jar")
    });
    jars.sort();
    if jars.is_empty() {
        return Err("Missing script runtime assets".into());
    }
    if request.language == Language::Scala {
        let classes = paths.workspace.join("classes");
        std::fs::create_dir_all(&classes)?;
        jars.push(classes);
    }
    let id = request.id;
    let output_id = Arc::new(AtomicU32::new(id));
    let written = Arc::new(AtomicUsize::new(0));
    let (phase, method) = if request.action == Action::Check {
        ("checking", "check(Ljava/lang/String;)V")
    } else {
        ("running", "run(Ljava/lang/String;)V")
    };
    emit(&json!({"id": id, "type": "phase", "phase": phase}))?;
    let config = configuration(&output_id, &written, paths)
        .class_path(ClassPath::from(jars.as_slice()))
        .add_system_property("ristretto.scala.version", scala_version)
        .add_system_property("ristretto.language.path", paths.languages.to_string_lossy())
        .add_system_property("java.awt.headless", "true")
        // The browser worker runs on one thread; fork/join work must stay on its caller.
        .add_system_property("java.util.concurrent.ForkJoinPool.common.parallelism", "0")
        .add_system_property("kotlin.environment.keepalive", "false")
        .add_system_property("sun.reflect.inflationThreshold", "2147483647")
        .build()?;
    let vm = VM::new(config).await?;
    let result = async {
        vm.invoke(bridge, method, &[request.source.as_str()])
            .await?;
        if request.action == Action::Check {
            emit(&json!({"id": id, "type": "checked"}))?;
        }
        Ok::<(), ristretto_vm::Error>(())
    }
    .await;
    if let Err(error) = result {
        if let ristretto_vm::Error::Throwable(ref throwable) = error {
            let _result = vm
                .invoke(
                    "java.lang.Throwable",
                    "printStackTrace()V",
                    std::slice::from_ref(throwable),
                )
                .await;
        }
        return Err(error.to_string().into());
    }
    emit(&json!({"id": id, "type": "done"}))?;
    Ok(())
}

/// Decode and execute one host request, emitting its result as NDJSON on stdout.
///
/// The browser calls this repeatedly on a reactor, keeping `JShell`'s heap alive.
pub async fn handle_request(bytes: &[u8]) {
    let paths = RuntimePaths::default();
    match serde_json::from_slice::<Request>(bytes) {
        Ok(request) => {
            if let Err(error) = execute(&request, &paths).await {
                let _result =
                    emit(&json!({"id": request.id, "type": "error", "message": error.to_string()}));
            }
        }
        Err(error) => {
            let _result = emit(&json!({"id": 0, "type": "error", "message": error.to_string()}));
        }
    }
}

struct ShellSession {
    vm: Arc<VM>,
    version: u16,
    id: Arc<AtomicU32>,
    written: Arc<AtomicUsize>,
}

#[derive(Deserialize)]
struct Reload {
    feedback: String,
    entries: Vec<ReloadEntry>,
}

#[derive(Deserialize)]
struct ReloadEntry {
    source: String,
    drop: bool,
}

thread_local! {
    static SHELL: RefCell<Option<ShellSession>> = const { RefCell::new(None) };
}

async fn new_shell(
    request: &Request,
    paths: &RuntimePaths,
) -> Result<ShellSession, Box<dyn std::error::Error>> {
    let output_id = Arc::new(AtomicU32::new(request.id));
    let written = Arc::new(AtomicUsize::new(0));
    let config = configuration(&output_id, &written, paths)
        // Set this before VM initialization, when older JDKs cache reflection settings.
        .add_system_property("sun.reflect.inflationThreshold", "2147483647")
        .class_path(ClassPath::from(&[paths
            .java_home
            .join("browser-jshell.jar")]))
        .build()?;
    Ok(ShellSession {
        vm: VM::new(config).await?,
        version: request.java_version,
        id: output_id,
        written,
    })
}

async fn shell_call(
    session: &ShellSession,
    method: &str,
    source: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let value = session
        .vm
        .try_invoke("BrowserJShell", method, &[source])
        .await
        // Preserve any throwable while this session still owns its heap.
        .map_err(|error| error.to_string())?;
    Ok(serde_json::from_str(&value.as_string()?)?)
}

async fn execute_jshell(
    request: &Request,
    paths: &RuntimePaths,
) -> Result<(), Box<dyn std::error::Error>> {
    if request.java_version < 11 {
        return Err("JShell requires Java 11 or newer in this playground".into());
    }
    let id = request.id;
    emit(&json!({"id": id, "type": "phase", "phase": "evaluating"}))?;
    let previous = SHELL.with(|cell| cell.borrow_mut().take());
    let mut session = match previous {
        Some(session) if session.version == request.java_version => session,
        _ => new_shell(request, paths).await?,
    };
    session.id.store(id, Ordering::Relaxed);
    session.written.store(0, Ordering::Relaxed);
    let result = match request.operation {
        ShellOperation::Input => {
            session
                .vm
                .try_invoke(
                    "BrowserJShell",
                    "input(Ljava/lang/String;)Ljava/lang/String;",
                    std::slice::from_ref(&request.source),
                )
                .await
        }
        ShellOperation::Complete => {
            session
                .vm
                .try_invoke(
                    "BrowserJShell",
                    "complete(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    &[request.source.clone(), request.cursor.to_string()],
                )
                .await
        }
        ShellOperation::Cancel => {
            session
                .vm
                .try_invoke(
                    "BrowserJShell",
                    "cancel()Ljava/lang/String;",
                    &[] as &[ristretto_vm::Value],
                )
                .await
        }
    };
    match result {
        Ok(value) => {
            let mut event: serde_json::Value = serde_json::from_str(&value.as_string()?)?;
            if let Some(reload) = event.get("reload") {
                let reload: Reload = serde_json::from_value(reload.clone())?;
                drop(session);
                session = new_shell(request, paths).await?;
                event = shell_call(
                    &session,
                    "beginReload(Ljava/lang/String;)Ljava/lang/String;",
                    &reload.feedback,
                )
                .await?;
                for entry in reload.entries {
                    let method = if entry.drop {
                        "dropSource(Ljava/lang/String;)Ljava/lang/String;"
                    } else {
                        "input(Ljava/lang/String;)Ljava/lang/String;"
                    };
                    event = shell_call(&session, method, &entry.source).await?;
                }
            }
            let object = event.as_object_mut().ok_or("Invalid JShell response")?;
            let closed = object.get("closed").and_then(serde_json::Value::as_bool) == Some(true)
                || object.get("reset").and_then(serde_json::Value::as_bool) == Some(true);
            object.insert("id".to_string(), json!(id));
            emit(&event)?;
            if !closed {
                SHELL.with(|cell| *cell.borrow_mut() = Some(session));
            }
        }
        Err(error) => {
            if let ristretto_vm::Error::Throwable(ref throwable) = error {
                let _result = session
                    .vm
                    .invoke(
                        "java.lang.Throwable",
                        "printStackTrace()V",
                        std::slice::from_ref(throwable),
                    )
                    .await;
            }
            return Err(error.to_string().into());
        }
    }
    Ok(())
}

#[cfg(target_family = "wasm")]
#[expect(
    unsafe_code,
    reason = "wit-bindgen generates the canonical component ABI exports"
)]
#[expect(
    clippy::same_length_and_capacity,
    reason = "wit-bindgen lifts canonical ABI strings with capacity equal to their length"
)]
mod component {
    wit_bindgen::generate!({ path: "wit", world: "playground" });
    struct Engine;
    thread_local! {
        static RUNTIME: Result<tokio::runtime::Runtime, std::io::Error> = tokio::runtime::Builder::new_current_thread()
            .enable_all().build();
    }
    impl Guest for Engine {
        fn execute(request: String) {
            RUNTIME.with(|runtime| match runtime {
                Ok(runtime) => runtime.block_on(super::handle_request(request.as_bytes())),
                Err(error) => {
                    let _result = super::emit(&serde_json::json!({"id": 0, "type": "error", "message": error.to_string()}));
                }
            });
        }
    }
    export!(Engine);
}
