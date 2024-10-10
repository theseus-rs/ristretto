<p align="center"><img width="250" height="250" src="images/logo.jpeg"></p>

# Ristretto

[![ci](https://github.com/theseus-rs/ristretto/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/ristretto/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/ristretto_classfile/badge.svg)](https://docs.rs/ristretto_classfile)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_classfile.svg)](https://crates.io/crates/ristretto_classfile)
[![License](https://img.shields.io/crates/l/ristretto_classfile)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

[JVM](https://docs.oracle.com/javase/specs/jvms/se23/html/) implementation with deterministic memory deallocation.

**This is a work in progress and is not ready for production use.**

## Getting Started

`ristretto` java can be installed using the following methods:

### Linux / MacOS

```shell
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/theseus-rs/ristretto/releases/latest/download/ristretto_cli-installer.sh | sh
```

### Windows

```shell
irm https://github.com/theseus-rs/ristretto/releases/latest/download/ristretto_cli-installer.ps1 | iex
```

For more information, and additional installations instructions (cargo, homebrew, msi),
visit the [ristretto](https://theseus-rs.github.io/ristretto/ristretto_cli/) site.

### Features

- Deterministic memory allocation / deallocation
- No garbage collector
- Loading classes from any version of [AWS Corretto](https://github.com/corretto)
- Load classes from directories, jars, modules
- Url class loading from jars and modules
- Reading, writing, verifying classes for any version of Java version up to 24
- Verification of class files is supported, but is still a work in progress.

# Limitations

- The instructions Athrow, Multianewarray and Invokedynamic are not implemented
- Exceptions are not implemented
- Threading is not implemented
- Numerous JDK native methods are not implemented
- JNI is not implemented
- JIT is not implemented
- No Security manager; see: [JEP 411](https://openjdk.org/jeps/411)

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
