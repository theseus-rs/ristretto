//! Transactional destinations for materialized artifacts.
//!
//! [`ArtifactDestination`] begins one [`ArtifactTransaction`] per selected artifact. Bytes remain
//! uncommitted until transport and checksum verification succeed; failures call
//! [`ArtifactTransaction::abort`]. Native callers can use [`FileDestination`] for atomic
//! filesystem delivery, while custom destinations can stream into memory, object storage, or
//! another application-owned store.

#[cfg(not(target_family = "wasm"))]
use crate::Error;
use crate::{BoxFuture, ResolvedArtifact, Result};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::fmt;

#[cfg(not(target_family = "wasm"))]
static FILE_TRANSACTION_NONCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Starts transactional writes for resolved artifacts.
pub trait ArtifactDestination: fmt::Debug + Send + Sync {
    /// Destination-specific write transaction.
    type Transaction: ArtifactTransaction<Output = Self::Output>;
    /// Value returned for a committed artifact.
    type Output: fmt::Debug;

    /// Begins an uncommitted artifact write.
    fn begin<'a>(
        &'a self,
        artifact: &'a ResolvedArtifact,
    ) -> BoxFuture<'a, Result<Self::Transaction>>;
}

/// An in-progress streamed artifact write.
pub trait ArtifactTransaction: fmt::Debug {
    /// Value returned after commit.
    type Output: fmt::Debug;

    /// Writes one body chunk.
    fn write(&mut self, chunk: Bytes) -> BoxFuture<'_, Result<()>>;

    /// Makes the artifact visible and returns its destination representation.
    fn commit(self) -> BoxFuture<'static, Result<Self::Output>>;

    /// Abandons the artifact and removes partial state.
    fn abort(self) -> BoxFuture<'static, Result<()>>;
}

/// One successfully delivered artifact and its destination output.
#[derive(Debug)]
pub struct DownloadedArtifact<T: fmt::Debug> {
    /// Artifact metadata.
    pub artifact: ResolvedArtifact,
    /// Handler-specific committed value.
    pub output: T,
    /// Verified checksum, if the repository supplied one.
    pub checksum: Option<VerifiedChecksum>,
}

/// Result of delivering every requested artifact.
#[derive(Debug)]
pub struct DownloadReport<T: fmt::Debug> {
    /// Committed artifacts in deterministic classpath order.
    pub artifacts: Vec<DownloadedArtifact<T>>,
    /// Non-fatal warnings, including missing checksums under warn policy.
    pub diagnostics: Vec<String>,
}

/// A checksum validated during transfer.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct VerifiedChecksum {
    /// Checksum algorithm name.
    pub algorithm: String,
    /// Lowercase hexadecimal digest.
    pub value: String,
}

/// Native filesystem destination.
#[cfg(not(target_family = "wasm"))]
#[derive(Debug, Clone)]
pub struct FileDestination {
    root: std::path::PathBuf,
}

#[cfg(not(target_family = "wasm"))]
impl FileDestination {
    /// Creates a destination rooted at an explicit directory.
    #[must_use]
    pub fn new(root: impl Into<std::path::PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Returns the configured root.
    #[must_use]
    pub fn root(&self) -> &std::path::Path {
        &self.root
    }
}

/// Filesystem artifact write transaction.
#[cfg(not(target_family = "wasm"))]
pub struct FileTransaction {
    file: Option<tokio::fs::File>,
    temporary_path: std::path::PathBuf,
    final_path: std::path::PathBuf,
}

#[cfg(not(target_family = "wasm"))]
impl fmt::Debug for FileTransaction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FileTransaction")
            .field("temporary_path", &self.temporary_path)
            .field("final_path", &self.final_path)
            .finish_non_exhaustive()
    }
}

#[cfg(not(target_family = "wasm"))]
impl ArtifactDestination for FileDestination {
    type Transaction = FileTransaction;
    type Output = std::path::PathBuf;

    fn begin<'a>(
        &'a self,
        artifact: &'a ResolvedArtifact,
    ) -> BoxFuture<'a, Result<Self::Transaction>> {
        Box::pin(async move {
            drop(crate::ArtifactCoordinate::from_key(
                artifact.coordinate.key.clone(),
                artifact.coordinate.version.clone(),
            )?);
            drop(crate::ArtifactCoordinate::from_key(
                artifact.coordinate.key.clone(),
                artifact.file_version.clone(),
            )?);
            let relative_path = artifact
                .coordinate
                .artifact_path_with_version(&artifact.file_version);
            let final_path = contained_path(&self.root, &self.root.join(relative_path)).await?;
            let (temporary_path, file) = loop {
                let nonce =
                    FILE_TRANSACTION_NONCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let temporary_path = final_path.with_extension(format!(
                    "{}.part-{}-{nonce}",
                    artifact.coordinate.key.extension,
                    std::process::id()
                ));
                match tokio::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&temporary_path)
                    .await
                {
                    Ok(file) => break (temporary_path, file),
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                    Err(error) => return Err(error.into()),
                }
            };
            Ok(FileTransaction {
                file: Some(file),
                temporary_path,
                final_path,
            })
        })
    }
}

