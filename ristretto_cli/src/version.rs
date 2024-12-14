use ristretto_vm::DEFAULT_JAVA_VERSION;
use std::env;

/// Get the full version of the program (e.g. "java/0.0.0 Linux/5.11.0-37-generic/x86_64").
pub fn full() -> String {
    let program_name = "java";
    let version = env!("CARGO_PKG_VERSION");
    let java_version = env::var("JAVA_VERSION").unwrap_or(DEFAULT_JAVA_VERSION.to_string());
    let info = os_info::get();
    let os = format!("{}", info.os_type()).replace(' ', "-");
    let os_version = info.version();
    let architecture = info.architecture().unwrap_or("unknown");

    format!("{program_name}/{version}/{java_version} {os}/{os_version}/{architecture}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let version = full();
        assert!(version.starts_with("java/"));
    }
}
