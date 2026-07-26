#![cfg(not(target_family = "wasm"))]
#![expect(
    clippy::panic_in_result_fn,
    reason = "assertions provide clearer diagnostics in integration tests"
)]

use bytes::Bytes;
use md5::Md5;
use ristretto_resolver::{
    ArtifactCoordinate, ArtifactDestination, ArtifactKey, ArtifactSelection, ArtifactTransaction,
    Authentication, BoxFuture, CertificateEncoding, ChecksumPolicy, Classpath, Error, FileCache,
    FileDestination, HttpTransport, Mirror, NodeStatus, OverrideAction, OverrideMatcher,
    OverrideRule, Proxy, RemoteRepository, ResolutionContext, ResolutionEvent, ResolutionRequest,
    ResolvedArtifact, Resolver, Result, SecretString, TlsConfiguration, Transport,
    TransportRequest, TransportResponse, Version,
};
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha512};
use std::collections::BTreeSet;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use tempfile::TempDir;
use url::Url;

const EMPTY_DEPENDENCIES: &str = "";

#[test]
fn configures_tls_trust_through_the_public_transport_api() -> Result<()> {
    let configuration = TlsConfiguration::default()
        .with_pem_certificate(b"PEM certificate".to_vec())
        .with_der_certificate(vec![1, 2, 3])
        .only_configured_roots();
    let transport = HttpTransport::with_tls(configuration);
    assert!(!transport.tls().default_roots);
    assert_eq!(transport.tls().trust_anchors.len(), 2);
    assert_eq!(
        transport
            .tls()
            .trust_anchors
            .first()
            .map(|anchor| anchor.encoding),
        Some(CertificateEncoding::Pem)
    );
    Resolver::builder().transport(Arc::new(transport)).build()?;
    Ok(())
}

#[derive(Debug, Default)]
struct MemoryDestination;

#[derive(Debug, Default)]
struct MemoryTransaction(Vec<u8>);

#[derive(Debug)]
struct RepositoryAwareTransport;

#[derive(Debug, Clone)]
struct RecordingTransport {
    repositories: Arc<StdMutex<Vec<RemoteRepository>>>,
}

impl Transport for RecordingTransport {
    fn supports(&self, scheme: &str) -> bool {
        scheme == "memory"
    }

    fn get<'a>(
        &'a self,
        request: &'a TransportRequest,
    ) -> BoxFuture<'a, Result<Option<TransportResponse>>> {
        Box::pin(async move {
            self.repositories
                .lock()
                .map_err(|error| Error::InvalidConfiguration(error.to_string()))?
                .push(request.repository.clone());
            let xml = pom("org.test", "secured", "1.0", "", EMPTY_DEPENDENCIES);
            Ok(Some(TransportResponse {
                body: Box::pin(futures_util::stream::once(
                    async move { Ok(Bytes::from(xml)) },
                )),
                content_length: None,
                from_cache: false,
            }))
        })
    }
}

impl Transport for RepositoryAwareTransport {
    fn supports(&self, scheme: &str) -> bool {
        scheme == "memory"
    }

    fn get<'a>(
        &'a self,
        request: &'a TransportRequest,
    ) -> BoxFuture<'a, Result<Option<TransportResponse>>> {
        Box::pin(async move {
            let version = if request.repository.id == "tenant-a" {
                "1.0"
            } else {
                "2.0"
            };
            let xml = format!(
                "<metadata><groupId>org.test</groupId><artifactId>isolated</artifactId>\
                 <versioning><versions><version>{version}</version></versions></versioning></metadata>"
            );
            Ok(Some(TransportResponse {
                body: Box::pin(futures_util::stream::once(
                    async move { Ok(Bytes::from(xml)) },
                )),
                content_length: None,
                from_cache: false,
            }))
        })
    }
}

impl ArtifactDestination for MemoryDestination {
    type Transaction = MemoryTransaction;
    type Output = Vec<u8>;

    fn begin<'a>(
        &'a self,
        _artifact: &'a ResolvedArtifact,
    ) -> BoxFuture<'a, Result<Self::Transaction>> {
        Box::pin(async { Ok(MemoryTransaction::default()) })
    }
}

impl ArtifactTransaction for MemoryTransaction {
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

fn pom(
    group: &str,
    artifact: &str,
    version: &str,
    dependency_management: &str,
    dependencies: &str,
) -> String {
    format!(
        r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
  <modelVersion>4.0.0</modelVersion>
  <groupId>{group}</groupId>
  <artifactId>{artifact}</artifactId>
  <version>{version}</version>
  {dependency_management}
  <dependencies>{dependencies}</dependencies>
</project>"#
    )
}

fn dependency(group: &str, artifact: &str, version: &str) -> String {
    format!(
        r"<dependency><groupId>{group}</groupId><artifactId>{artifact}</artifactId><version>{version}</version></dependency>"
    )
}

fn artifact_directory(root: &Path, group: &str, artifact: &str, version: &str) -> PathBuf {
    root.join(group.replace('.', "/"))
        .join(artifact)
        .join(version)
}

fn publish(root: &Path, group: &str, artifact: &str, version: &str, pom_xml: &str) -> Result<()> {
    let directory = artifact_directory(root, group, artifact, version);
    fs::create_dir_all(&directory)?;
    fs::write(directory.join(format!("{artifact}-{version}.pom")), pom_xml)?;
    let jar = format!("fixture:{group}:{artifact}:{version}").into_bytes();
    let jar_path = directory.join(format!("{artifact}-{version}.jar"));
    fs::write(&jar_path, &jar)?;
    let checksum =
        Sha256::digest(&jar)
            .iter()
            .fold(String::with_capacity(64), |mut checksum, byte| {
                let _ = write!(checksum, "{byte:02x}");
                checksum
            });
    fs::write(jar_path.with_extension("jar.sha256"), checksum)?;
    Ok(())
}

fn publish_metadata(root: &Path, group: &str, artifact: &str, versions: &[&str]) -> Result<()> {
    let directory = root.join(group.replace('.', "/")).join(artifact);
    fs::create_dir_all(&directory)?;
    let versions = versions.iter().fold(String::new(), |mut xml, version| {
        let _ = write!(xml, "<version>{version}</version>");
        xml
    });
    fs::write(
        directory.join("maven-metadata.xml"),
        format!(
            "<metadata><groupId>{group}</groupId><artifactId>{artifact}</artifactId>\
             <versioning><latest>3.0</latest><release>3.0</release>\
             <versions>{versions}</versions><lastUpdated>20260725000000</lastUpdated>\
             </versioning></metadata>"
        ),
    )?;
    Ok(())
}

fn fixture_repository() -> Result<TempDir> {
    let repository = TempDir::new()?;
    publish(
        repository.path(),
        "org.test",
        "common",
        "1.0",
        &pom("org.test", "common", "1.0", "", EMPTY_DEPENDENCIES),
    )?;
    publish(
        repository.path(),
        "org.test",
        "common",
        "1.5",
        &pom("org.test", "common", "1.5", "", EMPTY_DEPENDENCIES),
    )?;
    publish(
        repository.path(),
        "org.test",
        "common",
        "2.0",
        &pom("org.test", "common", "2.0", "", EMPTY_DEPENDENCIES),
    )?;
    publish(
        repository.path(),
        "org.test",
        "common",
        "3.0",
        &pom("org.test", "common", "3.0", "", EMPTY_DEPENDENCIES),
    )?;
    publish_metadata(
        repository.path(),
        "org.test",
        "common",
        &["1.0", "1.5", "2.0", "3.0"],
    )?;
    publish(
        repository.path(),
        "org.test",
        "alpha",
        "1.0",
        &pom(
            "org.test",
            "alpha",
            "1.0",
            "",
            &dependency("org.test", "common", "1.0"),
        ),
    )?;
    publish(
        repository.path(),
        "org.test",
        "beta",
        "1.0",
        &pom(
            "org.test",
            "beta",
            "1.0",
            "",
            &dependency("org.test", "common", "2.0"),
        ),
    )?;
    let management = format!(
        "<dependencyManagement><dependencies>{}</dependencies></dependencyManagement>",
        dependency("org.test", "common", "1.5")
    );
    let dependencies = format!(
        "{}{}",
        dependency("org.test", "alpha", "1.0"),
        dependency("org.test", "beta", "1.0")
    );
    publish(
        repository.path(),
        "org.test",
        "app",
        "1.0",
        &pom("org.test", "app", "1.0", &management, &dependencies),
    )?;
    Ok(repository)
}

#[tokio::test]
async fn session_cache_is_isolated_by_repository_identity() -> Result<()> {
    let repositories = vec![
        RemoteRepository::new("tenant-a", "memory://repository/")?,
        RemoteRepository::new("tenant-b", "memory://repository/")?,
    ];
    let resolver = Resolver::builder()
        .repositories(repositories)
        .transport(Arc::new(RepositoryAwareTransport))
        .build()?;
    let listing = resolver
        .available_versions(&ArtifactKey::new("org.test", "isolated")?)
        .await?;

    assert_eq!(
        listing
            .versions
            .iter()
            .map(|version| version.version.to_string())
            .collect::<Vec<_>>(),
        ["1.0", "2.0"]
    );
    Ok(())
}

#[tokio::test]
async fn exposes_the_staged_resolution_pipeline() -> Result<()> {
    let repository = fixture_repository()?;
    let resolver = resolver(repository.path(), None)?;
    let coordinate = ArtifactCoordinate::new("org.test", "app", "1.0")?;

    let loaded = resolver.model_loader().load(&coordinate).await?;
    assert_eq!(loaded.coordinate, coordinate);
    assert_eq!(loaded.project.artifact_id, "app");
    let versions = resolver
        .model_loader()
        .available_versions(&ArtifactKey::new("org.test", "common")?)
        .await?;
    assert_eq!(versions.versions.len(), 4);

    let model = resolver
        .effective_model_builder()
        .build(
            ristretto_resolver::ResolutionRoot::Artifact(coordinate.clone()),
            ResolutionContext::default(),
        )
        .await?;
    assert_eq!(model.coordinate, coordinate);
    assert!(!model.dependencies.is_empty());

    let request = ResolutionRequest::new(coordinate)
        .with_conflict_policy(ristretto_resolver::ConflictPolicy::NearestFirst);
    let resolution = resolver.graph_resolver().resolve(&request).await?;
    let report = resolver
        .materializer()
        .materialize(&resolution, &MemoryDestination)
        .await?;
    assert_eq!(report.artifacts.len(), resolution.artifacts.len());
    let repositories = resolution.repositories.clone();
    let (locked, report) = resolver
        .materializer()
        .materialize_lock(resolution.to_lock()?, repositories, &MemoryDestination)
        .await?;
    assert!(!locked.repository_fallback);
    assert_eq!(report.artifacts.len(), locked.artifacts.len());

    let local_model = pom(
        "org.test",
        "app",
        "1.0",
        "",
        &dependency("org.test", "alpha", "1.0"),
    );
    let local_request = ResolutionRequest::from_project_bytes(local_model)
        .with_artifacts(ArtifactSelection::default());
    let mut local_request = local_request;
    if let Some(ristretto_resolver::ResolutionRoot::ProjectBytes {
        include_artifact, ..
    }) = local_request.roots.first_mut()
    {
        *include_artifact = true;
    }
    let local_resolution = resolver.resolve(&local_request).await?;
    assert!(local_resolution.artifacts.iter().any(|artifact| {
        artifact.node == ristretto_resolver::NodeId(0)
            && artifact.coordinate.to_string() == "org.test:app:1.0"
    }));
    assert!(
        local_resolution
            .nodes
            .first()
            .is_some_and(|node| node.repository_id.is_some())
    );
    let downloaded = resolver
        .download(&local_resolution, &MemoryDestination)
        .await?;
    assert_eq!(downloaded.artifacts.len(), local_resolution.artifacts.len());
    Ok(())
}

#[tokio::test]
async fn validates_builder_requests_and_offline_transport_boundaries() -> Result<()> {
    assert!(
        Resolver::builder()
            .repositories(Vec::new())
            .build()
            .is_err()
    );
    assert!(Resolver::builder().transports(Vec::new()).build().is_err());
    assert!(
        Resolver::builder()
            .repositories(vec![RemoteRepository::new(
                "__system",
                "https://repo.example/"
            )?])
            .build()
            .is_err()
    );
    assert!(
        Resolver::builder()
            .repositories(vec![
                RemoteRepository::new("duplicate", "https://one.example/")?,
                RemoteRepository::new("duplicate", "https://two.example/")?,
            ])
            .build()
            .is_err()
    );

    let unsupported = Resolver::builder()
        .repositories(vec![RemoteRepository::new(
            "memory",
            "memory://repository/",
        )?])
        .build()?;
    assert!(matches!(
        unsupported
            .available_versions(&ArtifactKey::new("org.test", "missing")?)
            .await,
        Err(Error::UnsupportedTransport(_))
    ));

    let offline = Resolver::builder()
        .repositories(vec![RemoteRepository::new(
            "offline",
            "https://offline.example/",
        )?])
        .offline(true)
        .build()?;
    assert!(matches!(
        offline
            .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
                "org.test", "missing", "1.0"
            )?))
            .await,
        Err(Error::Offline(_))
    ));

    let mut empty = ResolutionRequest::from_project_bytes(
        b"<project><modelVersion>4.0.0</modelVersion><groupId>org.test</groupId>\
          <artifactId>empty</artifactId><version>1</version></project>"
            .to_vec(),
    )
    .with_context(ResolutionContext::default())
    .with_transitive_repositories(false)
    .with_artifacts(ArtifactSelection::default());
    empty.roots.clear();
    assert!(matches!(
        unsupported.resolve(&empty).await,
        Err(Error::InvalidConfiguration(_))
    ));
    Ok(())
}

