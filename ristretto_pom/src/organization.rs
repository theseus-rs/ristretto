//! Organization and parent-related types.

use serde::{Deserialize, Serialize};

/// Represents the parent project of a Maven project.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    /// The group ID of the parent project.
    pub group_id: String,
    /// The artifact ID of the parent project.
    pub artifact_id: String,
    /// The version of the parent project.
    pub version: String,
    /// The relative path to the parent project's POM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}

impl Parent {
    /// Creates a new `Parent` with the required fields.
    #[must_use]
    pub fn new(
        group_id: impl Into<String>,
        artifact_id: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            group_id: group_id.into(),
            artifact_id: artifact_id.into(),
            version: version.into(),
            relative_path: None,
        }
    }

    /// Creates a builder for `Parent`.
    #[must_use]
    pub fn builder(
        group_id: impl Into<String>,
        artifact_id: impl Into<String>,
        version: impl Into<String>,
    ) -> ParentBuilder {
        ParentBuilder::new(group_id, artifact_id, version)
    }
}

/// Builder for `Parent`.
#[derive(Debug, Clone)]
pub struct ParentBuilder {
    parent: Parent,
}

impl ParentBuilder {
    /// Creates a new builder with the required fields.
    #[must_use]
    pub fn new(
        group_id: impl Into<String>,
        artifact_id: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            parent: Parent::new(group_id, artifact_id, version),
        }
    }

    /// Sets the relative path.
    #[must_use]
    pub fn relative_path(mut self, path: impl Into<String>) -> Self {
        self.parent.relative_path = Some(path.into());
        self
    }

    /// Builds the `Parent`.
    #[must_use]
    pub fn build(self) -> Parent {
        self.parent
    }
}

/// Represents an organization.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    /// The name of the organization.
    pub name: String,
    /// The URL of the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Organization {
    /// Creates a new `Organization` with the given name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: None,
        }
    }

    /// Creates a builder for `Organization`.
    #[must_use]
    pub fn builder(name: impl Into<String>) -> OrganizationBuilder {
        OrganizationBuilder::new(name)
    }
}

/// Builder for `Organization`.
#[derive(Debug, Clone)]
pub struct OrganizationBuilder {
    org: Organization,
}

impl OrganizationBuilder {
    /// Creates a new builder with the required name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            org: Organization::new(name),
        }
    }

    /// Sets the URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.org.url = Some(url.into());
        self
    }

    /// Builds the `Organization`.
    #[must_use]
    pub fn build(self) -> Organization {
        self.org
    }
}

/// Represents the prerequisites for a project.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Prerequisites {
    /// The Maven version required.
    pub maven: String,
}

impl Prerequisites {
    /// Creates a new `Prerequisites` with the required Maven version.
    #[must_use]
    pub fn new(maven: impl Into<String>) -> Self {
        Self {
            maven: maven.into(),
        }
    }
}

/// Represents a list of mailing lists.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct MailingLists {
    /// The mailing lists.
    #[serde(rename = "mailingList", default)]
    pub mailing_lists: Vec<MailingList>,
}

impl MailingLists {
    /// Creates an empty `MailingLists`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `MailingLists` from a vector of mailing lists.
    #[must_use]
    pub fn from_vec(mailing_lists: Vec<MailingList>) -> Self {
        Self { mailing_lists }
    }

    /// Adds a mailing list.
    pub fn add(&mut self, mailing_list: MailingList) {
        self.mailing_lists.push(mailing_list);
    }
}

/// Represents a mailing list.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MailingList {
    /// The name of the mailing list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The subscription address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<String>,
    /// The unsubscription address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe: Option<String>,
    /// The posting address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<String>,
    /// The archive URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive: Option<String>,
    /// Other archive URLs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_archives: Vec<String>,
}

impl MailingList {
    /// Creates a builder for `MailingList`.
    #[must_use]
    pub fn builder() -> MailingListBuilder {
        MailingListBuilder::new()
    }
}

/// Builder for `MailingList`.
#[derive(Debug, Clone, Default)]
pub struct MailingListBuilder {
    mailing_list: MailingList,
}

impl MailingListBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.mailing_list.name = Some(name.into());
        self
    }

    /// Sets the subscribe address.
    #[must_use]
    pub fn subscribe(mut self, addr: impl Into<String>) -> Self {
        self.mailing_list.subscribe = Some(addr.into());
        self
    }

    /// Sets the unsubscribe address.
    #[must_use]
    pub fn unsubscribe(mut self, addr: impl Into<String>) -> Self {
        self.mailing_list.unsubscribe = Some(addr.into());
        self
    }

    /// Sets the post address.
    #[must_use]
    pub fn post(mut self, addr: impl Into<String>) -> Self {
        self.mailing_list.post = Some(addr.into());
        self
    }

    /// Sets the archive URL.
    #[must_use]
    pub fn archive(mut self, url: impl Into<String>) -> Self {
        self.mailing_list.archive = Some(url.into());
        self
    }

    /// Adds another archive URL.
    #[must_use]
    pub fn other_archive(mut self, url: impl Into<String>) -> Self {
        self.mailing_list.other_archives.push(url.into());
        self
    }

    /// Builds the `MailingList`.
    #[must_use]
    pub fn build(self) -> MailingList {
        self.mailing_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parent_builder() {
        let parent = Parent::builder("com.example", "parent-pom", "1.0.0")
            .relative_path("../parent/pom.xml")
            .build();

        assert_eq!(parent.group_id, "com.example");
        assert_eq!(parent.artifact_id, "parent-pom");
        assert_eq!(parent.version, "1.0.0");
        assert_eq!(parent.relative_path, Some("../parent/pom.xml".to_string()));
    }

    #[test]
    fn test_organization_builder() {
        let org = Organization::builder("Example Corp")
            .url("https://example.com")
            .build();

        assert_eq!(org.name, "Example Corp");
        assert_eq!(org.url, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_prerequisites() {
        let prereqs = Prerequisites::new("3.6.0");
        assert_eq!(prereqs.maven, "3.6.0");
    }

    #[test]
    fn test_mailing_list_builder() {
        let ml = MailingList::builder()
            .name("Users")
            .subscribe("users-subscribe@example.com")
            .unsubscribe("users-unsubscribe@example.com")
            .archive("https://lists.example.com/users")
            .build();

        assert_eq!(ml.name, Some("Users".to_string()));
        assert!(ml.subscribe.is_some());
        assert!(ml.archive.is_some());
    }
}
