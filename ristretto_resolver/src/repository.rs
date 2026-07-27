//! Maven repository, mirror, authentication, proxy, and update policies.
//!
//! [`RemoteRepository`] describes an HTTP(S), `file:`, or custom-transport repository and keeps
//! separate release and snapshot policies. Repository and resource URLs are normalized and
//! containment-checked before use. [`Mirror`] implements Maven-style repository selection, while
//! [`Authentication`], [`Proxy`], and [`SecretString`] keep request configuration explicit and
//! redact credentials from debug output.

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::time::{Duration, SystemTime};
use url::Url;

/// Default repository identifier.
pub const CENTRAL_REPOSITORY_ID: &str = "central";
/// Default repository base URL.
pub const CENTRAL_REPOSITORY_URL: &str = "https://repo.maven.apache.org/maven2/";

/// Handling policy for absent or invalid checksums.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum ChecksumPolicy {
    /// Warn if no checksum is published, but always fail on mismatches.
    #[default]
    Warn,
    /// Require a supported checksum and fail on mismatches.
    Fail,
    /// Do not request or verify checksums.
    Ignore,
}

/// Policy controlling when mutable repository resources are refreshed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum UpdatePolicy {
    /// Check on every resolver session.
    Always,
    /// Check once per UTC day.
    #[default]
    Daily,
    /// Never refresh an existing cache entry.
    Never,
    /// Refresh after this number of minutes.
    IntervalMinutes(u64),
}

impl UpdatePolicy {
    pub(crate) fn permits_refresh(self, modified: SystemTime, now: SystemTime) -> bool {
        let age = now.duration_since(modified).unwrap_or_default();
        match self {
            Self::Always => true,
            Self::Daily => utc_day(modified) < utc_day(now),
            Self::Never => false,
            Self::IntervalMinutes(minutes) => {
                age >= Duration::from_secs(minutes.saturating_mul(60))
            }
        }
    }
}

fn utc_day(time: SystemTime) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        / Duration::from_hours(24).as_secs()
}

/// Release or snapshot policy for one repository.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct RepositoryPolicy {
    /// Whether this category of artifact may be read.
    pub enabled: bool,
    /// Metadata and snapshot refresh policy.
    pub update: UpdatePolicy,
    /// Checksum validation policy.
    pub checksum: ChecksumPolicy,
}

impl RepositoryPolicy {
    /// Creates an enabled policy with compatible defaults.
    #[must_use]
    pub const fn enabled() -> Self {
        Self {
            enabled: true,
            update: UpdatePolicy::Daily,
            checksum: ChecksumPolicy::Warn,
        }
    }

    /// Creates a disabled repository policy.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            enabled: false,
            update: UpdatePolicy::Never,
            checksum: ChecksumPolicy::Warn,
        }
    }
}

impl Default for RepositoryPolicy {
    fn default() -> Self {
        Self::enabled()
    }
}

/// A secret whose debug output is always redacted.
#[derive(Clone, PartialEq, Eq)]
pub struct SecretString(String);

impl SecretString {
    /// Wraps sensitive text.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Exposes the secret for constructing an authenticated request.
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

/// HTTP authentication attached to a repository.
#[derive(Clone, PartialEq, Eq)]
pub enum Authentication {
    /// HTTP basic authentication.
    Basic {
        /// Login name.
        username: String,
        /// Login password.
        password: SecretString,
    },
    /// HTTP bearer token authentication.
    Bearer(SecretString),
    /// Custom sensitive request headers.
    Headers(BTreeMap<String, SecretString>),
}

impl fmt::Debug for Authentication {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Basic { username, .. } => formatter
                .debug_struct("Basic")
                .field("username", username)
                .field("password", &"[REDACTED]")
                .finish(),
            Self::Bearer(_) => formatter
                .debug_tuple("Bearer")
                .field(&"[REDACTED]")
                .finish(),
            Self::Headers(headers) => formatter
                .debug_tuple("Headers")
                .field(&headers.keys().collect::<Vec<_>>())
                .finish(),
        }
    }
}

/// HTTP proxy configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proxy {
    /// Proxy URI.
    pub url: String,
    /// Optional proxy authentication.
    pub authentication: Option<Authentication>,
    /// Host globs that bypass this proxy.
    pub non_proxy_hosts: Vec<String>,
}

