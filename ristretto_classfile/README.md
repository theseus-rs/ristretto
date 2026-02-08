# Ristretto ClassFile

[![Documentation](https://docs.rs/ristretto_classfile/badge.svg)](https://docs.rs/ristretto_classfile)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_classfile.svg)](https://crates.io/crates/ristretto_classfile)
[![License](https://img.shields.io/crates/l/ristretto_classfile)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

Implementation of the [JVM Class File Format](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html) that
is used to read, write and verify Java classes.

Supports reading and writing class files for any version of Java version up to 25. Verification of class files is
supported, but is still a work in progress.

# Examples

```rust
use ristretto_classfile::{ClassFile, ConstantPool, Result, Version, JAVA_21};

fn main() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let this_class = constant_pool.add_class("Foo")?;
    let class_file = ClassFile {
        version: JAVA_21,
        constant_pool,
        this_class,
        ..Default::default()
    };
    class_file.verify()
}
```
