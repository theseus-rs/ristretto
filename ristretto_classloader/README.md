# Ristretto ClassLoader

[![ci](https://github.com/theseus-rs/ristretto/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/ristretto/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/ristretto_classloader/badge.svg)](https://docs.rs/ristretto_classloader)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_classloader.svg)](https://crates.io/crates/ristretto_classloader)
[![License](https://img.shields.io/crates/l/ristretto_classloader)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

## Getting Started

Implementation of a [JVM Class Loader](https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html)
that is used to load Java classes. Classes can be loaded from the file system or from a URL;
jar and modules are supported. A runtime Java class loader can be created from any version of
[AWS Corretto](https://github.com/corretto). The runtime class loader will download and install
the requested version of Corretto and create a class loader that can be used to load Java
classes.

The AWS Corretto runtime is installed in the following directory:

- Unix: `$HOME/.ristretto/<version>`
- Windows: `%USERPROFILE%\.ristretto\<version>`

# Examples

```rust
use ristretto_classloader::{ClassLoader, ClassPath, Result};
use std::sync::Arc;

fn main() -> Result<()> {
    fn main() -> Result<()> {
        let (version, class_loader) = runtime::class_loader("21")?;
        let class_name = "java.util.HashMap";
        println!("Loading {class_name} from Java runtime {version}");
        let class = class_loader.load(class_name)?;
        println!("{class:?}");
        Ok(())
    }
}
```

## Feature flags

The following features are available:

| Name  | Description                    | Default? |
|-------|--------------------------------|----------|
| `url` | Enables url class path entries | No       |

## Safety

These crates use `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

<a href="https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/theseus-rs/ristretto">
<img
  src="https://img.shields.io/static/v1?label=VSCode%20Development%20Container&logo=visualstudiocode&message=Open&color=orange"
  alt="VSCode Development Container"
/>
</a>
<br/>
<a href="https://github.dev/theseus-rs/ristretto">
<img
  src="https://img.shields.io/static/v1?label=GitHub%20Codespaces&logo=github&message=Open&color=orange"
  alt="GitHub Codespaces"
/>
</a>
