//! Repository-related types.

use crate::types::{ChecksumPolicy, RepositoryLayout, UpdatePolicy};
use serde::{Deserialize, Serialize};

/// Represents a list of repositories.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Repositories {
    /// The repositories.
    #[serde(rename = "repository", default)]
    pub repositories: Vec<Repository>,
}

impl Repositories {
    /// Creates an empty `Repositories`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `Repositories` from a vector of repositories.
    #[must_use]
    pub fn from_vec(repositories: Vec<Repository>) -> Self {
        Self { repositories }
    }

    /// Adds a repository.
    pub fn add(&mut self, repository: Repository) {
        self.repositories.push(repository);
    }
}

/// Represents a repository.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    /// The ID of the repository.
    pub id: String,
    /// The name of the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the repository.
    pub url: String,
    /// The layout of the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<RepositoryLayout>,
    /// The releases policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub releases: Option<RepositoryPolicy>,
    /// The snapshots policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<RepositoryPolicy>,
}

impl Repository {
    /// Creates a new `Repository` with the minimum required fields.
    #[must_use]
    pub fn new(id: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: None,
            url: url.into(),
            layout: None,
            releases: None,
            snapshots: None,
        }
    }

    /// Creates a builder for `Repository`.
    #[must_use]
    pub fn builder(id: impl Into<String>, url: impl Into<String>) -> RepositoryBuilder {
        RepositoryBuilder::new(id, url)
    }
}

/// Builder for `Repository`.
#[derive(Debug, Clone)]
pub struct RepositoryBuilder {
    repository: Repository,
}

impl RepositoryBuilder {
    /// Creates a new builder with the required fields.
    #[must_use]
    pub fn new(id: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            repository: Repository::new(id, url),
        }
    }

    /// Sets the name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.repository.name = Some(name.into());
        self
    }

    /// Sets the layout.
    #[must_use]
    pub fn layout(mut self, layout: RepositoryLayout) -> Self {
        self.repository.layout = Some(layout);
        self
    }

    /// Sets the releases policy.
    #[must_use]
    pub fn releases(mut self, policy: RepositoryPolicy) -> Self {
        self.repository.releases = Some(policy);
        self
    }

    /// Sets the snapshots policy.
    #[must_use]
    pub fn snapshots(mut self, policy: RepositoryPolicy) -> Self {
        self.repository.snapshots = Some(policy);
        self
    }

    /// Builds the `Repository`.
    #[must_use]
    pub fn build(self) -> Repository {
        self.repository
    }
}

/// Represents a repository policy.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryPolicy {
    /// Whether the policy is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// The update policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_policy: Option<UpdatePolicy>,
    /// The checksum policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_policy: Option<ChecksumPolicy>,
}

impl RepositoryPolicy {
    /// Creates a new `RepositoryPolicy` with enabled set to true.
    #[must_use]
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            update_policy: None,
            checksum_policy: None,
        }
    }

    /// Creates a new `RepositoryPolicy` with enabled set to false.
    #[must_use]
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            update_policy: None,
            checksum_policy: None,
        }
    }

    /// Creates a builder for `RepositoryPolicy`.
    #[must_use]
    pub fn builder() -> RepositoryPolicyBuilder {
        RepositoryPolicyBuilder::new()
    }
}

/// Builder for `RepositoryPolicy`.
#[derive(Debug, Clone, Default)]
pub struct RepositoryPolicyBuilder {
    enabled: bool,
    update_policy: Option<UpdatePolicy>,
    checksum_policy: Option<ChecksumPolicy>,
}

impl RepositoryPolicyBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether the policy is enabled.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Sets the update policy.
    #[must_use]
    pub fn update_policy(mut self, policy: UpdatePolicy) -> Self {
        self.update_policy = Some(policy);
        self
    }

    /// Sets the checksum policy.
    #[must_use]
    pub fn checksum_policy(mut self, policy: ChecksumPolicy) -> Self {
        self.checksum_policy = Some(policy);
        self
    }

    /// Builds the `RepositoryPolicy`.
    #[must_use]
    pub fn build(self) -> RepositoryPolicy {
        RepositoryPolicy {
            enabled: self.enabled,
            update_policy: self.update_policy,
            checksum_policy: self.checksum_policy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_builder() {
        let repo = Repository::builder("central", "https://repo.maven.apache.org/maven2")
            .name("Maven Central")
            .snapshots(RepositoryPolicy::disabled())
            .build();

        assert_eq!(repo.id, "central");
        assert_eq!(repo.url, "https://repo.maven.apache.org/maven2");
        assert_eq!(repo.name, Some("Maven Central".to_string()));
        assert!(repo.snapshots.is_some());
        assert!(!repo.snapshots.as_ref().unwrap().enabled);
    }

    #[test]
    fn test_repository_policy_builder() {
        let policy = RepositoryPolicy::builder()
            .enabled(true)
            .update_policy(UpdatePolicy::Daily)
            .checksum_policy(ChecksumPolicy::Fail)
            .build();

        assert!(policy.enabled);
        assert_eq!(policy.update_policy, Some(UpdatePolicy::Daily));
        assert_eq!(policy.checksum_policy, Some(ChecksumPolicy::Fail));
    }
}
