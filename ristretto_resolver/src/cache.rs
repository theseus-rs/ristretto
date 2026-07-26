//! Persistent, opt-in caching for repository resources.
//!
//! [`FileCache`] stores POMs, metadata, checksum sidecars, and artifacts using Maven repository
//! paths beneath a caller-selected directory. Cache writes are atomic, mutable metadata follows
//! each repository's update policy, and origin records prevent two repositories with identical
//! paths from sharing unverified content.
//!
//! A cache is a resolver implementation detail, unlike
//! [`FileDestination`], which receives the artifacts requested by the
//! caller. This module is available only on native targets.

#![cfg(not(target_family = "wasm"))]

use crate::{
    ArtifactDestination, ArtifactTransaction, BoxFuture, Error, FileDestination, RemoteRepository,
    ResourceKind, Result, TransportResponse,
};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

static CACHE_NONCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[derive(Debug)]
struct TemporaryPath(PathBuf);

impl Drop for TemporaryPath {
    fn drop(&mut self) {
        drop(std::fs::remove_file(&self.0));
    }
}

/// Explicit filesystem cache for repository metadata, POMs, checksums, and artifacts.
#[derive(Debug, Clone)]
pub struct FileCache {
    root: PathBuf,
}

impl FileCache {
    /// Creates a local repository cache at an explicit root.
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Returns the configured cache root.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub(crate) async fn response(
        &self,
        repository: &RemoteRepository,
        url: &str,
        kind: ResourceKind,
        allow_stale: bool,
    ) -> Result<Option<TransportResponse>> {
        let path = self.path_for(repository, url)?;
        if !tokio::fs::try_exists(&path).await? {
            return Ok(None);
        }
        if !allow_stale
            && (matches!(
                kind,
                ResourceKind::Metadata | ResourceKind::SnapshotMetadata
            ) || snapshot_resource_url(url))
            && self.resource_stale(repository, url, &path).await?
        {
            return Ok(None);
        }
        if !matches!(kind, ResourceKind::Metadata)
            && !self.origin_matches(repository, &path).await?
        {
            return Ok(None);
        }
        crate::transport::file_response(path, true).await
    }

    pub(crate) async fn store_small(
        &self,
        repository: &RemoteRepository,
        url: &str,
        bytes: &[u8],
    ) -> Result<()> {
        let path = self.path_for(repository, url)?;
        atomic_write(&self.root, &path, bytes).await?;
        self.track_origin(repository, &path).await
    }

    pub(crate) async fn begin_artifact(
        &self,
        artifact: &crate::ResolvedArtifact,
        repository: &RemoteRepository,
    ) -> Result<CacheTransaction> {
        Ok(CacheTransaction::active(
            FileDestination::new(&self.root).begin(artifact).await?,
            self.clone(),
            repository.clone(),
        ))
    }

    fn path_for(&self, repository: &RemoteRepository, url: &str) -> Result<PathBuf> {
        repository.validate_resource_url(url)?;
        let repository_url = url::Url::parse(&repository.url)?;
        let resource_url = url::Url::parse(url)?;
        let base = repository_url.path().trim_end_matches('/');
        let resource = resource_url.path();
        let relative = resource
            .strip_prefix(base)
            .unwrap_or(resource)
            .trim_start_matches('/');
        let mut path = self.root.join(relative);
        if path.file_name().and_then(|name| name.to_str()) == Some("maven-metadata.xml") {
            let file_name = format!("maven-metadata-{}.xml", cache_repository_id(&repository.id));
            path.set_file_name(file_name);
        }
        Ok(path)
    }

    async fn resource_stale(
        &self,
        repository: &RemoteRepository,
        url: &str,
        path: &Path,
    ) -> Result<bool> {
        let modified = tokio::fs::metadata(path)
            .await?
            .modified()
            .unwrap_or(SystemTime::UNIX_EPOCH);
        let now = SystemTime::now();
        let stale = if snapshot_resource_url(url) {
            repository.snapshots.update.permits_refresh(modified, now)
        } else {
            repository.releases.update.permits_refresh(modified, now)
                || repository.snapshots.update.permits_refresh(modified, now)
        };
        Ok(stale)
    }

