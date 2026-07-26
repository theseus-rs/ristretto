#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

#[cfg(not(target_family = "wasm"))]
mod cache;
mod condition;
mod context;
mod coordinate;
mod destination;
mod error;
mod graph;
mod metadata;
mod override_rule;
mod repository;
mod resolver;
mod transport;
mod version;

use std::future::Future;
use std::pin::Pin;

/// Boxed future used by extensible asynchronous interfaces on native targets.
#[cfg(not(target_family = "wasm"))]
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Boxed future used by extensible asynchronous interfaces on WebAssembly.
#[cfg(target_family = "wasm")]
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

#[cfg(not(target_family = "wasm"))]
pub use cache::FileCache;
pub use context::ResolutionContext;
pub use coordinate::{ArtifactCoordinate, ArtifactKey};
pub use destination::{
    ArtifactDestination, ArtifactTransaction, DownloadReport, DownloadedArtifact, VerifiedChecksum,
};
#[cfg(not(target_family = "wasm"))]
pub use destination::{FileDestination, FileTransaction};
pub use error::{Error, Result};
pub use graph::{
    DependencyEdge, DependencyNode, NodeId, NodeStatus, Resolution, ResolutionEvent,
    ResolutionLock, ResolvedArtifact,
};
pub use metadata::{AvailableVersion, VersionListing};
pub use override_rule::{OverrideAction, OverrideMatcher, OverrideRule};
pub use repository::{
    Authentication, CENTRAL_REPOSITORY_ID, CENTRAL_REPOSITORY_URL, ChecksumPolicy, Mirror, Proxy,
    RemoteRepository, RepositoryPolicy, SecretString, UpdatePolicy,
};
pub use resolver::{
    ArtifactMaterializer, ArtifactSelection, Classpath, ConflictPolicy, DependencyGraphResolver,
    EffectiveDependency, EffectiveModel, EffectiveModelBuilder, LoadedPom, ModelLoader,
    ResolutionRequest, ResolutionRoot, Resolver, ResolverBuilder,
};
pub use transport::{ByteStream, ResourceKind, Transport, TransportRequest, TransportResponse};
#[cfg(not(target_family = "wasm"))]
pub use transport::{
    CertificateEncoding, FileTransport, HttpTransport, TlsConfiguration, TrustAnchor,
};
pub use version::{Version, VersionRange, VersionSpec};
