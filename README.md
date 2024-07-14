<p align="center"><img width="250" height="250" src="images/logo.jpeg"></p>

# Ristretto

[![ci](https://github.com/theseus-rs/ristretto/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/ristretto/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/ristretto_classfile/badge.svg)](https://docs.rs/ristretto_classfile)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_classfile.svg)](https://crates.io/crates/ristretto_classfile)
[![License](https://img.shields.io/crates/l/ristretto_classfile)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

Crates for the [JVM Specification](https://docs.oracle.com/javase/specs/jvms/se22/html/)

## Getting Started

Crates for the [JVM Specification](https://docs.oracle.com/javase/specs/jvms/se22/html/)

Supports reading and writing class files for any version of Java version up to 23. Verification of class files is
supported, but is still a work in progress.

# Examples

```rust
use ristretto_classfile::{ClassFile, ConstantPool, Result, Version};

fn main() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let this_class = constant_pool.add_class("Foo")?;
    let class_file = ClassFile {
        version: Version::Java21 { minor: 0 },
        constant_pool,
        this_class,
        ..Default::default()
    };
    class_file.verify()
}
```

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
