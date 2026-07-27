//! Resolution orchestration from raw POMs through artifact materialization.
//!
//! [`ResolverBuilder`] configures repositories, mirrors, transports, caching, and offline mode.
//! [`ResolutionRequest`] then supplies roots, classpath, activation context, override rules, and
//! artifact selection for one graph. [`Resolver`] exposes both concise resolve/download methods
//! and the individual model-loading, effective-model, graph, and materialization stages.
//!
//! Resolution builds an annotated graph without writing binary artifacts. Materialization is a
//! separate transactional operation, so callers can inspect or lock the selected graph before any
//! download occurs.

use crate::metadata::RepositoryMetadata;
use crate::{
    ArtifactCoordinate, ArtifactDestination, ArtifactKey, ArtifactTransaction, BoxFuture,
    ChecksumPolicy, DependencyEdge, DependencyNode, DownloadReport, DownloadedArtifact, Error,
    Mirror, NodeId, NodeStatus, OverrideAction, OverrideRule, RemoteRepository, RepositoryPolicy,
    Resolution, ResolutionContext, ResolutionEvent, ResolvedArtifact, ResourceKind, Result,
    Transport, TransportRequest, UpdatePolicy, VerifiedChecksum, Version, VersionListing,
    VersionSpec,
};
use fancy_regex::RegexBuilder;
use futures_util::StreamExt;
use ristretto_pom::{Activation, Dependencies, Dependency, DependencyScope, Project};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha512};
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::fmt::Write;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use url::Url;

const SYSTEM_REPOSITORY_ID: &str = "__system";

/// Classpath being resolved.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum Classpath {
    /// Compile classpath.
    Compile,
    /// Runtime classpath.
    #[default]
    Runtime,
    /// Test classpath.
    Test,
}

/// Deterministic dependency conflict mediation strategy.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum ConflictPolicy {
    /// Maven-compatible nearest-first mediation with declaration-order tie breaking.
    #[default]
    NearestFirst,
}

impl Classpath {
    fn includes(self, scope: DependencyScope) -> bool {
        match self {
            Self::Compile => matches!(
                scope,
                DependencyScope::Compile | DependencyScope::Provided | DependencyScope::System
            ),
            Self::Runtime => {
                matches!(scope, DependencyScope::Compile | DependencyScope::Runtime)
            }
            Self::Test => !matches!(scope, DependencyScope::Import),
        }
    }
}

/// Root source for dependency resolution.
#[derive(Debug, Clone)]
pub enum ResolutionRoot {
    /// Published repository artifact.
    Artifact(ArtifactCoordinate),
    /// POM bytes. [`ResolutionContext::base_directory`] controls relative profile references.
    ProjectBytes {
        /// Raw POM XML.
        bytes: Vec<u8>,
        /// Whether the root's own artifact should be downloaded.
        include_artifact: bool,
    },
    /// POM file on a native filesystem.
    #[cfg(not(target_family = "wasm"))]
    ProjectFile(std::path::PathBuf),
}

/// Which artifact files a download operation should deliver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactSelection {
    /// Include the main artifact selected for each graph node.
    pub main_artifacts: bool,
    /// Include POM descriptors in addition to binaries.
    pub poms: bool,
    /// Additional classifier/extension pairs.
    pub attachments: Vec<(String, String)>,
}

impl Default for ArtifactSelection {
    fn default() -> Self {
        Self {
            main_artifacts: true,
            poms: false,
            attachments: Vec::new(),
        }
    }
}

/// Input for one graph resolution.
#[derive(Debug, Clone)]
pub struct ResolutionRequest {
    /// Roots resolved in declaration order.
    pub roots: Vec<ResolutionRoot>,
    /// Requested classpath.
    pub classpath: Classpath,
    /// Profile and interpolation inputs.
    pub context: ResolutionContext,
    /// Declarative transitive dependency overrides.
    pub overrides: Vec<OverrideRule>,
    /// Conflict mediation policy.
    pub conflict_policy: ConflictPolicy,
    /// Whether repositories declared by dependency POMs may be used.
    pub transitive_repositories: bool,
    /// Artifact files selected by download operations.
    pub artifacts: ArtifactSelection,
}

impl ResolutionRequest {
    /// Creates a runtime-classpath request for one published artifact.
    #[must_use]
    pub fn new(root: ArtifactCoordinate) -> Self {
        Self {
            roots: vec![ResolutionRoot::Artifact(root)],
            classpath: Classpath::Runtime,
            context: ResolutionContext::default(),
            overrides: Vec::new(),
            conflict_policy: ConflictPolicy::default(),
            transitive_repositories: true,
            artifacts: ArtifactSelection::default(),
        }
    }

    /// Creates a request from POM bytes.
    #[must_use]
    pub fn from_project_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            roots: vec![ResolutionRoot::ProjectBytes {
                bytes: bytes.into(),
                include_artifact: false,
            }],
            classpath: Classpath::Runtime,
            context: ResolutionContext::default(),
            overrides: Vec::new(),
            conflict_policy: ConflictPolicy::default(),
            transitive_repositories: true,
            artifacts: ArtifactSelection::default(),
        }
    }

    /// Creates a request from a native POM path.
    #[cfg(not(target_family = "wasm"))]
    #[must_use]
    pub fn from_project_file(path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            roots: vec![ResolutionRoot::ProjectFile(path.into())],
            classpath: Classpath::Runtime,
            context: ResolutionContext::default(),
            overrides: Vec::new(),
            conflict_policy: ConflictPolicy::default(),
            transitive_repositories: true,
            artifacts: ArtifactSelection::default(),
        }
    }

    /// Adds an additional root.
    #[must_use]
    pub fn with_root(mut self, root: ResolutionRoot) -> Self {
        self.roots.push(root);
        self
    }

    /// Sets the requested classpath.
    #[must_use]
    pub fn with_classpath(mut self, classpath: Classpath) -> Self {
        self.classpath = classpath;
        self
    }

    /// Sets profile activation and interpolation inputs.
    #[must_use]
    pub fn with_context(mut self, context: ResolutionContext) -> Self {
        self.context = context;
        self
    }

    /// Adds a transitive dependency override.
    #[must_use]
    pub fn with_override(mut self, rule: OverrideRule) -> Self {
        self.overrides.push(rule);
        self
    }

    /// Sets the deterministic conflict mediation policy.
    #[must_use]
    pub fn with_conflict_policy(mut self, policy: ConflictPolicy) -> Self {
        self.conflict_policy = policy;
        self
    }

    /// Enables or disables repositories declared by published dependency POMs.
    ///
    /// Repositories declared by a caller-provided local project remain available.
    #[must_use]
    pub fn with_transitive_repositories(mut self, enabled: bool) -> Self {
        self.transitive_repositories = enabled;
        self
    }

    /// Selects the main, POM, and attached artifacts to materialize.
    #[must_use]
    pub fn with_artifacts(mut self, artifacts: ArtifactSelection) -> Self {
        self.artifacts = artifacts;
        self
    }
}

/// Configures a [`Resolver`].
#[derive(Debug)]
pub struct ResolverBuilder {
    repositories: Vec<RemoteRepository>,
    mirrors: Vec<Mirror>,
    transports: Vec<Arc<dyn Transport>>,
    offline: bool,
    #[cfg(not(target_family = "wasm"))]
    file_cache: Option<crate::FileCache>,
}

impl Default for ResolverBuilder {
    fn default() -> Self {
        let transports: Vec<Arc<dyn Transport>> = vec![
            #[cfg(not(target_family = "wasm"))]
            Arc::new(crate::HttpTransport::default()),
            #[cfg(not(target_family = "wasm"))]
            Arc::new(crate::FileTransport),
        ];
        Self {
            repositories: vec![RemoteRepository::central()],
            mirrors: Vec::new(),
            transports,
            offline: false,
            #[cfg(not(target_family = "wasm"))]
            file_cache: None,
        }
    }
}

impl ResolverBuilder {
    /// Creates a builder with Maven Central and session-only caching.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a repository after those already configured.
    #[must_use]
    pub fn repository(mut self, repository: RemoteRepository) -> Self {
        self.repositories.push(repository);
        self
    }

    /// Replaces the repository list.
    #[must_use]
    pub fn repositories(mut self, repositories: Vec<RemoteRepository>) -> Self {
        self.repositories = repositories;
        self
    }

    /// Adds a mirror selection rule.
    #[must_use]
    pub fn mirror(mut self, mirror: Mirror) -> Self {
        self.mirrors.push(mirror);
        self
    }

    /// Adds or replaces a transport for its supported schemes.
    #[must_use]
    pub fn transport(mut self, transport: Arc<dyn Transport>) -> Self {
        self.transports.push(transport);
        self
    }

    /// Replaces the registered transport list.
    #[must_use]
    pub fn transports(mut self, transports: Vec<Arc<dyn Transport>>) -> Self {
        self.transports = transports;
        self
    }

    /// Enables or disables offline mode.
    #[must_use]
    pub fn offline(mut self, offline: bool) -> Self {
        self.offline = offline;
        self
    }

    /// Enables an explicit native filesystem cache.
    #[cfg(not(target_family = "wasm"))]
    #[must_use]
    pub fn file_cache(mut self, cache: crate::FileCache) -> Self {
        self.file_cache = Some(cache);
        self
    }

    /// Validates and constructs the resolver.
    ///
    /// # Errors
    ///
    /// Returns an error if no repository or transport is configured.
    pub fn build(self) -> Result<Resolver> {
        if self.repositories.is_empty() {
            return Err(Error::InvalidConfiguration(
                "at least one repository is required".to_string(),
            ));
        }
        if self.transports.is_empty() {
            return Err(Error::InvalidConfiguration(
                "at least one repository transport is required".to_string(),
            ));
        }
        if self
            .repositories
            .iter()
            .chain(self.mirrors.iter().map(|mirror| &mirror.repository))
            .any(|repository| repository.id == SYSTEM_REPOSITORY_ID)
        {
            return Err(Error::InvalidConfiguration(format!(
                "repository id '{SYSTEM_REPOSITORY_ID}' is reserved"
            )));
        }
        let mut repository_ids = BTreeMap::<&str, &RemoteRepository>::new();
        for repository in self
            .repositories
            .iter()
            .chain(self.mirrors.iter().map(|mirror| &mirror.repository))
        {
            repository.validate()?;
            if let Some(existing) = repository_ids.insert(&repository.id, repository)
                && existing != repository
            {
                return Err(Error::InvalidConfiguration(format!(
                    "repository id '{}' is configured more than once with different settings",
                    repository.id
                )));
            }
        }
        Ok(Resolver {
            repositories: self.repositories,
            mirrors: self.mirrors,
            transports: self.transports,
            offline: self.offline,
            #[cfg(not(target_family = "wasm"))]
            file_cache: self.file_cache,
            session_cache: Mutex::new(HashMap::new()),
        })
    }
}

/// Asynchronous dependency resolver.
#[derive(Debug)]
pub struct Resolver {
    repositories: Vec<RemoteRepository>,
    mirrors: Vec<Mirror>,
    transports: Vec<Arc<dyn Transport>>,
    offline: bool,
    #[cfg(not(target_family = "wasm"))]
    file_cache: Option<crate::FileCache>,
    session_cache: Mutex<SessionCache>,
}

type SessionCache = HashMap<(String, String), Arc<Vec<u8>>>;

/// Raw POM loaded with repository provenance.
#[derive(Debug, Clone)]
pub struct LoadedPom {
    /// Concrete coordinate selected before loading.
    pub coordinate: ArtifactCoordinate,
    /// Parsed and validated project model.
    pub project: Project,
    /// Repository that supplied the POM.
    pub repository_id: String,
}

/// Metadata and raw-POM loading stage.
#[derive(Debug, Clone, Copy)]
pub struct ModelLoader<'a> {
    resolver: &'a Resolver,
}

impl ModelLoader<'_> {
    /// Lists repository-provenanced versions.
    ///
    /// # Errors
    ///
    /// Returns transport, metadata-validation, or version-parsing errors.
    pub async fn available_versions(&self, key: &ArtifactKey) -> Result<VersionListing> {
        self.resolver.available_versions(key).await
    }

    /// Selects a concrete version and loads its validated raw POM.
    ///
    /// # Errors
    ///
    /// Returns an error when version selection, transport, parsing, or POM
    /// validation fails.
    pub async fn load(&self, coordinate: &ArtifactCoordinate) -> Result<LoadedPom> {
        let request = ResolutionRequest::new(coordinate.clone());
        let session = ResolutionSession::new(self.resolver, &request);
        let coordinate = session
            .select_coordinate(
                &coordinate.key,
                &coordinate.version,
                &self.resolver.repositories,
                &mut Vec::new(),
            )
            .await?;
        let (project, repository_id) = session
            .fetch_project(&coordinate, &self.resolver.repositories)
            .await?;
        Ok(LoadedPom {
            coordinate,
            project,
            repository_id,
        })
    }
}

/// Effective-model construction stage.
#[derive(Debug, Clone, Copy)]
pub struct EffectiveModelBuilder<'a> {
    resolver: &'a Resolver,
}

impl EffectiveModelBuilder<'_> {
    /// Builds the dependency-relevant effective model for one root.
    ///
    /// # Errors
    ///
    /// Returns an error when the model, its parent, an imported BOM, interpolation,
    /// or profile evaluation is invalid.
    pub async fn build(
        &self,
        root: ResolutionRoot,
        context: ResolutionContext,
    ) -> Result<EffectiveModel> {
        let request = ResolutionRequest {
            roots: vec![root.clone()],
            classpath: Classpath::Runtime,
            context,
            overrides: Vec::new(),
            conflict_policy: ConflictPolicy::NearestFirst,
            transitive_repositories: true,
            artifacts: ArtifactSelection::default(),
        };
        let mut session = ResolutionSession::new(self.resolver, &request);
        let (model, _, _, _) = session.load_root(&root).await?;
        Ok((*model).clone())
    }
}

/// Dependency-graph and conflict-policy stage.
#[derive(Debug, Clone, Copy)]
pub struct DependencyGraphResolver<'a> {
    resolver: &'a Resolver,
}

impl DependencyGraphResolver<'_> {
    /// Resolves an annotated graph without materializing artifacts.
    ///
    /// # Errors
    ///
    /// Returns model-loading, version-selection, policy, or graph-validation errors.
    pub async fn resolve(&self, request: &ResolutionRequest) -> Result<Resolution> {
        self.resolver.resolve(request).await
    }
}

/// Artifact materialization stage.
#[derive(Debug, Clone, Copy)]
pub struct ArtifactMaterializer<'a> {
    resolver: &'a Resolver,
}

impl ArtifactMaterializer<'_> {
    /// Materializes a live resolved graph into a caller-provided transactional store.
    ///
    /// # Errors
    ///
    /// Returns transport, content-verification, cache, or destination errors.
    pub async fn materialize<D>(
        &self,
        resolution: &Resolution,
        destination: &D,
    ) -> Result<DownloadReport<D::Output>>
    where
        D: ArtifactDestination,
        D::Output: 'static,
        D::Transaction: 'static,
    {
        self.resolver.download(resolution, destination).await
    }

    /// Reattaches runtime repositories to a lock and materializes it.
    ///
    /// # Errors
    ///
    /// Returns lock-validation, transport, content-verification, cache, or
    /// destination errors.
    pub async fn materialize_lock<D>(
        &self,
        lock: crate::ResolutionLock,
        repositories: Vec<RemoteRepository>,
        destination: &D,
    ) -> Result<(Resolution, DownloadReport<D::Output>)>
    where
        D: ArtifactDestination,
        D::Output: 'static,
        D::Transaction: 'static,
    {
        let resolution = lock.attach_repositories(repositories)?;
        let report = self.materialize(&resolution, destination).await?;
        Ok((resolution, report))
    }
}

impl Resolver {
    /// Creates a resolver builder.
    #[must_use]
    pub fn builder() -> ResolverBuilder {
        ResolverBuilder::new()
    }