    async fn track_origin(&self, repository: &RemoteRepository, artifact: &Path) -> Result<()> {
        let Some(parent) = artifact.parent() else {
            return Ok(());
        };
        let Some(name) = artifact.file_name().and_then(|name| name.to_str()) else {
            return Ok(());
        };
        let tracking_path = parent.join("_remote.repositories");
        let entry = format!("{name}>{}=\n", repository.id);
        let contents = match tokio::fs::read_to_string(&tracking_path).await {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(error) => return Err(error.into()),
        };
        let mut updated = contents
            .lines()
            .filter(|line| !line.starts_with(&format!("{name}>")))
            .fold(String::new(), |mut output, line| {
                output.push_str(line);
                output.push('\n');
                output
            });
        updated.push_str(&entry);
        atomic_write(&self.root, &tracking_path, updated.as_bytes()).await?;
        Ok(())
    }

    async fn origin_matches(&self, repository: &RemoteRepository, artifact: &Path) -> Result<bool> {
        let Some(parent) = artifact.parent() else {
            return Ok(false);
        };
        let Some(name) = artifact.file_name().and_then(|name| name.to_str()) else {
            return Ok(false);
        };
        let tracking_path = parent.join("_remote.repositories");
        let contents = match tokio::fs::read_to_string(tracking_path).await {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(error.into()),
        };
        Ok(contents
            .lines()
            .any(|line| line == format!("{name}>{}=", repository.id)))
    }
}

fn cache_repository_id(id: &str) -> String {
    use std::fmt::Write;

    let mut encoded = String::with_capacity(id.len());
    for byte in id.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_') {
            encoded.push(char::from(byte));
        } else {
            let _ = write!(encoded, "~{byte:02X}");
        }
    }
    encoded
}