#[tokio::test]
#[expect(
    clippy::too_many_lines,
    reason = "one end-to-end fixture verifies relocation mediation, cycles, roots, and ranges"
)]
async fn resolves_relocations_and_explains_dependency_cycles() -> Result<()> {
    let repository = fixture_repository()?;
    let relocation = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
      <modelVersion>4.0.0</modelVersion><groupId>org.test</groupId>
      <artifactId>legacy</artifactId><version>1.0</version>
      <distributionManagement><relocation><groupId>org.test</groupId>
        <artifactId>common</artifactId><version>2.0</version>
      </relocation></distributionManagement>
    </project>"#;
    publish(repository.path(), "org.test", "legacy", "1.0", relocation)?;
    publish_metadata(repository.path(), "org.test", "legacy", &["1.0"])?;
    let a_dependencies = dependency("org.test", "cycle-b", "1.0");
    let b_dependencies = dependency("org.test", "cycle-a", "1.0");
    publish(
        repository.path(),
        "org.test",
        "cycle-a",
        "1.0",
        &pom("org.test", "cycle-a", "1.0", "", &a_dependencies),
    )?;
    publish(
        repository.path(),
        "org.test",
        "cycle-b",
        "1.0",
        &pom("org.test", "cycle-b", "1.0", "", &b_dependencies),
    )?;
    let project = pom(
        "org.local",
        "relocation-cycle",
        "1.0",
        "",
        &format!(
            "{}{}{}",
            dependency("org.test", "legacy", "[1,2]"),
            dependency("org.test", "common", "2.0"),
            dependency("org.test", "cycle-a", "1.0")
        ),
    );

    let resolution = resolver(repository.path(), None)?
        .resolve(&ResolutionRequest::from_project_bytes(project))
        .await?;
    let relocated = resolution
        .nodes
        .iter()
        .find(|node| {
            node.events
                .iter()
                .any(|event| matches!(event, ResolutionEvent::Relocated { .. }))
        })
        .ok_or_else(|| Error::InvalidConfiguration("missing relocation event".to_string()))?;
    assert_eq!(relocated.coordinate.to_string(), "org.test:common:2.0");
    assert_eq!(
        resolution
            .nodes
            .iter()
            .filter(|node| node.coordinate.key.artifact_id == "common")
            .filter(|node| node.status == NodeStatus::Selected)
            .count(),
        1
    );
    assert_eq!(
        resolution
            .artifacts
            .iter()
            .filter(|artifact| artifact.coordinate.key.artifact_id == "common")
            .count(),
        1
    );
    assert!(resolution.nodes.iter().any(|node| {
        node.coordinate.key.artifact_id == "common"
            && matches!(node.status, NodeStatus::Conflict { .. })
    }));
    assert!(
        resolution
            .nodes
            .iter()
            .any(|node| node.status == NodeStatus::Cycle)
    );

    let root_resolution = resolver(repository.path(), None)?
        .resolve(
            &ResolutionRequest::new(ArtifactCoordinate::new("org.test", "legacy", "1.0")?)
                .with_root(ristretto_resolver::ResolutionRoot::Artifact(
                    ArtifactCoordinate::new("org.test", "common", "2.0")?,
                )),
        )
        .await?;
    assert_eq!(
        root_resolution
            .nodes
            .iter()
            .filter(|node| node.coordinate.key.artifact_id == "common")
            .filter(|node| node.status == NodeStatus::Selected)
            .count(),
        1
    );
    assert_eq!(root_resolution.artifacts.len(), 1);

    let ranged_relocation = resolver(repository.path(), None)?
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.test", "legacy", "[1,2]",
        )?))
        .await?;
    let root = ranged_relocation
        .nodes
        .first()
        .ok_or_else(|| Error::InvalidConfiguration("missing relocated range root".to_string()))?;
    assert_eq!(root.coordinate.to_string(), "org.test:common:2.0");
    assert!(root.events.iter().any(|event| matches!(
        event,
        ResolutionEvent::VersionSelected {
            specification,
            selected
        } if specification == "[1,2]" && selected == "1.0"
    )));
    assert!(root.events.iter().any(|event| matches!(
        event,
        ResolutionEvent::Relocated { from, to }
            if from.to_string() == "org.test:legacy:1.0"
                && to.to_string() == "org.test:common:2.0"
    )));
    Ok(())
}

#[tokio::test]
async fn rejects_parent_cycles_and_invalid_local_system_dependencies() -> Result<()> {
    let repository = fixture_repository()?;
    let parent_a = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
      <modelVersion>4.0.0</modelVersion>
      <parent><groupId>org.test</groupId><artifactId>parent-b</artifactId>
        <version>1.0</version><relativePath/></parent>
      <groupId>org.test</groupId><artifactId>parent-a</artifactId><version>1.0</version>
      <packaging>pom</packaging>
    </project>"#;
    let parent_b = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
      <modelVersion>4.0.0</modelVersion>
      <parent><groupId>org.test</groupId><artifactId>parent-a</artifactId>
        <version>1.0</version><relativePath/></parent>
      <groupId>org.test</groupId><artifactId>parent-b</artifactId><version>1.0</version>
      <packaging>pom</packaging>
    </project>"#;
    publish(repository.path(), "org.test", "parent-a", "1.0", parent_a)?;
    publish(repository.path(), "org.test", "parent-b", "1.0", parent_b)?;
    assert!(matches!(
        resolver(repository.path(), None)?
            .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
                "org.test", "parent-a", "1.0"
            )?))
            .await,
        Err(Error::Cycle(_))
    ));

    let invalid_dependencies = [
        "<dependency><groupId>org.local</groupId><artifactId>system</artifactId>\
         <version>[1,2]</version><scope>system</scope><systemPath>/missing</systemPath></dependency>",
        "<dependency><groupId>org.local</groupId><artifactId>system</artifactId>\
         <version>1</version><scope>system</scope></dependency>",
        "<dependency><groupId>org.local</groupId><artifactId>system</artifactId>\
         <version>1</version><scope>system</scope><systemPath>relative.jar</systemPath></dependency>",
        "<dependency><groupId>org.local</groupId><artifactId>system</artifactId>\
         <version>1</version><scope>system</scope><systemPath>/definitely/missing.jar</systemPath>\
         </dependency>",
    ];
    for (index, dependency) in invalid_dependencies.into_iter().enumerate() {
        let project = pom("org.local", "invalid-system", "1", "", dependency);
        let result = resolver(repository.path(), None)?
            .resolve(
                &ResolutionRequest::from_project_bytes(project).with_classpath(Classpath::Compile),
            )
            .await;
        assert!(
            matches!(result, Err(Error::Model { .. } | Error::Pom(_))),
            "system case {index}: {result:?}"
        );
    }
    Ok(())
}

#[tokio::test]
async fn handles_snapshot_metadata_fallback_and_coordinate_mismatch() -> Result<()> {
    let repository = fixture_repository()?;
    publish(
        repository.path(),
        "org.test",
        "plain-snapshot",
        "1.0-SNAPSHOT",
        &pom(
            "org.test",
            "plain-snapshot",
            "1.0-SNAPSHOT",
            "",
            EMPTY_DEPENDENCIES,
        ),
    )?;
    let resolution = resolver(repository.path(), None)?
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.test",
            "plain-snapshot",
            "1.0-SNAPSHOT",
        )?))
        .await?;
    assert_eq!(
        resolution
            .artifacts
            .first()
            .map(|artifact| artifact.file_version.as_str()),
        Some("1.0-SNAPSHOT")
    );

    publish(
        repository.path(),
        "org.test",
        "bad-snapshot",
        "1.0-SNAPSHOT",
        &pom(
            "org.test",
            "bad-snapshot",
            "1.0-SNAPSHOT",
            "",
            EMPTY_DEPENDENCIES,
        ),
    )?;
    let directory = artifact_directory(
        repository.path(),
        "org.test",
        "bad-snapshot",
        "1.0-SNAPSHOT",
    );
    fs::write(
        directory.join("maven-metadata.xml"),
        "<metadata><groupId>org.other</groupId><artifactId>bad-snapshot</artifactId>\
         <version>1.0-SNAPSHOT</version><versioning/></metadata>",
    )?;
    assert!(
        resolver(repository.path(), None)?
            .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
                "org.test",
                "bad-snapshot",
                "1.0-SNAPSHOT",
            )?))
            .await
            .is_err()
    );
    Ok(())
}

#[tokio::test]
async fn reports_unmanaged_versions_and_applies_replace_and_exclude_overrides() -> Result<()> {
    let repository = fixture_repository()?;
    let unmanaged = pom(
        "org.local",
        "unmanaged",
        "1",
        "",
        "<dependency><groupId>org.test</groupId><artifactId>common</artifactId></dependency>",
    );
    assert!(matches!(
        resolver(repository.path(), None)?
            .resolve(&ResolutionRequest::from_project_bytes(unmanaged))
            .await,
        Err(Error::Model { .. })
    ));

    publish(
        repository.path(),
        "org.test",
        "replacement",
        "1.0",
        &pom("org.test", "replacement", "1.0", "", EMPTY_DEPENDENCIES),
    )?;
    let replace = OverrideRule::new(
        OverrideMatcher::new("org.test", "common"),
        OverrideAction::Replace(ArtifactCoordinate::new("org.test", "replacement", "1.0")?),
    );
    let resolution = resolver(repository.path(), None)?
        .resolve(
            &ResolutionRequest::new(ArtifactCoordinate::new("org.test", "alpha", "1.0")?)
                .with_override(replace),
        )
        .await?;
    assert!(resolution.nodes.iter().any(|node| {
        node.coordinate.key.artifact_id == "replacement"
            && node
                .events
                .iter()
                .any(|event| matches!(event, ResolutionEvent::Override { .. }))
    }));

    let exclude = OverrideRule::new(
        OverrideMatcher::new("org.test", "common"),
        OverrideAction::Exclude,
    );
    let resolution = resolver(repository.path(), None)?
        .resolve(
            &ResolutionRequest::new(ArtifactCoordinate::new("org.test", "alpha", "1.0")?)
                .with_override(exclude),
        )
        .await?;
    assert!(
        resolution
            .nodes
            .iter()
            .any(|node| node.status == NodeStatus::OverriddenOut)
    );
    Ok(())
}

