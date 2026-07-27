//! SCM (Source Control Management) and CI types.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Represents the SCM information.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Scm {
    /// Whether inherited SCM connection URLs append the child's path.
    #[serde(
        rename = "@child.scm.connection.inherit.append.path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub child_scm_connection_inherit_append_path: Option<String>,
    /// Whether inherited developer connection URLs append the child's path.
    #[serde(
        rename = "@child.scm.developerConnection.inherit.append.path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub child_scm_developer_connection_inherit_append_path: Option<String>,
    /// Whether inherited browsable SCM URLs append the child's path.
    #[serde(
        rename = "@child.scm.url.inherit.append.path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub child_scm_url_inherit_append_path: Option<String>,
    /// The connection URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
    /// The developer connection URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_connection: Option<String>,
    /// The tag name.
    #[serde(default = "default_scm_tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// The URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Default for Scm {
    fn default() -> Self {
        Self {
            child_scm_connection_inherit_append_path: None,
            child_scm_developer_connection_inherit_append_path: None,
            child_scm_url_inherit_append_path: None,
            connection: None,
            developer_connection: None,
            tag: default_scm_tag(),
            url: None,
        }
    }
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "serde default function must return the optional field type"
)]
fn default_scm_tag() -> Option<String> {
    Some("HEAD".to_string())
}

impl Scm {
    /// Creates a builder for `Scm`.
    #[must_use]
    pub fn builder() -> ScmBuilder {
        ScmBuilder::new()
    }

    /// Creates an `Scm` for a GitHub repository.
    #[must_use]
    pub fn github(owner: &str, repo: &str) -> Self {
        Self {
            child_scm_connection_inherit_append_path: None,
            child_scm_developer_connection_inherit_append_path: None,
            child_scm_url_inherit_append_path: None,
            connection: Some(format!("scm:git:git://github.com/{owner}/{repo}.git")),
            developer_connection: Some(format!("scm:git:ssh://github.com/{owner}/{repo}.git")),
            tag: Some("HEAD".to_string()),
            url: Some(format!("https://github.com/{owner}/{repo}")),
        }
    }
}

/// Builder for `Scm`.
#[derive(Debug, Clone, Default)]
pub struct ScmBuilder {
    scm: Scm,
}

impl ScmBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Controls path appending for inherited SCM connection URLs.
    #[must_use]
    pub fn child_connection_append_path(mut self, append: bool) -> Self {
        self.scm.child_scm_connection_inherit_append_path = Some(append.to_string());
        self
    }

    /// Controls path appending for inherited developer connection URLs.
    #[must_use]
    pub fn child_developer_connection_append_path(mut self, append: bool) -> Self {
        self.scm.child_scm_developer_connection_inherit_append_path = Some(append.to_string());
        self
    }

    /// Controls path appending for inherited browsable SCM URLs.
    #[must_use]
    pub fn child_url_append_path(mut self, append: bool) -> Self {
        self.scm.child_scm_url_inherit_append_path = Some(append.to_string());
        self
    }

    /// Sets the connection URL.
    #[must_use]
    pub fn connection(mut self, connection: impl Into<String>) -> Self {
        self.scm.connection = Some(connection.into());
        self
    }

    /// Sets the developer connection URL.
    #[must_use]
    pub fn developer_connection(mut self, connection: impl Into<String>) -> Self {
        self.scm.developer_connection = Some(connection.into());
        self
    }

    /// Sets the tag.
    #[must_use]
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.scm.tag = Some(tag.into());
        self
    }

    /// Sets the URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.scm.url = Some(url.into());
        self
    }

    /// Builds the `Scm`.
    #[must_use]
    pub fn build(self) -> Scm {
        self.scm
    }
}

/// Represents the issue management information.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IssueManagement {
    /// The issue management system.
    #[serde(default)]
    pub system: String,
    /// The URL.
    #[serde(default)]
    pub url: String,
}

impl IssueManagement {
    /// Creates a new `IssueManagement`.
    #[must_use]
    pub fn new(system: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            system: system.into(),
            url: url.into(),
        }
    }
}

/// Represents the CI management information.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CiManagement {
    /// The CI system.
    #[serde(default)]
    pub system: String,
    /// The URL.
    #[serde(default)]
    pub url: String,
    /// The notifiers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifiers: Option<Notifiers>,
}

impl CiManagement {
    /// Creates a new `CiManagement`.
    #[must_use]
    pub fn new(system: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            system: system.into(),
            url: url.into(),
            notifiers: None,
        }
    }

    /// Creates a builder for `CiManagement`.
    #[must_use]
    pub fn builder(system: impl Into<String>, url: impl Into<String>) -> CiManagementBuilder {
        CiManagementBuilder::new(system, url)
    }
}

/// Builder for `CiManagement`.
#[derive(Debug, Clone)]
pub struct CiManagementBuilder {
    ci: CiManagement,
}