fn snapshot_resource_url(value: &str) -> bool {
    url::Url::parse(value).is_ok_and(|url| {
        let mut segments = url.path_segments().into_iter().flatten().rev();
        let _file_name = segments.next();
        segments
            .next()
            .is_some_and(|version| version.ends_with("-SNAPSHOT"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ArtifactCoordinate, NodeId, RepositoryPolicy, ResolvedArtifact, UpdatePolicy};
    use futures_util::StreamExt;
    use tempfile::TempDir;

    #[test]
    fn metadata_cache_repository_ids_are_collision_free() {
        assert_ne!(
            cache_repository_id("first+repo"),
            cache_repository_id("first_2Brepo")
        );
        assert_ne!(
            cache_repository_id("first.repo"),
            cache_repository_id("first_repo")
        );
    }

    #[tokio::test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    async fn stores_reads_and_isolates_cached_resources() -> Result<()> {
        let directory = TempDir::new()?;
        let cache = FileCache::new(directory.path());
        assert_eq!(cache.root(), directory.path());
        let repository = RemoteRepository::new("repo", "https://repo.example/maven/")?;
        let url = repository.resource_url("org/example/demo/1.0/demo-1.0.pom")?;
        cache.store_small(&repository, &url, b"pom").await?;
        cache.store_small(&repository, &url, b"replacement").await?;

        let mut response = cache
            .response(&repository, &url, ResourceKind::Pom, false)
            .await?
            .expect("cached POM");
        assert!(response.from_cache);
        assert_eq!(
            response.body.next().await.transpose()?.as_deref(),
            Some(b"replacement".as_slice())
        );

        let other = RemoteRepository::new("other", "https://repo.example/maven/")?;
        assert!(
            cache
                .response(&other, &url, ResourceKind::Pom, false)
                .await?
                .is_none()
        );
        assert!(
            cache
                .response(
                    &repository,
                    &repository.resource_url("missing.pom")?,
                    ResourceKind::Pom,
                    false,
                )
                .await?
                .is_none()
        );
        Ok(())
    }

    #[tokio::test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    async fn applies_metadata_staleness_and_cache_transactions() -> Result<()> {
        let directory = TempDir::new()?;
        let cache = FileCache::new(directory.path());
        let mut repository = RemoteRepository::new("repo", "https://repo.example/")?;
        repository.releases = RepositoryPolicy {
            update: UpdatePolicy::Always,
            ..RepositoryPolicy::enabled()
        };
        let metadata_url = repository.resource_url("org/example/demo/maven-metadata.xml")?;
        cache
            .store_small(&repository, &metadata_url, b"<metadata/>")
            .await?;
        assert!(
            cache
                .response(&repository, &metadata_url, ResourceKind::Metadata, false)
                .await?
                .is_none()
        );
        assert!(
            cache
                .response(&repository, &metadata_url, ResourceKind::Metadata, true)
                .await?
                .is_some()
        );

        let coordinate = ArtifactCoordinate::new("org.example", "demo", "1.0")?;
        let artifact = ResolvedArtifact {
            node: NodeId(0),
            url: repository.resource_url(&coordinate.artifact_path())?,
            file_version: "1.0".to_string(),
            repository_id: repository.id.clone(),
            coordinate,
        };
        let mut transaction = cache.begin_artifact(&artifact, &repository).await?;
        transaction
            .write(bytes::Bytes::from_static(b"cached"))
            .await?;
        transaction.commit().await?;
        assert!(
            cache
                .response(&repository, &artifact.url, ResourceKind::Artifact, false,)
                .await?
                .is_some()
        );

        let mut disabled = CacheTransaction::Disabled;
        disabled
            .write(bytes::Bytes::from_static(b"ignored"))
            .await?;
        disabled.commit().await?;
        CacheTransaction::Disabled.abort().await;

        let transaction = cache.begin_artifact(&artifact, &repository).await?;
        transaction.abort().await;
        Ok(())
    }

    #[tokio::test]
    async fn handles_tracking_and_atomic_write_edge_paths() {
        let directory = TempDir::new().expect("temporary directory");
        let cache = FileCache::new(directory.path());
        let mut repository =
            RemoteRepository::new("repo", "https://repo.example/").expect("repository");
        repository.snapshots.update = UpdatePolicy::Always;
        assert!(
            cache
                .track_origin(&repository, Path::new("/"))
                .await
                .is_ok()
        );
        assert!(
            !cache
                .origin_matches(&repository, Path::new("/"))
                .await
                .expect("origin result")
        );
        assert!(
            atomic_write(directory.path(), Path::new("/"), b"invalid")
                .await
                .is_err()
        );
        let directory_target = directory.path().join("directory-target");
        tokio::fs::create_dir(&directory_target)
            .await
            .expect("directory target");
        assert!(
            atomic_write(directory.path(), &directory_target, b"invalid")
                .await
                .is_err()
        );

        let snapshot_path = directory.path().join("snapshot");
        tokio::fs::write(&snapshot_path, b"snapshot")
            .await
            .expect("snapshot fixture");
        assert!(
            cache
                .resource_stale(
                    &repository,
                    "https://repo.example/demo/1-SNAPSHOT/demo-1.jar",
                    &snapshot_path,
                )
                .await
                .expect("snapshot staleness")
        );
        repository.releases.update = UpdatePolicy::Never;
        assert!(
            cache
                .resource_stale(
                    &repository,
                    "https://repo.example/demo/1/demo-1.jar",
                    &snapshot_path,
                )
                .await
                .expect("release staleness")
        );

        let tracking_parent = directory.path().join("tracking-error");
        tokio::fs::create_dir_all(tracking_parent.join("_remote.repositories"))
            .await
            .expect("tracking directory");
        let artifact = tracking_parent.join("artifact.jar");
        assert!(cache.track_origin(&repository, &artifact).await.is_err());
        assert!(cache.origin_matches(&repository, &artifact).await.is_err());
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStringExt;
            let non_utf8 = tracking_parent.join(std::ffi::OsString::from_vec(vec![0xff]));
            assert!(cache.track_origin(&repository, &non_utf8).await.is_ok());
            assert!(
                !cache
                    .origin_matches(&repository, &non_utf8)
                    .await
                    .expect("non-UTF-8 origin")
            );
        }
        assert!(!snapshot_resource_url("not a URL"));
        assert!(snapshot_resource_url(
            "https://repo.example/org/example/demo/1.0-SNAPSHOT/demo-1.0-SNAPSHOT.pom"
        ));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn handles_atomic_write_collisions_and_permission_failures() {
        use std::os::unix::fs::PermissionsExt;

        let directory = TempDir::new().expect("temporary cache");
        let path = directory.path().join("nested/artifact.jar");
        tokio::fs::create_dir_all(path.parent().expect("artifact parent"))
            .await
            .expect("artifact parent");

        let first_nonce = CACHE_NONCE.load(std::sync::atomic::Ordering::Relaxed);
        for nonce in first_nonce..first_nonce + 16 {
            let collision = path.with_extension(format!("part-{}-{nonce}", std::process::id()));
            tokio::fs::write(collision, b"collision")
                .await
                .expect("temporary collision");
        }
        atomic_write(directory.path(), &path, b"first")
            .await
            .expect("write after collisions");

        let backup_nonce = CACHE_NONCE.load(std::sync::atomic::Ordering::Relaxed);
        for nonce in backup_nonce..backup_nonce + 16 {
            let collision = path.with_extension(format!("backup-{}-{nonce}", std::process::id()));
            tokio::fs::write(collision, b"collision")
                .await
                .expect("backup collision");
        }
        atomic_write(directory.path(), &path, b"replacement")
            .await
            .expect("replace after collisions");

        let parent = path.parent().expect("artifact parent");
        let original_permissions = tokio::fs::metadata(parent)
            .await
            .expect("parent metadata")
            .permissions();
        tokio::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o555))
            .await
            .expect("read-only parent");
        let error = atomic_write(directory.path(), &path, b"blocked").await;
        tokio::fs::set_permissions(parent, original_permissions)
            .await
            .expect("restore parent permissions");
        assert!(error.is_err());

        let blocked = directory.path().join("blocked");
        tokio::fs::create_dir(&blocked)
            .await
            .expect("blocked directory");
        let blocked_permissions = tokio::fs::metadata(&blocked)
            .await
            .expect("blocked metadata")
            .permissions();
        tokio::fs::set_permissions(&blocked, std::fs::Permissions::from_mode(0o000))
            .await
            .expect("block directory");
        let metadata_error = optional_metadata(&blocked.join("child")).await;
        tokio::fs::set_permissions(&blocked, blocked_permissions)
            .await
            .expect("restore blocked permissions");
        assert!(metadata_error.is_err());

        let original = directory.path().join("replace-original");
        let backup = directory.path().join("replace-backup");
        std::fs::write(&original, b"original").expect("replacement original");
        assert!(
            replace_with_backup(
                &original,
                &directory.path().join("missing-temporary"),
                &backup,
            )
            .await
            .is_err()
        );
        assert_eq!(
            std::fs::read(original).expect("restored original"),
            b"original"
        );
    }
}