#[tokio::test]
async fn routes_mirror_authentication_and_proxy_configuration_to_transport() -> Result<()> {
    let mut source = RemoteRepository::new("private", "memory://private/")?;
    source.snapshots = ristretto_resolver::RepositoryPolicy::disabled();

    let mut proxy = Proxy::new("https://proxy.example/")?;
    proxy.authentication = Some(Authentication::Basic {
        username: "proxy-user".to_string(),
        password: SecretString::new("proxy-secret"),
    });
    proxy.non_proxy_hosts = vec!["localhost|*.internal".to_string()];

    let mut mirror_repository = RemoteRepository::new("private-mirror", "memory://mirror/")?;
    mirror_repository.authentication = Some(Authentication::Bearer(SecretString::new("token")));
    mirror_repository.proxy = Some(proxy.clone());
    let mirror = Mirror {
        repository: mirror_repository,
        mirror_of: "private".to_string(),
    };
    let repositories = Arc::new(StdMutex::new(Vec::new()));
    let transport = RecordingTransport {
        repositories: Arc::clone(&repositories),
    };
    let resolver = Resolver::builder()
        .repositories(vec![source.clone()])
        .mirror(mirror)
        .transport(Arc::new(transport))
        .build()?;

    let loaded = resolver
        .model_loader()
        .load(&ArtifactCoordinate::new("org.test", "secured", "1.0")?)
        .await?;

    assert_eq!(loaded.repository_id, "private-mirror");
    let repositories = repositories
        .lock()
        .map_err(|error| Error::InvalidConfiguration(error.to_string()))?;
    assert!(!repositories.is_empty());
    assert!(repositories.iter().all(|repository| {
        repository.id == "private-mirror"
            && repository.releases == source.releases
            && repository.snapshots == source.snapshots
            && repository.authentication == Some(Authentication::Bearer(SecretString::new("token")))
            && repository.proxy.as_ref() == Some(&proxy)
    }));
    Ok(())
}

fn resolver(repository: &Path, cache: Option<&Path>) -> Result<Resolver> {
    let repository_url = Url::from_directory_path(repository)
        .map_err(|()| {
            Error::InvalidConfiguration("fixture repository path is not absolute".to_string())
        })?
        .to_string();
    let builder =
        Resolver::builder().repositories(vec![RemoteRepository::new("fixture", repository_url)?]);
    let builder = if let Some(cache) = cache {
        builder.file_cache(FileCache::new(cache))
    } else {
        builder
    };
    builder.build()
}

#[tokio::test]
async fn resolves_managed_transitives_and_retains_conflicts() -> Result<()> {
    let repository = fixture_repository()?;
    let resolver = resolver(repository.path(), None)?;
    let request = ResolutionRequest::new(ArtifactCoordinate::new("org.test", "app", "1.0")?);

    let resolution = resolver.resolve(&request).await?;
    let selected = resolution
        .selected_dependencies()
        .into_iter()
        .map(|node| node.coordinate.to_string())
        .collect::<Vec<_>>();

    assert_eq!(
        selected,
        [
            "org.test:app:1.0",
            "org.test:alpha:1.0",
            "org.test:beta:1.0",
            "org.test:common:1.5",
        ]
    );
    assert_eq!(resolution.artifacts.len(), 4);
    assert!(
        resolution
            .nodes
            .iter()
            .any(|node| matches!(node.status, NodeStatus::Conflict { .. }))
    );
    assert_eq!(
        resolution
            .paths_to(&ArtifactKey::new("org.test", "common")?)
            .len(),
        2
    );
    Ok(())
}

#[tokio::test]
async fn first_declared_root_wins_same_key_conflicts() -> Result<()> {
    let repository = fixture_repository()?;
    let request = ResolutionRequest::new(ArtifactCoordinate::new("org.test", "common", "1.0")?)
        .with_root(ristretto_resolver::ResolutionRoot::Artifact(
            ArtifactCoordinate::new("org.test", "common", "2.0")?,
        ));
    let resolution = resolver(repository.path(), None)?.resolve(&request).await?;

    assert_eq!(
        resolution
            .selected_dependencies()
            .iter()
            .filter(|node| node.coordinate.key.artifact_id == "common")
            .map(|node| node.coordinate.version.as_str())
            .collect::<Vec<_>>(),
        ["1.0"]
    );
    assert!(matches!(
        resolution.nodes.get(1).map(|node| &node.status),
        Some(NodeStatus::Conflict { .. })
    ));
    assert_eq!(resolution.artifacts.len(), 1);
    Ok(())
}

#[tokio::test]
async fn rejects_repository_pom_with_mismatched_coordinates() -> Result<()> {
    let repository = TempDir::new()?;
    let requested_directory =
        artifact_directory(repository.path(), "org.expected", "module", "1.0");
    fs::create_dir_all(&requested_directory)?;
    fs::write(
        requested_directory.join("module-1.0.pom"),
        pom("org.wrong", "module", "1.0", "", EMPTY_DEPENDENCIES),
    )?;
    let result = resolver(repository.path(), None)?
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.expected",
            "module",
            "1.0",
        )?))
        .await;
    assert!(matches!(result, Err(Error::Model { .. })));

    let malformed = ArtifactCoordinate::new("org.expected", "malformed", "1.0")?;
    let malformed_directory = artifact_directory(
        repository.path(),
        &malformed.key.group_id,
        &malformed.key.artifact_id,
        &malformed.version,
    );
    fs::create_dir_all(&malformed_directory)?;
    fs::write(
        malformed_directory.join("malformed-1.0.pom"),
        "<project><modelVersion>4.0.0</modelVersion></project>",
    )?;
    let result = resolver(repository.path(), None)?
        .resolve(&ResolutionRequest::new(malformed.clone()))
        .await;
    assert!(matches!(
        result,
        Err(Error::Model {
            coordinate,
            message
        }) if coordinate == malformed && message.contains("XML error")
    ));
    Ok(())
}

#[tokio::test]
async fn applies_transitive_version_override() -> Result<()> {
    let repository = fixture_repository()?;
    publish(
        repository.path(),
        "org.alternate",
        "replacement",
        "9.0",
        &pom(
            "org.alternate",
            "replacement",
            "9.0",
            "",
            EMPTY_DEPENDENCIES,
        ),
    )?;
    let resolver = resolver(repository.path(), None)?;
    let request = ResolutionRequest::new(ArtifactCoordinate::new("org.test", "app", "1.0")?)
        .with_override(OverrideRule::new(
            OverrideMatcher::new("org.test", "common"),
            OverrideAction::ForceVersion(Version::new("3.0")),
        ));

    let resolution = resolver.resolve(&request).await?;
    let common = resolution
        .selected_dependencies()
        .into_iter()
        .find(|node| node.coordinate.key.artifact_id == "common")
        .ok_or_else(|| Error::InvalidConfiguration("common dependency not selected".to_string()))?;
    assert_eq!(common.coordinate.version, "3.0");
    assert!(
        common
            .events
            .iter()
            .any(|event| matches!(event, ResolutionEvent::Override { .. }))
    );

    let excluded = resolver
        .resolve(
            &ResolutionRequest::new(ArtifactCoordinate::new("org.test", "app", "1.0")?)
                .with_override(OverrideRule::new(
                    OverrideMatcher::new("org.test", "common"),
                    OverrideAction::Exclude,
                )),
        )
        .await?;
    assert!(
        excluded
            .nodes
            .iter()
            .any(|node| node.status == NodeStatus::OverriddenOut)
    );
    assert!(
        excluded
            .selected_dependencies()
            .iter()
            .all(|node| node.coordinate.key.artifact_id != "common")
    );

    let replaced = resolver
        .resolve(
            &ResolutionRequest::new(ArtifactCoordinate::new("org.test", "app", "1.0")?)
                .with_override(OverrideRule::new(
                    OverrideMatcher::new("org.test", "common"),
                    OverrideAction::Replace(ArtifactCoordinate::new(
                        "org.alternate",
                        "replacement",
                        "9.0",
                    )?),
                )),
        )
        .await?;
    assert!(
        replaced
            .selected_dependencies()
            .iter()
            .any(|node| node.coordinate.to_string() == "org.alternate:replacement:9.0")
    );
    Ok(())
}

#[tokio::test]
async fn lists_versions_and_downloads_to_file_destination() -> Result<()> {
    let repository = fixture_repository()?;
    let cache = TempDir::new()?;
    let destination = TempDir::new()?;
    let resolver = resolver(repository.path(), Some(cache.path()))?;

    let versions = resolver
        .available_versions(&ArtifactKey::new("org.test", "common")?)
        .await?;
    assert_eq!(
        versions
            .versions
            .iter()
            .map(|version| version.version.to_string())
            .collect::<Vec<_>>(),
        ["1.0", "1.5", "2.0", "3.0"]
    );

    let request = ResolutionRequest::new(ArtifactCoordinate::new("org.test", "app", "1.0")?);
    let (resolution, report) = resolver
        .resolve_and_download(&request, &FileDestination::new(destination.path()))
        .await?;
    assert_eq!(report.artifacts.len(), 4);
    assert!(report.diagnostics.is_empty());
    for downloaded in report.artifacts {
        assert!(downloaded.output.is_file());
        assert_eq!(
            downloaded
                .checksum
                .as_ref()
                .map(|checksum| checksum.algorithm.as_str()),
            Some("SHA-256")
        );
    }
    assert_eq!(resolution.transitive_dependencies().len(), 3);
    assert!(cache.path().join("org/test/app/1.0/app-1.0.jar").is_file());
    Ok(())
}

#[tokio::test]
async fn streams_to_a_caller_defined_destination() -> Result<()> {
    let repository = fixture_repository()?;
    let resolver = resolver(repository.path(), None)?;
    let resolution = resolver
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.test", "app", "1.0",
        )?))
        .await?;

    let report = resolver.download(&resolution, &MemoryDestination).await?;

    assert_eq!(report.artifacts.len(), 4);
    assert!(
        report
            .artifacts
            .iter()
            .all(|artifact| artifact.output.starts_with(b"fixture:org.test:"))
    );
    Ok(())
}

#[tokio::test]
async fn selects_poms_and_attached_artifacts_independently_from_main_artifacts() -> Result<()> {
    let repository = fixture_repository()?;
    let directory = artifact_directory(repository.path(), "org.test", "common", "1.0");
    let sources = b"fixture sources";
    let sources_path = directory.join("common-1.0-sources.jar");
    fs::write(&sources_path, sources)?;
    let checksum =
        Sha256::digest(sources)
            .iter()
            .fold(String::with_capacity(64), |mut checksum, byte| {
                let _ = write!(checksum, "{byte:02x}");
                checksum
            });
    fs::write(sources_path.with_extension("jar.sha256"), checksum)?;

    let request = ResolutionRequest::new(ArtifactCoordinate::new("org.test", "common", "1.0")?)
        .with_artifacts(ArtifactSelection {
            main_artifacts: false,
            poms: true,
            attachments: vec![
                ("sources".to_string(), "jar".to_string()),
                ("sources".to_string(), "jar".to_string()),
            ],
        });
    let resolver = resolver(repository.path(), None)?;
    let resolution = resolver.resolve(&request).await?;
    assert_eq!(resolution.artifacts.len(), 2);
    assert!(
        resolution
            .artifacts
            .iter()
            .any(|artifact| artifact.coordinate.key.extension == "pom")
    );
    assert!(
        resolution
            .artifacts
            .iter()
            .any(|artifact| artifact.coordinate.key.classifier.as_deref() == Some("sources"))
    );

    let report = resolver.download(&resolution, &MemoryDestination).await?;
    assert_eq!(report.artifacts.len(), 2);
    assert!(
        report
            .artifacts
            .iter()
            .any(|artifact| artifact.output == sources)
    );
    Ok(())
}