    /// Returns the metadata and POM loading stage.
    #[must_use]
    pub fn model_loader(&self) -> ModelLoader<'_> {
        ModelLoader { resolver: self }
    }

    /// Returns the effective-model construction stage.
    #[must_use]
    pub fn effective_model_builder(&self) -> EffectiveModelBuilder<'_> {
        EffectiveModelBuilder { resolver: self }
    }

    /// Returns the dependency-graph and policy stage.
    #[must_use]
    pub fn graph_resolver(&self) -> DependencyGraphResolver<'_> {
        DependencyGraphResolver { resolver: self }
    }

    /// Returns the artifact materialization stage.
    #[must_use]
    pub fn materializer(&self) -> ArtifactMaterializer<'_> {
        ArtifactMaterializer { resolver: self }
    }

    /// Returns all versions advertised for an artifact.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or metadata parsing fails. A missing metadata file in one
    /// repository is not an error when another repository provides metadata.
    pub async fn available_versions(&self, key: &ArtifactKey) -> Result<VersionListing> {
        let mut listing = VersionListing::default();
        let repositories = self.apply_mirrors(&self.repositories);
        for repository in repositories {
            if !repository.releases.enabled && !repository.snapshots.enabled {
                continue;
            }
            let url = repository.resource_url(&key.metadata_path())?;
            let Some(bytes) = self
                .fetch_small(&repository, &url, ResourceKind::Metadata)
                .await?
            else {
                continue;
            };
            let metadata: RepositoryMetadata =
                quick_xml::de::from_reader(Cursor::new(bytes.as_slice()))?;
            validate_metadata_key(&metadata, key, &url)?;
            listing.merge(&repository, metadata);
        }
        if listing.versions.is_empty() {
            return Err(Error::MetadataNotFound(key.to_string()));
        }
        Ok(listing)
    }

    /// Resolves an annotated graph without downloading selected binary artifacts.
    ///
    /// # Errors
    ///
    /// Returns an error for missing descriptors, invalid effective models, or unsatisfied versions.
    pub async fn resolve(&self, request: &ResolutionRequest) -> Result<Resolution> {
        if request.roots.is_empty() {
            return Err(Error::InvalidConfiguration(
                "at least one resolution root is required".to_string(),
            ));
        }
        validate_overrides(&request.overrides)?;
        let mut session = ResolutionSession::new(self, request);
        match request.conflict_policy {
            ConflictPolicy::NearestFirst => session.resolve().await,
        }
    }

    /// Streams every selected artifact to a transactional destination.
    ///
    /// # Errors
    ///
    /// Returns an error after aborting the active transaction if transport, checksum validation, or
    /// destination writing fails. Earlier committed artifacts remain committed.
    #[expect(
        clippy::too_many_lines,
        reason = "download keeps repository fallback, checksum verification, cache, and destination transactions in one failure-safe pipeline"
    )]
    pub async fn download<D>(
        &self,
        resolution: &Resolution,
        destination: &D,
    ) -> Result<DownloadReport<D::Output>>
    where
        D: ArtifactDestination,
        D::Output: 'static,
        D::Transaction: 'static,
    {
        let mut report = DownloadReport {
            artifacts: Vec::new(),
            diagnostics: Vec::new(),
        };
        for artifact in &resolution.artifacts {
            drop(ArtifactCoordinate::from_key(
                artifact.coordinate.key.clone(),
                artifact.coordinate.version.clone(),
            )?);
            drop(ArtifactCoordinate::from_key(
                artifact.coordinate.key.clone(),
                artifact.file_version.clone(),
            )?);
            let preferred_repository = resolution
                .repositories
                .iter()
                .find(|repository| repository.id == artifact.repository_id)
                .ok_or_else(|| {
                    Error::InvalidConfiguration(format!(
                        "resolution omitted repository '{}'",
                        artifact.repository_id
                    ))
                })?;
            if preferred_repository.id == SYSTEM_REPOSITORY_ID {
                preferred_repository.validate_resource_url(&artifact.url)?;
            } else {
                let expected_url = preferred_repository.resource_url(
                    &artifact
                        .coordinate
                        .artifact_path_with_version(&artifact.file_version),
                )?;
                if artifact.url != expected_url {
                    return Err(Error::InvalidConfiguration(format!(
                        "artifact URL does not match {} in repository '{}'",
                        artifact.coordinate, preferred_repository.id
                    )));
                }
            }

            let mut candidate_repositories = vec![preferred_repository];
            if resolution.repository_fallback && preferred_repository.id != SYSTEM_REPOSITORY_ID {
                candidate_repositories.extend(resolution.repositories.iter().filter(
                    |repository| {
                        repository.id != preferred_repository.id
                            && repository.id != SYSTEM_REPOSITORY_ID
                            && repository.accepts_version(&artifact.coordinate.version)
                    },
                ));
            }
            let mut downloaded = None;
            let mut found_without_checksum = false;
            let mut last_failure = None;
            for repository in candidate_repositories {
                if repository.id != SYSTEM_REPOSITORY_ID
                    && !repository.accepts_version(&artifact.coordinate.version)
                {
                    continue;
                }
                let candidate = if repository.id == preferred_repository.id {
                    artifact.clone()
                } else {
                    match self.artifact_for_repository(artifact, repository).await {
                        Ok(candidate) => candidate,
                        Err(error) => {
                            last_failure = Some(error);
                            continue;
                        }
                    }
                };
                let policy = repository.policy_for(&candidate.coordinate.version);
                let expected = if policy.checksum == ChecksumPolicy::Ignore {
                    None
                } else {
                    match self.fetch_checksum(repository, &candidate.url).await {
                        Ok(checksum) => checksum,
                        Err(error) => {
                            last_failure = Some(error);
                            continue;
                        }
                    }
                };
                let response = match self
                    .fetch_stream(repository, &candidate.url, ResourceKind::Artifact)
                    .await
                {
                    Ok(Some(response)) => response,
                    Ok(None) => continue,
                    Err(error) => {
                        last_failure = Some(error);
                        continue;
                    }
                };
                if expected.is_none() && policy.checksum == ChecksumPolicy::Fail {
                    found_without_checksum = true;
                    continue;
                }
                match self
                    .deliver_candidate(
                        candidate.clone(),
                        repository,
                        expected,
                        response,
                        destination,
                    )
                    .await
                {
                    Ok(delivered) => {
                        if delivered.checksum.is_none() && policy.checksum == ChecksumPolicy::Warn {
                            report.diagnostics.push(format!(
                                "repository '{}' did not publish a supported checksum for {}",
                                repository.id, candidate.coordinate
                            ));
                        }
                        downloaded = Some(delivered);
                        break;
                    }
                    Err(error) => {
                        last_failure = Some(error);
                    }
                }
            }
            if let Some(downloaded) = downloaded {
                report.artifacts.push(downloaded);
            } else if let Some(error) = last_failure {
                return Err(error);
            } else if found_without_checksum {
                return Err(Error::MissingChecksum(artifact.coordinate.clone()));
            } else {
                return Err(Error::ArtifactNotFound(artifact.coordinate.clone()));
            }
        }
        Ok(report)
    }

    async fn deliver_candidate<D>(
        &self,
        artifact: ResolvedArtifact,
        repository: &RemoteRepository,
        expected: Option<VerifiedChecksum>,
        mut response: crate::TransportResponse,
        destination: &D,
    ) -> Result<DownloadedArtifact<D::Output>>
    where
        D: ArtifactDestination,
        D::Output: 'static,
        D::Transaction: 'static,
    {
        #[cfg(target_family = "wasm")]
        let _ = repository;
        let mut transaction = destination.begin(&artifact).await?;
        #[cfg(not(target_family = "wasm"))]
        let mut cache_transaction = if response.from_cache || repository.id == SYSTEM_REPOSITORY_ID
        {
            crate::cache::CacheTransaction::Disabled
        } else if let Some(cache) = &self.file_cache {
            cache.begin_artifact(&artifact, repository).await?
        } else {
            crate::cache::CacheTransaction::Disabled
        };
        let mut digester = expected
            .as_ref()
            .map(|checksum| ChecksumDigester::new(&checksum.algorithm));
        let expected_length = response.content_length;
        let mut received_length = 0_u64;
        let transfer_result = async {
            while let Some(chunk) = response.body.next().await {
                let chunk = chunk?;
                received_length += chunk.len() as u64;
                if let Some(digester) = &mut digester {
                    digester.update(&chunk);
                }
                transaction.write(chunk.clone()).await?;
                #[cfg(not(target_family = "wasm"))]
                cache_transaction.write(chunk).await?;
            }
            if let Some(expected_length) = expected_length
                && expected_length != received_length
            {
                return Err(Error::Transport {
                    url: artifact.url.clone(),
                    message: format!(
                        "artifact response length mismatch: expected {expected_length}, received \
                         {received_length}"
                    ),
                });
            }
            Ok::<(), Error>(())
        }
        .await;
        if let Err(error) = transfer_result {
            drop(transaction.abort().await);
            #[cfg(not(target_family = "wasm"))]
            cache_transaction.abort().await;
            return Err(error);
        }
        let verified = if let (Some(expected), Some(digester)) = (expected, digester) {
            let actual = digester.finish();
            if !actual.eq_ignore_ascii_case(&expected.value) {
                drop(transaction.abort().await);
                #[cfg(not(target_family = "wasm"))]
                cache_transaction.abort().await;
                return Err(Error::ChecksumMismatch {
                    coordinate: artifact.coordinate.clone(),
                    expected: expected.value,
                    actual,
                });
            }
            Some(expected)
        } else {
            None
        };
        #[cfg(not(target_family = "wasm"))]
        if let Err(error) = cache_transaction.commit().await {
            drop(transaction.abort().await);
            return Err(error);
        }
        let output = transaction.commit().await?;
        Ok(DownloadedArtifact {
            artifact,
            output,
            checksum: verified,
        })
    }

    /// Resolves and then downloads a request.
    ///
    /// # Errors
    ///
    /// Returns any graph resolution or artifact download error.
    pub async fn resolve_and_download<D>(
        &self,
        request: &ResolutionRequest,
        destination: &D,
    ) -> Result<(Resolution, DownloadReport<D::Output>)>
    where
        D: ArtifactDestination,
        D::Output: 'static,
        D::Transaction: 'static,
    {
        let resolution = self.resolve(request).await?;
        let report = self.download(&resolution, destination).await?;
        Ok((resolution, report))
    }

    fn apply_mirrors(&self, repositories: &[RemoteRepository]) -> Vec<RemoteRepository> {
        let mut result = Vec::new();
        for repository in repositories {
            let already_mirrored = self.mirrors.iter().any(|mirror| {
                mirror.repository.id == repository.id && mirror.repository.url == repository.url
            });
            let mirror = if already_mirrored {
                None
            } else {
                self.mirrors
                    .iter()
                    .find(|mirror| mirror.exactly_matches(repository))
                    .or_else(|| {
                        self.mirrors
                            .iter()
                            .find(|mirror| mirror.matches(repository))
                    })
            };
            let effective = mirror.map_or_else(
                || repository.clone(),
                |mirror| {
                    let mut effective = mirror.repository.clone();
                    effective.releases = repository.releases;
                    effective.snapshots = repository.snapshots;
                    effective
                },
            );
            if let Some(known) = result.iter_mut().find(|known: &&mut RemoteRepository| {
                known.id == effective.id && known.url == effective.url
            }) {
                merge_repository_policy(&mut known.releases, effective.releases);
                merge_repository_policy(&mut known.snapshots, effective.snapshots);
            } else {
                result.push(effective);
            }
        }
        result
    }

    async fn fetch_small(
        &self,
        repository: &RemoteRepository,
        url: &str,
        kind: ResourceKind,
    ) -> Result<Option<Vec<u8>>> {
        repository.validate_resource_url(url)?;
        let cache_key = (repository.id.clone(), url.to_string());
        if let Some(bytes) = self
            .session_cache
            .lock()
            .map_err(|error| Error::InvalidConfiguration(error.to_string()))?
            .get(&cache_key)
            .cloned()
        {
            return Ok(Some((*bytes).clone()));
        }
        #[cfg(not(target_family = "wasm"))]
        if let Some(cache) = &self.file_cache
            && let Some(mut response) = cache.response(repository, url, kind, self.offline).await?
        {
            let expected_length = response.content_length;
            let mut bytes = Vec::new();
            while let Some(chunk) = response.body.next().await {
                let chunk = chunk?;
                if bytes.len().saturating_add(chunk.len()) > 16 * 1024 * 1024 {
                    return Err(Error::Transport {
                        url: url.to_string(),
                        message: "metadata or POM exceeded the 16 MiB safety limit".to_string(),
                    });
                }
                bytes.extend_from_slice(&chunk);
            }
            validate_response_length(url, expected_length, bytes.len())?;
            self.session_cache
                .lock()
                .map_err(|error| Error::InvalidConfiguration(error.to_string()))?
                .insert(cache_key.clone(), Arc::new(bytes.clone()));
            return Ok(Some(bytes));
        }
        let Some(mut response) = self.fetch_stream(repository, url, kind).await? else {
            return Ok(None);
        };
        let expected_length = response.content_length;
        let mut bytes = Vec::new();
        while let Some(chunk) = response.body.next().await {
            let chunk = chunk?;
            if bytes.len().saturating_add(chunk.len()) > 16 * 1024 * 1024 {
                return Err(Error::Transport {
                    url: url.to_string(),
                    message: "metadata or POM exceeded the 16 MiB safety limit".to_string(),
                });
            }
            bytes.extend_from_slice(&chunk);
        }
        validate_response_length(url, expected_length, bytes.len())?;
        self.session_cache
            .lock()
            .map_err(|error| Error::InvalidConfiguration(error.to_string()))?
            .insert(cache_key, Arc::new(bytes.clone()));
        #[cfg(not(target_family = "wasm"))]
        if let Some(cache) = &self.file_cache {
            cache.store_small(repository, url, &bytes).await?;
        }
        Ok(Some(bytes))
    }

    async fn fetch_stream(
        &self,
        repository: &RemoteRepository,
        url: &str,
        kind: ResourceKind,
    ) -> Result<Option<crate::TransportResponse>> {
        repository.validate_resource_url(url)?;
        #[cfg(not(target_family = "wasm"))]
        if let Some(cache) = &self.file_cache
            && let Some(response) = cache.response(repository, url, kind, self.offline).await?
        {
            return Ok(Some(response));
        }
        let scheme = Url::parse(url)?.scheme().to_string();
        if self.offline && scheme != "file" {
            return Err(Error::Offline(url.to_string()));
        }
        let transport = self
            .transports
            .iter()
            .rev()
            .find(|transport| transport.supports(&scheme))
            .ok_or_else(|| Error::UnsupportedTransport(scheme.clone()))?;
        transport
            .get(&TransportRequest {
                url: url.to_string(),
                repository: repository.clone(),
                kind,
            })
            .await
    }

    async fn fetch_checksum(
        &self,
        repository: &RemoteRepository,
        artifact_url: &str,
    ) -> Result<Option<VerifiedChecksum>> {
        for (extension, algorithm) in [
            ("sha512", "SHA-512"),
            ("sha256", "SHA-256"),
            ("sha1", "SHA-1"),
            ("md5", "MD5"),
        ] {
            let url = format!("{artifact_url}.{extension}");
            let bytes = match self
                .fetch_small(repository, &url, ResourceKind::Checksum)
                .await
            {
                Ok(bytes) => bytes,
                Err(Error::Offline(_)) => None,
                Err(error) => return Err(error),
            };
            if let Some(bytes) = bytes {
                let value = parse_checksum(&bytes, algorithm).ok_or_else(|| Error::Transport {
                    url,
                    message: format!("malformed {algorithm} checksum"),
                })?;
                return Ok(Some(VerifiedChecksum {
                    algorithm: algorithm.to_string(),
                    value,
                }));
            }
        }
        Ok(None)
    }

    async fn snapshot_file_version(
        &self,
        coordinate: &ArtifactCoordinate,
        repository: &RemoteRepository,
        extension: &str,
        classifier: Option<&str>,
    ) -> Result<String> {
        if !coordinate.version.ends_with("-SNAPSHOT") {
            return Ok(coordinate.version.clone());
        }
        let url = repository.resource_url(&coordinate.snapshot_metadata_path())?;
        let Some(bytes) = self
            .fetch_small(repository, &url, ResourceKind::SnapshotMetadata)
            .await?
        else {
            return Ok(coordinate.version.clone());
        };
        let metadata: RepositoryMetadata = quick_xml::de::from_reader(Cursor::new(bytes))?;
        if !metadata.matches_coordinate(coordinate) {
            return Err(Error::Transport {
                url,
                message: format!(
                    "snapshot metadata coordinates do not match requested artifact {coordinate}"
                ),
            });
        }
        let file_version = metadata
            .snapshot_version(&coordinate.version, extension, classifier)
            .unwrap_or_else(|| coordinate.version.clone());
        drop(ArtifactCoordinate::from_key(
            coordinate.key.clone(),
            file_version.clone(),
        )?);
        Ok(file_version)
    }

    async fn artifact_for_repository(
        &self,
        artifact: &ResolvedArtifact,
        repository: &RemoteRepository,
    ) -> Result<ResolvedArtifact> {
        let file_version = self
            .snapshot_file_version(
                &artifact.coordinate,
                repository,
                &artifact.coordinate.key.extension,
                artifact.coordinate.key.classifier.as_deref(),
            )
            .await?;
        let path = artifact
            .coordinate
            .artifact_path_with_version(&file_version);
        Ok(ResolvedArtifact {
            node: artifact.node,
            coordinate: artifact.coordinate.clone(),
            file_version,
            repository_id: repository.id.clone(),
            url: repository.resource_url(&path)?,
        })
    }
}

/// One dependency after interpolation, inheritance, and dependency management.
#[derive(Debug, Clone)]
pub struct EffectiveDependency {
    /// Conflict key after type/classifier mapping.
    pub key: ArtifactKey,
    /// Effective version requirement.
    pub version: Option<String>,
    /// Effective dependency scope.
    pub scope: DependencyScope,
    /// Whether the scope was explicitly declared.
    pub scope_explicit: bool,
    /// Interpolated path for a system dependency.
    pub system_path: Option<String>,
    /// Whether the dependency is optional.
    pub optional: bool,
    /// Whether optionality was explicitly declared.
    pub optional_explicit: bool,
    /// Effective `(groupId, artifactId)` exclusions.
    pub exclusions: BTreeSet<(String, String)>,
}

/// Dependency-relevant effective Maven model.
#[derive(Debug, Clone)]
pub struct EffectiveModel {
    /// Effective project coordinate.
    pub coordinate: ArtifactCoordinate,
    /// Effective packaging.
    pub packaging: String,
    /// Effective interpolated properties.
    pub properties: BTreeMap<String, String>,
    /// Effective dependency-management entries.
    pub dependency_management: BTreeMap<ArtifactKey, EffectiveDependency>,
    /// Effective direct dependencies.
    pub dependencies: Vec<EffectiveDependency>,
    /// Effective repositories after parent/profile merging.
    pub repositories: Vec<RemoteRepository>,
    /// Effective relocation, when declared.
    pub relocation: Option<ArtifactCoordinate>,
    /// Repository that supplied this model, if remote.
    pub repository_id: Option<String>,
}

#[derive(Debug)]
struct ResolutionSession<'a> {
    resolver: &'a Resolver,
    request: &'a ResolutionRequest,
    model_cache: BTreeMap<ArtifactCoordinate, Arc<EffectiveModel>>,
    forced_versions: BTreeMap<ArtifactKey, Version>,
}

impl<'a> ResolutionSession<'a> {
    fn new(resolver: &'a Resolver, request: &'a ResolutionRequest) -> Self {
        Self {
            resolver,
            request,
            model_cache: BTreeMap::new(),
            forced_versions: BTreeMap::new(),
        }
    }

