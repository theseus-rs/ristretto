# Ristretto javac

[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_javac.svg)](https://crates.io/crates/ristretto_javac)
[![License](https://img.shields.io/crates/l/ristretto_javac)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

`ristretto_javac` provides a `javac` command-line program backed by the JDK compiler hosted by the
Ristretto virtual machine.

```shell
cargo install ristretto_javac
javac -d classes HelloWorld.java
```

Rust applications can reuse the compiler through `ristretto_vm`:

```rust
use ristretto_vm::{Compiler, CompilerError};

#[tokio::main]
async fn main() -> Result<(), CompilerError> {
    let compiler = Compiler::default().await?;
    compiler.compile(&["-d", "classes", "HelloWorld.java"]).await?;
    Ok(())
}
```

Pass a `ristretto_vm::Configuration` to `Compiler::new` to select a Java home or version, redirect
the compiler streams, or customize VM behavior. `CompilerError` preserves standard `javac` failure
categories and exit codes. The Rust API can also compile one or more source strings entirely in
memory and return every generated class.
