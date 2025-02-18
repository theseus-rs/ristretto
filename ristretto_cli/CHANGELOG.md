# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## `ristretto_cli` - [0.13.1](https://github.com/theseus-rs/ristretto/compare/v0.13.0...v0.13.1) - 2025-02-18

### Fixed
- correct if_acmpeq and if_acmpne instructions when comparing class references
- remove unnecessary clone in values returned from frame

### Other
- update Cargo.toml dependencies
- refactor Class constructor functions to return Arc<Class>
- update dependencies
