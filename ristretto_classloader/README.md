# Ristretto ClassLoader

[![Documentation](https://docs.rs/ristretto_classloader/badge.svg)](https://docs.rs/ristretto_classloader)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_classloader.svg)](https://crates.io/crates/ristretto_classloader)
[![License](https://img.shields.io/crates/l/ristretto_classloader)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

Implementation of a [JVM Class Loader](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html)
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