    async fn resolve(&mut self) -> Result<Resolution> {
        let mut attempted_selections = Vec::new();
        loop {
            match self.resolve_pass().await? {
                ResolutionPass::Complete(resolution) => return Ok(resolution),
                ResolutionPass::Restart { artifact, versions } => {
                    let restart = (artifact, versions);
                    let state = (&mut attempted_selections, &self.forced_versions);
                    record_selection_attempt(state, restart)?;
                }
            }
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "breadth-first graph collection keeps mediation state and annotation updates together"
    )]
    async fn resolve_pass(&mut self) -> Result<ResolutionPass> {
        let mut resolution = Resolution {
            roots: Vec::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
            artifacts: Vec::new(),
            repositories: self.resolver.apply_mirrors(&self.resolver.repositories),
            diagnostics: Vec::new(),
            repository_fallback: true,
        };
        let mut queue = VecDeque::new();
        let mut selected = BTreeMap::<ArtifactKey, NodeId>::new();
        let mut selected_versions = BTreeMap::<ArtifactKey, Version>::new();
        let mut fixed_root_keys = BTreeSet::<ArtifactKey>::new();
        let mut range_constraints = BTreeMap::<ArtifactKey, Vec<(String, VersionSpec)>>::new();
        let mut constraint_repositories = BTreeMap::<ArtifactKey, Vec<RemoteRepository>>::new();
        for root in &self.request.roots {
            let (model, include_artifact, local_project, selected_root) =
                self.load_root(root).await?;
            let node_id = NodeId(resolution.nodes.len());
            let requested_root = match root {
                ResolutionRoot::Artifact(coordinate) => Some(coordinate.clone()),
                ResolutionRoot::ProjectBytes { .. } => Some(model.coordinate.clone()),
                #[cfg(not(target_family = "wasm"))]
                ResolutionRoot::ProjectFile(_) => Some(model.coordinate.clone()),
            };
            let mut root_events = Vec::new();
            if let Some(requested) = &requested_root
                && requested.version != selected_root.version
            {
                root_events.push(ResolutionEvent::VersionSelected {
                    specification: requested.version.clone(),
                    selected: selected_root.version.clone(),
                });
            }
            if selected_root != model.coordinate {
                root_events.push(ResolutionEvent::Relocated {
                    from: selected_root.clone(),
                    to: model.coordinate.clone(),
                });
            }
            let selection_keys = vec![selected_root.key.clone(), model.coordinate.key.clone()];
            let winner = selection_keys
                .iter()
                .find_map(|key| selected.get(key).copied());
            if let Some(winner) = winner {
                root_events.push(ResolutionEvent::Mediated { winner });
            } else {
                for key in selection_keys {
                    let version = if key == selected_root.key {
                        &selected_root.version
                    } else {
                        &model.coordinate.version
                    };
                    selected.insert(key.clone(), node_id);
                    selected_versions.insert(key, Version::new(version));
                }
            }
            if let ResolutionRoot::Artifact(requested) = root {
                let specification: VersionSpec = requested.version.parse()?;
                if matches!(specification, VersionSpec::Ranges(_)) {
                    range_constraints
                        .entry(requested.key.clone())
                        .or_default()
                        .push((requested.version.clone(), specification));
                    retain_repositories(
                        constraint_repositories
                            .entry(requested.key.clone())
                            .or_default(),
                        &self.resolver.repositories,
                    );
                } else if winner.is_none() && specification.exact().is_some() {
                    fixed_root_keys.insert(requested.key.clone());
                }
            }
            resolution.roots.push(node_id);
            resolution.nodes.push(DependencyNode {
                id: node_id,
                coordinate: model.coordinate.clone(),
                requested: requested_root,
                scope: DependencyScope::Compile,
                optional: false,
                depth: 0,
                status: winner.map_or(NodeStatus::Selected, |winner| NodeStatus::Conflict {
                    winner,
                }),
                events: root_events,
                repository_id: model.repository_id.clone(),
            });
            if winner.is_none() && include_artifact {
                let artifacts = self
                    .artifacts_for(node_id, &model, &resolution.repositories)
                    .await?;
                if let Some(repository_id) = artifacts
                    .first()
                    .map(|artifact| artifact.repository_id.clone())
                    && let Some(node) = resolution.nodes.get_mut(node_id.0)
                {
                    node.repository_id = Some(repository_id);
                }
                resolution.artifacts.extend(artifacts);
            }
            let repositories = if local_project || self.request.transitive_repositories {
                self.repositories_with_declared(&resolution.repositories, &model.repositories)
            } else {
                self.resolver.apply_mirrors(&resolution.repositories)
            };
            retain_repositories(&mut resolution.repositories, &repositories);
            if winner.is_none() {
                for dependency in &model.dependencies {
                    queue.push_back(PendingDependency {
                        parent: node_id,
                        dependency: dependency.clone(),
                        depth: 1,
                        parent_scope: DependencyScope::Compile,
                        exclusions: BTreeSet::new(),
                        ancestry: vec![model.coordinate.clone()],
                        repositories: repositories.clone(),
                        override_eligible: !local_project,
                        management: model.dependency_management.clone(),
                    });
                }
            }
        }

        while let Some(pending) = queue.pop_front() {
            let excluded_by_parent =
                exclusion_matches(&pending.exclusions, &pending.dependency.key);
            let mut dependency = pending.dependency.clone();
            let requested = dependency.version.as_ref().and_then(|version| {
                ArtifactCoordinate::from_key(dependency.key.clone(), version.clone()).ok()
            });
            let mut events = Vec::new();
            if pending.depth > 1
                && let Some(managed) = pending.management.get(&dependency.key)
            {
                if let Some(version) = &managed.version {
                    events.push(ResolutionEvent::Managed {
                        requested: dependency.version.clone(),
                        selected: version.clone(),
                    });
                    dependency.version = Some(version.clone());
                }
                if managed.scope_explicit {
                    dependency.scope = managed.scope;
                }
                if managed.optional_explicit {
                    dependency.optional = managed.optional;
                }
                dependency.exclusions.extend(managed.exclusions.clone());
            }
            let scope = if pending.depth == 1 || !pending.override_eligible {
                dependency.scope
            } else {
                let Some(scope) = derive_scope(pending.parent_scope, dependency.scope) else {
                    continue;
                };
                scope
            };
            if !self.request.classpath.includes(scope) {
                continue;
            }
            if excluded_by_parent {
                let coordinate = ArtifactCoordinate::from_key(
                    dependency.key.clone(),
                    dependency
                        .version
                        .clone()
                        .unwrap_or_else(|| "*".to_string()),
                )?;
                let node_id = NodeId(resolution.nodes.len());
                resolution.edges.push(DependencyEdge {
                    from: pending.parent,
                    to: node_id,
                });
                resolution.nodes.push(DependencyNode {
                    id: node_id,
                    coordinate,
                    requested,
                    scope,
                    optional: dependency.optional,
                    depth: pending.depth,
                    status: NodeStatus::Excluded,
                    events,
                    repository_id: None,
                });
                continue;
            }
            if pending.override_eligible
                && let Some(rule) = matching_override(&self.request.overrides, &dependency.key)
            {
                match &rule.action {
                    OverrideAction::ForceVersion(version) => {
                        dependency.version = Some(version.to_string());
                        events.push(ResolutionEvent::Override {
                            action: format!("forced version {version}"),
                        });
                    }
                    OverrideAction::Replace(coordinate) => {
                        dependency.key = coordinate.key.clone();
                        dependency.version = Some(coordinate.version.clone());
                        events.push(ResolutionEvent::Override {
                            action: format!("replaced with {coordinate}"),
                        });
                    }
                    OverrideAction::Exclude => {
                        let coordinate = ArtifactCoordinate::from_key(
                            dependency.key.clone(),
                            dependency
                                .version
                                .clone()
                                .unwrap_or_else(|| "*".to_string()),
                        )?;
                        let node_id = NodeId(resolution.nodes.len());
                        events.push(ResolutionEvent::Override {
                            action: "excluded".to_string(),
                        });
                        resolution.edges.push(DependencyEdge {
                            from: pending.parent,
                            to: node_id,
                        });
                        resolution.nodes.push(DependencyNode {
                            id: node_id,
                            coordinate,
                            requested,
                            scope,
                            optional: dependency.optional,
                            depth: pending.depth,
                            status: NodeStatus::OverriddenOut,
                            events,
                            repository_id: None,
                        });
                        continue;
                    }
                }
            }
            let Some(specification) = dependency.version.clone() else {
                let parent = graph_node_coordinate(&resolution, pending.parent)?;
                return Err(Error::Model {
                    coordinate: parent,
                    message: format!("dependency {} has no managed version", dependency.key),
                });
            };
            if scope == DependencyScope::System {
                if pending.override_eligible {
                    let parent = graph_node_coordinate(&resolution, pending.parent)?;
                    return Err(Error::Model {
                        coordinate: parent,
                        message: format!(
                            "repository POM declares system-scoped dependency {}",
                            dependency.key
                        ),
                    });
                }
                #[cfg(target_family = "wasm")]
                return Err(Error::UnsupportedTransport("file".to_string()));
                #[cfg(not(target_family = "wasm"))]
                {
                    let specification_value: VersionSpec = specification.parse()?;
                    let parent_coordinate = graph_node_coordinate(&resolution, pending.parent)?;
                    let version = specification_value.exact().ok_or_else(|| Error::Model {
                        coordinate: parent_coordinate,
                        message: format!(
                            "system dependency {} must use an exact version",
                            dependency.key
                        ),
                    })?;
                    let coordinate =
                        ArtifactCoordinate::from_key(dependency.key.clone(), version.to_string())?;
                    let system_path = required_system_path(&dependency, &coordinate)?;
                    let mut path = std::path::PathBuf::from(system_path);
                    if path.is_relative() {
                        let base =
                            self.request
                                .context
                                .base_directory
                                .as_ref()
                                .ok_or_else(|| Error::Model {
                                    coordinate: coordinate.clone(),
                                    message: format!(
                                        "relative systemPath '{system_path}' has no base directory"
                                    ),
                                })?;
                        path = base.join(path);
                    }
                    if !path.is_file() {
                        return Err(Error::Model {
                            coordinate: coordinate.clone(),
                            message: format!("systemPath '{}' is not a file", path.display()),
                        });
                    }
                    let url = system_file_url(&path, &coordinate)?;
                    let node_id = NodeId(resolution.nodes.len());
                    let mut node = DependencyNode {
                        id: node_id,
                        coordinate: coordinate.clone(),
                        requested,
                        scope,
                        optional: dependency.optional,
                        depth: pending.depth,
                        status: NodeStatus::Selected,
                        events,
                        repository_id: Some(SYSTEM_REPOSITORY_ID.to_string()),
                    };
                    resolution.edges.push(DependencyEdge {
                        from: pending.parent,
                        to: node_id,
                    });
                    if let Some(winner) = selected.get(&coordinate.key).copied() {
                        node.status = NodeStatus::Conflict { winner };
                        node.events.push(ResolutionEvent::Mediated { winner });
                    } else {
                        selected.insert(coordinate.key.clone(), node_id);
                        selected_versions
                            .insert(coordinate.key.clone(), Version::new(&coordinate.version));
                        let repository = system_repository()?;
                        if !resolution
                            .repositories
                            .iter()
                            .any(|known| known.id == repository.id)
                        {
                            resolution.repositories.push(repository);
                        }
                        if self.request.artifacts.main_artifacts {
                            resolution.artifacts.push(ResolvedArtifact {
                                node: node_id,
                                coordinate,
                                file_version: version.to_string(),
                                repository_id: SYSTEM_REPOSITORY_ID.to_string(),
                                url: url.to_string(),
                            });
                        }
                    }
                    resolution.nodes.push(node);
                    continue;
                }
            }
            let specification_value: VersionSpec = specification.parse()?;
            if dependency.optional && pending.override_eligible {
                let coordinate = ArtifactCoordinate::from_key(
                    dependency.key.clone(),
                    specification_value.to_string(),
                )?;
                let node_id = NodeId(resolution.nodes.len());
                resolution.edges.push(DependencyEdge {
                    from: pending.parent,
                    to: node_id,
                });
                resolution.nodes.push(DependencyNode {
                    id: node_id,
                    coordinate,
                    requested,
                    scope,
                    optional: true,
                    depth: pending.depth,
                    status: NodeStatus::Optional,
                    events,
                    repository_id: None,
                });
                continue;
            }
            if matches!(specification_value, VersionSpec::Ranges(_)) {
                let constraints = range_constraints.entry(dependency.key.clone()).or_default();
                if !constraints.iter().any(|(known, _)| known == &specification) {
                    constraints.push((specification.clone(), specification_value.clone()));
                }
                let repositories = constraint_repositories
                    .entry(dependency.key.clone())
                    .or_default();
                retain_repositories(repositories, &pending.repositories);
            }
            let coordinate = if let Some(version) = self.forced_versions.get(&dependency.key) {
                events.push(ResolutionEvent::VersionSelected {
                    specification: specification.clone(),
                    selected: version.to_string(),
                });
                ArtifactCoordinate::from_key(dependency.key.clone(), version.to_string())?
            } else if specification_value.exact().is_none()
                && let Some(winner) = selected.get(&dependency.key)
                && let Some(winner) = resolution.nodes.get(winner.0)
            {
                events.push(ResolutionEvent::VersionSelected {
                    specification: specification.clone(),
                    selected: winner.coordinate.version.clone(),
                });
                coordinate_with_version(&dependency.key, &winner.coordinate.version)?
            } else {
                self.select_coordinate(
                    &dependency.key,
                    &specification,
                    &pending.repositories,
                    &mut events,
                )
                .await?
            };
            let node_id = NodeId(resolution.nodes.len());
            let mut node = DependencyNode {
                id: node_id,
                coordinate: coordinate.clone(),
                requested,
                scope,
                optional: dependency.optional,
                depth: pending.depth,
                status: NodeStatus::Selected,
                events,
                repository_id: None,
            };
            resolution.edges.push(DependencyEdge {
                from: pending.parent,
                to: node_id,
            });
            if pending.ancestry.contains(&coordinate) {
                node.status = NodeStatus::Cycle;
                resolution.nodes.push(node);
                continue;
            }
            if let Some(winner) = selected.get(&coordinate.key).copied() {
                node.status = NodeStatus::Conflict { winner };
                node.events.push(ResolutionEvent::Mediated { winner });
                resolution.nodes.push(node);
                continue;
            }

            let model = self
                .effective_model(
                    &coordinate,
                    &pending.repositories,
                    &mut pending.ancestry.clone(),
                )
                .await?;
            if let Some(relocated) = &model.relocation {
                node.events.push(ResolutionEvent::Relocated {
                    from: coordinate.clone(),
                    to: relocated.clone(),
                });
                node.coordinate = relocated.clone();
            }
            node.repository_id = model.repository_id.clone();
            if let Some(winner) = selected.get(&node.coordinate.key).copied() {
                node.status = NodeStatus::Conflict { winner };
                node.events.push(ResolutionEvent::Mediated { winner });
                resolution.nodes.push(node);
                continue;
            }
            selected.insert(coordinate.key.clone(), node_id);
            selected.insert(node.coordinate.key.clone(), node_id);
            selected_versions.insert(coordinate.key.clone(), Version::new(&coordinate.version));
            if node.coordinate.key != coordinate.key {
                selected_versions.insert(
                    node.coordinate.key.clone(),
                    Version::new(&node.coordinate.version),
                );
            }
            resolution.nodes.push(node);
            resolution.artifacts.extend(
                self.artifacts_for(node_id, &model, &pending.repositories)
                    .await?,
            );
            let mut exclusions = pending.exclusions;
            exclusions.extend(dependency.exclusions);
            let mut ancestry = pending.ancestry;
            ancestry.push(model.coordinate.clone());
            let repositories = self.repositories_for_model(&pending.repositories, &model);
            retain_repositories(&mut resolution.repositories, &repositories);
            for child in &model.dependencies {
                queue.push_back(PendingDependency {
                    parent: node_id,
                    dependency: child.clone(),
                    depth: pending.depth + 1,
                    parent_scope: scope,
                    exclusions: exclusions.clone(),
                    ancestry: ancestry.clone(),
                    repositories: repositories.clone(),
                    override_eligible: true,
                    management: pending.management.clone(),
                });
            }
        }
        for (key, constraints) in range_constraints {
            let selected_version =
                selected_constraint_version(&selected, &selected_versions, &key)?;
            if constraints
                .iter()
                .all(|(_, constraint)| constraint.matches(&selected_version))
            {
                continue;
            }
            let versions = constraints
                .iter()
                .map(|(text, _)| text.clone())
                .collect::<Vec<_>>();
            if fixed_root_keys.contains(&key) {
                return Err(Error::VersionConflict {
                    artifact: key.to_string(),
                    versions,
                });
            }
            let repositories = constraint_repositories
                .get(&key)
                .map(Vec::as_slice)
                .unwrap_or_default();
            let candidate = self
                .select_version_matching_all(&key, &constraints, repositories)
                .await?
                .ok_or_else(|| Error::VersionConflict {
                    artifact: key.to_string(),
                    versions: versions.clone(),
                })?;
            self.forced_versions.insert(key.clone(), candidate);
            return Ok(ResolutionPass::Restart {
                artifact: key.to_string(),
                versions,
            });
        }
        Ok(ResolutionPass::Complete(resolution))
    }

    async fn load_root(
        &mut self,
        root: &ResolutionRoot,
    ) -> Result<(Arc<EffectiveModel>, bool, bool, ArtifactCoordinate)> {
        match root {
            ResolutionRoot::Artifact(coordinate) => {
                let selected_coordinate =
                    if let Some(version) = self.forced_versions.get(&coordinate.key) {
                        ArtifactCoordinate::from_key(coordinate.key.clone(), version.to_string())?
                    } else {
                        self.select_coordinate(
                            &coordinate.key,
                            &coordinate.version,
                            &self.resolver.repositories,
                            &mut Vec::new(),
                        )
                        .await?
                    };
                let model = self
                    .effective_model(
                        &selected_coordinate,
                        &self.resolver.repositories,
                        &mut Vec::new(),
                    )
                    .await?;
                Ok((model, true, false, selected_coordinate))
            }
            ResolutionRoot::ProjectBytes {
                bytes,
                include_artifact,
            } => {
                let project = Project::from_reader(Cursor::new(bytes))?;
                let model = self
                    .build_effective_model(
                        project,
                        None,
                        None,
                        &self.resolver.repositories,
                        &mut Vec::new(),
                    )
                    .await?;
                let coordinate = model.coordinate.clone();
                Ok((Arc::new(model), *include_artifact, true, coordinate))
            }
            #[cfg(not(target_family = "wasm"))]
            ResolutionRoot::ProjectFile(path) => {
                let bytes = tokio::fs::read(path).await?;
                let project = Project::from_reader(Cursor::new(bytes))?;
                let mut context = self.request.context.clone();
                context.base_directory = path.parent().map(std::path::Path::to_path_buf);
                let model = self
                    .build_effective_model_with_context(
                        project,
                        None,
                        None,
                        &self.resolver.repositories,
                        &mut Vec::new(),
                        &context,
                    )
                    .await?;
                let coordinate = model.coordinate.clone();
                Ok((Arc::new(model), false, true, coordinate))
            }
        }
    }

