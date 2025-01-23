# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## `ristretto_vm` - [0.12.4](https://github.com/theseus-rs/ristretto/compare/ristretto_vm-v0.12.3...ristretto_vm-v0.12.4) - 2025-01-23

### Fixed
- correct exception byte to instruction offset conversion error
- remove unnecessary parameter cloning
- remove unnecessary clone from LocalVariables
- correct bug in java.lang.Class.getDeclaredMethods0() where class constructors were incorrectly returned

### Other
- add vm benchmarks
- update VM to support LTS and latest Java versions only
- update java runtime versions
- move invokedynamic into separate module
- update Cargo.toml dependencies
