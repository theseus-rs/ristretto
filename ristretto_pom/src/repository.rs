//! Repository-related types.

use crate::types::{ChecksumPolicy, RepositoryLayout, UpdatePolicy};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;

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
    /// Whether deployment snapshots use timestamped unique versions.
    ///
    /// This element is used by distribution-management repositories and is
    /// absent for ordinary download repositories. Maven defaults it to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_version: Option<bool>,
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
            unique_version: None,
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

    /// Sets whether deployment snapshots use timestamped unique versions.
    #[must_use]
    pub fn unique_version(mut self, unique: bool) -> Self {
        self.repository.unique_version = Some(unique);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryPolicy {
    /// Whether the policy is enabled.
    pub enabled: bool,
    /// The update policy.
    pub update_policy: Option<UpdatePolicy>,
    /// The checksum policy.
    pub checksum_policy: Option<ChecksumPolicy>,
    /// Original `enabled` expression, when it was explicitly present.
    enabled_expression: Option<String>,
    /// Original `updatePolicy` expression, when it was explicitly present.
    update_policy_expression: Option<String>,
    /// Original `checksumPolicy` expression, when it was explicitly present.
    checksum_policy_expression: Option<String>,
}

impl Default for RepositoryPolicy {
    fn default() -> Self {
        Self::enabled()
    }
}

impl RepositoryPolicy {
    /// Creates a new `RepositoryPolicy` with enabled set to true.
    #[must_use]
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            update_policy: None,
            checksum_policy: None,
            enabled_expression: None,
            update_policy_expression: None,
            checksum_policy_expression: None,
        }
    }

    /// Creates a new `RepositoryPolicy` with enabled set to false.
    #[must_use]
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            update_policy: None,
            checksum_policy: None,
            enabled_expression: None,
            update_policy_expression: None,
            checksum_policy_expression: None,
        }
    }

    /// Creates a builder for `RepositoryPolicy`.
    #[must_use]
    pub fn builder() -> RepositoryPolicyBuilder {
        RepositoryPolicyBuilder::new()
    }

    /// Returns the original explicit `enabled` value.
    ///
    /// This can contain a Maven property expression such as `${releases.enabled}`.
    #[must_use]
    pub fn enabled_expression(&self) -> Option<&str> {
        self.enabled_expression.as_deref()
    }

    /// Returns the original explicit `updatePolicy` value.
    ///
    /// This can contain a Maven property expression.
    #[must_use]
    pub fn update_policy_expression(&self) -> Option<&str> {
        self.update_policy_expression.as_deref()
    }

    /// Returns the original explicit `checksumPolicy` value.
    ///
    /// This can contain a Maven property expression.
    #[must_use]
    pub fn checksum_policy_expression(&self) -> Option<&str> {
        self.checksum_policy_expression.as_deref()
    }
}

/// Builder for `RepositoryPolicy`.
#[derive(Debug, Clone)]
pub struct RepositoryPolicyBuilder {
    enabled: bool,
    enabled_expression: Option<String>,
    update_policy: Option<UpdatePolicy>,
    checksum_policy: Option<ChecksumPolicy>,
    update_policy_expression: Option<String>,
    checksum_policy_expression: Option<String>,
}