async fn atomic_write(root: &Path, path: &Path, bytes: &[u8]) -> Result<()> {
    use tokio::io::AsyncWriteExt;

    let path = crate::destination::contained_path(root, path).await?;
    let (temporary, mut file) = loop {
        let nonce = CACHE_NONCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let temporary = path.with_extension(format!("part-{}-{nonce}", std::process::id()));
        match tokio::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .await
        {
            Ok(file) => break (temporary, file),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error.into()),
        }
    };
    let temporary_guard = TemporaryPath(temporary.clone());
    let write_result = async {
        file.write_all(bytes).await?;
        file.flush().await?;
        file.sync_all().await
    }
    .await;
    drop(file);
    write_result?;
    let existing = optional_metadata(&path).await?;
    let outcome = if let Some(metadata) = existing {
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(Error::InvalidConfiguration(format!(
                "cache target is not a file: {}",
                path.display()
            )));
        }
        let backup = loop {
            let nonce = CACHE_NONCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let candidate = path.with_extension(format!("backup-{}-{nonce}", std::process::id()));
            if optional_metadata(&candidate).await?.is_none() {
                break candidate;
            }
        };
        replace_with_backup(&path, &temporary, &backup).await
    } else {
        tokio::fs::rename(&temporary, &path)
            .await
            .map_err(Into::into)
    };
    drop(temporary_guard);
    outcome
}

async fn optional_metadata(path: &Path) -> Result<Option<std::fs::Metadata>> {
    match tokio::fs::symlink_metadata(path).await {
        Ok(metadata) => Ok(Some(metadata)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.into()),
    }
}

async fn replace_with_backup(path: &Path, temporary: &Path, backup: &Path) -> Result<()> {
    tokio::fs::rename(path, backup).await?;
    if let Err(error) = tokio::fs::rename(temporary, path).await {
        drop(tokio::fs::rename(backup, path).await);
        return Err(error.into());
    }
    drop(tokio::fs::remove_file(backup).await);
    Ok(())
}

#[derive(Debug)]
pub(crate) enum CacheTransaction {
    Active(Box<CacheState>),
    Disabled,
}

#[derive(Debug)]
pub(crate) struct CacheState {
    transaction: crate::FileTransaction,
    cache: FileCache,
    repository: RemoteRepository,
}

impl CacheTransaction {
    pub(crate) fn active(
        transaction: crate::FileTransaction,
        cache: FileCache,
        repository: RemoteRepository,
    ) -> Self {
        Self::Active(Box::new(CacheState {
            transaction,
            cache,
            repository,
        }))
    }

    pub(crate) fn write(&mut self, bytes: bytes::Bytes) -> BoxFuture<'_, Result<()>> {
        match self {
            Self::Active(state) => state.transaction.write(bytes),
            Self::Disabled => Box::pin(async { Ok(()) }),
        }
    }

    pub(crate) async fn commit(self) -> Result<()> {
        if let Self::Active(state) = self {
            let CacheState {
                transaction,
                cache,
                repository,
            } = *state;
            let path = transaction.commit().await?;
            cache.track_origin(&repository, &path).await?;
        }
        Ok(())
    }

    pub(crate) async fn abort(self) {
        if let Self::Active(state) = self {
            drop(state.transaction.abort().await);
        }
    }
}
