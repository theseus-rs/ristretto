# Ristretto CLI

[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_java.svg)](https://crates.io/crates/ristretto_java)
[![License](https://img.shields.io/crates/l/ristretto_java)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

## Getting Started

Command line interface for the Ristretto [JVM](https://docs.oracle.com/javase/specs/jvms/se24/html/index.html).

# Examples

```shell
java HelloWorld
```

## Feature flags

`ristretto_java` uses feature flags to address compile time and binary size uses.

The following features are available:

| Name                       | Description                                    | Default? |
|----------------------------|------------------------------------------------|----------|
| `audio`                    | Enables `javax.sound` support                  | Yes      |
| `startup-trace`            | Enables startup tracing                        | No       |
| `tls-native-tls`           | Enables Native TLS support                     | No       |
| `tls-rustls-aws-lc-rs`     | Enables Rustls with the AWS-LC crypto provider | No       |
| `tls-rustls-ring`          | Enables Rustls with the Ring crypto provider   | Yes      |
| `url`                      | Enables URL class path entries                 | No       |

The TLS backend features are alternatives. To select a non-default backend without compiling Ring,
disable default features and enable `tls-native-tls` or `tls-rustls-aws-lc-rs`, together with any
other required features such as `audio`.
