//! Maven repository metadata and version discovery results.
//!
//! [`VersionListing`] merges `maven-metadata.xml` from all eligible repositories while retaining
//! which repositories advertised each [`AvailableVersion`]. It also preserves repository-specific
//! `latest`, `release`, and `lastUpdated` markers and selects concrete versions with Maven
//! ordering and range semantics.

use crate::{ArtifactCoordinate, ArtifactKey, RemoteRepository, Version, VersionSpec};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// A repository-provenanced available version.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AvailableVersion {
    /// Version value.
    pub version: Version,
    /// Repositories that advertised this value.
    pub repositories: BTreeSet<String>,
    /// Whether the value is a snapshot base version.
    pub snapshot: bool,
}

/// Result of artifact-level version discovery.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct VersionListing {
    /// Versions in ascending Maven order.
    pub versions: Vec<AvailableVersion>,
    /// Repository-provided latest markers.
    pub latest: BTreeMap<String, String>,
    /// Repository-provided latest release markers.
    pub release: BTreeMap<String, String>,
    /// Repository-provided last-updated markers.
    pub last_updated: BTreeMap<String, String>,
}

impl VersionListing {
    /// Selects the greatest version satisfying a specification.
    #[must_use]
    pub fn select(&self, specification: &VersionSpec) -> Option<Version> {
        match specification {
            VersionSpec::Latest => greatest_marker(self.latest.values())
                .or_else(|| self.versions.last().map(|version| version.version.clone())),
            VersionSpec::Release => greatest_marker(
                self.release
                    .values()
                    .filter(|version| !version.ends_with("-SNAPSHOT")),
            )
            .or_else(|| {
                self.versions
                    .iter()
                    .rev()
                    .find(|version| !version.snapshot)
                    .map(|version| version.version.clone())
            }),
            VersionSpec::Exact(version) => Some(version.clone()),
            VersionSpec::Ranges(_) => self
                .versions
                .iter()
                .rev()
                .find(|version| specification.matches(&version.version))
                .map(|version| version.version.clone()),
        }
    }

    pub(crate) fn merge(&mut self, repository: &RemoteRepository, metadata: RepositoryMetadata) {
        if let Some(versioning) = metadata.versioning {
            if let Some(latest) = versioning
                .latest
                .filter(|version| repository.accepts_version(version))
            {
                self.latest.insert(repository.id.clone(), latest);
            }
            if repository.releases.enabled
                && let Some(release) = versioning.release
            {
                self.release.insert(repository.id.clone(), release);
            }
            if let Some(updated) = versioning.last_updated {
                self.last_updated.insert(repository.id.clone(), updated);
            }
            for value in versioning.versions.versions {
                let version = Version::new(value);
                if !repository.accepts_version(version.as_str()) {
                    continue;
                }
                if let Some(existing) = self
                    .versions
                    .iter_mut()
                    .find(|candidate| candidate.version.as_str() == version.as_str())
                {
                    existing.repositories.insert(repository.id.clone());
                } else {
                    self.versions.push(AvailableVersion {
                        snapshot: version.as_str().ends_with("-SNAPSHOT"),
                        version,
                        repositories: BTreeSet::from([repository.id.clone()]),
                    });
                }
            }
            self.versions.sort_by(|left, right| {
                left.version
                    .cmp(&right.version)
                    .then_with(|| left.version.as_str().cmp(right.version.as_str()))
            });
        }
    }
}