    fn effective_model<'b>(
        &'b mut self,
        coordinate: &'b ArtifactCoordinate,
        repositories: &'b [RemoteRepository],
        stack: &'b mut Vec<ArtifactCoordinate>,
    ) -> BoxFuture<'b, Result<Arc<EffectiveModel>>> {
        Box::pin(async move {
            if let Some(model) = self.model_cache.get(coordinate) {
                return Ok(model.clone());
            }
            if stack.contains(coordinate) {
                return Err(Error::Cycle(format!(
                    "{} -> {coordinate}",
                    stack
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" -> ")
                )));
            }
            stack.push(coordinate.clone());
            let (project, repository_id) = self.fetch_project(coordinate, repositories).await?;
            let mut model = self
                .build_effective_model(
                    project,
                    Some(repository_id),
                    Some(coordinate.clone()),
                    repositories,
                    stack,
                )
                .await?;
            if !same_gav(&model.coordinate, coordinate) {
                return Err(Error::Model {
                    coordinate: coordinate.clone(),
                    message: format!(
                        "repository POM declares {}, which does not match the requested coordinate",
                        model.coordinate
                    ),
                });
            }
            if let Some(relocated) = model.relocation.clone()
                && relocated != model.coordinate
            {
                let mut relocated_model = (*self
                    .effective_model(&relocated, repositories, stack)
                    .await?)
                    .clone();
                relocated_model.relocation = Some(relocated_model.coordinate.clone());
                model = relocated_model;
            }
            drop(stack.pop());
            let model = Arc::new(model);
            self.model_cache.insert(coordinate.clone(), model.clone());
            Ok(model)
        })
    }

    fn build_effective_model<'b>(
        &'b mut self,
        project: Project,
        repository_id: Option<String>,
        expected_coordinate: Option<ArtifactCoordinate>,
        repositories: &'b [RemoteRepository],
        stack: &'b mut Vec<ArtifactCoordinate>,
    ) -> BoxFuture<'b, Result<EffectiveModel>> {
        let mut context = self.request.context.clone();
        if repository_id.is_some() {
            context.base_directory = None;
        }
        Box::pin(async move {
            self.build_effective_model_with_context(
                project,
                repository_id,
                expected_coordinate,
                repositories,
                stack,
                &context,
            )
            .await
        })
    }

    #[expect(
        clippy::too_many_lines,
        reason = "effective model construction follows the ordered parent/profile/BOM interpolation pipeline"
    )]
    fn build_effective_model_with_context<'b>(
        &'b mut self,
        project: Project,
        repository_id: Option<String>,
        expected_coordinate: Option<ArtifactCoordinate>,
        repositories: &'b [RemoteRepository],
        stack: &'b mut Vec<ArtifactCoordinate>,
        context: &'b ResolutionContext,
    ) -> BoxFuture<'b, Result<EffectiveModel>> {
        Box::pin(async move {
            // Maven injects raw-model profiles and makes repositories from the
            // current model available before it attempts external parent
            // resolution. At this point parent properties are unavailable, but
            // project, user, and environment properties can still be expanded.
            let mut raw_properties = project.properties.clone();
            raw_properties.extend(context.properties.clone());
            let raw_packaging = interpolate(
                project.packaging.as_deref().unwrap_or("jar"),
                &raw_properties,
                context,
            )?;
            let raw_active_profiles =
                active_profiles(&project, &raw_packaging, context, &raw_properties);
            for profile in &raw_active_profiles {
                raw_properties.extend(profile.properties.clone());
            }
            let mut raw_model_repositories = Vec::new();
            append_pom_repositories(
                &mut raw_model_repositories,
                project.repositories.as_ref(),
                &raw_properties,
                context,
            )?;
            for profile in raw_active_profiles {
                append_pom_repositories(
                    &mut raw_model_repositories,
                    profile.repositories.as_ref(),
                    &raw_properties,
                    context,
                )?;
            }
            let may_use_declared_repositories =
                repository_id.is_none() || self.request.transitive_repositories;
            let parent_repositories = if may_use_declared_repositories {
                self.repositories_with_declared(repositories, &raw_model_repositories)
            } else {
                self.resolver.apply_mirrors(repositories)
            };

            let parent_model = if let Some(parent) = &project.parent {
                let mut initial_properties = project.properties.clone();
                initial_properties.extend(context.properties.clone());
                let group = interpolate(&parent.group_id, &initial_properties, context)?;
                let artifact = interpolate(&parent.artifact_id, &initial_properties, context)?;
                let version = interpolate(&parent.version, &initial_properties, context)?;
                let parent_key = ArtifactKey::new(group, artifact)?.with_extension("pom")?;
                #[cfg(not(target_family = "wasm"))]
                let parent_specification: VersionSpec = version.parse()?;
                #[cfg(not(target_family = "wasm"))]
                let local_parent = if let Some(base_directory) = &context.base_directory {
                    let relative_path = parent.relative_path.as_deref().unwrap_or("../pom.xml");
                    if relative_path.is_empty() {
                        None
                    } else {
                        let mut parent_path = base_directory.join(relative_path);
                        if parent_path.is_dir() {
                            parent_path = parent_path.join("pom.xml");
                        }
                        if tokio::fs::try_exists(&parent_path).await? {
                            let bytes = tokio::fs::read(&parent_path).await?;
                            let parent_project: Project =
                                quick_xml::de::from_reader(Cursor::new(bytes))?;
                            if let Some(candidate_coordinate) =
                                raw_project_coordinate(&parent_project, context)
                                && candidate_coordinate.key.group_id == parent_key.group_id
                                && candidate_coordinate.key.artifact_id == parent_key.artifact_id
                                && parent_specification
                                    .matches(&Version::new(&candidate_coordinate.version))
                            {
                                parent_project.validate()?;
                                if stack.contains(&candidate_coordinate) {
                                    return Err(Error::Cycle(candidate_coordinate.to_string()));
                                }
                                stack.push(candidate_coordinate);
                                let mut parent_context = context.clone();
                                parent_context.base_directory =
                                    parent_path.parent().map(std::path::Path::to_path_buf);
                                let parent_model = self
                                    .build_effective_model_with_context(
                                        parent_project,
                                        None,
                                        None,
                                        &parent_repositories,
                                        stack,
                                        &parent_context,
                                    )
                                    .await?;
                                drop(stack.pop());
                                if parent_model.coordinate.key.group_id == parent_key.group_id
                                    && parent_model.coordinate.key.artifact_id
                                        == parent_key.artifact_id
                                    && parent_specification
                                        .matches(&Version::new(&parent_model.coordinate.version))
                                {
                                    Some(Arc::new(parent_model))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                } else {
                    None
                };
                #[cfg(target_family = "wasm")]
                let local_parent: Option<Arc<EffectiveModel>> = None;

                if let Some(local_parent) = local_parent {
                    Some(local_parent)
                } else {
                    let parent_coordinate = self
                        .select_coordinate(
                            &parent_key,
                            &version,
                            &parent_repositories,
                            &mut Vec::new(),
                        )
                        .await?;
                    Some(
                        self.effective_model(&parent_coordinate, &parent_repositories, stack)
                            .await?,
                    )
                }
            } else {
                None
            };
            if let Some(parent) = &parent_model
                && parent.packaging != "pom"
            {
                return Err(Error::Model {
                    coordinate: parent.coordinate.clone(),
                    message: format!(
                        "parent POM must use 'pom' packaging, found '{}'",
                        parent.packaging
                    ),
                });
            }

            let parent_coordinate = parent_model.as_ref().map(|model| &model.coordinate);
            let group_id = project
                .group_id
                .clone()
                .or_else(|| parent_coordinate.map(|coordinate| coordinate.key.group_id.clone()))
                .ok_or_else(|| Error::InvalidConfiguration("project has no groupId".to_string()))?;
            let version = project
                .version
                .clone()
                .or_else(|| parent_coordinate.map(|coordinate| coordinate.version.clone()))
                .ok_or_else(|| Error::InvalidConfiguration("project has no version".to_string()))?;

            let mut properties = parent_model
                .as_ref()
                .map_or_else(BTreeMap::new, |model| model.properties.clone());
            properties.extend(project.properties.clone());
            let preliminary_coordinate =
                ArtifactCoordinate::new(group_id, project.artifact_id.clone(), version)?;
            add_project_properties(&mut properties, &preliminary_coordinate, parent_coordinate);
            properties.extend(context.properties.clone());
            let preliminary_packaging = interpolate(
                project.packaging.as_deref().unwrap_or("jar"),
                &properties,
                context,
            )?;
            properties.insert(
                "project.packaging".to_string(),
                preliminary_packaging.clone(),
            );
            properties.insert("pom.packaging".to_string(), preliminary_packaging.clone());

            let active_profiles =
                active_profiles(&project, &preliminary_packaging, context, &properties);
            for profile in &active_profiles {
                properties.extend(profile.properties.clone());
            }

            let group_id = interpolate(&preliminary_coordinate.key.group_id, &properties, context)?;
            let artifact_id = interpolate(
                &preliminary_coordinate.key.artifact_id,
                &properties,
                context,
            )?;
            let version = interpolate(&preliminary_coordinate.version, &properties, context)?;
            let packaging = interpolate(
                &project
                    .packaging
                    .clone()
                    .unwrap_or_else(|| "jar".to_string()),
                &properties,
                context,
            )?;
            let key = if let Some(expected) = &expected_coordinate {
                let mut key = expected.key.clone();
                key.group_id = group_id;
                key.artifact_id = artifact_id;
                key
            } else {
                let extension = artifact_type(&packaging).0;
                ArtifactKey::new(group_id, artifact_id)?.with_extension(extension)?
            };
            let coordinate = ArtifactCoordinate::from_key(key, version)?;
            add_project_properties(&mut properties, &coordinate, parent_coordinate);

            let mut model_repositories = parent_model
                .as_ref()
                .map_or_else(Vec::new, |model| model.repositories.clone());
            append_pom_repositories(
                &mut model_repositories,
                project.repositories.as_ref(),
                &properties,
                context,
            )?;
            for profile in &active_profiles {
                append_pom_repositories(
                    &mut model_repositories,
                    profile.repositories.as_ref(),
                    &properties,
                    context,
                )?;
            }
            let import_repositories = if may_use_declared_repositories {
                self.repositories_with_declared(repositories, &model_repositories)
            } else {
                self.resolver.apply_mirrors(repositories)
            };

            let mut management = parent_model
                .as_ref()
                .map_or_else(BTreeMap::new, |model| model.dependency_management.clone());
            let mut imported_management = BTreeMap::new();
            let mut management_sources = Vec::new();
            if let Some(dependency_management) = &project.dependency_management {
                management_sources.push(dependency_management);
            }
            for profile in &active_profiles {
                if let Some(dependency_management) = &profile.dependency_management {
                    management_sources.push(dependency_management);
                }
            }

            for source in &management_sources {
                for dependency in &source.dependencies.dependencies {
                    let effective = dependency_from_pom(dependency, &properties, context)?;
                    if effective.scope == DependencyScope::Import
                        && effective.key.extension == "pom"
                    {
                        let version = effective.version.clone().ok_or_else(|| Error::Model {
                            coordinate: coordinate.clone(),
                            message: format!("imported BOM {} has no version", effective.key),
                        })?;
                        let bom_coordinate = self
                            .select_coordinate(
                                &effective.key,
                                &version,
                                &import_repositories,
                                &mut Vec::new(),
                            )
                            .await?;
                        let bom = self
                            .effective_model(&bom_coordinate, &import_repositories, stack)
                            .await?;
                        for (key, managed) in &bom.dependency_management {
                            if !exclusion_matches(&effective.exclusions, key) {
                                imported_management
                                    .entry(key.clone())
                                    .or_insert_with(|| managed.clone());
                            }
                        }
                    }
                }
            }
            // Entries imported in the current model outrank inherited
            // dependency management, while the first imported BOM wins when
            // multiple imports manage the same key.
            management.extend(imported_management);
            for source in management_sources {
                for dependency in &source.dependencies.dependencies {
                    let effective = dependency_from_pom(dependency, &properties, context)?;
                    if !(effective.scope == DependencyScope::Import
                        && effective.key.extension == "pom")
                    {
                        management.insert(effective.key.clone(), effective);
                    }
                }
            }

            let mut dependencies = parent_model
                .as_ref()
                .map_or_else(Vec::new, |model| model.dependencies.clone());
            merge_dependencies(
                &mut dependencies,
                &project.dependencies,
                &management,
                &properties,
                context,
            )?;
            for profile in active_profiles {
                if let Some(profile_dependencies) = &profile.dependencies {
                    merge_dependencies(
                        &mut dependencies,
                        profile_dependencies,
                        &management,
                        &properties,
                        context,
                    )?;
                }
            }

            let relocation = project
                .distribution_management
                .as_ref()
                .and_then(|management| management.relocation.as_ref())
                .map(|relocation| {
                    let group_id = relocation
                        .group_id
                        .as_deref()
                        .map(|value| interpolate(value, &properties, context))
                        .transpose()?
                        .unwrap_or_else(|| coordinate.key.group_id.clone());
                    let artifact_id = relocation
                        .artifact_id
                        .as_deref()
                        .map(|value| interpolate(value, &properties, context))
                        .transpose()?
                        .unwrap_or_else(|| coordinate.key.artifact_id.clone());
                    let version = relocation
                        .version
                        .as_deref()
                        .map(|value| interpolate(value, &properties, context))
                        .transpose()?
                        .unwrap_or_else(|| coordinate.version.clone());
                    ArtifactCoordinate::new(group_id, artifact_id, version)
                })
                .transpose()?;

            Ok(EffectiveModel {
                coordinate,
                packaging,
                properties,
                dependency_management: management,
                dependencies,
                repositories: model_repositories,
                relocation,
                repository_id,
            })
        })
    }

    async fn fetch_project(
        &self,
        coordinate: &ArtifactCoordinate,
        repositories: &[RemoteRepository],
    ) -> Result<(Project, String)> {
        let repositories = self.resolver.apply_mirrors(repositories);
        for repository in repositories {
            if !repository.accepts_version(&coordinate.version) {
                continue;
            }
            let file_version = self
                .snapshot_file_version(coordinate, &repository, "pom", None)
                .await?;
            let path = coordinate.pom_path_with_version(&file_version);
            let url = repository.resource_url(&path)?;
            if let Some(bytes) = self
                .resolver
                .fetch_small(&repository, &url, ResourceKind::Pom)
                .await?
            {
                let project =
                    Project::from_reader(Cursor::new(bytes)).map_err(|error| Error::Model {
                        coordinate: coordinate.clone(),
                        message: error.to_string(),
                    })?;
                return Ok((project, repository.id));
            }
        }
        Err(Error::ArtifactNotFound(coordinate.clone()))
    }

    async fn select_coordinate(
        &self,
        key: &ArtifactKey,
        specification: &str,
        repositories: &[RemoteRepository],
        events: &mut Vec<ResolutionEvent>,
    ) -> Result<ArtifactCoordinate> {
        let specification_value: VersionSpec = specification.parse()?;
        if let Some(version) = specification_value.exact() {
            return ArtifactCoordinate::from_key(key.clone(), version.to_string());
        }
        let mut listing = VersionListing::default();
        for repository in self.resolver.apply_mirrors(repositories) {
            if !repository.releases.enabled && !repository.snapshots.enabled {
                continue;
            }
            let url = repository.resource_url(&key.metadata_path())?;
            let Some(bytes) = self
                .resolver
                .fetch_small(&repository, &url, ResourceKind::Metadata)
                .await?
            else {
                continue;
            };
            let metadata: RepositoryMetadata = quick_xml::de::from_reader(Cursor::new(bytes))?;
            validate_metadata_key(&metadata, key, &url)?;
            listing.merge(&repository, metadata);
        }
        let selected =
            listing
                .select(&specification_value)
                .ok_or_else(|| Error::VersionConflict {
                    artifact: key.to_string(),
                    versions: vec![specification.to_string()],
                })?;
        events.push(ResolutionEvent::VersionSelected {
            specification: specification.to_string(),
            selected: selected.to_string(),
        });
        ArtifactCoordinate::from_key(key.clone(), selected.to_string())
    }

    async fn select_version_matching_all(
        &self,
        key: &ArtifactKey,
        constraints: &[(String, VersionSpec)],
        repositories: &[RemoteRepository],
    ) -> Result<Option<Version>> {
        let mut listing = VersionListing::default();
        for repository in self.resolver.apply_mirrors(repositories) {
            if !repository.releases.enabled && !repository.snapshots.enabled {
                continue;
            }
            let url = repository.resource_url(&key.metadata_path())?;
            let Some(bytes) = self
                .resolver
                .fetch_small(&repository, &url, ResourceKind::Metadata)
                .await?
            else {
                continue;
            };
            let metadata: RepositoryMetadata = quick_xml::de::from_reader(Cursor::new(bytes))?;
            validate_metadata_key(&metadata, key, &url)?;
            listing.merge(&repository, metadata);
        }
        Ok(listing
            .versions
            .iter()
            .rev()
            .find(|available| {
                constraints
                    .iter()
                    .all(|(_, constraint)| constraint.matches(&available.version))
            })
            .map(|available| available.version.clone()))
    }

    async fn snapshot_file_version(
        &self,
        coordinate: &ArtifactCoordinate,
        repository: &RemoteRepository,
        extension: &str,
        classifier: Option<&str>,
    ) -> Result<String> {
        self.resolver
            .snapshot_file_version(coordinate, repository, extension, classifier)
            .await
    }

    async fn artifacts_for(
        &self,
        node: NodeId,
        model: &EffectiveModel,
        repositories: &[RemoteRepository],
    ) -> Result<Vec<ResolvedArtifact>> {
        let repositories = self.resolver.apply_mirrors(repositories);
        let repository = model.repository_id.as_ref().map_or_else(
            || {
                repositories
                    .iter()
                    .find(|repository| repository.accepts_version(&model.coordinate.version))
            },
            |repository_id| {
                repositories
                    .iter()
                    .find(|repository| &repository.id == repository_id)
            },
        );
        let Some(repository) = repository else {
            return Ok(Vec::new());
        };
        let mut selections = Vec::new();
        if self.request.artifacts.main_artifacts && model.packaging != "pom" {
            selections.push(model.coordinate.key.clone());
        }
        if self.request.artifacts.poms {
            let key = selected_artifact_key(&model.coordinate, "pom", None)?;
            if !selections.contains(&key) {
                selections.push(key);
            }
        }
        for (classifier, extension) in &self.request.artifacts.attachments {
            let key =
                selected_artifact_key(&model.coordinate, extension, Some(classifier.as_str()))?;
            if !selections.contains(&key) {
                selections.push(key);
            }
        }
        let mut artifacts = Vec::new();
        for key in selections {
            let coordinate = ArtifactCoordinate::from_key(key, model.coordinate.version.clone())?;
            let file_version = self
                .snapshot_file_version(
                    &coordinate,
                    repository,
                    &coordinate.key.extension,
                    coordinate.key.classifier.as_deref(),
                )
                .await?;
            let path = coordinate.artifact_path_with_version(&file_version);
            artifacts.push(ResolvedArtifact {
                node,
                coordinate,
                file_version,
                repository_id: repository.id.clone(),
                url: repository.resource_url(&path)?,
            });
        }
        Ok(artifacts)
    }

    fn repositories_for_model(
        &self,
        current: &[RemoteRepository],
        model: &EffectiveModel,
    ) -> Vec<RemoteRepository> {
        if self.request.transitive_repositories {
            self.repositories_with_declared(current, &model.repositories)
        } else {
            self.resolver.apply_mirrors(current)
        }
    }

    fn repositories_with_declared(
        &self,
        current: &[RemoteRepository],
        declared: &[RemoteRepository],
    ) -> Vec<RemoteRepository> {
        let mut repositories = self.resolver.apply_mirrors(current);
        for mut repository in self.resolver.apply_mirrors(declared) {
            if repositories
                .iter()
                .any(|known| known.id == repository.id && known.url == repository.url)
            {
                // Keep caller-supplied credentials and proxy state for the
                // same logical repository.
                continue;
            }
            if repositories
                .iter()
                .any(|known| same_repository_settings(known, &repository))
            {
                continue;
            }
            if repositories.iter().any(|known| known.id == repository.id) {
                // Artifact provenance must identify one concrete repository.
                // Preserve both same-id declarations under a deterministic,
                // credential-free internal alias instead of silently binding a
                // downloaded artifact to the wrong URL.
                let original_id = repository.id.clone();
                let mut suffix = 1_u64;
                loop {
                    let candidate = format!("{original_id}~declared-{suffix}");
                    if !repositories.iter().any(|known| known.id == candidate) {
                        repository.id = candidate;
                        break;
                    }
                    suffix = suffix.saturating_add(1);
                }
            }
            repositories.push(repository);
        }
        repositories
    }
}

enum ResolutionPass {
    Complete(Resolution),
    Restart {
        artifact: String,
        versions: Vec<String>,
    },
}

#[derive(Debug, Clone)]
struct PendingDependency {
    parent: NodeId,
    dependency: EffectiveDependency,
    depth: usize,
    parent_scope: DependencyScope,
    exclusions: BTreeSet<(String, String)>,
    ancestry: Vec<ArtifactCoordinate>,
    repositories: Vec<RemoteRepository>,
    override_eligible: bool,
    management: BTreeMap<ArtifactKey, EffectiveDependency>,
}

fn same_gav(left: &ArtifactCoordinate, right: &ArtifactCoordinate) -> bool {
    left.key.group_id == right.key.group_id
        && left.key.artifact_id == right.key.artifact_id
        && left.version == right.version
}

fn raw_project_coordinate(
    project: &Project,
    context: &ResolutionContext,
) -> Option<ArtifactCoordinate> {
    let mut properties = project.properties.clone();
    properties.extend(context.properties.clone());
    let group_id = project.group_id.as_deref().or_else(|| {
        project
            .parent
            .as_ref()
            .map(|parent| parent.group_id.as_str())
    })?;
    let version = project.version.as_deref().or_else(|| {
        project
            .parent
            .as_ref()
            .map(|parent| parent.version.as_str())
    })?;
    ArtifactCoordinate::new(
        interpolate(group_id, &properties, context).ok()?,
        interpolate(&project.artifact_id, &properties, context).ok()?,
        interpolate(version, &properties, context).ok()?,
    )
    .ok()
}

fn retain_repositories(destination: &mut Vec<RemoteRepository>, repositories: &[RemoteRepository]) {
    for repository in repositories {
        if !destination.iter().any(|known| known.id == repository.id) {
            destination.push(repository.clone());
        }
    }
}

fn same_repository_settings(left: &RemoteRepository, right: &RemoteRepository) -> bool {
    left.url == right.url
        && left.releases == right.releases
        && left.snapshots == right.snapshots
        && left.authentication == right.authentication
        && left.proxy == right.proxy
}

fn graph_node_coordinate(resolution: &Resolution, node: NodeId) -> Result<ArtifactCoordinate> {
    resolution
        .nodes
        .get(node.0)
        .map(|node| node.coordinate.clone())
        .ok_or_else(|| Error::InvalidConfiguration(format!("missing graph node {}", node.0)))
}

fn coordinate_with_version(key: &ArtifactKey, version: &str) -> Result<ArtifactCoordinate> {
    ArtifactCoordinate::from_key(key.clone(), version)
}

fn record_selection_attempt(
    state: (
        &mut Vec<BTreeMap<ArtifactKey, Version>>,
        &BTreeMap<ArtifactKey, Version>,
    ),
    restart: (String, Vec<String>),
) -> Result<()> {
    let (attempted, selected) = state;
    if attempted.contains(selected) {
        let (artifact, versions) = restart;
        Err(Error::VersionConflict { artifact, versions })
    } else {
        attempted.push(selected.clone());
        Ok(())
    }
}

fn selected_constraint_version(
    selected: &BTreeMap<ArtifactKey, NodeId>,
    versions: &BTreeMap<ArtifactKey, Version>,
    key: &ArtifactKey,
) -> Result<Version> {
    let winner = selected.get(key).copied().ok_or_else(|| {
        Error::InvalidConfiguration(format!(
            "missing selected graph node for constrained artifact {key}"
        ))
    })?;
    let version = versions.get(key).cloned().ok_or_else(|| {
        Error::InvalidConfiguration(format!(
            "missing selected version for graph node {}",
            winner.0
        ))
    })?;
    Ok(version)
}

fn selected_artifact_key(
    coordinate: &ArtifactCoordinate,
    extension: &str,
    classifier: Option<&str>,
) -> Result<ArtifactKey> {
    let key = ArtifactKey::new(&coordinate.key.group_id, &coordinate.key.artifact_id)?
        .with_extension(extension)?;
    classifier.map_or(Ok(key.clone()), |classifier| {
        key.with_classifier(classifier)
    })
}

#[cfg(not(target_family = "wasm"))]
fn required_system_path<'a>(
    dependency: &'a EffectiveDependency,
    coordinate: &ArtifactCoordinate,
) -> Result<&'a str> {
    dependency
        .system_path
        .as_deref()
        .ok_or_else(|| Error::Model {
            coordinate: coordinate.clone(),
            message: "system dependency has no systemPath".to_string(),
        })
}

#[cfg(not(target_family = "wasm"))]
fn system_file_url(path: &std::path::Path, coordinate: &ArtifactCoordinate) -> Result<Url> {
    Url::from_file_path(path).map_err(|()| Error::Model {
        coordinate: coordinate.clone(),
        message: format!(
            "systemPath '{}' could not be converted to a file URL",
            path.display()
        ),
    })
}

fn dependency_from_pom(
    dependency: &Dependency,
    properties: &BTreeMap<String, String>,
    context: &ResolutionContext,
) -> Result<EffectiveDependency> {
    let group_id = interpolate(&dependency.group_id, properties, context)?;
    let artifact_id = interpolate(&dependency.artifact_id, properties, context)?;
    let dependency_type = dependency
        .r#type
        .as_deref()
        .unwrap_or("jar")
        .to_ascii_lowercase();
    let (extension, default_classifier) = artifact_type(&dependency_type);
    let classifier = dependency
        .classifier
        .as_ref()
        .map(|value| interpolate(value, properties, context))
        .transpose()?
        .or_else(|| default_classifier.map(str::to_string));
    let mut key = ArtifactKey::new(group_id, artifact_id)?.with_extension(extension)?;
    if let Some(classifier) = classifier {
        key = key.with_classifier(classifier)?;
    }
    let version = dependency
        .version
        .as_ref()
        .map(|value| interpolate(value, properties, context))
        .transpose()?;
    let system_path = dependency
        .system_path
        .as_ref()
        .map(|value| interpolate(value, properties, context))
        .transpose()?;
    let exclusions = if let Some(exclusions) = &dependency.exclusions {
        exclusions
            .exclusions
            .iter()
            .map(|exclusion| {
                Ok((
                    interpolate(&exclusion.group_id, properties, context)?,
                    interpolate(&exclusion.artifact_id, properties, context)?,
                ))
            })
            .collect::<Result<BTreeSet<_>>>()?
    } else {
        BTreeSet::new()
    };
    let optional = dependency
        .optional_expression()
        .map(|value| {
            interpolate(value, properties, context)
                .map(|value| value.trim().eq_ignore_ascii_case("true"))
        })
        .transpose()?
        .unwrap_or(dependency.optional);
    let scope = dependency
        .scope_expression()
        .map(|value| {
            interpolate(value, properties, context)
                .and_then(|value| value.parse::<DependencyScope>().map_err(Error::from))
        })
        .transpose()?
        .or(dependency.scope)
        .unwrap_or_default();
    Ok(EffectiveDependency {
        key,
        version,
        scope,
        scope_explicit: dependency.scope_expression().is_some(),
        system_path,
        optional,
        optional_explicit: dependency.optional_explicit,
        exclusions,
    })
}

fn merge_dependencies(
    destination: &mut Vec<EffectiveDependency>,
    source: &Dependencies,
    management: &BTreeMap<ArtifactKey, EffectiveDependency>,
    properties: &BTreeMap<String, String>,
    context: &ResolutionContext,
) -> Result<()> {
    for dependency in &source.dependencies {
        let mut effective = dependency_from_pom(dependency, properties, context)?;
        let existing = destination
            .iter()
            .find(|candidate| candidate.key == effective.key)
            .cloned();
        if let Some(managed) = management.get(&effective.key) {
            if effective.version.is_none() {
                effective.version.clone_from(&managed.version);
            }
            if dependency.scope.is_none() && managed.scope_explicit {
                effective.scope = managed.scope;
            }
            if !dependency.optional_explicit && managed.optional_explicit {
                effective.optional = managed.optional;
            }
            effective.exclusions.extend(managed.exclusions.clone());
            if effective.system_path.is_none() {
                effective.system_path.clone_from(&managed.system_path);
            }
        }
        if let Some(existing) = &existing {
            if effective.version.is_none() {
                effective.version.clone_from(&existing.version);
            }
            if dependency.scope.is_none()
                && !management
                    .get(&effective.key)
                    .is_some_and(|managed| managed.scope_explicit)
            {
                effective.scope = existing.scope;
            }
            if !dependency.optional_explicit
                && !management
                    .get(&effective.key)
                    .is_some_and(|managed| managed.optional_explicit)
            {
                effective.optional = existing.optional;
            }
            effective.exclusions.extend(existing.exclusions.clone());
            if effective.system_path.is_none() {
                effective.system_path.clone_from(&existing.system_path);
            }
        }
        if let Some(existing) = destination
            .iter_mut()
            .find(|candidate| candidate.key == effective.key)
        {
            *existing = effective;
        } else {
            destination.push(effective);
        }
    }
    Ok(())
}

