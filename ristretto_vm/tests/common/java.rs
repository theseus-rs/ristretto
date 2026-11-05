//! JDK resolution and fixture compilation shared with the compatibility harness.

use super::process::{self, Output};
use ristretto_classloader::runtime;
use ristretto_vm::Error::InternalError;
use ristretto_vm::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

pub fn home(version: &str) -> Result<PathBuf> {
    let runtime =
        tokio::runtime::Runtime::new().map_err(|error| InternalError(error.to_string()))?;
    let (home, _version, _loader) = runtime.block_on(runtime::version_class_loader(version))?;
    Ok(home)
}

/// Prevent ambient Java flags from overriding limits, GC selection, or class paths.
pub fn command(home: &Path, tool: &str) -> Command {
    let mut command = Command::new(
        home.join("bin")
            .join(format!("{tool}{}", std::env::consts::EXE_SUFFIX)),
    );
    for variable in [
        "JAVA_TOOL_OPTIONS",
        "JDK_JAVA_OPTIONS",
        "_JAVA_OPTIONS",
        "CLASSPATH",
    ] {
        command.env_remove(variable);
    }
    command
}

pub fn compile(
    home: &Path,
    source_dir: &Path,
    class_dir: &Path,
    java_release: Option<&str>,
    java8_bytecode: bool,
    timeout: Duration,
) -> Result<Output> {
    std::fs::create_dir_all(class_dir)?;
    let mut javac = command(home, "javac");
    javac.arg("-parameters");
    if let Some(release) = java_release {
        // Fixtures using records or nest attributes can opt into newer bytecode.
        javac.args(["--release", release]);
    } else if java8_bytecode {
        javac.args(["-source", "8", "-target", "8"]);
    }
    // javac also compiles referenced helper sources from the fixture directory.
    javac
        .arg("-d")
        .arg(class_dir)
        .arg("-cp")
        .arg(source_dir)
        .arg(source_dir.join("Test.java"))
        .current_dir(source_dir);
    let output = process::run(&mut javac, timeout)?;
    output.require_success()?;
    Ok(output)
}

pub fn runtime_properties(path: &Path) -> Result<HashMap<String, String>> {
    let contents = std::fs::read_to_string(path)?;
    let mut properties = HashMap::new();
    for (line_number, line) in contents.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(InternalError(format!(
                "invalid property at {}:{}",
                path.display(),
                line_number + 1
            )));
        };
        properties.insert(key.trim().to_string(), value.trim().to_string());
    }
    Ok(properties)
}