impl Proxy {
    /// Creates a proxy configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is malformed.
    pub fn new(url: impl Into<String>) -> Result<Self> {
        let url = url.into();
        let parsed = Url::parse(&url)?;
        reject_url_credentials(&parsed, "proxy")?;
        if parsed.cannot_be_a_base()
            || !matches!(parsed.scheme(), "http" | "https")
            || parsed.host_str().is_none()
            || parsed.query().is_some()
            || parsed.fragment().is_some()
        {
            return Err(Error::InvalidConfiguration(format!(
                "proxy URL must be an HTTP(S) URL without a query or fragment: '{url}'"
            )));
        }
        Ok(Self {
            url,
            authentication: None,
            non_proxy_hosts: Vec::new(),
        })
    }
}

/// A remote dependency repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteRepository {
    /// Repository identifier used by mirrors, provenance, and credentials.
    pub id: String,
    /// Base repository URL.
    pub url: String,
    /// Release artifact policy.
    pub releases: RepositoryPolicy,
    /// Snapshot artifact policy.
    pub snapshots: RepositoryPolicy,
    /// Optional request authentication.
    pub authentication: Option<Authentication>,
    /// Optional HTTP proxy.
    pub proxy: Option<Proxy>,
}

impl RemoteRepository {
    /// Creates an enabled repository.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty/unsafe identifier or malformed URL.
    pub fn new(id: impl Into<String>, url: impl Into<String>) -> Result<Self> {
        let id = id.into();
        if id.is_empty()
            || id.contains(['/', '\\'])
            || id
                .chars()
                .any(|character| character.is_whitespace() || character.is_control())
        {
            return Err(Error::InvalidConfiguration(format!(
                "invalid repository id '{id}'"
            )));
        }
        let url = normalize_url(&url.into())?;
        Ok(Self {
            id,
            url,
            releases: RepositoryPolicy::enabled(),
            snapshots: RepositoryPolicy::enabled(),
            authentication: None,
            proxy: None,
        })
    }

    /// Returns the default central repository.
    #[must_use]
    pub fn central() -> Self {
        Self {
            id: CENTRAL_REPOSITORY_ID.to_string(),
            url: CENTRAL_REPOSITORY_URL.to_string(),
            releases: RepositoryPolicy::enabled(),
            snapshots: RepositoryPolicy::disabled(),
            authentication: None,
            proxy: None,
        }
    }