fn artifact_type(value: &str) -> (String, Option<&'static str>) {
    match value {
        "test-jar" => ("jar".to_string(), Some("tests")),
        "java-source" => ("jar".to_string(), Some("sources")),
        "javadoc" => ("jar".to_string(), Some("javadoc")),
        "ejb-client" => ("jar".to_string(), Some("client")),
        "jar"
        | "bundle"
        | "maven-plugin"
        | "ejb"
        | "classpath-jar"
        | "modular-jar"
        | "processor"
        | "classpath-processor"
        | "modular-processor" => ("jar".to_string(), None),
        other => (other.to_string(), None),
    }
}

fn add_project_properties(
    properties: &mut BTreeMap<String, String>,
    coordinate: &ArtifactCoordinate,
    parent: Option<&ArtifactCoordinate>,
) {
    for prefix in ["project", "pom"] {
        properties.insert(format!("{prefix}.groupId"), coordinate.key.group_id.clone());
        properties.insert(
            format!("{prefix}.artifactId"),
            coordinate.key.artifact_id.clone(),
        );
        properties.insert(format!("{prefix}.version"), coordinate.version.clone());
        if let Some(parent) = parent {
            properties.insert(
                format!("{prefix}.parent.groupId"),
                parent.key.group_id.clone(),
            );
            properties.insert(
                format!("{prefix}.parent.artifactId"),
                parent.key.artifact_id.clone(),
            );
            properties.insert(format!("{prefix}.parent.version"), parent.version.clone());
        }
    }
}

fn interpolate(
    value: &str,
    properties: &BTreeMap<String, String>,
    context: &ResolutionContext,
) -> Result<String> {
    let mut result = value.to_string();
    let mut seen = BTreeSet::new();
    for _ in 0..64 {
        if !seen.insert(result.clone()) {
            return Err(Error::Interpolation(format!(
                "property cycle while expanding '{value}'"
            )));
        }
        let Some(start) = result.find("${") else {
            return Ok(result);
        };
        let Some(relative_end) = result.get(start + 2..).and_then(|tail| tail.find('}')) else {
            return Ok(result);
        };
        let end = start + 2 + relative_end;
        let key = result
            .get(start + 2..end)
            .ok_or_else(|| Error::Interpolation(value.to_string()))?;
        let base_directory;
        let replacement = match key {
            "basedir" | "project.basedir" | "project.rootDirectory" => {
                base_directory = context
                    .base_directory
                    .as_ref()
                    .map(|path| path.to_string_lossy().into_owned());
                base_directory.as_ref()
            }
            _ => key
                .strip_prefix("env.")
                .and_then(|name| context.environment.get(name))
                .or_else(|| context.properties.get(key))
                .or_else(|| properties.get(key)),
        };
        let Some(replacement) = replacement else {
            return Ok(result);
        };
        result.replace_range(start..=end, replacement);
    }
    Err(Error::Interpolation(format!(
        "expansion depth exceeded for '{value}'"
    )))
}

fn active_profiles<'a>(
    project: &'a Project,
    packaging: &str,
    context: &ResolutionContext,
    properties: &BTreeMap<String, String>,
) -> Vec<&'a ristretto_pom::Profile> {
    let Some(profiles) = &project.profiles else {
        return Vec::new();
    };
    let mut active = profiles
        .profiles
        .iter()
        .filter(|profile| !context.inactive_profiles.contains(&profile.id))
        .filter(|profile| {
            context.active_profiles.contains(&profile.id)
                || profile.activation.as_ref().is_some_and(|activation| {
                    !activation.active_by_default
                        && has_activation_criteria(activation)
                        && activation_matches(activation, packaging, context, properties)
                })
        })
        .collect::<Vec<_>>();
    if active.is_empty() {
        active.extend(
            profiles
                .profiles
                .iter()
                .filter(|profile| !context.inactive_profiles.contains(&profile.id))
                .filter(|profile| {
                    profile
                        .activation
                        .as_ref()
                        .is_some_and(|activation| activation.active_by_default)
                }),
        );
    }
    active
}

fn has_activation_criteria(activation: &Activation) -> bool {
    activation.jdk.is_some()
        || activation.os.is_some()
        || activation.property.is_some()
        || activation.file.is_some()
        || activation.packaging.is_some()
        || activation.condition.is_some()
}

fn activation_matches(
    activation: &Activation,
    packaging: &str,
    context: &ResolutionContext,
    properties: &BTreeMap<String, String>,
) -> bool {
    if let Some(jdk) = &activation.jdk
        && !context
            .java_version
            .as_ref()
            .is_some_and(|version| jdk_matches(jdk, version))
    {
        return false;
    }
    if let Some(os) = &activation.os
        && (!optional_match(os.name.as_deref(), context.os_name.as_deref())
            || !optional_match(os.arch.as_deref(), context.os_arch.as_deref())
            || !os_version_matches(os.version.as_deref(), context.os_version.as_deref())
            || !os
                .family
                .as_ref()
                .is_none_or(|family| os_family_matches(family, context.os_name.as_deref())))
    {
        return false;
    }
    if let Some(property) = &activation.property {
        let negated_name = property.name.starts_with('!');
        let name = property.name.trim_start_matches('!');
        let actual = if name == "packaging" {
            Some(packaging)
        } else {
            name.strip_prefix("env.")
                .and_then(|name| context.environment.get(name))
                .or_else(|| context.properties.get(name))
                .map(String::as_str)
        };
        if negated_name == actual.is_some() {
            return false;
        }
        if let Some(expected) = &property.value {
            let negated_value = expected.starts_with('!');
            let expected = expected.trim_start_matches('!');
            let expected = interpolate(expected, &BTreeMap::new(), context)
                .unwrap_or_else(|_| expected.to_string());
            if actual.is_some_and(|actual| actual == expected) == negated_value {
                return false;
            }
        }
    }
    if let Some(file) = &activation.file {
        let Some(base) = &context.base_directory else {
            return false;
        };
        let exists = file.exists.as_ref().and_then(|path| {
            interpolate(path, &BTreeMap::new(), context)
                .ok()
                .map(|path| base.join(path).exists())
        });
        let missing = file.missing.as_ref().and_then(|path| {
            interpolate(path, &BTreeMap::new(), context)
                .ok()
                .map(|path| !base.join(path).exists())
        });
        if exists == Some(false)
            || file.exists.is_some() && exists.is_none()
            || missing == Some(false)
            || file.missing.is_some() && missing.is_none()
        {
            return false;
        }
    }
    if activation
        .packaging
        .as_ref()
        .is_some_and(|expected| expected != packaging)
    {
        return false;
    }
    if let Some(condition) = &activation.condition
        && !condition_matches(condition, context, properties)
    {
        return false;
    }
    true
}

fn condition_matches(
    condition: &str,
    context: &ResolutionContext,
    properties: &BTreeMap<String, String>,
) -> bool {
    crate::condition::matches(condition, context, properties)
}

fn optional_match(expected: Option<&str>, actual: Option<&str>) -> bool {
    expected.is_none_or(|expected| {
        let negated = expected.starts_with('!');
        let expected = expected.trim_start_matches('!');
        actual.is_some_and(|actual| actual.eq_ignore_ascii_case(expected)) != negated
    })
}

fn os_family_matches(family: &str, os_name: Option<&str>) -> bool {
    let Some(os_name) = os_name else {
        return false;
    };
    let negated = family.starts_with('!');
    let family = family.trim_start_matches('!');
    let os_name = os_name.to_ascii_lowercase();
    let matches = match family.to_ascii_lowercase().as_str() {
        "windows" => os_name.contains("windows"),
        "os/2" => os_name.contains("os/2"),
        "netware" => os_name.contains("netware"),
        "dos" => os_name.contains("dos") && !os_name.contains("windows"),
        "mac" => os_name.contains("mac"),
        "tandem" => os_name.contains("nonstop_kernel"),
        "unix" => {
            !os_name.contains("windows")
                && !os_name.contains("openvms")
                && (!os_name.contains("mac") || os_name.ends_with('x') || os_name == "macos")
        }
        "win9x" => {
            os_name.contains("windows")
                && ["95", "98", "me", "ce"]
                    .iter()
                    .any(|version| os_name.contains(version))
        }
        "z/os" => os_name.contains("z/os") || os_name.contains("os/390"),
        "os/400" => os_name.contains("os/400"),
        "openvms" => os_name.contains("openvms"),
        other => os_name.contains(other),
    };
    matches != negated
}

fn os_version_matches(expected: Option<&str>, actual: Option<&str>) -> bool {
    let Some(expected) = expected else {
        return true;
    };
    let Some(actual) = actual else {
        return false;
    };
    if let Some(pattern) = expected.strip_prefix("regex:") {
        let mut builder = RegexBuilder::new(pattern);
        builder.backtrack_limit(1_000_000);
        let actual = actual.to_ascii_lowercase();
        builder
            .build()
            .ok()
            .and_then(|regex| regex.find(&actual).ok())
            .flatten()
            .is_some_and(|found| found.start() == 0 && found.end() == actual.len())
    } else {
        optional_match(Some(expected), Some(actual))
    }
}

fn jdk_matches(specification: &str, version: &str) -> bool {
    if specification.starts_with(['[', '(']) {
        specification
            .parse::<VersionSpec>()
            .is_ok_and(|range| range.matches(&Version::new(version)))
    } else if let Some(excluded) = specification.strip_prefix('!') {
        !version.starts_with(excluded)
    } else {
        version.starts_with(specification)
    }
}

fn append_pom_repositories(
    destination: &mut Vec<RemoteRepository>,
    repositories: Option<&ristretto_pom::Repositories>,
    properties: &BTreeMap<String, String>,
    context: &ResolutionContext,
) -> Result<()> {
    let Some(repositories) = repositories else {
        return Ok(());
    };
    for repository in &repositories.repositories {
        let layout = repository
            .layout
            .as_ref()
            .map(ToString::to_string)
            .map(|layout| interpolate(&layout, properties, context))
            .transpose()?;
        if layout.as_deref().is_some_and(|layout| layout != "default") {
            return Err(Error::InvalidConfiguration(format!(
                "repository '{}' uses unsupported layout '{}'",
                repository.id,
                layout.unwrap_or_default()
            )));
        }
        let id = interpolate(&repository.id, properties, context)?;
        let url = interpolate(&repository.url, properties, context)?;
        let mut converted = RemoteRepository::new(id, url)?;
        if let Some(releases) = &repository.releases {
            converted.releases.enabled =
                interpolated_policy_enabled(releases, properties, context)?;
            converted.releases.update = interpolated_update_policy(releases, properties, context)?;
            converted.releases.checksum =
                interpolated_checksum_policy(releases, properties, context)?;
        }
        if let Some(snapshots) = &repository.snapshots {
            converted.snapshots.enabled =
                interpolated_policy_enabled(snapshots, properties, context)?;
            converted.snapshots.update =
                interpolated_update_policy(snapshots, properties, context)?;
            converted.snapshots.checksum =
                interpolated_checksum_policy(snapshots, properties, context)?;
        }
        if let Some(existing) = destination
            .iter_mut()
            .find(|existing| existing.id == converted.id)
        {
            *existing = converted;
        } else {
            destination.push(converted);
        }
    }
    Ok(())
}

fn interpolated_policy_enabled(
    policy: &ristretto_pom::RepositoryPolicy,
    properties: &BTreeMap<String, String>,
    context: &ResolutionContext,
) -> Result<bool> {
    policy
        .enabled_expression()
        .map(|value| {
            interpolate(value, properties, context)
                .map(|value| value.trim().eq_ignore_ascii_case("true"))
        })
        .transpose()
        .map(|enabled| enabled.unwrap_or(policy.enabled))
}

fn pom_update_policy(policy: Option<&ristretto_pom::UpdatePolicy>) -> Result<UpdatePolicy> {
    Ok(match policy {
        Some(ristretto_pom::UpdatePolicy::Always) => UpdatePolicy::Always,
        Some(ristretto_pom::UpdatePolicy::Never) => UpdatePolicy::Never,
        Some(ristretto_pom::UpdatePolicy::Interval(value)) => value
            .strip_prefix("interval:")
            .and_then(|minutes| minutes.parse().ok())
            .map(UpdatePolicy::IntervalMinutes)
            .ok_or_else(|| {
                Error::InvalidConfiguration(format!("invalid repository update policy '{value}'"))
            })?,
        Some(ristretto_pom::UpdatePolicy::Daily) | None => UpdatePolicy::Daily,
    })
}

fn interpolated_update_policy(
    policy: &ristretto_pom::RepositoryPolicy,
    properties: &BTreeMap<String, String>,
    context: &ResolutionContext,
) -> Result<UpdatePolicy> {
    if let Some(expression) = policy.update_policy_expression() {
        let value = interpolate(expression, properties, context)?;
        return match value.to_ascii_lowercase().as_str() {
            "always" => Ok(UpdatePolicy::Always),
            "daily" => Ok(UpdatePolicy::Daily),
            "never" => Ok(UpdatePolicy::Never),
            _ => value
                .strip_prefix("interval:")
                .and_then(|minutes| minutes.parse().ok())
                .map(UpdatePolicy::IntervalMinutes)
                .ok_or_else(|| {
                    Error::InvalidConfiguration(format!(
                        "invalid repository update policy '{value}'"
                    ))
                }),
        };
    }
    pom_update_policy(policy.update_policy.as_ref())
}

fn pom_checksum_policy(policy: Option<&ristretto_pom::ChecksumPolicy>) -> ChecksumPolicy {
    match policy {
        Some(ristretto_pom::ChecksumPolicy::Fail) => ChecksumPolicy::Fail,
        Some(ristretto_pom::ChecksumPolicy::Ignore) => ChecksumPolicy::Ignore,
        Some(ristretto_pom::ChecksumPolicy::Warn) | None => ChecksumPolicy::Warn,
    }
}

fn interpolated_checksum_policy(
    policy: &ristretto_pom::RepositoryPolicy,
    properties: &BTreeMap<String, String>,
    context: &ResolutionContext,
) -> Result<ChecksumPolicy> {
    if let Some(expression) = policy.checksum_policy_expression() {
        let value = interpolate(expression, properties, context)?;
        return match value.to_ascii_lowercase().as_str() {
            "fail" => Ok(ChecksumPolicy::Fail),
            "ignore" => Ok(ChecksumPolicy::Ignore),
            "warn" => Ok(ChecksumPolicy::Warn),
            _ => Err(Error::InvalidConfiguration(format!(
                "invalid repository checksum policy '{value}'"
            ))),
        };
    }
    Ok(pom_checksum_policy(policy.checksum_policy.as_ref()))
}

fn merge_repository_policy(target: &mut RepositoryPolicy, candidate: RepositoryPolicy) {
    if !target.enabled && candidate.enabled {
        *target = candidate;
    }
}

#[cfg(not(target_family = "wasm"))]
fn system_repository() -> Result<RemoteRepository> {
    let mut repository = RemoteRepository::new(SYSTEM_REPOSITORY_ID, "file:///")?;
    repository.releases.checksum = ChecksumPolicy::Ignore;
    repository.snapshots.checksum = ChecksumPolicy::Ignore;
    Ok(repository)
}

fn derive_scope(parent: DependencyScope, child: DependencyScope) -> Option<DependencyScope> {
    use DependencyScope::{Compile, Runtime};
    match (parent, child) {
        (Compile, Compile) => Some(Compile),
        (Compile, Runtime) | (Runtime, Compile | Runtime) => Some(Runtime),
        _ => None,
    }
}

fn matching_override<'a>(rules: &'a [OverrideRule], key: &ArtifactKey) -> Option<&'a OverrideRule> {
    rules
        .iter()
        .filter(|rule| rule.matcher.matches(key))
        .max_by_key(|rule| rule.matcher.specificity())
}

fn exclusion_matches(exclusions: &BTreeSet<(String, String)>, key: &ArtifactKey) -> bool {
    exclusions.iter().any(|(group_id, artifact_id)| {
        (group_id == "*" || group_id == &key.group_id)
            && (artifact_id == "*" || artifact_id == &key.artifact_id)
    })
}

fn validate_overrides(rules: &[OverrideRule]) -> Result<()> {
    for (index, left) in rules.iter().enumerate() {
        for right in rules.iter().skip(index + 1) {
            if left.matcher == right.matcher {
                return Err(Error::InvalidConfiguration(format!(
                    "duplicate override matcher for {}:{}",
                    left.matcher.group_id, left.matcher.artifact_id
                )));
            }
        }
    }
    Ok(())
}

fn validate_metadata_key(
    metadata: &RepositoryMetadata,
    key: &ArtifactKey,
    url: &str,
) -> Result<()> {
    if metadata.matches_key(key) {
        Ok(())
    } else {
        Err(Error::Transport {
            url: url.to_string(),
            message: format!("metadata coordinates do not match requested artifact {key}"),
        })
    }
}

fn validate_response_length(url: &str, expected: Option<u64>, actual: usize) -> Result<()> {
    let actual = actual as u64;
    if let Some(expected) = expected
        && expected != actual
    {
        return Err(Error::Transport {
            url: url.to_string(),
            message: format!("response length mismatch: expected {expected}, received {actual}"),
        });
    }
    Ok(())
}

fn parse_checksum(bytes: &[u8], algorithm: &str) -> Option<String> {
    let length = match algorithm {
        "SHA-512" => 128,
        "SHA-256" => 64,
        "SHA-1" => 40,
        "MD5" => 32,
        _ => return None,
    };
    let value = String::from_utf8_lossy(bytes);
    let checksum = if let Some((_, checksum)) = value.split_once('=') {
        let candidates = checksum
            .split(|character: char| !character.is_ascii_hexdigit())
            .filter(|candidate| !candidate.is_empty())
            .collect::<Vec<_>>();
        candidates
            .iter()
            .copied()
            .find(|candidate| candidate.len() == length)
            .or_else(|| candidates.first().copied())?
    } else {
        value.split_whitespace().next()?
    };
    checksum
        .bytes()
        .all(|byte| byte.is_ascii_hexdigit())
        .then(|| checksum.to_ascii_lowercase())
}

#[derive(Debug)]
enum ChecksumDigester {
    Sha512(Sha512),
    Sha256(Sha256),
    Sha1(Sha1),
    Md5(md5::Md5),
}

impl ChecksumDigester {
    fn new(algorithm: &str) -> Self {
        match algorithm {
            "SHA-512" => Self::Sha512(Sha512::new()),
            "SHA-256" => Self::Sha256(Sha256::new()),
            "SHA-1" => Self::Sha1(Sha1::new()),
            _ => Self::Md5(md5::Md5::new()),
        }
    }

    fn update(&mut self, bytes: &[u8]) {
        match self {
            Self::Sha512(digester) => digester.update(bytes),
            Self::Sha256(digester) => digester.update(bytes),
            Self::Sha1(digester) => digester.update(bytes),
            Self::Md5(digester) => digester.update(bytes),
        }
    }

    fn finish(self) -> String {
        let bytes = match self {
            Self::Sha512(digester) => digester.finalize().to_vec(),
            Self::Sha256(digester) => digester.finalize().to_vec(),
            Self::Sha1(digester) => digester.finalize().to_vec(),
            Self::Md5(digester) => digester.finalize().to_vec(),
        };
        bytes.iter().fold(
            String::with_capacity(bytes.len().saturating_mul(2)),
            |mut output, byte| {
                let _ = write!(output, "{byte:02x}");
                output
            },
        )
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod checksum_tests {
    #![expect(
        clippy::too_many_lines,
        reason = "coverage tests enumerate independent resolver boundary conditions"
    )]

    use super::*;
    use crate::{Authentication, Proxy, SecretString};
    use bytes::Bytes;
    use futures_util::stream;
    use ristretto_pom::DependencyScope::{Compile, Provided, Runtime};
    use ristretto_pom::{
        ActivationFile, ActivationOs, ChecksumPolicy as PomChecksumPolicy, Exclusion, Parent,
        Profile, Profiles, Repositories, Repository as PomRepository, RepositoryLayout,
        RepositoryPolicy as PomRepositoryPolicy, UpdatePolicy as PomUpdatePolicy,
    };
    use tempfile::TempDir;

    #[derive(Debug, Clone, Copy)]
    enum TestBehavior {
        Missing,
        TransportError,
        StreamError,
        Bytes,
        LengthMismatch,
        Oversized,
        MalformedChecksum,
        Metadata,
        InvalidSnapshot,
    }

    #[derive(Debug)]
    struct TestTransport(TestBehavior);

    impl Transport for TestTransport {
        fn supports(&self, scheme: &str) -> bool {
            scheme == "memory"
        }

