# Ristretto VM

[![Documentation](https://docs.rs/ristretto_vm/badge.svg)](https://docs.rs/ristretto_vm)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_vm.svg)](https://crates.io/crates/ristretto_vm)
[![License](https://img.shields.io/crates/l/ristretto_vm)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

Ristretto VM is a [Java Virtual Machine](https://docs.oracle.com/javase/specs/jvms/se24/html/index.html)
implementation written in pure Rust. It executes Java bytecode by interpreting class files loaded
through the Ristretto classloader.

## Features

- Bytecode interpretation with no dependencies on existing JVM implementations
- Pure Rust implementation for memory safety and performance
- Support for Java class loading and execution
- Configurable VM parameters
- Basic JIT compilation capabilities
- Type safe concurrent, mark-sweep garbage collector

## Examples

```rust,ignore
use ristretto_vm::{VM, Configuration, ConfigurationBuilder};
use ristretto_classloader::ClassPath;

#[tokio::main]
async fn main() -> ristretto_vm::Result<()> {
    // Create a VM configuration
    let configuration = ConfigurationBuilder::new()
        .class_path(ClassPath::from(&["/path/to/classes"]))
        .build()?;

    // Create the VM instance
    let mut vm = VM::new(configuration).await?;

    // Execute main method of a class
    let _ = vm.invoke_main(&[] as &[&str]).await?;
    Ok(())
}
```

## Feature flags

The following features are available:

| Name            | Description                    | Default? |
|-----------------|--------------------------------|----------|
| `native-tls`    | Enables native TLS support     | No       |
| `rustls-tls`    | Enables rustls TLS support     | Yes      |
| `startup-trace` | Enables startup tracing        | No       |
| `url`           | Enables URL class path entries | No       |