#[cfg(not(target_family = "wasm"))]
impl ArtifactTransaction for FileTransaction {
    type Output = std::path::PathBuf;

    fn write(&mut self, chunk: Bytes) -> BoxFuture<'_, Result<()>> {
        Box::pin(async move {
            use tokio::io::AsyncWriteExt;

            let file = self
                .file
                .as_mut()
                .ok_or_else(|| Error::Destination("transaction is closed".to_string()))?;
            file.write_all(&chunk).await?;
            Ok(())
        })
    }

    fn commit(mut self) -> BoxFuture<'static, Result<Self::Output>> {
        Box::pin(async move {
            use tokio::io::AsyncWriteExt;

            let temporary_path = self.temporary_path.clone();
            let final_path = self.final_path.clone();
            if let Some(mut file) = self.file.take() {
                file.flush().await?;
                file.sync_all().await?;
            }
            let existing = optional_metadata(&final_path).await?;
            if let Some(metadata) = existing {
                if metadata.file_type().is_symlink() || !metadata.is_file() {
                    return Err(Error::Destination(format!(
                        "artifact target is not a file: {}",
                        final_path.display()
                    )));
                }
                let backup_path = loop {
                    let candidate = final_path.with_extension(format!(
                        "{}.backup-{}",
                        self.final_path
                            .extension()
                            .and_then(|extension| extension.to_str())
                            .unwrap_or("artifact"),
                        FILE_TRANSACTION_NONCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                    ));
                    if optional_metadata(&candidate).await?.is_none() {
                        break candidate;
                    }
                };
                tokio::fs::rename(&final_path, &backup_path).await?;
                if let Err(error) = tokio::fs::rename(&temporary_path, &final_path).await {
                    drop(tokio::fs::rename(&backup_path, &final_path).await);
                    return Err(error.into());
                }
                drop(tokio::fs::remove_file(backup_path).await);
            } else {
                tokio::fs::rename(&temporary_path, &final_path).await?;
            }
            Ok(final_path)
        })
    }

    fn abort(mut self) -> BoxFuture<'static, Result<()>> {
        Box::pin(async move {
            let temporary_path = self.temporary_path.clone();
            drop(self.file.take());
            match tokio::fs::remove_file(&temporary_path).await {
                Ok(()) => Ok(()),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
                Err(error) => Err(error.into()),
            }
        })
    }
}

#[cfg(not(target_family = "wasm"))]
impl Drop for FileTransaction {
    fn drop(&mut self) {
        drop(self.file.take());
        drop(std::fs::remove_file(&self.temporary_path));
    }
}

#[cfg(not(target_family = "wasm"))]
pub(crate) async fn contained_path(
    root: &std::path::Path,
    path: &std::path::Path,
) -> Result<std::path::PathBuf> {
    tokio::fs::create_dir_all(root).await?;
    let canonical_root = tokio::fs::canonicalize(root).await?;
    let relative = path
        .strip_prefix(root)
        .or_else(|_| path.strip_prefix(&canonical_root))
        .map_err(|_| {
            Error::Destination(format!(
                "artifact path is outside destination root: {}",
                path.display()
            ))
        })?;
    let relative_parent = relative.parent().ok_or_else(|| {
        Error::Destination(format!("artifact path has no parent: {}", path.display()))
    })?;
    let mut parent = canonical_root.clone();
    for component in relative_parent.components() {
        let std::path::Component::Normal(component) = component else {
            return Err(Error::Destination(format!(
                "artifact path contains an invalid component: {}",
                path.display()
            )));
        };
        parent.push(component);
        match tokio::fs::symlink_metadata(&parent).await {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
                return Err(Error::Destination(format!(
                    "artifact path traverses a non-directory or symlink: {}",
                    path.display()
                )));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                tokio::fs::create_dir(&parent).await?;
            }
            Err(error) => return Err(error.into()),
        }
    }
    let canonical_parent = tokio::fs::canonicalize(parent).await?;
    ensure_contained(&canonical_root, &canonical_parent, path)?;
    let file_name = path.file_name().ok_or_else(|| {
        Error::Destination(format!(
            "artifact path has no file name: {}",
            path.display()
        ))
    })?;
    Ok(canonical_parent.join(file_name))
}