    /// Revalidates this repository after any direct field mutation.
    ///
    /// # Errors
    ///
    /// Returns an error if the identifier, normalized base URL, or proxy
    /// configuration is invalid.
    pub fn validate(&self) -> Result<()> {
        let validated = Self::new(&self.id, &self.url)?;
        if validated.url != self.url {
            return Err(Error::InvalidConfiguration(format!(
                "repository '{}' URL must be normalized as '{}'",
                self.id, validated.url
            )));
        }
        if let Some(proxy) = &self.proxy {
            drop(Proxy::new(&proxy.url)?);
            if proxy.authentication.is_some()
                && !matches!(
                    proxy.authentication.as_ref(),
                    Some(Authentication::Basic { .. })
                )
            {
                return Err(Error::InvalidConfiguration(
                    "proxy authentication only supports HTTP basic authentication".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Joins a repository-relative path to the base URL.
    ///
    /// # Errors
    ///
    /// Returns an error if the configured base URL is invalid or `path` is not a
    /// plain repository-relative path.
    pub fn resource_url(&self, path: &str) -> Result<String> {
        self.validate()?;
        if path.is_empty()
            || path.starts_with('/')
            || path.contains(['\\', '%', '?', '#'])
            || path
                .split('/')
                .any(|segment| segment.is_empty() || matches!(segment, "." | ".."))
        {
            return Err(Error::InvalidConfiguration(format!(
                "invalid repository resource path '{path}'"
            )));
        }
        let base = Url::parse(&self.url)?;
        let resource = base.join(path)?;
        let resource = resource.to_string();
        self.validate_resource_url(&resource)?;
        Ok(resource)
    }

    /// Verifies that a resource URL is contained by this repository base URL.
    ///
    /// # Errors
    ///
    /// Returns an error when the resource changes origin, escapes the base path, or
    /// includes URL components that are not part of a repository resource name.
    pub fn validate_resource_url(&self, value: &str) -> Result<()> {
        self.validate()?;
        let base = Url::parse(&self.url)?;
        let resource = Url::parse(value)?;
        let relative_path = resource.path().strip_prefix(base.path());
        if resource.scheme() != base.scheme()
            || resource.host_str() != base.host_str()
            || resource.port_or_known_default() != base.port_or_known_default()
            || resource.username() != base.username()
            || resource.password() != base.password()
            || resource.query().is_some()
            || resource.fragment().is_some()
            || relative_path.is_none()
            || relative_path.is_some_and(|path| {
                !path.is_empty()
                    && (path.contains('\\')
                        || has_unsafe_percent_encoding(path)
                        || path
                            .split('/')
                            .any(|segment| segment.is_empty() || matches!(segment, "." | "..")))
            })
        {
            return Err(Error::InvalidConfiguration(format!(
                "resource URL is outside repository '{}': '{value}'",
                self.id
            )));
        }
        Ok(())
    }

    /// Returns whether this repository accepts a version.
    #[must_use]
    pub fn accepts_version(&self, version: &str) -> bool {
        if version.ends_with("-SNAPSHOT") {
            self.snapshots.enabled
        } else {
            self.releases.enabled
        }
    }

    /// Returns the policy relevant to a version.
    #[must_use]
    pub fn policy_for(&self, version: &str) -> RepositoryPolicy {
        if version.ends_with("-SNAPSHOT") {
            self.snapshots
        } else {
            self.releases
        }
    }
}

/// A mirror replacement rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mirror {
    /// Mirror repository.
    pub repository: RemoteRepository,
    /// Comma-separated repository patterns such as `*`, `external:*`,
    /// `external:http:*`, or `*,!internal`.
    pub mirror_of: String,
}

impl Mirror {
    pub(crate) fn exactly_matches(&self, repository: &RemoteRepository) -> bool {
        self.mirror_of.trim() == repository.id
    }

    /// Returns whether this mirror replaces the supplied repository.
    #[must_use]
    pub fn matches(&self, repository: &RemoteRepository) -> bool {
        let mut included = false;
        for pattern in self.mirror_of.split(',').map(str::trim) {
            if let Some(excluded) = pattern.strip_prefix('!') {
                if excluded == repository.id {
                    return false;
                }
            } else if pattern == "*"
                || pattern == repository.id
                || pattern == "external:*" && is_external(&repository.url)
                || pattern == "external:http:*" && is_external_http(&repository.url)
            {
                included = true;
            }
        }
        included
    }
}

fn normalize_url(value: &str) -> Result<String> {
    let mut url = Url::parse(value)?;
    if url.cannot_be_a_base() || url.query().is_some() || url.fragment().is_some() {
        return Err(Error::InvalidConfiguration(format!(
            "repository URL must be a hierarchical URL without a query or fragment: '{value}'"
        )));
    }
    reject_url_credentials(&url, "repository")?;
    if !url.path().ends_with('/') {
        let path = format!("{}/", url.path());
        url.set_path(&path);
    }
    Ok(url.to_string())
}

fn reject_url_credentials(url: &Url, kind: &str) -> Result<()> {
    if !url.username().is_empty() || url.password().is_some() {
        return Err(Error::InvalidConfiguration(format!(
            "{kind} URL credentials must be configured through authentication"
        )));
    }
    Ok(())
}

fn has_unsafe_percent_encoding(path: &str) -> bool {
    let bytes = path.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes.get(index) != Some(&b'%') {
            index += 1;
            continue;
        }
        let Some(encoded) = bytes
            .get(index + 1..index + 3)
            .and_then(|digits| std::str::from_utf8(digits).ok())
            .and_then(|digits| u8::from_str_radix(digits, 16).ok())
        else {
            return true;
        };
        if encoded.is_ascii_control() || matches!(encoded, b'.' | b'/' | b'\\' | b'%' | b'?' | b'#')
        {
            return true;
        }
        index += 3;
    }
    false
}

fn is_external(value: &str) -> bool {
    Url::parse(value).is_ok_and(|url| {
        url.scheme() != "file"
            && !matches!(
                url.host_str(),
                Some("localhost" | "127.0.0.1" | "::1") | None
            )
    })
}

fn is_external_http(value: &str) -> bool {
    Url::parse(value).is_ok_and(|url| {
        url.scheme() == "http"
            && !matches!(
                url.host_str(),
                Some("localhost" | "127.0.0.1" | "::1") | None
            )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    fn normalizes_and_joins_repository_url() -> Result<()> {
        let repository = RemoteRepository::new("test", "https://example.com/repository")?;
        assert_eq!(
            repository.resource_url("org/example/demo/1.0/demo-1.0.pom")?,
            "https://example.com/repository/org/example/demo/1.0/demo-1.0.pom"
        );
        Ok(())
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    fn rejects_escaping_resource_paths() -> Result<()> {
        let repository = RemoteRepository::new("test", "file:///safe/repository/")?;
        for path in [
            "../secret",
            "%2e%2e/secret",
            "org/example%2f..%2fsecret",
            "/absolute",
            "https://example.com/escape",
            "artifact?query",
            "artifact#fragment",
        ] {
            assert!(repository.resource_url(path).is_err(), "{path}");
        }
        Ok(())
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    fn rejects_resource_urls_outside_repository() -> Result<()> {
        let repository = RemoteRepository::new("test", "https://example.com/repository/")?;
        for url in [
            "https://example.com/other/artifact.jar",
            "https://attacker.example/repository/artifact.jar",
            "https://example.com/repository/%2e%2e/secret",
            "https://example.com/repository/artifact.jar?token=leak",
        ] {
            assert!(repository.validate_resource_url(url).is_err(), "{url}");
        }
        let artifact_url = "https://example.com/repository/org/example/demo/1.0/demo-1.0.jar";
        repository.validate_resource_url(artifact_url)?;
        RemoteRepository::new("local", "file:///safe/repository/")?
            .validate_resource_url("file:///safe/repository/artifact%20name.jar")?;
        Ok(())
    }

    #[test]
    fn rejects_credentials_embedded_in_urls() {
        assert!(
            RemoteRepository::new("test", "https://user:secret@example.com/repository/").is_err()
        );
        assert!(Proxy::new("http://user:secret@proxy.example.com").is_err());

        let mut repository =
            RemoteRepository::new("test", "https://repo.example/base/").expect("repository");
        repository.url = "https://user:secret@repo.example/base/".to_string();
        assert!(repository.validate().is_err());
        repository.url = "https://repo.example/base".to_string();
        assert!(repository.validate().is_err());

        let mut repository =
            RemoteRepository::new("test", "https://repo.example/base/").expect("repository");
        let mut proxy = Proxy::new("http://proxy.example/").expect("proxy");
        proxy.authentication = Some(Authentication::Bearer(SecretString::new("unsupported")));
        repository.proxy = Some(proxy);
        assert!(repository.validate().is_err());
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    fn applies_mirror_patterns() -> Result<()> {
        let mirror = Mirror {
            repository: RemoteRepository::new("mirror", "https://mirror.example")?,
            mirror_of: "*,!internal".to_string(),
        };
        assert!(mirror.matches(&RemoteRepository::central()));
        let internal = RemoteRepository::new("internal", "https://internal.example")?;
        assert!(!mirror.matches(&internal));
        let insecure_only = Mirror {
            repository: RemoteRepository::new("secure", "https://mirror.example")?,
            mirror_of: "external:http:*".to_string(),
        };
        assert!(insecure_only.matches(&RemoteRepository::new("insecure", "http://repo.example")?));
        assert!(!insecure_only.matches(&RemoteRepository::new("secure", "https://repo.example")?));
        let local = RemoteRepository::new("local", "http://localhost/repository")?;
        assert!(!insecure_only.matches(&local));
        Ok(())
    }

    #[test]
    fn update_and_repository_policies_cover_release_and_snapshot_paths() {
        let midnight = SystemTime::UNIX_EPOCH + Duration::from_hours(480_000);
        assert!(UpdatePolicy::Always.permits_refresh(midnight, midnight));
        assert!(!UpdatePolicy::Never.permits_refresh(SystemTime::UNIX_EPOCH, midnight));
        assert!(!UpdatePolicy::Daily.permits_refresh(
            midnight + Duration::from_hours(1),
            midnight + Duration::from_hours(23)
        ));
        assert!(UpdatePolicy::Daily.permits_refresh(
            midnight + Duration::from_hours(23),
            midnight + Duration::from_hours(24)
        ));
        assert!(
            !UpdatePolicy::IntervalMinutes(5)
                .permits_refresh(midnight, midnight + Duration::from_secs(299))
        );
        assert!(
            UpdatePolicy::IntervalMinutes(5)
                .permits_refresh(midnight, midnight + Duration::from_mins(5))
        );
        assert!(
            !UpdatePolicy::IntervalMinutes(u64::MAX)
                .permits_refresh(SystemTime::UNIX_EPOCH, midnight)
        );

        let mut repository = RemoteRepository::central();
        assert_eq!(repository.id, CENTRAL_REPOSITORY_ID);
        assert_eq!(repository.url, CENTRAL_REPOSITORY_URL);
        assert!(repository.accepts_version("1.0"));
        assert!(!repository.accepts_version("1.0-SNAPSHOT"));
        assert_eq!(repository.policy_for("1.0"), repository.releases);
        assert_eq!(repository.policy_for("1.0-SNAPSHOT"), repository.snapshots);
        repository.snapshots = RepositoryPolicy::enabled();
        assert!(repository.accepts_version("1.0-SNAPSHOT"));
        assert_eq!(RepositoryPolicy::default(), RepositoryPolicy::enabled());
        assert!(!RepositoryPolicy::disabled().enabled);
    }

    #[test]
    fn secrets_and_authentication_debug_output_are_redacted() {
        let secret = SecretString::new("secret");
        assert_eq!(secret.expose(), "secret");
        assert_eq!(format!("{secret:?}"), "[REDACTED]");
        let variants = [
            Authentication::Basic {
                username: "user".to_string(),
                password: secret.clone(),
            },
            Authentication::Bearer(secret.clone()),
            Authentication::Headers(BTreeMap::from([("X-Token".to_string(), secret)])),
        ];
        for authentication in variants {
            let debug = format!("{authentication:?}");
            assert!(!debug.contains("secret"));
        }
    }

    #[test]
    fn rejects_invalid_repository_and_proxy_configuration() {
        for id in ["", "with space", "with/slash", "with\\slash", "line\nbreak"] {
            assert!(RemoteRepository::new(id, "https://repo.example").is_err());
        }
        for url in [
            "mailto:repository@example.com",
            "https://repo.example/path?query",
            "https://repo.example/path#fragment",
        ] {
            assert!(RemoteRepository::new("repo", url).is_err());
        }
        for url in [
            "socks5://proxy.example",
            "http://proxy.example?query",
            "http://proxy.example#fragment",
        ] {
            assert!(Proxy::new(url).is_err(), "{url}");
        }
        assert!(Proxy::new("https://proxy.example").is_ok());
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    fn validates_every_resource_url_component_and_mirror_kind() -> Result<()> {
        let repository = RemoteRepository::new("repo", "https://example.com:8443/base/")?;
        for url in [
            "http://example.com:8443/base/file",
            "https://example.com/base/file",
            "https://example.com:8443/base/a//b",
            "https://example.com:8443/base/%",
            "https://example.com:8443/base/%00",
            "https://example.com:8443/base/a%5Cb",
            "https://example.com:8443/base/file#fragment",
        ] {
            assert!(repository.validate_resource_url(url).is_err(), "{url}");
        }
        repository.validate_resource_url("https://example.com:8443/base/")?;
        assert!(
            RemoteRepository::new("repo", "file:///base/")?
                .validate_resource_url("file:///base/%41rtifact.jar")
                .is_ok()
        );

        let external = Mirror {
            repository: RemoteRepository::central(),
            mirror_of: "external:*".to_string(),
        };
        assert!(external.matches(&RemoteRepository::new("remote", "https://repo.example")?));
        assert!(!external.matches(&RemoteRepository::new("file", "file:///repo")?));
        let loopback = RemoteRepository::new("loopback", "https://127.0.0.1/repo")?;
        assert!(!external.matches(&loopback));
        let exact = RemoteRepository::new("external:*", "https://repo.example")?;
        assert!(external.exactly_matches(&exact));
        Ok(())
    }
}
