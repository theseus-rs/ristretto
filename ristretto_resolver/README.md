# Ristretto Resolver

[![Documentation](https://docs.rs/ristretto_resolver/badge.svg)](https://docs.rs/ristretto_resolver)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_resolver.svg)](https://crates.io/crates/ristretto_resolver)
[![License](https://img.shields.io/crates/l/ristretto_resolver)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

`ristretto_resolver` asynchronously resolves JVM dependency graphs and retrieves artifacts from
Maven-compatible repositories. The default configuration uses Maven Central, keeps metadata only
for the life of the resolver, and neither reads Maven settings nor creates persistent files.

The resolver provides:

- effective POM construction with parents, imported BOMs, dependency management, properties,
  dependency-relevant profiles (including Maven 4 condition activation), repositories, and
  relocations;
- release, snapshot, `LATEST`, `RELEASE`, and version-range selection;
- scope derivation, exclusions, optional dependencies, cycle annotation, and nearest-first
  conflict mediation;
- `ForceVersion`, `Replace`, and `Exclude` rules for transitive dependencies;
- an annotated graph containing selected and omitted dependency occurrences, provenance,
  mediation events, root-to-dependency paths, and a flattened classpath view;
- separate resolution and download operations, plus a convenience operation combining them;
- transactional streaming to caller-defined destinations, with a native `FileDestination`;
- an opt-in native `FileCache`, checksum validation, mirrors, proxies, and authentication.

## Staged architecture

The public API keeps graph resolution separate from artifact I/O:

```text
Transport
   ↓
ModelLoader (metadata and raw POMs)
   ↓
EffectiveModelBuilder (parents, profiles, and BOMs)
   ↓
DependencyGraphResolver
   ↓
ConflictPolicy and OverrideRule
   ↓
Resolution / ResolutionLock
   ↓
ArtifactMaterializer
   ↓
ArtifactDestination / ArtifactTransaction
```

`Resolver::model_loader`, `effective_model_builder`, `graph_resolver`, and `materializer` expose
these stages individually. `Resolver::resolve` and `Resolver::download` are concise facades over
the same stages; resolving never writes artifacts, and materializing never changes graph
mediation.

## Resolve, inspect, and download

```no_run
use ristretto_resolver::{
    ArtifactCoordinate, ArtifactKey, FileDestination, OverrideAction, OverrideMatcher,
    OverrideRule, ResolutionRequest, Resolver, Result, Version,
};

# async fn example() -> Result<()> {
let resolver = Resolver::builder().build()?;
let request = ResolutionRequest::new(ArtifactCoordinate::new(
    "org.slf4j",
    "slf4j-api",
    "2.0.17",
)?)
.with_override(OverrideRule::new(
    OverrideMatcher::new("com.example", "legacy-api"),
    OverrideAction::ForceVersion(Version::new("4.2.0")),
));

let resolution = resolver.resolve(&request).await?;
for dependency in resolution.transitive_dependencies() {
    println!("{}: {:?}", dependency.coordinate, dependency.status);
}

let key = ArtifactKey::new("com.example", "legacy-api")?;
for path in resolution.paths_to(&key) {
    println!("dependency path: {path:?}");
}

let report = resolver
    .download(&resolution, &FileDestination::new("target/dependencies"))
    .await?;
println!("downloaded {} artifacts", report.artifacts.len());
# Ok(())
# }
```

`Resolver::available_versions` returns repository provenance, the full advertised version list,
and the `latest` and `release` markers. `Resolution::selected_dependencies` returns the resolved
classpath (including roots), while `Resolution::transitive_dependencies` excludes roots.
`Resolution::nodes` retains conflicts, exclusions, optional edges, override removals, and cycles
for diagnostics.

## Locks and reproducible materialization

`Resolution::to_lock` creates a credential-free, Serde-compatible `ResolutionLock` containing the
annotated graph, concrete versions, timestamped snapshot filenames, repository provenance, and
artifact URLs. Runtime credentials and proxy state are deliberately reattached:

```no_run
use ristretto_resolver::{FileDestination, RemoteRepository, ResolutionLock, Resolver, Result};

# async fn materialize_lock(
#     resolver: &Resolver,
#     lock: ResolutionLock,
#     repositories: Vec<RemoteRepository>,
# ) -> Result<()> {
let (_resolution, report) = resolver
    .materializer()
    .materialize_lock(
        lock,
        repositories,
        &FileDestination::new("target/dependencies"),
    )
    .await?;
println!("materialized {} locked artifacts", report.artifacts.len());
# Ok(())
# }
```

Lock reattachment validates graph indexes, coordinates, selected-node provenance, repository IDs,
and every artifact URL. A lock disables cross-repository fallback so a timestamped snapshot cannot
silently change. Machine-local `systemPath` dependencies are intentionally not lockable.

## Alternate storage and transports

Implement `ArtifactDestination` and `ArtifactTransaction` to stream each artifact into an object
store or another caller-owned location. A transaction receives chunks through `write` and must
make data visible only from `commit`; checksum or transfer failures invoke `abort`. The committed
value becomes `DownloadedArtifact::output`, so an S3 implementation can return its bucket/key or
version identifier.

Repository access is similarly extensible through `Transport`. Native builds include HTTP(S) and
`file:` transports. WebAssembly builds include the resolution core but intentionally require the
caller to register an environment-appropriate transport.

Native HTTP transport supports Basic, Bearer, and sensitive-header repository authentication,
Maven-style mirror selection, Basic-authenticated HTTP proxies with non-proxy host globs, and
explicit TLS trust:

```no_run
use ristretto_resolver::{HttpTransport, Resolver, Result, TlsConfiguration};
use std::sync::Arc;

# fn configured() -> Result<Resolver> {
let tls = TlsConfiguration::default()
    .with_pem_certificate(std::fs::read("company-ca.pem")?)
    .only_configured_roots();
Resolver::builder()
    .transport(Arc::new(HttpTransport::with_tls(tls)))
    .build()
# }
```

## Configuration and persistence

Repositories, mirrors, proxy and authentication values, explicit profiles, environment values,
and user properties are configured through Rust APIs. Credentials use `SecretString`, whose debug
representation is redacted. Maven `settings.xml` is deliberately not read. Gradle
`gradle.properties` is deliberately not read.

Pass `FileCache::new(path)` to `ResolverBuilder::file_cache` to opt into a local repository cache.
The cache uses repository-layout paths, atomic writes, origin tracking, update policies, and stale
metadata when resolving offline. Without this option, only small POM and metadata responses are
cached in memory for the resolver session.

`ResolverBuilder::offline(true)` prohibits non-`file:` transport access. With a `FileCache`, cached
POMs, metadata, checksums, and artifacts remain usable according to origin tracking; missing
resources return an offline error without attempting a network connection.

Checksums are attempted in SHA-512, SHA-256, SHA-1, then MD5 order. Missing checksums warn by
default, can be required with `ChecksumPolicy::Fail`, or disabled with `ChecksumPolicy::Ignore`;
a published checksum mismatch always aborts the active destination transaction.

This crate focuses on dependency resolution and retrieval. It does not execute builds or plugins,
install or deploy artifacts, resolve reactors, or parse settings from files.