fn greatest_marker<'a>(markers: impl Iterator<Item = &'a String>) -> Option<Version> {
    markers.map(Version::new).max()
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RepositoryMetadata {
    pub(crate) group_id: Option<String>,
    pub(crate) artifact_id: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) versioning: Option<MetadataVersioning>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MetadataVersioning {
    pub(crate) latest: Option<String>,
    pub(crate) release: Option<String>,
    #[serde(default)]
    pub(crate) versions: MetadataVersions,
    pub(crate) last_updated: Option<String>,
    pub(crate) snapshot: Option<Snapshot>,
    #[serde(default)]
    pub(crate) snapshot_versions: SnapshotVersions,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct MetadataVersions {
    #[serde(rename = "version", default)]
    pub(crate) versions: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Snapshot {
    pub(crate) timestamp: Option<String>,
    pub(crate) build_number: Option<u64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct SnapshotVersions {
    #[serde(rename = "snapshotVersion", default)]
    pub(crate) versions: Vec<SnapshotVersion>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct SnapshotVersion {
    pub(crate) classifier: Option<String>,
    pub(crate) extension: String,
    pub(crate) value: String,
    pub(crate) updated: Option<String>,
}

impl RepositoryMetadata {
    pub(crate) fn matches_key(&self, key: &ArtifactKey) -> bool {
        self.group_id
            .as_deref()
            .is_none_or(|group_id| group_id == key.group_id)
            && self
                .artifact_id
                .as_deref()
                .is_none_or(|artifact_id| artifact_id == key.artifact_id)
    }

    pub(crate) fn matches_coordinate(&self, coordinate: &ArtifactCoordinate) -> bool {
        self.matches_key(&coordinate.key)
            && self
                .version
                .as_deref()
                .is_none_or(|version| version == coordinate.version)
    }

    pub(crate) fn snapshot_version(
        &self,
        base_version: &str,
        extension: &str,
        classifier: Option<&str>,
    ) -> Option<String> {
        let versioning = self.versioning.as_ref()?;
        if let Some(version) = versioning
            .snapshot_versions
            .versions
            .iter()
            .filter(|version| {
                version.extension == extension && version.classifier.as_deref() == classifier
            })
            .max_by(|left, right| {
                left.updated
                    .cmp(&right.updated)
                    .then_with(|| left.value.cmp(&right.value))
            })
        {
            return Some(version.value.clone());
        }
        let snapshot = versioning.snapshot.as_ref()?;
        let timestamp = snapshot.timestamp.as_ref()?;
        let build_number = snapshot.build_number?;
        Some(format!(
            "{}-{timestamp}-{build_number}",
            base_version.trim_end_matches("-SNAPSHOT")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    fn parses_version_and_snapshot_metadata() -> Result<()> {
        let xml = r"<metadata><versioning><latest>2.0</latest><release>1.0</release>
              <versions><version>1.0</version><version>2.0-SNAPSHOT</version></versions>
              <snapshot><timestamp>20260101.120000</timestamp><buildNumber>3</buildNumber></snapshot>
              <snapshotVersions><snapshotVersion><extension>jar</extension>
              <value>2.0-20260101.120000-3</value><updated>20260101120000</updated>
              </snapshotVersion></snapshotVersions></versioning></metadata>";
        let metadata: RepositoryMetadata = quick_xml::de::from_str(xml)?;
        assert_eq!(
            metadata.snapshot_version("2.0-SNAPSHOT", "jar", None),
            Some("2.0-20260101.120000-3".to_string())
        );
        let mut listing = VersionListing::default();
        listing.merge(
            &RemoteRepository::new("test", "https://example.test/repository/")?,
            metadata,
        );
        assert_eq!(listing.versions.len(), 2);
        Ok(())
    }

    #[test]
    fn special_selectors_honor_repository_markers() {
        let listing = VersionListing {
            versions: vec![
                AvailableVersion {
                    version: Version::new("1.0"),
                    repositories: BTreeSet::new(),
                    snapshot: false,
                },
                AvailableVersion {
                    version: Version::new("3.0"),
                    repositories: BTreeSet::new(),
                    snapshot: false,
                },
            ],
            latest: BTreeMap::from([
                ("first".to_string(), "2.0".to_string()),
                ("second".to_string(), "2.5".to_string()),
            ]),
            release: BTreeMap::from([("first".to_string(), "2.0".to_string())]),
            last_updated: BTreeMap::new(),
        };
        assert_eq!(
            listing.select(&VersionSpec::Latest),
            Some(Version::new("2.5"))
        );
        assert_eq!(
            listing.select(&VersionSpec::Release),
            Some(Version::new("2.0"))
        );
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    fn merges_provenance_policies_and_selector_fallbacks() -> Result<()> {
        let mut releases = RemoteRepository::new("releases", "https://releases.example/")?;
        releases.snapshots.enabled = false;
        let mut snapshots = RemoteRepository::new("snapshots", "https://snapshots.example/")?;
        snapshots.releases.enabled = false;

        let xml = "<metadata><groupId>org.example</groupId><artifactId>demo</artifactId>\
             <versioning><latest>2.0-SNAPSHOT</latest><release>1.0</release>\
             <lastUpdated>20260725000000</lastUpdated><versions><version>1.0</version>\
             <version>2.0-SNAPSHOT</version></versions></versioning></metadata>";
        let release_metadata: RepositoryMetadata = quick_xml::de::from_str(xml)?;
        let snapshot_metadata = release_metadata.clone();
        let mut listing = VersionListing::default();
        listing.merge(&releases, release_metadata);
        listing.merge(&snapshots, snapshot_metadata);
        assert_eq!(listing.versions.len(), 2);
        assert_eq!(
            listing
                .versions
                .iter()
                .find(|version| version.version.as_str() == "1.0")
                .map(|version| &version.repositories),
            Some(&BTreeSet::from(["releases".to_string()]))
        );
        assert_eq!(
            listing.select(&"[1,2)".parse()?),
            Some(Version::new("2.0-SNAPSHOT"))
        );
        assert_eq!(
            listing.select(&VersionSpec::Exact(Version::new("9.0"))),
            Some(Version::new("9.0"))
        );

        listing.latest.clear();
        listing.release.clear();
        assert_eq!(
            listing.select(&VersionSpec::Latest),
            Some(Version::new("2.0-SNAPSHOT"))
        );
        assert_eq!(
            listing.select(&VersionSpec::Release),
            Some(Version::new("1.0"))
        );
        Ok(())
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    fn validates_metadata_coordinates_and_snapshot_fallbacks() -> Result<()> {
        let key = ArtifactKey::new("org.example", "demo")?;
        let coordinate = ArtifactCoordinate::new("org.example", "demo", "1.0-SNAPSHOT")?;
        let metadata = RepositoryMetadata {
            group_id: Some("org.example".to_string()),
            artifact_id: Some("demo".to_string()),
            version: Some("1.0-SNAPSHOT".to_string()),
            versioning: Some(MetadataVersioning {
                snapshot: Some(Snapshot {
                    timestamp: Some("20260725.120000".to_string()),
                    build_number: Some(4),
                }),
                ..MetadataVersioning::default()
            }),
        };
        assert!(metadata.matches_key(&key));
        assert!(metadata.matches_coordinate(&coordinate));
        assert_eq!(
            metadata.snapshot_version("1.0-SNAPSHOT", "jar", None),
            Some("1.0-20260725.120000-4".to_string())
        );

        let mismatched = RepositoryMetadata {
            group_id: Some("other".to_string()),
            ..metadata.clone()
        };
        assert!(!mismatched.matches_key(&key));
        assert!(!mismatched.matches_coordinate(&coordinate));
        assert!(
            RepositoryMetadata::default()
                .snapshot_version("1.0-SNAPSHOT", "jar", None)
                .is_none()
        );
        assert!(
            RepositoryMetadata {
                versioning: Some(MetadataVersioning::default()),
                ..RepositoryMetadata::default()
            }
            .snapshot_version("1.0-SNAPSHOT", "jar", None)
            .is_none()
        );
        assert!(
            RepositoryMetadata {
                versioning: Some(MetadataVersioning {
                    snapshot: Some(Snapshot {
                        timestamp: None,
                        build_number: Some(1),
                    }),
                    ..MetadataVersioning::default()
                }),
                ..RepositoryMetadata::default()
            }
            .snapshot_version("1.0-SNAPSHOT", "jar", None)
            .is_none()
        );
        Ok(())
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    fn merges_duplicate_versions_and_selects_newest_snapshot_entry() -> Result<()> {
        let first = RemoteRepository::new("first", "https://first.example/")?;
        let second = RemoteRepository::new("second", "https://second.example/")?;
        let xml = "<metadata><versioning><versions><version>1</version><version>1.0</version>\
             </versions></versioning></metadata>";
        let metadata: RepositoryMetadata = quick_xml::de::from_str(xml)?;
        let mut listing = VersionListing::default();
        listing.merge(&first, RepositoryMetadata::default());
        listing.merge(&first, metadata.clone());
        listing.merge(&second, metadata);
        assert_eq!(
            listing
                .versions
                .first()
                .expect("merged version")
                .repositories,
            BTreeSet::from(["first".to_string(), "second".to_string()])
        );
        assert_eq!(listing.versions.len(), 2);

        let xml = "<metadata><versioning><snapshotVersions>\
             <snapshotVersion><extension>jar</extension><value>1.0-20260101.000000-1</value>\
             <updated>20260101000000</updated></snapshotVersion>\
             <snapshotVersion><extension>jar</extension><value>1.0-20260102.000000-2</value>\
             <updated>20260102000000</updated></snapshotVersion>\
             </snapshotVersions></versioning></metadata>";
        let metadata: RepositoryMetadata = quick_xml::de::from_str(xml)?;
        assert_eq!(
            metadata.snapshot_version("1.0-SNAPSHOT", "jar", None),
            Some("1.0-20260102.000000-2".to_string())
        );
        Ok(())
    }
}