#[tokio::test]
async fn serializes_and_materializes_a_resolution_lock() -> Result<()> {
    let repository = fixture_repository()?;
    let resolver = resolver(repository.path(), None)?;
    let resolution = resolver
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.test", "common", "1.0",
        )?))
        .await?;
    let repositories = resolution.repositories.clone();
    let json = serde_json::to_string_pretty(&resolution.to_lock()?).map_err(|error| {
        Error::InvalidConfiguration(format!("serialize resolution lock: {error}"))
    })?;
    assert!(!json.contains("authentication"));
    let lock = serde_json::from_str(&json).map_err(|error| {
        Error::InvalidConfiguration(format!("deserialize resolution lock: {error}"))
    })?;
    let restored = ristretto_resolver::ResolutionLock::attach_repositories(lock, repositories)?;

    let report = resolver.download(&restored, &MemoryDestination).await?;
    assert_eq!(report.artifacts.len(), 1);
    assert_eq!(
        report.artifacts.first().expect("locked artifact").output,
        b"fixture:org.test:common:1.0"
    );
    Ok(())
}

#[tokio::test]
async fn downloads_artifact_from_a_later_repository_when_descriptor_repository_lacks_it()
-> Result<()> {
    let descriptor_repository = TempDir::new()?;
    let artifact_repository = TempDir::new()?;
    let coordinate = ArtifactCoordinate::new("org.test", "split", "1.0")?;
    let pom_xml = pom("org.test", "split", "1.0", "", EMPTY_DEPENDENCIES);
    publish(
        descriptor_repository.path(),
        "org.test",
        "split",
        "1.0",
        &pom_xml,
    )?;
    let descriptor_directory =
        artifact_directory(descriptor_repository.path(), "org.test", "split", "1.0");
    fs::remove_file(descriptor_directory.join("split-1.0.jar"))?;
    fs::remove_file(descriptor_directory.join("split-1.0.jar.sha256"))?;

    publish(
        artifact_repository.path(),
        "org.test",
        "split",
        "1.0",
        &pom_xml,
    )?;
    fs::remove_file(
        artifact_directory(artifact_repository.path(), "org.test", "split", "1.0")
            .join("split-1.0.pom"),
    )?;

    let repositories = [
        ("descriptors", descriptor_repository.path()),
        ("artifacts", artifact_repository.path()),
    ]
    .into_iter()
    .map(|(id, path)| {
        let url = Url::from_directory_path(path).map_err(|()| {
            Error::InvalidConfiguration("fixture repository path is not absolute".to_string())
        })?;
        RemoteRepository::new(id, url.to_string())
    })
    .collect::<Result<Vec<_>>>()?;
    let resolver = Resolver::builder().repositories(repositories).build()?;
    let resolution = resolver
        .resolve(&ResolutionRequest::new(coordinate))
        .await?;
    assert_eq!(
        resolution
            .artifacts
            .first()
            .map(|artifact| artifact.repository_id.as_str()),
        Some("descriptors")
    );

    let report = resolver.download(&resolution, &MemoryDestination).await?;
    let downloaded = report.artifacts.first().ok_or_else(|| {
        Error::InvalidConfiguration("split artifact was not downloaded".to_string())
    })?;
    assert_eq!(downloaded.artifact.repository_id, "artifacts");
    assert_eq!(downloaded.output, b"fixture:org.test:split:1.0");
    Ok(())
}

#[tokio::test]
async fn retries_a_later_repository_after_checksum_mismatch() -> Result<()> {
    let corrupted_repository = TempDir::new()?;
    let healthy_repository = TempDir::new()?;
    let pom_xml = pom("org.test", "fallback", "1.0", "", EMPTY_DEPENDENCIES);
    publish(
        corrupted_repository.path(),
        "org.test",
        "fallback",
        "1.0",
        &pom_xml,
    )?;
    publish(
        healthy_repository.path(),
        "org.test",
        "fallback",
        "1.0",
        &pom_xml,
    )?;
    fs::write(
        artifact_directory(corrupted_repository.path(), "org.test", "fallback", "1.0")
            .join("fallback-1.0.jar"),
        b"corrupted",
    )?;

    let repositories = [
        ("corrupted", corrupted_repository.path()),
        ("healthy", healthy_repository.path()),
    ]
    .into_iter()
    .map(|(id, path)| {
        let url = Url::from_directory_path(path).map_err(|()| {
            Error::InvalidConfiguration("fixture repository path is not absolute".to_string())
        })?;
        RemoteRepository::new(id, url.to_string())
    })
    .collect::<Result<Vec<_>>>()?;
    let resolver = Resolver::builder().repositories(repositories).build()?;
    let resolution = resolver
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.test", "fallback", "1.0",
        )?))
        .await?;

    let report = resolver.download(&resolution, &MemoryDestination).await?;
    let downloaded = report.artifacts.first().ok_or_else(|| {
        Error::InvalidConfiguration("fallback artifact was not downloaded".to_string())
    })?;
    assert_eq!(downloaded.artifact.repository_id, "healthy");
    assert_eq!(downloaded.output, b"fixture:org.test:fallback:1.0");
    Ok(())
}

#[tokio::test]
async fn rejects_tampered_artifact_urls_before_downloading() -> Result<()> {
    let repository = fixture_repository()?;
    let resolver = resolver(repository.path(), None)?;
    let mut resolution = resolver
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.test", "app", "1.0",
        )?))
        .await?;
    resolution
        .artifacts
        .first_mut()
        .ok_or_else(|| Error::InvalidConfiguration("root artifact is missing".to_string()))?
        .url = "file:///etc/passwd".to_string();

    let result = resolver.download(&resolution, &MemoryDestination).await;
    assert!(matches!(result, Err(Error::InvalidConfiguration(_))));
    Ok(())
}

#[tokio::test]
async fn annotates_optional_and_excluded_dependencies() -> Result<()> {
    let repository = fixture_repository()?;
    let dependencies = "<dependency><groupId>org.test</groupId><artifactId>alpha</artifactId>\
        <version>1.0</version><exclusions><exclusion><groupId>org.test</groupId>\
        <artifactId>common</artifactId></exclusion></exclusions></dependency>\
        <dependency><groupId>org.test</groupId><artifactId>beta</artifactId>\
        <version>1.0</version><optional>true</optional></dependency>";
    publish(
        repository.path(),
        "org.test",
        "filtered-app",
        "1.0",
        &pom("org.test", "filtered-app", "1.0", "", dependencies),
    )?;
    let resolver = resolver(repository.path(), None)?;

    let resolution = resolver
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.test",
            "filtered-app",
            "1.0",
        )?))
        .await?;

    assert!(resolution.nodes.iter().any(
        |node| node.coordinate.key.artifact_id == "beta" && node.status == NodeStatus::Optional
    ));
    assert!(
        resolution
            .nodes
            .iter()
            .any(|node| node.coordinate.key.artifact_id == "common"
                && node.status == NodeStatus::Excluded)
    );
    Ok(())
}

#[tokio::test]
async fn optional_dynamic_transitives_do_not_require_metadata() -> Result<()> {
    let repository = fixture_repository()?;
    let dependencies = "<dependency><groupId>org.missing</groupId>\
        <artifactId>optional-range</artifactId><version>[9,10]</version>\
        <optional>true</optional></dependency>";
    publish(
        repository.path(),
        "org.test",
        "optional-app",
        "1.0",
        &pom("org.test", "optional-app", "1.0", "", dependencies),
    )?;

    let resolution = resolver(repository.path(), None)?
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.test",
            "optional-app",
            "1.0",
        )?))
        .await?;
    assert!(resolution.nodes.iter().any(|node| {
        node.coordinate.key.artifact_id == "optional-range" && node.status == NodeStatus::Optional
    }));
    Ok(())
}

#[tokio::test]
async fn provided_dependencies_are_direct_only() -> Result<()> {
    let repository = fixture_repository()?;
    let dependencies = "<dependency><groupId>org.test</groupId><artifactId>alpha</artifactId>\
        <version>1.0</version><scope>provided</scope></dependency>";
    publish(
        repository.path(),
        "org.test",
        "provided-app",
        "1.0",
        &pom("org.test", "provided-app", "1.0", "", dependencies),
    )?;

    let resolution = resolver(repository.path(), None)?
        .resolve(
            &ResolutionRequest::new(ArtifactCoordinate::new("org.test", "provided-app", "1.0")?)
                .with_classpath(Classpath::Test),
        )
        .await?;
    assert!(resolution.selected_dependencies().iter().any(|node| {
        node.coordinate.key.artifact_id == "alpha"
            && node.scope == ristretto_pom::DependencyScope::Provided
    }));
    assert!(
        resolution
            .selected_dependencies()
            .iter()
            .all(|node| node.coordinate.key.artifact_id != "common")
    );
    Ok(())
}

#[tokio::test]
async fn remote_profiles_do_not_use_the_consuming_projects_base_directory() -> Result<()> {
    let repository = fixture_repository()?;
    let remote = format!(
        r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
          <modelVersion>4.0.0</modelVersion><groupId>org.test</groupId>
          <artifactId>remote-profile</artifactId><version>1.0</version>
          <profiles><profile><id>local-file</id><activation><file>
            <exists>activate-remote.txt</exists></file></activation>
            <dependencies>{}</dependencies>
          </profile></profiles>
        </project>"#,
        dependency("org.test", "common", "1.0")
    );
    publish(
        repository.path(),
        "org.test",
        "remote-profile",
        "1.0",
        &remote,
    )?;
    let project = TempDir::new()?;
    fs::write(project.path().join("activate-remote.txt"), b"present")?;
    fs::write(
        project.path().join("pom.xml"),
        pom(
            "org.local",
            "consumer",
            "1.0",
            "",
            &dependency("org.test", "remote-profile", "1.0"),
        ),
    )?;

    let resolution = resolver(repository.path(), None)?
        .resolve(&ResolutionRequest::from_project_file(
            project.path().join("pom.xml"),
        ))
        .await?;
    assert!(
        resolution
            .selected_dependencies()
            .iter()
            .all(|node| node.coordinate.key.artifact_id != "common")
    );
    Ok(())
}

#[tokio::test]
async fn rejects_system_dependencies_from_repository_poms() -> Result<()> {
    let repository = fixture_repository()?;
    let dependencies = "<dependency><groupId>org.local</groupId><artifactId>secret</artifactId>\
        <version>1.0</version><scope>system</scope><systemPath>/etc/passwd</systemPath></dependency>";
    publish(
        repository.path(),
        "org.test",
        "unsafe-system",
        "1.0",
        &pom("org.test", "unsafe-system", "1.0", "", dependencies),
    )?;
    let result = resolver(repository.path(), None)?
        .resolve(
            &ResolutionRequest::new(ArtifactCoordinate::new("org.test", "unsafe-system", "1.0")?)
                .with_classpath(Classpath::Test),
        )
        .await;
    assert!(matches!(result, Err(Error::Model { .. })));
    Ok(())
}

