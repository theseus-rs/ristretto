//! Ristretto `javac` CLI entry point.

#![forbid(unsafe_code)]

mod logging;

use ristretto_vm::{Compiler, CompilerError, ConfigurationBuilder};
use std::env;
use std::process::ExitCode;

#[cfg(any(
    target_family = "wasm",
    target_endian = "big",
    target_os = "dragonfly",
    target_arch = "mips",
    target_arch = "mips64"
))]
#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    common_main().await
}

#[cfg(all(
    not(target_family = "wasm"),
    target_endian = "little",
    not(target_os = "dragonfly"),
    not(any(target_arch = "mips", target_arch = "mips64"))
))]
#[tokio::main]
async fn main() -> ExitCode {
    common_main().await
}

async fn common_main() -> ExitCode {
    if let Err(error) = logging::initialize() {
        eprintln!("{error}");
        return ExitCode::from(3);
    }

    let mut configuration = ConfigurationBuilder::new();
    // Javac's complex diagnostic paths currently expose JIT state corruption on Windows. Keep
    // the CLI on the interpreter there until the Cranelift ABI boundary is reliable.
    if cfg!(target_os = "windows") {
        configuration = configuration.interpreted(true);
    }
    if let Ok(java_version) = env::var("JAVA_VERSION") {
        configuration = configuration.java_version(java_version);
    }
    if let Some(class_path) = env::var_os("CLASSPATH") {
        configuration =
            configuration.add_system_property("env.class.path", class_path.to_string_lossy());
    }
    let configuration = match configuration.build() {
        Ok(configuration) => configuration,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(3);
        }
    };
    let compiler = match Compiler::new(configuration).await {
        Ok(compiler) => compiler,
        Err(error) => return compiler_error_exit_code(&error),
    };
    let arguments = env::args_os().skip(1).collect::<Vec<_>>();
    match compiler.compile(&arguments).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => compiler_error_exit_code(&error),
    }
}

fn compiler_error_exit_code(error: &CompilerError) -> ExitCode {
    if matches!(
        error,
        CompilerError::Vm(_) | CompilerError::UnknownExitCode(_)
    ) {
        eprintln!("{error}");
    }
    u8::try_from(error.exit_code()).map_or(ExitCode::from(3), ExitCode::from)
}
