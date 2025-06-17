<p align="center"><img width="250" height="250" src="images/logo.svg"></p>

# Ristretto

[![ci](https://github.com/theseus-rs/ristretto/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/ristretto/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/ristretto_classfile/badge.svg)](https://docs.rs/ristretto_classfile)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_vm.svg)](https://crates.io/crates/ristretto_vm)
[![License](https://img.shields.io/crates/l/ristretto_classfile)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

Embeddable Java Virtual Machine [JVM](https://docs.oracle.com/javase/specs/jvms/se24/html/) implementation.

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

- Runtime classes based on LTS versions of [AWS Corretto](https://github.com/corretto)
- Load classes from directories, jars, modules
- Url class loading from jars and modules
- Reading, writing, verifying classes
- Verification of class files is supported, but is still a work in progress.
- Just-In-Time (JIT) compilation for functions that exclusively use primitive type byte code

### Limitations

#### String Encoding

Ristretto uses Rust's `String` type to represent Java strings. This means that Ristretto does not support unpaired
surrogates from Java's UTF-16 encoding. Any unpaired surrogates will be replaced with the replacement character `�`
(U+FFFD) when decoding Java strings. For additional details on how Java and Rust handle strings, see
[encodings](docs/encoding/index.md).

#### Instructions

The Invokedynamic instruction is not implemented.

#### Just-In-Time (JIT) Compilation

The JIT compiler only supports functions that use primitive type byte codes. The JIT compiler is not implemented for
functions that use array, reference/object byte codes or call other functions.

#### Threading

Threading is not implemented. The JVM has been structured to allow for threading in the future by utilizing async with
the `tokio` runtime.

#### Runtime Native Methods

The Java runtime requires hundreds of native methods. This project aims to provide Rust equivalents for these methods
on an as needed basis. Currently, only a small subset of these are implemented. If a native method is called that is not
implemented, the program will panic. Please submit a pull request, or open an issue if you need a specific native
method implemented.

#### Java Native Interface (JNI)

JNI is not implemented.

#### Security Manager

Support for the Security Manager is not implemented and there are no plans to implement it. The security manager has
been deprecated; see: [JEP 411](https://openjdk.org/jeps/411)

`System.getSecurityManager()` will always return `null`, and `System.setSecurityManager()` will throw an exception.

#### Finalizers

Support for finalizers is not implemented and there are no plans to implement it. Finalizers have been deprecated; see:
[JEP 421](https://openjdk.java.net/jeps/421)

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