#[tokio::test]
async fn imports_bom_and_activates_dependency_profile() -> Result<()> {
    let repository = fixture_repository()?;
    let bom_management = format!(
        "<packaging>pom</packaging><dependencyManagement><dependencies>{}</dependencies></dependencyManagement>",
        dependency("org.test", "common", "2.0")
    );
    publish(
        repository.path(),
        "org.test",
        "platform",
        "1.0",
        &pom(
            "org.test",
            "platform",
            "1.0",
            &bom_management,
            EMPTY_DEPENDENCIES,
        ),
    )?;
    let import = "<dependencyManagement><dependencies><dependency>\
                  <groupId>org.test</groupId><artifactId>platform</artifactId>\
                  <version>1.0</version><type>pom</type><scope>import</scope>\
                  </dependency></dependencies></dependencyManagement>";
    publish(
        repository.path(),
        "org.test",
        "bom-app",
        "1.0",
        &pom(
            "org.test",
            "bom-app",
            "1.0",
            import,
            &dependency("org.test", "alpha", "1.0"),
        ),
    )?;
    let resolver = resolver(repository.path(), None)?;
    let resolution = resolver
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.test", "bom-app", "1.0",
        )?))
        .await?;
    assert!(
        resolution
            .selected_dependencies()
            .iter()
            .any(|node| node.coordinate.to_string() == "org.test:common:2.0")
    );

    let project = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
      <modelVersion>4.0.0</modelVersion><groupId>org.local</groupId>
      <artifactId>profile-app</artifactId><version>1.0</version>
      <profiles><profile><id>with-alpha</id>
        <activation><property><name>enable.alpha</name><value>true</value></property></activation>
        <properties><alpha.version>1.0</alpha.version></properties>
        <dependencies><dependency><groupId>org.test</groupId><artifactId>alpha</artifactId>
          <version>${alpha.version}</version></dependency></dependencies>
      </profile></profiles>
    </project>"#;
    let mut request = ResolutionRequest::from_project_bytes(project.as_bytes());
    request.context = ResolutionContext::default().with_property("enable.alpha", "true");
    let resolution = resolver.resolve(&request).await?;
    assert!(
        resolution
            .selected_dependencies()
            .iter()
            .any(|node| node.coordinate.to_string() == "org.test:alpha:1.0")
    );

    let project = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
      <modelVersion>4.0.0</modelVersion><groupId>org.local</groupId>
      <artifactId>condition-app</artifactId><version>1.0</version>
      <profiles><profile><id>condition-profile</id>
        <activation><condition>${feature.mode} == enabled</condition></activation>
        <dependencies><dependency><groupId>org.test</groupId><artifactId>beta</artifactId>
          <version>1.0</version></dependency></dependencies>
      </profile></profiles>
    </project>"#;
    let mut request = ResolutionRequest::from_project_bytes(project.as_bytes());
    request.context = ResolutionContext::default().with_property("feature.mode", "enabled");
    let resolution = resolver.resolve(&request).await?;
    assert!(
        resolution
            .selected_dependencies()
            .iter()
            .any(|node| node.coordinate.to_string() == "org.test:beta:1.0")
    );
    Ok(())
}

#[tokio::test]
async fn resolves_timestamped_snapshot() -> Result<()> {
    let repository = TempDir::new()?;
    let directory = artifact_directory(repository.path(), "org.test", "snapshot", "1.0-SNAPSHOT");
    fs::create_dir_all(&directory)?;
    let timestamped = "1.0-20260725.120000-2";
    fs::write(
        directory.join("maven-metadata.xml"),
        format!(
            "<metadata><versioning><snapshot><timestamp>20260725.120000</timestamp>\
             <buildNumber>2</buildNumber></snapshot><snapshotVersions>\
             <snapshotVersion><extension>pom</extension><value>{timestamped}</value></snapshotVersion>\
             <snapshotVersion><extension>jar</extension><value>{timestamped}</value></snapshotVersion>\
             </snapshotVersions></versioning></metadata>"
        ),
    )?;
    fs::write(
        directory.join(format!("snapshot-{timestamped}.pom")),
        pom(
            "org.test",
            "snapshot",
            "1.0-SNAPSHOT",
            "",
            EMPTY_DEPENDENCIES,
        ),
    )?;
    let jar = b"snapshot".to_vec();
    let jar_path = directory.join(format!("snapshot-{timestamped}.jar"));
    fs::write(&jar_path, &jar)?;
    let checksum =
        Sha256::digest(&jar)
            .iter()
            .fold(String::with_capacity(64), |mut checksum, byte| {
                let _ = write!(checksum, "{byte:02x}");
                checksum
            });
    fs::write(jar_path.with_extension("jar.sha256"), checksum)?;

    let resolver = resolver(repository.path(), None)?;
    let destination = TempDir::new()?;
    let (resolution, report) = resolver
        .resolve_and_download(
            &ResolutionRequest::new(ArtifactCoordinate::new(
                "org.test",
                "snapshot",
                "1.0-SNAPSHOT",
            )?),
            &FileDestination::new(destination.path()),
        )
        .await?;
    assert_eq!(
        resolution
            .artifacts
            .first()
            .ok_or_else(|| {
                Error::InvalidConfiguration("snapshot artifact was not resolved".to_string())
            })?
            .file_version,
        timestamped
    );
    assert!(
        report
            .artifacts
            .first()
            .ok_or_else(|| {
                Error::InvalidConfiguration("snapshot artifact was not downloaded".to_string())
            })?
            .output
            .is_file()
    );
    Ok(())
}

#[tokio::test]
async fn aborts_file_transaction_on_checksum_mismatch() -> Result<()> {
    let repository = fixture_repository()?;
    let jar = artifact_directory(repository.path(), "org.test", "app", "1.0").join("app-1.0.jar");
    fs::write(jar.with_extension("jar.sha256"), "0000")?;
    let resolver = resolver(repository.path(), None)?;
    let destination = TempDir::new()?;
    let result = resolver
        .resolve_and_download(
            &ResolutionRequest::new(ArtifactCoordinate::new("org.test", "app", "1.0")?),
            &FileDestination::new(destination.path()),
        )
        .await;
    assert!(matches!(result, Err(Error::ChecksumMismatch { .. })));
    assert!(
        !destination
            .path()
            .join("org/test/app/1.0/app-1.0.jar")
            .exists()
    );
    let entries = fs::read_dir(destination.path().join("org/test/app/1.0"))?
        .collect::<std::io::Result<Vec<_>>>()?;
    assert!(
        entries
            .iter()
            .all(|entry| !entry.path().to_string_lossy().contains(".part-"))
    );
    Ok(())
}

#[tokio::test]
async fn enforces_missing_checksum_policies_and_supported_algorithms() -> Result<()> {
    let coordinate = ArtifactCoordinate::new("org.test", "verified", "1.0")?;
    for (extension, algorithm, checksum) in [
        (
            "sha512",
            "SHA-512",
            Sha512::digest(b"fixture:org.test:verified:1.0").to_vec(),
        ),
        (
            "sha1",
            "SHA-1",
            Sha1::digest(b"fixture:org.test:verified:1.0").to_vec(),
        ),
        (
            "md5",
            "MD5",
            Md5::digest(b"fixture:org.test:verified:1.0").to_vec(),
        ),
    ] {
        let repository = TempDir::new()?;
        publish(
            repository.path(),
            "org.test",
            "verified",
            "1.0",
            &pom("org.test", "verified", "1.0", "", EMPTY_DEPENDENCIES),
        )?;
        let jar = artifact_directory(repository.path(), "org.test", "verified", "1.0")
            .join("verified-1.0.jar");
        fs::remove_file(jar.with_extension("jar.sha256"))?;
        let checksum = checksum.iter().fold(String::new(), |mut output, byte| {
            let _ = write!(output, "{byte:02x}");
            output
        });
        fs::write(jar.with_extension(format!("jar.{extension}")), checksum)?;
        let resolver = resolver(repository.path(), None)?;
        let resolution = resolver
            .resolve(&ResolutionRequest::new(coordinate.clone()))
            .await?;
        let report = resolver.download(&resolution, &MemoryDestination).await?;
        assert_eq!(
            report
                .artifacts
                .first()
                .and_then(|artifact| artifact.checksum.as_ref())
                .map(|checksum| checksum.algorithm.as_str()),
            Some(algorithm)
        );
    }

    let repository = TempDir::new()?;
    publish(
        repository.path(),
        "org.test",
        "verified",
        "1.0",
        &pom("org.test", "verified", "1.0", "", EMPTY_DEPENDENCIES),
    )?;
    let jar = artifact_directory(repository.path(), "org.test", "verified", "1.0")
        .join("verified-1.0.jar");
    fs::remove_file(jar.with_extension("jar.sha256"))?;
    let url = Url::from_directory_path(repository.path())
        .map_err(|()| Error::InvalidConfiguration("invalid repository path".to_string()))?;

    let mut strict_repository = RemoteRepository::new("strict", url.to_string())?;
    strict_repository.releases.checksum = ChecksumPolicy::Fail;
    let strict = Resolver::builder()
        .repositories(vec![strict_repository])
        .build()?;
    let resolution = strict
        .resolve(&ResolutionRequest::new(coordinate.clone()))
        .await?;
    assert!(matches!(
        strict.download(&resolution, &MemoryDestination).await,
        Err(Error::MissingChecksum(_))
    ));

    let mut warning_repository = RemoteRepository::new("warning", url.to_string())?;
    warning_repository.releases.checksum = ChecksumPolicy::Warn;
    let warning = Resolver::builder()
        .repositories(vec![warning_repository])
        .build()?;
    let resolution = warning
        .resolve(&ResolutionRequest::new(coordinate.clone()))
        .await?;
    let report = warning.download(&resolution, &MemoryDestination).await?;
    assert_eq!(report.diagnostics.len(), 1);

    let mut ignored_repository = RemoteRepository::new("ignored", url.to_string())?;
    ignored_repository.releases.checksum = ChecksumPolicy::Ignore;
    fs::write(&jar, b"changed without a checksum")?;
    let ignored = Resolver::builder()
        .repositories(vec![ignored_repository])
        .build()?;
    let resolution = ignored.resolve(&ResolutionRequest::new(coordinate)).await?;
    let report = ignored.download(&resolution, &MemoryDestination).await?;
    let downloaded = report
        .artifacts
        .first()
        .ok_or_else(|| Error::InvalidConfiguration("artifact was not downloaded".to_string()))?;
    assert!(downloaded.checksum.is_none());
    assert_eq!(downloaded.output, b"changed without a checksum");
    Ok(())
}

#[tokio::test]
async fn resolves_relative_parent_for_local_project() -> Result<()> {
    let repository = fixture_repository()?;
    let project = TempDir::new()?;
    let parent = format!(
        r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
          <modelVersion>4.0.0</modelVersion><groupId>org.local</groupId>
          <artifactId>parent</artifactId><version>1.0</version><packaging>pom</packaging>
          <dependencyManagement><dependencies>{}</dependencies></dependencyManagement>
        </project>"#,
        dependency("org.test", "alpha", "1.0")
    );
    fs::write(project.path().join("pom.xml"), parent)?;
    let child_directory = project.path().join("child");
    fs::create_dir_all(&child_directory)?;
    fs::write(
        child_directory.join("pom.xml"),
        r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
          <modelVersion>4.0.0</modelVersion>
          <parent><groupId>org.local</groupId><artifactId>parent</artifactId>
            <version>1.0</version><relativePath>../pom.xml</relativePath></parent>
          <artifactId>child</artifactId>
          <dependencies><dependency><groupId>org.test</groupId>
            <artifactId>alpha</artifactId></dependency></dependencies>
        </project>"#,
    )?;
    let resolver = resolver(repository.path(), None)?;
    let resolution = resolver
        .resolve(&ResolutionRequest::from_project_file(
            child_directory.join("pom.xml"),
        ))
        .await?;
    let root = resolution
        .roots
        .first()
        .copied()
        .ok_or_else(|| Error::InvalidConfiguration("root node id is missing".to_string()))?;
    assert_eq!(
        resolution
            .nodes
            .get(root.0)
            .ok_or_else(|| Error::InvalidConfiguration("root node is missing".to_string()))?
            .coordinate
            .to_string(),
        "org.local:child:1.0"
    );
    assert!(
        resolution
            .selected_dependencies()
            .iter()
            .any(|node| node.coordinate.to_string() == "org.test:alpha:1.0")
    );
    Ok(())
}