        fn get<'a>(
            &'a self,
            request: &'a TransportRequest,
        ) -> BoxFuture<'a, Result<Option<crate::TransportResponse>>> {
            Box::pin(async move {
                match self.0 {
                    TestBehavior::Missing => Ok(None),
                    TestBehavior::TransportError => Err(Error::Transport {
                        url: request.url.clone(),
                        message: "scripted transport failure".to_string(),
                    }),
                    TestBehavior::StreamError => Ok(Some(crate::TransportResponse {
                        body: Box::pin(stream::once(async {
                            Err(Error::Destination("scripted stream failure".to_string()))
                        })),
                        content_length: None,
                        from_cache: false,
                    })),
                    TestBehavior::Bytes => Ok(Some(crate::TransportResponse {
                        body: Box::pin(stream::once(async { Ok(Bytes::from_static(b"artifact")) })),
                        content_length: Some(8),
                        from_cache: false,
                    })),
                    TestBehavior::LengthMismatch => Ok(Some(crate::TransportResponse {
                        body: Box::pin(stream::once(async { Ok(Bytes::from_static(b"artifact")) })),
                        content_length: Some(99),
                        from_cache: false,
                    })),
                    TestBehavior::Oversized => Ok(Some(crate::TransportResponse {
                        body: Box::pin(stream::once(async {
                            Ok(Bytes::from(vec![0_u8; 16 * 1024 * 1024 + 1]))
                        })),
                        content_length: None,
                        from_cache: false,
                    })),
                    TestBehavior::MalformedChecksum => {
                        if request.kind == ResourceKind::Checksum {
                            Ok(Some(crate::TransportResponse {
                                body: Box::pin(stream::once(async {
                                    Ok(Bytes::from_static(b"not-a-checksum"))
                                })),
                                content_length: None,
                                from_cache: false,
                            }))
                        } else {
                            Ok(None)
                        }
                    }
                    TestBehavior::Metadata => Ok(Some(crate::TransportResponse {
                        body: Box::pin(stream::once(async {
                            Ok(Bytes::from_static(
                                b"<metadata><groupId>org.example</groupId>\
                                  <artifactId>demo</artifactId><versioning><versions>\
                                  <version>1</version></versions></versioning></metadata>",
                            ))
                        })),
                        content_length: None,
                        from_cache: false,
                    })),
                    TestBehavior::InvalidSnapshot => {
                        if request.kind == ResourceKind::SnapshotMetadata {
                            Ok(Some(crate::TransportResponse {
                                body: Box::pin(stream::once(async {
                                    Ok(Bytes::from_static(
                                        b"<metadata><groupId>org.example</groupId>\
                                          <artifactId>demo</artifactId>\
                                          <version>1-SNAPSHOT</version><versioning>\
                                          <snapshotVersions><snapshotVersion>\
                                          <extension>jar</extension><value>..</value>\
                                          </snapshotVersion></snapshotVersions>\
                                          </versioning></metadata>",
                                    ))
                                })),
                                content_length: None,
                                from_cache: false,
                            }))
                        } else {
                            Ok(None)
                        }
                    }
                }
            })
        }
    }

    #[derive(Debug, Default)]
    struct Sink;

    #[derive(Debug, Default)]
    struct SinkTransaction(Vec<u8>);

    impl ArtifactDestination for Sink {
        type Transaction = SinkTransaction;
        type Output = Vec<u8>;

        fn begin<'a>(
            &'a self,
            _artifact: &'a ResolvedArtifact,
        ) -> BoxFuture<'a, Result<Self::Transaction>> {
            Box::pin(async { Ok(SinkTransaction::default()) })
        }
    }

    impl ArtifactTransaction for SinkTransaction {
        type Output = Vec<u8>;

        fn write(&mut self, chunk: Bytes) -> BoxFuture<'_, Result<()>> {
            Box::pin(async move {
                self.0.extend_from_slice(&chunk);
                Ok(())
            })
        }

        fn commit(self) -> BoxFuture<'static, Result<Self::Output>> {
            Box::pin(async move { Ok(self.0) })
        }

        fn abort(self) -> BoxFuture<'static, Result<()>> {
            Box::pin(async { Ok(()) })
        }
    }

    #[expect(
        clippy::expect_used,
        reason = "the scripted resolver fixture must use a valid test configuration"
    )]
    fn scripted_resolver(
        repository: RemoteRepository,
        behavior: TestBehavior,
    ) -> (Resolver, RemoteRepository) {
        let resolver = Resolver::builder()
            .repositories(vec![repository.clone()])
            .transports(vec![Arc::new(TestTransport(behavior))])
            .build()
            .expect("scripted resolver");
        (resolver, repository)
    }

    #[expect(
        clippy::expect_used,
        reason = "the scripted resolution fixture must construct a valid artifact URL"
    )]
    fn scripted_resolution(
        repository: RemoteRepository,
        coordinate: ArtifactCoordinate,
    ) -> Resolution {
        let artifact = ResolvedArtifact {
            node: NodeId(0),
            file_version: coordinate.version.clone(),
            repository_id: repository.id.clone(),
            url: repository
                .resource_url(&coordinate.artifact_path())
                .expect("artifact URL"),
            coordinate: coordinate.clone(),
        };
        Resolution {
            roots: vec![NodeId(0)],
            nodes: vec![DependencyNode {
                id: NodeId(0),
                requested: Some(coordinate.clone()),
                coordinate,
                scope: Compile,
                optional: false,
                depth: 0,
                status: NodeStatus::Selected,
                events: Vec::new(),
                repository_id: Some(repository.id.clone()),
            }],
            edges: Vec::new(),
            artifacts: vec![artifact],
            repositories: vec![repository],
            diagnostics: Vec::new(),
            repository_fallback: true,
        }
    }

    fn effective_dependency(key: ArtifactKey, version: impl Into<String>) -> EffectiveDependency {
        EffectiveDependency {
            key,
            version: Some(version.into()),
            scope: Compile,
            scope_explicit: false,
            system_path: None,
            optional: false,
            optional_explicit: false,
            exclusions: BTreeSet::new(),
        }
    }

    fn effective_model(
        coordinate: ArtifactCoordinate,
        dependencies: Vec<EffectiveDependency>,
    ) -> Arc<EffectiveModel> {
        Arc::new(EffectiveModel {
            coordinate,
            packaging: "jar".to_string(),
            properties: BTreeMap::new(),
            dependency_management: BTreeMap::new(),
            dependencies,
            repositories: Vec::new(),
            relocation: None,
            repository_id: Some(crate::CENTRAL_REPOSITORY_ID.to_string()),
        })
    }

    #[expect(
        clippy::expect_used,
        reason = "test project fixtures must contain valid POM XML"
    )]
    fn project(xml: &str) -> Project {
        Project::from_reader(Cursor::new(xml.as_bytes())).expect("test project")
    }

    #[expect(
        clippy::expect_used,
        reason = "the effective-model fixture must use a valid resolver configuration"
    )]
    async fn build_test_project(
        project: Project,
        context: ResolutionContext,
        mut stack: Vec<ArtifactCoordinate>,
    ) -> Result<EffectiveModel> {
        let resolver = Resolver::builder().build().expect("resolver");
        let request = ResolutionRequest::from_project_bytes(b"<project/>".to_vec())
            .with_context(context.clone());
        let repositories = resolver.repositories.clone();
        let mut session = ResolutionSession::new(&resolver, &request);
        session
            .build_effective_model_with_context(
                project,
                None,
                None,
                &repositories,
                &mut stack,
                &context,
            )
            .await
    }

    #[test]
    fn parses_common_checksum_sidecar_formats() {
        let checksum = "0123456789abcdef0123456789abcdef01234567";
        for sidecar in [
            checksum.to_string(),
            format!("{checksum}  artifact.jar"),
            format!("SHA1(artifact.jar)={checksum}"),
            format!("SHA1(artifact.jar) = {checksum}"),
        ] {
            assert_eq!(
                parse_checksum(sidecar.as_bytes(), "SHA-1").as_deref(),
                Some(checksum)
            );
        }
        assert!(parse_checksum(b"not-a-checksum", "SHA-1").is_none());
    }

    #[test]
    fn exact_mirror_precedes_an_earlier_wildcard_and_preserves_policies() {
        let source = RemoteRepository::central();
        let wildcard = Mirror {
            repository: RemoteRepository::new("wildcard", "https://wildcard.example")
                .expect("wildcard mirror"),
            mirror_of: "*".to_string(),
        };
        let exact = Mirror {
            repository: RemoteRepository::new("exact", "https://exact.example")
                .expect("exact mirror"),
            mirror_of: "central".to_string(),
        };
        let resolver = Resolver::builder()
            .repositories(vec![source.clone()])
            .mirror(wildcard)
            .mirror(exact)
            .build()
            .expect("resolver");

        let repositories = resolver.apply_mirrors(std::slice::from_ref(&source));
        assert_eq!(repositories.len(), 1);
        let repository = repositories.first().expect("exact mirror");
        assert_eq!(repository.id, "exact");
        assert_eq!(repository.releases, source.releases);
        assert_eq!(repository.snapshots, source.snapshots);
        assert_eq!(resolver.apply_mirrors(&repositories), repositories);

        let mut releases = source.clone();
        releases.snapshots.enabled = false;
        let mut snapshots = source.clone();
        snapshots.releases.enabled = false;
        snapshots.snapshots.enabled = true;
        let merged = resolver.apply_mirrors(&[releases, snapshots]);
        assert_eq!(merged.len(), 1);
        assert!(
            merged
                .first()
                .is_some_and(|repository| repository.releases.enabled)
        );
        assert!(
            merged
                .first()
                .is_some_and(|repository| repository.snapshots.enabled)
        );
    }

    #[test]
    fn covers_request_builder_classpath_and_artifact_type_boundaries() {
        let coordinate = ArtifactCoordinate::new("org.example", "demo", "1").expect("coordinate");
        let context = ResolutionContext::default().with_property("revision", "1");
        let override_rule = OverrideRule::new(
            crate::OverrideMatcher::new("org.example", "demo"),
            OverrideAction::Exclude,
        );
        let selection = ArtifactSelection {
            main_artifacts: false,
            poms: true,
            attachments: vec![("sources".to_string(), "jar".to_string())],
        };
        let request = ResolutionRequest::new(coordinate.clone())
            .with_root(ResolutionRoot::ProjectBytes {
                bytes: b"<project/>".to_vec(),
                include_artifact: false,
            })
            .with_classpath(Classpath::Compile)
            .with_context(context.clone())
            .with_override(override_rule)
            .with_conflict_policy(ConflictPolicy::NearestFirst)
            .with_transitive_repositories(false)
            .with_artifacts(selection.clone());
        assert_eq!(request.roots.len(), 2);
        assert_eq!(request.context, context);
        assert_eq!(request.artifacts, selection);
        assert!(!request.transitive_repositories);
        assert!(Classpath::Compile.includes(Provided));
        assert!(!Classpath::Compile.includes(Runtime));
        assert!(Classpath::Runtime.includes(Runtime));
        assert!(!Classpath::Runtime.includes(DependencyScope::Test));
        assert!(Classpath::Test.includes(DependencyScope::System));
        assert!(!Classpath::Test.includes(DependencyScope::Import));

        let bytes_request = ResolutionRequest::from_project_bytes(b"<project/>".to_vec());
        assert!(matches!(
            bytes_request.roots.as_slice(),
            [ResolutionRoot::ProjectBytes { .. }]
        ));
        let path_request = ResolutionRequest::from_project_file("pom.xml");
        assert!(matches!(
            path_request.roots.as_slice(),
            [ResolutionRoot::ProjectFile(_)]
        ));

        for (kind, extension, classifier) in [
            ("test-jar", "jar", Some("tests")),
            ("java-source", "jar", Some("sources")),
            ("javadoc", "jar", Some("javadoc")),
            ("ejb-client", "jar", Some("client")),
            ("bundle", "jar", None),
            ("maven-plugin", "jar", None),
            ("ejb", "jar", None),
            ("classpath-jar", "jar", None),
            ("modular-jar", "jar", None),
            ("processor", "jar", None),
            ("classpath-processor", "jar", None),
            ("modular-processor", "jar", None),
            ("zip", "zip", None),
        ] {
            assert_eq!(
                artifact_type(kind),
                (extension.to_string(), classifier),
                "{kind}"
            );
        }
        assert_eq!(coordinate.key.extension, "jar");
    }

    #[test]
    fn covers_interpolation_and_activation_boundaries() {
        let coordinate = ArtifactCoordinate::new("org.example", "child", "2").expect("coordinate");
        let parent = ArtifactCoordinate::new("org.example", "parent", "1").expect("parent");
        let mut properties = BTreeMap::new();
        add_project_properties(&mut properties, &coordinate, Some(&parent));
        properties.insert("one".to_string(), "${two}".to_string());
        properties.insert("two".to_string(), "value".to_string());
        let temporary = TempDir::new().expect("temp directory");
        std::fs::write(temporary.path().join("present"), b"fixture").expect("fixture");
        let mut context = ResolutionContext {
            base_directory: Some(temporary.path().to_path_buf()),
            java_version: Some("17.0.2".to_string()),
            os_name: Some("Linux".to_string()),
            os_arch: Some("x86_64".to_string()),
            os_version: Some("6.8".to_string()),
            ..ResolutionContext::default()
        };
        context
            .environment
            .insert("CHANNEL".to_string(), "release".to_string());
        context
            .properties
            .insert("mode".to_string(), "release".to_string());
        assert_eq!(
            interpolate("${one}-${project.parent.version}", &properties, &context)
                .expect("interpolation"),
            "value-1"
        );
        assert_eq!(
            interpolate("${env.CHANNEL}", &properties, &context).expect("environment"),
            "release"
        );
        assert_eq!(
            interpolate("${basedir}", &properties, &context).expect("base directory"),
            temporary.path().to_string_lossy()
        );
        assert_eq!(
            interpolate("${unknown}", &properties, &context).expect("unresolved"),
            "${unknown}"
        );
        assert_eq!(
            interpolate("${unterminated", &properties, &context).expect("unterminated"),
            "${unterminated"
        );
        properties.insert("cycle".to_string(), "${cycle}".to_string());
        assert!(interpolate("${cycle}", &properties, &context).is_err());
        for index in 0..65 {
            properties.insert(format!("deep{index}"), format!("${{deep{}}}", index + 1));
        }
        assert!(interpolate("${deep0}", &properties, &context).is_err());

        let activation = Activation::builder()
            .jdk("[17,18)")
            .os(ActivationOs::builder()
                .name("linux")
                .family("unix")
                .arch("x86_64")
                .version("regex:6\\.[0-9]+")
                .build())
            .property("mode", Some("release".to_string()))
            .file(ActivationFile::exists("present"))
            .packaging("jar")
            .condition("true")
            .build();
        assert!(activation_matches(
            &activation,
            "jar",
            &context,
            &properties
        ));
        assert!(activation_matches(
            &Activation::property("mode", None),
            "jar",
            &context,
            &properties
        ));

        assert!(!activation_matches(
            &Activation::jdk("21"),
            "jar",
            &context,
            &properties
        ));
        assert!(!activation_matches(
            &Activation::builder().os(ActivationOs::windows()).build(),
            "jar",
            &context,
            &properties
        ));
        assert!(!activation_matches(
            &Activation::property("missing", None),
            "jar",
            &context,
            &properties
        ));
        assert!(!activation_matches(
            &Activation::property("mode", Some("!release".to_string())),
            "jar",
            &context,
            &properties
        ));
        assert!(!activation_matches(
            &Activation::builder()
                .file(ActivationFile::missing("present"))
                .build(),
            "jar",
            &context,
            &properties
        ));
        assert!(!activation_matches(
            &Activation::builder().packaging("war").build(),
            "jar",
            &context,
            &properties
        ));
        assert!(!activation_matches(
            &Activation::builder().condition("false").build(),
            "jar",
            &context,
            &properties
        ));
        let no_base = ResolutionContext::default();
        assert!(!activation_matches(
            &Activation::builder()
                .file(ActivationFile::exists("present"))
                .build(),
            "jar",
            &no_base,
            &properties
        ));

        let mut project = Project::new("org.example", "profiles", "1");
        let mut default_profile = Profile::new("default");
        default_profile.activation = Some(Activation::active_by_default());
        let mut explicit = Profile::new("explicit");
        explicit.activation = Some(Activation::default());
        project.profiles = Some(Profiles::from_vec(vec![default_profile, explicit]));
        assert_eq!(
            active_profiles(&project, "jar", &context, &properties)
                .first()
                .map(|profile| profile.id.as_str()),
            Some("default")
        );
        context.active_profiles.insert("explicit".to_string());
        assert_eq!(
            active_profiles(&project, "jar", &context, &properties)
                .first()
                .map(|profile| profile.id.as_str()),
            Some("explicit")
        );
        context.inactive_profiles.insert("explicit".to_string());
        assert!(
            active_profiles(&Project::new("g", "a", "1"), "jar", &context, &properties).is_empty()
        );
    }

    #[test]
    fn covers_os_jdk_scope_override_and_checksum_helpers() {
        for (family, name) in [
            ("windows", "Windows 11"),
            ("os/2", "OS/2"),
            ("netware", "NetWare"),
            ("dos", "MS-DOS"),
            ("mac", "Mac OS X"),
            ("tandem", "NONSTOP_KERNEL"),
            ("unix", "Linux"),
            ("win9x", "Windows 98"),
            ("z/os", "OS/390"),
            ("os/400", "OS/400"),
            ("openvms", "OpenVMS"),
            ("haiku", "Haiku"),
        ] {
            assert!(os_family_matches(family, Some(name)), "{family}");
            assert!(!os_family_matches(&format!("!{family}"), Some(name)));
        }
        assert!(!os_family_matches("unix", None));
        assert!(os_version_matches(None, None));
        assert!(!os_version_matches(Some("1"), None));
        assert!(os_version_matches(Some("!2"), Some("1")));
        assert!(!os_version_matches(Some("regex:["), Some("1")));
        assert!(jdk_matches("[17,18)", "17.0.1"));
        assert!(jdk_matches("!8", "17"));
        assert!(jdk_matches("17", "17.0.1"));
        assert!(!jdk_matches("[invalid", "17"));

        assert_eq!(derive_scope(Compile, Compile), Some(Compile));
        assert_eq!(derive_scope(Compile, Runtime), Some(Runtime));
        assert_eq!(derive_scope(Runtime, Compile), Some(Runtime));
        assert_eq!(derive_scope(Provided, Compile), None);

        let key = ArtifactKey::new("org.example", "demo")
            .and_then(|key| key.with_extension("zip"))
            .expect("key");
        let broad = OverrideRule::new(
            crate::OverrideMatcher::new("org.example", "demo"),
            OverrideAction::Exclude,
        );
        let mut narrow_matcher = crate::OverrideMatcher::new("org.example", "demo");
        narrow_matcher.extension = Some("zip".to_string());
        let narrow = OverrideRule::new(narrow_matcher, OverrideAction::Exclude);
        assert_eq!(
            matching_override(&[broad.clone(), narrow.clone()], &key),
            Some(&narrow)
        );
        assert!(
            matching_override(
                std::slice::from_ref(&broad),
                &ArtifactKey::new("x", "y").expect("key")
            )
            .is_none()
        );
        assert!(validate_overrides(&[broad.clone(), narrow]).is_ok());
        assert!(validate_overrides(&[broad.clone(), broad]).is_err());

        let exact = BTreeSet::from([("org.example".to_string(), "demo".to_string())]);
        let wildcard = BTreeSet::from([("*".to_string(), "*".to_string())]);
        assert!(exclusion_matches(&exact, &key));
        assert!(exclusion_matches(&wildcard, &key));
        assert!(!exclusion_matches(&BTreeSet::new(), &key));

        for algorithm in ["SHA-512", "SHA-256", "SHA-1", "MD5"] {
            let expected_length = match algorithm {
                "SHA-512" => 128,
                "SHA-256" => 64,
                "SHA-1" => 40,
                _ => 32,
            };
            let mut digester = ChecksumDigester::new(algorithm);
            digester.update(b"fixture");
            assert_eq!(digester.finish().len(), expected_length);
            assert!(parse_checksum(&vec![b'a'; expected_length], algorithm).is_some());
        }
        assert!(parse_checksum(b"00", "unsupported").is_none());
        assert!(parse_checksum(b"", "MD5").is_none());

        let attempt_key = ArtifactKey::new("org.example", "attempt").expect("attempt key");
        let selection = BTreeMap::from([(attempt_key.clone(), Version::new("1"))]);
        let mut attempted = Vec::new();
        record_selection_attempt(
            (&mut attempted, &selection),
            (attempt_key.to_string(), vec!["[1,2)".to_string()]),
        )
        .expect("first attempt");
        assert!(
            record_selection_attempt(
                (&mut attempted, &selection),
                (attempt_key.to_string(), vec!["[1,2)".to_string()],),
            )
            .is_err()
        );

        assert!(
            selected_constraint_version(&BTreeMap::new(), &BTreeMap::new(), &attempt_key).is_err()
        );
        assert!(
            selected_constraint_version(
                &BTreeMap::from([(attempt_key.clone(), NodeId(7))]),
                &BTreeMap::new(),
                &attempt_key,
            )
            .is_err()
        );

        let coordinate =
            ArtifactCoordinate::new("org.example", "system", "1").expect("system coordinate");
        let dependency = effective_dependency(coordinate.key.clone(), "1");
        assert!(required_system_path(&dependency, &coordinate).is_err());
        assert!(system_file_url(std::path::Path::new("relative"), &coordinate).is_err());
        assert!(selected_artifact_key(&coordinate, "/", None).is_err());
        assert!(selected_artifact_key(&coordinate, "jar", Some("/")).is_err());
    }

    #[test]
    fn covers_pom_repository_conversion_and_builder_validation() {
        let properties = BTreeMap::from([
            ("repo.id".to_string(), "declared".to_string()),
            (
                "repo.url".to_string(),
                "https://repo.example/maven".to_string(),
            ),
            ("enabled".to_string(), "true".to_string()),
            ("update".to_string(), "interval:15".to_string()),
            ("checksum".to_string(), "fail".to_string()),
        ]);
        let context = ResolutionContext::default();
        let policy: PomRepositoryPolicy = quick_xml::de::from_str(
            "<releases><enabled>${enabled}</enabled><updatePolicy>${update}</updatePolicy>\
             <checksumPolicy>${checksum}</checksumPolicy></releases>",
        )
        .expect("policy");
        let repository = PomRepository::builder("${repo.id}", "${repo.url}")
            .layout(RepositoryLayout::Default)
            .releases(policy.clone())
            .snapshots(policy)
            .build();
        let mut converted = Vec::new();
        append_pom_repositories(
            &mut converted,
            Some(&Repositories::from_vec(vec![repository.clone()])),
            &properties,
            &context,
        )
        .expect("repositories");
        let first = converted.first().expect("converted repository");
        assert_eq!(first.id, "declared");
        assert_eq!(first.releases.update, UpdatePolicy::IntervalMinutes(15));
        assert_eq!(first.snapshots.checksum, ChecksumPolicy::Fail);
        append_pom_repositories(
            &mut converted,
            Some(&Repositories::from_vec(vec![repository])),
            &properties,
            &context,
        )
        .expect("replace repository");
        assert_eq!(converted.len(), 1);
        append_pom_repositories(&mut converted, None, &properties, &context).expect("none");

        let legacy = PomRepository::builder("legacy", "https://repo.example")
            .layout(RepositoryLayout::Legacy)
            .build();
        assert!(
            append_pom_repositories(
                &mut Vec::new(),
                Some(&Repositories::from_vec(vec![legacy])),
                &properties,
                &context,
            )
            .is_err()
        );

        for (pom, expected) in [
            (PomUpdatePolicy::Always, UpdatePolicy::Always),
            (PomUpdatePolicy::Daily, UpdatePolicy::Daily),
            (PomUpdatePolicy::Never, UpdatePolicy::Never),
            (
                PomUpdatePolicy::Interval("interval:5".to_string()),
                UpdatePolicy::IntervalMinutes(5),
            ),
        ] {
            assert_eq!(pom_update_policy(Some(&pom)).expect("policy"), expected);
        }
        assert_eq!(
            pom_update_policy(None).expect("default"),
            UpdatePolicy::Daily
        );
        assert!(
            pom_update_policy(Some(&PomUpdatePolicy::Interval("invalid".to_string()))).is_err()
        );
        assert_eq!(
            pom_checksum_policy(Some(&PomChecksumPolicy::Fail)),
            ChecksumPolicy::Fail
        );
        assert_eq!(
            pom_checksum_policy(Some(&PomChecksumPolicy::Ignore)),
            ChecksumPolicy::Ignore
        );
        assert_eq!(
            pom_checksum_policy(Some(&PomChecksumPolicy::Warn)),
            ChecksumPolicy::Warn
        );
        assert_eq!(pom_checksum_policy(None), ChecksumPolicy::Warn);

        let invalid_update: PomRepositoryPolicy =
            quick_xml::de::from_str("<releases><updatePolicy>${missing}</updatePolicy></releases>")
                .expect("expression");
        assert!(interpolated_update_policy(&invalid_update, &properties, &context).is_err());
        let invalid_checksum: PomRepositoryPolicy = quick_xml::de::from_str(
            "<releases><checksumPolicy>${missing}</checksumPolicy></releases>",
        )
        .expect("expression");
        assert!(interpolated_checksum_policy(&invalid_checksum, &properties, &context).is_err());

        let reserved = RemoteRepository::new(SYSTEM_REPOSITORY_ID, "file:///").expect("reserved");
        assert!(
            Resolver::builder()
                .repositories(vec![reserved])
                .build()
                .is_err()
        );
        let first = RemoteRepository::new("duplicate", "https://one.example").expect("first");
        let second = RemoteRepository::new("duplicate", "https://two.example").expect("second");
        assert!(
            Resolver::builder()
                .repositories(vec![first, second])
                .build()
                .is_err()
        );
        let source = RemoteRepository::new("source", "https://source.example").expect("source");
        assert!(
            Resolver::builder()
                .repository(source.clone())
                .build()
                .is_ok()
        );
        let reserved_mirror = Mirror {
            repository: RemoteRepository::new(SYSTEM_REPOSITORY_ID, "file:///")
                .expect("reserved mirror"),
            mirror_of: "*".to_string(),
        };
        assert!(
            Resolver::builder()
                .repositories(vec![source])
                .mirror(reserved_mirror)
                .build()
                .is_err()
        );
        assert!(system_repository().expect("system").releases.enabled);

        let mut disabled = RepositoryPolicy::disabled();
        let enabled = RepositoryPolicy {
            update: UpdatePolicy::Always,
            ..RepositoryPolicy::enabled()
        };
        merge_repository_policy(&mut disabled, enabled);
        assert_eq!(disabled.update, UpdatePolicy::Always);

        let key = ArtifactKey::new("org.example", "demo").expect("key");
        let mismatched: RepositoryMetadata = quick_xml::de::from_str(
            "<metadata><groupId>other</groupId><artifactId>demo</artifactId></metadata>",
        )
        .expect("metadata");
        assert!(validate_metadata_key(&mismatched, &key, "memory://metadata").is_err());
    }

    #[test]
    fn covers_raw_coordinates_and_dependency_merge_precedence() {
        let context = ResolutionContext::default();
        let inherited = Project::builder("child")
            .parent(Parent::new("org.example", "parent", "1"))
            .build_project();
        assert_eq!(
            raw_project_coordinate(&inherited, &context)
                .expect("inherited coordinate")
                .to_string(),
            "org.example:child:1"
        );
        assert!(
            raw_project_coordinate(&Project::builder("child").build_project(), &context).is_none()
        );

        let source = Dependencies::from_vec(vec![
            Dependency::builder("org.example", "demo")
                .r#type("test-jar")
                .exclusion(Exclusion::new("org.excluded", "direct"))
                .build(),
        ]);
        // Type participates in the management key, so use a matching managed
        // test-jar entry to exercise every inherited technical value.
        let managed_test = dependency_from_pom(
            &Dependency::builder("org.example", "demo")
                .version("2")
                .r#type("test-jar")
                .scope(Runtime)
                .optional(true)
                .system_path("/managed")
                .exclusion(Exclusion::new("org.excluded", "managed"))
                .build(),
            &BTreeMap::new(),
            &context,
        )
        .expect("managed test dependency");
        let management = BTreeMap::from([(managed_test.key.clone(), managed_test)]);
        let mut destination = Vec::new();
        merge_dependencies(
            &mut destination,
            &source,
            &management,
            &BTreeMap::new(),
            &context,
        )
        .expect("managed merge");
        let merged = destination.first().expect("merged dependency");
        assert_eq!(merged.version.as_deref(), Some("2"));
        assert_eq!(merged.scope, Runtime);
        assert!(merged.optional);
        assert_eq!(merged.system_path.as_deref(), Some("/managed"));
        assert_eq!(merged.key.classifier.as_deref(), Some("tests"));

        let replacement = Dependencies::from_vec(vec![
            Dependency::builder("org.example", "demo")
                .version("3")
                .r#type("test-jar")
                .build(),
        ]);
        merge_dependencies(
            &mut destination,
            &replacement,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &context,
        )
        .expect("replacement merge");
        assert_eq!(destination.len(), 1);
        let merged = destination.first().expect("replaced dependency");
        assert_eq!(merged.version.as_deref(), Some("3"));
        assert_eq!(merged.scope, Runtime);
        assert!(merged.optional);

        let inherited_version = Dependencies::from_vec(vec![
            Dependency::builder("org.example", "demo")
                .r#type("test-jar")
                .build(),
        ]);
        merge_dependencies(
            &mut destination,
            &inherited_version,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &context,
        )
        .expect("inherit existing version");
        assert_eq!(
            destination
                .first()
                .expect("inherited dependency")
                .version
                .as_deref(),
            Some("3")
        );

        let mut repositories = vec![RemoteRepository::central()];
        retain_repositories(
            &mut repositories,
            &[
                RemoteRepository::central(),
                RemoteRepository::new("other", "https://other.example").expect("other"),
            ],
        );
        assert_eq!(repositories.len(), 2);

        let base = RemoteRepository::new("same", "https://one.example").expect("base");
        assert!(same_repository_settings(&base, &base));
        let mut different = base.clone();
        different.releases.enabled = false;
        assert!(!same_repository_settings(&base, &different));
        let mut different = base.clone();
        different.snapshots.enabled = false;
        assert!(!same_repository_settings(&base, &different));
        let mut different = base.clone();
        different.authentication = Some(Authentication::Bearer(SecretString::new("token")));
        assert!(!same_repository_settings(&base, &different));
        let mut different = base.clone();
        different.proxy = Some(Proxy::new("http://proxy.example").expect("proxy"));
        assert!(!same_repository_settings(&base, &different));

        let resolver = Resolver::builder()
            .repositories(vec![base.clone()])
            .build()
            .expect("resolver");
        let request = ResolutionRequest::new(
            ArtifactCoordinate::new("org.example", "root", "1").expect("root coordinate"),
        );
        let session = ResolutionSession::new(&resolver, &request);
        let same_url = RemoteRepository::new("alternate", "https://one.example").expect("same URL");
        assert_eq!(
            session
                .repositories_with_declared(std::slice::from_ref(&base), &[same_url])
                .len(),
            1
        );
        let occupied =
            RemoteRepository::new("same~declared-1", "https://occupied.example").expect("occupied");
        let declared =
            RemoteRepository::new("same", "https://two.example").expect("declared duplicate");
        let repositories = session.repositories_with_declared(&[base, occupied], &[declared]);
        assert!(repositories.iter().any(|repository| {
            repository.id == "same~declared-2" && repository.url == "https://two.example/"
        }));
        let known = repositories.first().expect("known repository").clone();
        assert_eq!(
            session
                .repositories_with_declared(&repositories, &[known])
                .len(),
            repositories.len()
        );
    }

    #[tokio::test]
    async fn covers_graph_resolution_policy_boundaries() {
        let resolver = Resolver::builder().build().expect("resolver");
        let a = ArtifactCoordinate::new("org.example", "a", "1").expect("a coordinate");
        let b_key = ArtifactKey::new("org.example", "b").expect("b key");
        let b = ArtifactCoordinate::from_key(b_key.clone(), "1").expect("b coordinate");

        let managed_root = br"<project><modelVersion>4.0.0</modelVersion>
            <groupId>org.example</groupId><artifactId>root</artifactId><version>1</version>
            <dependencyManagement><dependencies><dependency>
            <groupId>org.example</groupId><artifactId>b</artifactId><version>1</version>
            <scope>runtime</scope></dependency></dependencies></dependencyManagement>
            <dependencies><dependency><groupId>org.example</groupId><artifactId>a</artifactId>
            <version>1</version></dependency></dependencies></project>";
        let request = ResolutionRequest::from_project_bytes(managed_root.to_vec())
            .with_classpath(Classpath::Compile);
        let mut session = ResolutionSession::new(&resolver, &request);
        session.model_cache.insert(
            a.clone(),
            effective_model(a.clone(), vec![effective_dependency(b_key.clone(), "1")]),
        );
        let resolution = session.resolve().await.expect("managed resolution");
        assert!(
            resolution
                .nodes
                .iter()
                .all(|node| node.coordinate.key != b_key)
        );

        let excluded_root = br"<project><modelVersion>4.0.0</modelVersion>
            <groupId>org.example</groupId><artifactId>excluded-root</artifactId><version>1</version>
            <dependencies><dependency><groupId>org.example</groupId><artifactId>a</artifactId>
            <version>1</version><exclusions><exclusion><groupId>org.example</groupId>
            <artifactId>b</artifactId></exclusion></exclusions></dependency></dependencies></project>";
        let request = ResolutionRequest::from_project_bytes(excluded_root.to_vec());
        let mut invalid_excluded = effective_dependency(b_key.clone(), "..");
        invalid_excluded.scope = Compile;
        let mut session = ResolutionSession::new(&resolver, &request);
        session.model_cache.insert(
            a.clone(),
            effective_model(a.clone(), vec![invalid_excluded]),
        );
        assert!(session.resolve().await.is_err());

        let published_root =
            ArtifactCoordinate::new("org.example", "published-root", "1").expect("root");
        let invalid_key = ArtifactKey::new("org.example", "invalid").expect("invalid key");
        let invalid_dependency = effective_dependency(invalid_key.clone(), "..");
        let request =
            ResolutionRequest::new(published_root.clone()).with_override(OverrideRule::new(
                crate::OverrideMatcher::new("org.example", "invalid"),
                OverrideAction::Exclude,
            ));
        let mut session = ResolutionSession::new(&resolver, &request);
        session.model_cache.insert(
            published_root.clone(),
            effective_model(published_root.clone(), vec![invalid_dependency]),
        );
        assert!(session.resolve().await.is_err());

        let mut optional = effective_dependency(invalid_key, "/");
        optional.optional = true;
        let request = ResolutionRequest::new(published_root.clone());
        let mut session = ResolutionSession::new(&resolver, &request);
        session.model_cache.insert(
            published_root.clone(),
            effective_model(published_root.clone(), vec![optional]),
        );
        assert!(session.resolve().await.is_err());

        let request = ResolutionRequest::new(published_root.clone());
        let mut session = ResolutionSession::new(&resolver, &request);
        session.model_cache.insert(
            published_root.clone(),
            effective_model(
                published_root.clone(),
                vec![
                    effective_dependency(b_key.clone(), "1"),
                    effective_dependency(b_key.clone(), "LATEST"),
                ],
            ),
        );
        session
            .model_cache
            .insert(b.clone(), effective_model(b.clone(), Vec::new()));
        let resolution = session.resolve().await.expect("dynamic winner");
        assert!(resolution.nodes.iter().any(|node| {
            node.requested
                .as_ref()
                .is_some_and(|requested| requested.version == "LATEST")
        }));

        let target =
            ArtifactCoordinate::new("org.example", "target", "1").expect("target coordinate");
        let old = ArtifactCoordinate::new("org.example", "old", "1").expect("old coordinate");
        let request = ResolutionRequest::new(published_root.clone());
        let mut session = ResolutionSession::new(&resolver, &request);
        session.model_cache.insert(
            published_root.clone(),
            effective_model(
                published_root,
                vec![
                    effective_dependency(target.key.clone(), "1"),
                    effective_dependency(old.key.clone(), "1"),
                ],
            ),
        );
        session
            .model_cache
            .insert(target.clone(), effective_model(target.clone(), Vec::new()));
        let mut relocated = (*effective_model(old.clone(), Vec::new())).clone();
        relocated.relocation = Some(target);
        session.model_cache.insert(old, Arc::new(relocated));
        let resolution = session.resolve().await.expect("relocation conflict");
        assert!(
            resolution
                .nodes
                .iter()
                .any(|node| matches!(node.status, NodeStatus::Conflict { .. }))
        );

        let range_root = br"<project><modelVersion>4.0.0</modelVersion>
            <groupId>org.example</groupId><artifactId>range-root</artifactId><version>1</version>
            <dependencies><dependency><groupId>org.example</groupId><artifactId>b</artifactId>
            <version>[2,3)</version></dependency></dependencies></project>";
        let request = ResolutionRequest::new(b.clone()).with_root(ResolutionRoot::ProjectBytes {
            bytes: range_root.to_vec(),
            include_artifact: false,
        });
        let mut session = ResolutionSession::new(&resolver, &request);
        session
            .model_cache
            .insert(b.clone(), effective_model(b, Vec::new()));
        assert!(matches!(
            session.resolve().await,
            Err(Error::VersionConflict { .. })
        ));
    }

    #[tokio::test]
    async fn covers_local_system_and_artifact_selection_boundaries() {
        let resolver = Resolver::builder().build().expect("resolver");
        let missing_path = br"<project><modelVersion>4.0.0</modelVersion>
            <groupId>org.example</groupId><artifactId>system-root</artifactId><version>1</version>
            <dependencies><dependency><groupId>org.example</groupId><artifactId>system</artifactId>
            <version>1</version><scope>system</scope></dependency></dependencies></project>";
        assert!(
            resolver
                .resolve(
                    &ResolutionRequest::from_project_bytes(missing_path.to_vec())
                        .with_classpath(Classpath::Test)
                )
                .await
                .is_err()
        );

        let directory = TempDir::new().expect("system directory");
        let system_path = directory.path().join("system.jar");
        std::fs::write(&system_path, b"system").expect("system artifact");
        let system_project = |artifact: &str| {
            format!(
                "<project><modelVersion>4.0.0</modelVersion>\
                 <groupId>org.example</groupId><artifactId>{artifact}</artifactId><version>1</version>\
                 <dependencies><dependency><groupId>org.example</groupId>\
                 <artifactId>system</artifactId><version>1</version><scope>system</scope>\
                 <systemPath>{}</systemPath></dependency></dependencies></project>",
                system_path.display()
            )
            .into_bytes()
        };
        let request = ResolutionRequest::from_project_bytes(system_project("first"))
            .with_root(ResolutionRoot::ProjectBytes {
                bytes: system_project("second"),
                include_artifact: false,
            })
            .with_classpath(Classpath::Test);
        let resolution = resolver.resolve(&request).await.expect("system conflict");
        assert!(resolution.nodes.iter().any(|node| {
            node.coordinate.key.artifact_id == "system"
                && matches!(node.status, NodeStatus::Conflict { .. })
        }));

        let root =
            ArtifactCoordinate::new("org.example", "artifact-root", "1").expect("root coordinate");
        let request = ResolutionRequest::new(root);
        let session = ResolutionSession::new(&resolver, &request);
        let unavailable = EffectiveModel {
            coordinate: ArtifactCoordinate::new("org.example", "unavailable", "1")
                .expect("unavailable"),
            packaging: "jar".to_string(),
            properties: BTreeMap::new(),
            dependency_management: BTreeMap::new(),
            dependencies: Vec::new(),
            repositories: Vec::new(),
            relocation: None,
            repository_id: Some("missing".to_string()),
        };
        assert!(
            session
                .artifacts_for(NodeId(0), &unavailable, &[])
                .await
                .expect("empty artifact selection")
                .is_empty()
        );
    }

    #[tokio::test]
    async fn covers_effective_model_failure_boundaries() {
        let context = ResolutionContext::default();

        let raw_packaging_cycle = project(
            r"<project><modelVersion>4.0.0</modelVersion><groupId>org.example</groupId>
              <artifactId>raw-packaging</artifactId><version>1</version>
              <packaging>${loop}</packaging><properties><loop>${loop}</loop></properties></project>",
        );
        assert!(
            build_test_project(raw_packaging_cycle, context.clone(), Vec::new())
                .await
                .is_err()
        );

        let raw_repository_error = project(
            r"<project><modelVersion>4.0.0</modelVersion><groupId>org.example</groupId>
              <artifactId>raw-repository</artifactId><version>1</version><repositories>
              <repository><id>legacy</id><url>https://repo.example</url>
              <layout>legacy</layout></repository></repositories></project>",
        );
        assert!(
            build_test_project(raw_repository_error, context.clone(), Vec::new())
                .await
                .is_err()
        );

        let mut active = context.clone();
        active.active_profiles.insert("active".to_string());
        let raw_profile_repository_error = project(
            r"<project><modelVersion>4.0.0</modelVersion><groupId>org.example</groupId>
              <artifactId>raw-profile-repository</artifactId><version>1</version><profiles>
              <profile><id>active</id><repositories><repository><id>legacy</id>
              <url>https://repo.example</url><layout>legacy</layout></repository>
              </repositories></profile></profiles></project>",
        );
        assert!(
            build_test_project(raw_profile_repository_error, active.clone(), Vec::new())
                .await
                .is_err()
        );

        let profile_packaging_cycle = project(
            r"<project><modelVersion>4.0.0</modelVersion><groupId>org.example</groupId>
              <artifactId>profile-packaging</artifactId><version>1</version>
              <packaging>${kind}</packaging><properties><kind>jar</kind></properties><profiles>
              <profile><id>active</id><properties><kind>${kind}</kind></properties>
              </profile></profiles></project>",
        );
        assert!(
            build_test_project(profile_packaging_cycle, active.clone(), Vec::new())
                .await
                .is_err()
        );

        let profile_artifact_cycle = project(
            r"<project><modelVersion>4.0.0</modelVersion><groupId>org.example</groupId>
              <artifactId>${name}</artifactId><version>1</version>
              <properties><name>profile-artifact</name></properties><profiles>
              <profile><id>active</id><properties><name>${name}</name></properties>
              </profile></profiles></project>",
        );
        assert!(
            build_test_project(profile_artifact_cycle, active.clone(), Vec::new())
                .await
                .is_err()
        );

        let final_packaging_cycle = project(
            r"<project><modelVersion>4.0.0</modelVersion><groupId>org.example</groupId>
              <artifactId>final-packaging</artifactId><version>1</version>
              <packaging>${project.packaging}</packaging></project>",
        );
        assert!(
            build_test_project(final_packaging_cycle, context.clone(), Vec::new())
                .await
                .is_err()
        );

        let dependency_cycle = project(
            r"<project><modelVersion>4.0.0</modelVersion><groupId>org.example</groupId>
              <artifactId>dependency-cycle</artifactId><version>1</version>
              <properties><loop>${loop}</loop></properties><dependencies><dependency>
              <groupId>${loop}</groupId><artifactId>child</artifactId><version>1</version>
              </dependency></dependencies></project>",
        );
        assert!(
            build_test_project(dependency_cycle, context.clone(), Vec::new())
                .await
                .is_err()
        );

        let profile_dependency_cycle = project(
            r"<project><modelVersion>4.0.0</modelVersion><groupId>org.example</groupId>
              <artifactId>profile-dependency-cycle</artifactId><version>1</version>
              <properties><loop>${loop}</loop></properties><profiles><profile><id>active</id>
              <dependencies><dependency><groupId>${loop}</groupId><artifactId>child</artifactId>
              <version>1</version></dependency></dependencies></profile></profiles></project>",
        );
        assert!(
            build_test_project(profile_dependency_cycle, active.clone(), Vec::new())
                .await
                .is_err()
        );

        let profile_management = project(
            r"<project><modelVersion>4.0.0</modelVersion><groupId>org.example</groupId>
              <artifactId>profile-management</artifactId><version>1</version><profiles>
              <profile><id>active</id><dependencyManagement><dependencies><dependency>
              <groupId>org.example</groupId><artifactId>managed</artifactId><version>1</version>
              </dependency></dependencies></dependencyManagement></profile></profiles></project>",
        );
        let model = build_test_project(profile_management, active.clone(), Vec::new())
            .await
            .expect("profile management");
        assert_eq!(model.dependency_management.len(), 1);

        let mut bom_without_version = project(
            r"<project><modelVersion>4.0.0</modelVersion><groupId>org.example</groupId>
              <artifactId>missing-bom-version</artifactId><version>1</version>
              <dependencyManagement><dependencies><dependency><groupId>org.example</groupId>
              <artifactId>bom</artifactId><version>1</version><type>pom</type><scope>import</scope>
              </dependency></dependencies></dependencyManagement></project>",
        );
        bom_without_version
            .dependency_management
            .as_mut()
            .expect("dependency management")
            .dependencies
            .dependencies
            .first_mut()
            .expect("BOM dependency")
            .version = None;
        assert!(
            build_test_project(bom_without_version, context.clone(), Vec::new())
                .await
                .is_err()
        );

        let directory = TempDir::new().expect("parent directory");
        let child_directory = directory.path().join("child");
        std::fs::create_dir(&child_directory).expect("child directory");
        let mut parent_context = context.clone();
        parent_context.base_directory = Some(child_directory.clone());

        let empty_relative_parent = project(
            r"<project><modelVersion>4.0.0</modelVersion><parent>
              <groupId>org.example</groupId><artifactId>parent</artifactId><version>1</version>
              <relativePath></relativePath></parent><artifactId>empty-relative</artifactId></project>",
        );
        assert!(
            build_test_project(empty_relative_parent, parent_context.clone(), Vec::new())
                .await
                .is_err()
        );

        std::fs::create_dir(directory.path().join("parent")).expect("parent path directory");
        let directory_parent = project(
            r"<project><modelVersion>4.0.0</modelVersion><parent>
              <groupId>org.example</groupId><artifactId>parent</artifactId><version>1</version>
              <relativePath>../parent</relativePath></parent><artifactId>directory-parent</artifactId>
              </project>",
        );
        assert!(
            build_test_project(directory_parent, parent_context.clone(), Vec::new())
                .await
                .is_err()
        );

        let parent_file = directory.path().join("parent.xml");
        std::fs::write(
            &parent_file,
            r"<project><modelVersion>4.0.0</modelVersion><groupId>org.example</groupId>
              <artifactId>parent</artifactId><version>1</version><packaging>pom</packaging></project>",
        )
        .expect("parent POM");
        let cyclic_parent = project(
            r"<project><modelVersion>4.0.0</modelVersion><parent>
              <groupId>org.example</groupId><artifactId>parent</artifactId><version>1</version>
              <relativePath>../parent.xml</relativePath></parent><artifactId>cyclic-parent</artifactId>
              </project>",
        );
        assert!(matches!(
            build_test_project(
                cyclic_parent,
                parent_context.clone(),
                vec![
                    ArtifactCoordinate::new("org.example", "parent", "1")
                        .expect("parent coordinate")
                ]
            )
            .await,
            Err(Error::Cycle(_))
        ));

        std::fs::write(
            &parent_file,
            r"<project><modelVersion>4.0.0</modelVersion><groupId>org.example</groupId>
              <artifactId>${name}</artifactId><version>1</version><packaging>pom</packaging>
              <properties><name>parent</name></properties><profiles><profile><id>active</id>
              <properties><name>other</name></properties></profile></profiles></project>",
        )
        .expect("mismatched parent POM");
        let mismatched_parent = project(
            r"<project><modelVersion>4.0.0</modelVersion><parent>
              <groupId>org.example</groupId><artifactId>parent</artifactId><version>1</version>
              <relativePath>../parent.xml</relativePath></parent>
              <artifactId>mismatched-parent</artifactId></project>",
        );
        assert!(
            build_test_project(
                mismatched_parent,
                {
                    let mut context = parent_context.clone();
                    context.active_profiles.insert("active".to_string());
                    context
                },
                Vec::new()
            )
            .await
            .is_err()
        );

        std::fs::write(
            &parent_file,
            r"<project><modelVersion>4.0.0</modelVersion><groupId>org.example</groupId>
              <artifactId>parent</artifactId><version>1</version><packaging>pom</packaging>
              <properties><kind>${kind}</kind></properties></project>",
        )
        .expect("packaging parent POM");
        let inherited_packaging_cycle = project(
            r"<project><modelVersion>4.0.0</modelVersion><parent>
              <groupId>org.example</groupId><artifactId>parent</artifactId><version>1</version>
              <relativePath>../parent.xml</relativePath></parent>
              <artifactId>inherited-packaging</artifactId><packaging>${kind}</packaging></project>",
        );
        assert!(
            build_test_project(
                inherited_packaging_cycle,
                parent_context.clone(),
                Vec::new()
            )
            .await
            .is_err()
        );

        std::fs::write(
            &parent_file,
            r"<project><modelVersion>4.0.0</modelVersion><groupId>org.example</groupId>
              <artifactId>parent</artifactId><version>1</version><packaging>pom</packaging>
              <properties><bad>with space</bad></properties></project>",
        )
        .expect("property parent POM");
        let invalid_inherited_repository = project(
            r"<project><modelVersion>4.0.0</modelVersion><parent>
              <groupId>org.example</groupId><artifactId>parent</artifactId><version>1</version>
              <relativePath>../parent.xml</relativePath></parent><artifactId>invalid-repo</artifactId>
              <repositories><repository><id>${bad}</id><url>https://repo.example</url>
              </repository></repositories></project>",
        );
        assert!(
            build_test_project(
                invalid_inherited_repository,
                parent_context.clone(),
                Vec::new()
            )
            .await
            .is_err()
        );

        let invalid_profile_repository = project(
            r"<project><modelVersion>4.0.0</modelVersion><parent>
              <groupId>org.example</groupId><artifactId>parent</artifactId><version>1</version>
              <relativePath>../parent.xml</relativePath></parent>
              <artifactId>invalid-profile-repo</artifactId><profiles><profile><id>active</id>
              <repositories><repository><id>${bad}</id><url>https://repo.example</url>
              </repository></repositories></profile></profiles></project>",
        );
        let mut inherited_active = parent_context;
        inherited_active
            .active_profiles
            .insert("active".to_string());
        assert!(
            build_test_project(invalid_profile_repository, inherited_active, Vec::new())
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn covers_metadata_and_materialization_failure_boundaries() {
        let mut disabled =
            RemoteRepository::new("disabled", "memory://repository/").expect("disabled repository");
        disabled.releases.enabled = false;
        disabled.snapshots.enabled = false;
        let (resolver, _) = scripted_resolver(disabled, TestBehavior::Missing);
        assert!(matches!(
            resolver
                .available_versions(&ArtifactKey::new("org.example", "demo").expect("key"))
                .await,
            Err(Error::MetadataNotFound(_))
        ));

        let repository =
            RemoteRepository::new("memory", "memory://repository/").expect("repository");
        let coordinate = ArtifactCoordinate::new("org.example", "demo", "1").expect("coordinate");
        let (missing_metadata_resolver, _) =
            scripted_resolver(repository.clone(), TestBehavior::Missing);
        assert!(
            missing_metadata_resolver
                .available_versions(&coordinate.key)
                .await
                .is_err()
        );
        let (metadata_resolver, metadata_repository) =
            scripted_resolver(repository.clone(), TestBehavior::Metadata);
        let listing = metadata_resolver
            .available_versions(&coordinate.key)
            .await
            .expect("available versions");
        assert_eq!(listing.versions.len(), 1);
        let metadata_request = TransportRequest {
            repository: metadata_repository,
            url: "memory://repository/artifact.jar".to_string(),
            kind: ResourceKind::Artifact,
        };
        assert!(
            TestTransport(TestBehavior::MalformedChecksum)
                .get(&metadata_request)
                .await
                .expect("malformed transport artifact")
                .is_none()
        );

        let (resolver, repository) = scripted_resolver(repository, TestBehavior::Missing);
        let resolution = scripted_resolution(repository.clone(), coordinate.clone());
        assert!(matches!(
            resolver.download(&resolution, &Sink).await,
            Err(Error::ArtifactNotFound(_))
        ));

        let mut omitted = resolution.clone();
        omitted.repositories.clear();
        assert!(matches!(
            resolver.download(&omitted, &Sink).await,
            Err(Error::InvalidConfiguration(_))
        ));

        let mut invalid = resolution.clone();
        invalid
            .artifacts
            .first_mut()
            .expect("invalid artifact")
            .coordinate
            .version = "..".to_string();
        assert!(resolver.download(&invalid, &Sink).await.is_err());
        let mut invalid_file = resolution.clone();
        invalid_file
            .artifacts
            .first_mut()
            .expect("invalid file artifact")
            .file_version = "..".to_string();
        assert!(resolver.download(&invalid_file, &Sink).await.is_err());

        let mut invalid_repository = resolution.clone();
        invalid_repository
            .repositories
            .first_mut()
            .expect("invalid repository")
            .url = "not a URL".to_string();
        assert!(resolver.download(&invalid_repository, &Sink).await.is_err());

        let mut disabled_repository = resolution.clone();
        disabled_repository
            .repositories
            .first_mut()
            .expect("disabled repository")
            .releases
            .enabled = false;
        assert!(matches!(
            resolver.download(&disabled_repository, &Sink).await,
            Err(Error::ArtifactNotFound(_))
        ));

        let mut fallback_failure = resolution.clone();
        let mut invalid_fallback = RemoteRepository::new("invalid-fallback", "memory://fallback/")
            .expect("fallback repository");
        invalid_fallback.url = "not a URL".to_string();
        fallback_failure.repositories.push(invalid_fallback);
        assert!(resolver.download(&fallback_failure, &Sink).await.is_err());

        let (resolver, repository) =
            scripted_resolver(repository.clone(), TestBehavior::TransportError);
        let mut transport_error_resolution =
            scripted_resolution(repository.clone(), coordinate.clone());
        transport_error_resolution
            .repositories
            .first_mut()
            .expect("transport error repository")
            .releases
            .checksum = ChecksumPolicy::Ignore;
        assert!(
            resolver
                .download(&transport_error_resolution, &Sink)
                .await
                .is_err()
        );
        let strict_transport_resolution =
            scripted_resolution(repository.clone(), coordinate.clone());
        assert!(
            resolver
                .download(&strict_transport_resolution, &Sink)
                .await
                .is_err()
        );

        let mut length_repository = repository.clone();
        length_repository.releases.checksum = ChecksumPolicy::Ignore;
        let (resolver, length_repository) =
            scripted_resolver(length_repository, TestBehavior::LengthMismatch);
        assert!(
            resolver
                .available_versions(&ArtifactKey::new("org.example", "demo").expect("key"))
                .await
                .is_err()
        );
        assert!(
            resolver
                .download(
                    &scripted_resolution(length_repository, coordinate.clone()),
                    &Sink,
                )
                .await
                .is_err()
        );

        let (resolver, mut repository) =
            scripted_resolver(repository.clone(), TestBehavior::StreamError);
        repository.releases.checksum = ChecksumPolicy::Ignore;
        assert!(
            resolver
                .download(
                    &scripted_resolution(repository.clone(), coordinate.clone()),
                    &Sink,
                )
                .await
                .is_err()
        );

        let mut checksum_repository = repository.clone();
        checksum_repository.releases.checksum = ChecksumPolicy::Fail;
        let (resolver, checksum_repository) =
            scripted_resolver(checksum_repository, TestBehavior::MalformedChecksum);
        assert!(
            resolver
                .download(
                    &scripted_resolution(checksum_repository, coordinate.clone()),
                    &Sink
                )
                .await
                .is_err()
        );

        let mut no_artifact_repository = repository.clone();
        no_artifact_repository.releases.checksum = ChecksumPolicy::Warn;
        let (resolver, no_artifact_repository) =
            scripted_resolver(no_artifact_repository, TestBehavior::MalformedChecksum);
        assert!(
            resolver
                .download(
                    &scripted_resolution(no_artifact_repository, coordinate.clone()),
                    &Sink
                )
                .await
                .is_err()
        );

        let mut successful_repository = repository.clone();
        successful_repository.releases.checksum = ChecksumPolicy::Ignore;
        let (resolver, successful_repository) =
            scripted_resolver(successful_repository, TestBehavior::Bytes);
        let report = resolver
            .download(
                &scripted_resolution(successful_repository, coordinate.clone()),
                &Sink,
            )
            .await
            .expect("successful scripted materialization");
        assert_eq!(
            report
                .artifacts
                .first()
                .map(|artifact| artifact.output.as_slice()),
            Some(b"artifact".as_slice())
        );

        let cache_directory = TempDir::new().expect("cache directory");
        let mut cached_repository = repository.clone();
        cached_repository.releases.checksum = ChecksumPolicy::Ignore;
        let cache_target = cache_directory.path().join(coordinate.artifact_path());
        std::fs::create_dir_all(&cache_target).expect("cache target directory");
        let resolver = Resolver::builder()
            .repositories(vec![cached_repository.clone()])
            .transports(vec![Arc::new(TestTransport(TestBehavior::Bytes))])
            .file_cache(crate::FileCache::new(cache_directory.path()))
            .build()
            .expect("cached resolver");
        assert!(
            resolver
                .download(
                    &scripted_resolution(cached_repository, coordinate.clone()),
                    &Sink,
                )
                .await
                .is_err()
        );

        let (resolver, oversized_repository) =
            scripted_resolver(repository, TestBehavior::Oversized);
        let metadata_url = oversized_repository
            .resource_url(
                &ArtifactKey::new("org.example", "demo")
                    .expect("key")
                    .metadata_path(),
            )
            .expect("metadata URL");
        assert!(
            resolver
                .fetch_small(&oversized_repository, &metadata_url, ResourceKind::Metadata,)
                .await
                .is_err()
        );

        let cache_directory = TempDir::new().expect("oversized cache directory");
        let cache = crate::FileCache::new(cache_directory.path());
        cache
            .store_small(
                &oversized_repository,
                &metadata_url,
                &vec![0_u8; 16 * 1024 * 1024 + 1],
            )
            .await
            .expect("oversized cache fixture");
        let resolver = Resolver::builder()
            .repositories(vec![oversized_repository.clone()])
            .transports(vec![Arc::new(TestTransport(TestBehavior::Missing))])
            .file_cache(cache)
            .offline(true)
            .build()
            .expect("offline oversized resolver");
        assert!(
            resolver
                .fetch_small(&oversized_repository, &metadata_url, ResourceKind::Metadata)
                .await
                .is_err()
        );

        let snapshot =
            ArtifactCoordinate::new("org.example", "demo", "1-SNAPSHOT").expect("snapshot");
        let snapshot_repository =
            RemoteRepository::new("snapshot", "memory://snapshot/").expect("snapshot repository");
        let (snapshot_resolver, snapshot_repository) =
            scripted_resolver(snapshot_repository, TestBehavior::InvalidSnapshot);
        assert!(
            snapshot_resolver
                .snapshot_file_version(&snapshot, &snapshot_repository, "jar", None)
                .await
                .is_err()
        );
        let non_snapshot_request = TransportRequest {
            repository: snapshot_repository,
            url: "memory://snapshot/artifact.jar".to_string(),
            kind: ResourceKind::Artifact,
        };
        assert!(
            TestTransport(TestBehavior::InvalidSnapshot)
                .get(&non_snapshot_request)
                .await
                .expect("invalid snapshot non-metadata request")
                .is_none()
        );
    }

    #[tokio::test]
    async fn covers_version_selection_repository_boundaries() {
        let mut disabled =
            RemoteRepository::new("disabled", "memory://repository/").expect("repository");
        disabled.releases.enabled = false;
        disabled.snapshots.enabled = false;
        let (resolver, disabled) = scripted_resolver(disabled, TestBehavior::Missing);
        let root = ArtifactCoordinate::new("org.example", "root", "1").expect("root coordinate");
        let request = ResolutionRequest::new(root);
        let session = ResolutionSession::new(&resolver, &request);
        let coordinate = ArtifactCoordinate::new("org.example", "demo", "1").expect("coordinate");
        assert!(
            session
                .fetch_project(&coordinate, std::slice::from_ref(&disabled))
                .await
                .is_err()
        );
        let mut events = Vec::new();
        assert!(
            session
                .select_coordinate(
                    &coordinate.key,
                    "LATEST",
                    std::slice::from_ref(&disabled),
                    &mut events,
                )
                .await
                .is_err()
        );
        let constraints = vec![(
            "[1,2)".to_string(),
            "[1,2)".parse::<VersionSpec>().expect("constraint"),
        )];
        assert!(
            session
                .select_version_matching_all(
                    &coordinate.key,
                    &constraints,
                    std::slice::from_ref(&disabled),
                )
                .await
                .expect("disabled selection")
                .is_none()
        );

        let enabled =
            RemoteRepository::new("enabled", "memory://enabled/").expect("enabled repository");
        let (resolver, enabled) = scripted_resolver(enabled, TestBehavior::Missing);
        let request = ResolutionRequest::new(
            ArtifactCoordinate::new("org.example", "root", "1").expect("root"),
        );
        let session = ResolutionSession::new(&resolver, &request);
        assert!(
            session
                .select_version_matching_all(
                    &coordinate.key,
                    &constraints,
                    std::slice::from_ref(&enabled),
                )
                .await
                .expect("missing metadata selection")
                .is_none()
        );

        let repository =
            RemoteRepository::new("metadata", "memory://metadata/").expect("repository");
        let (resolver, repository) = scripted_resolver(repository, TestBehavior::Metadata);
        let request = ResolutionRequest::new(
            ArtifactCoordinate::new("org.example", "root", "1").expect("root"),
        );
        let session = ResolutionSession::new(&resolver, &request);
        assert_eq!(
            session
                .select_version_matching_all(
                    &coordinate.key,
                    &constraints,
                    std::slice::from_ref(&repository),
                )
                .await
                .expect("matching selection"),
            Some(Version::new("1"))
        );
    }

    #[tokio::test]
    async fn covers_imported_bom_exclusion_boundary() {
        let resolver = Resolver::builder().offline(true).build().expect("resolver");
        let request = ResolutionRequest::from_project_bytes(b"<project/>".to_vec());
        let repositories = resolver.repositories.clone();
        let mut session = ResolutionSession::new(&resolver, &request);
        let bom_key = ArtifactKey::new("org.example", "bom")
            .and_then(|key| key.with_extension("pom"))
            .expect("BOM key");
        let bom_coordinate =
            ArtifactCoordinate::from_key(bom_key.clone(), "1").expect("BOM coordinate");
        let managed_key = ArtifactKey::new("org.example", "managed").expect("managed key");
        let mut management = BTreeMap::new();
        management.insert(managed_key.clone(), effective_dependency(managed_key, "1"));
        session.model_cache.insert(
            bom_coordinate.clone(),
            Arc::new(EffectiveModel {
                coordinate: bom_coordinate,
                packaging: "pom".to_string(),
                properties: BTreeMap::new(),
                dependency_management: management,
                dependencies: Vec::new(),
                repositories: Vec::new(),
                relocation: None,
                repository_id: Some(crate::CENTRAL_REPOSITORY_ID.to_string()),
            }),
        );
        let project = project(
            r"<project><modelVersion>4.0.0</modelVersion><groupId>org.example</groupId>
              <artifactId>bom-root</artifactId><version>1</version>
              <dependencyManagement><dependencies><dependency><groupId>org.example</groupId>
              <artifactId>bom</artifactId><version>1</version><type>pom</type><scope>import</scope>
              <exclusions><exclusion><groupId>org.example</groupId><artifactId>managed</artifactId>
              </exclusion></exclusions></dependency></dependencies></dependencyManagement></project>",
        );
        let model = session
            .build_effective_model(project, None, None, &repositories, &mut Vec::new())
            .await
            .expect("excluded BOM management");
        assert!(model.dependency_management.is_empty());
    }
}
