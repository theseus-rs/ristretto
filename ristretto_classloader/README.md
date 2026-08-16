# Ristretto ClassLoader

[![Documentation](https://docs.rs/ristretto_classloader/badge.svg)](https://docs.rs/ristretto_classloader)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
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
use ristretto_classloader::{ClassLoader, ClassPath, JavaStr, Result};
use std::sync::Arc;

fn main() -> Result<()> {
    fn main() -> Result<()> {
        let (version, class_loader) = runtime::class_loader("21")?;
        let class_name = "java.util.HashMap";
        println!("Loading {class_name} from Java runtime {version}");
        let class = class_loader.load(JavaStr::try_from_str(class_name)?)?;
        println!("{class:?}");
        Ok(())
    }
}
```

Classes can also be retained and loaded entirely in memory:

```rust,no_run
use ristretto_classloader::{ClassLoader, ClassPath, ClassPathEntry, JavaStr, Memory, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let class_bytes = std::fs::read("HelloWorld.class")?;
    let memory = Memory::new("memory");
    memory.add_class(&class_bytes).await?;
    let class_path = ClassPath::new(vec![ClassPathEntry::Memory(memory)]);
    let class_loader = ClassLoader::new("memory", class_path);
    let class = class_loader.load(JavaStr::try_from_str("HelloWorld")?).await?;
    println!("{class:?}");
    Ok(())
}
```

## Feature flags

`ristretto_classloader` uses feature flags to address compile time and binary size uses.

The following features are available:

| Name                       | Description                                    | Default? |
|----------------------------|------------------------------------------------|----------|
| `tls-native-tls`           | Enables Native TLS support                     | No       |
| `tls-rustls-aws-lc-rs`     | Enables Rustls with the AWS-LC crypto provider | No       |
| `tls-rustls-ring`          | Enables Rustls with the Ring crypto provider   | Yes      |
| `url`                      | Enables URL class path entries                 | No       |

The TLS backend features are alternatives. To select a non-default backend without compiling Ring,
disable default features and enable `tls-native-tls` or `tls-rustls-aws-lc-rs`.