#[tokio::test]
async fn ignores_mismatched_relative_parent_and_resolves_declared_parent() -> Result<()> {
    let repository = fixture_repository()?;
    let remote_management = format!(
        "<packaging>pom</packaging><dependencyManagement><dependencies>{}</dependencies></dependencyManagement>",
        dependency("org.test", "alpha", "1.0")
    );
    publish(
        repository.path(),
        "org.expected",
        "parent",
        "1.0",
        &pom(
            "org.expected",
            "parent",
            "1.0",
            &remote_management,
            EMPTY_DEPENDENCIES,
        ),
    )?;

    let project = TempDir::new()?;
    let mismatched_management = format!(
        "<packaging>pom</packaging><dependencyManagement><dependencies>{}</dependencies></dependencyManagement>",
        dependency("org.test", "alpha", "9.0")
    );
    fs::write(
        project.path().join("pom.xml"),
        pom(
            "org.wrong",
            "parent",
            "1.0",
            &mismatched_management,
            EMPTY_DEPENDENCIES,
        ),
    )?;
    let child_directory = project.path().join("child");
    fs::create_dir_all(&child_directory)?;
    fs::write(
        child_directory.join("pom.xml"),
        r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
          <modelVersion>4.0.0</modelVersion>
          <parent><groupId>org.expected</groupId><artifactId>parent</artifactId>
            <version>1.0</version></parent>
          <artifactId>child</artifactId>
          <dependencies><dependency><groupId>org.test</groupId>
            <artifactId>alpha</artifactId></dependency></dependencies>
        </project>"#,
    )?;

    let resolution = resolver(repository.path(), None)?
        .resolve(&ResolutionRequest::from_project_file(
            child_directory.join("pom.xml"),
        ))
        .await?;
    assert!(
        resolution
            .selected_dependencies()
            .iter()
            .any(|node| node.coordinate.to_string() == "org.test:alpha:1.0")
    );
    Ok(())
}

#[tokio::test]
async fn resolves_parent_ranges_from_a_repository_declared_by_the_child() -> Result<()> {
    let primary = fixture_repository()?;
    let parent_repository = TempDir::new()?;
    let parent_management = format!(
        "<packaging>pom</packaging><dependencyManagement><dependencies>{}</dependencies></dependencyManagement>",
        dependency("org.test", "common", "2.0")
    );
    publish(
        parent_repository.path(),
        "org.parent",
        "platform-parent",
        "1.0",
        &pom(
            "org.parent",
            "platform-parent",
            "1.0",
            &parent_management,
            EMPTY_DEPENDENCIES,
        ),
    )?;
    publish_metadata(
        parent_repository.path(),
        "org.parent",
        "platform-parent",
        &["1.0"],
    )?;
    let parent_url = Url::from_directory_path(parent_repository.path())
        .map_err(|()| Error::InvalidConfiguration("invalid parent repository path".to_string()))?;
    let project = format!(
        r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
          <modelVersion>4.0.0</modelVersion>
          <parent><groupId>org.parent</groupId><artifactId>platform-parent</artifactId>
            <version>[1,2)</version><relativePath/></parent>
          <artifactId>child</artifactId>
          <repositories><repository><id>parent-repository</id><url>{parent_url}</url>
          </repository></repositories>
          <dependencies><dependency><groupId>org.test</groupId>
            <artifactId>common</artifactId></dependency></dependencies>
        </project>"#
    );

    let resolution = resolver(primary.path(), None)?
        .resolve(&ResolutionRequest::from_project_bytes(project))
        .await?;
    let root = resolution
        .roots
        .first()
        .ok_or_else(|| Error::InvalidConfiguration("resolution has no root".to_string()))?;
    assert_eq!(
        resolution
            .nodes
            .get(root.0)
            .map(|node| node.coordinate.to_string())
            .as_deref(),
        Some("org.parent:child:1.0")
    );
    assert!(
        resolution
            .selected_dependencies()
            .iter()
            .any(|node| node.coordinate.to_string() == "org.test:common:2.0")
    );
    assert!(
        resolution
            .repositories
            .iter()
            .any(|repository| repository.id == "parent-repository")
    );
    Ok(())
}

#[tokio::test]
async fn rejects_a_non_pom_parent_model() -> Result<()> {
    let repository = fixture_repository()?;
    publish(
        repository.path(),
        "org.parent",
        "invalid-parent",
        "1.0",
        &pom(
            "org.parent",
            "invalid-parent",
            "1.0",
            "",
            EMPTY_DEPENDENCIES,
        ),
    )?;
    let project = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
      <modelVersion>4.0.0</modelVersion>
      <parent><groupId>org.parent</groupId><artifactId>invalid-parent</artifactId>
        <version>1.0</version><relativePath/></parent>
      <artifactId>child</artifactId>
    </project>"#;

    let result = resolver(repository.path(), None)?
        .resolve(&ResolutionRequest::from_project_bytes(project))
        .await;
    assert!(matches!(result, Err(Error::Model { .. })));
    Ok(())
}

#[tokio::test]
async fn resolves_imported_bom_from_importing_model_repository() -> Result<()> {
    let primary = fixture_repository()?;
    let bom_repository = TempDir::new()?;
    let bom_management = format!(
        "<packaging>pom</packaging><dependencyManagement><dependencies>{}</dependencies></dependencyManagement>",
        dependency("org.test", "common", "2.0")
    );
    publish(
        bom_repository.path(),
        "org.platform",
        "bom",
        "1.0",
        &pom(
            "org.platform",
            "bom",
            "1.0",
            &bom_management,
            EMPTY_DEPENDENCIES,
        ),
    )?;
    let bom_url = Url::from_directory_path(bom_repository.path())
        .map_err(|()| Error::InvalidConfiguration("invalid BOM repository path".to_string()))?;
    let import = format!(
        "<repositories><repository><id>bom-repository</id><url>{bom_url}</url>\
         <releases><updatePolicy>always</updatePolicy></releases></repository></repositories>\
         <dependencyManagement><dependencies><dependency><groupId>org.platform</groupId>\
         <artifactId>bom</artifactId><version>1.0</version><type>pom</type>\
         <scope>import</scope></dependency></dependencies></dependencyManagement>"
    );
    publish(
        primary.path(),
        "org.test",
        "bom-repository-app",
        "1.0",
        &pom(
            "org.test",
            "bom-repository-app",
            "1.0",
            &import,
            &dependency("org.test", "common", ""),
        )
        .replace("<version></version>", ""),
    )?;

    let resolution = resolver(primary.path(), None)?
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.test",
            "bom-repository-app",
            "1.0",
        )?))
        .await?;
    assert!(
        resolution
            .selected_dependencies()
            .iter()
            .any(|node| node.coordinate.to_string() == "org.test:common:2.0")
    );
    Ok(())
}

#[tokio::test]
async fn current_model_bom_management_overrides_inherited_management() -> Result<()> {
    let repository = fixture_repository()?;
    let parent_management = format!(
        "<packaging>pom</packaging><dependencyManagement><dependencies>{}</dependencies></dependencyManagement>",
        dependency("org.test", "common", "1.0")
    );
    publish(
        repository.path(),
        "org.platform",
        "parent",
        "1.0",
        &pom(
            "org.platform",
            "parent",
            "1.0",
            &parent_management,
            EMPTY_DEPENDENCIES,
        ),
    )?;
    let bom_management = format!(
        "<packaging>pom</packaging><dependencyManagement><dependencies>{}</dependencies></dependencyManagement>",
        dependency("org.test", "common", "2.0")
    );
    publish(
        repository.path(),
        "org.platform",
        "bom",
        "1.0",
        &pom(
            "org.platform",
            "bom",
            "1.0",
            &bom_management,
            EMPTY_DEPENDENCIES,
        ),
    )?;
    let project = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
      <modelVersion>4.0.0</modelVersion>
      <parent><groupId>org.platform</groupId><artifactId>parent</artifactId>
        <version>1.0</version><relativePath/></parent>
      <artifactId>child</artifactId>
      <dependencyManagement><dependencies><dependency>
        <groupId>org.platform</groupId><artifactId>bom</artifactId><version>1.0</version>
        <type>pom</type><scope>import</scope>
      </dependency></dependencies></dependencyManagement>
      <dependencies><dependency><groupId>org.test</groupId><artifactId>common</artifactId>
      </dependency></dependencies>
    </project>"#;

    let resolution = resolver(repository.path(), None)?
        .resolve(&ResolutionRequest::from_project_bytes(project))
        .await?;
    assert!(
        resolution
            .selected_dependencies()
            .iter()
            .any(|node| node.coordinate.to_string() == "org.test:common:2.0")
    );
    Ok(())
}

#[tokio::test]
async fn retains_transitive_repositories_for_download() -> Result<()> {
    let primary = fixture_repository()?;
    let secondary = TempDir::new()?;
    publish(
        secondary.path(),
        "org.secondary",
        "child",
        "1.0",
        &pom("org.secondary", "child", "1.0", "", EMPTY_DEPENDENCIES),
    )?;
    let secondary_url = Url::from_directory_path(secondary.path()).map_err(|()| {
        Error::InvalidConfiguration("invalid secondary repository path".to_string())
    })?;
    let declaring_pom = format!(
        r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
          <modelVersion>4.0.0</modelVersion><groupId>org.test</groupId>
          <artifactId>repository-declarer</artifactId><version>1.0</version>
          <repositories><repository><id>fixture</id><url>{secondary_url}</url></repository></repositories>
          <dependencies>{}</dependencies>
        </project>"#,
        dependency("org.secondary", "child", "1.0")
    );
    publish(
        primary.path(),
        "org.test",
        "repository-declarer",
        "1.0",
        &declaring_pom,
    )?;
    publish(
        primary.path(),
        "org.test",
        "repository-app",
        "1.0",
        &pom(
            "org.test",
            "repository-app",
            "1.0",
            "",
            &dependency("org.test", "repository-declarer", "1.0"),
        ),
    )?;

    let resolver = resolver(primary.path(), None)?;
    let resolution = resolver
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.test",
            "repository-app",
            "1.0",
        )?))
        .await?;
    assert!(
        resolution
            .repositories
            .iter()
            .any(|repository| repository.id.starts_with("fixture~declared-"))
    );
    let report = resolver.download(&resolution, &MemoryDestination).await?;
    assert_eq!(report.artifacts.len(), 3);

    let disabled = resolver
        .resolve(
            &ResolutionRequest::new(ArtifactCoordinate::new(
                "org.test",
                "repository-app",
                "1.0",
            )?)
            .with_transitive_repositories(false),
        )
        .await;
    assert!(matches!(disabled, Err(Error::ArtifactNotFound(_))));

    let local = resolver
        .resolve(
            &ResolutionRequest::from_project_bytes(declaring_pom)
                .with_transitive_repositories(false),
        )
        .await?;
    assert!(
        local
            .selected_dependencies()
            .iter()
            .any(|node| node.coordinate.to_string() == "org.secondary:child:1.0")
    );
    Ok(())
}