#[cfg(not(target_family = "wasm"))]
async fn optional_metadata(path: &std::path::Path) -> Result<Option<std::fs::Metadata>> {
    match tokio::fs::symlink_metadata(path).await {
        Ok(metadata) => Ok(Some(metadata)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.into()),
    }
}

#[cfg(not(target_family = "wasm"))]
fn ensure_contained(
    root: &std::path::Path,
    parent: &std::path::Path,
    path: &std::path::Path,
) -> Result<()> {
    if parent.starts_with(root) {
        Ok(())
    } else {
        Err(Error::Destination(format!(
            "artifact path resolves outside destination root: {}",
            path.display()
        )))
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use crate::{ArtifactCoordinate, NodeId, ResolvedArtifact};
    use tempfile::TempDir;

    fn artifact() -> Result<ResolvedArtifact> {
        let coordinate = ArtifactCoordinate::new("org.example", "demo", "1.0")?;
        Ok(ResolvedArtifact {
            node: NodeId(0),
            url: "memory://repository/demo.jar".to_string(),
            file_version: "1.0".to_string(),
            repository_id: "memory".to_string(),
            coordinate,
        })
    }

    #[tokio::test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    async fn commits_aborts_and_rejects_closed_file_transactions() -> Result<()> {
        let directory = TempDir::new()?;
        let destination = FileDestination::new(directory.path());
        assert_eq!(destination.root(), directory.path());
        let mut transaction = destination.begin(&artifact()?).await?;
        assert!(format!("{transaction:?}").contains("FileTransaction"));
        transaction.write(Bytes::from_static(b"artifact")).await?;
        let path = transaction.commit().await?;
        assert_eq!(tokio::fs::read(path).await?, b"artifact");

        let mut replacement = destination.begin(&artifact()?).await?;
        replacement
            .write(Bytes::from_static(b"replacement"))
            .await?;
        let path = replacement.commit().await?;
        assert_eq!(tokio::fs::read(path).await?, b"replacement");

        let mut failed_replacement = destination.begin(&artifact()?).await?;
        failed_replacement
            .write(Bytes::from_static(b"not-committed"))
            .await?;
        tokio::fs::remove_file(&failed_replacement.temporary_path).await?;
        assert!(failed_replacement.commit().await.is_err());
        let final_path = directory.path().join("org/example/demo/1.0/demo-1.0.jar");
        assert_eq!(tokio::fs::read(final_path).await?, b"replacement");

        let transaction = destination.begin(&artifact()?).await?;
        let temporary_path = transaction.temporary_path.clone();
        transaction.abort().await?;
        assert!(!temporary_path.exists());

        let closed = FileTransaction {
            file: None,
            temporary_path: directory.path().join("missing.part"),
            final_path: directory.path().join("missing.jar"),
        };
        let mut closed_write = closed;
        assert!(
            closed_write
                .write(Bytes::from_static(b"data"))
                .await
                .is_err()
        );
        closed_write.abort().await?;
        Ok(())
    }

    #[tokio::test]
    async fn rejects_invalid_artifacts_and_commit_destination_errors() {
        let directory = TempDir::new().expect("temporary directory");
        let destination = FileDestination::new(directory.path());
        let mut invalid = artifact().expect("artifact");
        invalid.file_version = "..".to_string();
        assert!(destination.begin(&invalid).await.is_err());
        let mut invalid = artifact().expect("artifact");
        invalid.coordinate.version = "..".to_string();
        assert!(destination.begin(&invalid).await.is_err());
        assert!(
            contained_path(
                directory.path(),
                &directory.path().join("../escape/artifact.jar")
            )
            .await
            .is_err()
        );
        assert!(
            contained_path(directory.path(), directory.path())
                .await
                .is_err()
        );
        assert!(
            contained_path(
                directory.path(),
                &directory.path().join("filename-parent/..")
            )
            .await
            .is_err()
        );

        let blocked_parent = directory.path().join("blocked-parent");
        tokio::fs::write(&blocked_parent, b"file")
            .await
            .expect("blocked parent fixture");
        assert!(
            contained_path(directory.path(), &blocked_parent.join("child/artifact.jar"))
                .await
                .is_err()
        );

        let temporary_path = directory.path().join("temporary");
        tokio::fs::write(&temporary_path, b"artifact")
            .await
            .expect("temporary artifact");
        let final_path = directory.path().join("directory");
        tokio::fs::create_dir(&final_path)
            .await
            .expect("destination directory");
        let transaction = FileTransaction {
            file: None,
            temporary_path,
            final_path,
        };
        assert!(transaction.commit().await.is_err());

        let missing_transaction = FileTransaction {
            file: None,
            temporary_path: directory.path().join("not-created.part"),
            final_path: directory.path().join("not-created.jar"),
        };
        assert!(missing_transaction.commit().await.is_err());

        let abort_directory = directory.path().join("abort-directory");
        tokio::fs::create_dir(&abort_directory)
            .await
            .expect("abort directory");
        let abort_transaction = FileTransaction {
            file: None,
            temporary_path: abort_directory,
            final_path: directory.path().join("unused.jar"),
        };
        assert!(abort_transaction.abort().await.is_err());
    }

    #[cfg(unix)]
    #[tokio::test]
    #[expect(
        clippy::too_many_lines,
        reason = "the transaction collision and permission scenarios share filesystem setup"
    )]
    async fn handles_file_collisions_and_permission_failures() {
        use std::os::unix::fs::PermissionsExt;

        let directory = TempDir::new().expect("temporary destination");
        let destination = FileDestination::new(directory.path());
        let artifact = artifact().expect("artifact");
        let final_path = directory.path().join(artifact.coordinate.artifact_path());
        let parent = final_path.parent().expect("artifact parent");
        tokio::fs::create_dir_all(parent)
            .await
            .expect("artifact parent");

        let first_nonce = FILE_TRANSACTION_NONCE.load(std::sync::atomic::Ordering::Relaxed);
        for nonce in first_nonce..first_nonce + 16 {
            let collision = final_path.with_extension(format!(
                "{}.part-{}-{nonce}",
                artifact.coordinate.key.extension,
                std::process::id()
            ));
            tokio::fs::write(collision, b"collision")
                .await
                .expect("temporary collision");
        }
        let collision_transaction = destination
            .begin(&artifact)
            .await
            .expect("transaction after collisions");
        collision_transaction
            .abort()
            .await
            .expect("abort collision transaction");

        tokio::fs::write(&final_path, b"old")
            .await
            .expect("existing artifact");
        let mut replacement = destination
            .begin(&artifact)
            .await
            .expect("replacement transaction");
        replacement
            .write(Bytes::from_static(b"new"))
            .await
            .expect("replacement bytes");
        let backup_nonce = FILE_TRANSACTION_NONCE.load(std::sync::atomic::Ordering::Relaxed);
        for nonce in backup_nonce..backup_nonce + 16 {
            let collision = final_path.with_extension(format!(
                "{}.backup-{nonce}",
                final_path
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .unwrap_or("artifact")
            ));
            tokio::fs::write(collision, b"collision")
                .await
                .expect("backup collision");
        }
        replacement.commit().await.expect("replacement commit");

        let original_permissions = tokio::fs::metadata(parent)
            .await
            .expect("parent metadata")
            .permissions();
        tokio::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o555))
            .await
            .expect("read-only parent");
        let open_error = destination.begin(&artifact).await;
        tokio::fs::set_permissions(parent, original_permissions)
            .await
            .expect("restore parent permissions");
        assert!(open_error.is_err());

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
        let contained_error =
            contained_path(directory.path(), &blocked.join("child/artifact.jar")).await;
        let temporary_path = directory.path().join("permission.part");
        tokio::fs::write(&temporary_path, b"artifact")
            .await
            .expect("permission fixture");
        let commit_error = FileTransaction {
            file: None,
            temporary_path,
            final_path: blocked.join("artifact.jar"),
        }
        .commit()
        .await;
        tokio::fs::set_permissions(&blocked, blocked_permissions)
            .await
            .expect("restore blocked permissions");
        assert!(contained_error.is_err());
        assert!(commit_error.is_err());
        assert!(
            ensure_contained(
                directory.path(),
                &std::path::PathBuf::from("/outside"),
                &directory.path().join("artifact.jar"),
            )
            .is_err()
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn rejects_symlink_escapes_and_symlink_targets() {
        let directory = TempDir::new().expect("temporary destination");
        let outside = TempDir::new().expect("temporary outside directory");
        std::os::unix::fs::symlink(outside.path(), directory.path().join("org"))
            .expect("directory symlink");
        let destination = FileDestination::new(directory.path());
        assert!(
            destination
                .begin(&artifact().expect("artifact"))
                .await
                .is_err()
        );
        assert!(!outside.path().join("example").exists());

        tokio::fs::remove_file(directory.path().join("org"))
            .await
            .expect("remove directory symlink");
        let mut transaction = destination
            .begin(&artifact().expect("artifact"))
            .await
            .expect("transaction");
        transaction
            .write(Bytes::from_static(b"replacement"))
            .await
            .expect("transaction write");
        let outside_file = outside.path().join("outside.jar");
        tokio::fs::write(&outside_file, b"outside")
            .await
            .expect("outside fixture");
        std::os::unix::fs::symlink(&outside_file, &transaction.final_path).expect("target symlink");
        assert!(transaction.commit().await.is_err());
        assert_eq!(
            tokio::fs::read(outside_file)
                .await
                .expect("outside contents"),
            b"outside"
        );
    }
}