impl CiManagementBuilder {
    /// Creates a new builder with the required fields.
    #[must_use]
    pub fn new(system: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            ci: CiManagement::new(system, url),
        }
    }

    /// Adds a notifier.
    #[must_use]
    pub fn notifier(mut self, notifier: Notifier) -> Self {
        if self.ci.notifiers.is_none() {
            self.ci.notifiers = Some(Notifiers::default());
        }
        if let Some(ref mut notifiers) = self.ci.notifiers {
            notifiers.notifiers.push(notifier);
        }
        self
    }

    /// Builds the `CiManagement`.
    #[must_use]
    pub fn build(self) -> CiManagement {
        self.ci
    }
}

/// Represents a list of notifiers.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Notifiers {
    /// The notifiers.
    #[serde(rename = "notifier", default)]
    pub notifiers: Vec<Notifier>,
}

/// Represents a notifier.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[expect(
    clippy::struct_excessive_bools,
    reason = "POM XSD defines 4 boolean flags for Notifier."
)]
pub struct Notifier {
    /// The type of notifier.
    #[serde(rename = "type", default = "default_notifier_type")]
    pub r#type: String,
    /// Whether to send on error.
    #[serde(default = "notification_enabled_by_default")]
    pub send_on_error: bool,
    /// Whether to send on failure.
    #[serde(default = "notification_enabled_by_default")]
    pub send_on_failure: bool,
    /// Whether to send on success.
    #[serde(default = "notification_enabled_by_default")]
    pub send_on_success: bool,
    /// Whether to send on warning.
    #[serde(default = "notification_enabled_by_default")]
    pub send_on_warning: bool,
    /// The address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Configuration defaults.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub configuration: BTreeMap<String, String>,
}

impl Notifier {
    /// Creates a new mail notifier.
    #[must_use]
    pub fn mail(address: impl Into<String>) -> Self {
        Self {
            r#type: "mail".to_string(),
            send_on_error: true,
            send_on_failure: true,
            send_on_success: true,
            send_on_warning: true,
            address: Some(address.into()),
            configuration: BTreeMap::new(),
        }
    }

    /// Creates a builder for `Notifier`.
    #[must_use]
    pub fn builder(r#type: impl Into<String>) -> NotifierBuilder {
        NotifierBuilder::new(r#type)
    }
}

impl Default for Notifier {
    fn default() -> Self {
        Self {
            r#type: "mail".to_string(),
            send_on_error: true,
            send_on_failure: true,
            send_on_success: true,
            send_on_warning: true,
            address: None,
            configuration: BTreeMap::new(),
        }
    }
}

const fn notification_enabled_by_default() -> bool {
    true
}

fn default_notifier_type() -> String {
    "mail".to_string()
}

/// Builder for `Notifier`.
#[derive(Debug, Clone)]
pub struct NotifierBuilder {
    notifier: Notifier,
}

impl NotifierBuilder {
    /// Creates a new builder with the required type.
    #[must_use]
    pub fn new(r#type: impl Into<String>) -> Self {
        Self {
            notifier: Notifier {
                r#type: r#type.into(),
                send_on_error: true,
                send_on_failure: true,
                send_on_success: true,
                send_on_warning: true,
                address: None,
                configuration: BTreeMap::new(),
            },
        }
    }

    /// Sets the address.
    #[must_use]
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.notifier.address = Some(address.into());
        self
    }

    /// Sets whether to send on error.
    #[must_use]
    pub fn send_on_error(mut self, value: bool) -> Self {
        self.notifier.send_on_error = value;
        self
    }

    /// Sets whether to send on failure.
    #[must_use]
    pub fn send_on_failure(mut self, value: bool) -> Self {
        self.notifier.send_on_failure = value;
        self
    }

    /// Sets whether to send on success.
    #[must_use]
    pub fn send_on_success(mut self, value: bool) -> Self {
        self.notifier.send_on_success = value;
        self
    }

    /// Sets whether to send on warning.
    #[must_use]
    pub fn send_on_warning(mut self, value: bool) -> Self {
        self.notifier.send_on_warning = value;
        self
    }

    /// Adds a configuration entry.
    #[must_use]
    pub fn config(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.notifier.configuration.insert(key.into(), value.into());
        self
    }

    /// Builds the `Notifier`.
    #[must_use]
    pub fn build(self) -> Notifier {
        self.notifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scm_github() {
        let scm = Scm::github("example", "repo");
        assert!(scm.connection.as_ref().unwrap().contains("github.com"));
        assert!(scm.url.as_ref().unwrap().contains("example/repo"));
    }

    #[test]
    fn test_scm_builder() {
        let scm = Scm::builder()
            .connection("scm:git:git@gitlab.com:org/repo.git")
            .url("https://gitlab.com/org/repo")
            .build();

        assert!(scm.connection.is_some());
        assert!(scm.url.is_some());
    }

    #[test]
    fn test_notifier_builder() {
        let notifier = Notifier::builder("mail")
            .address("team@example.com")
            .send_on_failure(true)
            .send_on_error(true)
            .build();

        assert_eq!(notifier.r#type, "mail");
        assert!(notifier.send_on_failure);
        assert!(notifier.send_on_error);
    }
}