#[tokio::test]
async fn mediates_all_hard_version_ranges() -> Result<()> {
    let repository = fixture_repository()?;
    for (artifact, range) in [
        ("wide-range", "[1.0,4.0)"),
        ("narrow-range", "[1.0,2.0]"),
        ("low-range", "[1.0,2.0)"),
        ("high-range", "[2.0,3.0)"),
    ] {
        publish(
            repository.path(),
            "org.test",
            artifact,
            "1.0",
            &pom(
                "org.test",
                artifact,
                "1.0",
                "",
                &dependency("org.test", "common", range),
            ),
        )?;
    }
    publish(
        repository.path(),
        "org.test",
        "compatible-ranges",
        "1.0",
        &pom(
            "org.test",
            "compatible-ranges",
            "1.0",
            "",
            &format!(
                "{}{}",
                dependency("org.test", "wide-range", "1.0"),
                dependency("org.test", "narrow-range", "1.0")
            ),
        ),
    )?;
    let resolver = resolver(repository.path(), None)?;
    let resolution = resolver
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.test",
            "compatible-ranges",
            "1.0",
        )?))
        .await?;
    let common = resolution
        .selected_dependencies()
        .into_iter()
        .find(|node| node.coordinate.key.artifact_id == "common")
        .ok_or_else(|| Error::InvalidConfiguration("common was not selected".to_string()))?;
    assert_eq!(common.coordinate.version, "2.0");

    publish(
        repository.path(),
        "org.test",
        "incompatible-ranges",
        "1.0",
        &pom(
            "org.test",
            "incompatible-ranges",
            "1.0",
            "",
            &format!(
                "{}{}",
                dependency("org.test", "low-range", "1.0"),
                dependency("org.test", "high-range", "1.0")
            ),
        ),
    )?;
    let error = resolver
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.test",
            "incompatible-ranges",
            "1.0",
        )?))
        .await
        .expect_err("incompatible hard ranges must fail");
    assert!(matches!(error, Error::VersionConflict { .. }));
    Ok(())
}

#[tokio::test]
async fn mediates_ranges_declared_on_roots_with_transitive_ranges() -> Result<()> {
    let repository = fixture_repository()?;
    let ranged_dependency = "<dependency><groupId>org.test</groupId>\
        <artifactId>common</artifactId><version>[1,2]</version></dependency>";
    publish(
        repository.path(),
        "org.test",
        "range-consumer",
        "1.0",
        &pom("org.test", "range-consumer", "1.0", "", ranged_dependency),
    )?;
    let request = ResolutionRequest::new(ArtifactCoordinate::new("org.test", "common", "[1,4]")?)
        .with_root(ristretto_resolver::ResolutionRoot::Artifact(
            ArtifactCoordinate::new("org.test", "range-consumer", "1.0")?,
        ));

    let resolution = resolver(repository.path(), None)?.resolve(&request).await?;
    assert!(resolution.selected_dependencies().iter().any(|node| {
        node.coordinate.key.artifact_id == "common" && node.coordinate.version == "2.0"
    }));
    Ok(())
}

#[tokio::test]
async fn dependency_management_does_not_default_transitive_scope_or_optional() -> Result<()> {
    let repository = fixture_repository()?;
    let child_dependencies = "<dependency><groupId>org.test</groupId>\
        <artifactId>common</artifactId><version>1.0</version><scope>runtime</scope>\
        <optional>true</optional></dependency>";
    publish(
        repository.path(),
        "org.test",
        "managed-child",
        "1.0",
        &pom("org.test", "managed-child", "1.0", "", child_dependencies),
    )?;
    let management = "<dependencyManagement><dependencies><dependency>\
        <groupId>org.test</groupId><artifactId>common</artifactId><version>1.0</version>\
        </dependency></dependencies></dependencyManagement>";
    publish(
        repository.path(),
        "org.test",
        "management-app",
        "1.0",
        &pom(
            "org.test",
            "management-app",
            "1.0",
            management,
            &dependency("org.test", "managed-child", "1.0"),
        ),
    )?;

    let resolution = resolver(repository.path(), None)?
        .resolve(
            &ResolutionRequest::new(ArtifactCoordinate::new(
                "org.test",
                "management-app",
                "1.0",
            )?)
            .with_classpath(Classpath::Test),
        )
        .await?;
    assert!(resolution.nodes.iter().any(|node| {
        node.coordinate.key.artifact_id == "common"
            && node.scope == ristretto_pom::DependencyScope::Runtime
            && node.status == NodeStatus::Optional
    }));
    Ok(())
}

#[tokio::test]
async fn dependency_management_can_explicitly_clear_transitive_optional() -> Result<()> {
    let repository = fixture_repository()?;
    let child_dependencies = "<dependency><groupId>org.test</groupId>\
        <artifactId>common</artifactId><version>1.0</version>\
        <optional>true</optional></dependency>";
    publish(
        repository.path(),
        "org.test",
        "managed-child",
        "1.0",
        &pom("org.test", "managed-child", "1.0", "", child_dependencies),
    )?;
    let management = "<dependencyManagement><dependencies><dependency>\
        <groupId>org.test</groupId><artifactId>common</artifactId><version>1.0</version>\
        <optional>false</optional></dependency></dependencies></dependencyManagement>";
    publish(
        repository.path(),
        "org.test",
        "management-app",
        "1.0",
        &pom(
            "org.test",
            "management-app",
            "1.0",
            management,
            &dependency("org.test", "managed-child", "1.0"),
        ),
    )?;

    let resolution = resolver(repository.path(), None)?
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.test",
            "management-app",
            "1.0",
        )?))
        .await?;
    assert!(resolution.nodes.iter().any(|node| {
        node.coordinate.key.artifact_id == "common" && node.status == NodeStatus::Selected
    }));
    Ok(())
}

#[tokio::test]
async fn explicit_dependency_optional_false_overrides_management() -> Result<()> {
    let repository = fixture_repository()?;
    let management = "<dependencyManagement><dependencies><dependency>\
        <groupId>org.test</groupId><artifactId>common</artifactId><version>1.0</version>\
        <optional>true</optional></dependency></dependencies></dependencyManagement>";
    let dependencies = "<dependency><groupId>org.test</groupId><artifactId>common</artifactId>\
        <optional>false</optional></dependency>";
    publish(
        repository.path(),
        "org.test",
        "optional-app",
        "1.0",
        &pom("org.test", "optional-app", "1.0", management, dependencies),
    )?;

    let resolution = resolver(repository.path(), None)?
        .resolve(&ResolutionRequest::new(ArtifactCoordinate::new(
            "org.test",
            "optional-app",
            "1.0",
        )?))
        .await?;
    assert!(resolution.nodes.iter().any(|node| {
        node.coordinate.key.artifact_id == "common"
            && !node.optional
            && node.status == NodeStatus::Selected
    }));
    Ok(())
}

#[tokio::test]
async fn evaluates_maven4_condition_profiles() -> Result<()> {
    let repository = fixture_repository()?;
    let base = TempDir::new()?;
    let project = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
      <modelVersion>4.0.0</modelVersion><groupId>org.local</groupId>
      <artifactId>condition-functions</artifactId><version>1.0</version>
      <profiles>
        <profile><id>true-functions</id>
          <activation><condition>contains(${feature.mode}, 'enabled') &amp;&amp; length(${user.name}) &gt; 5</condition></activation>
          <dependencies><dependency><groupId>org.test</groupId><artifactId>alpha</artifactId>
            <version>1.0</version></dependency></dependencies>
        </profile>
        <profile><id>false-match</id>
          <activation><condition>matches(${feature.mode}, 'disabled-.*')</condition></activation>
          <dependencies><dependency><groupId>org.test</groupId><artifactId>beta</artifactId>
            <version>1.0</version></dependency></dependencies>
        </profile>
        <profile><id>missing-file</id>
          <activation><condition>missing('not-created.txt')</condition></activation>
          <dependencies><dependency><groupId>org.test</groupId><artifactId>common</artifactId>
            <version>3.0</version></dependency></dependencies>
        </profile>
      </profiles>
    </project>"#;
    let mut request = ResolutionRequest::from_project_bytes(project.as_bytes());
    request.context = ResolutionContext::default()
        .with_property("feature.mode", "resolver-enabled")
        .with_property("user.name", "Ristretto");
    request.context.base_directory = Some(base.path().to_path_buf());
    let resolution = resolver(repository.path(), None)?.resolve(&request).await?;
    let selected = resolution
        .selected_dependencies()
        .into_iter()
        .map(|node| node.coordinate.to_string())
        .collect::<BTreeSet<_>>();
    assert!(selected.contains("org.test:alpha:1.0"));
    assert!(selected.contains("org.test:common:3.0"));
    assert!(!selected.contains("org.test:beta:1.0"));
    Ok(())
}

#[tokio::test]
async fn interpolates_dependency_and_repository_policy_booleans() -> Result<()> {
    let artifact_repository = fixture_repository()?;
    let initial_repository = TempDir::new()?;
    let artifact_url = Url::from_directory_path(artifact_repository.path()).map_err(|()| {
        Error::InvalidConfiguration("fixture repository path is not absolute".to_string())
    })?;
    let project = format!(
        r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
          <modelVersion>4.0.0</modelVersion><groupId>org.test</groupId>
          <artifactId>expressions</artifactId><version>1.0</version>
          <properties><repository.enabled>true</repository.enabled>
          <repository.update>always</repository.update>
          <repository.checksum>ignore</repository.checksum>
          <repository.layout>default</repository.layout>
          <dependency.optional>false</dependency.optional>
          <dependency.scope>runtime</dependency.scope></properties>
          <repositories><repository><id>expressions</id><url>{artifact_url}</url>
          <layout>${{repository.layout}}</layout>
          <releases><enabled>${{repository.enabled}}</enabled>
          <updatePolicy>${{repository.update}}</updatePolicy>
          <checksumPolicy>${{repository.checksum}}</checksumPolicy></releases>
          <snapshots><enabled>false</enabled></snapshots></repository></repositories>
          <dependencies><dependency><groupId>org.test</groupId><artifactId>common</artifactId>
          <version>1.0</version><optional>${{dependency.optional}}</optional>
          <scope>${{dependency.scope}}</scope>
          </dependency></dependencies>
        </project>"#
    );
    let resolution = resolver(initial_repository.path(), None)?
        .resolve(&ResolutionRequest::from_project_bytes(project))
        .await?;
    let common = resolution
        .nodes
        .iter()
        .find(|node| node.coordinate.key.artifact_id == "common")
        .ok_or_else(|| Error::InvalidConfiguration("common was not resolved".to_string()))?;
    assert!(!common.optional);
    assert_eq!(common.scope, ristretto_pom::DependencyScope::Runtime);
    assert_eq!(common.repository_id.as_deref(), Some("expressions"));
    Ok(())
}

#[tokio::test]
async fn activates_profiles_by_project_packaging() -> Result<()> {
    let repository = fixture_repository()?;
    let project = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
      <modelVersion>4.0.0</modelVersion><groupId>org.local</groupId>
      <artifactId>packaging-activation</artifactId><version>1.0</version>
      <profiles>
        <profile><id>jar-profile</id><activation><packaging>jar</packaging></activation>
          <dependencies><dependency><groupId>org.test</groupId><artifactId>alpha</artifactId>
            <version>1.0</version></dependency></dependencies>
        </profile>
        <profile><id>war-profile</id><activation><packaging>war</packaging></activation>
          <dependencies><dependency><groupId>missing</groupId><artifactId>must-not-resolve</artifactId>
            <version>1.0</version></dependency></dependencies>
        </profile>
        <profile><id>packaging-property</id><activation><property>
          <name>packaging</name><value>jar</value>
        </property></activation><dependencies><dependency>
          <groupId>org.test</groupId><artifactId>beta</artifactId><version>1.0</version>
        </dependency></dependencies></profile>
      </profiles>
    </project>"#;
    let resolution = resolver(repository.path(), None)?
        .resolve(&ResolutionRequest::from_project_bytes(project.as_bytes()))
        .await?;
    assert!(resolution.nodes.iter().any(|node| {
        node.coordinate.key.artifact_id == "alpha" && node.status == NodeStatus::Selected
    }));
    assert!(
        !resolution
            .nodes
            .iter()
            .any(|node| node.coordinate.key.artifact_id == "must-not-resolve")
    );
    assert!(resolution.nodes.iter().any(|node| {
        node.coordinate.key.artifact_id == "beta" && node.status == NodeStatus::Selected
    }));
    Ok(())
}

