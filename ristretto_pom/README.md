# Ristretto POM

[![Documentation](https://docs.rs/ristretto_pom/badge.svg)](https://docs.rs/ristretto_pom)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_pom.svg)](https://crates.io/crates/ristretto_pom)
[![License](https://img.shields.io/crates/l/ristretto_pom)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

A standalone library for parsing and manipulating [Maven Project Object Model (POM)](https://maven.apache.org/pom.html)
files.

## Examples

```rust
use ristretto_pom::Project;

let project = Project::from_file("pom.xml") ?;
println!("Group ID: {:?}", project.group_id);
println!("Artifact ID: {}", project.artifact_id);
println!("Version: {:?}", project.version);
```
