# Ristretto JImage

[![Documentation](https://docs.rs/ristretto_jimage/badge.svg)](https://docs.rs/ristretto_jimage)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_jimage.svg)](https://crates.io/crates/ristretto_jimage)
[![License](https://img.shields.io/crates/l/ristretto_jimage)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

Ristretto JImage reads Java Image (JImage) files, which are used in Java 9 and later to store Java class files and
resources in a compact format. This crate provides functionality to parse JImage files, extract class files and
resources, and access metadata about the contents of the JImage.
