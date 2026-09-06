//! A single-request WASI command for the browser playground.
//!
//! The host mounts a JSON request at /workspace/request.json and a Java runtime at /jdk.
//! stdout carries JSON events; Java output is encoded as bytes so split UTF-8 writes survive.

use ristretto_vm::{ClassPath, ClassPathEntry, Compiler, ConfigurationBuilder, Memory, VM};
use serde::Deserialize;
use serde_json::json;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::Mutex;

const OUTPUT_LIMIT: usize = 1024 * 1024;

struct RuntimePaths {
    java_home: PathBuf,
    workspace: PathBuf,
}

impl Default for RuntimePaths {
    fn default() -> Self {
        // Native process tests supply isolated mounts; the browser uses the WASI defaults.
        Self {
            java_home: std::env::var_os("RISTRETTO_PLAYGROUND_JAVA_HOME")
                .map_or_else(|| PathBuf::from("/jdk"), PathBuf::from),
            workspace: std::env::var_os("RISTRETTO_PLAYGROUND_WORKSPACE")
                .map_or_else(|| PathBuf::from("/workspace"), PathBuf::from),
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
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Action {
    Compile,
    Run,
}

fn emit(event: &serde_json::Value) -> io::Result<()> {
    let mut output = io::stdout().lock();
    serde_json::to_writer(&mut output, event)?;
    output.write_all(b"\n")?;
    output.flush()
}

#[derive(Debug)]
struct EventWriter {
    id: u32,
    stream: &'static str,
    written: Arc<AtomicUsize>,
}

impl Write for EventWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let previous = self.written.fetch_add(bytes.len(), Ordering::Relaxed);
        if previous.saturating_add(bytes.len()) > OUTPUT_LIMIT {
            emit(
                &json!({"id": self.id, "type": "error", "message": "Output exceeded 1 MiB; execution stopped."}),
            )?;
            std::process::exit(1);
        }
        if !bytes.is_empty() {
            emit(&json!({"id": self.id, "type": "output", "stream": self.stream, "bytes": bytes}))?;
        }
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()
    }
}

fn configuration(
    id: u32,
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
            id,
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
    emit(&json!({"id": id, "type": "phase", "phase": "compiling"}))?;
    let compiler = Compiler::new(configuration(id, &written, paths).build()?).await?;
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
        let config = configuration(id, &written, paths)
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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let paths = RuntimePaths::default();
    let request = std::fs::read(paths.workspace.join("request.json"))
        .map_err(|error| error.to_string())
        .and_then(|bytes| {
            serde_json::from_slice::<Request>(&bytes).map_err(|error| error.to_string())
        });
    match request {
        Ok(request) => {
            if let Err(error) = execute(&request, &paths).await {
                let _result =
                    emit(&json!({"id": request.id, "type": "error", "message": error.to_string()}));
            }
        }
        Err(message) => {
            let _result = emit(&json!({"id": 0, "type": "error", "message": message}));
        }
    }
}