impl Default for RepositoryPolicyBuilder {
    fn default() -> Self {
        Self {
            enabled: true,
            enabled_expression: None,
            update_policy: None,
            checksum_policy: None,
            update_policy_expression: None,
            checksum_policy_expression: None,
        }
    }
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
        self.enabled_expression = Some(enabled.to_string());
        self
    }

    /// Sets an interpolatable Maven `enabled` expression.
    #[must_use]
    pub fn enabled_expression(mut self, expression: impl Into<String>) -> Self {
        let expression = expression.into();
        self.enabled = parse_maven_boolean(&expression);
        self.enabled_expression = Some(expression);
        self
    }

    /// Sets the update policy.
    #[must_use]
    pub fn update_policy(mut self, policy: UpdatePolicy) -> Self {
        self.update_policy_expression = Some(policy.to_string());
        self.update_policy = Some(policy);
        self
    }

    /// Sets the checksum policy.
    #[must_use]
    pub fn checksum_policy(mut self, policy: ChecksumPolicy) -> Self {
        self.checksum_policy_expression = Some(policy.to_string());
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
            enabled_expression: self.enabled_expression,
            update_policy_expression: self.update_policy_expression,
            checksum_policy_expression: self.checksum_policy_expression,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RepositoryPolicyXml {
    enabled: Option<String>,
    update_policy: Option<String>,
    checksum_policy: Option<String>,
}

impl<'de> Deserialize<'de> for RepositoryPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let policy = RepositoryPolicyXml::deserialize(deserializer)?;
        let update_policy = policy
            .update_policy
            .as_deref()
            .map(parse_update_policy)
            .transpose()
            .map_err(serde::de::Error::custom)?
            .flatten();
        let checksum_policy = policy
            .checksum_policy
            .as_deref()
            .map(parse_checksum_policy)
            .transpose()
            .map_err(serde::de::Error::custom)?
            .flatten();
        Ok(Self {
            enabled: policy.enabled.as_deref().is_none_or(parse_maven_boolean),
            enabled_expression: policy.enabled,
            update_policy,
            checksum_policy,
            update_policy_expression: policy.update_policy,
            checksum_policy_expression: policy.checksum_policy,
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RepositoryPolicyXmlRef<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_policy: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    checksum_policy: Option<Cow<'a, str>>,
}

impl Serialize for RepositoryPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RepositoryPolicyXmlRef {
            enabled: self
                .enabled_expression
                .as_deref()
                .map(Cow::Borrowed)
                .or_else(|| (!self.enabled).then_some(Cow::Borrowed("false"))),
            update_policy: self.update_policy_expression.as_deref().map_or_else(
                || {
                    self.update_policy
                        .as_ref()
                        .map(|policy| Cow::Owned(policy.to_string()))
                },
                |expression| Some(Cow::Borrowed(expression)),
            ),
            checksum_policy: self.checksum_policy_expression.as_deref().map_or_else(
                || {
                    self.checksum_policy
                        .as_ref()
                        .map(|policy| Cow::Owned(policy.to_string()))
                },
                |expression| Some(Cow::Borrowed(expression)),
            ),
        }
        .serialize(serializer)
    }
}

fn parse_maven_boolean(value: &str) -> bool {
    value.trim().eq_ignore_ascii_case("true")
}

fn parse_update_policy(value: &str) -> Result<Option<UpdatePolicy>, String> {
    let normalized = value.to_ascii_lowercase();
    match normalized.as_str() {
        "always" => Ok(Some(UpdatePolicy::Always)),
        "daily" => Ok(Some(UpdatePolicy::Daily)),
        "never" => Ok(Some(UpdatePolicy::Never)),
        _ if value.contains("${") => Ok(None),
        _ => normalized
            .strip_prefix("interval:")
            .and_then(|minutes| minutes.parse::<u64>().ok())
            .map(|minutes| Some(UpdatePolicy::Interval(format!("interval:{minutes}"))))
            .ok_or_else(|| format!("invalid repository update policy '{value}'")),
    }
}

fn parse_checksum_policy(value: &str) -> Result<Option<ChecksumPolicy>, String> {
    match value.to_ascii_lowercase().as_str() {
        "fail" => Ok(Some(ChecksumPolicy::Fail)),
        "ignore" => Ok(Some(ChecksumPolicy::Ignore)),
        "warn" => Ok(Some(ChecksumPolicy::Warn)),
        _ if value.contains("${") => Ok(None),
        _ => Err(format!("invalid repository checksum policy '{value}'")),
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

    #[test]
    fn repository_policy_defaults_to_enabled() {
        assert!(RepositoryPolicy::default().enabled);
        assert!(RepositoryPolicy::builder().build().enabled);
    }

    #[test]
    fn covers_typed_policy_parsing_and_fallback_serialization() {
        for (xml, update, checksum) in [
            (
                "<releases><updatePolicy>never</updatePolicy><checksumPolicy>ignore</checksumPolicy></releases>",
                UpdatePolicy::Never,
                ChecksumPolicy::Ignore,
            ),
            (
                "<releases><updatePolicy>interval:15</updatePolicy><checksumPolicy>warn</checksumPolicy></releases>",
                UpdatePolicy::Interval("interval:15".to_string()),
                ChecksumPolicy::Warn,
            ),
        ] {
            let policy: RepositoryPolicy = quick_xml::de::from_str(xml).expect("repository policy");
            assert_eq!(policy.update_policy, Some(update));
            assert_eq!(policy.checksum_policy, Some(checksum));
        }

        let policy = RepositoryPolicy {
            enabled: false,
            update_policy: Some(UpdatePolicy::Never),
            checksum_policy: Some(ChecksumPolicy::Ignore),
            enabled_expression: None,
            update_policy_expression: None,
            checksum_policy_expression: None,
        };
        let xml = quick_xml::se::to_string(&policy).expect("repository policy XML");
        assert!(xml.contains("<enabled>false</enabled>"));
        assert!(xml.contains("<updatePolicy>never</updatePolicy>"));
        assert!(xml.contains("<checksumPolicy>ignore</checksumPolicy>"));
    }
}
