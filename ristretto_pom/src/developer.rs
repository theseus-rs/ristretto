//! Developer and contributor types.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Represents a list of developers.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Developers {
    /// The developers.
    #[serde(rename = "developer", default)]
    pub developers: Vec<Developer>,
}

impl Developers {
    /// Creates an empty `Developers`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `Developers` from a vector of developers.
    #[must_use]
    pub fn from_vec(developers: Vec<Developer>) -> Self {
        Self { developers }
    }

    /// Adds a developer.
    pub fn add(&mut self, developer: Developer) {
        self.developers.push(developer);
    }
}

/// Represents a list of roles.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Roles {
    /// The roles.
    #[serde(rename = "role", default)]
    pub roles: Vec<String>,
}

impl Roles {
    /// Creates a `Roles` from a vector of role strings.
    #[must_use]
    pub fn from_vec(roles: Vec<String>) -> Self {
        Self { roles }
    }
}

/// Represents a developer.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Developer {
    /// The ID of the developer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the developer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The email of the developer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The URL of the developer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The organization of the developer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// The URL of the organization of the developer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_url: Option<String>,
    /// The roles of the developer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Roles>,
    /// The timezone of the developer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// The properties of the developer.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<String, String>,
}

impl Developer {
    /// Creates a builder for `Developer`.
    #[must_use]
    pub fn builder() -> DeveloperBuilder {
        DeveloperBuilder::new()
    }
}

/// Builder for `Developer`.
#[derive(Debug, Clone, Default)]
pub struct DeveloperBuilder {
    developer: Developer,
}

impl DeveloperBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the ID.
    #[must_use]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.developer.id = Some(id.into());
        self
    }

    /// Sets the name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.developer.name = Some(name.into());
        self
    }

    /// Sets the email.
    #[must_use]
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.developer.email = Some(email.into());
        self
    }

    /// Sets the URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.developer.url = Some(url.into());
        self
    }

    /// Sets the organization.
    #[must_use]
    pub fn organization(mut self, org: impl Into<String>) -> Self {
        self.developer.organization = Some(org.into());
        self
    }

    /// Sets the organization URL.
    #[must_use]
    pub fn organization_url(mut self, url: impl Into<String>) -> Self {
        self.developer.organization_url = Some(url.into());
        self
    }

    /// Sets the timezone.
    #[must_use]
    pub fn timezone(mut self, tz: impl Into<String>) -> Self {
        self.developer.timezone = Some(tz.into());
        self
    }

    /// Adds a role.
    #[must_use]
    pub fn role(mut self, role: impl Into<String>) -> Self {
        if self.developer.roles.is_none() {
            self.developer.roles = Some(Roles::default());
        }
        if let Some(ref mut roles) = self.developer.roles {
            roles.roles.push(role.into());
        }
        self
    }

    /// Adds a property.
    #[must_use]
    pub fn property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.developer.properties.insert(key.into(), value.into());
        self
    }

    /// Builds the `Developer`.
    #[must_use]
    pub fn build(self) -> Developer {
        self.developer
    }
}

/// Represents a list of contributors.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Contributors {
    /// The contributors.
    #[serde(rename = "contributor", default)]
    pub contributors: Vec<Contributor>,
}

impl Contributors {
    /// Creates an empty `Contributors`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `Contributors` from a vector of contributors.
    #[must_use]
    pub fn from_vec(contributors: Vec<Contributor>) -> Self {
        Self { contributors }
    }

    /// Adds a contributor.
    pub fn add(&mut self, contributor: Contributor) {
        self.contributors.push(contributor);
    }
}

/// Represents a contributor.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
    /// The name of the contributor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The email of the contributor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The URL of the contributor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The organization of the contributor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// The URL of the organization of the contributor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_url: Option<String>,
    /// The roles of the contributor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Roles>,
    /// The timezone of the contributor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// The properties of the contributor.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<String, String>,
}

impl Contributor {
    /// Creates a builder for `Contributor`.
    #[must_use]
    pub fn builder() -> ContributorBuilder {
        ContributorBuilder::new()
    }
}

/// Builder for `Contributor`.
#[derive(Debug, Clone, Default)]
pub struct ContributorBuilder {
    contributor: Contributor,
}

impl ContributorBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.contributor.name = Some(name.into());
        self
    }

    /// Sets the email.
    #[must_use]
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.contributor.email = Some(email.into());
        self
    }

    /// Sets the URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.contributor.url = Some(url.into());
        self
    }

    /// Sets the organization.
    #[must_use]
    pub fn organization(mut self, org: impl Into<String>) -> Self {
        self.contributor.organization = Some(org.into());
        self
    }

    /// Adds a role.
    #[must_use]
    pub fn role(mut self, role: impl Into<String>) -> Self {
        if self.contributor.roles.is_none() {
            self.contributor.roles = Some(Roles::default());
        }
        if let Some(ref mut roles) = self.contributor.roles {
            roles.roles.push(role.into());
        }
        self
    }

    /// Builds the `Contributor`.
    #[must_use]
    pub fn build(self) -> Contributor {
        self.contributor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_developer_builder() {
        let dev = Developer::builder()
            .id("john")
            .name("John Doe")
            .email("john@example.com")
            .organization("Example Corp")
            .role("Lead Developer")
            .role("Architect")
            .build();

        assert_eq!(dev.id, Some("john".to_string()));
        assert_eq!(dev.name, Some("John Doe".to_string()));
        assert!(dev.roles.is_some());
        assert_eq!(dev.roles.as_ref().unwrap().roles.len(), 2);
    }

    #[test]
    fn test_contributor_builder() {
        let contrib = Contributor::builder()
            .name("John Doe")
            .email("jane@example.com")
            .url("https://github.com/john/doe")
            .role("Tester")
            .build();

        assert_eq!(contrib.name, Some("John Doe".to_string()));
        assert!(contrib.roles.is_some());
    }
}