#[tokio::test]
async fn activates_profiles_with_negated_os_family_and_version_regex() -> Result<()> {
    let repository = fixture_repository()?;
    let project = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
      <modelVersion>4.0.0</modelVersion><groupId>org.local</groupId>
      <artifactId>os-activation</artifactId><version>1.0</version>
      <profiles><profile><id>matching-os</id><activation><os>
        <family>!windows</family><version>regex:6\..*</version>
      </os></activation><dependencies><dependency>
        <groupId>org.test</groupId><artifactId>alpha</artifactId><version>1.0</version>
      </dependency></dependencies></profile></profiles>
    </project>"#;
    let mut request = ResolutionRequest::from_project_bytes(project.as_bytes());
    request.context.os_name = Some("Linux".to_string());
    request.context.os_arch = Some("x86_64".to_string());
    request.context.os_version = Some("6.8.12".to_string());
    let resolution = resolver(repository.path(), None)?.resolve(&request).await?;
    assert!(resolution.nodes.iter().any(|node| {
        node.coordinate.key.artifact_id == "alpha" && node.status == NodeStatus::Selected
    }));
    Ok(())
}

#[tokio::test]
async fn empty_profile_activation_is_not_implicitly_active() -> Result<()> {
    let repository = fixture_repository()?;
    let project = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
      <modelVersion>4.0.0</modelVersion><groupId>org.local</groupId>
      <artifactId>empty-activation</artifactId><version>1.0</version>
      <profiles><profile><id>inactive</id><activation/>
        <dependencies><dependency><groupId>missing</groupId>
          <artifactId>must-not-resolve</artifactId><version>1.0</version>
        </dependency></dependencies>
      </profile></profiles>
    </project>"#;
    let resolution = resolver(repository.path(), None)?
        .resolve(&ResolutionRequest::from_project_bytes(project.as_bytes()))
        .await?;
    assert_eq!(resolution.nodes.len(), 1);
    Ok(())
}

#[tokio::test]
async fn pom_properties_do_not_implicitly_activate_profiles() -> Result<()> {
    let repository = fixture_repository()?;
    let project = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
      <modelVersion>4.0.0</modelVersion><groupId>org.local</groupId>
      <artifactId>pom-property-activation</artifactId><version>1.0</version>
      <properties><activate.profile>true</activate.profile></properties>
      <profiles><profile><id>inactive</id>
        <activation><property><name>activate.profile</name><value>true</value></property></activation>
        <dependencies><dependency><groupId>missing</groupId>
          <artifactId>must-not-resolve</artifactId><version>1.0</version>
        </dependency></dependencies>
      </profile></profiles>
    </project>"#;
    let resolution = resolver(repository.path(), None)?
        .resolve(&ResolutionRequest::from_project_bytes(project.as_bytes()))
        .await?;
    assert_eq!(resolution.nodes.len(), 1);
    Ok(())
}

#[tokio::test]
async fn resolves_local_system_dependency() -> Result<()> {
    let repository = fixture_repository()?;
    let project = TempDir::new()?;
    let library_directory = project.path().join("lib files");
    fs::create_dir_all(&library_directory)?;
    fs::write(
        library_directory.join("system.jar"),
        b"local-system-artifact",
    )?;
    let project_xml = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
      <modelVersion>4.0.0</modelVersion><groupId>org.local</groupId>
      <artifactId>system-app</artifactId><version>1.0</version>
      <properties><system.file>lib files/system.jar</system.file></properties>
      <dependencies><dependency><groupId>org.local</groupId><artifactId>system-lib</artifactId>
        <version>1.0</version><scope>system</scope>
        <systemPath>${system.file}</systemPath></dependency></dependencies>
    </project>"#;
    let mut request = ResolutionRequest::from_project_bytes(project_xml.as_bytes())
        .with_classpath(Classpath::Compile);
    request.context.base_directory = Some(project.path().to_path_buf());
    let resolver = resolver(repository.path(), None)?;
    let destination = TempDir::new()?;

    let (resolution, report) = resolver
        .resolve_and_download(&request, &FileDestination::new(destination.path()))
        .await?;

    assert!(
        resolution
            .transitive_dependencies()
            .iter()
            .any(|node| node.coordinate.to_string() == "org.local:system-lib:1.0")
    );
    assert_eq!(report.artifacts.len(), 1);
    let artifact = report
        .artifacts
        .first()
        .ok_or_else(|| Error::InvalidConfiguration("system artifact is missing".to_string()))?;
    assert_eq!(fs::read(&artifact.output)?, b"local-system-artifact");
    assert!(resolution.to_lock().is_err());
    Ok(())
}

#[tokio::test]
async fn reuses_file_cache_in_offline_mode() -> Result<()> {
    let repository = fixture_repository()?;
    let cache = TempDir::new()?;
    let first_destination = TempDir::new()?;
    let request = ResolutionRequest::new(ArtifactCoordinate::new("org.test", "app", "1.0")?);
    let online = resolver(repository.path(), Some(cache.path()))?;
    drop(
        online
            .resolve_and_download(&request, &FileDestination::new(first_destination.path()))
            .await?,
    );

    let offline_repository =
        RemoteRepository::new("fixture", "https://offline.invalid/repository/")?;
    let offline = Resolver::builder()
        .repositories(vec![offline_repository])
        .file_cache(FileCache::new(cache.path()))
        .offline(true)
        .build()?;
    let second_destination = TempDir::new()?;
    let (_, report) = offline
        .resolve_and_download(&request, &FileDestination::new(second_destination.path()))
        .await?;
    assert_eq!(report.artifacts.len(), 4);
    Ok(())
}

#[tokio::test]
async fn file_cache_does_not_serve_another_repositorys_artifact() -> Result<()> {
    let repository_a = TempDir::new()?;
    let repository_b = TempDir::new()?;
    let model = pom("org.test", "cache-origin", "1.0", "", EMPTY_DEPENDENCIES);
    publish(
        repository_a.path(),
        "org.test",
        "cache-origin",
        "1.0",
        &model,
    )?;
    publish(
        repository_b.path(),
        "org.test",
        "cache-origin",
        "1.0",
        &model,
    )?;
    let repository_b_jar =
        artifact_directory(repository_b.path(), "org.test", "cache-origin", "1.0")
            .join("cache-origin-1.0.jar");
    fs::write(&repository_b_jar, b"repository-b")?;
    let repository_b_checksum = Sha256::digest(b"repository-b").iter().fold(
        String::with_capacity(64),
        |mut checksum, byte| {
            let _ = write!(checksum, "{byte:02x}");
            checksum
        },
    );
    fs::write(
        repository_b_jar.with_extension("jar.sha256"),
        repository_b_checksum,
    )?;

    let first_repository_url = Url::from_directory_path(repository_a.path())
        .map_err(|()| Error::InvalidConfiguration("invalid repository A path".to_string()))?
        .to_string();
    let second_repository_url = Url::from_directory_path(repository_b.path())
        .map_err(|()| Error::InvalidConfiguration("invalid repository B path".to_string()))?
        .to_string();
    let cache = TempDir::new()?;
    let request =
        ResolutionRequest::new(ArtifactCoordinate::new("org.test", "cache-origin", "1.0")?);

    for (id, url) in [
        ("repository-a", first_repository_url.clone()),
        ("repository-b", second_repository_url),
    ] {
        let resolver = Resolver::builder()
            .repositories(vec![RemoteRepository::new(id, url)?])
            .file_cache(FileCache::new(cache.path()))
            .build()?;
        let destination = TempDir::new()?;
        drop(
            resolver
                .resolve_and_download(&request, &FileDestination::new(destination.path()))
                .await?,
        );
    }

    drop(repository_a);
    let offline_a = Resolver::builder()
        .repositories(vec![RemoteRepository::new(
            "repository-a",
            first_repository_url,
        )?])
        .file_cache(FileCache::new(cache.path()))
        .offline(true)
        .build()?;
    let error = offline_a
        .resolve(&request)
        .await
        .expect_err("repository B cache content must not be attributed to repository A");
    assert!(matches!(error, Error::ArtifactNotFound(_)));
    Ok(())
}

#[tokio::test]
#[ignore = "requires Apache Maven 3.9.16 and network access for the dependency-tree plugin"]
async fn matches_maven_3_9_16_dependency_tree() -> Result<()> {
    let version_output = Command::new("mvn").arg("--version").output()?;
    let version = String::from_utf8_lossy(&version_output.stdout);
    if !version_output.status.success() || !version.contains("Apache Maven 3.9.16") {
        return Err(Error::InvalidConfiguration(
            "compatibility test requires Apache Maven 3.9.16".to_string(),
        ));
    }

    let repository = fixture_repository()?;
    let project_directory = TempDir::new()?;
    let repository_url = Url::from_directory_path(repository.path())
        .map_err(|()| {
            Error::InvalidConfiguration("fixture repository path is not absolute".to_string())
        })?
        .to_string();
    let management = format!(
        "<dependencyManagement><dependencies>{}</dependencies></dependencyManagement>",
        dependency("org.test", "common", "1.5")
    );
    let dependencies = format!(
        "{}{}",
        dependency("org.test", "alpha", "1.0"),
        dependency("org.test", "beta", "1.0")
    );
    let project = format!(
        r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
          <modelVersion>4.0.0</modelVersion><groupId>org.compat</groupId>
          <artifactId>app</artifactId><version>1.0</version>
          <repositories><repository><id>fixture</id><url>{repository_url}</url>
          </repository></repositories>{management}
          <dependencies>{dependencies}</dependencies>
        </project>"#
    );
    let project_path = project_directory.path().join("pom.xml");
    fs::write(&project_path, &project)?;
    let local_repository = project_directory.path().join("local-repository");
    let tree_path = project_directory.path().join("tree.txt");
    let output = Command::new("mvn")
        .arg("--batch-mode")
        .arg("--no-transfer-progress")
        .arg("--file")
        .arg(&project_path)
        .arg(format!("-Dmaven.repo.local={}", local_repository.display()))
        .arg("org.apache.maven.plugins:maven-dependency-plugin:3.8.1:tree")
        .arg("-Dscope=runtime")
        .arg(format!("-DoutputFile={}", tree_path.display()))
        .output()?;
    if !output.status.success() {
        return Err(Error::InvalidConfiguration(format!(
            "Maven compatibility command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let resolver = resolver(repository.path(), None)?;
    let mut request = ResolutionRequest::from_project_bytes(project.as_bytes());
    request.context = ResolutionContext::default();
    let actual = resolver
        .resolve(&request)
        .await?
        .transitive_dependencies()
        .iter()
        .map(|node| node.coordinate.to_string())
        .collect::<BTreeSet<_>>();
    let expected = parse_maven_tree(&fs::read_to_string(tree_path)?);
    assert_eq!(actual, expected);
    Ok(())
}

fn parse_maven_tree(tree: &str) -> BTreeSet<String> {
    tree.lines()
        .filter_map(|line| {
            let start = line.find("org.test:")?;
            let mut fields = line.get(start..)?.split(':');
            let group = fields.next()?;
            let artifact = fields.next()?;
            let _extension = fields.next()?;
            let version = fields.next()?;
            Some(format!("{group}:{artifact}:{version}"))
        })
        .collect()
}
