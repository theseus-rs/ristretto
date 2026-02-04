//! Distribution management types.

use crate::repository::Repository;
use serde::{Deserialize, Serialize};

/// Represents the distribution management information.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DistributionManagement {
    /// The repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    /// The snapshot repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_repository: Option<Repository>,
    /// The site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
    /// The download URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// The relocation information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relocation: Option<Relocation>,
    /// The status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl DistributionManagement {
    /// Creates a builder for `DistributionManagement`.
    #[must_use]
    pub fn builder() -> DistributionManagementBuilder {
        DistributionManagementBuilder::new()
    }
}

/// Builder for `DistributionManagement`.
#[derive(Debug, Clone, Default)]
pub struct DistributionManagementBuilder {
    dm: DistributionManagement,
}

impl DistributionManagementBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the repository.
    #[must_use]
    pub fn repository(mut self, repo: Repository) -> Self {
        self.dm.repository = Some(repo);
        self
    }

    /// Sets the snapshot repository.
    #[must_use]
    pub fn snapshot_repository(mut self, repo: Repository) -> Self {
        self.dm.snapshot_repository = Some(repo);
        self
    }

    /// Sets the site.
    #[must_use]
    pub fn site(mut self, site: Site) -> Self {
        self.dm.site = Some(site);
        self
    }

    /// Sets the download URL.
    #[must_use]
    pub fn download_url(mut self, url: impl Into<String>) -> Self {
        self.dm.download_url = Some(url.into());
        self
    }

    /// Sets the relocation.
    #[must_use]
    pub fn relocation(mut self, relocation: Relocation) -> Self {
        self.dm.relocation = Some(relocation);
        self
    }

    /// Sets the status.
    #[must_use]
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.dm.status = Some(status.into());
        self
    }

    /// Builds the `DistributionManagement`.
    #[must_use]
    pub fn build(self) -> DistributionManagement {
        self.dm
    }
}

/// Represents a site.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    /// The ID of the site.
    pub id: String,
    /// The name of the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the site.
    pub url: String,
}

impl Site {
    /// Creates a new `Site`.
    #[must_use]
    pub fn new(id: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: None,
            url: url.into(),
        }
    }

    /// Creates a builder for `Site`.
    #[must_use]
    pub fn builder(id: impl Into<String>, url: impl Into<String>) -> SiteBuilder {
        SiteBuilder::new(id, url)
    }
}

/// Builder for `Site`.
#[derive(Debug, Clone)]
pub struct SiteBuilder {
    site: Site,
}

impl SiteBuilder {
    /// Creates a new builder with the required fields.
    #[must_use]
    pub fn new(id: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            site: Site::new(id, url),
        }
    }

    /// Sets the name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.site.name = Some(name.into());
        self
    }

    /// Builds the `Site`.
    #[must_use]
    pub fn build(self) -> Site {
        self.site
    }
}

/// Represents relocation information.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Relocation {
    /// The group ID to relocate to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The artifact ID to relocate to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    /// The version to relocate to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Relocation {
    /// Creates a builder for `Relocation`.
    #[must_use]
    pub fn builder() -> RelocationBuilder {
        RelocationBuilder::new()
    }
}

/// Builder for `Relocation`.
#[derive(Debug, Clone, Default)]
pub struct RelocationBuilder {
    relocation: Relocation,
}

impl RelocationBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the group ID.
    #[must_use]
    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.relocation.group_id = Some(group_id.into());
        self
    }

    /// Sets the artifact ID.
    #[must_use]
    pub fn artifact_id(mut self, artifact_id: impl Into<String>) -> Self {
        self.relocation.artifact_id = Some(artifact_id.into());
        self
    }

    /// Sets the version.
    #[must_use]
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.relocation.version = Some(version.into());
        self
    }

    /// Sets the message.
    #[must_use]
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.relocation.message = Some(message.into());
        self
    }

    /// Builds the `Relocation`.
    #[must_use]
    pub fn build(self) -> Relocation {
        self.relocation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::Repository;

    #[test]
    fn test_distribution_management_builder() {
        let dm = DistributionManagement::builder()
            .repository(Repository::new(
                "releases",
                "https://repo.example.com/releases",
            ))
            .snapshot_repository(Repository::new(
                "snapshots",
                "https://repo.example.com/snapshots",
            ))
            .site(Site::new("site", "https://site.example.com"))
            .build();

        assert!(dm.repository.is_some());
        assert!(dm.snapshot_repository.is_some());
        assert!(dm.site.is_some());
    }

    #[test]
    fn test_site_builder() {
        let site = Site::builder("docs", "https://docs.example.com")
            .name("Project Documentation")
            .build();

        assert_eq!(site.id, "docs");
        assert_eq!(site.name, Some("Project Documentation".to_string()));
    }

    #[test]
    fn test_relocation_builder() {
        let relocation = Relocation::builder()
            .group_id("com.new")
            .artifact_id("new-artifact")
            .message("This artifact has been relocated")
            .build();

        assert_eq!(relocation.group_id, Some("com.new".to_string()));
        assert!(relocation.message.is_some());
    }
}
